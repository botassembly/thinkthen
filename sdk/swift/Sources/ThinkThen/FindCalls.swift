import Foundation

extension Client {
  /// Evaluate a lazy stream of records with find.
  public func find<S: Sequence & Sendable>(
    _ question: FindQuestion, records: S, name: String? = nil
  ) -> FindRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(FindQuestion.self)
    }
  }
  /// Evaluate a lazy stream of records with find.
  public func find<S: AsyncSequence & Sendable>(
    _ question: FindQuestion, records: S, name: String? = nil
  ) -> FindRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(FindQuestion.self)
    }
  }
  /// Evaluate one record with find.
  public func find(_ question: FindQuestion, record: Record, name: String? = nil)
    async throws -> FindRow
  {
    try await single(find(question, records: [record], name: name))
  }
  /// Evaluate a lazy stream of records with find.
  public func find<S: Sequence & Sendable>(
    _ question: String, records: S, options: FindOptions = .init(), name: String? = nil
  ) -> FindRows where S.Element == Record {
    find(FindQuestion(question, options: options), records: records, name: name)
  }
  /// Evaluate a lazy stream of records with find.
  public func find<S: AsyncSequence & Sendable>(
    _ question: String, records: S, options: FindOptions = .init(), name: String? = nil
  ) -> FindRows where S.Element == Record {
    find(FindQuestion(question, options: options), records: records, name: name)
  }
  /// Evaluate one record with find.
  public func find(
    _ question: String, record: Record, options: FindOptions = .init(), name: String? = nil
  ) async throws -> FindRow {
    try await find(FindQuestion(question, options: options), record: record, name: name)
  }
  /// Evaluate text records without wrapping each string in Record.text.
  public func find<S: Sequence & Sendable>(
    _ question: FindQuestion, records: S, name: String? = nil
  ) -> FindRows where S.Element == String {
    find(question, records: TextRecords(records), name: name)
  }
  /// Evaluate text records without wrapping each string in Record.text.
  public func find<S: AsyncSequence & Sendable>(
    _ question: FindQuestion, records: S, name: String? = nil
  ) -> FindRows where S.Element == String {
    find(question, records: records.map { Record.text($0) }, name: name)
  }
  /// Evaluate text records using a question and options.
  public func find<S: Sequence & Sendable>(
    _ question: String, records: S, options: FindOptions = .init(), name: String? = nil
  ) -> FindRows where S.Element == String {
    find(FindQuestion(question, options: options), records: records, name: name)
  }
  /// Evaluate text records using a question and options.
  public func find<S: AsyncSequence & Sendable>(
    _ question: String, records: S, options: FindOptions = .init(), name: String? = nil
  ) -> FindRows where S.Element == String {
    find(FindQuestion(question, options: options), records: records, name: name)
  }
  /// Evaluate a text value without wrapping it in Record.text.
  public func find(_ question: FindQuestion, record: String, name: String? = nil)
    async throws -> FindRow
  {
    try await find(question, record: .text(record), name: name)
  }
  /// Evaluate a text value without wrapping it in Record.text.
  public func find(
    _ question: String, record: String, options: FindOptions = .init(), name: String? = nil
  ) async throws -> FindRow {
    try await find(question, record: .text(record), options: options, name: name)
  }
}
