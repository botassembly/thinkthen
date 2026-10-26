import Foundation

extension Client {
  public func find<S: Sequence & Sendable>(
    _ question: FindQuestion, records: S, name: String? = nil
  ) -> FindRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(FindQuestion.self)
    }
  }
  public func find<S: AsyncSequence & Sendable>(
    _ question: FindQuestion, records: S, name: String? = nil
  ) -> FindRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(FindQuestion.self)
    }
  }
  public func find(_ question: FindQuestion, record: Record, name: String? = nil)
    async throws(ThinkThenError) -> FindRow
  {
    try await single(find(question, records: [record], name: name))
  }
  public func find<S: Sequence & Sendable>(
    _ question: String, records: S, options: FindOptions = .init(), name: String? = nil
  ) -> FindRows where S.Element == Record {
    find(FindQuestion(question, options: options), records: records, name: name)
  }
  public func find<S: AsyncSequence & Sendable>(
    _ question: String, records: S, options: FindOptions = .init(), name: String? = nil
  ) -> FindRows where S.Element == Record {
    find(FindQuestion(question, options: options), records: records, name: name)
  }
  public func find(
    _ question: String, record: Record, options: FindOptions = .init(), name: String? = nil
  ) async throws(ThinkThenError) -> FindRow {
    try await find(FindQuestion(question, options: options), record: record, name: name)
  }
}
