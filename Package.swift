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
			checksum: "79560ec409aac96fbdb57dc469dd843492d106e9c54003156cf02c25e22d7703"
		),
		// .binaryTarget(
		// 	name: "Nand7400FFI",
		// 	path: "./target/Nand7400FFI.xcframework"
		// ),
	]
)
