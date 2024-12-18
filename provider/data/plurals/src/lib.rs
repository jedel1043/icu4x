// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data for the `icu_plurals` crate
//!
//! This data was generated with CLDR version 46.1.0-BETA1, ICU version icu4x/2024-12-16/76.x, and
//! LSTM segmenter version v0.1.0.

#![no_std]
// The source is not readable and is massive as HTML.
#![doc(html_no_source)]

#[cfg(icu4x_custom_data)]
include!(concat!(core::env!("ICU4X_DATA_DIR"), "/mod.rs"));
#[cfg(not(icu4x_custom_data))]
include!("../data/mod.rs");

#[rustfmt::skip]
pub use icu_provider_baked;
pub use icu_locale;
