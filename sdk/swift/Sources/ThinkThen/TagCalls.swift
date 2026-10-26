import Foundation

extension Client {
  public func tag<S: Sequence & Sendable>(_ question: TagQuestion, records: S, name: String? = nil)
    -> TagRows where S.Element == Record
  {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(TagQuestion.self)
    }
  }
  public func tag<S: AsyncSequence & Sendable>(
    _ question: TagQuestion, records: S, name: String? = nil
  ) -> TagRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(TagQuestion.self)
    }
  }
  public func tag(_ question: TagQuestion, record: Record, name: String? = nil)
    async throws(ThinkThenError) -> TagRow
  {
    try await single(tag(question, records: [record], name: name))
  }
  public func tag<S: Sequence & Sendable>(
    _ question: String, labels: [String], records: S, options: TagOptions = .init(),
    name: String? = nil
  ) -> TagRows where S.Element == Record {
    tag(TagQuestion(question, labels: labels, options: options), records: records, name: name)
  }
  public func tag<S: AsyncSequence & Sendable>(
    _ question: String, labels: [String], records: S, options: TagOptions = .init(),
    name: String? = nil
  ) -> TagRows where S.Element == Record {
    tag(TagQuestion(question, labels: labels, options: options), records: records, name: name)
  }
  public func tag(
    _ question: String, labels: [String], record: Record, options: TagOptions = .init(),
    name: String? = nil
  ) async throws(ThinkThenError) -> TagRow {
    try await tag(
      TagQuestion(question, labels: labels, options: options), record: record, name: name)
  }
}
