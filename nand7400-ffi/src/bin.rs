fn main() {
    // Only run the uniffi bindgen if the cli feature is enabled. If this isn't included, rust-analyzer throws
    // errors about the uniffi_bindgen_main function not existing because the "uniffi/cli" feature isn't enabled.
    // "cli" is a wrapper feature that enables all of the features needed to run the uniffi_bindgen_main function.
    #[cfg(feature = "cli")]
    uniffi::uniffi_bindgen_main()
}
