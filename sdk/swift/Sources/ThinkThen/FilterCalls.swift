import Foundation

extension Client {
  public func filter<S: Sequence & Sendable>(
    _ question: FilterQuestion, records: S, name: String? = nil
  ) -> FilterRows where S.Element == Record {
    rows(question, source: source(records), name: name, retainItems: true) { wire, item in
      guard let item else {
        throw ThinkThenError.contract(message: "Filter row has no original item")
      }
      return FilterResult(item: item, row: try wire.decode(FilterQuestion.self))
    }
  }
  public func filter<S: AsyncSequence & Sendable>(
    _ question: FilterQuestion, records: S, name: String? = nil
  ) -> FilterRows where S.Element == Record {
    rows(question, source: source(records), name: name, retainItems: true) { wire, item in
      guard let item else {
        throw ThinkThenError.contract(message: "Filter row has no original item")
      }
      return FilterResult(item: item, row: try wire.decode(FilterQuestion.self))
    }
  }
  public func filter<S: Sequence & Sendable>(
    _ question: String, records: S, options: FilterOptions = .init(), name: String? = nil
  ) -> FilterRows where S.Element == Record {
    filter(FilterQuestion(question, options: options), records: records, name: name)
  }
  public func filter<S: AsyncSequence & Sendable>(
    _ question: String, records: S, options: FilterOptions = .init(), name: String? = nil
  ) -> FilterRows where S.Element == Record {
    filter(FilterQuestion(question, options: options), records: records, name: name)
  }
}
