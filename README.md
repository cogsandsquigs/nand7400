# nand7000-asm

An assembler library for the nand7000 computer, created by me during my internship at [The WCL](https://thewcl.com).

This library is built in rust, and also binds directly to Swift as well, so you can treat it like a standard Swift package.

## Compiling

### Rust

To build the rust library, simply run `cargo build` in the root directory of the project. You can use this library as any other standard rust library.

### Swift

To build this for Swift, you'll need `cargo-swift` installed. Install it via `cargo install cargo-swift`. Then, run `cargo swift package` to generate the Swift package. You can then use this package as any other Swift package.
