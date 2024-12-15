fn main() {
    cbindgen::generate("../rust_flutter_ffi_core")
        .unwrap()
        .write_to_file("../src/rust_flutter_ffi.h");
}
