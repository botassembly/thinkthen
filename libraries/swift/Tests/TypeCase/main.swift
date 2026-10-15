import Foundation

@testable import ThinkThen

struct DriverCall: Sendable {
  let call: Call
  let result: DataValue?
}

@main enum SharedConsumer {
  static func describe<V: ValueRepresentable, P: ValueRepresentable, R: ValueRepresentable>(
    _ result: CallResult<V, P, R>
  ) -> DriverCall {
    DriverCall(
      call: result.call,
      result: .object([
        "value": result.value.json, "probability": result.probability.json,
        "results": result.results.json, "facts": result.facts.json,
      ]))
  }
  static func run(_ fixture: [String: Any], settings: String, originalJSON: DataValue) async throws
    -> DriverCall
  {
    let client = try Client(settings: EngineSettings.read(DataValue.parse(Data(settings.utf8))))
    defer { client.close() }
    let verb = fixture["verb"] as! String
    let contract =
      fixture["call_result_contract"] as? Bool == true
      || (fixture["expect"] as? [String: Any])?["call_result"] != nil
    func value(_ object: Any) throws -> DataValue {
      try DataValue.parse(
        JSONSerialization.data(withJSONObject: object, options: [.fragmentsAllowed]))
    }
    let preserved: [String: DataValue]
    if case .orderedObject(let members) = originalJSON.kind {
      preserved = Dictionary(
        members.map { ($0.key, $0.value) }, uniquingKeysWith: { _, last in last })
    } else {
      throw ValueConversionError("Fixture is not an object")
    }
    let question: Question
    switch fixture["loader"] as? String {
    case "named", "load_named": question = .name(fixture["reference"] as! String)
    case "reference", "load_reference": question = .reference(fixture["reference"] as! String)
    case "file", "load": question = .file(fixture["reference"] as! String)
    default:
      if fixture["question_form"] as? String == "file" {
        question = .file("fixture-question.json")
      } else {
        var authored =
          try (fixture["raw"] as? String).map { try DataValue.parse(Data($0.utf8)) } ?? preserved[
            "question"]!
        if case .orderedObject(let members) = authored.kind {
          authored = .orderedObject(verb == "find" ? members.filter { $0.key != "none" } : members)
        }
        let role: QuestionGrammar =
          verb == "recognize"
          ? .recognize
          : verb == "relate"
            ? .relate
            : verb == "annotate"
              ? .set
              : verb == "rank"
                ? ((fixture["question"] as! [String: Any])["questions"] == nil ? .rank : .rankSet)
                : verb == "find" ? .find : .atomic
        question = try client.parseQuestion(role, authored: authored)
      }
    }
    let injection = (fixture["operation"] as? [String: Any])?["injection"] as? String
    let contexts = fixture["contexts"] as? [Any]
    let projectContext =
      contexts?.contains { !($0 is String || $0 is [String: Any] || $0 is NSNull) } ?? false
    var input: Input
    if let paths = fixture["paths"] as? [String], !paths.isEmpty {
      let unit = fixture["source_unit"] as? Int ?? 3
      var source: [String: Any] = [
        "paths": paths,
        "reading": ["unit": unit == 2 ? "window" : (unit == 1 || unit == 5 ? "line" : "file")],
      ]
      if unit == 2, let window = fixture["window"] as? Int, window != 0 {
        source["reading"] = ["unit": "window", "window": window]
      }
      if unit == 5 || fixture["owned_jsonl"] as? Bool == true { source["framing"] = "jsonl" }
      if unit == 4 || fixture["image_reader"] as? Bool == true { source["media"] = "image" }
      input = try Input.read(value(["kind": "source", "source": source]))
    } else {
      let images: [[String: Any]] = try (fixture["image_paths"] as? [String] ?? []).map {
        [
          "kind": "bytes",
          "bytes": try Data(contentsOf: URL(fileURLWithPath: $0)).base64EncodedString(),
          "media": fixture["media"] ?? "image/png",
        ]
      }
      var records: [[String: Any]] = []
      for (index, original) in (fixture["items"] as? [Any] ?? []).enumerated() {
        var item: [String: Any] = [:]
        if fixture["image_only"] as? Bool != true {
          let original =
            fixture["caption_files"] as? Bool == true
            ? try String(contentsOfFile: "caption-\(index).txt", encoding: .utf8) : original
          item["original"] =
            fixture["text"] as? Bool == true && original is String
            ? ["kind": "text", "text": original] : ["kind": "json", "value": original]
        }
        if projectContext, let contexts {
          item["original"] = [
            "kind": "json", "value": ["item": original, "context": contexts[index]],
          ]
        } else if let contexts {
          item["context"] = contexts[index]
        } else if fixture["context_present"] as? Bool == true {
          item["context"] = fixture["context"] ?? NSNull()
        }
        if let orders = fixture["candidate_orders"] as? [[String]] {
          item["options"] = orders[index].map { ["name": $0] }
        }
        if !images.isEmpty { item["images"] = images }
        records.append(item)
      }
      input = try Input.read(
        value([
          "kind": verb == "find" ? "units" : verb == "relate" ? "entities" : "records",
          "items": records,
        ]))
    }
    if contract, let originals = fixture["items"] as? [Any], originals.count == 1 {
      if fixture["text"] as? Bool == true, let text = originals[0] as? String {
        input = .text(InputText(text: text))
      } else {
        input = .json(InputJson(value: try value(originals[0])))
      }
    }
    var options: [String: Any] = ["attempts": true]
    if projectContext {
      options["field"] = ["/item"]
      options["context_field"] = "/context"
    }
    if let context = fixture["shared_context"] as? String { options["context"] = context }
    if verb == "find" {
      options["none"] = (fixture["question"] as? [String: Any])?["none"] as? Bool ?? false
    }
    if injection == "expired_deadline" { options["deadline_ms"] = 0 }
    if fixture["context_present"] as? Bool == true, fixture["context"] is NSNull,
      fixture["contexts"] is NSNull || fixture["contexts"] == nil
    {
      let first = (fixture["items"] as! [Any])[0]
      input = .json(InputJson(value: try value(["item": first, "context": NSNull()])))
      options["field"] = ["/item"]
      options["context_field"] = "/context"
    }
    let typedOptions = Presence.value(try Options.read(value(options)))
    let incremental = fixture["incremental"] as? Bool == true
    let worker = Task {
      if incremental, case .records(let records) = input {
        let request = Request(
          call: CallRequest(
            function: .decide, input: .feed(InputFeed(name: "records")),
            options: typedOptions, question: question), schema: .thinkthenRequest_1)
        let session = try client.start(request)
        defer { session.close() }
        for item in records.items {
          if try await !session.push(SessionDescriptor(item: item)) { break }
        }
        try session.finish()
        var packets: [SessionPacket] = []
        var terminal: SessionPacketTerminal?
        while let packet = try await session.read() {
          packets.append(packet)
          if case .terminal(let value) = packet { terminal = value }
        }
        guard let terminal else { throw ValueConversionError("Native End has no terminal") }
        return DriverCall(call: Call(packets: packets, terminal: terminal), result: nil)
      }
      switch verb {
      case "decide":
        return try await namedDecide(client, question, input, typedOptions)
      case "choose":
        return try await namedChoose(client, question, input, typedOptions)
      case "tag":
        return try await namedTag(client, question, input, typedOptions)
      case "score":
        return try await namedScore(client, question, input, typedOptions)
      case "filter":
        return describe(try await client.filter(question, input: input, options: typedOptions))
      case "rank":
        return describe(try await client.rank(question, input: input, options: typedOptions))
      case "find":
        return describe(try await client.find(question, input: input, options: typedOptions))
      case "annotate":
        return try await namedAnnotate(client, question, input, typedOptions)
      case "recognize":
        return try await namedRecognize(client, question, input, typedOptions)
      case "relate":
        return describe(try await client.relate(question, input: input, options: typedOptions))
      default: throw ValueConversionError("Unknown fixture function")
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
  static func namedDecide(
    _ client: Client, _ question: Question, _ input: Input, _ options: Presence<Options>
  ) async throws -> DriverCall {
    switch input {
    case .text(let value):
      return describe(try await client.decide(question, input: value, options: options))
    case .json(let value):
      return describe(try await client.decide(question, input: value, options: options))
    case .records(let value):
      return describe(try await client.decide(question, input: value, options: options))
    case .source(let value):
      return describe(try await client.decide(question, input: value, options: options))
    case .feed(let value):
      let feed = AsyncThrowingStream<SessionDescriptor, any Error> { $0.finish() }
      return describe(try await client.decide(question, input: value, options: options, feed: feed))
    case .units(let value):
      return describe(try await client.decide(question, input: value, options: options))
    case .entities(let value):
      return describe(try await client.decide(question, input: value, options: options))
    }
  }
  static func namedChoose(
    _ client: Client, _ question: Question, _ input: Input, _ options: Presence<Options>
  ) async throws -> DriverCall {
    switch input {
    case .text(let value):
      return describe(try await client.choose(question, input: value, options: options))
    case .json(let value):
      return describe(try await client.choose(question, input: value, options: options))
    case .records(let value):
      return describe(try await client.choose(question, input: value, options: options))
    case .source(let value):
      return describe(try await client.choose(question, input: value, options: options))
    case .feed(let value):
      let feed = AsyncThrowingStream<SessionDescriptor, any Error> { $0.finish() }
      return describe(try await client.choose(question, input: value, options: options, feed: feed))
    case .units(let value):
      return describe(try await client.choose(question, input: value, options: options))
    case .entities(let value):
      return describe(try await client.choose(question, input: value, options: options))
    }
  }
  static func namedTag(
    _ client: Client, _ question: Question, _ input: Input, _ options: Presence<Options>
  ) async throws -> DriverCall {
    switch input {
    case .text(let value):
      return describe(try await client.tag(question, input: value, options: options))
    case .json(let value):
      return describe(try await client.tag(question, input: value, options: options))
    case .records(let value):
      return describe(try await client.tag(question, input: value, options: options))
    case .source(let value):
      return describe(try await client.tag(question, input: value, options: options))
    case .feed(let value):
      let feed = AsyncThrowingStream<SessionDescriptor, any Error> { $0.finish() }
      return describe(try await client.tag(question, input: value, options: options, feed: feed))
    case .units(let value):
      return describe(try await client.tag(question, input: value, options: options))
    case .entities(let value):
      return describe(try await client.tag(question, input: value, options: options))
    }
  }
  static func namedScore(
    _ client: Client, _ question: Question, _ input: Input, _ options: Presence<Options>
  ) async throws -> DriverCall {
    switch input {
    case .text(let value):
      return describe(try await client.score(question, input: value, options: options))
    case .json(let value):
      return describe(try await client.score(question, input: value, options: options))
    case .records(let value):
      return describe(try await client.score(question, input: value, options: options))
    case .source(let value):
      return describe(try await client.score(question, input: value, options: options))
    case .feed(let value):
      let feed = AsyncThrowingStream<SessionDescriptor, any Error> { $0.finish() }
      return describe(try await client.score(question, input: value, options: options, feed: feed))
    case .units(let value):
      return describe(try await client.score(question, input: value, options: options))
    case .entities(let value):
      return describe(try await client.score(question, input: value, options: options))
    }
  }
  static func namedAnnotate(
    _ client: Client, _ question: Question, _ input: Input, _ options: Presence<Options>
  ) async throws -> DriverCall {
    switch input {
    case .text(let value):
      return describe(try await client.annotate(question, input: value, options: options))
    case .json(let value):
      return describe(try await client.annotate(question, input: value, options: options))
    case .records(let value):
      return describe(try await client.annotate(question, input: value, options: options))
    case .source(let value):
      return describe(try await client.annotate(question, input: value, options: options))
    case .feed(let value):
      let feed = AsyncThrowingStream<SessionDescriptor, any Error> { $0.finish() }
      return describe(
        try await client.annotate(question, input: value, options: options, feed: feed))
    case .units(let value):
      return describe(try await client.annotate(question, input: value, options: options))
    case .entities(let value):
      return describe(try await client.annotate(question, input: value, options: options))
    }
  }
  static func namedRecognize(
    _ client: Client, _ question: Question, _ input: Input, _ options: Presence<Options>
  ) async throws -> DriverCall {
    switch input {
    case .text(let value):
      return describe(try await client.recognize(question, input: value, options: options))
    case .json(let value):
      return describe(try await client.recognize(question, input: value, options: options))
    case .records(let value):
      return describe(try await client.recognize(question, input: value, options: options))
    case .source(let value):
      return describe(try await client.recognize(question, input: value, options: options))
    case .feed(let value):
      let feed = AsyncThrowingStream<SessionDescriptor, any Error> { $0.finish() }
      return describe(
        try await client.recognize(question, input: value, options: options, feed: feed))
    case .units(let value):
      return describe(try await client.recognize(question, input: value, options: options))
    case .entities(let value):
      return describe(try await client.recognize(question, input: value, options: options))
    }
  }
  static func main() async throws {
    let bytes = try Data(contentsOf: URL(fileURLWithPath: CommandLine.arguments[1]))
    let originalJSON = try DataValue.parse(bytes)
    let fixture = try JSONSerialization.jsonObject(with: bytes) as! [String: Any]
    let payload: DataValue
    do {
      let call = try await run(
        fixture, settings: CommandLine.arguments[2], originalJSON: originalJSON)
      var fields: [String: DataValue] = ["packets": .array(call.call.packets.map(\.json))]
      if fixture["call_result_contract"] as? Bool == true
        || (fixture["expect"] as? [String: Any])?["call_result"] != nil, let result = call.result
      {
        fields["call_result"] = result
      }
      payload = .object(fields)
    } catch let failure as SessionFailure {
      payload = .object(["packets": .array(failure.call.packets.map(\.json))])
    } catch let failure as SessionBoundaryFailure {
      payload = try DataValue.parse(
        JSONSerialization.data(withJSONObject: [
          "admission": ["code": failure.code, "message": failure.message]
        ]))
    } catch is CancellationError {
      payload = try DataValue.parse(
        Data(#"{"admission":{"code":5,"message":"the call was cancelled"}}"#.utf8))
    }
    print(String(decoding: try payload.data(), as: UTF8.self))
  }
}
