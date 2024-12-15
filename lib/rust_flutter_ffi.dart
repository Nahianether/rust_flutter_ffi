import 'dart:ffi';
import 'dart:io';

import 'package:ffi/ffi.dart';

import 'rust_flutter_ffi_bindings_generated.dart';

const String _libName = 'rust_flutter_ffi';

final DynamicLibrary _dylib = () {
  if (Platform.isMacOS || Platform.isIOS) return DynamicLibrary.open('$_libName.framework/$_libName');
  if (Platform.isAndroid || Platform.isLinux) return DynamicLibrary.open('librust_flutter_ffi_core.so');
  if (Platform.isWindows) return DynamicLibrary.open('$_libName.dll');
  throw UnsupportedError('Unknown platform: ${Platform.operatingSystem}');
}();

/// Bindings for the library.
final RustFlutterFfiBindings bindings = RustFlutterFfiBindings(_dylib);

extension StringFfi on String {
  /// Converts a Dart string to a `Pointer<Char>` and allocates memory.
  /// Remember to free the pointer after use to avoid memory leaks.
  Pointer<Char> toPtr() => toNativeUtf8(allocator: malloc).cast<Char>();
}

extension PointerFfi on Pointer<Char> {
  /// Converts a `Pointer<Char>` back to a Dart string and frees Rust-allocated memory.
  String toStr() {
    final dartString = cast<Utf8>().toDartString();
    bindings.free_c_string(this);
    return dartString;
  }
}
