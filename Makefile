# This makefile is used to build the Nand7400 framework for iOS, macOS and Mac Catalyst. To use it, run `make

# Rust-specific configuration
PACKAGE_NAME=nand7400-ffi
LIB_NAME=libnand7400_ffi.a
CARGO_FLAGS= --package ${PACKAGE_NAME} --lib --locked --release

# General binding configuration
UNIFFI_CMD=cargo run -p nand7400-ffi --features=cli --bin uniffi --
UNIFFI_UDL_FILE=src/ffi.udl

# General build configuration
BUILD_FOLDER=target
ARTIFACTS_FOLDER=target/uniffi-artifacts
BINDINGS_FOLDER=nand7400-ffi-bindings

# Testing/fuzzing stuff
FUZZ_TARGET=nand7400-fuzz
# Additional flags for AFL besides the input, output and target binary
AFL_FLAGS=-d -D

# Swift-specific stuff
FRAMEWORK_NAME=Nand7400FFI
XCFRAMEWORK_FOLDER=target/${FRAMEWORK_NAME}.xcframework

# Install all the necessary build targets to build for Mac, iOS and Mac Catalyst.
setup-build:
	@echo "▸ Installing toolchains..."
# 	iOS Simulator (Intel)
	@rustup target add x86_64-apple-ios
#	iOS Simulator (M1)
	@rustup target add aarch64-apple-ios-sim
#	iOS Device 
	@rustup target add aarch64-apple-ios
#	macOS ARM/M1
	@rustup target add aarch64-apple-darwin
#	macOS Intel/x86 
	@rustup target add x86_64-apple-darwin

# Setup testing utilities
setup-test:
	@echo "▸ Installing testing utilities..."
#	AFL++ with rust.
	@cargo install cargo-afl

clean:
	@echo ▸ Cleaning build...
	@rm -rf ${ARTIFACTS_FOLDER}
	@rm -rf ${XCFRAMEWORK_FOLDER}
	@rm -rf ${BINDINGS_FOLDER}
	@mkdir -p ${ARTIFACTS_FOLDER}
	@mkdir -p ${BINDINGS_FOLDER}

bind: setup-build clean
	@echo "▸ Generating Swift scaffolding code..."
	${UNIFFI_CMD} generate ${PACKAGE_NAME}/${UNIFFI_UDL_FILE} --language swift --out-dir ${BINDINGS_FOLDER}/swift

build-swift: bind
	@echo "▸ Building for x86_64-apple-ios..."
	@CFLAGS_x86_64_apple_ios="-target x86_64-apple-ios" \
	cargo build --target x86_64-apple-ios ${CARGO_FLAGS}

	@echo "▸ Building for aarch64-apple-ios-sim..."
	@CFLAGS_aarch64_apple_ios="-target aarch64-apple-ios-sim" \
	cargo build --target aarch64-apple-ios-sim ${CARGO_FLAGS}

	@echo "▸ Building for aarch64-apple-ios..."
	@CFLAGS_aarch64_apple_ios="-target aarch64-apple-ios" \
	cargo build --target aarch64-apple-ios ${CARGO_FLAGS}

	@echo "▸ Building for aarch64-apple-darwin..."
	@CFLAGS_aarch64_apple_darwin="-target aarch64-apple-darwin" \
	cargo build --target aarch64-apple-darwin ${CARGO_FLAGS}

	@echo "▸ Building for x86_64-apple-darwin..."
	@CFLAGS_x86_64_apple_darwin="-target x86_64-apple-darwin" \
	cargo build --target x86_64-apple-darwin ${CARGO_FLAGS}

	@echo "▸ Building for x86_64-apple-ios-macabi..."
	@CFLAGS_x86_64_apple_ios="-target x86_64-apple-ios-macabi" \
	cargo +nightly build -Z build-std --target x86_64-apple-ios-macabi ${CARGO_FLAGS}

	@echo "▸ Building for aarch64-apple-ios-macabi..."
	@CFLAGS_aarch64_apple_ios="-target aarch64-apple-ios-macabi" \
	cargo +nightly build -Z build-std --target aarch64-apple-ios-macabi ${CARGO_FLAGS}

	@echo "▸ Consolidating the headers and modulemaps for XCFramework generation..."
	@mkdir -p ${ARTIFACTS_FOLDER}/includes
	@cp ${BINDINGS_FOLDER}/swift/${FRAMEWORK_NAME}.h ${ARTIFACTS_FOLDER}/includes
	@cp ${BINDINGS_FOLDER}/swift/${FRAMEWORK_NAME}.modulemap ${ARTIFACTS_FOLDER}/includes/module.modulemap
	
	@echo "▸ Merging x86 and arm iOS simulator static libraries into a fat static binary..."
	@mkdir -p ${ARTIFACTS_FOLDER}/ios-simulator/release
	@lipo -create \
		./${BUILD_FOLDER}/x86_64-apple-ios/release/${LIB_NAME} \
		./${BUILD_FOLDER}/aarch64-apple-ios-sim/release/${LIB_NAME} \
		-output ${ARTIFACTS_FOLDER}/ios-simulator/release/${LIB_NAME}

	@echo "▸ Merging x86 and arm macOS static libraries into a fat static binary..."
	@mkdir -p ${ARTIFACTS_FOLDER}/apple-darwin/release
	@lipo -create \
		./${BUILD_FOLDER}/x86_64-apple-darwin/release/${LIB_NAME} \
		./${BUILD_FOLDER}/aarch64-apple-darwin/release/${LIB_NAME} \
		-output ${ARTIFACTS_FOLDER}/apple-darwin/release/${LIB_NAME}

	@echo "▸ Merging x86 and arm macOS Catalyst static libraries into a fat static binary..."
	@mkdir -p ${ARTIFACTS_FOLDER}/mac-catalyst/release
	@lipo -create \
		./${BUILD_FOLDER}/x86_64-apple-ios-macabi/release/${LIB_NAME} \
		./${BUILD_FOLDER}/aarch64-apple-ios-macabi/release/${LIB_NAME} \
		-output ${ARTIFACTS_FOLDER}/mac-catalyst/release/${LIB_NAME}


package-swift: build-swift
	@echo "▸ Creating XCFramework..."
#	what docs there are:
#	xcodebuild -create-xcframework -help
#	https://developer.apple.com/documentation/xcode/creating-a-multi-platform-binary-framework-bundle
	@BUILD_LIBRARY_FOR_DISTRIBUTION=YES \
	xcodebuild -create-xcframework \
		-library ./${BUILD_FOLDER}/aarch64-apple-ios/release/${LIB_NAME} \
		-headers ./${ARTIFACTS_FOLDER}/includes \
		-library ./${ARTIFACTS_FOLDER}/ios-simulator/release/${LIB_NAME} \
		-headers ./${ARTIFACTS_FOLDER}/includes \
		-library ./${ARTIFACTS_FOLDER}/apple-darwin/release/${LIB_NAME} \
		-headers ./${ARTIFACTS_FOLDER}/includes \
		-library ./${ARTIFACTS_FOLDER}/mac-catalyst/release/${LIB_NAME} \
		-headers ./${ARTIFACTS_FOLDER}/includes \
		-output ./${XCFRAMEWORK_FOLDER}

#	Move modulemaps to the right place, so that the XCFramework can be used directly in Swift Package Manager
	@mkdir -p ${XCFRAMEWORK_FOLDER}/ios-arm64/Modules
	@mkdir -p ${XCFRAMEWORK_FOLDER}/ios-x86_64-simulator/Modules
	@mkdir -p ${XCFRAMEWORK_FOLDER}/ios-arm64_x86_64-simulator/Modules
	@mkdir -p ${XCFRAMEWORK_FOLDER}/ios-arm64_x86_64-maccatalyst/Modules
	@mkdir -p ${XCFRAMEWORK_FOLDER}/macos-arm64_x86_64/Modules
	@cp ${BINDINGS_FOLDER}/swift/${FRAMEWORK_NAME}.modulemap ${XCFRAMEWORK_FOLDER}/ios-arm64/Modules/module.modulemap
	@cp ${BINDINGS_FOLDER}/swift/${FRAMEWORK_NAME}.modulemap ${XCFRAMEWORK_FOLDER}/ios-x86_64-simulator/Modules/module.modulemap
	@cp ${BINDINGS_FOLDER}/swift/${FRAMEWORK_NAME}.modulemap ${XCFRAMEWORK_FOLDER}/ios-arm64_x86_64-simulator/Modules/module.modulemap
	@cp ${BINDINGS_FOLDER}/swift/${FRAMEWORK_NAME}.modulemap ${XCFRAMEWORK_FOLDER}/ios-arm64_x86_64-maccatalyst/Modules/module.modulemap
	@cp ${BINDINGS_FOLDER}/swift/${FRAMEWORK_NAME}.modulemap ${XCFRAMEWORK_FOLDER}/macos-arm64_x86_64/Modules/module.modulemap

	@echo "▸ Compressing XCFramework..."
	@ditto -c -k --sequesterRsrc --keepParent ${XCFRAMEWORK_FOLDER} ${XCFRAMEWORK_FOLDER}.zip

	@echo "▸ Computing checksum..."
	@swift package compute-checksum ${XCFRAMEWORK_FOLDER}.zip > ${XCFRAMEWORK_FOLDER}.zip.sha256

	@echo "▸ Finished Swift bindings!"

# Convenience target to build and package everything
package: package-swift
	@echo "▸ Done!"

# Convenience target to fuzz the main rust crate all in one go.
fuzz: setup-test
	@echo "▸ Building fuzz target..."
	@cargo afl build -p ${FUZZ_TARGET}
	@echo "▸ Fuzzing..."
	@AFL_AUTORESUME=1 \
	cargo afl fuzz ${AFL_FLAGS} -i nand7400-fuzz/seeds -o ${BUILD_FOLDER}/afl-out ${BUILD_FOLDER}/debug/nand7400-fuzz
	@echo "▸ Done!"
