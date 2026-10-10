// swift-tools-version: 6.0
import PackageDescription
import Foundation

#if os(macOS)
let nativeTarget: Target = .binaryTarget(name: "CThinkThen", path: "CThinkThen.xcframework")
#elseif os(Linux)
#if arch(x86_64)
let nativeTriple = "x86_64-unknown-linux-gnu"
#elseif arch(arm64)
let nativeTriple = "aarch64-unknown-linux-gnu"
#else
#error("ThinkThen has no native asset for this Swift host")
#endif
let nativeRoot = URL(fileURLWithPath: #filePath).deletingLastPathComponent().appendingPathComponent("native/" + nativeTriple).path
let nativeTarget: Target = .target(
    name: "CThinkThen", path: "Sources/CThinkThen", publicHeadersPath: "include",
    linkerSettings: [.unsafeFlags([nativeRoot + "/lib/libthinkthen.a"]),
                     .linkedLibrary("gcc_s"), .linkedLibrary("util"), .linkedLibrary("rt"),
                     .linkedLibrary("pthread"), .linkedLibrary("m"), .linkedLibrary("dl"), .linkedLibrary("c")])
#else
#error("ThinkThen Swift has no native package for this operating system")
#endif

let package = Package(
    name: "ThinkThen",
    platforms: [.macOS(.v15)],
    products: [.library(name: "ThinkThen", targets: ["ThinkThen"])],
    targets: [
        nativeTarget,
        .target(name: "ThinkThen", dependencies: ["CThinkThen"], path: "Sources/ThinkThen"),
        .executableTarget(name: "ThinkThenExample", dependencies: ["ThinkThen"], path: "Examples"),
        .executableTarget(name: "ThinkThenTypeCase", dependencies: ["ThinkThen"], path: "Tests/TypeCase"),
    ]
)
