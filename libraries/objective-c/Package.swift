// swift-tools-version: 6.0
import PackageDescription
let package = Package(
    name: "ThinkThenFoundation",
    platforms: [.macOS(.v15)],
    products: [.library(name: "ThinkThenFoundation", targets: ["ThinkThen"])],
    targets: [
        .binaryTarget(name: "CThinkThen", path: "CThinkThen.xcframework"),
        .target(name: "ThinkThen", dependencies: ["CThinkThen"],
                path: "Sources/Foundation", publicHeadersPath: ".",
                cSettings: [.unsafeFlags(["-fobjc-arc", "-fblocks"])],
                linkerSettings: [.linkedFramework("Foundation")])
    ]
)
