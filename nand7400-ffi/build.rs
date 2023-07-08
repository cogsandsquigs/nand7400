fn main() {
    // If we are building with UniFFI, then generate the scaffolding.
    uniffi::generate_scaffolding("src/ffi.udl").unwrap();
}
