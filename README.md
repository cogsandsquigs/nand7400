# nand7400asm

An assembler library for the nand7400 computer, created by me during my internship at [The WCL](https://thewcl.com).

This library is built in rust, and also binds directly to Swift as well, so you can treat it like a standard Swift package. This is because the nand7400 computer was (and still is) programmed via an iOS/MacOS app.

## Lay of the Land

-   `nand7400asm`: The rust library itself. This contains bindings to `UniFFI` under the `uniffi` feature.
-   `nand7400asm-swift`: The Swift package that binds to the `nand7400asm` rust library.
-   `uniffi-bindgen`: A wrapper rust library that is used to execute `uniffi-bindgen` commands.

## Building

### Rust

To build the rust library, simply run `cargo build` in the root directory of the project. You can use this library as any other standard rust library.

### Swift

#### IMPORTANT!

If you change the `uniffi` version in the `nand7400asm` package, you **_MUST_** change it **_EVERYWHERE_** else in the package. Otherwise, it will not compile correctly in Xcode (you will see a symbols missing/undefined error). There is a global `uniffi` version set in `Cargo.toml`, which should mitigate this issue, but it is still a good idea to keep this in mind.

To build this library for Swift, you'll need a Mac with Xcode 12 or later. To install certain Rust utilities and targets for building, run `make init`. Then, to build, run `make package`. You can then use this package as any other Swift package. However, to use this with `Xcode`, you need to go into `Targets > (your target) > Build Phases > Link Binary With Libraries` and add the `nand7400asm` framework inside the library.

## How I did this

These are the resources I used to help me learn how to bind Rust into Swift:

-   [Creating an XCFramework](https://rhonabwy.com/2023/02/10/creating-an-xcframework/)
-   [UniFFI](https://mozilla.github.io/uniffi-rs/)
-   [The YSwift repository](https://github.com/y-crdt/yswift)
