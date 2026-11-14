import Foundation
import ThinkThen

func encoded<T: Encodable>(_ value: T) throws -> Data { try JSONEncoder().encode(value) }
func decoded<T: Decodable>(_ type: T.Type, _ value: Any) throws -> T {
  try JSONDecoder().decode(
    type, from: JSONSerialization.data(withJSONObject: value, options: [.fragmentsAllowed]))
}
func diagnostic(_ error: any Error) -> RowFailure? {
  guard let typed = error as? any ThinkThenError else { return nil }
  return RowFailure(
    code: typed.code, message: typed.message, httpStatus: typed.httpStatus)
}
func httpStatus(_ failure: RowFailure) -> Data {
  failure.httpStatus.map { Data(",\"http_status\":\($0)".utf8) } ?? Data()
}
func collect<E: Sendable & Encodable>(_ rows: Rows<E>, cancelled: Bool = false) async throws -> Data
{
  if cancelled { rows.cancel() }
  var values: [E] = []
  do {
    for try await row in rows { values.append(row) }
  } catch {
    if let failure = diagnostic(error) {
      return Data("{\"error\":".utf8) + (try encoded(failure.code)) + httpStatus(failure)
        + Data(",\"rows\":".utf8)
        + (try encoded(values)) + Data("}".utf8)
    }
    if error is CancellationError {
      return Data(#"{"error":"local.cancelled","rows":"#.utf8) + (try encoded(values))
        + Data("}".utf8)
    }
    throw error
  }
  return try encoded(values)
}

// Feature: installed Swift argument mapping and typed rows for shared cases.
@MainActor func run(_ test: [String: Any], url: String?, single: Bool = false) async throws -> Data
{
  var settings = test["client"] as? [String: Any] ?? [:]
  if let url { settings["url"] = url }
  // The public Retry initializer requires a complete policy. Give partial
  // fixture policies explicit test durations, rather than copying defaults.
  if var retry = settings["retry"] as? [String: Any] {
    retry["initial_backoff_ms"] = retry["initial_backoff_ms"] ?? 1
    retry["attempt_timeout_ms"] = retry["attempt_timeout_ms"] ?? 10000
    settings["retry"] = retry
  }
  let client = try Client(decoded(Settings.self, settings))
  let request = test["request"] as! [String: Any]
  let question = request["question"] as! [String: Any]
  let name = request["name"] as? String
  let cancelled = test["cancelled"] as? Bool ?? false
  let records = try (request["records"] as? [Any] ?? []).map { value -> Record in
    if value is NSNull { return .missing }
    if let text = value as? String { return .text(text) }
    return .json(try decoded(JSONValue.self, value))
  }
  if single {
    precondition(records.count == 1 && !cancelled && question["function"] as? String == "decide")
    let row = try await client.decide(
      decoded(DecideQuestion.self, question), record: records[0], name: name)
    return try encoded([row])
  }
  switch question["function"] as! String {
  case "decide":
    return try await collect(
      client.decide(decoded(DecideQuestion.self, question), records: records, name: name),
      cancelled: cancelled)
  case "choose":
    return try await collect(
      client.choose(decoded(ChooseQuestion<String>.self, question), records: records, name: name),
      cancelled: cancelled)
  case "tag":
    return try await collect(
      client.tag(decoded(TagQuestion<String>.self, question), records: records, name: name),
      cancelled: cancelled)
  case "score":
    return try await collect(
      client.score(decoded(ScoreQuestion.self, question), records: records, name: name),
      cancelled: cancelled)
  case "filter":
    let filter = try decoded(FilterQuestion.self, question)
    let filtered = client.filter(filter, records: records, name: name)
    if cancelled { filtered.cancel() }
    var kept: [JSONValue] = []
    var failed: [AnyRow] = []
    do {
      for try await item in filtered {
        precondition(records.contains(item), "filter changed its original value")
        switch item {
        case .text(let text): kept.append(.string(text))
        case .json(let value): kept.append(value)
        case .missing: preconditionFailure("filter kept a missing input")
        }
      }
    } catch let error as Failed {
      failed = error.rows
      for row in failed {
        precondition(
          row.item as? Record == records[row.position - 1], "filter lost a failed original")
      }
    }
    struct Projection: Encodable {
      let kept: [JSONValue]
      let failed: [AnyRow]
    }
    return try encoded(Projection(kept: kept, failed: failed))
  case "rank":
    return try await collect(
      client.rank(decoded(RankQuestion.self, question), records: records, name: name),
      cancelled: cancelled)
  case "find":
    return try await collect(
      client.find(decoded(FindQuestion.self, question), records: records, name: name),
      cancelled: cancelled)
  case "recognize":
    return try await collect(
      client.recognize(decoded(RecognizeQuestion.self, question), records: records, name: name),
      cancelled: cancelled)
  case "relate":
    return try await collect(
      client.relate(decoded(RelateQuestion.self, question), records: records, name: name),
      cancelled: cancelled)
  case "annotate":
    let definitions = try JSONSerialization.data(withJSONObject: question["questions"]!)
    let entries = try AnnotationQuestions(json: String(decoding: definitions, as: UTF8.self))
    let field = try question["field"].map { try decoded(String.self, $0) }
    return try await collect(
      client.annotate(AnnotateQuestion(entries, field: field), records: records, name: name),
      cancelled: cancelled)
  default: preconditionFailure("shared fixture has an unknown function")
  }
}

@main struct Consumer {
  static func main() async throws {
    var args = CommandLine.arguments
    let single = args[1] == "single"
    if single { args.remove(at: 1) }
    if args[1] == "admission-surface" {
      print("swift")
      return
    }
    if args[1].hasPrefix("stress") {
      try await stress(args)
      return
    }
    if args[1] == "toolbox" {
      try await toolboxJSON(args[2])
      return
    }
    if args[1] == "admission-question" {
      let data = Data(args[2].utf8)
      let decoder = JSONDecoder()
      let definition = try decoder.decode(JSONValue.self, from: data)
      let client = try Client(backend: .stub, maxRequests: 0)
      do {
        switch definition {
        case .object(let fields):
          switch fields["function"] {
          case .string("score"):
            _ = try await client.score(
              decoder.decode(ScoreQuestion.self, from: data), record: "text")
          case .string("tag"):
            _ = try await client.tag(
              decoder.decode(TagQuestion<String>.self, from: data), record: "text")
          case .string("relate"):
            _ = try await client.relate(
              decoder.decode(RelateQuestion.self, from: data), record: "text")
          default: preconditionFailure("unknown shared admission question")
          }
        default: preconditionFailure("shared admission question is not an object")
        }
        preconditionFailure("malformed saved question was admitted")
      } catch let error as UsageError {
        print(error.code.rawValue)
        print(error.message)
      }
      precondition(client.usage.attempts == 0)
      return
    }
    if args[1] == "saved-name-probe" {
      let definitions =
        #"{"é":{"function":"decide","question":"First?"},"e\u0301":{"function":"decide","question":"Second?"}}"#
      do {
        _ = try AnnotationQuestions(json: definitions)
        preconditionFailure("colliding form names silently lost a question")
      } catch let error as UsageError {
        precondition(error.code == .usageArgument && error.message.contains("collide"))
      }
      let saved =
        #"{"entries":{"é":{"question":{"function":"decide","question":"First?"}},"e\u0301":{"question":{"function":"decide","question":"Second?"}}}}"#
      do {
        _ = try Toolbox(json: saved)
        preconditionFailure("colliding toolbox names silently lost a question")
      } catch let error as UsageError {
        precondition(error.code == .usageArgument && error.message.contains("collide"))
      }
      let directory = FileManager.default.temporaryDirectory.appendingPathComponent(
        UUID().uuidString)
      try FileManager.default.createDirectory(at: directory, withIntermediateDirectories: true)
      defer { try? FileManager.default.removeItem(at: directory) }
      let formFile = directory.appendingPathComponent("form.json")
      let toolboxFile = directory.appendingPathComponent("toolbox.json")
      try definitions.write(to: formFile, atomically: true, encoding: .utf8)
      try saved.write(to: toolboxFile, atomically: true, encoding: .utf8)
      do {
        _ = try AnnotationQuestions(contentsOf: formFile)
        preconditionFailure("saved form collision was not refused")
      } catch let error as UsageError { precondition(error.code == .usageArgument) }
      do {
        _ = try Toolbox(contentsOf: toolboxFile)
        preconditionFailure("saved toolbox collision was not refused")
      } catch let error as UsageError { precondition(error.code == .usageArgument) }
      _ = try AnnotationQuestions(json: #"{"é":{"function":"decide","question":"First?"}}"#)
      _ = try AnnotationQuestions(json: #"{"e\u0301":{"function":"decide","question":"Second?"}}"#)
      let single = #"{"entries":{"e\u0301":{"question":{"function":"decide","question":"Only?"}}}}"#
      let restored = try Toolbox(json: single)
      _ = try restored.decide("e\u{301}")
      do {
        _ = try restored.decide("é")
        preconditionFailure("a byte-distinct alias was accepted")
      } catch is UsageError {}
      let roundTrip = try JSONDecoder().decode(
        JSONValue.self, from: Data(restored.jsonString().utf8))
      if case .object(let root) = roundTrip, case .object(let entries) = root["entries"] {
        precondition(entries.count == 1 && entries.keys.first!.utf8.elementsEqual("e\u{301}".utf8))
      } else {
        preconditionFailure("saved single Unicode name changed shape")
      }
      let invalid = definitions.replacingOccurrences(of: "decide", with: "unknown")
      do {
        _ = try AnnotationQuestions(json: invalid)
        preconditionFailure("native invalid grammar was accepted")
      } catch let error as UsageError { precondition(error.code == .usageQuestion) }
      do {
        _ = try Toolbox(json: saved.replacingOccurrences(of: "decide", with: "unknown"))
        preconditionFailure("native invalid toolbox grammar was accepted")
      } catch let error as UsageError { precondition(error.code == .usageQuestion) }
      print("saved name collision refused")
      return
    }
    if args[1] == "held-input-cleanup" {
      try await heldInputCleanup(args[2], drop: args[3] == "drop")
      return
    }
    if args[1] == "toolbox-invalid" {
      do {
        _ = try Toolbox(json: args[2])
        preconditionFailure("malformed saved toolbox was admitted")
      } catch {
        guard let failure = diagnostic(error) else { throw error }
        print(failure.code.rawValue)
        print(failure.message)
      }
      return
    }
    if args[1] == "abi" {
      do {
        _ = try Client(.init(backend: .stub))
        print("accepted")
      } catch {
        guard let typed = error as? any ThinkThenError else { throw error }
        print(typed.code.rawValue)
        print(typed.message)
      }
      return
    }
    if args[1] == "load-failure" {
      do {
        _ = try Client(.init(backend: .stub))
        preconditionFailure("unloadable engine was accepted")
      } catch {
        guard let failure = diagnostic(error), failure.code == .localIO,
          failure.message.contains(args[2])
        else { throw error }
      }
      return
    }
    if args[1] == "audit-thread-starts" {
      try await auditStreamThreadStarts(Int(args[2])!)
      return
    }
    if args[1] == "output-input" {
      try await outputWhileInputWaits(args[2])
      return
    }
    #if os(macOS) || os(Linux)
      if args[1] == "stream-threads" {
        try await streamThreadLifetime()
        try await streamThreadCleanup()
        try await concurrentStreamThreads()

        return
      }
    #endif
    if args[1] == "surface" {
      try await surface()
      return
    }
    if args[1] == "cancel" {
      let client = try Client(
        .init(
          url: URL(string: args[2])!,
          retry: .init(
            maxRetries: 0, initialBackoff: .milliseconds(1), attemptTimeout: .seconds(120))))
      let task = Task { try await client.decide("Ready?", record: Record.text("evidence")) }
      await withCheckedContinuation { continuation in
        DispatchQueue.global().async {
          _ = FileHandle.standardInput.readData(ofLength: 1)
          continuation.resume()
        }
      }
      task.cancel()
      do {
        _ = try await task.value
        preconditionFailure("held call did not cancel")
      } catch { guard error is CancellationError else { throw error } }
      return
    }
    if args[1] == "backend" {
      let client = try Client(.init(url: URL(string: args[2])!))
      do {
        _ = try await client.decide("Ready?", record: Record.text("evidence"))
        preconditionFailure("backend failure returned a scalar result")
      } catch let failure as Failed {
        let code = String(decoding: try encoded(failure.code), as: UTF8.self)
        precondition(code.contains("backend."))
        precondition(failure.rows.count == 1 && failure.rows[0].isFailed)
        precondition(failure.rows[0].item as? Record == .text("evidence"))
      }
      let used = client.usage
      precondition(used.attempts > 0 && used.requestBodyBytes > 0)
      return
    }
    // An engine that will not load fails the process. It is not a case result.
    do {
      _ = try Client(.init(backend: .stub))
    } catch {
      let message = diagnostic(error)?.message ?? String(describing: error)
      FileHandle.standardError.write(Data((message + "\n").utf8))
      exit(1)
    }
    let value = try JSONSerialization.jsonObject(
      with: Data(contentsOf: URL(fileURLWithPath: args[1])))
    let cases = (value as? [[String: Any]]) ?? [value as! [String: Any]]
    for test in cases {
      do {
        FileHandle.standardOutput.write(
          try await run(test, url: args.count > 2 ? args[2] : nil, single: single))
      } catch {
        if let failure = diagnostic(error) {
          FileHandle.standardOutput.write(
            Data("{\"error\":".utf8) + (try encoded(failure.code)) + Data("}".utf8))
        } else if error is CancellationError {
          FileHandle.standardOutput.write(Data(#"{"error":"local.cancelled"}"#.utf8))
        } else {
          throw error
        }
      }
      FileHandle.standardOutput.write(Data("\n".utf8))
    }
  }
}
