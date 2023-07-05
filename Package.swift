// swift-tools-version:5.3
import PackageDescription
import Foundation

let package = Package(
        name: "Nand7000Asm",
        platforms: [
            .iOS(.v13), 
            .macOS(.v11)
        ],
        products: [
            .library(
                name: "Nand7000Asm",
                targets: ["Nand7000Asm"]),
        ],
        targets: [
            .target(
                name: "Nand7000Asm",
                dependencies: ["nand7000_asm"]),
            .binaryTarget(
                name: "nand7000_asm",
                url: "https://github.com/cogsandsquigs/nand7400-asm/releases/download/0.0.0-0/bundle.zip",
                checksum: "789735c3609cff13b62bde42478a05ab445be02defcab832a90d635c5e3d5967"),
            .testTarget(
                name: "Nand7000AsmTests",
                dependencies: ["Nand7000Asm"]),
        ]
)