# nand7400-asm

An assembler library for the nand7400 computer, created by me during my internship at [The WCL](https://thewcl.com).

This library is built in rust, and also binds directly to Swift as well, so you can treat it like a standard Swift package. This is because the nand7400 computer was (and still is) programmed via an iOS/MacOS app.

## Lay of the Land

-   `src/` contains the rust source code for the library
-   `include/` contains the modulemap for the Swift package
-   `Sources/` contains the Swift source code for the library
-   `Makefile` contains the build instructions for the Swift package

## Compiling

### Rust

To build the rust library, simply run `cargo build` in the root directory of the project. You can use this library as any other standard rust library.

### Swift

To build this library for Swift, you'll need a Mac with Xcode 12 or later. To install certain Rust utilities and targets for building, run `make init`. Then, to build, run `make build`. You can then use this package as any other Swift package.
