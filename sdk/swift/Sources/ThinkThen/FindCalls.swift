import Foundation
import Synchronization

extension Client {
  public func find<S: Sequence & Sendable>(
    _ question: FindQuestion, records: S, name: String? = nil
  ) -> FindRows<S.Element> where S.Element: Encodable & Sendable {
    evaluate(question, input: source(records), item: S.Element.self, name: name)
  }
  public func find<S: AsyncSequence & Sendable>(
    _ question: FindQuestion, records: S, name: String? = nil
  ) -> FindRows<S.Element> where S.Element: Encodable & Sendable {
    evaluate(question, input: source(records), item: S.Element.self, name: name)
  }
  public func find<T: Encodable & Sendable>(
    _ question: FindQuestion, record: T, name: String? = nil
  ) async throws -> FindRow<T> {
    try requireSingleRecord(record, function: .find)
    return try await single(find(question, records: [record], name: name))
  }
  public func find<S: Sequence & Sendable>(
    _ question: String, records: S, threshold: Double? = nil, field: String? = nil,
    name: String? = nil
  ) -> FindRows<S.Element> where S.Element: Encodable & Sendable {
    find(FindQuestion(question, threshold: threshold, field: field), records: records, name: name)
  }
  public func find<S: AsyncSequence & Sendable>(
    _ question: String, records: S, threshold: Double? = nil, field: String? = nil,
    name: String? = nil
  ) -> FindRows<S.Element> where S.Element: Encodable & Sendable {
    find(FindQuestion(question, threshold: threshold, field: field), records: records, name: name)
  }
  public func find<T: Encodable & Sendable>(
    _ question: String, record: T, threshold: Double? = nil, field: String? = nil,
    name: String? = nil
  ) async throws -> FindRow<T> {
    try await find(
      FindQuestion(question, threshold: threshold, field: field), record: record, name: name)
  }
}
