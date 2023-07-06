fn main() {
    // If we are building with UniFFI, then generate the scaffolding.
    #[cfg(feature = "uniffi")]
    uniffi::generate_scaffolding("src/lib.udl").unwrap();
}
