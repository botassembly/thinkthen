import Foundation

extension Client {
  /// Evaluate a lazy stream of records with relate.
  public func relate<S: Sequence & Sendable>(
    _ question: RelateQuestion, records: S, name: String? = nil
  ) -> RelateRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(RelateQuestion.self)
    }
  }
  /// Evaluate a lazy stream of records with relate.
  public func relate<S: AsyncSequence & Sendable>(
    _ question: RelateQuestion, records: S, name: String? = nil
  ) -> RelateRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(RelateQuestion.self)
    }
  }
  /// Evaluate one record with relate.
  public func relate(_ question: RelateQuestion, record: Record, name: String? = nil)
    async throws -> RelateRow
  {
    try await single(relate(question, records: [record], name: name))
  }
  /// Evaluate a lazy stream of records with relate.
  public func relate<S: Sequence & Sendable>(
    rules: [RelationRule], records: S, options: RelateOptions = .init(), name: String? = nil
  ) -> RelateRows where S.Element == Record {
    relate(RelateQuestion(rules: rules, options: options), records: records, name: name)
  }
  /// Evaluate a lazy stream of records with relate.
  public func relate<S: AsyncSequence & Sendable>(
    rules: [RelationRule], records: S, options: RelateOptions = .init(), name: String? = nil
  ) -> RelateRows where S.Element == Record {
    relate(RelateQuestion(rules: rules, options: options), records: records, name: name)
  }
  /// Evaluate one record with relate.
  public func relate(
    rules: [RelationRule], record: Record, options: RelateOptions = .init(), name: String? = nil
  ) async throws -> RelateRow {
    try await relate(RelateQuestion(rules: rules, options: options), record: record, name: name)
  }
  /// Evaluate text records without wrapping each string in Record.text.
  public func relate<S: Sequence & Sendable>(
    _ question: RelateQuestion, records: S, name: String? = nil
  ) -> RelateRows where S.Element == String {
    relate(question, records: TextRecords(records), name: name)
  }
  /// Evaluate text records without wrapping each string in Record.text.
  public func relate<S: AsyncSequence & Sendable>(
    _ question: RelateQuestion, records: S, name: String? = nil
  ) -> RelateRows where S.Element == String {
    relate(question, records: records.map { Record.text($0) }, name: name)
  }
  /// Evaluate text input without converting each value to Record.text.
  public func relate<S: Sequence & Sendable>(
    rules: [RelationRule], records: S, options: RelateOptions = .init(), name: String? = nil
  ) -> RelateRows where S.Element == String {
    relate(RelateQuestion(rules: rules, options: options), records: records, name: name)
  }
  /// Evaluate text input without converting each value to Record.text.
  public func relate<S: AsyncSequence & Sendable>(
    rules: [RelationRule], records: S, options: RelateOptions = .init(), name: String? = nil
  ) -> RelateRows where S.Element == String {
    relate(RelateQuestion(rules: rules, options: options), records: records, name: name)
  }
  /// Evaluate a text value without wrapping it in Record.text.
  public func relate(_ question: RelateQuestion, record: String, name: String? = nil)
    async throws -> RelateRow
  {
    try await relate(question, record: .text(record), name: name)
  }
  /// Evaluate a text value without wrapping it in Record.text.
  public func relate(
    rules: [RelationRule], record: String, options: RelateOptions = .init(), name: String? = nil
  ) async throws -> RelateRow {
    try await relate(rules: rules, record: .text(record), options: options, name: name)
  }
}
