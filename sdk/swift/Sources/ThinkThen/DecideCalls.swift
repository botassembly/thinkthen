import Foundation
import Synchronization

extension Client {
  public func decide<S: Sequence & Sendable>(
    _ question: DecideQuestion, records: S, name: String? = nil
  ) -> DecideRows<S.Element> where S.Element: Encodable & Sendable {
    evaluate(question, input: source(records), item: S.Element.self, name: name)
  }
  public func decide<S: AsyncSequence & Sendable>(
    _ question: DecideQuestion, records: S, name: String? = nil
  ) -> DecideRows<S.Element> where S.Element: Encodable & Sendable {
    evaluate(question, input: source(records), item: S.Element.self, name: name)
  }
  public func decide<T: Encodable & Sendable>(
    _ question: DecideQuestion, record: T, name: String? = nil
  ) async throws -> DecideRow<T> {
    try requireSingleRecord(record, function: .decide)
    return try await single(decide(question, records: [record], name: name))
  }
  public func decide<S: Sequence & Sendable>(
    _ question: String, records: S, yesMeans: String? = nil, noMeans: String? = nil,
    threshold: Threshold? = nil, field: String? = nil, name: String? = nil
  ) -> DecideRows<S.Element> where S.Element: Encodable & Sendable {
    decide(
      DecideQuestion(
        question, yesMeans: yesMeans, noMeans: noMeans, threshold: threshold, field: field),
      records: records, name: name)
  }
  public func decide<S: AsyncSequence & Sendable>(
    _ question: String, records: S, yesMeans: String? = nil, noMeans: String? = nil,
    threshold: Threshold? = nil, field: String? = nil, name: String? = nil
  ) -> DecideRows<S.Element> where S.Element: Encodable & Sendable {
    decide(
      DecideQuestion(
        question, yesMeans: yesMeans, noMeans: noMeans, threshold: threshold, field: field),
      records: records, name: name)
  }
  public func decide<T: Encodable & Sendable>(
    _ question: String, record: T, yesMeans: String? = nil, noMeans: String? = nil,
    threshold: Threshold? = nil, field: String? = nil, name: String? = nil
  ) async throws -> DecideRow<T> {
    try await decide(
      DecideQuestion(
        question, yesMeans: yesMeans, noMeans: noMeans, threshold: threshold, field: field),
      record: record, name: name)
  }
}
