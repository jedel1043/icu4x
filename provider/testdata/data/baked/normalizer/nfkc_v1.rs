// @generated
type DataStruct = < :: icu_normalizer :: provider :: CompatibilityCompositionPassthroughV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_store_unchecked(&[("und", UND)]);
static UND: &DataStruct = &::icu_normalizer::provider::CompositionPassthroughV1 {
    first: 160u32,
    trie: ::icu_collections::codepointtrie::CodePointTrie::from_parts(
        ::icu_collections::codepointtrie::CodePointTrieHeader {
            high_start: 24576u32,
            shifted12_high_start: 6u16,
            index3_null_offset: 26u16,
            data_null_offset: 597u32,
            null_value: 0u32,
            trie_type: ::icu_collections::codepointtrie::TrieType::Small,
        },
        unsafe {
            ::zerovec::ZeroVec::from_bytes_unchecked(&[
                0u8, 0u8, 64u8, 0u8, 128u8, 0u8, 190u8, 0u8, 254u8, 0u8, 62u8, 1u8, 121u8, 1u8,
                181u8, 1u8, 240u8, 1u8, 41u8, 2u8, 85u8, 2u8, 115u8, 2u8, 175u8, 2u8, 237u8, 2u8,
                44u8, 3u8, 108u8, 3u8, 172u8, 3u8, 231u8, 3u8, 27u8, 4u8, 85u8, 2u8, 85u8, 2u8,
                90u8, 4u8, 139u8, 4u8, 203u8, 4u8, 11u8, 5u8, 75u8, 5u8, 85u8, 2u8, 85u8, 2u8,
                85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8,
                2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8,
                85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8,
                2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8,
                85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 18u8, 2u8, 50u8,
                2u8, 0u8, 0u8, 16u8, 0u8, 32u8, 0u8, 48u8, 0u8, 64u8, 0u8, 80u8, 0u8, 96u8, 0u8,
                112u8, 0u8, 128u8, 0u8, 144u8, 0u8, 160u8, 0u8, 176u8, 0u8, 190u8, 0u8, 206u8, 0u8,
                222u8, 0u8, 238u8, 0u8, 254u8, 0u8, 14u8, 1u8, 30u8, 1u8, 46u8, 1u8, 62u8, 1u8,
                78u8, 1u8, 94u8, 1u8, 110u8, 1u8, 121u8, 1u8, 137u8, 1u8, 153u8, 1u8, 169u8, 1u8,
                181u8, 1u8, 197u8, 1u8, 213u8, 1u8, 229u8, 1u8, 240u8, 1u8, 0u8, 2u8, 16u8, 2u8,
                32u8, 2u8, 41u8, 2u8, 57u8, 2u8, 73u8, 2u8, 89u8, 2u8, 85u8, 2u8, 101u8, 2u8,
                117u8, 2u8, 133u8, 2u8, 115u8, 2u8, 131u8, 2u8, 147u8, 2u8, 163u8, 2u8, 175u8, 2u8,
                191u8, 2u8, 207u8, 2u8, 223u8, 2u8, 237u8, 2u8, 253u8, 2u8, 13u8, 3u8, 29u8, 3u8,
                44u8, 3u8, 60u8, 3u8, 76u8, 3u8, 92u8, 3u8, 108u8, 3u8, 124u8, 3u8, 140u8, 3u8,
                156u8, 3u8, 172u8, 3u8, 188u8, 3u8, 204u8, 3u8, 220u8, 3u8, 231u8, 3u8, 247u8, 3u8,
                7u8, 4u8, 23u8, 4u8, 27u8, 4u8, 43u8, 4u8, 59u8, 4u8, 75u8, 4u8, 85u8, 2u8, 101u8,
                2u8, 117u8, 2u8, 133u8, 2u8, 85u8, 2u8, 101u8, 2u8, 117u8, 2u8, 133u8, 2u8, 90u8,
                4u8, 106u8, 4u8, 122u8, 4u8, 138u8, 4u8, 139u8, 4u8, 155u8, 4u8, 171u8, 4u8, 187u8,
                4u8, 203u8, 4u8, 219u8, 4u8, 235u8, 4u8, 251u8, 4u8, 11u8, 5u8, 27u8, 5u8, 43u8,
                5u8, 59u8, 5u8, 75u8, 5u8, 91u8, 5u8, 107u8, 5u8, 123u8, 5u8, 85u8, 2u8, 101u8,
                2u8, 117u8, 2u8, 133u8, 2u8, 85u8, 2u8, 101u8, 2u8, 117u8, 2u8, 133u8, 2u8, 85u8,
                2u8, 101u8, 2u8, 117u8, 2u8, 133u8, 2u8, 85u8, 2u8, 101u8, 2u8, 117u8, 2u8, 133u8,
                2u8, 85u8, 2u8, 101u8, 2u8, 117u8, 2u8, 133u8, 2u8, 85u8, 2u8, 101u8, 2u8, 117u8,
                2u8, 133u8, 2u8, 85u8, 2u8, 101u8, 2u8, 117u8, 2u8, 133u8, 2u8, 85u8, 2u8, 101u8,
                2u8, 117u8, 2u8, 133u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8,
                85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 139u8,
                5u8, 155u8, 5u8, 170u8, 5u8, 185u8, 5u8, 201u8, 5u8, 209u8, 5u8, 224u8, 5u8, 235u8,
                5u8, 85u8, 2u8, 245u8, 5u8, 4u8, 6u8, 18u8, 6u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8,
                85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8,
                2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 39u8, 4u8, 39u8, 4u8,
                39u8, 4u8, 39u8, 4u8, 39u8, 4u8, 39u8, 4u8, 39u8, 4u8, 39u8, 4u8, 39u8, 4u8, 39u8,
                4u8, 39u8, 4u8, 39u8, 4u8, 39u8, 4u8, 39u8, 4u8, 39u8, 4u8, 39u8, 4u8, 85u8, 2u8,
                85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8,
                2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8,
                85u8, 2u8, 85u8, 2u8, 39u8, 4u8, 39u8, 4u8, 34u8, 6u8, 48u8, 6u8, 64u8, 6u8, 74u8,
                6u8, 39u8, 4u8, 39u8, 4u8, 85u8, 6u8, 99u8, 6u8, 115u8, 6u8, 130u8, 6u8, 146u8,
                6u8, 155u8, 6u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 169u8, 6u8, 85u8, 2u8, 172u8,
                5u8, 185u8, 6u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8,
                85u8, 2u8, 85u8, 2u8, 201u8, 6u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8,
                216u8, 6u8, 224u8, 6u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 237u8, 6u8,
                85u8, 2u8, 85u8, 2u8, 248u8, 6u8, 0u8, 7u8, 16u8, 7u8, 24u8, 7u8, 5u8, 4u8, 40u8,
                7u8, 55u8, 7u8, 65u8, 7u8, 255u8, 3u8, 74u8, 7u8, 85u8, 2u8, 89u8, 7u8, 101u8, 7u8,
                85u8, 2u8, 112u8, 7u8, 215u8, 4u8, 122u8, 7u8, 133u8, 7u8, 85u8, 2u8, 5u8, 4u8,
                85u8, 2u8, 143u8, 7u8, 172u8, 5u8, 153u8, 7u8, 193u8, 2u8, 85u8, 2u8, 85u8, 2u8,
                215u8, 4u8, 85u8, 2u8, 162u8, 7u8, 48u8, 3u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8,
                85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8,
                2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8,
                85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 171u8, 7u8, 186u8, 7u8, 85u8, 2u8, 85u8, 2u8,
                85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 193u8, 7u8, 85u8,
                2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8,
                85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8,
                2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8,
                85u8, 2u8, 85u8, 2u8, 208u8, 7u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8,
                2u8, 85u8, 2u8, 213u8, 7u8, 229u8, 7u8, 238u8, 7u8, 85u8, 2u8, 85u8, 2u8, 85u8,
                2u8, 254u8, 7u8, 11u8, 8u8, 27u8, 8u8, 39u8, 4u8, 39u8, 4u8, 43u8, 6u8, 39u8, 4u8,
                38u8, 8u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8,
                2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8,
                85u8, 2u8, 85u8, 2u8, 54u8, 8u8, 85u8, 2u8, 186u8, 7u8, 85u8, 2u8, 85u8, 2u8, 65u8,
                8u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8,
                85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 79u8, 8u8, 90u8, 8u8, 85u8, 2u8, 85u8,
                2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8,
                106u8, 8u8, 122u8, 8u8, 85u8, 2u8, 85u8, 2u8, 138u8, 8u8, 182u8, 5u8, 154u8, 8u8,
                85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8,
                2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8,
                85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 165u8, 8u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8,
                2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8,
                85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 39u8, 4u8, 39u8, 4u8, 39u8,
                4u8, 39u8, 4u8, 181u8, 8u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8,
                85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 85u8, 2u8, 66u8, 0u8, 98u8,
                0u8, 130u8, 0u8, 162u8, 0u8, 170u8, 0u8, 170u8, 0u8, 170u8, 0u8, 170u8, 0u8, 26u8,
                0u8, 26u8, 0u8, 202u8, 0u8, 26u8, 0u8, 26u8, 0u8, 226u8, 0u8, 26u8, 0u8, 2u8, 1u8,
                34u8, 1u8, 66u8, 1u8, 26u8, 0u8, 26u8, 0u8, 26u8, 0u8, 26u8, 0u8, 94u8, 1u8, 26u8,
                0u8, 26u8, 0u8, 26u8, 0u8, 26u8, 0u8, 126u8, 1u8, 26u8, 0u8, 156u8, 1u8, 188u8,
                1u8, 218u8, 1u8, 26u8, 0u8, 26u8, 0u8, 26u8, 0u8, 26u8, 0u8, 26u8, 0u8, 26u8, 0u8,
                26u8, 0u8, 26u8, 0u8, 26u8, 0u8, 26u8, 0u8, 26u8, 0u8, 26u8, 0u8, 26u8, 0u8, 26u8,
                0u8, 26u8, 0u8, 242u8, 1u8,
            ])
        },
        unsafe {
            ::zerovec::ZeroVec::from_bytes_unchecked(&[
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 1u8, 133u8, 60u8, 119u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 12u8, 128u8, 1u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                128u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 240u8, 31u8, 0u8, 0u8, 0u8, 0u8,
                14u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 255u8, 1u8, 0u8, 0u8, 0u8, 63u8, 31u8, 0u8,
                0u8, 0u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 127u8,
                255u8, 255u8, 255u8, 255u8, 16u8, 68u8, 176u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 127u8, 0u8, 0u8, 0u8, 55u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 248u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 128u8, 0u8, 254u8, 255u8, 255u8, 255u8,
                255u8, 191u8, 182u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 255u8, 7u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 248u8, 255u8, 255u8, 0u8, 0u8, 225u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 192u8, 159u8, 159u8, 61u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8,
                0u8, 255u8, 255u8, 255u8, 7u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 248u8, 15u8, 32u8, 0u8, 0u8, 192u8,
                251u8, 239u8, 62u8, 0u8, 0u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 255u8, 0u8, 0u8, 0u8, 0u8, 0u8, 252u8, 255u8, 255u8, 251u8, 255u8, 255u8,
                255u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 16u8, 0u8, 32u8, 30u8, 255u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 80u8, 0u8, 32u8, 128u8, 176u8, 0u8,
                0u8, 0u8, 64u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 72u8, 16u8, 0u8, 32u8, 0u8, 78u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 16u8, 0u8, 32u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 80u8, 0u8, 32u8, 192u8,
                48u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 64u8, 0u8, 32u8,
                128u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 16u8, 0u8, 32u8, 96u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 16u8, 4u8, 32u8, 96u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 88u8, 0u8, 32u8, 128u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 132u8, 0u8, 128u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 8u8, 7u8, 0u8, 15u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 8u8, 7u8, 0u8, 15u8, 0u8, 48u8, 0u8, 0u8, 0u8, 0u8, 0u8, 16u8,
                0u8, 3u8, 0u8, 0u8, 160u8, 2u8, 8u8, 32u8, 132u8, 16u8, 0u8, 2u8, 254u8, 63u8,
                223u8, 0u8, 8u8, 32u8, 132u8, 16u8, 0u8, 2u8, 64u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 64u8, 128u8, 6u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 32u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 16u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 254u8, 255u8, 63u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 255u8, 255u8, 255u8, 7u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 224u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 48u8, 0u8, 0u8, 0u8, 16u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 4u8, 32u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 128u8, 1u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 224u8, 159u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 255u8, 191u8, 255u8, 127u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 48u8, 0u8, 16u8, 0u8, 0u8, 0u8, 0u8, 248u8, 15u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 12u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 64u8, 0u8, 12u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 128u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 247u8, 255u8, 253u8, 33u8, 16u8, 3u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 112u8, 255u8, 247u8, 255u8, 191u8, 255u8, 255u8, 255u8, 7u8, 0u8, 1u8,
                0u8, 0u8, 0u8, 248u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
                255u8, 255u8, 255u8, 255u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 12u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 170u8, 42u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 232u8, 3u8, 234u8,
                8u8, 232u8, 8u8, 232u8, 0u8, 106u8, 255u8, 7u8, 130u8, 0u8, 112u8, 128u8, 216u8,
                80u8, 128u8, 3u8, 128u8, 128u8, 0u8, 0u8, 243u8, 255u8, 255u8, 127u8, 255u8, 31u8,
                0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 255u8, 31u8, 226u8, 255u8, 1u8, 0u8, 239u8, 254u8,
                111u8, 62u8, 87u8, 189u8, 251u8, 251u8, 225u8, 3u8, 255u8, 255u8, 255u8, 255u8,
                255u8, 255u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 176u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 255u8, 255u8,
                255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
                255u8, 255u8, 255u8, 7u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 16u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 112u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 16u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 48u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 128u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 128u8, 0u8, 128u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 255u8, 255u8, 255u8, 255u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 128u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 8u8, 0u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
                255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
                255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 63u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 252u8, 64u8, 7u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 158u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                128u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 254u8, 255u8, 255u8, 255u8, 255u8, 255u8,
                255u8, 255u8, 255u8, 255u8, 255u8, 127u8, 252u8, 255u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 255u8, 255u8, 255u8, 127u8, 255u8, 255u8, 255u8,
                255u8, 255u8, 0u8, 255u8, 255u8, 255u8, 255u8, 255u8, 127u8, 255u8, 255u8, 255u8,
                255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
                255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
                255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
                255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 128u8, 240u8, 63u8, 0u8, 0u8, 0u8,
                240u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 28u8, 3u8, 64u8, 0u8, 0u8, 0u8, 0u8,
                16u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 16u8, 0u8, 0u8, 0u8, 255u8,
                255u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 56u8, 0u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 8u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 157u8, 193u8, 2u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 64u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 240u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 32u8, 0u8, 0u8, 255u8, 63u8, 229u8, 127u8, 101u8, 252u8, 255u8, 255u8, 255u8,
                255u8, 255u8, 255u8, 255u8, 63u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
                255u8, 255u8, 255u8, 255u8, 3u8, 0u8, 0u8, 0u8, 0u8, 127u8, 0u8, 248u8, 224u8,
                255u8, 255u8, 127u8, 95u8, 219u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
                3u8, 0u8, 0u8, 0u8, 248u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 63u8,
                0u8, 0u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 252u8, 255u8, 255u8, 255u8,
                255u8, 255u8, 255u8, 0u8, 0u8, 0u8, 0u8, 0u8, 255u8, 31u8, 0u8, 0u8, 255u8, 3u8,
                255u8, 255u8, 255u8, 255u8, 159u8, 255u8, 247u8, 255u8, 127u8, 15u8, 215u8, 255u8,
                255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
                255u8, 255u8, 31u8, 254u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
                255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 127u8, 252u8, 252u8, 252u8, 28u8,
                127u8, 127u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 32u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                192u8, 7u8, 190u8, 255u8, 255u8, 255u8, 255u8, 255u8, 253u8, 7u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 160u8, 0u8, 0u8, 0u8, 0u8, 0u8, 135u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 96u8, 0u8, 0u8, 0u8, 0u8, 240u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 24u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 192u8, 255u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 60u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 64u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 1u8, 128u8, 7u8, 0u8, 0u8, 0u8, 128u8, 0u8, 24u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 4u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 96u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 88u8,
                0u8, 32u8, 128u8, 0u8, 192u8, 31u8, 31u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                68u8, 0u8, 0u8, 64u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 36u8, 12u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 128u8, 0u8, 128u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                192u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 96u8, 8u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 16u8,
                0u8, 128u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 52u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 31u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                127u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8,
                0u8, 0u8, 0u8, 64u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                192u8, 255u8, 227u8, 7u8, 248u8, 231u8, 15u8, 0u8, 0u8, 0u8, 60u8, 0u8, 248u8, 1u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 28u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 223u8, 255u8,
                255u8, 255u8, 255u8, 255u8, 223u8, 100u8, 222u8, 255u8, 235u8, 239u8, 255u8, 255u8,
                255u8, 255u8, 255u8, 255u8, 255u8, 191u8, 231u8, 223u8, 223u8, 255u8, 255u8, 255u8,
                123u8, 95u8, 252u8, 253u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
                255u8, 207u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 127u8, 255u8, 255u8, 249u8,
                219u8, 7u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 64u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 240u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 127u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 240u8, 7u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                239u8, 255u8, 255u8, 255u8, 150u8, 254u8, 247u8, 10u8, 132u8, 234u8, 150u8, 170u8,
                150u8, 247u8, 247u8, 94u8, 255u8, 251u8, 255u8, 15u8, 238u8, 251u8, 255u8, 15u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 255u8, 7u8, 255u8, 255u8, 255u8, 127u8,
                255u8, 255u8, 255u8, 255u8, 0u8, 0u8, 0u8, 28u8, 0u8, 0u8, 7u8, 0u8, 255u8, 255u8,
                255u8, 255u8, 255u8, 15u8, 255u8, 1u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 255u8, 3u8, 255u8, 255u8, 255u8, 63u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            ])
        },
        0u8,
    ),
};
