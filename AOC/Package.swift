// swift-tools-version:5.5
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

func resourceList() -> [Resource] {
    (1...25).map {
        .copy("Resources/Day\($0).txt")
    }
}

let package = Package(
    name: "Advent of Code",
    platforms: [.macOS("10.15")],
    products: [
        .executable(name: "advent", targets: ["advent"]),
        .library(name: "AOC", targets: ["AOC"]),
        .library(name: "AOCCore", targets: ["AOCCore"]),
        .library(name: "AOC2021", targets: ["AOC2021"])
    ],
    dependencies: [
        // Dependencies declare other packages that this package depends on.
        // .package(url: /* package url */, from: "1.0.0"),
        .package(name: "swift-algorithms", url: "https://github.com/apple/swift-algorithms", from: "1.0.0")
    ],
    targets: [
        // Targets are the basic building blocks of a package. A target can define a module or a test suite.
        // Targets can depend on other targets in this package, and on products in packages this package depends on.
        .executableTarget(name: "advent", dependencies: ["AOC"]),
        .target(name: "AOC", dependencies: ["AOCCore", "AOC2021"]),
        .target(name: "AOC2021", dependencies: ["AOCCore"], resources: resourceList()),
        .target(name: "AOCCore", dependencies: [
            .product(name: "Algorithms", package: "swift-algorithms")
        ]),
        .testTarget(name: "AOCTests", dependencies: ["AOC"])
    ]
)
