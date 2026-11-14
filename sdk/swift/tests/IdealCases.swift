import Foundation
import Synchronization
import ThinkThen

private struct IdealFailure: Error { let message: String }
private func expect(_ condition: @autoclosure () throws -> Bool, _ message: String) throws {
  guard try condition() else { throw IdealFailure(message: message) }
}
private final class OriginalMessage: Encodable, Sendable {
  let id: String
  private let content: Mutex<String>
  init(_ id: String, _ text: String) {
    self.id = id
    content = Mutex(text)
  }
  func change(_ text: String) { content.withLock { $0 = text } }
  func encode(to encoder: any Encoder) throws {
    var values = encoder.container(keyedBy: Keys.self)
    try values.encode(id, forKey: .id)
    try values.encode(content.withLock { $0 }, forKey: .text)
  }
  private enum Keys: String, CodingKey { case id, text }
}
private enum IdealTopic: String, CaseIterable { case billing, shipping, other }
private enum IdealUrgency: String, CaseIterable { case routine, urgent }

func checkIdealHostContracts(_ scenario: String) async throws {
  let client = try Client(retry: RetryPolicy(maxRetries: 0))
  let original = OriginalMessage("kept", "Please refund my order.")
  let question = DecideQuestion("Ready?", threshold: .band(low: 0.2, high: 0.8), field: "/text")
  let first: DecideRow<OriginalMessage>
  let equal: DecideRow<OriginalMessage>
  if scenario == "failed" || scenario == "refund-failed" {
    if scenario == "failed" {
      try await checkScalarFailures(client)
    } else {
      do {
        _ = try await client.decide(question, record: original)
        throw IdealFailure(message: "targeted primitive failure returned a scalar result")
      } catch let failure as Failed {
        try expect(
          failure.code == .backendReply && failure.httpStatus == nil,
          "targeted malformed answer fails at the call")
        try expect(
          failure.rows.count == 1 && failure.rows[0].item as? OriginalMessage === original,
          "targeted failure keeps the original")
      }
    }
    first = try await client.decide(question, records: [original]).collect()[0]
    equal = try await client.decide(
      question, records: [OriginalMessage("kept", "Please refund my order.")]
    ).collect()[0]
  } else {
    first = try await client.decide(question, record: original)
    equal = try await client.decide(
      question, record: OriginalMessage("kept", "Please refund my order."))
    try expect(!first.isFailed && first.error == nil, "a returned scalar has no own failure")
  }
  try expect(first.item === original, "single result keeps the caller's object")
  try expect(
    first == equal && first.hashValue == equal.hashValue, "own fields determine value equality")
  let set = Set([first])
  original.change("changed after the call")
  try expect(
    set.contains(first) && first == equal, "changing caller data cannot change equality or hashing")
  try expect(
    String(describing: first) == "\(first)" && String(reflecting: first) == first.description,
    "all safe printing paths agree")
  let saved = try JSONDecoder().decode(JSONValue.self, from: JSONEncoder().encode(first))
  if case .object(let fields) = saved {
    try expect(fields["item"] == nil, "saved library rows omit item")
    try expect(
      Set(fields.keys).isSubset(of: [
        "function", "status", "answer", "probability", "error", "position", "id", "source",
        "detail",
      ]), "only canonical fields are saved")
  } else {
    throw IdealFailure(message: "saved row is not an object")
  }
  if first.isAnswered {
    _ = try first.answer
    _ = try first.throwIfFailed()
    try expect(first.error == nil, "answered state has no error")
  } else {
    do {
      _ = try first.answer
      throw IdealFailure(message: "unanswered read was accepted")
    } catch let error as any NoAnswer {
      if first.isUnsure {
        try expect(
          error is Unsure && !(error is any ThinkThenError), "unsure belongs only to NoAnswer")
        let unsure = error as! Unsure
        try expect(
          unsure.row.item as? OriginalMessage === original, "unsure retains its original row")
        try expect(
          unsure.row.position == first.position && unsure.probability == first.probability,
          "unsure retains state metadata")
        _ = try first.throwIfFailed()
      } else {
        try expect(
          error is Failed && error is any ThinkThenError,
          "record failure belongs to both catches")
        let failure = error as! Failed
        try expect(
          failure.rows.count == 1 && failure.rows[0].item as? OriginalMessage === original,
          "failed answer retains one caller record")
      }
    }
  }
  let beforeBlank = client.usage.attempts
  for call in scalarCalls(client, text: "   ", includeForm: true) {
    do {
      try await call()
      throw IdealFailure(message: "blank scalar input returned a result")
    } catch let error as Failed {
      try expect(error.code == .recordEmpty && error.httpStatus == nil, "blank call diagnostic")
      try expect(
        error.rows.count == 1 && error.rows[0].position == 1,
        "blank call keeps its failed parent")
      try expect(error.rows[0].item as? String == "   ", "blank failure keeps the input")
      let diagnostic: any Error = error
      try expect(
        diagnostic is any NoAnswer && diagnostic is any ThinkThenError,
        "promoted failure belongs to both error families")
    }
  }
  try expect(client.usage.attempts == beforeBlank, "blank scalar calls make no requests")

  let questions = AnnotationQuestions([
    "refund": DecideQuestion("Refund?", threshold: .band(low: 0.2, high: 0.8)),
    "topic": ChooseQuestion("Topic?", choices: IdealTopic.self),
    "urgency": ScoreQuestion("Urgency?", levels: IdealUrgency.self),
  ])
  let form = try await client.annotate(questions, record: original)
  let another = try await client.annotate(
    questions, record: OriginalMessage("kept", "changed after the call"))
  try expect(form == another, "form equality excludes originals including its nested entries")
  let forms = Set([form])
  original.change("changed again")
  try expect(forms.contains(form), "a retained form stays a stable set key")
  if scenario.hasSuffix("-failed") || scenario == "failed" {
    do {
      _ = try form.throwIfFailed()
      throw IdealFailure(message: "failed child was not raised")
    } catch let error as Failed {
      try expect(error.rows.count == 1, "a failed form counts one affected record")
      try expect(
        error.rows[0].item as? OriginalMessage === original, "failed form keeps parent original")
      try expect(error.rows[0].position == form.position, "failed form keeps parent position")
    }
  } else {
    try expect(try form.throwIfFailed() == form, "unsure entries do not fail their form")
  }
  if form.isAnswered {
    let entries = try form.answer
    let refund = try entries.decide("refund")
    let topic = try entries.choose("topic", choices: IdealTopic.self)
    let urgency = try entries.score("urgency")
    try expect(
      refund.item === original && topic.item === original && urgency.item === original,
      "ordinary entries retain the caller's original")
    do {
      _ = try entries.decide("missing")
      throw IdealFailure(message: "missing form entry accepted")
    } catch is UsageError {}
    do {
      _ = try entries.find("refund")
      throw IdealFailure(message: "wrong form function accepted")
    } catch is UsageError {}
    let refundState: Status =
      ["failed", "refund-failed"].contains(scenario)
      ? .failed : ["refund-unsure", "unsure"].contains(scenario) ? .unsure : .answered
    let topicState: Status =
      ["failed", "topic-failed"].contains(scenario)
      ? .failed : ["topic-unsure", "unsure"].contains(scenario) ? .unsure : .answered
    let urgencyState: Status =
      ["failed", "urgency-failed"].contains(scenario) ? .failed : .answered
    try checkEntryState(refund, expected: refundState)
    try checkEntryState(topic, expected: topicState)
    try checkEntryState(urgency, expected: urgencyState)

  }
  let records = [
    OriginalMessage("one", "one"), OriginalMessage("two", "two"), OriginalMessage("three", "three"),
  ]
  let ordinary = try await client.decide("Ready?", records: records).collect()
  for row in ordinary {
    try expect(row.item === records[row.position - 1], "many rows retain exact input identity")
  }
  let ranked = try await client.rank(
    "Urgency?", records: records, levels: IdealUrgency.self, top: 1
  ).collect()
  for row in ranked {
    try expect(
      row.item === records[row.position - 1], "rank retains original identity even with top")
  }
  do {
    for try await kept in client.filter("Ready?", records: records) {
      try expect(records.contains { $0 === kept }, "filter yields original objects")
    }
  } catch let error as Failed {
    try expect(error.rows.count == records.count, "filter aggregates every failed row")
    for failed in error.rows {
      try expect(
        failed.item as? OriginalMessage === records[failed.position - 1],
        "filter failure keeps each original")
    }
  }
  if scenario == "relations" {
    try await checkRetainedSpatialFields()
    let message = "Morgan works for Example Group.\nPlease refund my order."
    let decided = try await client.decide("Ready?", record: "text")
    try expect(decided.description == "true", "decide prints command words")
    let tagged = try await client.tag("Topics?", record: "text", labels: IdealTopic.self)
    try expect(
      tagged.description == "billing\tshipping\tother", "tag preserves structural separators")
    let scored = try await client.score("Urgency?", record: "text", levels: IdealUrgency.self)
    try expect(scored.description == "1", "score prints command number")
    let found = try await client.find("Which line?", record: message)
    try expect(found.description == "Please refund my order.", "find prints only matching text")
    let names = try await client.recognize(record: message, kinds: ["person", "organization"])
    try expect(
      names.description == "person: Morgan (0..6)\torganization: Example Group (17..30)",
      "names print character offsets")
    let relations = try await client.relate(
      record: message,
      rules: [RelationRule(source: "person", target: "organization", relation: "works_for")])
    try expect(
      relations.description == "Morgan --works_for--> Example Group",
      "relations print command links")
    try expect(
      form.description == "refund=true\ttopic=billing\turgency=1",
      "forms print entries in engine order")
  }
  if scenario == "yes" {
    try await checkEncodingFailureRows()
    let arrays = [["body"], ["other"]]
    let arrayAttempts = client.usage.attempts
    let nested = try await client.decide("Ready?", records: arrays).collect()
    try expect(
      nested.count == arrays.count && nested.allSatisfy(\.isAnswered),
      "nonempty array-shaped stream inputs are judged as engine records")
    try expect(
      nested.map(\.item) == arrays && client.usage.attempts > arrayAttempts,
      "stream arrays retain their original values and reach the fixture")
    let failedFields = ["id": "ordered", "empty": "", "valid": "text"]
    let failureOrder = try await client.annotate(
      AnnotationQuestions([
        "2": DecideQuestion("Ready?", field: "/missing"),
        "10": DecideQuestion("Ready?", field: "/empty"),
        "ok": DecideQuestion("Ready?", field: "/valid"),
      ]), record: failedFields)
    do {
      _ = try failureOrder.throwIfFailed()
      throw IdealFailure(message: "partial form failures were not raised")
    } catch let error as Failed {
      try expect(
        error.code == .recordEmpty && error.rows.count == 1
          && error.rows[0].isFailed && error.rows[0].function == .decide
          && error.rows[0].error?.code == .recordEmpty,
        "first failed child follows canonical UTF8 order and counts its form once")
      try expect(
        error.rows[0].item as? [String: String] == failedFields
          && error.rows[0].position == failureOrder.position
          && error.rows[0].id == failureOrder.id && error.rows[0].source == failureOrder.source,
        "failure keeps parent original and position")
    }
    let message = "é 🦀\nPlease refund my order."
    let found = try await client.find("Which line?", record: message)
    guard let span = try found.answer else {
      throw IdealFailure(message: "expected Unicode find span")
    }
    try expect(
      String(message[span.start..<span.end]) == span.text,
      "Swift indexes slice the original Unicode string")
    let savedSpan = try JSONDecoder().decode(JSONValue.self, from: JSONEncoder().encode(span))
    if case .object(let fields) = savedSpan {
      try expect(fields["start"] == .integer(4), "standalone span saves character offsets")
    }
    let whole = try await client.find("Which line?", record: ["text": "🦀 / é"])
    guard let wholeSpan = try whole.answer else {
      throw IdealFailure(message: "expected whole-record span")
    }
    try expect(
      wholeSpan.start == wholeSpan.text.startIndex && wholeSpan.end == wholeSpan.text.endIndex,
      "whole JSON uses the engine's selected normalization")
    let indexed = try await client.annotate(questions, records: [original, original]).collect()
    let firstEntries = try indexed[0].answer
    let secondEntries = try indexed[1].answer
    let firstChild = try firstEntries.decide("refund")
    let secondChild = try secondEntries.decide("refund")
    try expect(
      firstChild != secondChild && firstEntries != secondEntries,
      "nested equality includes exposed parent position metadata")
    let ordered = AnnotationQuestions([
      "2": DecideQuestion("Ready?"), "10": DecideQuestion("Ready?"),
      "\u{e000}": DecideQuestion("Ready?"), "\u{10000}": DecideQuestion("Ready?"),
    ])
    let order = try await client.annotate(ordered, record: "text")
    try expect(
      order.description == "10=true\t2=true\t\u{e000}=true\t\u{10000}=true",
      "form print order follows canonical UTF8 name order")
    let toolbox = Toolbox(["topic": ChooseQuestion("Topic?", choices: IdealTopic.self)])
    _ = try toolbox.choose("topic", choices: IdealTopic.self)
    do {
      _ = try toolbox.decide("topic")
      throw IdealFailure(message: "wrong toolbox function accepted")
    } catch is UsageError {}
    do {
      _ = try toolbox.choose("missing")
      throw IdealFailure(message: "missing toolbox name accepted")
    } catch is UsageError {}
  }
  print("Swift ideal host contracts passed: \(scenario)")
}

private final class SpatialRecord: Encodable, Sendable {
  private let fields: Mutex<[String: String]>
  init(_ fields: [String: String]) { self.fields = Mutex(fields) }
  func change() {
    fields.withLock {
      $0 = ["id": "changed", "body": "changed", "override": "changed", "names": "changed"]
    }
  }
  func encode(to encoder: any Encoder) throws { try fields.withLock { $0 }.encode(to: encoder) }
}

private func checkRetainedSpatialFields() async throws {
  let body = "é 🦀\nPlease refund my order."
  let alternate = "𐀀 e\u{301}\nPlease refund my order."
  let namesText = "Morgan 🦀 works for Example Group."
  let original = SpatialRecord([
    "id": "spatial", "body": body, "override": alternate, "names": namesText,
  ])
  let definitions = AnnotationQuestions([
    "inherited": FindQuestion("Which line?"),
    "override": FindQuestion("Which line?", field: "/override"),
    "names": RecognizeQuestion(kinds: ["person", "organization"], field: "/names"),
    "relations": RelateQuestion(
      rules: [RelationRule(source: "person", target: "organization", relation: "works_for")],
      field: "/names"),
  ])
  weak var finishedClient: Client?
  let retained = try await { () async throws in
    let client = try Client(retry: RetryPolicy(maxRetries: 0))
    finishedClient = client
    let form = try await client.annotate(definitions, record: original, field: "/body")
    let entries = try form.answer
    return (
      form, try entries.find("inherited"), try entries.find("override"),
      try entries.recognize("names"), try entries.relate("relations")
    )
  }()
  try expect(finishedClient == nil, "spatial results do not retain their client")
  let forms = Set([retained.0])
  let previousHash = retained.0.hashValue
  original.change()
  try expect(
    forms.contains(retained.0) && retained.0.hashValue == previousHash,
    "nested spatial hashing ignores later original changes")
  try expect(
    retained.1.item === original && retained.1.id == .string("spatial"),
    "detached entry retains original identity and parent metadata")
  guard let inherited = try retained.1.answer, let overridden = try retained.2.answer else {
    throw IdealFailure(message: "expected retained form spans")
  }
  try expect(
    String(body[inherited.start..<inherited.end]) == inherited.text,
    "entry inherits the parent selected field")
  try expect(
    String(alternate[overridden.start..<overridden.end]) == overridden.text,
    "entry override uses its own selected field")
  let encoded =
    try JSONSerialization.jsonObject(with: JSONEncoder().encode(retained.2)) as! [String: Any]
  let answer = encoded["answer"] as! [String: Any]
  try expect(
    answer["start"] as? Int == 5 && encoded["item"] == nil,
    "detached entry saves canonical offsets after client and parent lifetime")
  let savedSpan =
    try JSONSerialization.jsonObject(with: JSONEncoder().encode(overridden)) as! [String: Any]
  try expect(savedSpan["start"] as? Int == 5, "standalone retained span saves canonical positions")
  let names = try retained.3.answer
  try expect(names.count == 2, "structured field supplies both recognized names")
  for name in names {
    try expect(
      String(namesText[name.start..<name.end]) == name.text,
      "recognized indexes slice their effective field")
  }
  if let detail = retained.3.detail {
    for piece in detail.pieces {
      try expect(
        String(namesText[piece.start..<piece.end]) == piece.text,
        "piece detail preserves host indexes")
    }
    for name in detail.names {
      try expect(
        String(namesText[name.start..<name.end]) == name.text, "kind detail preserves host indexes")
    }
  } else {
    throw IdealFailure(message: "recognize detail was lost")
  }
  let relations = try retained.4.answer
  try expect(relations.count == 1, "structured field supplies its relation")
  for relation in relations {
    try expect(
      String(namesText[relation.source.start..<relation.source.end]) == relation.source.text,
      "relation source indexes slice selected field")
    try expect(
      String(namesText[relation.target.start..<relation.target.end]) == relation.target.text,
      "relation target indexes slice selected field")
    try expect(!relation.either, "absent optional wire flag is a nonoptional false in Swift")
  }
  guard let detail = retained.4.detail else {
    throw IdealFailure(message: "relation detail was lost")
  }
  for name in detail.names {
    try expect(
      String(namesText[name.start..<name.end]) == name.text,
      "relation detail preserves host indexes")
  }
  let parentJSON =
    try JSONSerialization.jsonObject(with: JSONEncoder().encode(retained.0)) as! [String: Any]
  let children = parentJSON["answer"] as! [String: [String: Any]]
  let inheritedJSON = children["inherited"]!["answer"] as! [String: Any]
  try expect(
    inheritedJSON["start"] as? Int == 4, "saved parent preserves inherited canonical coordinates")
}

private func checkEntryState<F: FunctionShape, T: Sendable>(_ row: Row<F, T>, expected: Status)
  throws
{
  try expect(row.status == expected, "entry preserves its designated scenario state")
  switch expected {
  case .answered: _ = try row.answer
  case .unsure:
    do {
      _ = try row.answer
      throw IdealFailure(message: "unsure entry read was accepted")
    } catch let error as Unsure {
      try expect(error.probability == row.probability, "unsure entry keeps its probability")
    }
    _ = try row.throwIfFailed()
  case .failed:
    do {
      _ = try row.answer
      throw IdealFailure(message: "failed entry read was accepted")
    } catch let error as Failed {
      try expect(
        error.rows.count == 1 && error.rows[0].position == row.position,
        "failed child keeps its metadata")
    }
    do {
      _ = try row.throwIfFailed()
      throw IdealFailure(message: "failed child was not raised")
    } catch is Failed {}
  }
}

private struct EncodingRecord: Encodable, Sendable {
  let id: Int
  let bad: Bool
  enum Failure: Error { case deliberate }
  func encode(to encoder: any Encoder) throws {
    if bad { throw Failure.deliberate }
    var values = encoder.container(keyedBy: Keys.self)
    try values.encode(id, forKey: .id)
    try values.encode("body", forKey: .body)
  }
  enum Keys: String, CodingKey { case id, body }
}
private func checkEncodingFailureRows() async throws {
  let forbidden = try Client(maxRequests: 0, retry: RetryPolicy(maxRetries: 0))
  let bad = EncodingRecord(id: 2, bad: true)
  do {
    _ = try await forbidden.decide("Ready?", record: bad, field: "/body")
    throw IdealFailure(message: "unrepresentable scalar returned a result")
  } catch let error as Failed {
    try expect(
      error.code == .recordInvalid && error.rows.count == 1,
      "unrepresentable scalar fails at the call")
    try expect(
      error.rows[0].item as? EncodingRecord != nil && forbidden.usage.attempts == 0,
      "encoding failure keeps its original without transport")
  }
  let client = try Client(retry: RetryPolicy(maxRetries: 0))
  let input = [EncodingRecord(id: 1, bad: false), bad, EncodingRecord(id: 3, bad: false)]
  let rows = try await client.decide("Ready?", records: input, field: "/body").collect()
  try expect(
    rows.count == 3 && rows.map(\.position) == [1, 2, 3],
    "one conversion failure does not stop or reorder the collection")
  try expect(
    rows[0].isAnswered && rows[1].isFailed && rows[2].isAnswered,
    "surrounding records are still judged")
  try expect(
    rows.map { $0.item.id } == [1, 2, 3] && rows[1].error?.code == .recordInvalid,
    "every outcome retains its original record")
  let stream = AsyncStream<EncodingRecord> { continuation in
    for value in input { continuation.yield(value) }
    continuation.finish()
  }
  let asynchronous = try await client.decide("Ready?", records: stream, field: "/body").collect()
  try expect(
    asynchronous == rows, "sync and async record sources preserve the same failed-row contract")
}

private func scalarCalls(_ client: Client, text: String, includeForm: Bool = false)
  -> [@Sendable () async throws -> Void]
{
  var calls: [@Sendable () async throws -> Void] = [
    { _ = try await client.decide("Ready?", record: text) },
    { _ = try await client.choose("Topic?", record: text, choices: IdealTopic.self) },
    { _ = try await client.tag("Topics?", record: text, labels: IdealTopic.self) },
    { _ = try await client.score("Urgency?", record: text, levels: IdealUrgency.self) },
    { _ = try await client.find("Which line?", record: text) },
    { _ = try await client.recognize(record: text, kinds: ["person"]) },
    {
      _ = try await client.relate(
        record: text,
        rules: [RelationRule(source: "person", target: "organization", relation: "works_for")])
    },
  ]
  if includeForm {
    calls.append {
      _ = try await client.annotate(
        AnnotationQuestions(["refund": DecideQuestion("Ready?")]), record: text)
    }
  }
  return calls
}

private func checkScalarFailures(_ client: Client) async throws {
  for call in scalarCalls(client, text: "body") {
    do {
      try await call()
      throw IdealFailure(message: "failed scalar returned a result")
    } catch let error as Failed {
      try expect(
        error.code == .backendStatus && error.httpStatus == 503,
        "scalar service failure retains its diagnostic")
      try expect(
        error.rows.count == 1 && error.rows[0].status == .failed,
        "scalar failure retains the failed parent")
      try expect(
        error.rows[0].item as? String == "body" && error.rows[0].position == 1,
        "scalar failure retains position and input")
      let diagnostic: any Error = error
      try expect(
        diagnostic is any NoAnswer && diagnostic is any ThinkThenError,
        "scalar failure belongs to both error families")
    }
  }
}
