// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// The raw canonical composition operation.
///
/// Callers should generally use ICU4XComposingNormalizer unless they specifically need raw composition operations
///
/// See the [Rust documentation for `CanonicalComposition`](https://docs.rs/icu/latest/icu/normalizer/properties/struct.CanonicalComposition.html) for more information.
final class CanonicalComposition implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  CanonicalComposition._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('ICU4XCanonicalComposition_destroy'));

  /// Construct a new ICU4XCanonicalComposition instance for NFC
  ///
  /// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/normalizer/properties/struct.CanonicalComposition.html#method.new) for more information.
  ///
  /// Throws [Error] on failure.
  factory CanonicalComposition(DataProvider provider) {
    final result = _ICU4XCanonicalComposition_create(provider._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return CanonicalComposition._(result.union.ok);
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XCanonicalComposition_create =
    _capi<ffi.NativeFunction<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>>('ICU4XCanonicalComposition_create')
      .asFunction<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Performs canonical composition (including Hangul) on a pair of characters
  /// or returns NUL if these characters don’t compose. Composition exclusions are taken into account.
  ///
  /// See the [Rust documentation for `compose`](https://docs.rs/icu/latest/icu/normalizer/properties/struct.CanonicalComposition.html#method.compose) for more information.
  Rune compose(Rune starter, Rune second) {
    final result = _ICU4XCanonicalComposition_compose(_underlying, starter, second);
    return result;
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XCanonicalComposition_compose =
    _capi<ffi.NativeFunction<ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>, ffi.Uint32, ffi.Uint32)>>('ICU4XCanonicalComposition_compose')
      .asFunction<Rune Function(ffi.Pointer<ffi.Opaque>, Rune, Rune)>(isLeaf: true);
}
