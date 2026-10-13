import Foundation
import ThinkThen

@main enum OwnedConsumer {
    static func expect(_ condition: Bool, _ message: String) throws {
        if !condition { throw JSONConversionError(message) }
    }
    static func arrived(_ path: String) async throws {
        let until = ContinuousClock.now + .seconds(10)
        while !FileManager.default.fileExists(atPath: path) {
            if ContinuousClock.now >= until { throw JSONConversionError("Provider did not arrive") }
            try await Task.sleep(for: .milliseconds(10))
        }
    }
    static func main() async {
        do { try await run() }
        catch { FileHandle.standardError.write(Data("Swift consumer failed: \(error)\n".utf8)); exit(1) }
    }
    static func run() async throws {
        let raw = Data(#"{"future":1e400,"large":9007199254740993,"duplicate":1,"duplicate":2,"escaped":"\uD83D\uDE00"}"#.utf8)
        try expect(try JSONValue.parse(raw).data() == raw, "Lossless original JSON")
        let terminalBytes = Data(#"{"kind":"terminal","failure":null,"future":{"large":9007199254740993,"n":1e400}}"#.utf8)
        let terminal = try OwnedSessionPacketTerminal.read(JSONValue.parse(terminalBytes))
        try expect(!terminal.facts.isPresent && terminal.failure.isPresent && terminal.failure.value == nil, "Missing and null presence")
        try expect(try terminal.json.data() == terminalBytes, "Generated unknown-field round trip")
        for bad in [#""\uD800""#, #""\uDC00""#, "[01]", "[1e]", "[1.]", "[true,]"] {
            var refused = false
            do { _ = try JSONValue.parse(Data(bad.utf8)) } catch { refused = true }
            try expect(refused, "Invalid JSON accepted")
        }
        var refusedInfinity = false
        do { _ = try Double.infinity.json.data() } catch { refusedInfinity = true }
        try expect(refusedInfinity, "Infinity encoded")
        let settings = try InputEngineSettings.read(JSONValue.parse(Data(CommandLine.arguments[1].utf8)))
        let barrier = CommandLine.arguments[2]
        let heldClient = try Client(settings: settings)
        let held = Task { try await heldClient.decide(.text("Is it?"), input: .text("hold-owned-swift")) }
        try await arrived(barrier + "/arrived-hold-owned-swift")
        let progress = await Task { "independent" }.value
        try expect(progress == "independent", "Unrelated task stalled")
        held.cancel()
        do { _ = try await held.value; throw JSONConversionError("Cancelled task answered") }
        catch is CancellationError { }
        heldClient.close()
        try expect(!FileManager.default.fileExists(atPath: barrier + "/release-hold-owned-swift"), "Provider released before cancellation and cleanup")
        _ = FileManager.default.createFile(atPath: barrier + "/release-hold-owned-swift", contents: Data())
        let client = try Client(settings: settings)
        let call = try await client.decide(.text("Is it?"), input: .text("owned-swift"))
        let rows: [OwnedAtomicDecideValue] = call.packets.compactMap { if case .decideRow(let row) = $0 { return row.value }; return nil }
        guard let row = rows.first else { throw JSONConversionError("Missing typed decide row") }
        try expect(try row.value.json.data() == Data("true".utf8), "Typed native answer")
        try expect(call.terminal.facts.isPresent, "Native final facts missing")
        try expect(try call.decideAnswer() == true, "Normalized scalar answer")
        for (items, count) in [([InputRequestItem](), 0), ([.text("cardinality-one"), .text("cardinality-two")], 2)] {
            do {
                _ = try await client.decideAnswer(.text("Is it?"), input: .records(InputRequestInputRecords(items: items)))
                throw JSONConversionError("Answer cardinality accepted")
            } catch let failure as AnswerCardinalityError {
                try expect(failure.count == count && failure.call.terminal.facts.isPresent, "Wrong answer cardinality or missing native settlement")
            }
        }
        let unknown = try await client.decideAnswer(.definition(InputRequestQuestionDefinition(value: .alternative0(InputRequestDefinitionAuthoredDecide(decide: .alternative0("Is it?"), threshold: .value(.alternative1("0.2:0.8")))))), input: .text("unsure"))
        try expect(unknown == nil, "Nullable normalized decision")
        let unknownChoice = try await client.chooseAnswer(.definition(InputRequestQuestionDefinition(value: .alternative1(InputRequestDefinitionAuthoredChoose(choose: .alternative0("Which?"), options: .value(.alternative0(["a", "b"])), threshold: .value(.alternative0(0.95)))))), input: .text("uncertain-choice"))
        try expect(unknownChoice == nil, "Nullable normalized choice")
        let authoredNull = try await client.decide(.definition(InputRequestQuestionDefinition(value: .alternative0(InputRequestDefinitionAuthoredDecide(decide: .alternative0("Is it?"), true: .value(.alternative3(.null)))))), input: .text("authored-null"))
        try expect(try authoredNull.decideAnswer() == true, "Authored null replaced normalized judgment")
        let explicit = try client.start(InputRequest(call: InputRequestCall(function: .decide, input: .text("hold-explicit-swift"), question: .text("Is it?")), schema: .thinkthenRequest_1))
        try explicit.finish()
        try await arrived(barrier + "/arrived-hold-explicit-swift")
        explicit.cancel()
        _ = FileManager.default.createFile(atPath: barrier + "/release-hold-explicit-swift", contents: Data())
        var settled: OwnedSessionPacketTerminal?
        while let packet = try await explicit.read() { if case .terminal(let value) = packet { settled = value } }
        explicit.close()
        try expect(settled?.facts.isPresent == true && settled?.failure.isPresent == true, "Explicit post-cancel drain lost terminal facts")
        let facts = try call.terminal.json.data()
        do { _ = try await client.decide(.text("Is it?"), input: .text("status-401")); throw JSONConversionError("Backend failure answered") }
        catch let failure as SessionFailure {
            guard case .backend = failure.kind else { throw JSONConversionError("Wrong typed failure kind") }
            try expect(failure.failure.facts.isPresent && failure.call.terminal.facts.isPresent, "Failure facts lost")
        }
        let recognition = try await client.recognize(.definition(InputRequestQuestionDefinition(value: .alternative6(InputRequestDefinitionRecognizeDefinition(recognize: InputRecognizeQuestion(kinds: .value(InputOrderedMap([.init("person", InputAuthoredDescription.alternative3(.null))]))))))), input: .text("Ana Lima"))
        let aggregateCount = recognition.packets.filter { if case .recognizeAggregate = $0 { return true }; return false }.count
        try expect(aggregateCount == 1, "Recognize did not emit exactly one aggregate")
        client.close()
        try expect(try call.terminal.json.data() == facts, "Result ownership ended with client")
        print("OWNED_SWIFT_PASS typed answer/failure, presence, lossless JSON, independent task, cancellation and cleanup")
    }
}
