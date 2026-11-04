import Foundation

extension Client {
  /// Evaluate a lazy stream of records with score.
  public func score<S: Sequence & Sendable>(
    _ question: ScoreQuestion, records: S, name: String? = nil
  ) -> ScoreRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(ScoreQuestion.self)
    }
  }
  /// Evaluate a lazy stream of records with score.
  public func score<S: AsyncSequence & Sendable>(
    _ question: ScoreQuestion, records: S, name: String? = nil
  ) -> ScoreRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(ScoreQuestion.self)
    }
  }
  /// Evaluate one record with score.
  public func score(_ question: ScoreQuestion, record: Record, name: String? = nil)
    async throws -> ScoreRow
  {
    try await single(score(question, records: [record], name: name))
  }
  /// Evaluate a lazy stream of records with score.
  public func score<S: Sequence & Sendable>(
    _ question: String, levels: [Choice], records: S, options: ScoreOptions = .init(),
    name: String? = nil
  ) -> ScoreRows where S.Element == Record {
    score(ScoreQuestion(question, levels: levels, options: options), records: records, name: name)
  }
  /// Evaluate a lazy stream of records with score.
  public func score<S: AsyncSequence & Sendable>(
    _ question: String, levels: [Choice], records: S, options: ScoreOptions = .init(),
    name: String? = nil
  ) -> ScoreRows where S.Element == Record {
    score(ScoreQuestion(question, levels: levels, options: options), records: records, name: name)
  }
  /// Evaluate one record with score.
  public func score(
    _ question: String, levels: [Choice], record: Record, options: ScoreOptions = .init(),
    name: String? = nil
  ) async throws -> ScoreRow {
    try await score(
      ScoreQuestion(question, levels: levels, options: options), record: record, name: name)
  }
  /// Evaluate text records without wrapping each string in Record.text.
  public func score<S: Sequence & Sendable>(
    _ question: ScoreQuestion, records: S, name: String? = nil
  ) -> ScoreRows where S.Element == String {
    score(question, records: TextRecords(records), name: name)
  }
  /// Evaluate text records without wrapping each string in Record.text.
  public func score<S: AsyncSequence & Sendable>(
    _ question: ScoreQuestion, records: S, name: String? = nil
  ) -> ScoreRows where S.Element == String {
    score(question, records: records.map { Record.text($0) }, name: name)
  }
  /// Evaluate text records using a question and options.
  public func score<S: Sequence & Sendable>(
    _ question: String, levels: [Choice], records: S, options: ScoreOptions = .init(),
    name: String? = nil
  ) -> ScoreRows where S.Element == String {
    score(ScoreQuestion(question, levels: levels, options: options), records: records, name: name)
  }
  /// Evaluate text records using a question and options.
  public func score<S: AsyncSequence & Sendable>(
    _ question: String, levels: [Choice], records: S, options: ScoreOptions = .init(),
    name: String? = nil
  ) -> ScoreRows where S.Element == String {
    score(ScoreQuestion(question, levels: levels, options: options), records: records, name: name)
  }
  /// Evaluate a text value without wrapping it in Record.text.
  public func score(_ question: ScoreQuestion, record: String, name: String? = nil)
    async throws -> ScoreRow
  {
    try await score(question, record: .text(record), name: name)
  }
  /// Evaluate a text value without wrapping it in Record.text.
  public func score(
    _ question: String, levels: [Choice], record: String, options: ScoreOptions = .init(),
    name: String? = nil
  ) async throws -> ScoreRow {
    try await score(question, levels: levels, record: .text(record), options: options, name: name)
  }
}
