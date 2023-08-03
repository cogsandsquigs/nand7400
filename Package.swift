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
			url: "https://github.com/cogsandsquigs/nand7400/releases/download/v0.4.0-alpha.0/Nand7400FFI.xcframework.zip",
			checksum: "bcd5c6b14a72c8c94d33a63256a95bdd271c2636135beb1953d5cbc3e3054a06"
		),
		// .binaryTarget(
		// 	name: "Nand7400FFI",
		// 	path: "./target/Nand7400FFI.xcframework"
		// ),
	]
)
