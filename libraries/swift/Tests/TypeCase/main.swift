import Foundation
import ThinkThen

@main enum SharedConsumer {
    static func run(_ fixture: [String: Any], settings: String, originalJSON: JSONValue) async throws -> OwnedCall {
        let client = try Client(settings: InputEngineSettings.read(JSONValue.parse(Data(settings.utf8))))
        defer { client.close() }
        let verb = fixture["verb"] as! String
        func value(_ object: Any) throws -> JSONValue {
            try JSONValue.parse(JSONSerialization.data(withJSONObject: object, options: [.fragmentsAllowed]))
        }
        let preserved: [String: JSONValue]
        if case .orderedObject(let members) = originalJSON.kind { preserved = Dictionary(members.map { ($0.key, $0.value) }, uniquingKeysWith: { _, last in last }) }
        else { throw JSONConversionError("Fixture is not an object") }
                let question: InputRequestQuestion
        switch fixture["loader"] as? String {
        case "named", "load_named": question = .name(fixture["reference"] as! String)
        case "reference", "load_reference": question = .reference(fixture["reference"] as! String)
        case "file", "load": question = .file(fixture["reference"] as! String)
        default:
            if fixture["question_form"] as? String == "file" { question = .file("fixture-question.json") }
            else if let raw = fixture["raw"] as? String {
                try Data(raw.utf8).write(to: URL(fileURLWithPath: "raw-question.json"))
                question = .file("raw-question.json")
            } else {
                var authored = preserved["question"]!
                if case .orderedObject(let members) = authored.kind {
                    var members = verb == "find" ? members.filter { $0.key != "none" } : members
                    if let metadata = preserved["metadata"], case .orderedObject(let extra) = metadata.kind { members += extra }
                    authored = .orderedObject(members)
                }
                let role: QuestionGrammar = verb == "recognize" ? .recognize : verb == "relate" ? .relate : verb == "annotate" ? .set : verb == "rank" ? ((fixture["question"] as! [String: Any])["questions"] == nil ? .rank : .rankSet) : verb == "find" ? .find : .atomic
                question = try client.parseQuestion(role, authored: authored)
            }
        }
        let injection = (fixture["operation"] as? [String: Any])?["injection"] as? String
        var input: InputRequestInput
        if let paths = fixture["paths"] as? [String], !paths.isEmpty {
            let unit = fixture["source_unit"] as? Int ?? 3
            var source: [String: Any] = ["paths": paths, "reading": ["unit": unit == 2 ? "window" : (unit == 1 || unit == 5 ? "line" : "file")]]
            if unit == 2, let window = fixture["window"] as? Int, window != 0 { source["reading"] = ["unit": "window", "window": window] }
            if unit == 5 || fixture["owned_jsonl"] as? Bool == true { source["framing"] = "jsonl" }
            if unit == 4 || fixture["image_reader"] as? Bool == true { source["media"] = "image" }
            input = try InputRequestInput.read(value(["kind": "source", "source": source]))
        } else {
            let images: [[String: Any]] = (fixture["image_paths"] as? [String] ?? []).map { ["kind": "file", "path": $0, "media": fixture["media"] ?? "image/png"] }
            var records: [[String: Any]] = []
            for (index, original) in (fixture["items"] as? [Any] ?? []).enumerated() {
                var item: [String: Any] = [:]
                if fixture["image_only"] as? Bool != true {
                    let original = fixture["caption_files"] as? Bool == true ? try String(contentsOfFile: "caption-\(index).txt", encoding: .utf8) : original
                    item["original"] = fixture["text"] as? Bool == true && original is String ? ["kind": "text", "text": original] : ["kind": "json", "value": original]
                }
                if let contexts = fixture["contexts"] as? [Any] { item["context"] = contexts[index] }
                else if fixture["context_present"] as? Bool == true { item["context"] = fixture["context"] ?? NSNull() }
                if let orders = fixture["candidate_orders"] as? [[String]] { item["options"] = orders[index].map { ["name": $0] } }
                if !images.isEmpty { item["images"] = images }
                records.append(item)
            }
            input = try InputRequestInput.read(value(["kind": verb == "find" ? "units" : verb == "relate" ? "entities" : "records", "items": records]))
        }
        var options: [String: Any] = ["attempts": true]
        if let context = fixture["shared_context"] as? String { options["context"] = context }
        if verb == "find" { options["none"] = (fixture["question"] as? [String: Any])?["none"] as? Bool ?? false }
        if injection == "expired_deadline" { options["deadline_ms"] = 0 }
        if fixture["context_present"] as? Bool == true, fixture["context"] is NSNull, fixture["contexts"] is NSNull || fixture["contexts"] == nil {
            let first = (fixture["items"] as! [Any])[0]
            input = .json(InputRequestInputJson(value: try value(["item": first, "context": NSNull()])))
            options["field"] = ["/item"]
            options["context_field"] = "/context"
        }
        let typedOptions = Presence.value(try InputRequestOptions.read(value(options)))
        let worker = Task {
            switch verb {
            case "decide": return try await client.decide(question, input: input, options: typedOptions)
            case "choose": return try await client.choose(question, input: input, options: typedOptions)
            case "tag": return try await client.tag(question, input: input, options: typedOptions)
            case "score": return try await client.score(question, input: input, options: typedOptions)
            case "filter": return try await client.filter(question, input: input, options: typedOptions)
            case "rank": return try await client.rank(question, input: input, options: typedOptions)
            case "find": return try await client.find(question, input: input, options: typedOptions)
            case "annotate": return try await client.annotate(question, input: input, options: typedOptions)
            case "recognize": return try await client.recognize(question, input: input, options: typedOptions)
            case "relate": return try await client.relate(question, input: input, options: typedOptions)
            default: throw JSONConversionError("Unknown fixture function")
            }
        }
        if injection == "cancel_token" { worker.cancel() }
        if fixture["held_cancel"] as? Bool == true {
            Task.detached {
                _ = FileHandle.standardInput.readData(ofLength: 1)
                worker.cancel()
                FileHandle.standardOutput.write(Data("cancel-fired\n".utf8))
            }
        }
        return try await worker.value
    }
    static func main() async throws {
        let bytes = try Data(contentsOf: URL(fileURLWithPath: CommandLine.arguments[1]))
        let originalJSON = try JSONValue.parse(bytes)
        let fixture = try JSONSerialization.jsonObject(with: bytes) as! [String: Any]
        let payload: JSONValue
        do {
            let call = try await run(fixture, settings: CommandLine.arguments[2], originalJSON: originalJSON)
            payload = .object(["packets": .array(call.packets.map(\.json))])
        } catch let failure as SessionFailure {
            payload = .object(["packets": .array(failure.call.packets.map(\.json))])
        } catch let failure as SessionBoundaryFailure {
            payload = try JSONValue.parse(JSONSerialization.data(withJSONObject: ["admission": ["code": failure.code, "message": failure.message]]))
        } catch is CancellationError {
            payload = try JSONValue.parse(Data(#"{"admission":{"code":5,"message":"the call was cancelled"}}"#.utf8))
        }
        print(String(decoding: try payload.data(), as: UTF8.self))
    }
}
