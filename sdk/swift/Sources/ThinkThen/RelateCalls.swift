import Foundation

extension Client {
  public func relate<S: Sequence & Sendable>(
    _ question: RelateQuestion, records: S, name: String? = nil
  ) -> RelateRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(RelateQuestion.self)
    }
  }
  public func relate<S: AsyncSequence & Sendable>(
    _ question: RelateQuestion, records: S, name: String? = nil
  ) -> RelateRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(RelateQuestion.self)
    }
  }
  public func relate(_ question: RelateQuestion, record: Record, name: String? = nil)
    async throws(ThinkThenError) -> RelateRow
  {
    try await single(relate(question, records: [record], name: name))
  }
  public func relate<S: Sequence & Sendable>(
    rules: [RelationRule], records: S, options: RelateOptions = .init(), name: String? = nil
  ) -> RelateRows where S.Element == Record {
    relate(RelateQuestion(rules: rules, options: options), records: records, name: name)
  }
  public func relate<S: AsyncSequence & Sendable>(
    rules: [RelationRule], records: S, options: RelateOptions = .init(), name: String? = nil
  ) -> RelateRows where S.Element == Record {
    relate(RelateQuestion(rules: rules, options: options), records: records, name: name)
  }
  public func relate(
    rules: [RelationRule], record: Record, options: RelateOptions = .init(), name: String? = nil
  ) async throws(ThinkThenError) -> RelateRow {
    try await relate(RelateQuestion(rules: rules, options: options), record: record, name: name)
  }
}
