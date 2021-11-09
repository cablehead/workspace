// swift-tools-version:5.5
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "Clip",
    platforms: [
        .macOS(.v10_12), // v10.12 is needed for scheduledTimer usage
    ],
    dependencies: [
        // Dependencies declare other packages that this package depends on.
        // .package(url: /* package url */, from: "1.0.0"),
    ],
    targets: [
        // Targets are the basic building blocks of a package. A target can define a module or a test suite.
        // Targets can depend on other targets in this package, and on products in packages this package depends on.
        .executableTarget(
            name: "Clip",
            dependencies: []
        ),
        .testTarget(
            name: "ClipTests",
            dependencies: ["Clip"]
        ),
    ]
)
