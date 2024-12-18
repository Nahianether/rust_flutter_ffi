// ignore_for_file: always_specify_types
// ignore_for_file: camel_case_types
// ignore_for_file: non_constant_identifier_names

// AUTO GENERATED FILE, DO NOT EDIT.
//
// Generated by `package:ffigen`.
// ignore_for_file: type=lint
import 'dart:ffi' as ffi;

/// Bindings for `src/rust_flutter_ffi.h`.
///
/// Regenerate bindings with `dart run ffigen --config ffigen.yaml`.
///
class RustFlutterFfiBindings {
  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
      _lookup;

  /// The symbols are looked up in [dynamicLibrary].
  RustFlutterFfiBindings(ffi.DynamicLibrary dynamicLibrary)
      : _lookup = dynamicLibrary.lookup;

  /// The symbols are looked up with [lookup].
  RustFlutterFfiBindings.fromLookup(
      ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
          lookup)
      : _lookup = lookup;

  int sum(
    int a,
    int b,
  ) {
    return _sum(
      a,
      b,
    );
  }

  late final _sumPtr = _lookup<
      ffi
      .NativeFunction<ffi.UintPtr Function(ffi.UintPtr, ffi.UintPtr)>>('sum');
  late final _sum = _sumPtr.asFunction<int Function(int, int)>();

  int sum_async(
    int a,
    int b,
  ) {
    return _sum_async(
      a,
      b,
    );
  }

  late final _sum_asyncPtr = _lookup<
          ffi.NativeFunction<ffi.UintPtr Function(ffi.UintPtr, ffi.UintPtr)>>(
      'sum_async');
  late final _sum_async = _sum_asyncPtr.asFunction<int Function(int, int)>();

  ffi.Pointer<ffi.Char> sum_str(
    ffi.Pointer<ffi.Char> a,
    ffi.Pointer<ffi.Char> b,
  ) {
    return _sum_str(
      a,
      b,
    );
  }

  late final _sum_strPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<ffi.Char> Function(
              ffi.Pointer<ffi.Char>, ffi.Pointer<ffi.Char>)>>('sum_str');
  late final _sum_str = _sum_strPtr.asFunction<
      ffi.Pointer<ffi.Char> Function(
          ffi.Pointer<ffi.Char>, ffi.Pointer<ffi.Char>)>();

  void main_engine() {
    return _main_engine();
  }

  late final _main_enginePtr =
      _lookup<ffi.NativeFunction<ffi.Void Function()>>('main_engine');
  late final _main_engine = _main_enginePtr.asFunction<void Function()>();

  void free_c_string(
    ffi.Pointer<ffi.Char> s,
  ) {
    return _free_c_string(
      s,
    );
  }

  late final _free_c_stringPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Char>)>>(
          'free_c_string');
  late final _free_c_string =
      _free_c_stringPtr.asFunction<void Function(ffi.Pointer<ffi.Char>)>();
}
