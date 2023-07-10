// swift-tools-version:5.8

import Foundation
import PackageDescription

let FFIbinaryTarget: PackageDescription.Target

// If `NAND7400_LOCAL` is set, then we are using a local file reference to an XCFramework, otherwise we
// use the remote URL to download the XCFramework. This is useful for testing the package locally, but
// should not be used in production.
if ProcessInfo.processInfo.environment["NAND7400_LOCAL"] != nil {
	FFIbinaryTarget = .binaryTarget(
		name: "Nand7400FFI",
		path: "./target/Nand7400FFI.xcframework"
	)
} else {
	FFIbinaryTarget = .binaryTarget(
		name: "Nand7400FFI",
		url: "https://github.com/cogsandsquigs/nand7400/releases/download/v0.1.0/Nand7400FFI.xcframework.zip",
		checksum: "4291fe8fe39ca89cbfe1c64f68dce148795ca6990c4285c3b4db9346fe196cd5"
	)
}

let package = Package(
	name: "Nand7400",
	platforms: [.iOS(.v13), .macOS(.v10_15)],
	products: [
		.library(
			name: "Nand7400",
			targets: ["Nand7400"]
		),
	],
	dependencies: [],
	targets: [
		.target(
			name: "Nand7400",
			dependencies: ["Nand7400FFI"],
			path: "nand7400-ffi-bindings/swift"
		),
		FFIbinaryTarget,
	]
)
