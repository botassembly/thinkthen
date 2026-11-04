import Foundation

extension Client {
  /// Evaluate a lazy stream of records with choose.
  public func choose<S: Sequence & Sendable>(
    _ question: ChooseQuestion, records: S, name: String? = nil
  ) -> ChooseRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(ChooseQuestion.self)
    }
  }
  /// Evaluate a lazy stream of records with choose.
  public func choose<S: AsyncSequence & Sendable>(
    _ question: ChooseQuestion, records: S, name: String? = nil
  ) -> ChooseRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(ChooseQuestion.self)
    }
  }
  /// Evaluate one record with choose.
  public func choose(_ question: ChooseQuestion, record: Record, name: String? = nil)
    async throws -> ChooseRow
  {
    try await single(choose(question, records: [record], name: name))
  }
  /// Evaluate a lazy stream of records with choose.
  public func choose<S: Sequence & Sendable>(
    _ question: String, choices: [Choice], records: S, options: ChooseOptions = .init(),
    name: String? = nil
  ) -> ChooseRows where S.Element == Record {
    choose(
      ChooseQuestion(question, choices: choices, options: options), records: records, name: name)
  }
  /// Evaluate a lazy stream of records with choose.
  public func choose<S: AsyncSequence & Sendable>(
    _ question: String, choices: [Choice], records: S, options: ChooseOptions = .init(),
    name: String? = nil
  ) -> ChooseRows where S.Element == Record {
    choose(
      ChooseQuestion(question, choices: choices, options: options), records: records, name: name)
  }
  /// Evaluate one record with choose.
  public func choose(
    _ question: String, choices: [Choice], record: Record, options: ChooseOptions = .init(),
    name: String? = nil
  ) async throws -> ChooseRow {
    try await choose(
      ChooseQuestion(question, choices: choices, options: options), record: record, name: name)
  }
  /// Evaluate text records without wrapping each string in Record.text.
  public func choose<S: Sequence & Sendable>(
    _ question: ChooseQuestion, records: S, name: String? = nil
  ) -> ChooseRows where S.Element == String {
    choose(question, records: TextRecords(records), name: name)
  }
  /// Evaluate text records without wrapping each string in Record.text.
  public func choose<S: AsyncSequence & Sendable>(
    _ question: ChooseQuestion, records: S, name: String? = nil
  ) -> ChooseRows where S.Element == String {
    choose(question, records: records.map { Record.text($0) }, name: name)
  }
  /// Evaluate text records using a question and options.
  public func choose<S: Sequence & Sendable>(
    _ question: String, choices: [Choice], records: S, options: ChooseOptions = .init(),
    name: String? = nil
  ) -> ChooseRows where S.Element == String {
    choose(
      ChooseQuestion(question, choices: choices, options: options), records: records, name: name)
  }
  /// Evaluate text records using a question and options.
  public func choose<S: AsyncSequence & Sendable>(
    _ question: String, choices: [Choice], records: S, options: ChooseOptions = .init(),
    name: String? = nil
  ) -> ChooseRows where S.Element == String {
    choose(
      ChooseQuestion(question, choices: choices, options: options), records: records, name: name)
  }
  /// Evaluate a text value without wrapping it in Record.text.
  public func choose(_ question: ChooseQuestion, record: String, name: String? = nil)
    async throws -> ChooseRow
  {
    try await choose(question, record: .text(record), name: name)
  }
  /// Evaluate a text value without wrapping it in Record.text.
  public func choose(
    _ question: String, choices: [Choice], record: String, options: ChooseOptions = .init(),
    name: String? = nil
  ) async throws -> ChooseRow {
    try await choose(
      question, choices: choices, record: .text(record), options: options, name: name)
  }
}
