import Foundation

@main
enum PortableBatch {
    static func main() throws {
        guard CommandLine.arguments.count == 2 else { fatalError("portable corpus required") }
        let data = try Data(contentsOf: URL(fileURLWithPath: CommandLine.arguments[1]))
        let corpus = try JSONSerialization.jsonObject(with: data) as! [String: Any]
        let question = corpus["question"] as! String
        let texts = corpus["texts"] as! [String]
        let engine = try ProcessInfo.processInfo.environment["TT_PORTABLE_SETTINGS"].map { try Engine(settingsJSON: $0) } ?? Engine()
        defer { engine.close() }
        let answers = try engine.decideMany(question, texts)
        precondition(answers.value.count == 5 && answers.value.allSatisfy { $0.outcome == .yes && $0.probability == 0.9 })
        print("SWIFT_PORTABLE_BATCH_PASS")
    }
}
