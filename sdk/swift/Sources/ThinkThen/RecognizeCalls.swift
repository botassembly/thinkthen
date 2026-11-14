import Foundation
import Synchronization

extension Client {
  public func recognize<S: Sequence & Sendable>(
    _ question: RecognizeQuestion, records: S, name: String? = nil
  ) -> RecognizeRows<S.Element> where S.Element: Encodable & Sendable {
    evaluate(question, input: source(records), item: S.Element.self, name: name)
  }
  public func recognize<S: AsyncSequence & Sendable>(
    _ question: RecognizeQuestion, records: S, name: String? = nil
  ) -> RecognizeRows<S.Element> where S.Element: Encodable & Sendable {
    evaluate(question, input: source(records), item: S.Element.self, name: name)
  }
  public func recognize<T: Encodable & Sendable>(
    _ question: RecognizeQuestion, record: T, name: String? = nil
  ) async throws -> RecognizeRow<T> {
    try requireSingleRecord(record, function: .recognize)
    return try await single(recognize(question, records: [record], name: name))
  }
  public func recognize<S: Sequence & Sendable>(
    records: S, kinds: [String], threshold: Double? = nil, field: String? = nil, name: String? = nil
  ) -> RecognizeRows<S.Element> where S.Element: Encodable & Sendable {
    recognize(
      RecognizeQuestion(kinds: kinds, threshold: threshold, field: field), records: records,
      name: name)
  }
  public func recognize<S: AsyncSequence & Sendable>(
    records: S, kinds: [String], threshold: Double? = nil, field: String? = nil, name: String? = nil
  ) -> RecognizeRows<S.Element> where S.Element: Encodable & Sendable {
    recognize(
      RecognizeQuestion(kinds: kinds, threshold: threshold, field: field), records: records,
      name: name)
  }
  public func recognize<T: Encodable & Sendable>(
    record: T, kinds: [String], threshold: Double? = nil, field: String? = nil, name: String? = nil
  ) async throws -> RecognizeRow<T> {
    try await recognize(
      RecognizeQuestion(kinds: kinds, threshold: threshold, field: field), record: record,
      name: name)
  }
}
