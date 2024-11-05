// generated by diplomat-tool

part of 'lib.g.dart';

/// An ICU4X DateFormatter object capable of formatting a [`Date`] as a string,
/// using some calendar specified at runtime in the locale.
///
/// See the [Rust documentation for `datetime`](https://docs.rs/icu/latest/icu/datetime/index.html) for more information.
final class DateFormatter implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _ffi;

  // These are "used" in the sense that they keep dependencies alive
  // ignore: unused_field
  final core.List<Object> _selfEdge;

  // This takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  DateFormatter._fromFfi(this._ffi, this._selfEdge) {
    if (_selfEdge.isEmpty) {
      _finalizer.attach(this, _ffi.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_icu4x_DateFormatter_destroy_mv1));

  /// Creates a new [`DateFormatter`] from locale data.
  ///
  /// Throws [PatternLoadError] on failure.
  factory DateFormatter.withLength(DataProvider provider, Locale locale, DateTimeLength length) {
    final result = _icu4x_DateFormatter_create_with_length_mv1(provider._ffi, locale._ffi, length.index);
    if (!result.isOk) {
      throw PatternLoadError.values.firstWhere((v) => v._ffi == result.union.err);
    }
    return DateFormatter._fromFfi(result.union.ok, []);
  }

  /// Formats a [`Date`] to a string.
  ///
  /// Throws [DateTimeFormatError] on failure.
  String formatDate(Date value) {
    final write = _Write();
    final result = _icu4x_DateFormatter_format_date_mv1(_ffi, value._ffi, write._ffi);
    if (!result.isOk) {
      throw DateTimeFormatError.values.firstWhere((v) => v._ffi == result.union.err);
    }
    return write.finalize();
  }

  /// Formats a [`IsoDate`] to a string.
  ///
  /// Will convert to this formatter's calendar first
  ///
  /// Throws [DateTimeFormatError] on failure.
  String formatIsoDate(IsoDate value) {
    final write = _Write();
    final result = _icu4x_DateFormatter_format_iso_date_mv1(_ffi, value._ffi, write._ffi);
    if (!result.isOk) {
      throw DateTimeFormatError.values.firstWhere((v) => v._ffi == result.union.err);
    }
    return write.finalize();
  }

  /// Formats a [`DateTime`] to a string.
  ///
  /// Throws [DateTimeFormatError] on failure.
  String formatDatetime(DateTime value) {
    final write = _Write();
    final result = _icu4x_DateFormatter_format_datetime_mv1(_ffi, value._ffi, write._ffi);
    if (!result.isOk) {
      throw DateTimeFormatError.values.firstWhere((v) => v._ffi == result.union.err);
    }
    return write.finalize();
  }

  /// Formats a [`IsoDateTime`] to a string.
  ///
  /// Will convert to this formatter's calendar first
  ///
  /// Throws [DateTimeFormatError] on failure.
  String formatIsoDatetime(IsoDateTime value) {
    final write = _Write();
    final result = _icu4x_DateFormatter_format_iso_datetime_mv1(_ffi, value._ffi, write._ffi);
    if (!result.isOk) {
      throw DateTimeFormatError.values.firstWhere((v) => v._ffi == result.union.err);
    }
    return write.finalize();
  }
}

@meta.RecordUse()
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'icu4x_DateFormatter_destroy_mv1')
// ignore: non_constant_identifier_names
external void _icu4x_DateFormatter_destroy_mv1(ffi.Pointer<ffi.Void> self);

@meta.RecordUse()
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Int32)>(isLeaf: true, symbol: 'icu4x_DateFormatter_create_with_length_mv1')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _icu4x_DateFormatter_create_with_length_mv1(ffi.Pointer<ffi.Opaque> provider, ffi.Pointer<ffi.Opaque> locale, int length);

@meta.RecordUse()
@ffi.Native<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'icu4x_DateFormatter_format_date_mv1')
// ignore: non_constant_identifier_names
external _ResultVoidInt32 _icu4x_DateFormatter_format_date_mv1(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> value, ffi.Pointer<ffi.Opaque> write);

@meta.RecordUse()
@ffi.Native<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'icu4x_DateFormatter_format_iso_date_mv1')
// ignore: non_constant_identifier_names
external _ResultVoidInt32 _icu4x_DateFormatter_format_iso_date_mv1(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> value, ffi.Pointer<ffi.Opaque> write);

@meta.RecordUse()
@ffi.Native<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'icu4x_DateFormatter_format_datetime_mv1')
// ignore: non_constant_identifier_names
external _ResultVoidInt32 _icu4x_DateFormatter_format_datetime_mv1(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> value, ffi.Pointer<ffi.Opaque> write);

@meta.RecordUse()
@ffi.Native<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'icu4x_DateFormatter_format_iso_datetime_mv1')
// ignore: non_constant_identifier_names
external _ResultVoidInt32 _icu4x_DateFormatter_format_iso_datetime_mv1(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> value, ffi.Pointer<ffi.Opaque> write);
