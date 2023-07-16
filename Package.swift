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
			url: "https://github.com/cogsandsquigs/nand7400/releases/download/v0.2.1/Nand7400FFI.xcframework.zip",
			checksum: "e0fc9c63683e120cf2bc8246041e71af107d99b017e43a1d892d2c0b83fa8f18"
		),
		// .binaryTarget(
		// 	name: "Nand7400FFI",
		// 	path: "./target/Nand7400FFI.xcframework"
		// ),
	]
)
