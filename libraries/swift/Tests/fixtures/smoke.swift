import Foundation

@main enum ReplaySmoke {
  static func main() async throws {
    let environment = ProcessInfo.processInfo.environment
    let client = try Client()
    defer { client.close() }
    let answer = try await client.decide(
      .text(environment["THINKTHEN_TEST_SMOKE_QUESTION"]!),
      input: InputText(text: environment["THINKTHEN_TEST_SMOKE_TEXT"]!))
    print("smoke: " + String(decoding: try (answer.value ?? .null).data(), as: UTF8.self))
  }
}
