import Foundation

extension Client {
  /// Evaluate a lazy stream of records with tag.
  public func tag<S: Sequence & Sendable>(_ question: TagQuestion, records: S, name: String? = nil)
    -> TagRows where S.Element == Record
  {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(TagQuestion.self)
    }
  }
  /// Evaluate a lazy stream of records with tag.
  public func tag<S: AsyncSequence & Sendable>(
    _ question: TagQuestion, records: S, name: String? = nil
  ) -> TagRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(TagQuestion.self)
    }
  }
  /// Evaluate one record with tag.
  public func tag(_ question: TagQuestion, record: Record, name: String? = nil)
    async throws -> TagRow
  {
    try await single(tag(question, records: [record], name: name))
  }
  /// Evaluate a lazy stream of records with tag.
  public func tag<S: Sequence & Sendable>(
    _ question: String, labels: [Choice], records: S, options: TagOptions = .init(),
    name: String? = nil
  ) -> TagRows where S.Element == Record {
    tag(TagQuestion(question, labels: labels, options: options), records: records, name: name)
  }
  /// Evaluate a lazy stream of records with tag.
  public func tag<S: AsyncSequence & Sendable>(
    _ question: String, labels: [Choice], records: S, options: TagOptions = .init(),
    name: String? = nil
  ) -> TagRows where S.Element == Record {
    tag(TagQuestion(question, labels: labels, options: options), records: records, name: name)
  }
  /// Evaluate one record with tag.
  public func tag(
    _ question: String, labels: [Choice], record: Record, options: TagOptions = .init(),
    name: String? = nil
  ) async throws -> TagRow {
    try await tag(
      TagQuestion(question, labels: labels, options: options), record: record, name: name)
  }
  /// Evaluate text records without wrapping each string in Record.text.
  public func tag<S: Sequence & Sendable>(
    _ question: TagQuestion, records: S, name: String? = nil
  ) -> TagRows where S.Element == String {
    tag(question, records: TextRecords(records), name: name)
  }
  /// Evaluate text records using a question and options.
  public func tag<S: Sequence & Sendable>(
    _ question: String, labels: [Choice], records: S, options: TagOptions = .init(),
    name: String? = nil
  ) -> TagRows where S.Element == String {
    tag(TagQuestion(question, labels: labels, options: options), records: records, name: name)
  }
  /// Evaluate text records without wrapping each string in Record.text.
  public func tag<S: AsyncSequence & Sendable>(
    _ question: TagQuestion, records: S, name: String? = nil
  ) -> TagRows where S.Element == String {
    tag(question, records: records.map { Record.text($0) }, name: name)
  }
  /// Evaluate text records using a question and options.
  public func tag<S: AsyncSequence & Sendable>(
    _ question: String, labels: [Choice], records: S, options: TagOptions = .init(),
    name: String? = nil
  ) -> TagRows where S.Element == String {
    tag(TagQuestion(question, labels: labels, options: options), records: records, name: name)
  }
  /// Evaluate a text value without wrapping it in Record.text.
  public func tag(_ question: TagQuestion, record: String, name: String? = nil)
    async throws -> TagRow
  {
    try await tag(question, record: .text(record), name: name)
  }
  /// Evaluate a text value without wrapping it in Record.text.
  public func tag(
    _ question: String, labels: [Choice], record: String, options: TagOptions = .init(),
    name: String? = nil
  ) async throws -> TagRow {
    try await tag(question, labels: labels, record: .text(record), options: options, name: name)
  }
}
