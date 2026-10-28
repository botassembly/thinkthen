// swift-tools-version: 6.3
import PackageDescription

let package = Package(
  name: "Consumer", platforms: [.macOS("15.0")],
  dependencies: [.package(path: "../ThinkThen")],
  targets: [
    .executableTarget(
      name: "Consumer", dependencies: [.product(name: "ThinkThen", package: "ThinkThen")],
      path: ".", exclude: ["Package.swift", "Readme", "Guide", "minor"]),
    .executableTarget(
      name: "Readme", dependencies: [.product(name: "ThinkThen", package: "ThinkThen")],
      path: "Readme"),
    .executableTarget(
      name: "Guide", dependencies: [.product(name: "ThinkThen", package: "ThinkThen")],
      path: "Guide", exclude: ["records.jsonl"]),
  ],
  swiftLanguageModes: [.v6]
)
