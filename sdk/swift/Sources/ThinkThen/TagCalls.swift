import Foundation
import Synchronization

extension Client {
  public func tag<C: Sendable, S: Sequence & Sendable>(
    _ question: TagQuestion<C>, records: S, name: String? = nil
  ) -> TagRows<C, S.Element> where S.Element: Encodable & Sendable {
    evaluate(question, input: source(records), item: S.Element.self, name: name)
  }
  public func tag<C: Sendable, S: AsyncSequence & Sendable>(
    _ question: TagQuestion<C>, records: S, name: String? = nil
  ) -> TagRows<C, S.Element> where S.Element: Encodable & Sendable {
    evaluate(question, input: source(records), item: S.Element.self, name: name)
  }
  public func tag<C: Sendable, T: Encodable & Sendable>(
    _ question: TagQuestion<C>, record: T, name: String? = nil
  ) async throws -> TagRow<C, T> {
    try requireSingleRecord(record, function: .tag)
    return try await single(tag(question, records: [record], name: name))
  }
  public func tag<S: Sequence & Sendable>(
    _ question: String, records: S, labels: [String], descriptions: [String: String] = [:],
    threshold: Double? = nil, field: String? = nil, name: String? = nil
  ) -> TagRows<String, S.Element> where S.Element: Encodable & Sendable {
    tag(
      TagQuestion(
        question, labels: labels, descriptions: descriptions, threshold: threshold, field: field),
      records: records, name: name)
  }
  public func tag<S: AsyncSequence & Sendable>(
    _ question: String, records: S, labels: [String], descriptions: [String: String] = [:],
    threshold: Double? = nil, field: String? = nil, name: String? = nil
  ) -> TagRows<String, S.Element> where S.Element: Encodable & Sendable {
    tag(
      TagQuestion(
        question, labels: labels, descriptions: descriptions, threshold: threshold, field: field),
      records: records, name: name)
  }
  public func tag<T: Encodable & Sendable>(
    _ question: String, record: T, labels: [String], descriptions: [String: String] = [:],
    threshold: Double? = nil, field: String? = nil, name: String? = nil
  ) async throws -> TagRow<String, T> {
    try await tag(
      TagQuestion(
        question, labels: labels, descriptions: descriptions, threshold: threshold, field: field),
      record: record, name: name)
  }
  public func tag<C: RawRepresentable & CaseIterable & Hashable & Sendable, S: Sequence & Sendable>(
    _ question: String, records: S, labels: C.Type, descriptions: [C: String] = [:],
    threshold: Double? = nil, field: String? = nil, name: String? = nil
  ) -> TagRows<C, S.Element> where S.Element: Encodable & Sendable, C.RawValue == String {
    tag(
      TagQuestion(
        question, labels: labels, descriptions: descriptions, threshold: threshold, field: field),
      records: records, name: name)
  }
  public func tag<
    C: RawRepresentable & CaseIterable & Hashable & Sendable, S: AsyncSequence & Sendable
  >(
    _ question: String, records: S, labels: C.Type, descriptions: [C: String] = [:],
    threshold: Double? = nil, field: String? = nil, name: String? = nil
  ) -> TagRows<C, S.Element> where S.Element: Encodable & Sendable, C.RawValue == String {
    tag(
      TagQuestion(
        question, labels: labels, descriptions: descriptions, threshold: threshold, field: field),
      records: records, name: name)
  }
  public func tag<
    C: RawRepresentable & CaseIterable & Hashable & Sendable, T: Encodable & Sendable
  >(
    _ question: String, record: T, labels: C.Type, descriptions: [C: String] = [:],
    threshold: Double? = nil, field: String? = nil, name: String? = nil
  ) async throws -> TagRow<C, T> where C.RawValue == String {
    try await tag(
      TagQuestion(
        question, labels: labels, descriptions: descriptions, threshold: threshold, field: field),
      record: record, name: name)
  }
}
