import Foundation

@main enum ReplaySmoke {
    static func main() async throws {
        let environment = ProcessInfo.processInfo.environment
        let client = try Client()
        defer { client.close() }
        let call = try await client.decide(.text(environment["THINKTHEN_TEST_SMOKE_QUESTION"]!), input: .text(environment["THINKTHEN_TEST_SMOKE_TEXT"]!))
        for packet in call.packets {
            if case .decideRow(let row) = packet {
                print("smoke: " + String(decoding: try row.value.value.json.data(), as: UTF8.self))
            }
        }
    }
}
