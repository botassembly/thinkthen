// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "ThinkThen",
    products: [.library(name: "ThinkThen", targets: ["ThinkThen"])],
    targets: [
        .systemLibrary(name: "CThinkThen", path: "Sources/CThinkThen"),
        .target(name: "ThinkThen", dependencies: ["CThinkThen"], path: "Sources/ThinkThen"),
        .executableTarget(name: "ThinkThenExample", dependencies: ["ThinkThen"], path: "Examples"),
        .executableTarget(name: "ThinkThenTypeCase", dependencies: ["ThinkThen"], path: "Tests/TypeCase"),
    ]
)
