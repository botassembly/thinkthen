// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "FirstCall",
    dependencies: [.package(path: "thinkthen-swift")],
    targets: [
        .executableTarget(
            name: "FirstCall",
            dependencies: [
                .product(
                    name: "ThinkThen",
                    package: "thinkthen-swift"
                ),
            ],
            path: ".",
            sources: ["main.swift"]
        ),
    ]
)
