import Foundation

extension Client {
  public func score<S: Sequence & Sendable>(
    _ question: ScoreQuestion, records: S, name: String? = nil
  ) -> ScoreRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(ScoreQuestion.self)
    }
  }
  public func score<S: AsyncSequence & Sendable>(
    _ question: ScoreQuestion, records: S, name: String? = nil
  ) -> ScoreRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(ScoreQuestion.self)
    }
  }
  public func score(_ question: ScoreQuestion, record: Record, name: String? = nil)
    async throws(ThinkThenError) -> ScoreRow
  {
    try await single(score(question, records: [record], name: name))
  }
  public func score<S: Sequence & Sendable>(
    _ question: String, levels: [String], records: S, options: ScoreOptions = .init(),
    name: String? = nil
  ) -> ScoreRows where S.Element == Record {
    score(ScoreQuestion(question, levels: levels, options: options), records: records, name: name)
  }
  public func score<S: AsyncSequence & Sendable>(
    _ question: String, levels: [String], records: S, options: ScoreOptions = .init(),
    name: String? = nil
  ) -> ScoreRows where S.Element == Record {
    score(ScoreQuestion(question, levels: levels, options: options), records: records, name: name)
  }
  public func score(
    _ question: String, levels: [String], record: Record, options: ScoreOptions = .init(),
    name: String? = nil
  ) async throws(ThinkThenError) -> ScoreRow {
    try await score(
      ScoreQuestion(question, levels: levels, options: options), record: record, name: name)
  }
}
