// swift-tools-version:5.3

import Foundation
import PackageDescription

let package = Package(
	name: "Nand7400Asm",
	platforms: [.iOS(.v13), .macOS(.v10_15)],
	products: [
		.library(
			name: "Nand7400Asm",
			targets: ["Nand7400Asm"]
		),
	],
	dependencies: [
		// .package(url: "https://github.com/apple/swift-docc-plugin", from: "1.1.0"),
	],
	targets: [
		// FFIbinaryTarget,
		// .target(
		// 	name: "Nand7400AsmFFI",
		// 	dependencies: ["yniffiFFI"],
		// 	path: "lib/swift/scaffold"
		// ),
		.target(
			name: "Nand7400Asm",
			dependencies: ["Nand7400AsmFFI"],
			path: "nand7400asm-swift"
			// path: "asdfasdf",
			// swiftSettings: globalSwiftSettings
		),
		.binaryTarget(
			name: "Nand7400AsmFFI",
			path: "./target/Nand7400AsmFFI.xcframework"
		),
		// .testTarget(
		// 	name: "YSwiftTests",
		// 	dependencies: ["YSwift"]
		// ),
	]
)
