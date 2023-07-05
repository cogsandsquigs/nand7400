/// This is the main build function used to run some things to compile stuff for swift. It's mostly
/// just for UniFFI
fn main() {
    uniffi::generate_scaffolding("src/lib.udl").unwrap();
}
