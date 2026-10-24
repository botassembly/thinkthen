// swift-tools-version: 6.3
import PackageDescription

let package = Package(
  name: "Consumer", platforms: [.macOS("15.0")],
  dependencies: [.package(path: "../ThinkThen")],
  targets: [
    .executableTarget(
      name: "Consumer", dependencies: [.product(name: "ThinkThen", package: "ThinkThen")],
      path: ".", exclude: ["Package.swift", "Readme"]),
    .executableTarget(
      name: "Readme", dependencies: [.product(name: "ThinkThen", package: "ThinkThen")],
      path: "Readme"),
  ],
  swiftLanguageModes: [.v6]
)
