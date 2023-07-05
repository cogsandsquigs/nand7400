// swift-tools-version:5.3
import PackageDescription
import Foundation

let package = Package(
        name: "RustToSwift",
        platforms: [
            .iOS(.v13), 
            .macOS(.v11)
        ],
        products: [
            .library(
                name: "RustToSwift",
                targets: ["RustToSwift"]),
        ],
        targets: [
            .target(
                name: "RustToSwift",
                dependencies: ["MathWiz"]),
            .binaryTarget(
                name: "MathWiz",
                url: "https://github.com/cogsandsquigs/nand7400-asm/bundle.zip",
                checksum: "ea7a.....35b2"),
            .testTarget(
                name: "RustToSwiftTests",
                dependencies: ["RustToSwift"]),
        ]
)