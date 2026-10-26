import Foundation

extension Client {
  public func choose<S: Sequence & Sendable>(
    _ question: ChooseQuestion, records: S, name: String? = nil
  ) -> ChooseRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(ChooseQuestion.self)
    }
  }
  public func choose<S: AsyncSequence & Sendable>(
    _ question: ChooseQuestion, records: S, name: String? = nil
  ) -> ChooseRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(ChooseQuestion.self)
    }
  }
  public func choose(_ question: ChooseQuestion, record: Record, name: String? = nil)
    async throws(ThinkThenError) -> ChooseRow
  {
    try await single(choose(question, records: [record], name: name))
  }
  public func choose<S: Sequence & Sendable>(
    _ question: String, choices: [String], records: S, options: ChooseOptions = .init(),
    name: String? = nil
  ) -> ChooseRows where S.Element == Record {
    choose(
      ChooseQuestion(question, choices: choices, options: options), records: records, name: name)
  }
  public func choose<S: AsyncSequence & Sendable>(
    _ question: String, choices: [String], records: S, options: ChooseOptions = .init(),
    name: String? = nil
  ) -> ChooseRows where S.Element == Record {
    choose(
      ChooseQuestion(question, choices: choices, options: options), records: records, name: name)
  }
  public func choose(
    _ question: String, choices: [String], record: Record, options: ChooseOptions = .init(),
    name: String? = nil
  ) async throws(ThinkThenError) -> ChooseRow {
    try await choose(
      ChooseQuestion(question, choices: choices, options: options), record: record, name: name)
  }
}
