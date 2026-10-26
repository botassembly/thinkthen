import Foundation

extension Client {
  public func recognize<S: Sequence & Sendable>(
    _ question: RecognizeQuestion, records: S, name: String? = nil
  ) -> RecognizeRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(RecognizeQuestion.self)
    }
  }
  public func recognize<S: AsyncSequence & Sendable>(
    _ question: RecognizeQuestion, records: S, name: String? = nil
  ) -> RecognizeRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(RecognizeQuestion.self)
    }
  }
  public func recognize(_ question: RecognizeQuestion, record: Record, name: String? = nil)
    async throws(ThinkThenError) -> RecognizeRow
  {
    try await single(recognize(question, records: [record], name: name))
  }
  public func recognize<S: Sequence & Sendable>(
    kinds: [String], records: S, options: RecognizeOptions = .init(), name: String? = nil
  ) -> RecognizeRows where S.Element == Record {
    recognize(RecognizeQuestion(kinds: kinds, options: options), records: records, name: name)
  }
  public func recognize<S: AsyncSequence & Sendable>(
    kinds: [String], records: S, options: RecognizeOptions = .init(), name: String? = nil
  ) -> RecognizeRows where S.Element == Record {
    recognize(RecognizeQuestion(kinds: kinds, options: options), records: records, name: name)
  }
  public func recognize(
    kinds: [String], record: Record, options: RecognizeOptions = .init(), name: String? = nil
  ) async throws(ThinkThenError) -> RecognizeRow {
    try await recognize(
      RecognizeQuestion(kinds: kinds, options: options), record: record, name: name)
  }
}
