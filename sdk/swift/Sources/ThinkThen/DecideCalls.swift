import Foundation

extension Client {
  /// Evaluate a lazy stream of records with decide.
  public func decide<S: Sequence & Sendable>(
    _ question: DecideQuestion, records: S, name: String? = nil
  ) -> DecideRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(DecideQuestion.self)
    }
  }
  /// Evaluate a lazy stream of records with decide.
  public func decide<S: AsyncSequence & Sendable>(
    _ question: DecideQuestion, records: S, name: String? = nil
  ) -> DecideRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(DecideQuestion.self)
    }
  }
  /// Evaluate one record with decide.
  public func decide(_ question: DecideQuestion, record: Record, name: String? = nil)
    async throws -> DecideRow
  {
    try await single(decide(question, records: [record], name: name))
  }
  /// Evaluate a lazy stream of records with decide.
  public func decide<S: Sequence & Sendable>(
    _ question: String, records: S, yesMeans: String? = nil, noMeans: String? = nil,
    options: DecideOptions = .init(), name: String? = nil
  ) -> DecideRows where S.Element == Record {
    decide(
      DecideQuestion(question, yesMeans: yesMeans, noMeans: noMeans, options: options),
      records: records, name: name)
  }
  /// Evaluate a lazy stream of records with decide.
  public func decide<S: AsyncSequence & Sendable>(
    _ question: String, records: S, yesMeans: String? = nil, noMeans: String? = nil,
    options: DecideOptions = .init(), name: String? = nil
  ) -> DecideRows where S.Element == Record {
    decide(
      DecideQuestion(question, yesMeans: yesMeans, noMeans: noMeans, options: options),
      records: records, name: name)
  }
  /// Evaluate one record with decide.
  public func decide(
    _ question: String, record: Record, yesMeans: String? = nil, noMeans: String? = nil,
    options: DecideOptions = .init(), name: String? = nil
  ) async throws -> DecideRow {
    try await decide(
      DecideQuestion(question, yesMeans: yesMeans, noMeans: noMeans, options: options),
      record: record, name: name)
  }
  /// Evaluate text records without wrapping each string in Record.text.
  public func decide<S: Sequence & Sendable>(
    _ question: DecideQuestion, records: S, name: String? = nil
  ) -> DecideRows where S.Element == String {
    decide(question, records: TextRecords(records), name: name)
  }
  /// Evaluate text records without wrapping each string in Record.text.
  public func decide<S: AsyncSequence & Sendable>(
    _ question: DecideQuestion, records: S, name: String? = nil
  ) -> DecideRows where S.Element == String {
    decide(question, records: records.map { Record.text($0) }, name: name)
  }
  /// Evaluate text records using a question and options.
  public func decide<S: Sequence & Sendable>(
    _ question: String, records: S, yesMeans: String? = nil, noMeans: String? = nil,
    options: DecideOptions = .init(), name: String? = nil
  ) -> DecideRows where S.Element == String {
    decide(
      DecideQuestion(question, yesMeans: yesMeans, noMeans: noMeans, options: options),
      records: records, name: name)
  }
  /// Evaluate text records using a question and options.
  public func decide<S: AsyncSequence & Sendable>(
    _ question: String, records: S, yesMeans: String? = nil, noMeans: String? = nil,
    options: DecideOptions = .init(), name: String? = nil
  ) -> DecideRows where S.Element == String {
    decide(
      DecideQuestion(question, yesMeans: yesMeans, noMeans: noMeans, options: options),
      records: records, name: name)
  }
  /// Evaluate a text value without wrapping it in Record.text.
  public func decide(_ question: DecideQuestion, record: String, name: String? = nil)
    async throws -> DecideRow
  {
    try await decide(question, record: .text(record), name: name)
  }
  /// Evaluate a text value without wrapping it in Record.text.
  public func decide(
    _ question: String, record: String, yesMeans: String? = nil, noMeans: String? = nil,
    options: DecideOptions = .init(), name: String? = nil
  ) async throws -> DecideRow {
    try await decide(
      question, record: .text(record), yesMeans: yesMeans, noMeans: noMeans, options: options,
      name: name)
  }
}
