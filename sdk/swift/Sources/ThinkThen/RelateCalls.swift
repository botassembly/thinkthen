import Foundation
import Synchronization

extension Client {
  public func relate<S: Sequence & Sendable>(
    _ question: RelateQuestion, records: S, name: String? = nil
  ) -> RelateRows<S.Element> where S.Element: Encodable & Sendable {
    evaluate(question, input: source(records), item: S.Element.self, name: name)
  }
  public func relate<S: AsyncSequence & Sendable>(
    _ question: RelateQuestion, records: S, name: String? = nil
  ) -> RelateRows<S.Element> where S.Element: Encodable & Sendable {
    evaluate(question, input: source(records), item: S.Element.self, name: name)
  }
  public func relate<T: Encodable & Sendable>(
    _ question: RelateQuestion, record: T, name: String? = nil
  ) async throws -> RelateRow<T> {
    try requireSingleRecord(record, function: .relate)
    return try await single(relate(question, records: [record], name: name))
  }
  public func relate<S: Sequence & Sendable>(
    records: S, rules: [RelationRule], threshold: Double? = nil, field: String? = nil,
    name: String? = nil
  ) -> RelateRows<S.Element> where S.Element: Encodable & Sendable {
    relate(
      RelateQuestion(rules: rules, threshold: threshold, field: field), records: records, name: name
    )
  }
  public func relate<S: AsyncSequence & Sendable>(
    records: S, rules: [RelationRule], threshold: Double? = nil, field: String? = nil,
    name: String? = nil
  ) -> RelateRows<S.Element> where S.Element: Encodable & Sendable {
    relate(
      RelateQuestion(rules: rules, threshold: threshold, field: field), records: records, name: name
    )
  }
  public func relate<T: Encodable & Sendable>(
    record: T, rules: [RelationRule], threshold: Double? = nil, field: String? = nil,
    name: String? = nil
  ) async throws -> RelateRow<T> {
    try await relate(
      RelateQuestion(rules: rules, threshold: threshold, field: field), record: record, name: name)
  }
}
