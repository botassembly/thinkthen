import Foundation

extension Client {
  /// Evaluate a lazy stream of records with recognize.
  public func recognize<S: Sequence & Sendable>(
    _ question: RecognizeQuestion, records: S, name: String? = nil
  ) -> RecognizeRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(RecognizeQuestion.self)
    }
  }
  /// Evaluate a lazy stream of records with recognize.
  public func recognize<S: AsyncSequence & Sendable>(
    _ question: RecognizeQuestion, records: S, name: String? = nil
  ) -> RecognizeRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(RecognizeQuestion.self)
    }
  }
  /// Evaluate one record with recognize.
  public func recognize(_ question: RecognizeQuestion, record: Record, name: String? = nil)
    async throws -> RecognizeRow
  {
    try await single(recognize(question, records: [record], name: name))
  }
  /// Evaluate a lazy stream of records with recognize.
  public func recognize<S: Sequence & Sendable>(
    kinds: [String], records: S, options: RecognizeOptions = .init(), name: String? = nil
  ) -> RecognizeRows where S.Element == Record {
    recognize(RecognizeQuestion(kinds: kinds, options: options), records: records, name: name)
  }
  /// Evaluate a lazy stream of records with recognize.
  public func recognize<S: AsyncSequence & Sendable>(
    kinds: [String], records: S, options: RecognizeOptions = .init(), name: String? = nil
  ) -> RecognizeRows where S.Element == Record {
    recognize(RecognizeQuestion(kinds: kinds, options: options), records: records, name: name)
  }
  /// Evaluate one record with recognize.
  public func recognize(
    kinds: [String], record: Record, options: RecognizeOptions = .init(), name: String? = nil
  ) async throws -> RecognizeRow {
    try await recognize(
      RecognizeQuestion(kinds: kinds, options: options), record: record, name: name)
  }
  /// Evaluate text records without wrapping each string in Record.text.
  public func recognize<S: Sequence & Sendable>(
    _ question: RecognizeQuestion, records: S, name: String? = nil
  ) -> RecognizeRows where S.Element == String {
    recognize(question, records: TextRecords(records), name: name)
  }
  /// Evaluate text records without wrapping each string in Record.text.
  public func recognize<S: AsyncSequence & Sendable>(
    _ question: RecognizeQuestion, records: S, name: String? = nil
  ) -> RecognizeRows where S.Element == String {
    recognize(question, records: records.map { Record.text($0) }, name: name)
  }
  /// Evaluate text input without converting each value to Record.text.
  public func recognize<S: Sequence & Sendable>(
    kinds: [String], records: S, options: RecognizeOptions = .init(), name: String? = nil
  ) -> RecognizeRows where S.Element == String {
    recognize(RecognizeQuestion(kinds: kinds, options: options), records: records, name: name)
  }
  /// Evaluate text input without converting each value to Record.text.
  public func recognize<S: AsyncSequence & Sendable>(
    kinds: [String], records: S, options: RecognizeOptions = .init(), name: String? = nil
  ) -> RecognizeRows where S.Element == String {
    recognize(RecognizeQuestion(kinds: kinds, options: options), records: records, name: name)
  }
  /// Evaluate a text value without wrapping it in Record.text.
  public func recognize(_ question: RecognizeQuestion, record: String, name: String? = nil)
    async throws -> RecognizeRow
  {
    try await recognize(question, record: .text(record), name: name)
  }
  /// Evaluate a text value without wrapping it in Record.text.
  public func recognize(
    kinds: [String], record: String, options: RecognizeOptions = .init(), name: String? = nil
  ) async throws -> RecognizeRow {
    try await recognize(kinds: kinds, record: .text(record), options: options, name: name)
  }
}
