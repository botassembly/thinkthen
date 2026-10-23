// swift-tools-version: 6.3
import Foundation
import PackageDescription

let root = URL(fileURLWithPath: #filePath).deletingLastPathComponent()
let sourceRoot =
  FileManager.default.fileExists(
    atPath: root.appendingPathComponent("sdk/swift/Package.swift").path) ? "sdk/swift/" : ""

let package = Package(
  name: "ThinkThen",
  platforms: [.macOS("15.0")],
  products: [.library(name: "ThinkThen", targets: ["ThinkThen"])],
  targets: [
    .target(
      name: "CThinkThen", path: sourceRoot + "Sources/CThinkThen", publicHeadersPath: "include"),
    .target(
      name: "ThinkThen", dependencies: ["CThinkThen"],
      path: sourceRoot + "Sources/ThinkThen",
      resources: [.copy("Native")],
      linkerSettings: [.linkedLibrary("dl", .when(platforms: [.linux]))]),
  ],
  swiftLanguageModes: [.v6]
)
