// swift-tools-version: 6.3
import PackageDescription

let package = Package(
  name: "Example",
  platforms: [.macOS("15.0")],
  dependencies: [
    .package(url: "https://github.com/botassembly/thinkthen", exact: "0.2.0"),
  ],
  targets: [
    .executableTarget(
      name: "Example",
      dependencies: [.product(name: "ThinkThen", package: "thinkthen")],
      path: "Sources/Example"),
  ]
)
