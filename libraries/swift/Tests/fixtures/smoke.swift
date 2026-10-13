import Foundation

@main enum ReplaySmoke {
    static func main() async throws {
        let environment = ProcessInfo.processInfo.environment
        let client = try Client()
        defer { client.close() }
        let answer = try await client.decideAnswer(.text(environment["THINKTHEN_TEST_SMOKE_QUESTION"]!), input: .text(environment["THINKTHEN_TEST_SMOKE_TEXT"]!))
        print("smoke: " + (answer.map { String($0) } ?? "null"))
    }
}
