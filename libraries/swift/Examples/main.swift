import Foundation
import ThinkThen

@main enum Example {
    static func main() async throws {
        let client = try Client()
        defer { client.close() }
        let call = try await client.decide(.text("Is it?"), input: .text("consumer-swift"))
        for packet in call.packets { print(String(decoding: try packet.json.data(), as: UTF8.self)) }
    }
}
