// @generated
/// Implement `DataProvider<RocYearSymbolsV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_datetime_symbols_roc_years_v1 {
    ($ provider : ty) => {
        #[clippy::msrv = "1.67"]
        const _: () = <$provider>::MUST_USE_MAKE_PROVIDER_MACRO;
        #[clippy::msrv = "1.67"]
        impl icu_provider::DataProvider<icu::datetime::provider::neo::RocYearSymbolsV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::datetime::provider::neo::RocYearSymbolsV1Marker>, icu_provider::DataError> {
                static HE_X_5: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0+\0\xD7\x9C\xD7\xA1\xD7\xA4\xD7\x99\xD7\xA8\xD7\xAA \xD7\x94\xD7\xA8\xD7\xA4\xD7\x95\xD7\x91\xD7\x9C\xD7\x99\xD7\xA7\xD7\x94 \xD7\xA9\xD7\x9C \xD7\xA1\xD7\x99\xD7\x9F\xD7\x9C\xD7\xA4\xD7\xA0\xD7\x99 \xD7\x94\xD7\xA8\xD7\xA4\xD7\x95\xD7\x91\xD7\x9C\xD7\x99\xD7\xA7\xD7\x94 \xD7\xA9\xD7\x9C \xD7\xA1\xD7\x99\xD7\x9F") })
                });
                static BR_X_5: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0Republik Sinaa-raok Republik Sina") })
                });
                static SR_LATN_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0RKPre RK") })
                });
                static CA_X_5: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0ROCAbans de ROC") })
                });
                static PL_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0ROCPrzed ROC") })
                });
                static CS_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0ROCp\xC5\x99ed ROC") })
                });
                static SK_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0ROCpred ROC") })
                });
                static PL_X_4: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0ROCprzed ROC") })
                });
                static FR_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0RdCav. RdC") })
                });
                static FR_X_5: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0RdCavant RdC") })
                });
                static RO_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0R.C.\xC3\xAE.R.C.") })
                });
                static SV_X_4: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0R.K.f.R.K.") })
                });
                static SV_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0R.K.f\xC3\xB6re R.K.") })
                });
                static BR_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0R.S.a-raok R.S.") })
                });
                static BS_CYRL_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0\xD0\xA0\xD0\x9A\xD0\x9F\xD1\x80\xD0\xB5 \xD0\xA0\xD0\x9A") })
                });
                static SC_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x05\0R.d.Ta.R.d.T.") })
                });
                static LV_X_5: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0Mi\xC5\x86gopirms \xC4\xB6\xC4\xABnas Republikas dibin\xC4\x81\xC5\xA1anas") })
                });
                static LV_X_4: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0Mi\xC5\x86gopirms rep.") })
                });
                static LV_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0Mi\xC5\x86gopirms republikas") })
                });
                static AST_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0MinguoA.R.D.C.") })
                });
                static PT_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0MinguoAntes da R.C.") })
                });
                static EN_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0MinguoB.R.O.C.") })
                });
                static FIL_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0MinguoBago ang R.O.C.") })
                });
                static DE_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0MinguoBefore R.O.C.") })
                });
                static NB_X_5: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0MinguoF\xC3\xB8r R.O.C.") })
                });
                static NB_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0MinguoF\xC3\xB8r ROC") })
                });
                static SO_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0MinguoKahor R.O.C.") })
                });
                static IT_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0MinguoPrima di R.O.C.") })
                });
                static TR_X_5: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0MinguoR.O.C. \xC3\x96ncesi") })
                });
                static AST_X_5: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0Minguoantes de la R.D.C.") })
                });
                static FI_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0Minguoe. Kiinan tasav.") })
                });
                static FI_X_5: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0Minguoennen Kiinan tasavaltaa") })
                });
                static SV_X_5: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0Minguof\xC3\xB6re Republiken Kina") })
                });
                static DA_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0Minguof\xC3\xB8r R.O.C.") })
                });
                static IS_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0Minguofyrir l\xC3\xBD\xC3\xB0v. K\xC3\xADna") })
                });
                static IS_X_5: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0Minguofyrir l\xC3\xBD\xC3\xB0veldi K\xC3\xADna") })
                });
                static IS_X_4: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0Minguofyrir lv.K.") })
                });
                static DE_X_4: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0Minguov. VR China") })
                });
                static NL_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0Minguovoor R.O.C.") })
                });
                static DE_X_5: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0Minguovor Volksrepublik China") })
                });
                static UND_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0R.O.C.Before R.O.C.") })
                });
                static LT_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0R.O.C.Prie\xC5\xA1 R.O.C.") })
                });
                static TA_X_5: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0R.O.C.R.O.C. -\xE0\xAE\x95\xE0\xAF\x8D\xE0\xAE\x95\xE0\xAF\x81 \xE0\xAE\xAE\xE0\xAF\x81\xE0\xAE\xA9\xE0\xAF\x8D\xE0\xAE\xAA\xE0\xAF\x81") })
                });
                static TE_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0R.O.C.R.O.C. \xE0\xB0\xAA\xE0\xB1\x82\xE0\xB0\xB0\xE0\xB1\x8D\xE0\xB0\xB5\xE0\xB0\x82") })
                });
                static EU_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0R.O.C.R.O.C. aurretik") })
                });
                static HU_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0R.O.C.R.O.C. el\xC5\x91tt") })
                });
                static TA_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0R.O.C.ROC\xE0\xAE\x95\xE0\xAF\x8D\xE0\xAE\x95\xE0\xAF\x81 \xE0\xAE\xAE\xE0\xAF\x81\xE0\xAE\xA9\xE0\xAF\x8D") })
                });
                static MS_X_4: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0R.O.C.Sblm R.O.C") })
                });
                static MS_X_5: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0R.O.C.Sebelum R.O.C") })
                });
                static ID_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0R.O.C.Sebelum R.O.C.") })
                });
                static VI_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0R.O.C.Tr\xC6\xB0\xE1\xBB\x9Bc R.O.C") })
                });
                static EL_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0R.O.C.\xCF\x80\xCF\x81\xCE\xBF R.O.C.") })
                });
                static HE_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0R.O.C.\xD7\x9C\xD7\xA4\xD7\xA0\xD7\x99 R.O.C") })
                });
                static LO_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0R.O.C.\xE0\xBA\x81\xE0\xBB\x88\xE0\xBA\xAD\xE0\xBA\x99 R.O.C.") })
                });
                static ES_419_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0R.O.C.antes de R.O.C.") })
                });
                static BS_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0R.O.C.prije R.O.C.") })
                });
                static SC_X_5: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0R.d.T.in antis de sa R.d.T.") })
                });
                static JA_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0\xE6\xB0\x91\xE5\x9B\xBD\xE6\xB0\x91\xE5\x9B\xBD\xE5\x89\x8D") })
                });
                static YUE_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0\xE6\xB0\x91\xE5\x9C\x8B\xE6\xB0\x91\xE5\x9C\x8B\xE5\x89\x8D") })
                });
                static ES_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0minguoantes de RDC") })
                });
                static GD_X_5: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x08\0M\xC3\xADngu\xC3\xB3Ro Ph. na S\xC3\xACne") })
                });
                static GD_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x08\0M\xC3\xADngu\xC3\xB3Ro PnS") })
                });
                static RU_X_5: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x0C\0\xD0\x9C\xD0\xB8\xD0\xBD\xD1\x8C\xD0\xB3\xD0\xBE\xD0\xB4\xD0\xBE \xD0\xBE\xD1\x81\xD0\xBD\xD0\xBE\xD0\xB2\xD0\xB0\xD0\xBD\xD0\xB8\xD1\x8F \xD0\x9A\xD0\xB8\xD1\x82\xD0\xB0\xD0\xB9\xD1\x81\xD0\xBA\xD0\xBE\xD0\xB9 \xD1\x80\xD0\xB5\xD1\x81\xD0\xBF\xD1\x83\xD0\xB1\xD0\xBB\xD0\xB8\xD0\xBA\xD0\xB8") })
                });
                static RU_X_4: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x0C\0\xD0\x9C\xD0\xB8\xD0\xBD\xD1\x8C\xD0\xB3\xD0\xBE\xD0\xB4\xD0\xBE \xD1\x80\xD0\xB5\xD1\x81\xD0\xBF.") })
                });
                static MK_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x0C\0\xD0\xBC\xD0\xB8\xD0\xBD\xD0\xB3\xD1\x83\xD0\xBE\xD0\xBF\xD1\x80. \xD0\xA0.\xD0\x9A.") })
                });
                static MK_X_5: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x0C\0\xD0\xBC\xD0\xB8\xD0\xBD\xD0\xB3\xD1\x83\xD0\xBE\xD0\xBF\xD1\x80\xD0\xB5\xD0\xB4 \xD0\xA0.\xD0\x9A.") })
                });
                static PA_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x0C\0\xE0\xA8\xAE\xE0\xA8\xBF\xE0\xA9\xB0\xE0\xA8\x97\xE0\xA8\x86\xE0\xA8\xB0.\xE0\xA8\x93.\xE0\xA8\xB8\xE0\xA9\x80 \xE0\xA8\xA4\xE0\xA9\x8B\xE0\xA8\x82 \xE0\xA8\xAA\xE0\xA8\xB9\xE0\xA8\xBF\xE0\xA8\xB2\xE0\xA8\xBE\xE0\xA8\x82") })
                });
                static PA_X_5: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x0C\0\xE0\xA8\xAE\xE0\xA8\xBF\xE0\xA9\xB0\xE0\xA8\x97\xE0\xA8\x9A\xE0\xA9\x80\xE0\xA8\xA8 \xE0\xA8\xA6\xE0\xA9\x87 \xE0\xA8\x97\xE0\xA8\xA3\xE0\xA8\xB0\xE0\xA8\xBE\xE0\xA8\x9C \xE0\xA8\xA4\xE0\xA9\x8B\xE0\xA8\x82 \xE0\xA8\xAA\xE0\xA8\xB9\xE0\xA8\xBF\xE0\xA8\xB2\xE0\xA8\xBE\xE0\xA8\x82") })
                });
                static KO_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x0C\0\xEC\xA4\x91\xED\x99\x94\xEB\xAF\xBC\xEA\xB5\xAD\xEC\xA4\x91\xED\x99\x94\xEB\xAF\xBC\xEA\xB5\xAD\xEC\xA0\x84") })
                });
                static SL_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x0E\0Minguo koledarpred RK") })
                });
                static RO_X_5: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x0F\0Republica China\xC3\xAEnainte de Republica China") })
                });
                static MR_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x0F\0\xE0\xA4\xAE\xE0\xA4\xBF\xE0\xA4\x82\xE0\xA4\x97\xE0\xA5\x82\xE0\xA4\x86\xE0\xA4\xB0.\xE0\xA4\x93.\xE0\xA4\xB8\xE0\xA5\x80. \xE0\xA4\x86\xE0\xA4\xA7\xE0\xA5\x80") })
                });
                static KN_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x0F\0\xE0\xB2\xAE\xE0\xB2\xBF\xE0\xB2\x82\xE0\xB2\x97\xE0\xB3\x8B\xE0\xB2\x86\xE0\xB2\xB0\xE0\xB3\x8D.\xE0\xB2\x93.\xE0\xB2\xB8\xE0\xB2\xBF.\xE0\xB2\x97\xE0\xB3\x86 \xE0\xB2\xAE\xE0\xB3\x81\xE0\xB2\x82\xE0\xB2\x9A\xE0\xB3\x86") })
                });
                static GU_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x12\0\xE0\xAA\x86\xE0\xAA\xB0.\xE0\xAA\x93.\xE0\xAA\xB8\xE0\xAB\x80.\xE0\xAA\x86\xE0\xAA\xB0.\xE0\xAA\x93.\xE0\xAA\xB8\xE0\xAB\x80. \xE0\xAA\xAA\xE0\xAA\xB9\xE0\xAB\x87\xE0\xAA\xB2\xE0\xAA\xBE\xE0\xAA\x82") })
                });
                static FA_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x15\0\xD8\xAA\xD9\x82\xD9\x88\xDB\x8C\xD9\x85 \xD9\x85\xDB\x8C\xD9\x86\xDA\xAF\xD9\x88\xD9\x82\xD8\xA8\xD9\x84 \xD8\xA7\xD8\xB2 R.O.C.") })
                });
                static UR_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x15\0\xD8\xAC\xD9\x85\xDB\x81\xD9\x88\xD8\xB1\xDB\x8C\xDB\x81 \xDA\x86\xDB\x8C\xD9\x86\xD9\x82\xD8\xA8\xD9\x84 \xD8\xA7\xD8\xB2 \xD8\xAC\xD9\x85\xDB\x81\xD9\x88\xD8\xB1\xDB\x8C\xDB\x81 \xDA\x86\xDB\x8C\xD9\x86") })
                });
                static ML_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x15\0\xE0\xB4\xAE\xE0\xB4\xBF\xE0\xB4\x82\xE0\xB4\x97\xE0\xB5\x8D\xE0\xB4\xB5\xE0\xB5\x8BR.O.C-\xE0\xB4\xAF\xE0\xB5\x8D\xE2\x80\x8C\xE0\xB4\x95\xE0\xB5\x8D\xE0\xB4\x95\xE0\xB5\x8D \xE0\xB4\xAE\xE0\xB5\x81.") })
                });
                static ML_X_5: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x15\0\xE0\xB4\xAE\xE0\xB4\xBF\xE0\xB4\x82\xE0\xB4\x97\xE0\xB5\x8D\xE0\xB4\xB5\xE0\xB5\x8BR.O.C-\xE0\xB4\xAF\xE0\xB5\x8D\xE2\x80\x8C\xE0\xB4\x95\xE0\xB5\x8D\xE0\xB4\x95\xE0\xB5\x8D \xE0\xB4\xAE\xE0\xB5\x81\xE0\xB4\xAE\xE0\xB5\x8D\xE0\xB4\xAA\xE0\xB5\x8D") })
                });
                static TH_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x15\0\xE0\xB9\x84\xE0\xB8\x95\xE0\xB9\x89\xE0\xB8\xAB\xE0\xB8\xA7\xE0\xB8\xB1\xE0\xB8\x99\xE0\xB8\x9B\xE0\xB8\xB5\xE0\xB8\x81\xE0\xB9\x88\xE0\xB8\xAD\xE0\xB8\x99\xE0\xB9\x84\xE0\xB8\x95\xE0\xB9\x89\xE0\xB8\xAB\xE0\xB8\xA7\xE0\xB8\xB1\xE0\xB8\x99") })
                });
                static AR_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x17\0\xD8\xAC\xD9\x85\xD9\x87\xD9\x88\xD8\xB1\xD9\x8A\xD8\xA9 \xD8\xA7\xD9\x84\xD8\xB5\xD9\x8ABefore R.O.C.") })
                });
                static FF_ADLM_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x18\0\xF0\x9E\xA4\x83\xF0\x9E\xA4\xAD\xF0\x9E\xA4\xB2\xF0\x9E\xA4\xBA\xF0\x9E\xA4\xB5\xF0\x9E\xA4\xAE\xF0\x9E\xA4\x80\xF0\x9E\xA5\x8B\xF0\x9E\xA4\x81\xF0\x9E\xA4\x95") })
                });
                static BN_X_3: <icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0rocroc-inverse") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x1B\0\xE0\xA6\xAE\xE0\xA6\xBF\xE0\xA6\x99\xE0\xA7\x8D\xE0\xA6\x97\xE0\xA7\x81\xE0\xA6\xAF\xE0\xA6\xBC\xE0\xA6\xBE\xE0\xA6\x86\xE0\xA6\x97\xE0\xA7\x87 R.O.C.") })
                });
                static VALUES: [&<icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable; 261usize] = [&AR_X_3, &AR_X_3, &AR_X_3, &AST_X_3, &AST_X_3, &AST_X_5, &BN_X_3, &BN_X_3, &BN_X_3, &BR_X_3, &BR_X_3, &BR_X_5, &BS_CYRL_X_3, &BS_CYRL_X_3, &BS_CYRL_X_3, &BS_X_3, &BS_X_3, &BS_X_3, &CA_X_5, &CS_X_3, &CS_X_3, &CS_X_3, &DA_X_3, &DA_X_3, &DA_X_3, &DE_X_3, &DE_X_4, &DE_X_5, &EL_X_3, &EL_X_3, &EL_X_3, &EN_X_3, &EN_X_3, &EN_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_419_X_3, &ES_X_3, &ES_X_3, &ES_X_3, &EU_X_3, &EU_X_3, &EU_X_3, &FA_X_3, &FA_X_3, &FA_X_3, &FF_ADLM_X_3, &FF_ADLM_X_3, &FF_ADLM_X_3, &FI_X_3, &FI_X_3, &FI_X_5, &FIL_X_3, &FIL_X_3, &FIL_X_3, &FR_X_3, &FR_X_3, &FR_X_5, &GD_X_3, &GD_X_3, &GD_X_5, &GU_X_3, &GU_X_3, &GU_X_3, &HE_X_3, &HE_X_3, &HE_X_5, &EN_X_3, &EN_X_3, &EN_X_3, &BS_X_3, &BS_X_3, &BS_X_3, &HU_X_3, &HU_X_3, &HU_X_3, &ID_X_3, &ID_X_3, &ID_X_3, &IS_X_3, &IS_X_4, &IS_X_5, &IT_X_3, &IT_X_3, &IT_X_3, &JA_X_3, &JA_X_3, &JA_X_3, &KN_X_3, &KN_X_3, &KN_X_3, &KO_X_3, &KO_X_3, &KO_X_3, &LO_X_3, &LO_X_3, &LO_X_3, &LT_X_3, &LT_X_3, &LT_X_3, &LV_X_3, &LV_X_4, &LV_X_5, &MK_X_3, &MK_X_3, &MK_X_5, &ML_X_3, &ML_X_3, &ML_X_5, &MR_X_3, &MR_X_3, &MR_X_3, &MS_X_4, &MS_X_5, &NB_X_3, &NB_X_3, &NB_X_5, &NL_X_3, &NL_X_3, &NL_X_3, &NB_X_3, &NB_X_3, &NB_X_5, &NB_X_3, &NB_X_3, &NB_X_5, &PA_X_3, &PA_X_3, &PA_X_5, &PL_X_3, &PL_X_4, &PL_X_4, &PT_X_3, &PT_X_3, &PT_X_3, &RO_X_3, &RO_X_3, &RO_X_5, &DE_X_3, &RU_X_4, &RU_X_5, &SC_X_3, &SC_X_3, &SC_X_5, &SK_X_3, &SK_X_3, &SK_X_3, &SL_X_3, &SL_X_3, &SL_X_3, &SO_X_3, &SO_X_3, &SO_X_3, &SR_LATN_X_3, &SR_LATN_X_3, &SR_LATN_X_3, &BS_CYRL_X_3, &BS_CYRL_X_3, &BS_CYRL_X_3, &SV_X_3, &SV_X_4, &SV_X_5, &TA_X_3, &TA_X_3, &TA_X_5, &TE_X_3, &TE_X_3, &TE_X_3, &TH_X_3, &TH_X_3, &TH_X_3, &DE_X_3, &DE_X_3, &TR_X_5, &UND_X_3, &UND_X_3, &UND_X_3, &UR_X_3, &UR_X_3, &UR_X_3, &VI_X_3, &VI_X_3, &VI_X_3, &JA_X_3, &JA_X_3, &JA_X_3, &YUE_X_3, &YUE_X_3, &YUE_X_3, &YUE_X_3, &YUE_X_3, &YUE_X_3, &JA_X_3, &JA_X_3, &JA_X_3];
                static KEYS: [&str; 261usize] = ["ar-x-3", "ar-x-4", "ar-x-5", "ast-x-3", "ast-x-4", "ast-x-5", "bn-x-3", "bn-x-4", "bn-x-5", "br-x-3", "br-x-4", "br-x-5", "bs-Cyrl-x-3", "bs-Cyrl-x-4", "bs-Cyrl-x-5", "bs-x-3", "bs-x-4", "bs-x-5", "ca-x-5", "cs-x-3", "cs-x-4", "cs-x-5", "da-x-3", "da-x-4", "da-x-5", "de-x-3", "de-x-4", "de-x-5", "el-x-3", "el-x-4", "el-x-5", "en-x-3", "en-x-4", "en-x-5", "es-419-x-3", "es-419-x-4", "es-419-x-5", "es-AR-x-3", "es-AR-x-4", "es-AR-x-5", "es-BO-x-3", "es-BO-x-4", "es-BO-x-5", "es-BR-x-3", "es-BR-x-4", "es-BR-x-5", "es-BZ-x-3", "es-BZ-x-4", "es-BZ-x-5", "es-CL-x-3", "es-CL-x-4", "es-CL-x-5", "es-CO-x-3", "es-CO-x-4", "es-CO-x-5", "es-CR-x-3", "es-CR-x-4", "es-CR-x-5", "es-CU-x-3", "es-CU-x-4", "es-CU-x-5", "es-DO-x-3", "es-DO-x-4", "es-DO-x-5", "es-EC-x-3", "es-EC-x-4", "es-EC-x-5", "es-GT-x-3", "es-GT-x-4", "es-GT-x-5", "es-HN-x-3", "es-HN-x-4", "es-HN-x-5", "es-MX-x-3", "es-MX-x-4", "es-MX-x-5", "es-NI-x-3", "es-NI-x-4", "es-NI-x-5", "es-PA-x-3", "es-PA-x-4", "es-PA-x-5", "es-PE-x-3", "es-PE-x-4", "es-PE-x-5", "es-PR-x-3", "es-PR-x-4", "es-PR-x-5", "es-PY-x-3", "es-PY-x-4", "es-PY-x-5", "es-SV-x-3", "es-SV-x-4", "es-SV-x-5", "es-US-x-3", "es-US-x-4", "es-US-x-5", "es-UY-x-3", "es-UY-x-4", "es-UY-x-5", "es-VE-x-3", "es-VE-x-4", "es-VE-x-5", "es-x-3", "es-x-4", "es-x-5", "eu-x-3", "eu-x-4", "eu-x-5", "fa-x-3", "fa-x-4", "fa-x-5", "ff-Adlm-x-3", "ff-Adlm-x-4", "ff-Adlm-x-5", "fi-x-3", "fi-x-4", "fi-x-5", "fil-x-3", "fil-x-4", "fil-x-5", "fr-x-3", "fr-x-4", "fr-x-5", "gd-x-3", "gd-x-4", "gd-x-5", "gu-x-3", "gu-x-4", "gu-x-5", "he-x-3", "he-x-4", "he-x-5", "hi-Latn-x-3", "hi-Latn-x-4", "hi-Latn-x-5", "hr-x-3", "hr-x-4", "hr-x-5", "hu-x-3", "hu-x-4", "hu-x-5", "id-x-3", "id-x-4", "id-x-5", "is-x-3", "is-x-4", "is-x-5", "it-x-3", "it-x-4", "it-x-5", "ja-x-3", "ja-x-4", "ja-x-5", "kn-x-3", "kn-x-4", "kn-x-5", "ko-x-3", "ko-x-4", "ko-x-5", "lo-x-3", "lo-x-4", "lo-x-5", "lt-x-3", "lt-x-4", "lt-x-5", "lv-x-3", "lv-x-4", "lv-x-5", "mk-x-3", "mk-x-4", "mk-x-5", "ml-x-3", "ml-x-4", "ml-x-5", "mr-x-3", "mr-x-4", "mr-x-5", "ms-x-4", "ms-x-5", "nb-x-3", "nb-x-4", "nb-x-5", "nl-x-3", "nl-x-4", "nl-x-5", "nn-x-3", "nn-x-4", "nn-x-5", "no-x-3", "no-x-4", "no-x-5", "pa-x-3", "pa-x-4", "pa-x-5", "pl-x-3", "pl-x-4", "pl-x-5", "pt-x-3", "pt-x-4", "pt-x-5", "ro-x-3", "ro-x-4", "ro-x-5", "ru-x-3", "ru-x-4", "ru-x-5", "sc-x-3", "sc-x-4", "sc-x-5", "sk-x-3", "sk-x-4", "sk-x-5", "sl-x-3", "sl-x-4", "sl-x-5", "so-x-3", "so-x-4", "so-x-5", "sr-Latn-x-3", "sr-Latn-x-4", "sr-Latn-x-5", "sr-x-3", "sr-x-4", "sr-x-5", "sv-x-3", "sv-x-4", "sv-x-5", "ta-x-3", "ta-x-4", "ta-x-5", "te-x-3", "te-x-4", "te-x-5", "th-x-3", "th-x-4", "th-x-5", "tr-x-3", "tr-x-4", "tr-x-5", "und-x-3", "und-x-4", "und-x-5", "ur-x-3", "ur-x-4", "ur-x-5", "vi-x-3", "vi-x-4", "vi-x-5", "yue-Hans-x-3", "yue-Hans-x-4", "yue-Hans-x-5", "yue-x-3", "yue-x-4", "yue-x-5", "zh-Hant-x-3", "zh-Hant-x-4", "zh-Hant-x-5", "zh-x-3", "zh-x-4", "zh-x-5"];
                let mut metadata = icu_provider::DataResponseMetadata::default();
                let payload = if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    payload
                } else {
                    const FALLBACKER: icu::locid_transform::fallback::LocaleFallbackerWithConfig<'static> = icu::locid_transform::fallback::LocaleFallbacker::new().for_config(<icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.fallback_config());
                    let mut fallback_iterator = FALLBACKER.fallback_for(req.locale.clone());
                    loop {
                        if fallback_iterator.get().is_und() {
                            return Err(icu_provider::DataErrorKind::MissingLocale.with_req(<icu::datetime::provider::neo::RocYearSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY, req));
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