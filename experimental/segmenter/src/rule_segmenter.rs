// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Note: Keep these constants in sync with build.rs.
const NOT_MATCH_RULE: i8 = -2;
const KEEP_RULE: i8 = -1;
// This is a mask bit chosen sufficiently large than all other concrete states.
// If a break state contains this bit, we have to look ahead one more character.
const INTERMEDIATE_MATCH_RULE: i8 = 64;

/// A trait allowing for RuleBreakIterator to be generalized to multiple string
/// encoding methods and granularity such as grapheme cluster, word, etc.
pub trait RuleBreakType<'l, 's> {
    /// The iterator over characters.
    type IterAttr: Iterator<Item = (usize, Self::CharType)> + Clone;

    /// The character type.
    type CharType: Copy + Into<u32>;

    fn get_current_position_character_len(iter: &RuleBreakIterator<'l, 's, Self>) -> usize;

    fn handle_complex_language(
        iter: &mut RuleBreakIterator<'l, 's, Self>,
        left_codepoint: Self::CharType,
    ) -> Option<usize>;
}

/// Implements the [`Iterator`] trait over the segmenter break opportunities of the given string.
/// Please see the [module-level documentation] for its usages.
///
/// Lifetimes:
///
/// - `'l` = lifetime of the segmenter object from which this iterator was created
/// - `'s` = lifetime of the string being segmented
///
/// [`Iterator`]: core::iter::Iterator
/// [module-level documentation]: index.html
pub struct RuleBreakIterator<'l, 's, Y: RuleBreakType<'l, 's> + ?Sized> {
    pub(crate) iter: Y::IterAttr,
    pub(crate) len: usize,
    pub(crate) current_pos_data: Option<(usize, Y::CharType)>,
    pub(crate) result_cache: alloc::vec::Vec<usize>,
    pub(crate) break_state_table: &'l [i8],
    pub(crate) property_table: &'l [&'l [u8; 1024]; 897],
    pub(crate) rule_property_count: usize,
    pub(crate) last_codepoint_property: i8,
    pub(crate) sot_property: u8,
    pub(crate) eot_property: u8,
    pub(crate) complex_property: u8,
}

impl<'l, 's, Y: RuleBreakType<'l, 's>> Iterator for RuleBreakIterator<'l, 's, Y> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // If we have break point cache by previous run, return this result
        if !self.result_cache.is_empty() {
            let mut i = 0;
            loop {
                if i == *self.result_cache.first().unwrap() {
                    self.result_cache = self.result_cache.iter().skip(1).map(|r| r - i).collect();
                    return Some(self.current_pos_data.unwrap().0);
                }
                i += Y::get_current_position_character_len(self);
                self.current_pos_data = self.iter.next();
                if self.current_pos_data.is_none() {
                    // Reach EOF
                    self.result_cache.clear();
                    return Some(self.len);
                }
            }
        }

        if self.current_pos_data.is_none() {
            let current_pos_data = self.iter.next()?;
            self.current_pos_data = Some(current_pos_data);
            // SOT x anything
            let right_prop = self.get_current_break_property();
            if self.is_break_from_table(self.sot_property, right_prop) {
                return Some(current_pos_data.0);
            }
        }

        loop {
            let left_codepoint = self.get_current_codepoint();
            let left_prop = self.get_break_property(left_codepoint);
            self.current_pos_data = self.iter.next();

            if self.current_pos_data.is_none() {
                return Some(self.len);
            }
            let right_prop = self.get_current_break_property();

            // Some segmenter rules doesn't have language-specific rules, we have to use LSTM (or dictionary) segmenter.
            // If property is marked as SA, use it
            if right_prop == self.complex_property {
                if left_prop != self.complex_property {
                    // break before SA
                    return Some(self.current_pos_data.unwrap().0);
                }
                let break_offset = Y::handle_complex_language(self, left_codepoint);
                if break_offset.is_some() {
                    return break_offset;
                }
            }

            // If break_state is equals or grater than 0, it is alias of property.
            let mut break_state = self.get_break_state_from_table(left_prop, right_prop);

            if break_state >= 0 {
                // This isn't simple rule set. We need marker to restore iterator to previous position.
                let mut previous_iter = self.iter.clone();
                let mut previous_pos_data = self.current_pos_data;

                loop {
                    self.current_pos_data = self.iter.next();
                    if self.current_pos_data.is_none() {
                        // Reached EOF. But we are analyzing multiple characters now, so next break may be previous point.
                        if self
                            .get_break_state_from_table(break_state as u8, self.eot_property as u8)
                            == NOT_MATCH_RULE
                        {
                            self.iter = previous_iter;
                            self.current_pos_data = previous_pos_data;
                            return Some(previous_pos_data.unwrap().0);
                        }
                        // EOF
                        return Some(self.len);
                    }

                    let previous_break_state = break_state;
                    let prop = self.get_current_break_property();
                    break_state = self.get_break_state_from_table(break_state as u8, prop);
                    if break_state < 0 {
                        break;
                    }
                    if previous_break_state >= 0
                        && previous_break_state <= self.last_codepoint_property
                    {
                        // Move marker
                        previous_iter = self.iter.clone();
                        previous_pos_data = self.current_pos_data;
                    }
                    if (break_state & INTERMEDIATE_MATCH_RULE) != 0 {
                        break_state -= INTERMEDIATE_MATCH_RULE;
                        previous_iter = self.iter.clone();
                        previous_pos_data = self.current_pos_data;
                    }
                }
                if break_state == KEEP_RULE {
                    continue;
                }
                if break_state == NOT_MATCH_RULE {
                    self.iter = previous_iter;
                    self.current_pos_data = previous_pos_data;
                    return Some(previous_pos_data.unwrap().0);
                }
                return Some(self.current_pos_data.unwrap().0);
            }

            if self.is_break_from_table(left_prop, right_prop) {
                return Some(self.current_pos_data.unwrap().0);
            }
        }
    }
}

impl<'l, 's, Y: RuleBreakType<'l, 's>> RuleBreakIterator<'l, 's, Y> {
    pub(crate) fn get_current_break_property(&self) -> u8 {
        self.get_break_property(self.get_current_codepoint())
    }

    fn get_current_codepoint(&self) -> Y::CharType {
        self.current_pos_data
            .expect("Not at the end of the string!")
            .1
    }

    fn get_break_property(&self, codepoint: Y::CharType) -> u8 {
        let codepoint = codepoint.into() as usize;
        if codepoint >= 897 * 1024 {
            // Unknown
            return 0;
        }
        self.property_table[codepoint / 1024][(codepoint & 0x3ff)]
    }

    fn get_break_state_from_table(&self, left: u8, right: u8) -> i8 {
        self.break_state_table[(left as usize) * self.rule_property_count + (right as usize)]
    }

    fn is_break_from_table(&self, left: u8, right: u8) -> bool {
        let rule = self.get_break_state_from_table(left, right);
        if rule == KEEP_RULE {
            return false;
        }
        if rule >= 0 {
            // need additional next characters to get break rule.
            return false;
        }
        true
    }
}
