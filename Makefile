RUST_PACKAGE_NAME := nand7400_asm
RUST_FOLDER_NAME := nand7400-asm
SWIFT_PACKAGE_NAME := Nand7400Asm

# Initiallize all build dependencies for building the project into a Swift package.
init:
	rustup target add x86_64-apple-darwin aarch64-apple-darwin x86_64-apple-ios aarch64-apple-ios aarch64-apple-ios-sim

# Package the project into a Swift package.
package: clean build bind bundle

# Build all the code for the project so it can be compiled into a Swift package.
build:
#	Make the generated directory
	mkdir generated

#	Build the library for macOS
	cargo build -p $(RUST_FOLDER_NAME) --release --target x86_64-apple-darwin
	cargo build -p $(RUST_FOLDER_NAME) --release --target aarch64-apple-darwin

#	Build the standard library and this library for macOS Catalyst
	cargo +nightly build -p $(RUST_FOLDER_NAME) --release -Z build-std --target x86_64-apple-ios-macabi
	cargo +nightly build -p $(RUST_FOLDER_NAME) --release -Z build-std --target aarch64-apple-ios-macabi

#	Build the library for iOS
	cargo build -p $(RUST_FOLDER_NAME) --release --target x86_64-apple-ios
	cargo build -p $(RUST_FOLDER_NAME) --release --target aarch64-apple-ios
	cargo build -p $(RUST_FOLDER_NAME) --release --target aarch64-apple-ios-sim

# Build the header file for swift
bind:
	cargo uniffi generate $(RUST_FOLDER_NAME)/src/lib.udl --language swift --out-dir ./include

#	Rename the modulemap to `module.modulemap`
	mv include/$(SWIFT_PACKAGE_NAME)FFI.modulemap include/$(SWIFT_PACKAGE_NAME)FFI.modulemap

# Bundle the header file and build files for swift
bundle:
# 	Bundle the library for macOS
	lipo -create \
		target/x86_64-apple-darwin/release/lib$(RUST_PACKAGE_NAME).a \
		target/aarch64-apple-darwin/release/lib$(RUST_PACKAGE_NAME).a \
		-output generated/lib$(RUST_PACKAGE_NAME)_macos.a

# 	Bundle the library for macOS Catalyst
	lipo -create \
		target/x86_64-apple-ios-macabi/release/lib$(RUST_PACKAGE_NAME).a \
		target/aarch64-apple-ios-macabi/release/lib$(RUST_PACKAGE_NAME).a \
		-output generated/lib$(RUST_PACKAGE_NAME)_maccatalyst.a

# 	Bundle the library for iOS
	lipo -create \
		target/x86_64-apple-ios/release/lib$(RUST_PACKAGE_NAME).a \
		target/aarch64-apple-ios-sim/release/lib$(RUST_PACKAGE_NAME).a \
		-output generated/lib$(RUST_PACKAGE_NAME)_iossimulator.a

#	Create the xcframework bundle.
	xcodebuild -create-xcframework \
		-library ./generated/lib$(RUST_PACKAGE_NAME)_macos.a \
		-headers ./include/ \
		-library ./generated/lib$(RUST_PACKAGE_NAME)_maccatalyst.a \
		-headers ./include/ \
		-library ./generated/lib$(RUST_PACKAGE_NAME)_iossimulator.a \
		-headers ./include/ \
		-library ./target/aarch64-apple-ios/release/lib$(RUST_PACKAGE_NAME).a \
		-headers ./include/ \
		-output ./generated/$(SWIFT_PACKAGE_NAME).xcframework

#	Move the module.modulemap to the Module folder in every platform
	mkdir generated/$(SWIFT_PACKAGE_NAME).xcframework/ios-arm64/Modules
	mkdir generated/$(SWIFT_PACKAGE_NAME).xcframework/ios-arm64_x86_64-simulator/Modules
	mkdir generated/$(SWIFT_PACKAGE_NAME).xcframework/ios-arm64_x86_64-maccatalyst/Modules
	mkdir generated/$(SWIFT_PACKAGE_NAME).xcframework/macos-arm64_x86_64/Modules

	cp include/$(SWIFT_PACKAGE_NAME)FFI.modulemap generated/$(SWIFT_PACKAGE_NAME).xcframework/ios-arm64/Modules/module.modulemap
	cp include/$(SWIFT_PACKAGE_NAME)FFI.modulemap generated/$(SWIFT_PACKAGE_NAME).xcframework/ios-arm64_x86_64-simulator/Modules/module.modulemap
	cp include/$(SWIFT_PACKAGE_NAME)FFI.modulemap generated/$(SWIFT_PACKAGE_NAME).xcframework/ios-arm64_x86_64-maccatalyst/Modules/module.modulemap
	cp include/$(SWIFT_PACKAGE_NAME)FFI.modulemap generated/$(SWIFT_PACKAGE_NAME).xcframework/macos-arm64_x86_64/Modules/module.modulemap

#	Now zip it up
	zip -r generated/bundle.zip generated/$(SWIFT_PACKAGE_NAME).xcframework

#	Generate the checksum
	swift package compute-checksum generated/bundle.zip > generated/bundle.zip.sha256

# Clean up everything from the build.
clean:
	rm -rf generated
	rm -rf include