// @generated
/// Implement `DataProvider<JapaneseYearSymbolsV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_datetime_symbols_japanese_years_v1 {
    ($ provider : ty) => {
        #[clippy::msrv = "1.67"]
        const _: () = <$provider>::MUST_USE_MAKE_PROVIDER_MACRO;
        #[clippy::msrv = "1.67"]
        impl icu_provider::DataProvider<icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker>, icu_provider::DataError> {
                static BN_IN_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0!\0E\0K\0P\0U\0[\0\xE0\xA6\x96\xE0\xA7\x8D\xE0\xA6\xB0\xE0\xA6\xBF\xE0\xA6\xB7\xE0\xA7\x8D\xE0\xA6\x9F\xE0\xA6\xBE\xE0\xA6\xAC\xE0\xA7\x8D\xE0\xA6\xA6\xE0\xA6\x96\xE0\xA7\x8D\xE0\xA6\xB0\xE0\xA6\xBF\xE0\xA6\xB8\xE0\xA7\x8D\xE0\xA6\x9F\xE0\xA6\xAA\xE0\xA7\x82\xE0\xA6\xB0\xE0\xA7\x8D\xE0\xA6\xACHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static BN_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0!\0E\0K\0P\0U\0[\0\xE0\xA6\x96\xE0\xA7\x8D\xE0\xA6\xB0\xE0\xA7\x80\xE0\xA6\xB7\xE0\xA7\x8D\xE0\xA6\x9F\xE0\xA6\xBE\xE0\xA6\xAC\xE0\xA7\x8D\xE0\xA6\xA6\xE0\xA6\x96\xE0\xA7\x8D\xE0\xA6\xB0\xE0\xA6\xBF\xE0\xA6\xB8\xE0\xA7\x8D\xE0\xA6\x9F\xE0\xA6\xAA\xE0\xA7\x82\xE0\xA6\xB0\xE0\xA7\x8D\xE0\xA6\xACHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static AS_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0!\0E\0K\0P\0U\0[\0\xE0\xA6\x96\xE0\xA7\x8D\xE0\xA7\xB0\xE0\xA7\x80\xE0\xA6\xB7\xE0\xA7\x8D\xE0\xA6\x9F\xE0\xA6\xBE\xE0\xA6\xAC\xE0\xA7\x8D\xE0\xA6\xA6\xE0\xA6\x96\xE0\xA7\x8D\xE0\xA7\xB0\xE0\xA7\x80\xE0\xA6\xB7\xE0\xA7\x8D\xE0\xA6\x9F\xE0\xA6\xAA\xE0\xA7\x82\xE0\xA7\xB0\xE0\xA7\x8D\xE0\xA6\xACHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static OR_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0!\0E\0K\0P\0U\0[\0\xE0\xAC\x96\xE0\xAD\x8D\xE0\xAC\xB0\xE0\xAD\x80\xE0\xAC\xB7\xE0\xAD\x8D\xE0\xAC\x9F\xE0\xAC\xBE\xE0\xAC\xAC\xE0\xAD\x8D\xE0\xAC\xA6\xE0\xAC\x96\xE0\xAD\x8D\xE0\xAC\xB0\xE0\xAD\x80\xE0\xAC\xB7\xE0\xAD\x8D\xE0\xAC\x9F\xE0\xAC\xAA\xE0\xAD\x82\xE0\xAC\xB0\xE0\xAD\x8D\xE0\xAC\xACHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static PS_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0#\0F\0L\0Q\0V\0\\\0\xD9\x84\xD9\x87 \xD9\x85\xDB\x8C\xD9\x84\xD8\xA7\xD8\xAF \xDA\x85\xD8\xAE\xD9\x87 \xD9\x88\xD8\xB1\xD9\x88\xD8\xB3\xD8\xAA\xD9\x87\xD9\x84\xD9\x87 \xD9\x85\xDB\x8C\xD9\x84\xD8\xA7\xD8\xAF \xDA\x85\xD8\xAE\xD9\x87 \xD9\x88\xDA\x93\xD8\xA7\xD9\x86\xD8\xAF\xDB\x90HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static TH_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0$\0N\0Z\0f\0r\0~\0\xE0\xB8\x84\xE0\xB8\xA3\xE0\xB8\xB4\xE0\xB8\xAA\xE0\xB8\x95\xE0\xB9\x8C\xE0\xB8\xA8\xE0\xB8\xB1\xE0\xB8\x81\xE0\xB8\xA3\xE0\xB8\xB2\xE0\xB8\x8A\xE0\xB8\x9B\xE0\xB8\xB5\xE0\xB8\x81\xE0\xB9\x88\xE0\xB8\xAD\xE0\xB8\x99\xE0\xB8\x84\xE0\xB8\xA3\xE0\xB8\xB4\xE0\xB8\xAA\xE0\xB8\x95\xE0\xB8\x81\xE0\xB8\xB2\xE0\xB8\xA5\xE0\xB9\x80\xE0\xB8\xAE\xE0\xB9\x80\xE0\xB8\x8B\xE0\xB9\x80\xE0\xB8\xA1\xE0\xB8\x88\xE0\xB8\xB4\xE0\xB9\x80\xE0\xB8\xA3\xE0\xB8\xA7\xE0\xB8\xB0\xE0\xB9\x82\xE0\xB8\x8A\xE0\xB8\xA7\xE0\xB8\xB0\xE0\xB8\x97\xE0\xB8\xB0\xE0\xB8\xAD\xE0\xB8\xB4\xE0\xB9\x82\xE0\xB8\x8A") })
                });
                static KM_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0$\0T\0Z\0_\0d\0j\0\xE1\x9E\x82\xE1\x9F\x92\xE1\x9E\x9A\xE1\x9E\xB7\xE1\x9E\x9F\xE1\x9F\x92\xE1\x9E\x8F\xE1\x9E\x9F\xE1\x9E\x80\xE1\x9E\x9A\xE1\x9E\xB6\xE1\x9E\x87\xE1\x9E\x98\xE1\x9E\xBB\xE1\x9E\x93\xE2\x80\x8B\xE1\x9E\x82\xE1\x9F\x92\xE1\x9E\x9A\xE1\x9E\xB7\xE1\x9E\x9F\xE1\x9F\x92\xE1\x9E\x8F\xE1\x9E\x9F\xE1\x9E\x80\xE1\x9E\x9A\xE1\x9E\xB6\xE1\x9E\x87HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static LO_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0$\0T\0`\0l\0q\0}\0\xE0\xBA\x84\xE0\xBA\xA3\xE0\xBA\xB4\xE0\xBA\x94\xE0\xBA\xAA\xE0\xBA\xB1\xE0\xBA\x81\xE0\xBA\x81\xE0\xBA\xB0\xE0\xBA\xA5\xE0\xBA\xB2\xE0\xBA\x94\xE0\xBA\x81\xE0\xBB\x88\xE0\xBA\xAD\xE0\xBA\x99\xE0\xBA\x84\xE0\xBA\xA3\xE0\xBA\xB4\xE0\xBA\x94\xE0\xBA\xAA\xE0\xBA\xB1\xE0\xBA\x81\xE0\xBA\x81\xE0\xBA\xB0\xE0\xBA\xA5\xE0\xBA\xB2\xE0\xBA\x94\xE0\xBA\xAE\xE0\xBA\xB5\xE0\xBA\x8A\xE0\xBA\xB5\xE0\xBA\xA1\xE0\xBA\xB5\xE0\xBA\x88\xE0\xBA\xB5Reiwa\xE0\xBB\x82\xE0\xBA\x8A\xE0\xBA\xA7\xE0\xBA\xB2\xE0\xBB\x84\xE0\xBA\x95\xE0\xBB\x82\xE0\xBA\x8A") })
                });
                static RU_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0(\0P\0g\0~\0\x88\0\x92\0\xD0\xBE\xD1\x82 \xD0\xA0\xD0\xBE\xD0\xB6\xD0\xB4\xD0\xB5\xD1\x81\xD1\x82\xD0\xB2\xD0\xB0 \xD0\xA5\xD1\x80\xD0\xB8\xD1\x81\xD1\x82\xD0\xBE\xD0\xB2\xD0\xB0\xD0\xB4\xD0\xBE \xD0\xA0\xD0\xBE\xD0\xB6\xD0\xB4\xD0\xB5\xD1\x81\xD1\x82\xD0\xB2\xD0\xB0 \xD0\xA5\xD1\x80\xD0\xB8\xD1\x81\xD1\x82\xD0\xBE\xD0\xB2\xD0\xB0\xD0\xAD\xD0\xBF\xD0\xBE\xD1\x85\xD0\xB0 \xD0\xA5\xD1\x8D\xD0\xB9\xD1\x81\xD1\x8D\xD0\xB9\xD0\xAD\xD0\xBF\xD0\xBE\xD1\x85\xD0\xB0 \xD0\x9C\xD1\x8D\xD0\xB9\xD0\xB4\xD0\xB7\xD0\xB8\xD0\xA0\xD1\x8D\xD0\xB9\xD0\xB2\xD0\xB0\xD0\xA1\xD1\x8C\xD0\xBE\xD0\xB2\xD0\xB0\xD0\xAD\xD0\xBF\xD0\xBE\xD1\x85\xD0\xB0 \xD0\xA2\xD0\xB0\xD0\xB9\xD1\x81\xD1\x8C\xD0\xBE") })
                });
                static SI_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0(\0S\0Y\0^\0c\0i\0\xE0\xB6\x9A\xE0\xB7\x8A\xE2\x80\x8D\xE0\xB6\xBB\xE0\xB7\x92\xE0\xB7\x83\xE0\xB7\x8A\xE0\xB6\xAD\xE0\xB7\x94 \xE0\xB7\x80\xE0\xB6\xBB\xE0\xB7\x8A\xE0\xB7\x82\xE0\xB6\x9A\xE0\xB7\x8A\xE2\x80\x8D\xE0\xB6\xBB\xE0\xB7\x92\xE0\xB7\x83\xE0\xB7\x8A\xE0\xB6\xAD\xE0\xB7\x94 \xE0\xB6\xB4\xE0\xB7\x96\xE0\xB6\xBB\xE0\xB7\x8A\xE0\xB7\x80HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static BE_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0*\0T\0Z\0_\0d\0j\0\xD0\xB0\xD0\xB4 \xD0\xBD\xD0\xB0\xD1\x80\xD0\xB0\xD0\xB4\xD0\xB6\xD1\x8D\xD0\xBD\xD0\xBD\xD1\x8F \xD0\xA5\xD1\x80\xD1\x8B\xD1\x81\xD1\x82\xD0\xBE\xD0\xB2\xD0\xB0\xD0\xB4\xD0\xB0 \xD0\xBD\xD0\xB0\xD1\x80\xD0\xB0\xD0\xB4\xD0\xB6\xD1\x8D\xD0\xBD\xD0\xBD\xD1\x8F \xD0\xA5\xD1\x80\xD1\x8B\xD1\x81\xD1\x82\xD0\xBE\xD0\xB2\xD0\xB0HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static CV_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0*\0V\0\\\0a\0f\0l\0\xD0\xA5\xD1\x80\xD0\xB8\xD1\x81\xD1\x82\xD0\xBE\xD1\x81 \xD2\xAB\xD1\x83\xD1\x80\xD0\xB0\xD0\xBB\xD0\xBD\xD3\x91 \xD0\xBA\xD1\x83\xD0\xBD\xD1\x80\xD0\xB0\xD0\xBD\xD0\xA5\xD1\x80\xD0\xB8\xD1\x81\xD1\x82\xD0\xBE\xD1\x81 \xD2\xAB\xD1\x83\xD1\x80\xD0\xB0\xD0\xBB\xD0\xBD\xD3\x91 \xD0\xBA\xD1\x83\xD0\xBD\xD1\x87\xD1\x87\xD0\xB5\xD0\xBDHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static FF_ADLM_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0B\0|\0\x94\0\xA8\0\xBC\0\xD0\0\xF0\x9E\xA4\x87\xF0\x9E\xA4\xA2\xF0\x9E\xA5\x84\xF0\x9E\xA4\xB1\xF0\x9E\xA4\xAE \xF0\x9E\xA4\x80\xF0\x9E\xA4\xB2\xF0\x9E\xA5\x86\xF0\x9E\xA4\xA2\xF0\x9E\xA4\xA6\xF0\x9E\xA4\xAD \xF0\x9E\xA4\x8B\xF0\x9E\xA5\x85\xF0\x9E\xA4\xA7\xF0\x9E\xA4\xA2\xF0\x9E\xA5\x84\xF0\x9E\xA4\x80\xF0\x9E\xA4\xA3\xF0\x9E\xA4\xAE \xF0\x9E\xA4\x80\xF0\x9E\xA4\xB2\xF0\x9E\xA5\x86\xF0\x9E\xA4\xA2\xF0\x9E\xA4\xA6\xF0\x9E\xA4\xAD \xF0\x9E\xA4\x8B\xF0\x9E\xA5\x85\xF0\x9E\xA4\xA7\xF0\x9E\xA4\xA2\xF0\x9E\xA5\x84\xF0\x9E\xA4\x96\xF0\x9E\xA4\xAB\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xA7\xF0\x9E\xA4\xAB\xF0\x9E\xA4\xB4\xF0\x9E\xA4\x83\xF0\x9E\xA4\xAB\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xB6\xF0\x9E\xA4\xAD\xF0\x9E\xA4\x88\xF0\x9E\xA4\xAB\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xB1\xF0\x9E\xA4\xA2\xF0\x9E\xA4\xA1\xF0\x9E\xA4\xAE\xF0\x9E\xA5\x85\xF0\x9E\xA4\xB1\xF0\x9E\xA4\xA2\xF0\x9E\xA4\x9A\xF0\x9E\xA4\xA2\xF0\x9E\xA4\xB4\xF0\x9E\xA5\x83\xF0\x9E\xA4\xAE\xF0\x9E\xA5\x85") })
                });
                static TE_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\"\0M\0S\0X\0]\0c\0\xE0\xB0\x95\xE0\xB1\x8D\xE0\xB0\xB0\xE0\xB1\x80\xE0\xB0\xB8\xE0\xB1\x8D\xE0\xB0\xA4\xE0\xB1\x81 \xE0\xB0\xB6\xE0\xB0\x95\xE0\xB0\x82\xE0\xB0\x95\xE0\xB1\x8D\xE0\xB0\xB0\xE0\xB1\x80\xE0\xB0\xB8\xE0\xB1\x8D\xE0\xB0\xA4\xE0\xB1\x81 \xE0\xB0\xAA\xE0\xB1\x82\xE0\xB0\xB0\xE0\xB1\x8D\xE0\xB0\xB5\xE0\xB0\x82HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static BRX_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\"\0P\0V\0[\0`\0f\0\xE0\xA4\x86\xE0\xA4\xA8\xE0\xA5\x8D\xE0\xA4\xA8\xE2\x80\x99 \xE0\xA4\xA6\xE0\xA4\xBE\xE0\xA4\xAE\xE0\xA4\xBF\xE0\xA4\xA8\xE0\xA4\xBF\xE0\xA4\x96\xE0\xA5\x8D\xE0\xA4\xB0\xE0\xA4\xBE\xE0\xA4\x87\xE0\xA4\xB7\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\xA8\xE0\xA4\xBF \xE0\xA4\xB8\xE0\xA4\xBF\xE0\xA4\x97\xE0\xA4\xBE\xE0\xA4\x82HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static TA_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\"\0Y\0_\0d\0i\0o\0\xE0\xAE\x85\xE0\xAE\xA9\xE0\xAF\x8D\xE0\xAE\xA9\xE0\xAF\x8B \xE0\xAE\x9F\xE0\xAF\x8B\xE0\xAE\xAE\xE0\xAE\xBF\xE0\xAE\xA9\xE0\xAE\xBF\xE0\xAE\x95\xE0\xAE\xBF\xE0\xAE\xB1\xE0\xAE\xBF\xE0\xAE\xB8\xE0\xAF\x8D\xE0\xAE\xA4\xE0\xAF\x81\xE0\xAE\xB5\xE0\xAF\x81\xE0\xAE\x95\xE0\xAF\x8D\xE0\xAE\x95\xE0\xAF\x81 \xE0\xAE\xAE\xE0\xAF\x81\xE0\xAE\xA9\xE0\xAF\x8DHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static ML_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\"\0\\\0b\0g\0l\0r\0\xE0\xB4\x86\xE0\xB4\xA8\xE0\xB5\x8D\xE0\xB4\xA8\xE0\xB5\x8B \xE0\xB4\xA1\xE0\xB5\x8A\xE0\xB4\xAE\xE0\xB4\xBF\xE0\xB4\xA8\xE0\xB4\xBF\xE0\xB4\x95\xE0\xB5\x8D\xE0\xB4\xB0\xE0\xB4\xBF\xE0\xB4\xB8\xE0\xB5\x8D\xE2\x80\x8C\xE0\xB4\xA4\xE0\xB5\x81\xE0\xB4\xB5\xE0\xB4\xBF\xE0\xB4\xA8\xE0\xB5\x8D \xE0\xB4\xAE\xE0\xB5\x81\xE0\xB4\xAE\xE0\xB5\x8D\xE0\xB4\xAA\xE0\xB5\x8DHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static HE_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\n\0\x12\0\x13\0\x14\0\x15\0\x16\0\xD7\x90\xD7\x97\xD7\xA8\xD7\x99\xD7\x99\xD7\x9C\xD7\xA4\xD7\xA0\xD7\x99HMRST") })
                });
                static BG_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\n\0\x14\0\x15\0\x16\0\x17\0\x18\0\xD1\x81\xD0\xBB.\xD0\xA5\xD1\x80.\xD0\xBF\xD1\x80.\xD0\xA5\xD1\x80.HMRST") })
                });
                static DSB_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\n\0\x14\0\x15\0\x16\0\x17\0\x18\0p\xC3\xB3 Chr.n.p\xC5\x9B.Chr.n.HMRST") })
                });
                static BG_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\n\0\x14\0\x1A\0\x1F\0$\0*\0\xD1\x81\xD0\xBB.\xD0\xA5\xD1\x80.\xD0\xBF\xD1\x80.\xD0\xA5\xD1\x80.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static DSB_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\n\0\x14\0\x1A\0\x1F\0$\0*\0p\xC3\xB3 Chr.n.p\xC5\x9B.Chr.n.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static BR_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\n\0\x15\0\x16\0\x17\0\x18\0\x19\0goude J.K.a-raok J.K.HMRST") })
                });
                static BR_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\n\0\x15\0\x1B\0 \0%\0+\0goude J.K.a-raok J.K.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static BRX_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\n\0\x18\0\x19\0\x1A\0\x1B\0\x1C\0\xE0\xA4\x8F.\xE0\xA4\xA6\xE0\xA4\xBF\xE0\xA4\xAC\xE0\xA4\xBF.\xE0\xA4\xB8\xE0\xA4\xBF.HMRST") })
                });
                static BRX_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\n\0\x18\0\x1E\0#\0(\0.\0\xE0\xA4\x8F.\xE0\xA4\xA6\xE0\xA4\xBF\xE0\xA4\xAC\xE0\xA4\xBF.\xE0\xA4\xB8\xE0\xA4\xBF.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static UR_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\n\0\x19\0\x1A\0\x1B\0\x1C\0\x1D\0\xD8\xB9\xDB\x8C\xD8\xB3\xD9\x88\xDB\x8C\xD9\x82\xD8\xA8\xD9\x84 \xD9\x85\xD8\xB3\xDB\x8C\xD8\xADHMRST") })
                });
                static UR_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\n\0\x19\0\x1F\0$\0)\0/\0\xD8\xB9\xDB\x8C\xD8\xB3\xD9\x88\xDB\x8C\xD9\x82\xD8\xA8\xD9\x84 \xD9\x85\xD8\xB3\xDB\x8C\xD8\xADHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static PL_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\n\0\x1B\0!\0&\0+\x001\0naszej eryprzed nasz\xC4\x85 er\xC4\x85HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static RO_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\r\0 \0&\0+\x000\x006\0dup\xC4\x83 Hristos\xC3\xAEnainte de HristosHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static SO_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\r\0\x19\0\x1F\0$\0)\0/\0Ciise DabadiiCiise HortiiHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static DA_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\r\0\x19\0\x1F\0$\0)\0/\0efter Kristusf\xC3\xB8r KristusHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static NB_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\r\0\x19\0\x1F\0$\0)\0/\0etter Kristusf\xC3\xB8r KristusHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static QU_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\r\0\x1A\0 \0%\0*\x000\0chanta cristu\xC3\xB1awpa cristuHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static SV_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\r\0\x1A\0 \0%\0*\x000\0efter Kristusf\xC3\xB6re KristusHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static SQ_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\r\0\x1A\0 \0%\0*\x000\0mbas Krishtitpara KrishtitHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static NE_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\t\0\"\0#\0$\0%\0&\0\xE0\xA4\xB8\xE0\xA4\xA8\xE0\xA5\x8D\xE0\xA4\x88\xE0\xA4\xB8\xE0\xA4\xBE \xE0\xA4\xAA\xE0\xA5\x82\xE0\xA4\xB0\xE0\xA5\x8D\xE0\xA4\xB5HMRST") })
                });
                static NE_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\t\0\"\0(\0-\x002\08\0\xE0\xA4\xB8\xE0\xA4\xA8\xE0\xA5\x8D\xE0\xA4\x88\xE0\xA4\xB8\xE0\xA4\xBE \xE0\xA4\xAA\xE0\xA5\x82\xE0\xA4\xB0\xE0\xA5\x8D\xE0\xA4\xB5HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static KS_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\t\0\x12\0\x13\0\x14\0\x15\0\x16\0\xD8\xA7\xDB\x92 \xDA\x88\xDB\x8C\xD8\xA8\xDB\x8C \xD8\xB3\xDB\x8CHMRST") })
                });
                static FR_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\t\0\x12\0\x13\0\x14\0\x15\0\x16\0ap. J.-C.av. J.-C.HMRST") })
                });
                static CY_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\t\0\x12\0\x18\0\x1D\0\"\0(\0Oed CristCyn CristHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static KS_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\t\0\x12\0\x18\0\x1D\0\"\0(\0\xD8\xA7\xDB\x92 \xDA\x88\xDB\x8C\xD8\xA8\xDB\x8C \xD8\xB3\xDB\x8CHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static FR_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\t\0\x12\0\x18\0\x1D\0\"\0(\0ap. J.-C.av. J.-C.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static HSB_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\t\0\x13\0\x14\0\x15\0\x16\0\x17\0po Chr.n.p\xC5\x99.Chr.n.HMRST") })
                });
                static HSB_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\t\0\x13\0\x19\0\x1E\0#\0)\0po Chr.n.p\xC5\x99.Chr.n.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static PA_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\t\0\x14\0\x15\0\x16\0\x17\0\x18\0\xE0\xA8\xB8\xE0\xA9\xB0\xE0\xA8\xA8\xE0\xA8\x88.\xE0\xA8\xAA\xE0\xA9\x82.HMRST") })
                });
                static SD_DEVA_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\t\0\x15\0\x16\0\x17\0\x18\0\x19\0\xE0\xA4\x8F\xE0\xA4\xA1\xE0\xA5\x80\xE0\xA4\xAC\xE0\xA5\x80\xE0\xA4\xB8\xE0\xA5\x80HMRST") })
                });
                static SD_DEVA_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\t\0\x15\0\x1B\0 \0%\0+\0\xE0\xA4\x8F\xE0\xA4\xA1\xE0\xA5\x80\xE0\xA4\xAC\xE0\xA5\x80\xE0\xA4\xB8\xE0\xA5\x80HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static PA_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\t\0\x15\0\x1B\0 \0%\0+\0\xE0\xA8\xB8\xE0\xA9\xB0\xE0\xA8\xA8\xE0\xA8\x88. \xE0\xA8\xAA\xE0\xA9\x82.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static MR_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\t\0\x1A\0 \0%\0*\x000\0\xE0\xA4\x87. \xE0\xA4\xB8.\xE0\xA4\x88. \xE0\xA4\xB8. \xE0\xA4\xAA\xE0\xA5\x82.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static MR_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\t\0\x1A\0\x1B\0\x1C\0\x1D\0\x1E\0\xE0\xA4\x87. \xE0\xA4\xB8.\xE0\xA4\x88. \xE0\xA4\xB8. \xE0\xA4\xAA\xE0\xA5\x82.HMRST") })
                });
                static ML_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\t\0\x1D\0#\0(\0-\x003\0\xE0\xB4\x8E\xE0\xB4\xA1\xE0\xB4\xBF\xE0\xB4\x95\xE0\xB5\x8D\xE0\xB4\xB0\xE0\xB4\xBF.\xE0\xB4\xAE\xE0\xB5\x81.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static ML_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\t\0\x1D\0\x1E\0\x1F\0 \0!\0\xE0\xB4\x8E\xE0\xB4\xA1\xE0\xB4\xBF\xE0\xB4\x95\xE0\xB5\x8D\xE0\xB4\xB0\xE0\xB4\xBF.\xE0\xB4\xAE\xE0\xB5\x81.HMRST") })
                });
                static EN_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0ABHMRST") })
                });
                static GD_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0ARHMRST") })
                });
                static CY_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0OCHMRST") })
                });
                static EU_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x01\0\x02\0\x03\0\x04\0\x05\0\x06\0oaHMRST") })
                });
                static ID_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x01\0\x03\0\t\0\x0E\0\x13\0\x19\0MSMHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static ID_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x01\0\x03\0\x04\0\x05\0\x06\0\x07\0MSMHMRST") })
                });
                static HR_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x02\0\t\0\n\0\x0B\0\x0C\0\r\0ADpr.n.e.HMRST") })
                });
                static CEB_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x02\0\x04\0\n\0\x0F\0\x14\0\x1A\0ADBCHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static WO_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x02\0\x04\0\n\0\x0F\0\x14\0\x1A\0ADJCHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static GA_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x02\0\x04\0\n\0\x0F\0\x14\0\x1A\0ADRCHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static SW_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x02\0\x04\0\n\0\x0F\0\x14\0\x1A\0BKKKHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static SD_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x02\0\x04\0\n\0\x0F\0\x14\0\x1A\0CDBCHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static KEA_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x02\0\x04\0\n\0\x0F\0\x14\0\x1A\0DKAKHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static CY_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x02\0\x04\0\n\0\x0F\0\x14\0\x1A\0OCCCHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static TO_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x02\0\x04\0\n\0\x0F\0\x14\0\x1A\0TSKMHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static CA_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x02\0\x04\0\n\0\x0F\0\x14\0\x1A\0dCaCHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static FA_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x02\0\x04\0\t\0\n\0\x0C\0\r\0\xD9\x85\xD9\x82\xD9\x87\xE2\x80\x8DM\xD8\xB1ST") })
                });
                static CEB_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x02\0\x04\0\x05\0\x06\0\x07\0\x08\0ADBCHMRST") })
                });
                static WO_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x02\0\x04\0\x05\0\x06\0\x07\0\x08\0ADJCHMRST") })
                });
                static GA_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x02\0\x04\0\x05\0\x06\0\x07\0\x08\0ADRCHMRST") })
                });
                static SW_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x02\0\x04\0\x05\0\x06\0\x07\0\x08\0BKKKHMRST") })
                });
                static SD_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x02\0\x04\0\x05\0\x06\0\x07\0\x08\0CDBCHMRST") })
                });
                static KEA_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x02\0\x04\0\x05\0\x06\0\x07\0\x08\0DKAKHMRST") })
                });
                static TO_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x02\0\x04\0\x05\0\x06\0\x07\0\x08\0TSKMHMRST") })
                });
                static CA_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x02\0\x04\0\x05\0\x06\0\x07\0\x08\0dCaCHMRST") })
                });
                static KO_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x02\0\x04\0\x10\0\x19\0\"\0(\0ADBC\xED\x97\xA4\xEC\x9D\xB4\xEC\x84\xB8\xEC\x9D\xB4\xEB\xA9\x94\xEC\x9D\xB4\xEC\xA7\x80\xEB\xA0\x88\xEC\x9D\xB4\xEC\x99\x80\xEC\x87\xBC\xEC\x99\x80\xEB\x8B\xA4\xEC\x9D\xB4\xEC\x87\xBC") })
                });
                static YO_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x02\0\x05\0\x06\0\x07\0\x08\0\t\0ADBCEHMRST") })
                });
                static UND_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x02\0\x05\0\x06\0\x07\0\x08\0\t\0CEBCEHMRST") })
                });
                static TR_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x02\0\x05\0\x06\0\x07\0\x08\0\t\0MSM\xC3\x96HMRST") })
                });
                static YO_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x02\0\x05\0\x0B\0\x10\0\x15\0\x1B\0ADBCEHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static UND_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x02\0\x05\0\x0B\0\x10\0\x15\0\x1B\0CEBCEHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static VI_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x02\0\x05\0\x0B\0\x10\0\x15\0\x1B\0CNTCNHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static TR_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x02\0\x05\0\x0B\0\x10\0\x15\0\x1B\0MSM\xC3\x96HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static MS_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x02\0\x06\0\x07\0\x08\0\t\0\n\0TMS.M.HMRST") })
                });
                static QU_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x02\0\x06\0\x07\0\x08\0\t\0\n\0dCa.d.HMRST") })
                });
                static MS_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x02\0\x06\0\x0C\0\x11\0\x16\0\x1C\0TMS.M.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static AR_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x02\0\x07\0\x08\0\t\0\n\0\x0B\0\xD9\x85\xD9\x82.\xD9\x85HMRST") })
                });
                static AR_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x02\0\x07\0\x0F\0\x17\0\x1F\0'\0\xD9\x85\xD9\x82.\xD9\x85\xD9\x87\xD9\x8A\xD8\xB3\xD9\x8A\xD9\x85\xD9\x8A\xD8\xAC\xD9\x8A\xD8\xB1\xD9\x8A\xD9\x88\xD8\xA7\xD8\xB4\xD9\x88\xD9\x88\xD8\xA7\xD8\xAA\xD9\x8A\xD8\xB4\xD9\x88") })
                });
                static WO_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x02\0\x08\0\x0E\0\x13\0\x18\0\x1E\0ADav. JCHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static FA_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\t\0\x11\0\x16\0\x1E\0$\0\xD9\x85.\xD9\x82.\xD9\x85.\xD9\x87\xDB\x8C\xD8\xB3\xDB\x8CMeiji\xD8\xB1\xDB\x8C\xD9\x88\xD8\xA7Sh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static PCM_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x06\0\x07\0\x08\0\t\0KIYBKHMRST") })
                });
                static PCM_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1B\0KIYBKHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static DA_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x06\0\x07\0\x08\0\t\0\n\0eKrfKrHMRST") })
                });
                static FI_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x06\0\x07\0\x08\0\t\0\n\0jKreKrHMRST") })
                });
                static ET_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x06\0\x07\0\x08\0\t\0\n\0pKreKrHMRST") })
                });
                static ET_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x06\0\x0C\0\x11\0\x16\0\x1C\0pKreKrHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static PS_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x1F\0 \0!\0\"\0#\0\xD9\x85.\xD9\x84\xD9\x87 \xD9\x85\xDB\x8C\xD9\x84\xD8\xA7\xD8\xAF \xD9\x88\xDA\x93\xD8\xA7\xD9\x86\xD8\xAF\xDB\x90HMRST") })
                });
                static PS_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x1F\0%\0*\0/\x005\0\xD9\x85.\xD9\x84\xD9\x87 \xD9\x85\xDB\x8C\xD9\x84\xD8\xA7\xD8\xAF \xD9\x88\xDA\x93\xD8\xA7\xD9\x86\xD8\xAF\xDB\x90HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static MN_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x04\0\n\0\x0B\0\x0C\0\r\0\x0E\0\xD0\x9C\xD0\xAD\xD0\x9C\xD0\xAD\xD3\xA8HMRST") })
                });
                static PL_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x04\0\n\0\x0B\0\x0C\0\r\0\x0E\0n.e.p.n.e.HMRST") })
                });
                static MN_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x04\0\n\0\x10\0\x15\0\x1A\0 \0\xD0\x9C\xD0\xAD\xD0\x9C\xD0\xAD\xD3\xA8HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static PL_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x04\0\n\0\x10\0\x15\0\x1A\0 \0n.e.p.n.e.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static AZ_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x04\0\t\0\n\0\x0B\0\x0C\0\r\0y.e.e.\xC9\x99.HMRST") })
                });
                static AZ_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x04\0\t\0\x0F\0\x14\0\x19\0\x1F\0y.e.e.\xC9\x99.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static HA_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x04\0\x07\0\r\0\x12\0\x17\0\x1D\0BHAIK.HHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static HA_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x04\0\x07\0\x08\0\t\0\n\0\x0B\0BHAIK.HHMRST") })
                });
                static HU_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x04\0\x07\0\x08\0\t\0\n\0\x0B\0isz.ie.HMRST") })
                });
                static IG_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x04\0\x08\0\t\0\n\0\x0B\0\x0C\0A.K.T.K.HMRST") })
                });
                static YRL_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x04\0\x08\0\t\0\n\0\x0B\0\x0C\0K.a.K.s.HMRST") })
                });
                static ES_419_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x04\0\x08\0\t\0\n\0\x0B\0\x0C\0d.C.a.C.HMRST") })
                });
                static AST_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x04\0\x08\0\t\0\n\0\x0B\0\x0C\0d.C.e.C.HMRST") })
                });
                static IS_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x04\0\x08\0\t\0\n\0\x0B\0\x0C\0e.k.f.k.HMRST") })
                });
                static AF_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x04\0\x08\0\t\0\n\0\x0B\0\x0C\0n.C.v.C.HMRST") })
                });
                static SC_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x04\0\x08\0\t\0\n\0\x0B\0\x0C\0p.C.a.C.HMRST") })
                });
                static AST_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x04\0\x08\0\x0E\0\x13\0\x18\0!\0d.C.e.C.HeiseiMeijiReiwae. Sh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static IG_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x04\0\x08\0\x0E\0\x13\0\x18\0\x1E\0A.K.T.K.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static YRL_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x04\0\x08\0\x0E\0\x13\0\x18\0\x1E\0K.a.K.s.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static EU_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x04\0\x08\0\x0E\0\x13\0\x18\0\x1E\0K.o.K.a.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static ES_419_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x04\0\x08\0\x0E\0\x13\0\x18\0\x1E\0d.C.a.C.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static QU_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x04\0\x08\0\x0E\0\x13\0\x18\0\x1E\0d.C.a.d.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static FI_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x04\0\x08\0\x0E\0\x13\0\x18\0\x1E\0jKr.eKr.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static AF_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x04\0\x08\0\x0E\0\x13\0\x18\0\x1E\0n.C.v.C.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static SC_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x04\0\x08\0\x0E\0\x13\0\x18\0\x1E\0p.C.a.C.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static TK_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x04\0\x0C\0\r\0\x0E\0\x0F\0\x10\0B.e.B.e.\xC3\xB6\xC5\x88HMRST") })
                });
                static CS_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x04\0\x0C\0\r\0\x0E\0\x0F\0\x10\0n.l.p\xC5\x99.n.l.HMRST") })
                });
                static TK_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x04\0\x0C\0\x12\0\x17\0\x1C\0\"\0B.e.B.e.\xC3\xB6\xC5\x88HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static ES_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x05\0\n\0\x0B\0\x0C\0\r\0\x0E\0d. C.a. C.HMRST") })
                });
                static NB_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x05\0\n\0\x0B\0\x0C\0\r\0\x0E\0e.Kr.f.Kr.HMRST") })
                });
                static ES_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x05\0\n\0\x10\0\x15\0\x1A\0 \0d. C.a. C.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static DA_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x05\0\n\0\x10\0\x15\0\x1A\0 \0e.Kr.f.Kr.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static BS_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x05\0\r\0\x13\0\x18\0\x1D\0#\0n. e.p. n. e.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static SR_LATN_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x05\0\r\0\x13\0\x19\0\x1E\0#\0n. e.p. n. e.HaiseiMei\xC4\x91iReiva\xC5\xA0ovaTai\xC5\xA1o") })
                });
                static KGP_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x05\0\t\0\n\0\x0B\0\x0C\0\r\0C.kk.C.j.HMRST") })
                });
                static SQ_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x05\0\t\0\n\0\x0B\0\x0C\0\r\0mb.K.p.K.HMRST") })
                });
                static KGP_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x05\0\t\0\x0F\0\x14\0\x19\0\x1F\0C.kk.C.j.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static SQ_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x05\0\t\0\x0F\0\x14\0\x19\0\x1F\0mb.K.p.K.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static RO_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x05\0\x0B\0\x0C\0\r\0\x0E\0\x0F\0d.Hr.\xC3\xAE.Hr.HMRST") })
                });
                static BS_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x05\0\x0B\0\x0C\0\r\0\x0E\0\x0F\0n. e.p.n.e.HMRST") })
                });
                static RO_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x05\0\x0B\0\x11\0\x16\0\x1B\0!\0d.Hr.\xC3\xAE.Hr.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static LV_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x05\0\x0C\0\r\0\x0E\0\x0F\0\x10\0m.\xC4\x93.p.m.\xC4\x93.HMRST") })
                });
                static LV_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x05\0\x0C\0\x12\0\x17\0\x1C\0\"\0m.\xC4\x93.p.m.\xC4\x93.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static CS_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x05\0\x0F\0\x15\0\x1A\0\x1F\0%\0n. l.p\xC5\x99. n. l.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static LT_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\r\0\x0E\0\x0F\0\x10\0\x11\0po Kr.pr. Kr.HMRST") })
                });
                static RM_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\r\0\x0E\0\x0F\0\x10\0\x11\0s. Cr.av. Cr.HMRST") })
                });
                static SL_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\r\0\x13\0\x18\0\x1D\0#\0po Kr.pr. Kr.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static RM_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\r\0\x13\0\x18\0\x1D\0#\0s. Cr.av. Cr.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static LT_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\r\0\x13\0\x1A\0\x1F\0$\0po Kr.pr. Kr.HeiseiMeid\xC5\xBEiReiwa\xC5\xA0ovaTai\xC5\xA1o") })
                });
                static HU_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x0B\0\x11\0\x16\0\x1B\0!\0i. sz.i. e.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static EL_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x0C\0\r\0\x0E\0\x0F\0\x10\0\xCE\xBC.\xCE\xA7.\xCF\x80.\xCE\xA7.HMRST") })
                });
                static TG_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x0C\0\r\0\x0E\0\x0F\0\x10\0\xD0\x9F\xD0\xB0\xD0\x9C\xD0\x9F\xD0\xB5\xD0\x9CHMRST") })
                });
                static IA_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x0C\0\r\0\x0E\0\x0F\0\x10\0p.Chr.a.Chr.HMRST") })
                });
                static VI_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x0C\0\r\0\x0E\0\x0F\0\x10\0sau CNtr. CNHMRST") })
                });
                static TG_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x0C\0\x12\0\x17\0\x1C\0\"\0\xD0\x9F\xD0\xB0\xD0\x9C\xD0\x9F\xD0\xB5\xD0\x9CHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static NL_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x0C\0\x12\0\x17\0\x1C\0\"\0n.Chr.v.Chr.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static IA_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x0C\0\x12\0\x17\0\x1C\0\"\0p.Chr.a.Chr.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static EL_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x0C\0\x18\0\x1D\0'\0-\0\xCE\xBC.\xCE\xA7.\xCF\x80.\xCE\xA7.\xCE\xA7\xCE\xB5\xCF\x8A\xCF\x83\xCE\xAD\xCE\xB9Meiji\xCE\xA1\xCE\xAD\xCE\xB9\xCE\xB2\xCE\xB1Sh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static SK_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x0E\0\x0F\0\x10\0\x11\0\x12\0po Kr.pred Kr.HMRST") })
                });
                static SK_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x0E\0\x14\0\x19\0\x1E\0$\0po Kr.pred Kr.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static KK_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x0F\0\x10\0\x11\0\x12\0\x13\0\xD0\xB1.\xD0\xB7.\xD0\xB1.\xD0\xB7.\xD0\xB4.HMRST") })
                });
                static KY_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x0F\0\x10\0\x11\0\x12\0\x13\0\xD0\xB1.\xD0\xB7.\xD0\xB1.\xD0\xB7.\xD1\x87.HMRST") })
                });
                static BS_CYRL_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x0F\0\x10\0\x11\0\x12\0\x13\0\xD0\xBD.\xD0\xB5.\xD0\xBF.\xD0\xBD.\xD0\xB5.HMRST") })
                });
                static HY_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x0F\0\x10\0\x11\0\x12\0\x13\0\xD5\xB4.\xD5\xA9.\xD5\xB4.\xD5\xA9.\xD5\xA1.HMRST") })
                });
                static ZH_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x0F\0\x10\0\x11\0\x12\0\x13\0\xE5\x85\xAC\xE5\x85\x83\xE5\x85\xAC\xE5\x85\x83\xE5\x89\x8DHMRST") })
                });
                static YUE_HANS_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x0F\0\x10\0\x11\0\x12\0\x13\0\xE8\xA5\xBF\xE5\x85\x83\xE8\xA5\xBF\xE5\x85\x83\xE5\x89\x8DHMRST") })
                });
                static KK_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x0F\0\x15\0\x1A\0\x1F\0%\0\xD0\xB1.\xD0\xB7.\xD0\xB1.\xD0\xB7.\xD0\xB4.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static KY_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x0F\0\x15\0\x1A\0\x1F\0%\0\xD0\xB1.\xD0\xB7.\xD0\xB1.\xD0\xB7.\xD1\x87.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static MK_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x0F\0\x15\0\x1A\0\x1F\0%\0\xD0\xBD.\xD0\xB5.\xD0\xBF.\xD0\xBD.\xD0\xB5.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static HY_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x0F\0\x15\0\x1A\0\x1F\0%\0\xD5\xB4.\xD5\xA9.\xD5\xB4.\xD5\xA9.\xD5\xA1.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static ZH_HK_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x0F\0\x15\0\x1B\0!\0'\0\xE5\x85\xAC\xE5\x85\x83\xE5\x85\xAC\xE5\x85\x83\xE5\x89\x8D\xE5\xB9\xB3\xE6\x88\x90\xE6\x98\x8E\xE6\xB2\xBB\xE4\xBB\xA4\xE5\x92\x8C\xE6\x98\xAD\xE5\x92\x8C\xE5\xA4\xA7\xE6\xAD\xA3") })
                });
                static YUE_HANS_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x0F\0\x15\0\x1B\0!\0'\0\xE8\xA5\xBF\xE5\x85\x83\xE8\xA5\xBF\xE5\x85\x83\xE5\x89\x8D\xE5\xB9\xB3\xE6\x88\x90\xE6\x98\x8E\xE6\xB2\xBB\xE4\xBB\xA4\xE5\x92\x8C\xE6\x98\xAD\xE5\x92\x8C\xE5\xA4\xA7\xE6\xAD\xA3") })
                });
                static JA_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x0F\0\x15\0\x1B\0!\0'\0\xE8\xA5\xBF\xE6\x9A\xA6\xE7\xB4\x80\xE5\x85\x83\xE5\x89\x8D\xE5\xB9\xB3\xE6\x88\x90\xE6\x98\x8E\xE6\xB2\xBB\xE4\xBB\xA4\xE5\x92\x8C\xE6\x98\xAD\xE5\x92\x8C\xE5\xA4\xA7\xE6\xAD\xA3") })
                });
                static KO_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x0F\0\x1B\0$\0-\x003\0\xEC\x84\x9C\xEA\xB8\xB0\xEA\xB8\xB0\xEC\x9B\x90\xEC\xA0\x84\xED\x97\xA4\xEC\x9D\xB4\xEC\x84\xB8\xEC\x9D\xB4\xEB\xA9\x94\xEC\x9D\xB4\xEC\xA7\x80\xEB\xA0\x88\xEC\x9D\xB4\xEC\x99\x80\xEC\x87\xBC\xEC\x99\x80\xEB\x8B\xA4\xEC\x9D\xB4\xEC\x87\xBC") })
                });
                static UK_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x11\0\x12\0\x13\0\x14\0\x15\0\xD0\xBD.\xD0\xB5.\xD0\xB4\xD0\xBE \xD0\xBD.\xD0\xB5.HMRST") })
                });
                static BE_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x11\0\x12\0\x13\0\x14\0\x15\0\xD0\xBD.\xD1\x8D.\xD0\xB4\xD0\xB0 \xD0\xBD.\xD1\x8D.HMRST") })
                });
                static RU_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x11\0\x12\0\x13\0\x14\0\x15\0\xD0\xBD.\xD1\x8D.\xD0\xB4\xD0\xBE \xD0\xBD.\xD1\x8D.HMRST") })
                });
                static BE_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x11\0\x17\0\x1C\0!\0'\0\xD0\xBD.\xD1\x8D.\xD0\xB4\xD0\xB0 \xD0\xBD.\xD1\x8D.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static GU_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x14\0\x15\0\x16\0\x17\0\x18\0\xE0\xAA\x87\xE0\xAA\xB8\xE0\xAA\x87 \xE0\xAA\xB8 \xE0\xAA\xAA\xE0\xAB\x81HMRST") })
                });
                static ID_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x14\0\x1A\0\x1F\0$\0*\0MasehiSebelum MasehiHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static JV_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x17\0\x1D\0\"\0'\0-\0MasehiSakdurunge MasehiHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static UZ_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x07\0\x0B\0\x0C\0\r\0\x0E\0\x0F\0milodiym.a.HMRST") })
                });
                static UZ_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x07\0\x0B\0\x11\0\x16\0\x1B\0!\0milodiym.a.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static CV_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x07\0\x0E\0\x0F\0\x10\0\x11\0\x12\0\xD1\x85. \xD1\x8D.\xD0\xBF. \xD1\x8D.HMRST") })
                });
                static AM_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x07\0\x0E\0\x0F\0\x10\0\x11\0\x12\0\xE1\x8B\x93/\xE1\x88\x9D\xE1\x8B\x93/\xE1\x8B\x93HMRST") })
                });
                static DE_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x07\0\x0E\0\x0F\0\x10\0\x11\0\x12\0n. Chr.v. Chr.HMRST") })
                });
                static CV_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x07\0\x0E\0\x14\0\x19\0\x1E\0$\0\xD1\x85. \xD1\x8D.\xD0\xBF. \xD1\x8D.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static AM_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x07\0\x0E\0\x14\0\x19\0\x1E\0$\0\xE1\x8B\x93/\xE1\x88\x9D\xE1\x8B\x93/\xE1\x8B\x93HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static DE_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x07\0\x0E\0\x14\0\x19\0\x1E\0$\0n. Chr.v. Chr.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static HR_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x07\0\x0E\0\x14\0\x19\0\x1E\0$\0po. Kr.pr. Kr.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static BS_CYRL_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x07\0\x12\0\x1E\0(\x002\0:\0\xD0\xBD. \xD0\xB5.\xD0\xBF. \xD0\xBD. \xD0\xB5.\xD0\xA5\xD0\xB0\xD0\xB8\xD1\x81\xD0\xB5\xD0\xB8\xD0\x9C\xD0\xB5\xD0\xB8\xD1\x92\xD0\xB8\xD0\xA0\xD0\xB5\xD0\xB8\xD0\xB2\xD0\xB0\xD0\xA8\xD0\xBE\xD0\xB2\xD0\xB0\xD0\xA2\xD0\xB0\xD0\xB8\xD1\x88\xD0\xBE") })
                });
                static RU_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x07\0\x13\0*\0A\0K\0U\0\xD0\xBD. \xD1\x8D.\xD0\xB4\xD0\xBE \xD0\xBD. \xD1\x8D.\xD0\xAD\xD0\xBF\xD0\xBE\xD1\x85\xD0\xB0 \xD0\xA5\xD1\x8D\xD0\xB9\xD1\x81\xD1\x8D\xD0\xB9\xD0\xAD\xD0\xBF\xD0\xBE\xD1\x85\xD0\xB0 \xD0\x9C\xD1\x8D\xD0\xB9\xD0\xB4\xD0\xB7\xD0\xB8\xD0\xA0\xD1\x8D\xD0\xB9\xD0\xB2\xD0\xB0\xD0\xA1\xD1\x8C\xD0\xBE\xD0\xB2\xD0\xB0\xD0\xAD\xD0\xBF\xD0\xBE\xD1\x85\xD0\xB0 \xD0\xA2\xD0\xB0\xD0\xB9\xD1\x81\xD1\x8C\xD0\xBE") })
                });
                static UK_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x07\0\x13\0\x1F\0$\0.\x004\0\xD0\xBD. \xD0\xB5.\xD0\xB4\xD0\xBE \xD0\xBD. \xD0\xB5.\xD0\xA5\xD0\xB5\xD0\xB9\xD1\x81\xD0\xB5\xD0\xB9Meiji\xD0\xA0\xD0\xB5\xD0\xB9\xD0\xB2\xD0\xB0Sh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static UZ_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x07\0\x17\0\x1D\0\"\0'\0-\0milodiymiloddan avvalgiHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static GU_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x08\0\"\0(\0-\x002\08\0\xE0\xAA\x88.\xE0\xAA\xB8.\xE0\xAA\x88.\xE0\xAA\xB8.\xE0\xAA\xAA\xE0\xAB\x82\xE0\xAA\xB0\xE0\xAB\x8D\xE0\xAA\xB5\xE0\xAB\x87HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static SR_LATN_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x08\0\x14\0\x1A\0 \0%\0*\0nove erepre nove ereHaiseiMei\xC4\x91iReiva\xC5\xA0ovaTai\xC5\xA1o") })
                });
                static BS_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x08\0\x16\0\x1C\0!\0&\0,\0nove ereprije nove ereHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static SR_LATN_BA_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x08\0\x16\0\x1C\0\"\0'\0,\0nove ereprije nove ereHaiseiMei\xC4\x91iReiva\xC5\xA0ovaTai\xC5\xA1o") })
                });
                static KM_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x08\0\x1A\0 \0%\0*\x000\0\xE1\x9E\x82.\xE1\x9E\x9F.\xE1\x9E\x98\xE1\x9E\xBB\xE1\x9E\x93 \xE1\x9E\x82.\xE1\x9E\x9F.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static AZ_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x08\0\x1A\0 \0%\0*\x000\0yeni eraeram\xC4\xB1zdan \xC9\x99vv\xC9\x99lHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static KM_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x08\0\x1A\0\x1B\0\x1C\0\x1D\0\x1E\0\xE1\x9E\x82.\xE1\x9E\x9F.\xE1\x9E\x98\xE1\x9E\xBB\xE1\x9E\x93 \xE1\x9E\x82.\xE1\x9E\x9F.HMRST") })
                });
                static LO_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x08\0\x1D\0)\x005\0:\0F\0\xE0\xBA\x84.\xE0\xBA\xAA.\xE0\xBA\x81\xE0\xBB\x88\xE0\xBA\xAD\xE0\xBA\x99 \xE0\xBA\x84.\xE0\xBA\xAA.\xE0\xBA\xAE\xE0\xBA\xB5\xE0\xBA\x8A\xE0\xBA\xB5\xE0\xBA\xA1\xE0\xBA\xB5\xE0\xBA\x88\xE0\xBA\xB5Reiwa\xE0\xBB\x82\xE0\xBA\x8A\xE0\xBA\xA7\xE0\xBA\xB2\xE0\xBB\x84\xE0\xBA\x95\xE0\xBB\x82\xE0\xBA\x8A") })
                });
                static TH_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x08\0\x1D\0)\x005\0A\0M\0\xE0\xB8\x84.\xE0\xB8\xA8.\xE0\xB8\x81\xE0\xB9\x88\xE0\xB8\xAD\xE0\xB8\x99 \xE0\xB8\x84.\xE0\xB8\xA8.\xE0\xB9\x80\xE0\xB8\xAE\xE0\xB9\x80\xE0\xB8\x8B\xE0\xB9\x80\xE0\xB8\xA1\xE0\xB8\x88\xE0\xB8\xB4\xE0\xB9\x80\xE0\xB8\xA3\xE0\xB8\xA7\xE0\xB8\xB0\xE0\xB9\x82\xE0\xB8\x8A\xE0\xB8\xA7\xE0\xB8\xB0\xE0\xB8\x97\xE0\xB8\xB0\xE0\xB8\xAD\xE0\xB8\xB4\xE0\xB9\x82\xE0\xB8\x8A") })
                });
                static TH_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x08\0\x1D\0\x1E\0\x1F\0 \0!\0\xE0\xB8\x84.\xE0\xB8\xA8.\xE0\xB8\x81\xE0\xB9\x88\xE0\xB8\xAD\xE0\xB8\x99 \xE0\xB8\x84.\xE0\xB8\xA8.HMRST") })
                });
                static LO_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x08\0\x1D\0\x1E\0\x1F\0 \0!\0\xE0\xBA\x84.\xE0\xBA\xAA.\xE0\xBA\x81\xE0\xBB\x88\xE0\xBA\xAD\xE0\xBA\x99 \xE0\xBA\x84.\xE0\xBA\xAA.HMRST") })
                });
                static FO_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0B\0\x15\0\x1B\0 \0%\0+\0eftir Kristfyri KristHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static TK_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0B\0\x16\0\x1C\0!\0&\0,\0Isadan so\xC5\x88Isadan \xC3\xB6\xC5\x88HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static IS_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0B\0\x16\0\x1C\0!\0&\0,\0eftir Kristfyrir KristHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static SK_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0B\0\x17\0\x1D\0\"\0'\0-\0po Kristovipred KristomHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static EN_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0B\0\x18\0\x1E\0#\0(\0.\0Anno DominiBefore ChristHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static EN_CA_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0B\0\x18\0\x1E\0#\0(\0.\0Anno Dominibefore ChristHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static IT_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0B\0\x18\0\x1E\0#\0(\0.\0dopo Cristoavanti CristoHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static AF_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0B\0\x18\0\x1E\0#\0(\0.\0na Christusvoor ChristusHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static GA_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0B\0\x19\0\x1F\0$\0)\0/\0Anno DominiRoimh Chr\xC3\xADostHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static SL_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0B\0\x19\0\x1F\0$\0)\0/\0po Kristusupred KristusomHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static LT_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0B\0\x19\0\x1F\0&\0+\x000\0po Kristausprie\xC5\xA1 Krist\xC5\xB3HeiseiMeid\xC5\xBEiReiwa\xC5\xA0ovaTai\xC5\xA1o") })
                });
                static LV_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0B\0\x1C\0\"\0'\0,\x002\0m\xC5\xABsu \xC4\x93r\xC4\x81pirms m\xC5\xABsu \xC4\x93rasHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static CEB_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0B\0\x1F\0%\0*\0/\x005\0Anno DominiSa Wala Pa Si KristoHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static HE_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0C\0!\0'\0,\x001\x007\0\xD7\x9C\xD7\xA1\xD7\xA4\xD7\x99\xD7\xA8\xD7\x94\xD7\x9C\xD7\xA4\xD7\xA0\xD7\x99 \xD7\x94\xD7\xA1\xD7\xA4\xD7\x99\xD7\xA8\xD7\x94HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static AR_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0C\0!\0)\x001\09\0A\0\xD9\x85\xD9\x8A\xD9\x84\xD8\xA7\xD8\xAF\xD9\x8A\xD9\x82\xD8\xA8\xD9\x84 \xD8\xA7\xD9\x84\xD9\x85\xD9\x8A\xD9\x84\xD8\xA7\xD8\xAF\xD9\x87\xD9\x8A\xD8\xB3\xD9\x8A\xD9\x85\xD9\x8A\xD8\xAC\xD9\x8A\xD8\xB1\xD9\x8A\xD9\x88\xD8\xA7\xD8\xB4\xD9\x88\xD9\x88\xD8\xA7\xD8\xAA\xD9\x8A\xD8\xB4\xD9\x88") })
                });
                static MAI_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0C\0%\0&\0'\0(\0)\0\xE0\xA4\x88\xE0\xA4\xB8\xE0\xA4\xB5\xE0\xA5\x80\xE0\xA4\x88\xE0\xA4\xB8\xE0\xA4\xBE-\xE0\xA4\xAA\xE0\xA5\x82\xE0\xA4\xB0\xE0\xA5\x8D\xE0\xA4\xB5HMRST") })
                });
                static MAI_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0C\0%\0+\x000\x005\0;\0\xE0\xA4\x88\xE0\xA4\xB8\xE0\xA4\xB5\xE0\xA5\x80\xE0\xA4\x88\xE0\xA4\xB8\xE0\xA4\xBE-\xE0\xA4\xAA\xE0\xA5\x82\xE0\xA4\xB0\xE0\xA5\x8D\xE0\xA4\xB5HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static SAT_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0C\0(\0)\0*\0+\0,\0\xE1\xB1\xA4\xE1\xB1\xA5\xE1\xB1\xA3\xE1\xB1\xA4\xE1\xB1\xA5\xE1\xB1\xAE\xE1\xB1\xA8\xE1\xB1\xA2\xE1\xB1\x9F \xE1\xB1\x9E\xE1\xB1\x9F\xE1\xB1\xA6\xE1\xB1\x9FHMRST") })
                });
                static SAT_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0C\0(\0.\x003\08\0>\0\xE1\xB1\xA4\xE1\xB1\xA5\xE1\xB1\xA3\xE1\xB1\xA4\xE1\xB1\xA5\xE1\xB1\xAE\xE1\xB1\xA8\xE1\xB1\xA2\xE1\xB1\x9F \xE1\xB1\x9E\xE1\xB1\x9F\xE1\xB1\xA6\xE1\xB1\x9FHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static TT_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0C\0.\x004\09\0>\0D\0\xD0\xBC\xD0\xB8\xD0\xBB\xD0\xB0\xD0\xB4\xD0\xB8\xD0\xB1\xD0\xB5\xD0\xB7\xD0\xBD\xD0\xB5\xD2\xA3 \xD1\x8D\xD1\x80\xD0\xB0\xD0\xB3\xD0\xB0 \xD0\xBA\xD0\xB0\xD0\xB4\xD3\x99\xD1\x80HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static FA_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0C\0\"\0*\0/\x007\0=\0\xD9\x85\xDB\x8C\xD9\x84\xD8\xA7\xD8\xAF\xDB\x8C\xD9\x82\xD8\xA8\xD9\x84 \xD8\xA7\xD8\xB2 \xD9\x85\xDB\x8C\xD9\x84\xD8\xA7\xD8\xAF\xD9\x87\xDB\x8C\xD8\xB3\xDB\x8CMeiji\xD8\xB1\xDB\x8C\xD9\x88\xD8\xA7Sh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static TT_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0C\0\x15\0\x16\0\x17\0\x18\0\x19\0\xD0\xBC\xD0\xB8\xD0\xBB\xD0\xB0\xD0\xB4\xD0\xB8\xD0\xB1.\xD1\x8D.\xD0\xBA.HMRST") })
                });
                static TT_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0C\0\x15\0\x1B\0 \0%\0+\0\xD0\xBC\xD0\xB8\xD0\xBB\xD0\xB0\xD0\xB4\xD0\xB8\xD0\xB1.\xD1\x8D.\xD0\xBA.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static DOI_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0C\0\x17\0\x18\0\x19\0\x1A\0\x1B\0\xE0\xA4\x88\xE0\xA4\xB8\xE0\xA4\xB5\xE0\xA5\x80\xE0\xA4\x88.\xE0\xA4\xAA\xE0\xA5\x82.HMRST") })
                });
                static YO_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0C\0\x17\0\x1D\0\"\0'\0-\0Lehin KristiSaju KristiHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static DOI_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0C\0\x17\0\x1D\0\"\0'\0-\0\xE0\xA4\x88\xE0\xA4\xB8\xE0\xA4\xB5\xE0\xA5\x80\xE0\xA4\x88.\xE0\xA4\xAA\xE0\xA5\x82.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static MY_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0C\0\x18\0\x19\0\x1A\0\x1B\0\x1C\0\xE1\x80\xA1\xE1\x80\x92\xE1\x80\xB1\xE1\x80\xAE\xE1\x80\x98\xE1\x80\xAE\xE1\x80\x85\xE1\x80\xAEHMRST") })
                });
                static KA_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0C\0\x18\0\x19\0\x1A\0\x1B\0\x1C\0\xE1\x83\x90\xE1\x83\xAE. \xE1\x83\xAC.\xE1\x83\xAB\xE1\x83\x95. \xE1\x83\xAC.HMRST") })
                });
                static FF_ADLM_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0C\0\x18\0\x1C\0 \0$\0(\0\xF0\x9E\xA4\x87\xF0\x9E\xA4\x80\xF0\x9E\xA4\x8B\xF0\x9E\xA4\x80\xF0\x9E\xA4\x80\xF0\x9E\xA4\x8B\xF0\x9E\xA4\x96\xF0\x9E\xA4\x83\xF0\x9E\xA4\x88\xF0\x9E\xA4\x85\xF0\x9E\xA4\x9A") })
                });
                static HE_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0C\0\x18\0\x1E\0#\0(\0.\0\xD7\x9C\xD7\xA1\xD7\xA4\xD7\x99\xD7\xA8\xD7\x94\xD7\x9C\xD7\xA4\xD7\xA0\xD7\x94\xD7\xB4\xD7\xA1HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static MY_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0C\0\x18\0\x1E\0#\0(\0.\0\xE1\x80\xA1\xE1\x80\x92\xE1\x80\xB1\xE1\x80\xAE\xE1\x80\x98\xE1\x80\xAE\xE1\x80\x85\xE1\x80\xAEHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static KA_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0C\0\x18\0\x1E\0#\0(\0.\0\xE1\x83\x90\xE1\x83\xAE. \xE1\x83\xAC.\xE1\x83\xAB\xE1\x83\x95. \xE1\x83\xAC.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static IA_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0C\0\x18\0\x1E\0#\0(\0.\0post Christoante ChristoHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static FF_ADLM_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0C\0\x18\x000\0D\0X\0l\0\xF0\x9E\xA4\x87\xF0\x9E\xA4\x80\xF0\x9E\xA4\x8B\xF0\x9E\xA4\x80\xF0\x9E\xA4\x80\xF0\x9E\xA4\x8B\xF0\x9E\xA4\x96\xF0\x9E\xA4\xAB\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xA7\xF0\x9E\xA4\xAB\xF0\x9E\xA4\xB4\xF0\x9E\xA4\x83\xF0\x9E\xA4\xAB\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xB6\xF0\x9E\xA4\xAD\xF0\x9E\xA4\x88\xF0\x9E\xA4\xAB\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xB1\xF0\x9E\xA4\xA2\xF0\x9E\xA4\xA1\xF0\x9E\xA4\xAE\xF0\x9E\xA5\x85\xF0\x9E\xA4\xB1\xF0\x9E\xA4\xA2\xF0\x9E\xA4\x9A\xF0\x9E\xA4\xA2\xF0\x9E\xA4\xB4\xF0\x9E\xA5\x83\xF0\x9E\xA4\xAE\xF0\x9E\xA5\x85") })
                });
                static UZ_CYRL_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0E\0-\x003\08\0=\0C\0\xD0\xBC\xD0\xB8\xD0\xBB\xD0\xBE\xD0\xB4\xD0\xB8\xD0\xB9\xD0\xBC\xD0\xB8\xD0\xBB\xD0\xBE\xD0\xB4\xD0\xB4\xD0\xB0\xD0\xBD \xD0\xB0\xD0\xB2\xD0\xB2\xD0\xB0\xD0\xBB\xD0\xB3\xD0\xB8HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static EU_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0E\0\x12\0\x18\0\x1D\0\"\0(\0Kristo ondorenK.a.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static UZ_CYRL_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0E\0\x14\0\x15\0\x16\0\x17\0\x18\0\xD0\xBC\xD0\xB8\xD0\xBB\xD0\xBE\xD0\xB4\xD0\xB8\xD0\xB9\xD0\xBC.\xD0\xB0.HMRST") })
                });
                static UZ_CYRL_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0E\0\x14\0\x1A\0\x1F\0$\0*\0\xD0\xBC\xD0\xB8\xD0\xBB\xD0\xBE\xD0\xB4\xD0\xB8\xD0\xB9\xD0\xBC.\xD0\xB0.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static IG_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0E\0\x19\0\x1F\0$\0)\0/\0Af\xE1\xBB\x8D Kra\xE1\xBB\x8BstTupu KraistHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static DOI_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0E\0\x19\0\x1F\0$\0)\0/\0\xE0\xA4\x88. \xE0\xA4\xB8\xE0\xA4\xA8\xE0\xA5\x8D\xE0\xA4\x88.\xE0\xA4\xAA\xE0\xA5\x82.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static HR_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0E\0\x1A\0 \0%\0*\x000\0poslije Kristaprije KristaHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static TR_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0E\0\x1C\0\"\0'\0,\x002\0Milattan SonraMilattan \xC3\x96nceHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static TA_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0E\0\x1C\0\"\0'\0,\x002\0\xE0\xAE\x95\xE0\xAE\xBF.\xE0\xAE\xAA\xE0\xAE\xBF.\xE0\xAE\x95\xE0\xAE\xBF.\xE0\xAE\xAE\xE0\xAF\x81.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static TA_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0E\0\x1C\0\x1D\0\x1E\0\x1F\0 \0\xE0\xAE\x95\xE0\xAE\xBF.\xE0\xAE\xAA\xE0\xAE\xBF.\xE0\xAE\x95\xE0\xAE\xBF.\xE0\xAE\xAE\xE0\xAF\x81.HMRST") })
                });
                static YRL_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0E\0\x1E\0$\0)\0.\x004\0Kiristu arir\xC3\xA9Kiristu sen\xC5\xA9d\xC3\xA9HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static TE_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0F\0!\0'\0,\x001\x007\0\xE0\xB0\x95\xE0\xB1\x8D\xE0\xB0\xB0\xE0\xB1\x80\xE0\xB0\xB6\xE0\xB0\x95\xE0\xB1\x8D\xE0\xB0\xB0\xE0\xB1\x80\xE0\xB0\xAA\xE0\xB1\x82HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static TE_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0F\0!\0\"\0#\0$\0%\0\xE0\xB0\x95\xE0\xB1\x8D\xE0\xB0\xB0\xE0\xB1\x80\xE0\xB0\xB6\xE0\xB0\x95\xE0\xB1\x8D\xE0\xB0\xB0\xE0\xB1\x80\xE0\xB0\xAA\xE0\xB1\x82HMRST") })
                });
                static SR_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0F\0%\x001\0;\0E\0M\0\xD0\xBD\xD0\xBE\xD0\xB2\xD0\xB5 \xD0\xB5\xD1\x80\xD0\xB5\xD0\xBF\xD1\x80\xD0\xB5 \xD0\xBD\xD0\xBE\xD0\xB2\xD0\xB5 \xD0\xB5\xD1\x80\xD0\xB5\xD0\xA5\xD0\xB0\xD0\xB8\xD1\x81\xD0\xB5\xD0\xB8\xD0\x9C\xD0\xB5\xD0\xB8\xD1\x92\xD0\xB8\xD0\xA0\xD0\xB5\xD0\xB8\xD0\xB2\xD0\xB0\xD0\xA8\xD0\xBE\xD0\xB2\xD0\xB0\xD0\xA2\xD0\xB0\xD0\xB8\xD1\x88\xD0\xBE") })
                });
                static BN_IN_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0F\0'\0(\0)\0*\0+\0\xE0\xA6\x96\xE0\xA7\x8D\xE0\xA6\xB0\xE0\xA6\xBF\xE0\xA6\x83\xE0\xA6\x96\xE0\xA7\x8D\xE0\xA6\xB0\xE0\xA6\xBF\xE0\xA6\x83\xE0\xA6\xAA\xE0\xA7\x82\xE0\xA6\x83HMRST") })
                });
                static BN_IN_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0F\0'\0-\x002\x007\0=\0\xE0\xA6\x96\xE0\xA7\x8D\xE0\xA6\xB0\xE0\xA6\xBF\xE0\xA6\x83\xE0\xA6\x96\xE0\xA7\x8D\xE0\xA6\xB0\xE0\xA6\xBF\xE0\xA6\x83\xE0\xA6\xAA\xE0\xA7\x82\xE0\xA6\x83HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static HI_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0F\0(\0)\0*\0+\0,\0\xE0\xA4\x88\xE0\xA4\xB8\xE0\xA5\x8D\xE0\xA4\xB5\xE0\xA5\x80\xE0\xA4\x88\xE0\xA4\xB8\xE0\xA4\xBE-\xE0\xA4\xAA\xE0\xA5\x82\xE0\xA4\xB0\xE0\xA5\x8D\xE0\xA4\xB5HMRST") })
                });
                static AS_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0F\0(\0)\0*\0+\0,\0\xE0\xA6\x96\xE0\xA7\x8D\xE0\xA7\xB0\xE0\xA7\x80\xE0\xA6\x83\xE0\xA6\x96\xE0\xA7\x8D\xE0\xA7\xB0\xE0\xA7\x80\xE0\xA6\x83 \xE0\xA6\xAA\xE0\xA7\x82\xE0\xA6\x83HMRST") })
                });
                static AS_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0F\0(\0.\x003\08\0>\0\xE0\xA6\x96\xE0\xA7\x8D\xE0\xA7\xB0\xE0\xA7\x80\xE0\xA6\x83\xE0\xA6\x96\xE0\xA7\x8D\xE0\xA7\xB0\xE0\xA7\x80\xE0\xA6\x83 \xE0\xA6\xAA\xE0\xA7\x82\xE0\xA6\x83HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static HI_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0F\0(\0:\0F\0U\0a\0\xE0\xA4\x88\xE0\xA4\xB8\xE0\xA5\x8D\xE0\xA4\xB5\xE0\xA5\x80\xE0\xA4\x88\xE0\xA4\xB8\xE0\xA4\xBE-\xE0\xA4\xAA\xE0\xA5\x82\xE0\xA4\xB0\xE0\xA5\x8D\xE0\xA4\xB5\xE0\xA4\xB9\xE0\xA5\x87\xE0\xA4\x88\xE0\xA4\xB8\xE0\xA5\x87\xE0\xA4\x88\xE0\xA4\xAE\xE0\xA5\x87\xE0\xA4\x9C\xE0\xA5\x80\xE0\xA4\xB0\xE0\xA5\x87\xE0\xA4\x87\xE0\xA4\xB5\xE0\xA4\xBE\xE0\xA4\xB6\xE0\xA5\x8B\xE0\xA4\xB5\xE0\xA4\xBE\xE0\xA4\xA4\xE0\xA4\xBE\xE0\xA4\x88\xE0\xA4\xB6\xE0\xA5\x8B") })
                });
                static BS_CYRL_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0F\0)\x005\0?\0I\0Q\0\xD0\xBD\xD0\xBE\xD0\xB2\xD0\xB5 \xD0\xB5\xD1\x80\xD0\xB5\xD0\xBF\xD1\x80\xD0\xB8\xD1\x98\xD0\xB5 \xD0\xBD\xD0\xBE\xD0\xB2\xD0\xB5 \xD0\xB5\xD1\x80\xD0\xB5\xD0\xA5\xD0\xB0\xD0\xB8\xD1\x81\xD0\xB5\xD0\xB8\xD0\x9C\xD0\xB5\xD0\xB8\xD1\x92\xD0\xB8\xD0\xA0\xD0\xB5\xD0\xB8\xD0\xB2\xD0\xB0\xD0\xA8\xD0\xBE\xD0\xB2\xD0\xB0\xD0\xA2\xD0\xB0\xD0\xB8\xD1\x88\xD0\xBE") })
                });
                static KS_DEVA_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0F\0+\x001\x006\0;\0A\0\xE0\xA4\x88\xE0\xA4\xB8\xE0\xA5\x8D\xE0\xA4\xB5\xE0\xA5\x80\xE0\xA4\x88\xE0\xA4\xB8\xE0\xA4\xBE \xE0\xA4\xAC\xE0\xA5\x8D\xE0\xA4\xB0\xE0\xA5\x8B\xE0\xA4\x82\xE0\xA4\xA0HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static KGP_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0F\0\x18\0\x1E\0#\0(\0.\0Cristo kar k\xE1\xBB\xB9Cristo joHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static RM_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0F\0\x1C\0\"\0'\0,\x002\0suenter Cristusavant CristusHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static SW_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0F\0\x1E\0$\0)\0.\x004\0Baada ya KristoKabla ya KristoHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static PCM_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x0F\0\x1E\0$\0)\0.\x004\0Kraist Im Yi\xE1\xBA\xB9Bif\xE1\xBB\x8D\xCC\x81 KraistHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static KN_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x10\0#\0$\0%\0&\0'\0\xE0\xB2\x95\xE0\xB3\x8D\xE0\xB2\xB0\xE0\xB2\xBF.\xE0\xB2\xB6\xE0\xB2\x95\xE0\xB3\x8D\xE0\xB2\xB0\xE0\xB2\xBF.\xE0\xB2\xAA\xE0\xB3\x82HMRST") })
                });
                static KN_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x10\0#\0)\0.\x003\09\0\xE0\xB2\x95\xE0\xB3\x8D\xE0\xB2\xB0\xE0\xB2\xBF.\xE0\xB2\xB6\xE0\xB2\x95\xE0\xB3\x8D\xE0\xB2\xB0\xE0\xB2\xBF.\xE0\xB2\xAA\xE0\xB3\x82HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static CHR_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x10\09\0?\0D\0I\0O\0\xE1\x8E\xA0\xE1\x8F\x83 \xE1\x8F\x99\xE1\x8E\xBB\xE1\x8F\x82\xE1\x8F\xA7\xE1\x8F\x93\xE1\x8E\xB7\xE1\x8E\xB8 \xE1\x8E\xA4\xE1\x8E\xB7\xE1\x8E\xAF\xE1\x8F\x8D\xE1\x8F\x97 \xE1\x8E\xA6\xE1\x8E\xB6\xE1\x8F\x81\xE1\x8F\x9BHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static TO_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x10\0\x18\0\x1E\0#\0(\0.\0ta\xCA\xBBu \xCA\xBBo S\xC4\xABs\xC5\xABki mu\xCA\xBBaHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static ET_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x10\0\x1D\0#\0(\0-\x003\0p\xC3\xA4rast Kristustenne KristustHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static PT_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x10\0\x1F\0%\0*\0/\x005\0depois de Cristoantes de CristoHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static KEA_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x10\0\x1F\0%\0*\0/\x005\0dispos di Kristuantis di KristuHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static GL_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x11\0 \0&\0+\x000\x006\0despois de Cristoantes de CristoHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static BR_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x11\0#\0)\0.\x003\09\0goude Jezuz-Krista-raok Jezuz-KristHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static VI_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x11\0&\0,\x001\x006\0<\0Sau C\xC3\xB4ng Nguy\xC3\xAAnTr\xC6\xB0\xE1\xBB\x9Bc Thi\xC3\xAAn Ch\xC3\xBAaHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static UK_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x11\0'\x003\08\0B\0H\0\xD0\xBD\xD0\xB0\xD1\x88\xD0\xBE\xD1\x97 \xD0\xB5\xD1\x80\xD0\xB8\xD0\xB4\xD0\xBE \xD0\xBD\xD0\xB0\xD1\x88\xD0\xBE\xD1\x97 \xD0\xB5\xD1\x80\xD0\xB8\xD0\xA5\xD0\xB5\xD0\xB9\xD1\x81\xD0\xB5\xD0\xB9Meiji\xD0\xA0\xD0\xB5\xD0\xB9\xD0\xB2\xD0\xB0Sh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static CA_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x11\0\x1F\0%\0*\0/\x005\0despr\xC3\xA9s de Cristabans de CristHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static KOK_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x11\x005\0;\0@\0E\0K\0\xE0\xA4\x95\xE0\xA5\x8D\xE0\xA4\xB0\xE0\xA4\xBF.\xE0\xA4\xB6.\xE0\xA4\x95\xE0\xA5\x8D\xE0\xA4\xB0\xE0\xA4\xBF\xE0\xA4\xB8\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\xAA\xE0\xA5\x82\xE0\xA4\xB0\xE0\xA5\x8D\xE0\xA4\xB5HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static KOK_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x11\x005\x006\x007\08\09\0\xE0\xA4\x95\xE0\xA5\x8D\xE0\xA4\xB0\xE0\xA4\xBF.\xE0\xA4\xB6.\xE0\xA4\x95\xE0\xA5\x8D\xE0\xA4\xB0\xE0\xA4\xBF\xE0\xA4\xB8\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\xAA\xE0\xA5\x82\xE0\xA4\xB0\xE0\xA5\x8D\xE0\xA4\xB5HMRST") })
                });
                static ES_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x12\0!\0'\0,\x001\x007\0despu\xC3\xA9s de Cristoantes de CristoHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static AST_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x12\0#\0)\0.\x003\0=\0despu\xC3\xA9s de Cristuenantes de CristuHeiseiMeijiReiwaera Sh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static SC_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x12\0$\0*\0/\x004\0:\0a pustis de Cristuin antis de CristuHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static CS_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x12\0+\x001\x006\0;\0A\0na\xC5\xA1eho letopo\xC4\x8Dtup\xC5\x99ed na\xC5\xA1\xC3\xADm letopo\xC4\x8DtemHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static MR_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x12\x003\09\0>\0C\0I\0\xE0\xA4\x88\xE0\xA4\xB8\xE0\xA4\xB5\xE0\xA5\x80\xE0\xA4\xB8\xE0\xA4\xA8\xE0\xA4\x88\xE0\xA4\xB8\xE0\xA4\xB5\xE0\xA5\x80\xE0\xA4\xB8\xE0\xA4\xA8\xE0\xA4\xAA\xE0\xA5\x82\xE0\xA4\xB0\xE0\xA5\x8D\xE0\xA4\xB5HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static GU_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x12\x007\0=\0B\0G\0M\0\xE0\xAA\x87\xE0\xAA\xB8\xE0\xAA\xB5\xE0\xAB\x80\xE0\xAA\xB8\xE0\xAA\xA8\xE0\xAA\x88\xE0\xAA\xB8\xE0\xAA\xB5\xE0\xAB\x80\xE0\xAA\xB8\xE0\xAA\xA8 \xE0\xAA\xAA\xE0\xAB\x82\xE0\xAA\xB0\xE0\xAB\x8D\xE0\xAA\xB5\xE0\xAB\x87HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static KS_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x13\0&\0,\x001\x006\0<\0\xD8\xA7\xDB\x8C\xD9\x86\xD9\x88 \xDA\x88\xD9\x88\xD9\x85\xD9\x86\xDB\x8C\xD9\x82\xD8\xA8\xD9\x95\xD9\x84 \xD9\x85\xD8\xB3\xDB\x8C\xD9\x96\xD8\xADHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static HI_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x13\0,\0>\0J\0Y\0e\0\xE0\xA4\x88\xE0\xA4\xB8\xE0\xA4\xB5\xE0\xA5\x80 \xE0\xA4\xB8\xE0\xA4\xA8\xE0\xA4\x88\xE0\xA4\xB8\xE0\xA4\xBE-\xE0\xA4\xAA\xE0\xA5\x82\xE0\xA4\xB0\xE0\xA5\x8D\xE0\xA4\xB5\xE0\xA4\xB9\xE0\xA5\x87\xE0\xA4\x88\xE0\xA4\xB8\xE0\xA5\x87\xE0\xA4\x88\xE0\xA4\xAE\xE0\xA5\x87\xE0\xA4\x9C\xE0\xA5\x80\xE0\xA4\xB0\xE0\xA5\x87\xE0\xA4\x87\xE0\xA4\xB5\xE0\xA4\xBE\xE0\xA4\xB6\xE0\xA5\x8B\xE0\xA4\xB5\xE0\xA4\xBE\xE0\xA4\xA4\xE0\xA4\xBE\xE0\xA4\x88\xE0\xA4\xB6\xE0\xA5\x8B") })
                });
                static GD_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x13\0\x1F\0%\0*\0/\x005\0An d\xC3\xA8idh Chr\xC3\xACostaRo Chr\xC3\xACostaHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static FR_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x14\0'\0-\x002\x007\0=\0apr\xC3\xA8s J\xC3\xA9sus-Christavant J\xC3\xA9sus-ChristHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static MNI_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x14\0(\0)\0*\0+\0,\0\xE0\xA6\x96\xE0\xA7\x83: \xE0\xA6\xAE\xE0\xA6\xA4\xE0\xA7\x81\xE0\xA6\x82\xE0\xA6\x96\xE0\xA7\x83: \xE0\xA6\xAE\xE0\xA6\xAE\xE0\xA6\xBE\xE0\xA6\x82HMRST") })
                });
                static HA_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x14\0(\0.\x003\08\0>\0Bayan haihuwar annabKafin haihuwar annabHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static MNI_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x14\0(\0.\x003\08\0>\0\xE0\xA6\x96\xE0\xA7\x83: \xE0\xA6\xAE\xE0\xA6\xA4\xE0\xA7\x81\xE0\xA6\x82\xE0\xA6\x96\xE0\xA7\x83: \xE0\xA6\xAE\xE0\xA6\xAE\xE0\xA6\xBE\xE0\xA6\x82HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static SI_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x14\0+\0,\0-\0.\0/\0\xE0\xB6\x9A\xE0\xB7\x8A\xE2\x80\x8D\xE0\xB6\xBB\xE0\xB7\x92.\xE0\xB7\x80.\xE0\xB6\x9A\xE0\xB7\x8A\xE2\x80\x8D\xE0\xB6\xBB\xE0\xB7\x92.\xE0\xB6\xB4\xE0\xB7\x96.HMRST") })
                });
                static SI_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x14\0+\x001\x006\0;\0A\0\xE0\xB6\x9A\xE0\xB7\x8A\xE2\x80\x8D\xE0\xB6\xBB\xE0\xB7\x92.\xE0\xB7\x80.\xE0\xB6\x9A\xE0\xB7\x8A\xE2\x80\x8D\xE0\xB6\xBB\xE0\xB7\x92.\xE0\xB6\xB4\xE0\xB7\x96.HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static BG_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x15\0,\x002\x007\0<\0B\0\xD1\x81\xD0\xBB\xD0\xB5\xD0\xB4 \xD0\xA5\xD1\x80\xD0\xB8\xD1\x81\xD1\x82\xD0\xB0\xD0\xBF\xD1\x80\xD0\xB5\xD0\xB4\xD0\xB8 \xD0\xA5\xD1\x80\xD0\xB8\xD1\x81\xD1\x82\xD0\xB0HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static AM_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x16\0)\0/\x004\09\0?\0\xE1\x8B\x93\xE1\x88\x98\xE1\x89\xB0 \xE1\x88\x9D\xE1\x88\x95\xE1\x88\xA8\xE1\x89\xB5\xE1\x8B\x93\xE1\x88\x98\xE1\x89\xB0 \xE1\x8B\x93\xE1\x88\x88\xE1\x88\x9DHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static TG_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x16\0,\x002\x007\0<\0B\0\xD0\x9F\xD0\xB0\xD1\x81 \xD0\xB0\xD0\xB7 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xBE\xD0\xB4\xD0\x9F\xD0\xB5\xD1\x88 \xD0\xB0\xD0\xB7 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xBE\xD0\xB4HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static PA_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x16\0/\x005\0:\0?\0E\0\xE0\xA8\x88\xE0\xA8\xB8\xE0\xA8\xB5\xE0\xA9\x80 \xE0\xA8\xB8\xE0\xA9\xB0\xE0\xA8\xA8\xE0\xA8\x88\xE0\xA8\xB8\xE0\xA8\xB5\xE0\xA9\x80 \xE0\xA8\xAA\xE0\xA9\x82\xE0\xA8\xB0\xE0\xA8\xB5HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static TI_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x16\0/\x005\0:\0?\0E\0\xE1\x8B\x93\xE1\x88\x98\xE1\x89\xB0 \xE1\x88\x9D\xE1\x88\x95\xE1\x88\xA8\xE1\x89\xB5\xE1\x89\x85\xE1\x8B\xB5\xE1\x88\x98 \xE1\x8A\xAD\xE1\x88\xAD\xE1\x88\xB5\xE1\x89\xB6\xE1\x88\xB5HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static EL_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x17\0,\08\0=\0G\0M\0\xCE\xBC\xCE\xB5\xCF\x84\xCE\xAC \xCE\xA7\xCF\x81\xCE\xB9\xCF\x83\xCF\x84\xCF\x8C\xCE\xBD\xCF\x80\xCF\x81\xCE\xBF \xCE\xA7\xCF\x81\xCE\xB9\xCF\x83\xCF\x84\xCE\xBF\xCF\x8D\xCE\xA7\xCE\xB5\xCF\x8A\xCF\x83\xCE\xAD\xCE\xB9Meiji\xCE\xA1\xCE\xAD\xCE\xB9\xCE\xB2\xCE\xB1Sh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static MN_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x17\09\0?\0D\0I\0O\0\xD0\xBC\xD0\xB0\xD0\xBD\xD0\xB0\xD0\xB9 \xD1\x8D\xD1\x80\xD0\xB8\xD0\xBD\xD0\xB8\xD0\xB9\xD0\xBC\xD0\xB0\xD0\xBD\xD0\xB0\xD0\xB9 \xD1\x8D\xD1\x80\xD0\xB8\xD0\xBD\xD0\xB8\xD0\xB9 \xD3\xA9\xD0\xBC\xD0\xBD\xD3\xA9\xD1\x85HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static KY_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x17\0=\0C\0H\0M\0S\0\xD0\xB1\xD0\xB8\xD0\xB7\xD0\xB4\xD0\xB8\xD0\xBD \xD0\xB7\xD0\xB0\xD0\xBC\xD0\xB0\xD0\xBD\xD0\xB1\xD0\xB8\xD0\xB7\xD0\xB4\xD0\xB8\xD0\xBD \xD0\xB7\xD0\xB0\xD0\xBC\xD0\xB0\xD0\xBD\xD0\xB3\xD0\xB0 \xD1\x87\xD0\xB5\xD0\xB9\xD0\xB8\xD0\xBDHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static MY_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x18\0R\0X\0]\0b\0h\0\xE1\x80\x81\xE1\x80\x9B\xE1\x80\x85\xE1\x80\xBA\xE1\x80\x94\xE1\x80\xBE\xE1\x80\x85\xE1\x80\xBA\xE1\x80\x81\xE1\x80\x9B\xE1\x80\x85\xE1\x80\xBA\xE1\x80\x90\xE1\x80\xB1\xE1\x80\xAC\xE1\x80\xBA \xE1\x80\x99\xE1\x80\x95\xE1\x80\xB1\xE1\x80\xAB\xE1\x80\xBA\xE1\x80\x99\xE1\x80\xAE\xE1\x80\x94\xE1\x80\xBE\xE1\x80\x85\xE1\x80\xBAHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static MK_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x18\x004\0:\0?\0D\0J\0\xD0\xBE\xD0\xB4 \xD0\xBD\xD0\xB0\xD1\x88\xD0\xB0\xD1\x82\xD0\xB0 \xD0\xB5\xD1\x80\xD0\xB0\xD0\xBF\xD1\x80\xD0\xB5\xD0\xB4 \xD0\xBD\xD0\xB0\xD1\x88\xD0\xB0\xD1\x82\xD0\xB0 \xD0\xB5\xD1\x80\xD0\xB0HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static HSB_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x19\x006\0<\0A\0F\0L\0po Chrystowym narod\xC5\xBAenjup\xC5\x99ed Chrystowym narod\xC5\xBAenjomHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static HU_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x1A\0)\0/\x004\09\0?\0id\xC5\x91sz\xC3\xA1m\xC3\xADt\xC3\xA1sunk szerintKrisztus el\xC5\x91ttHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static DSB_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x1A\x006\0<\0A\0F\0L\0p\xC3\xB3 Kristusowem naro\xC5\xBAenjup\xC5\x9Bed Kristusowym naro\xC5\xBAenimHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static BN_X_4: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x1B\0?\0@\0A\0B\0C\0\xE0\xA6\x96\xE0\xA7\x83\xE0\xA6\xB7\xE0\xA7\x8D\xE0\xA6\x9F\xE0\xA6\xBE\xE0\xA6\xAC\xE0\xA7\x8D\xE0\xA6\xA6\xE0\xA6\x96\xE0\xA7\x8D\xE0\xA6\xB0\xE0\xA6\xBF\xE0\xA6\xB8\xE0\xA7\x8D\xE0\xA6\x9F\xE0\xA6\xAA\xE0\xA7\x82\xE0\xA6\xB0\xE0\xA7\x8D\xE0\xA6\xACHMRST") })
                });
                static KOK_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x1B\0?\0E\0J\0O\0U\0\xE0\xA4\x95\xE0\xA5\x8D\xE0\xA4\xB0\xE0\xA4\xBF\xE0\xA4\xB8\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\xB6\xE0\xA4\x95\xE0\xA4\x95\xE0\xA5\x8D\xE0\xA4\xB0\xE0\xA4\xBF\xE0\xA4\xB8\xE0\xA5\x8D\xE0\xA4\xA4\xE0\xA4\xAA\xE0\xA5\x82\xE0\xA4\xB0\xE0\xA5\x8D\xE0\xA4\xB5HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static BN_X_3: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x1B\0?\0E\0J\0O\0U\0\xE0\xA6\x96\xE0\xA7\x83\xE0\xA6\xB7\xE0\xA7\x8D\xE0\xA6\x9F\xE0\xA6\xBE\xE0\xA6\xAC\xE0\xA7\x8D\xE0\xA6\xA6\xE0\xA6\x96\xE0\xA7\x8D\xE0\xA6\xB0\xE0\xA6\xBF\xE0\xA6\xB8\xE0\xA7\x8D\xE0\xA6\x9F\xE0\xA6\xAA\xE0\xA7\x82\xE0\xA6\xB0\xE0\xA7\x8D\xE0\xA6\xACHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static HY_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x1B\x006\0<\0A\0F\0L\0\xD5\x94\xD6\x80\xD5\xAB\xD5\xBD\xD5\xBF\xD5\xB8\xD5\xBD\xD5\xAB\xD6\x81 \xD5\xB0\xD5\xA5\xD5\xBF\xD5\xB8\xD5\x94\xD6\x80\xD5\xAB\xD5\xBD\xD5\xBF\xD5\xB8\xD5\xBD\xD5\xAB\xD6\x81 \xD5\xA1\xD5\xBC\xD5\xA1\xD5\xBBHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static SD_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x1C\0+\x001\x006\0;\0A\0\xD8\xB9\xD9\x8A\xD8\xB3\xD9\x88\xD9\x8A \xDA\xA9\xD8\xA7\xD9\x86 \xD9\xBE\xD9\x87\xD8\xB1\xD9\x8A\xD9\x86\xD9\x82\xD8\xA8\xD9\x84 \xD9\x85\xD8\xB3\xD9\x8A\xD8\xADHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static KN_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x1C\0A\0G\0L\0Q\0W\0\xE0\xB2\x95\xE0\xB3\x8D\xE0\xB2\xB0\xE0\xB2\xBF\xE0\xB2\xB8\xE0\xB3\x8D\xE0\xB2\xA4 \xE0\xB2\xB6\xE0\xB2\x95\xE0\xB2\x95\xE0\xB3\x8D\xE0\xB2\xB0\xE0\xB2\xBF\xE0\xB2\xB8\xE0\xB3\x8D\xE0\xB2\xA4 \xE0\xB2\xAA\xE0\xB3\x82\xE0\xB2\xB0\xE0\xB3\x8D\xE0\xB2\xB5HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static FI_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x1D\08\0>\0C\0H\0N\0j\xC3\xA4lkeen Kristuksen syntym\xC3\xA4nennen Kristuksen syntym\xC3\xA4\xC3\xA4HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static KK_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x1F\0M\0S\0X\0]\0c\0\xD0\xB1\xD1\x96\xD0\xB7\xD0\xB4\xD1\x96\xD2\xA3 \xD0\xB7\xD0\xB0\xD0\xBC\xD0\xB0\xD0\xBD\xD1\x8B\xD0\xBC\xD1\x8B\xD0\xB7\xD0\x91\xD1\x96\xD0\xB7\xD0\xB4\xD1\x96\xD2\xA3 \xD0\xB7\xD0\xB0\xD0\xBC\xD0\xB0\xD0\xBD\xD1\x8B\xD0\xBC\xD1\x8B\xD0\xB7\xD2\x93\xD0\xB0 \xD0\xB4\xD0\xB5\xD0\xB9\xD1\x96\xD0\xBDHeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static KA_X_5: <icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x03\0\x05\0\x0B\0\x10\0\x15\0\x1A\0bceceheiseimeijireiwashowataisho") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\x007\0n\0t\0y\0~\0\x84\0\xE1\x83\x90\xE1\x83\xAE\xE1\x83\x90\xE1\x83\x9A\xE1\x83\x98 \xE1\x83\xAC\xE1\x83\x94\xE1\x83\x9A\xE1\x83\x97\xE1\x83\x90\xE1\x83\xA6\xE1\x83\xA0\xE1\x83\x98\xE1\x83\xAA\xE1\x83\xAE\xE1\x83\x95\xE1\x83\x98\xE1\x83\x97\xE1\x83\xAB\xE1\x83\x95\xE1\x83\x94\xE1\x83\x9A\xE1\x83\x98 \xE1\x83\xAC\xE1\x83\x94\xE1\x83\x9A\xE1\x83\x97\xE1\x83\x90\xE1\x83\xA6\xE1\x83\xA0\xE1\x83\x98\xE1\x83\xAA\xE1\x83\xAE\xE1\x83\x95\xE1\x83\x98\xE1\x83\x97HeiseiMeijiReiwaSh\xC5\x8DwaTaish\xC5\x8D") })
                });
                static VALUES: [&<icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable; 444usize] = [&AF_X_3, &AF_X_4, &AF_X_5, &AM_X_3, &AM_X_4, &AM_X_5, &AR_X_3, &AR_X_4, &AR_X_5, &AS_X_3, &AS_X_4, &AS_X_5, &AST_X_3, &AST_X_4, &AST_X_5, &AZ_X_3, &AZ_X_4, &AZ_X_5, &BE_X_3, &BE_X_4, &BE_X_5, &BG_X_3, &BG_X_4, &BG_X_5, &BN_IN_X_3, &BN_IN_X_4, &BN_IN_X_5, &BN_X_3, &BN_X_4, &BN_X_5, &BR_X_3, &BR_X_4, &BR_X_5, &BRX_X_3, &BRX_X_4, &BRX_X_5, &BS_CYRL_X_3, &BS_CYRL_X_4, &BS_CYRL_X_5, &BS_X_3, &BS_X_4, &BS_X_5, &CA_X_3, &CA_X_4, &CA_X_5, &CEB_X_3, &CEB_X_4, &CEB_X_5, &CEB_X_3, &CEB_X_4, &CHR_X_5, &CS_X_3, &CS_X_4, &CS_X_5, &CV_X_3, &CV_X_4, &CV_X_5, &CY_X_3, &CY_X_4, &CY_X_5, &DA_X_3, &DA_X_4, &DA_X_5, &DE_X_3, &DE_X_4, &DE_X_3, &DOI_X_3, &DOI_X_4, &DOI_X_5, &DSB_X_3, &DSB_X_4, &DSB_X_5, &EL_X_3, &EL_X_4, &EL_X_5, &EN_CA_X_5, &CEB_X_3, &EN_X_4, &EN_X_5, &ES_419_X_3, &ES_419_X_4, &ES_419_X_3, &ES_419_X_4, &ES_419_X_3, &ES_419_X_4, &ES_419_X_3, &ES_419_X_4, &ES_419_X_3, &ES_419_X_4, &ES_419_X_3, &ES_419_X_4, &ES_419_X_3, &ES_419_X_4, &ES_419_X_3, &ES_419_X_4, &ES_419_X_3, &ES_419_X_4, &ES_419_X_3, &ES_419_X_4, &ES_419_X_3, &ES_419_X_4, &ES_419_X_3, &ES_419_X_4, &ES_419_X_3, &ES_419_X_4, &ES_419_X_3, &ES_419_X_4, &ES_419_X_3, &ES_419_X_4, &ES_419_X_3, &ES_419_X_4, &ES_419_X_3, &ES_419_X_4, &ES_419_X_3, &ES_419_X_4, &ES_419_X_3, &ES_419_X_4, &ES_419_X_3, &ES_419_X_4, &ES_419_X_3, &ES_419_X_4, &ES_419_X_3, &ES_419_X_4, &ES_419_X_3, &ES_419_X_4, &ES_X_3, &ES_X_4, &ES_X_5, &ET_X_3, &ET_X_4, &ET_X_5, &EU_X_3, &EU_X_4, &EU_X_5, &FA_X_3, &FA_X_4, &FA_X_5, &FF_ADLM_X_3, &FF_ADLM_X_4, &FF_ADLM_X_5, &FI_X_3, &FI_X_4, &FI_X_5, &CEB_X_3, &CEB_X_4, &EN_X_5, &DA_X_3, &DA_X_4, &FO_X_5, &FR_X_3, &FR_X_4, &FR_X_5, &GA_X_3, &GA_X_4, &GA_X_5, &GA_X_3, &GD_X_4, &GD_X_5, &ES_419_X_3, &ES_419_X_4, &GL_X_5, &GU_X_3, &GU_X_4, &GU_X_5, &HA_X_3, &HA_X_4, &HA_X_5, &HE_X_3, &HE_X_4, &HE_X_5, &CEB_X_3, &EN_X_4, &EN_X_5, &HI_X_3, &HI_X_4, &HI_X_5, &HR_X_3, &HR_X_4, &HR_X_5, &HSB_X_3, &HSB_X_4, &HSB_X_5, &HU_X_3, &HU_X_4, &HU_X_5, &HY_X_3, &HY_X_4, &HY_X_5, &IA_X_3, &IA_X_4, &IA_X_5, &ID_X_3, &ID_X_4, &ID_X_5, &IG_X_3, &IG_X_4, &IG_X_5, &DA_X_3, &IS_X_4, &IS_X_5, &ES_419_X_3, &CA_X_4, &IT_X_5, &JA_X_3, &CEB_X_4, &JA_X_3, &ID_X_3, &ID_X_4, &JV_X_5, &KA_X_3, &KA_X_4, &KA_X_5, &KEA_X_3, &KEA_X_4, &KEA_X_5, &KGP_X_3, &KGP_X_4, &KGP_X_5, &KK_X_3, &KK_X_4, &KK_X_5, &KM_X_3, &KM_X_4, &KM_X_5, &KN_X_3, &KN_X_4, &KN_X_5, &KO_X_3, &CEB_X_4, &KO_X_5, &KOK_X_3, &KOK_X_4, &KOK_X_5, &CEB_X_3, &CEB_X_4, &KS_DEVA_X_5, &KS_X_3, &KS_X_4, &KS_X_5, &KY_X_3, &KY_X_4, &KY_X_5, &LO_X_3, &LO_X_4, &LO_X_5, &LT_X_3, &LT_X_4, &LT_X_5, &LV_X_3, &LV_X_4, &LV_X_5, &MAI_X_3, &MAI_X_4, &MAI_X_3, &MK_X_3, &BS_CYRL_X_4, &MK_X_5, &ML_X_3, &ML_X_4, &ML_X_5, &MN_X_3, &MN_X_4, &MN_X_5, &MNI_X_3, &MNI_X_4, &MNI_X_3, &MR_X_3, &MR_X_4, &MR_X_5, &MS_X_3, &MS_X_4, &MS_X_3, &MY_X_3, &MY_X_4, &MY_X_5, &DA_X_3, &NB_X_4, &NB_X_5, &NE_X_3, &NE_X_4, &NE_X_3, &NL_X_3, &AF_X_4, &AF_X_5, &DA_X_3, &NB_X_4, &NB_X_5, &DA_X_3, &NB_X_4, &NB_X_5, &CEB_X_3, &CEB_X_4, &OR_X_5, &PA_X_3, &PA_X_4, &PA_X_5, &PCM_X_3, &PCM_X_4, &PCM_X_5, &PL_X_3, &PL_X_4, &PL_X_5, &PS_X_3, &PS_X_4, &PS_X_5, &ES_419_X_3, &ES_419_X_4, &PT_X_5, &QU_X_3, &QU_X_4, &QU_X_5, &RM_X_3, &RM_X_4, &RM_X_5, &RO_X_3, &RO_X_4, &RO_X_5, &RU_X_3, &RU_X_4, &RU_X_5, &SAT_X_3, &SAT_X_4, &SAT_X_3, &SC_X_3, &SC_X_4, &SC_X_5, &SD_DEVA_X_3, &SD_DEVA_X_4, &SD_DEVA_X_3, &SD_X_3, &SD_X_4, &SD_X_5, &SI_X_3, &SI_X_4, &SI_X_5, &SK_X_3, &SK_X_4, &SK_X_5, &SL_X_3, &LT_X_4, &SL_X_5, &CEB_X_3, &EN_X_4, &SO_X_5, &SQ_X_3, &SQ_X_4, &SQ_X_5, &BS_CYRL_X_5, &SR_LATN_BA_X_5, &SR_LATN_X_3, &PL_X_4, &SR_LATN_X_5, &SR_LATN_BA_X_5, &BS_CYRL_X_3, &BS_CYRL_X_4, &SR_X_5, &ID_X_3, &ID_X_4, &ID_X_3, &DA_X_3, &NB_X_4, &SV_X_5, &SW_X_3, &SW_X_4, &SW_X_5, &TA_X_3, &TA_X_4, &TA_X_5, &TE_X_3, &TE_X_4, &TE_X_5, &TG_X_3, &TG_X_4, &TG_X_5, &TH_X_3, &TH_X_4, &TH_X_5, &AM_X_5, &AM_X_3, &AM_X_4, &TI_X_5, &TK_X_3, &TK_X_4, &TK_X_5, &TO_X_3, &TO_X_4, &TO_X_5, &TR_X_3, &TR_X_4, &TR_X_5, &TT_X_3, &TT_X_4, &TT_X_5, &UK_X_3, &UK_X_4, &UK_X_5, &UND_X_3, &UND_X_4, &UND_X_3, &UR_X_3, &UR_X_4, &UR_X_3, &UZ_CYRL_X_3, &UZ_CYRL_X_4, &UZ_CYRL_X_5, &UZ_X_3, &UZ_X_4, &UZ_X_5, &VI_X_3, &VI_X_4, &VI_X_5, &WO_X_3, &WO_X_4, &WO_X_5, &CEB_X_3, &CEB_X_4, &CEB_X_3, &YO_X_3, &YO_X_4, &YO_X_5, &YRL_X_3, &YRL_X_4, &YRL_X_5, &YUE_HANS_X_3, &YUE_HANS_X_4, &YUE_HANS_X_3, &YUE_HANS_X_3, &YUE_HANS_X_4, &YUE_HANS_X_3, &ZH_HK_X_3, &ZH_HK_X_3, &ZH_HK_X_3, &YUE_HANS_X_3, &YUE_HANS_X_3, &YUE_HANS_X_3, &ZH_HK_X_3, &ZH_HK_X_3, &ZH_HK_X_3, &ZH_HK_X_3, &ZH_X_4, &ZH_HK_X_3, &CEB_X_3, &CEB_X_4, &CEB_X_3];
                static KEYS: [&str; 444usize] = ["af-x-3", "af-x-4", "af-x-5", "am-x-3", "am-x-4", "am-x-5", "ar-x-3", "ar-x-4", "ar-x-5", "as-x-3", "as-x-4", "as-x-5", "ast-x-3", "ast-x-4", "ast-x-5", "az-x-3", "az-x-4", "az-x-5", "be-x-3", "be-x-4", "be-x-5", "bg-x-3", "bg-x-4", "bg-x-5", "bn-IN-x-3", "bn-IN-x-4", "bn-IN-x-5", "bn-x-3", "bn-x-4", "bn-x-5", "br-x-3", "br-x-4", "br-x-5", "brx-x-3", "brx-x-4", "brx-x-5", "bs-Cyrl-x-3", "bs-Cyrl-x-4", "bs-Cyrl-x-5", "bs-x-3", "bs-x-4", "bs-x-5", "ca-x-3", "ca-x-4", "ca-x-5", "ceb-x-3", "ceb-x-4", "ceb-x-5", "chr-x-3", "chr-x-4", "chr-x-5", "cs-x-3", "cs-x-4", "cs-x-5", "cv-x-3", "cv-x-4", "cv-x-5", "cy-x-3", "cy-x-4", "cy-x-5", "da-x-3", "da-x-4", "da-x-5", "de-x-3", "de-x-4", "de-x-5", "doi-x-3", "doi-x-4", "doi-x-5", "dsb-x-3", "dsb-x-4", "dsb-x-5", "el-x-3", "el-x-4", "el-x-5", "en-CA-x-5", "en-x-3", "en-x-4", "en-x-5", "es-419-x-3", "es-419-x-4", "es-AR-x-3", "es-AR-x-4", "es-BO-x-3", "es-BO-x-4", "es-BR-x-3", "es-BR-x-4", "es-BZ-x-3", "es-BZ-x-4", "es-CL-x-3", "es-CL-x-4", "es-CO-x-3", "es-CO-x-4", "es-CR-x-3", "es-CR-x-4", "es-CU-x-3", "es-CU-x-4", "es-DO-x-3", "es-DO-x-4", "es-EC-x-3", "es-EC-x-4", "es-GT-x-3", "es-GT-x-4", "es-HN-x-3", "es-HN-x-4", "es-MX-x-3", "es-MX-x-4", "es-NI-x-3", "es-NI-x-4", "es-PA-x-3", "es-PA-x-4", "es-PE-x-3", "es-PE-x-4", "es-PR-x-3", "es-PR-x-4", "es-PY-x-3", "es-PY-x-4", "es-SV-x-3", "es-SV-x-4", "es-US-x-3", "es-US-x-4", "es-UY-x-3", "es-UY-x-4", "es-VE-x-3", "es-VE-x-4", "es-x-3", "es-x-4", "es-x-5", "et-x-3", "et-x-4", "et-x-5", "eu-x-3", "eu-x-4", "eu-x-5", "fa-x-3", "fa-x-4", "fa-x-5", "ff-Adlm-x-3", "ff-Adlm-x-4", "ff-Adlm-x-5", "fi-x-3", "fi-x-4", "fi-x-5", "fil-x-3", "fil-x-4", "fil-x-5", "fo-x-3", "fo-x-4", "fo-x-5", "fr-x-3", "fr-x-4", "fr-x-5", "ga-x-3", "ga-x-4", "ga-x-5", "gd-x-3", "gd-x-4", "gd-x-5", "gl-x-3", "gl-x-4", "gl-x-5", "gu-x-3", "gu-x-4", "gu-x-5", "ha-x-3", "ha-x-4", "ha-x-5", "he-x-3", "he-x-4", "he-x-5", "hi-Latn-x-3", "hi-Latn-x-4", "hi-Latn-x-5", "hi-x-3", "hi-x-4", "hi-x-5", "hr-x-3", "hr-x-4", "hr-x-5", "hsb-x-3", "hsb-x-4", "hsb-x-5", "hu-x-3", "hu-x-4", "hu-x-5", "hy-x-3", "hy-x-4", "hy-x-5", "ia-x-3", "ia-x-4", "ia-x-5", "id-x-3", "id-x-4", "id-x-5", "ig-x-3", "ig-x-4", "ig-x-5", "is-x-3", "is-x-4", "is-x-5", "it-x-3", "it-x-4", "it-x-5", "ja-x-3", "ja-x-4", "ja-x-5", "jv-x-3", "jv-x-4", "jv-x-5", "ka-x-3", "ka-x-4", "ka-x-5", "kea-x-3", "kea-x-4", "kea-x-5", "kgp-x-3", "kgp-x-4", "kgp-x-5", "kk-x-3", "kk-x-4", "kk-x-5", "km-x-3", "km-x-4", "km-x-5", "kn-x-3", "kn-x-4", "kn-x-5", "ko-x-3", "ko-x-4", "ko-x-5", "kok-x-3", "kok-x-4", "kok-x-5", "ks-Deva-x-3", "ks-Deva-x-4", "ks-Deva-x-5", "ks-x-3", "ks-x-4", "ks-x-5", "ky-x-3", "ky-x-4", "ky-x-5", "lo-x-3", "lo-x-4", "lo-x-5", "lt-x-3", "lt-x-4", "lt-x-5", "lv-x-3", "lv-x-4", "lv-x-5", "mai-x-3", "mai-x-4", "mai-x-5", "mk-x-3", "mk-x-4", "mk-x-5", "ml-x-3", "ml-x-4", "ml-x-5", "mn-x-3", "mn-x-4", "mn-x-5", "mni-x-3", "mni-x-4", "mni-x-5", "mr-x-3", "mr-x-4", "mr-x-5", "ms-x-3", "ms-x-4", "ms-x-5", "my-x-3", "my-x-4", "my-x-5", "nb-x-3", "nb-x-4", "nb-x-5", "ne-x-3", "ne-x-4", "ne-x-5", "nl-x-3", "nl-x-4", "nl-x-5", "nn-x-3", "nn-x-4", "nn-x-5", "no-x-3", "no-x-4", "no-x-5", "or-x-3", "or-x-4", "or-x-5", "pa-x-3", "pa-x-4", "pa-x-5", "pcm-x-3", "pcm-x-4", "pcm-x-5", "pl-x-3", "pl-x-4", "pl-x-5", "ps-x-3", "ps-x-4", "ps-x-5", "pt-x-3", "pt-x-4", "pt-x-5", "qu-x-3", "qu-x-4", "qu-x-5", "rm-x-3", "rm-x-4", "rm-x-5", "ro-x-3", "ro-x-4", "ro-x-5", "ru-x-3", "ru-x-4", "ru-x-5", "sat-x-3", "sat-x-4", "sat-x-5", "sc-x-3", "sc-x-4", "sc-x-5", "sd-Deva-x-3", "sd-Deva-x-4", "sd-Deva-x-5", "sd-x-3", "sd-x-4", "sd-x-5", "si-x-3", "si-x-4", "si-x-5", "sk-x-3", "sk-x-4", "sk-x-5", "sl-x-3", "sl-x-4", "sl-x-5", "so-x-3", "so-x-4", "so-x-5", "sq-x-3", "sq-x-4", "sq-x-5", "sr-BA-x-5", "sr-Latn-BA-x-5", "sr-Latn-x-3", "sr-Latn-x-4", "sr-Latn-x-5", "sr-ME-x-5", "sr-x-3", "sr-x-4", "sr-x-5", "su-x-3", "su-x-4", "su-x-5", "sv-x-3", "sv-x-4", "sv-x-5", "sw-x-3", "sw-x-4", "sw-x-5", "ta-x-3", "ta-x-4", "ta-x-5", "te-x-3", "te-x-4", "te-x-5", "tg-x-3", "tg-x-4", "tg-x-5", "th-x-3", "th-x-4", "th-x-5", "ti-ER-x-5", "ti-x-3", "ti-x-4", "ti-x-5", "tk-x-3", "tk-x-4", "tk-x-5", "to-x-3", "to-x-4", "to-x-5", "tr-x-3", "tr-x-4", "tr-x-5", "tt-x-3", "tt-x-4", "tt-x-5", "uk-x-3", "uk-x-4", "uk-x-5", "und-x-3", "und-x-4", "und-x-5", "ur-x-3", "ur-x-4", "ur-x-5", "uz-Cyrl-x-3", "uz-Cyrl-x-4", "uz-Cyrl-x-5", "uz-x-3", "uz-x-4", "uz-x-5", "vi-x-3", "vi-x-4", "vi-x-5", "wo-x-3", "wo-x-4", "wo-x-5", "xh-x-3", "xh-x-4", "xh-x-5", "yo-x-3", "yo-x-4", "yo-x-5", "yrl-x-3", "yrl-x-4", "yrl-x-5", "yue-Hans-x-3", "yue-Hans-x-4", "yue-Hans-x-5", "yue-x-3", "yue-x-4", "yue-x-5", "zh-HK-x-3", "zh-HK-x-4", "zh-HK-x-5", "zh-Hant-x-3", "zh-Hant-x-4", "zh-Hant-x-5", "zh-MO-x-3", "zh-MO-x-4", "zh-MO-x-5", "zh-x-3", "zh-x-4", "zh-x-5", "zu-x-3", "zu-x-4", "zu-x-5"];
                let mut metadata = icu_provider::DataResponseMetadata::default();
                let payload = if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    payload
                } else {
                    const FALLBACKER: icu::locid_transform::fallback::LocaleFallbackerWithConfig<'static> = icu::locid_transform::fallback::LocaleFallbacker::new().for_config(<icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.fallback_config());
                    let mut fallback_iterator = FALLBACKER.fallback_for(req.locale.clone());
                    loop {
                        if fallback_iterator.get().is_und() {
                            return Err(icu_provider::DataErrorKind::MissingLocale.with_req(<icu::datetime::provider::neo::JapaneseYearSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY, req));
                        }
                        if let Ok(payload) = KEYS.binary_search_by(|k| fallback_iterator.get().strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                            metadata.locale = Some(fallback_iterator.take());
                            break payload;
                        }
                        fallback_iterator.step();
                    }
                };
                Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(payload)), metadata })
            }
        }
    };
}