// swift-tools-version:5.8

import Foundation
import PackageDescription

let FFIbinaryTarget: PackageDescription.Target

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
		.binaryTarget(
			name: "Nand7400FFI",
			url: "https://github.com/cogsandsquigs/nand7400/releases/download/v0.3.0/Nand7400FFI.xcframework.zip",
			checksum: "9b3349f6f689c3433c957e203a2820cc371d16e307964398f80aecafa6e273a6"
		),
		// .binaryTarget(
		// 	name: "Nand7400FFI",
		// 	path: "./target/Nand7400FFI.xcframework"
		// ),
	]
)
