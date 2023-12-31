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
			url: "https://github.com/cogsandsquigs/nand7400/releases/download/v0.4.1/Nand7400FFI.xcframework.zip",
			checksum: "d695296794200adddfdb8eacf9800d482d6dac14b7b837f21a10a39c48837c35"
		),
		// .binaryTarget(
		// 	name: "Nand7400FFI",
		// 	path: "./target/Nand7400FFI.xcframework"
		// ),
	]
)
