fn main() {
    /// Create the bindings for the UDL file.
    uniffi::generate_scaffolding("./src/lib.udl").unwrap();
}
