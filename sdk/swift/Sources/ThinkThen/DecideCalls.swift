import Foundation

extension Client {
  public func decide<S: Sequence & Sendable>(
    _ question: DecideQuestion, records: S, name: String? = nil
  ) -> DecideRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(DecideQuestion.self)
    }
  }
  public func decide<S: AsyncSequence & Sendable>(
    _ question: DecideQuestion, records: S, name: String? = nil
  ) -> DecideRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(DecideQuestion.self)
    }
  }
  public func decide(_ question: DecideQuestion, record: Record, name: String? = nil)
    async throws(ThinkThenError) -> DecideRow
  {
    try await single(decide(question, records: [record], name: name))
  }
  public func decide<S: Sequence & Sendable>(
    _ question: String, records: S, options: DecideOptions = .init(), name: String? = nil
  ) -> DecideRows where S.Element == Record {
    decide(DecideQuestion(question, options: options), records: records, name: name)
  }
  public func decide<S: AsyncSequence & Sendable>(
    _ question: String, records: S, options: DecideOptions = .init(), name: String? = nil
  ) -> DecideRows where S.Element == Record {
    decide(DecideQuestion(question, options: options), records: records, name: name)
  }
  public func decide(
    _ question: String, record: Record, options: DecideOptions = .init(), name: String? = nil
  ) async throws(ThinkThenError) -> DecideRow {
    try await decide(DecideQuestion(question, options: options), record: record, name: name)
  }
}
