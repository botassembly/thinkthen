import Foundation
import Synchronization

extension Client {
  public func score<S: Sequence & Sendable>(
    _ question: ScoreQuestion, records: S, name: String? = nil
  ) -> ScoreRows<S.Element> where S.Element: Encodable & Sendable {
    evaluate(question, input: source(records), item: S.Element.self, name: name)
  }
  public func score<S: AsyncSequence & Sendable>(
    _ question: ScoreQuestion, records: S, name: String? = nil
  ) -> ScoreRows<S.Element> where S.Element: Encodable & Sendable {
    evaluate(question, input: source(records), item: S.Element.self, name: name)
  }
  public func score<T: Encodable & Sendable>(
    _ question: ScoreQuestion, record: T, name: String? = nil
  ) async throws -> ScoreRow<T> {
    try requireSingleRecord(record, function: .score)
    return try await single(score(question, records: [record], name: name))
  }
  public func score<S: Sequence & Sendable>(
    _ question: String, records: S, levels: [String], descriptions: [String: String] = [:],
    field: String? = nil, name: String? = nil
  ) -> ScoreRows<S.Element> where S.Element: Encodable & Sendable {
    score(
      ScoreQuestion(question, levels: levels, descriptions: descriptions, field: field),
      records: records, name: name)
  }
  public func score<S: AsyncSequence & Sendable>(
    _ question: String, records: S, levels: [String], descriptions: [String: String] = [:],
    field: String? = nil, name: String? = nil
  ) -> ScoreRows<S.Element> where S.Element: Encodable & Sendable {
    score(
      ScoreQuestion(question, levels: levels, descriptions: descriptions, field: field),
      records: records, name: name)
  }
  public func score<T: Encodable & Sendable>(
    _ question: String, record: T, levels: [String], descriptions: [String: String] = [:],
    field: String? = nil, name: String? = nil
  ) async throws -> ScoreRow<T> {
    try await score(
      ScoreQuestion(question, levels: levels, descriptions: descriptions, field: field),
      record: record, name: name)
  }
  public func score<E: RawRepresentable & CaseIterable & Hashable, S: Sequence & Sendable>(
    _ question: String, records: S, levels: E.Type, descriptions: [E: String] = [:],
    field: String? = nil, name: String? = nil
  ) -> ScoreRows<S.Element> where S.Element: Encodable & Sendable, E.RawValue == String {
    score(
      ScoreQuestion(question, levels: levels, descriptions: descriptions, field: field),
      records: records, name: name)
  }
  public func score<E: RawRepresentable & CaseIterable & Hashable, S: AsyncSequence & Sendable>(
    _ question: String, records: S, levels: E.Type, descriptions: [E: String] = [:],
    field: String? = nil, name: String? = nil
  ) -> ScoreRows<S.Element> where S.Element: Encodable & Sendable, E.RawValue == String {
    score(
      ScoreQuestion(question, levels: levels, descriptions: descriptions, field: field),
      records: records, name: name)
  }
  public func score<E: RawRepresentable & CaseIterable & Hashable, T: Encodable & Sendable>(
    _ question: String, record: T, levels: E.Type, descriptions: [E: String] = [:],
    field: String? = nil, name: String? = nil
  ) async throws -> ScoreRow<T> where E.RawValue == String {
    try await score(
      ScoreQuestion(question, levels: levels, descriptions: descriptions, field: field),
      record: record, name: name)
  }
}
