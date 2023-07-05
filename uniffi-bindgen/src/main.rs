/// This is the entry point for the `uniffi-bindgen` binary. This basically just calls the
/// binary for `uniffi-bindgen` in the `uniffi` crate, without having to go through Nightly.
fn main() {
    uniffi::uniffi_bindgen_main()
}
