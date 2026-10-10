// swift-tools-version: 6.0
import PackageDescription
import Foundation

#if os(macOS)
let nativeTarget: Target = .binaryTarget(name: "CThinkThen", path: "CThinkThen.xcframework")
#elseif os(Linux)
#if !arch(x86_64) && !arch(arm64)
#error("ThinkThen has no native asset for this Swift host")
#endif
let nativeTarget: Target = .target(
    name: "CThinkThen", path: "Sources/CThinkThen", publicHeadersPath: "include",
    linkerSettings: [.linkedLibrary("dl"), .linkedLibrary("pthread")])
#else
#error("ThinkThen Swift has no native package for this operating system")
#endif

#if os(Linux)
let nativeResources: [Resource] = [.copy("Native")]
#else
let nativeResources: [Resource] = []
#endif

let package = Package(
    name: "ThinkThen",
    platforms: [.macOS(.v15)],
    products: [.library(name: "ThinkThen", targets: ["ThinkThen"])],
    targets: [
        nativeTarget,
        .target(name: "ThinkThen", dependencies: ["CThinkThen"], path: "Sources/ThinkThen", resources: nativeResources),
        .executableTarget(name: "ThinkThenExample", dependencies: ["ThinkThen"], path: "Examples"),
        .executableTarget(name: "ThinkThenTypeCase", dependencies: ["ThinkThen"], path: "Tests/TypeCase"),
    ]
)
