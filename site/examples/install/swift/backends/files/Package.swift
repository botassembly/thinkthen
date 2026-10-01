// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "Backends",
    dependencies: [.package(path: "thinkthen-swift")],
    targets: [
        .executableTarget(
            name: "Backends",
            dependencies: [
                .product(
                    name: "ThinkThen",
                    package: "thinkthen-swift"
                ),
            ],
            path: ".",
            sources: ["backends.swift"]
        ),
    ]
)
