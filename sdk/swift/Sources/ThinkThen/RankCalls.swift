import Foundation
import Synchronization

extension Client {
  public func rank<S: Sequence & Sendable>(
    _ question: RankQuestion, records: S, name: String? = nil
  ) -> RankRows<S.Element> where S.Element: Encodable & Sendable {
    evaluate(question, input: source(records), item: S.Element.self, name: name)
  }
  public func rank<S: AsyncSequence & Sendable>(
    _ question: RankQuestion, records: S, name: String? = nil
  ) -> RankRows<S.Element> where S.Element: Encodable & Sendable {
    evaluate(question, input: source(records), item: S.Element.self, name: name)
  }
  public func rank<S: Sequence & Sendable>(
    _ question: String, records: S, levels: [String], descriptions: [String: String] = [:],
    top: Int? = nil, field: String? = nil, name: String? = nil
  ) -> RankRows<S.Element> where S.Element: Encodable & Sendable {
    rank(
      RankQuestion(question, levels: levels, descriptions: descriptions, top: top, field: field),
      records: records, name: name)
  }
  public func rank<S: AsyncSequence & Sendable>(
    _ question: String, records: S, levels: [String], descriptions: [String: String] = [:],
    top: Int? = nil, field: String? = nil, name: String? = nil
  ) -> RankRows<S.Element> where S.Element: Encodable & Sendable {
    rank(
      RankQuestion(question, levels: levels, descriptions: descriptions, top: top, field: field),
      records: records, name: name)
  }
  public func rank<E: RawRepresentable & CaseIterable & Hashable, S: Sequence & Sendable>(
    _ question: String, records: S, levels: E.Type, descriptions: [E: String] = [:],
    top: Int? = nil, field: String? = nil, name: String? = nil
  ) -> RankRows<S.Element> where S.Element: Encodable & Sendable, E.RawValue == String {
    rank(
      RankQuestion(question, levels: levels, descriptions: descriptions, top: top, field: field),
      records: records, name: name)
  }
  public func rank<E: RawRepresentable & CaseIterable & Hashable, S: AsyncSequence & Sendable>(
    _ question: String, records: S, levels: E.Type, descriptions: [E: String] = [:],
    top: Int? = nil, field: String? = nil, name: String? = nil
  ) -> RankRows<S.Element> where S.Element: Encodable & Sendable, E.RawValue == String {
    rank(
      RankQuestion(question, levels: levels, descriptions: descriptions, top: top, field: field),
      records: records, name: name)
  }
}
