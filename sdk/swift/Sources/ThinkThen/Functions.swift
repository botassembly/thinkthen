import Foundation

extension Client {
  public func decide(_ question: Question, records: [Record], options: CallOptions = .init()) throws
    -> Rows<Bool>
  {
    let settings = try JSONEncoder().encode(options)
    return rows(
      function: "decide", question: question, source: source(records), options: settings,
      decode: decoded(Bool.self))
  }

  public func decide<S: AsyncSequence & Sendable>(
    _ question: Question, records: S, options: CallOptions = .init()
  ) throws -> Rows<Bool> where S.Element == Record {
    let settings = try JSONEncoder().encode(options)
    return rows(
      function: "decide", question: question, source: source(records), options: settings,
      decode: decoded(Bool.self))
  }

  public func decide(_ question: Question, record: Record, options: CallOptions = .init())
    async throws -> Row<Bool>
  {
    let settings = try JSONEncoder().encode(options)
    return try await single(
      rows(
        function: "decide", question: question, source: source([record]), options: settings,
        decode: decoded(Bool.self)))
  }

  public func choose(
    _ question: Question, records: [Record], options: [String]? = nil,
    callOptions: CallOptions = .init()
  ) throws -> Rows<String> {
    let settings = try labelledOptions(options, named: "options", settings: callOptions)
    return rows(
      function: "choose", question: question, source: source(records), options: settings,
      decode: decoded(String.self))
  }

  public func choose<S: AsyncSequence & Sendable>(
    _ question: Question, records: S, options: [String]? = nil, callOptions: CallOptions = .init()
  ) throws -> Rows<String> where S.Element == Record {
    let settings = try labelledOptions(options, named: "options", settings: callOptions)
    return rows(
      function: "choose", question: question, source: source(records), options: settings,
      decode: decoded(String.self))
  }

  public func choose(
    _ question: Question, record: Record, options: [String]? = nil,
    callOptions: CallOptions = .init()
  ) async throws -> Row<String> {
    let settings = try labelledOptions(options, named: "options", settings: callOptions)
    return try await single(
      rows(
        function: "choose", question: question, source: source([record]), options: settings,
        decode: decoded(String.self)))
  }

  public func tag(
    _ question: Question, records: [Record], labels: [String]? = nil,
    callOptions: CallOptions = .init()
  ) throws -> Rows<[String]> {
    let settings = try labelledOptions(labels, named: "labels", settings: callOptions)
    return rows(
      function: "tag", question: question, source: source(records), options: settings,
      decode: decoded([String].self))
  }

  public func tag<S: AsyncSequence & Sendable>(
    _ question: Question, records: S, labels: [String]? = nil, callOptions: CallOptions = .init()
  ) throws -> Rows<[String]> where S.Element == Record {
    let settings = try labelledOptions(labels, named: "labels", settings: callOptions)
    return rows(
      function: "tag", question: question, source: source(records), options: settings,
      decode: decoded([String].self))
  }

  public func tag(
    _ question: Question, record: Record, labels: [String]? = nil,
    callOptions: CallOptions = .init()
  ) async throws -> Row<[String]> {
    let settings = try labelledOptions(labels, named: "labels", settings: callOptions)
    return try await single(
      rows(
        function: "tag", question: question, source: source([record]), options: settings,
        decode: decoded([String].self)))
  }

  public func score(
    _ question: Question, records: [Record], levels: [String]? = nil,
    callOptions: EvidenceOptions = .init()
  ) throws -> Rows<Double> {
    let settings = try labelledOptions(levels, named: "levels", settings: callOptions)
    return rows(
      function: "score", question: question, source: source(records), options: settings,
      decode: decoded(Double.self))
  }

  public func score<S: AsyncSequence & Sendable>(
    _ question: Question, records: S, levels: [String]? = nil,
    callOptions: EvidenceOptions = .init()
  ) throws -> Rows<Double> where S.Element == Record {
    let settings = try labelledOptions(levels, named: "levels", settings: callOptions)
    return rows(
      function: "score", question: question, source: source(records), options: settings,
      decode: decoded(Double.self))
  }

  public func score(
    _ question: Question, record: Record, levels: [String]? = nil,
    callOptions: EvidenceOptions = .init()
  ) async throws -> Row<Double> {
    let settings = try labelledOptions(levels, named: "levels", settings: callOptions)
    return try await single(
      rows(
        function: "score", question: question, source: source([record]), options: settings,
        decode: decoded(Double.self)))
  }

  public func filter(_ question: Question, records: [Record], options: CallOptions = .init()) throws
    -> Rows<Bool>
  {
    let settings = try JSONEncoder().encode(options)
    return rows(
      function: "filter", question: question, source: source(records), options: settings,
      decode: decoded(Bool.self))
  }

  public func filter<S: AsyncSequence & Sendable>(
    _ question: Question, records: S, options: CallOptions = .init()
  ) throws -> Rows<Bool> where S.Element == Record {
    let settings = try JSONEncoder().encode(options)
    return rows(
      function: "filter", question: question, source: source(records), options: settings,
      decode: decoded(Bool.self))
  }

  public func rank(_ question: Question, records: [Record], options: RankOptions = .init()) throws
    -> Rows<Int>
  {
    let settings = try JSONEncoder().encode(options)
    return rows(
      function: "rank", question: question, source: source(records), options: settings,
      decode: decoded(Int.self))
  }

  public func rank<S: AsyncSequence & Sendable>(
    _ question: Question, records: S, options: RankOptions = .init()
  ) throws -> Rows<Int> where S.Element == Record {
    let settings = try JSONEncoder().encode(options)
    return rows(
      function: "rank", question: question, source: source(records), options: settings,
      decode: decoded(Int.self))
  }

  public func find(_ question: Question, records: [Record], options: CallOptions = .init()) throws
    -> Rows<TextSpan>
  {
    let settings = try JSONEncoder().encode(options)
    return rows(
      function: "find", question: question, source: source(records), options: settings,
      decode: decoded(TextSpan.self))
  }

  public func find<S: AsyncSequence & Sendable>(
    _ question: Question, records: S, options: CallOptions = .init()
  ) throws -> Rows<TextSpan> where S.Element == Record {
    let settings = try JSONEncoder().encode(options)
    return rows(
      function: "find", question: question, source: source(records), options: settings,
      decode: decoded(TextSpan.self))
  }

  public func find(_ question: Question, record: Record, options: CallOptions = .init())
    async throws -> Row<TextSpan>
  {
    let settings = try JSONEncoder().encode(options)
    return try await single(
      rows(
        function: "find", question: question, source: source([record]), options: settings,
        decode: decoded(TextSpan.self)))
  }

  public func annotate(_ question: JSON, records: [Record], options: EvidenceOptions = .init())
    throws
    -> Rows<JSON>
  {
    let settings = try JSONEncoder().encode(options)
    return rows(
      function: "annotate", question: Question.toolbox(question), source: source(records),
      options: settings, decode: { JSON(utf8: $0) })
  }

  public func annotate<S: AsyncSequence & Sendable>(
    _ question: JSON, records: S, options: EvidenceOptions = .init()
  ) throws -> Rows<JSON> where S.Element == Record {
    let settings = try JSONEncoder().encode(options)
    return rows(
      function: "annotate", question: Question.toolbox(question), source: source(records),
      options: settings, decode: { JSON(utf8: $0) })
  }

  public func annotate(_ question: JSON, record: Record, options: EvidenceOptions = .init())
    async throws -> Row<JSON>
  {
    let settings = try JSONEncoder().encode(options)
    return try await single(
      rows(
        function: "annotate", question: Question.toolbox(question), source: source([record]),
        options: settings, decode: { JSON(utf8: $0) }))
  }

  public func recognize(_ question: Question, records: [Record], options: CallOptions = .init())
    throws -> Rows<[RecognizedName]>
  {
    let settings = try JSONEncoder().encode(options)
    return rows(
      function: "recognize", question: question, source: source(records), options: settings,
      decode: decoded([RecognizedName].self))
  }

  public func recognize<S: AsyncSequence & Sendable>(
    _ question: Question, records: S, options: CallOptions = .init()
  ) throws -> Rows<[RecognizedName]> where S.Element == Record {
    let settings = try JSONEncoder().encode(options)
    return rows(
      function: "recognize", question: question, source: source(records), options: settings,
      decode: decoded([RecognizedName].self))
  }

  public func recognize(_ question: Question, record: Record, options: CallOptions = .init())
    async throws -> Row<[RecognizedName]>
  {
    let settings = try JSONEncoder().encode(options)
    return try await single(
      rows(
        function: "recognize", question: question, source: source([record]), options: settings,
        decode: decoded([RecognizedName].self)))
  }

  public func relate(_ question: Question, records: [Record], options: CallOptions = .init()) throws
    -> Rows<[RelationEdge]>
  {
    let settings = try JSONEncoder().encode(options)
    return rows(
      function: "relate", question: question, source: source(records), options: settings,
      decode: decoded([RelationEdge].self))
  }

  public func relate<S: AsyncSequence & Sendable>(
    _ question: Question, records: S, options: CallOptions = .init()
  ) throws -> Rows<[RelationEdge]> where S.Element == Record {
    let settings = try JSONEncoder().encode(options)
    return rows(
      function: "relate", question: question, source: source(records), options: settings,
      decode: decoded([RelationEdge].self))
  }

  public func relate(_ question: Question, record: Record, options: CallOptions = .init())
    async throws -> Row<[RelationEdge]>
  {
    let settings = try JSONEncoder().encode(options)
    return try await single(
      rows(
        function: "relate", question: question, source: source([record]), options: settings,
        decode: decoded([RelationEdge].self)))
  }

}

private func labelledOptions<T: Encodable>(_ values: [String]?, named field: String, settings: T)
  throws -> Data
{
  var object =
    try JSONSerialization.jsonObject(with: JSONEncoder().encode(settings)) as! [String: Any]
  if let values { object[field] = values }
  return try JSONSerialization.data(withJSONObject: object)
}
