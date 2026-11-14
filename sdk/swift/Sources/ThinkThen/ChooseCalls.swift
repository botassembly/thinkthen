import Foundation
import Synchronization

extension Client {
  public func choose<C: Sendable, S: Sequence & Sendable>(
    _ question: ChooseQuestion<C>, records: S, name: String? = nil
  ) -> ChooseRows<C, S.Element> where S.Element: Encodable & Sendable {
    evaluate(question, input: source(records), item: S.Element.self, name: name)
  }
  public func choose<C: Sendable, S: AsyncSequence & Sendable>(
    _ question: ChooseQuestion<C>, records: S, name: String? = nil
  ) -> ChooseRows<C, S.Element> where S.Element: Encodable & Sendable {
    evaluate(question, input: source(records), item: S.Element.self, name: name)
  }
  public func choose<C: Sendable, T: Encodable & Sendable>(
    _ question: ChooseQuestion<C>, record: T, name: String? = nil
  ) async throws -> ChooseRow<C, T> {
    try requireSingleRecord(record, function: .choose)
    return try await single(choose(question, records: [record], name: name))
  }
  public func choose<S: Sequence & Sendable>(
    _ question: String, records: S, choices: [String], descriptions: [String: String] = [:],
    threshold: Double? = nil, field: String? = nil, name: String? = nil
  ) -> ChooseRows<String, S.Element> where S.Element: Encodable & Sendable {
    choose(
      ChooseQuestion(
        question, choices: choices, descriptions: descriptions, threshold: threshold, field: field),
      records: records, name: name)
  }
  public func choose<S: AsyncSequence & Sendable>(
    _ question: String, records: S, choices: [String], descriptions: [String: String] = [:],
    threshold: Double? = nil, field: String? = nil, name: String? = nil
  ) -> ChooseRows<String, S.Element> where S.Element: Encodable & Sendable {
    choose(
      ChooseQuestion(
        question, choices: choices, descriptions: descriptions, threshold: threshold, field: field),
      records: records, name: name)
  }
  public func choose<T: Encodable & Sendable>(
    _ question: String, record: T, choices: [String], descriptions: [String: String] = [:],
    threshold: Double? = nil, field: String? = nil, name: String? = nil
  ) async throws -> ChooseRow<String, T> {
    try await choose(
      ChooseQuestion(
        question, choices: choices, descriptions: descriptions, threshold: threshold, field: field),
      record: record, name: name)
  }
  public func choose<
    C: RawRepresentable & CaseIterable & Hashable & Sendable, S: Sequence & Sendable
  >(
    _ question: String, records: S, choices: C.Type, descriptions: [C: String] = [:],
    threshold: Double? = nil, field: String? = nil, name: String? = nil
  ) -> ChooseRows<C, S.Element> where S.Element: Encodable & Sendable, C.RawValue == String {
    choose(
      ChooseQuestion(
        question, choices: choices, descriptions: descriptions, threshold: threshold, field: field),
      records: records, name: name)
  }
  public func choose<
    C: RawRepresentable & CaseIterable & Hashable & Sendable, S: AsyncSequence & Sendable
  >(
    _ question: String, records: S, choices: C.Type, descriptions: [C: String] = [:],
    threshold: Double? = nil, field: String? = nil, name: String? = nil
  ) -> ChooseRows<C, S.Element> where S.Element: Encodable & Sendable, C.RawValue == String {
    choose(
      ChooseQuestion(
        question, choices: choices, descriptions: descriptions, threshold: threshold, field: field),
      records: records, name: name)
  }
  public func choose<
    C: RawRepresentable & CaseIterable & Hashable & Sendable, T: Encodable & Sendable
  >(
    _ question: String, record: T, choices: C.Type, descriptions: [C: String] = [:],
    threshold: Double? = nil, field: String? = nil, name: String? = nil
  ) async throws -> ChooseRow<C, T> where C.RawValue == String {
    try await choose(
      ChooseQuestion(
        question, choices: choices, descriptions: descriptions, threshold: threshold, field: field),
      record: record, name: name)
  }
}
