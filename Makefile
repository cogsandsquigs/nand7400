# Initiallize all build dependencies for building the project into a Swift package.
init:
	cargo install cbindgen
	rustup target add x86_64-apple-darwin aarch64-apple-darwin x86_64-apple-ios aarch64-apple-ios aarch64-apple-ios-sim

# Package the project into a Swift package.
package: clean build bind bundle

# Build all the code for the project so it can be compiled into a Swift package.
build:
#	Make the generated directory
	mkdir generated

#	Build the library for macOS
	cargo build --release --target x86_64-apple-darwin
	cargo build --release --target aarch64-apple-darwin

#	Build the standard library and this library for macOS Catalyst
	cargo +nightly build --release -Z build-std --target x86_64-apple-ios-macabi
	cargo +nightly build --release -Z build-std --target aarch64-apple-ios-macabi

#	Build the library for iOS
	cargo build --release --target x86_64-apple-ios
	cargo build --release --target aarch64-apple-ios
	cargo build --release --target aarch64-apple-ios-sim

# Build the header file for swift
bind:
	cbindgen --lang c --output include/ffi.h

# Bundle the header file and build files for swift
bundle:
# 	Bundle the library for macOS
	lipo -create \
		target/x86_64-apple-darwin/release/libnand7400_asm.a \
		target/aarch64-apple-darwin/release/libnand7400_asm.a \
		-output generated/libnand7400_asm_macos.a

# 	Bundle the library for macOS Catalyst
	lipo -create \
		target/x86_64-apple-ios-macabi/release/libnand7400_asm.a \
		target/aarch64-apple-ios-macabi/release/libnand7400_asm.a \
		-output generated/libnand7400_asm_maccatalyst.a

# 	Bundle the library for iOS
	lipo -create \
		target/x86_64-apple-ios/release/libnand7400_asm.a \
		target/aarch64-apple-ios-sim/release/libnand7400_asm.a \
		-output generated/libnand7400_asm_iossimulator.a

#	Create the xcframework bundle.
	xcodebuild -create-xcframework \
		-library ./generated/libnand7400_asm_iossimulator.a \
		-headers ./include/ \
		-library ./generated/libnand7400_asm_iossimulator.a \
		-headers ./include/ \
		-library ./generated/libnand7400_asm_iossimulator.a \
		-headers ./include/ \
		-library ./target/aarch64-apple-ios/release/libnand7400_asm.a \
		-headers ./include/ \
		-output ./generated/Nand7400Asm.xcframework

#	Now zip it up
	zip -r generated/bundle.zip generated/Nand7400Asm.xcframework

#	Generate the checksum
	shasum -a 256 generated/bundle.zip > generated/bundle.zip.sha256

# Clean up everything from the build.
clean:
	rm -rf generated
	rm -rf include/ffi.h