// swift-tools-version: 5.7
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "AOC2022",
    platforms: [.macOS(.v12)],
    products: [
        .executable(name: "AOC2022", targets: ["AOC2022"]),
        .library(name: "AOCCore", targets: ["AOCCore"]),
    ],
    dependencies: [
        // Dependencies declare other packages that this package depends on.
        .package(url: "git@github.com:apple/swift-collections.git", from: "1.0.2"),
        .package(url: "git@github.com:apple/swift-algorithms.git", from: "1.0.0"),
        .package(url: "https://github.com/apple/swift-argument-parser", from: "1.0.0"),
    ],
    targets: [
        // Targets are the basic building blocks of a package. A target can define a module or a test suite.
        // Targets can depend on other targets in this package, and on products in packages this package depends on.
        .executableTarget(
            name: "AOC2022",
            dependencies: [
                "AOCCore",
                .product(name: "ArgumentParser", package: "swift-argument-parser")],
            resources: [
                .process("Resources")
            ]),
        .target(name: "AOCCore", dependencies: [
            .product(name: "Algorithms", package: "swift-algorithms"),
            .product(name: "Collections", package: "swift-collections"),
        ]),
        .testTarget(
            name: "AOCTests",
            dependencies: ["AOC2022"]),
    ]
)
