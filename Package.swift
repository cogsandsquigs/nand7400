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
	dependencies: [],
	targets: [
		.target(
			name: "Nand7400Asm",
			dependencies: ["Nand7400AsmFFI"],
			path: "nand7400asm-swift"
		),
		.binaryTarget(
			name: "Nand7400AsmFFI",
			path: "./target/Nand7400AsmFFI.xcframework"
		),
	]
)
