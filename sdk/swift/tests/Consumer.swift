import Foundation
import ThinkThen

func encoded<T: Encodable>(_ value: T) throws -> Data { try JSONEncoder().encode(value) }
func decoded<T: Decodable>(_ type: T.Type, _ value: Any) throws -> T {
  try JSONDecoder().decode(
    type, from: JSONSerialization.data(withJSONObject: value, options: [.fragmentsAllowed]))
}
func collect<E: Sendable & Encodable>(_ rows: Rows<E>, cancelled: Bool = false) async throws -> Data
{
  if cancelled { rows.cancel() }
  var values: [E] = []
  do {
    for try await row in rows { values.append(row) }
  } catch {
    switch error {
    case .engine(let failure):
      return Data("{\"error\":".utf8) + (try encoded(failure.code)) + Data(",\"rows\":".utf8)
        + (try encoded(values)) + Data("}".utf8)
    case .cancelled:
      return Data(#"{"error":"local.cancelled","rows":"#.utf8) + (try encoded(values))
        + Data("}".utf8)
    default: throw error
    }
  }
  return try encoded(values)
}

// Feature: installed Swift argument mapping and typed rows for shared cases.
@MainActor func run(_ test: [String: Any], url: String?) async throws -> Data {
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
  let records = try (request["records"] as! [Any]).map { value -> Record in
    if value is NSNull { return .missing }
    if let text = value as? String { return .text(text) }
    return .json(try decoded(JSONValue.self, value))
  }
  switch question["function"] as! String {
  case "decide":
    return try await collect(
      client.decide(decoded(DecideQuestion.self, question), records: records, name: name),
      cancelled: cancelled)
  case "choose":
    return try await collect(
      client.choose(decoded(ChooseQuestion.self, question), records: records, name: name),
      cancelled: cancelled)
  case "tag":
    return try await collect(
      client.tag(decoded(TagQuestion.self, question), records: records, name: name),
      cancelled: cancelled)
  case "score":
    return try await collect(
      client.score(decoded(ScoreQuestion.self, question), records: records, name: name),
      cancelled: cancelled)
  case "filter":
    var rows: [FilterRow] = []
    let filter = try decoded(FilterQuestion.self, question)
    let filtered = client.filter(filter, records: records, name: name)
    if cancelled { filtered.cancel() }
    for try await result in filtered {
      rows.append(result.row)
    }
    return try encoded(rows)
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
    return try await collect(
      client.annotate(decoded(AnnotateQuestion.self, question), records: records, name: name),
      cancelled: cancelled)
  default: preconditionFailure("shared fixture has an unknown function")
  }
}

@main struct Consumer {
  static func main() async throws {
    let args = CommandLine.arguments
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
    if args[1] == "toolbox-invalid" {
      do {
        _ = try Toolbox.fromJSON(args[2])
        preconditionFailure("malformed saved toolbox was admitted")
      } catch ThinkThenError.engine(let failure) {
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
        switch error {
        case .engineVersion: print("version")
        case .engineUnavailable(let message): print(message)
        default: throw error
        }
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
      let client = try Client(.init(url: URL(string: args[2])!))
      let task = Task { try await client.decide("Ready?", record: .text("evidence")) }
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
      } catch { try requireError(error, equals: .cancelled) }
      return
    }
    if args[1] == "backend" {
      let client = try Client(.init(url: URL(string: args[2])!))
      let row = try await client.decide("Ready?", record: .text("evidence"))
      guard case .failed(let failure) = row.outcome else {
        preconditionFailure("backend record failure did not return a row")
      }
      let code = String(decoding: try encoded(failure.code), as: UTF8.self)
      precondition(code.contains("backend."))
      return
    }
    let value = try JSONSerialization.jsonObject(
      with: Data(contentsOf: URL(fileURLWithPath: args[1])))
    let cases = (value as? [[String: Any]]) ?? [value as! [String: Any]]
    for test in cases {
      do {
        FileHandle.standardOutput.write(try await run(test, url: args.count > 2 ? args[2] : nil))
      } catch {
        guard let typed = error as? ThinkThenError else { throw error }
        switch typed {
        case .engine(let failure):
          FileHandle.standardOutput.write(
            Data("{\"error\":".utf8) + (try encoded(failure.code)) + Data("}".utf8))
        case .cancelled:
          FileHandle.standardOutput.write(Data(#"{"error":"local.cancelled"}"#.utf8))
        default: throw error
        }
      }
      FileHandle.standardOutput.write(Data("\n".utf8))
    }
  }
}
