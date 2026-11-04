import Foundation

extension Client {
  /// Evaluate a lazy stream of records with rank.
  public func rank<S: Sequence & Sendable>(
    _ question: RankQuestion, records: S, name: String? = nil
  ) -> RankRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      guard let item = wire.original else { throw rowABIFailure("Rank row has no original item") }
      return RankResult(item: item, row: try wire.decode(RankQuestion.self))
    }
  }
  /// Evaluate a lazy stream of records with rank.
  public func rank<S: AsyncSequence & Sendable>(
    _ question: RankQuestion, records: S, name: String? = nil
  ) -> RankRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      guard let item = wire.original else { throw rowABIFailure("Rank row has no original item") }
      return RankResult(item: item, row: try wire.decode(RankQuestion.self))
    }
  }
  /// Evaluate a lazy stream of records with rank.
  public func rank<S: Sequence & Sendable>(
    _ question: String, levels: [Choice], records: S, options: RankOptions = .init(),
    name: String? = nil
  ) -> RankRows where S.Element == Record {
    rank(RankQuestion(question, levels: levels, options: options), records: records, name: name)
  }
  /// Evaluate a lazy stream of records with rank.
  public func rank<S: AsyncSequence & Sendable>(
    _ question: String, levels: [Choice], records: S, options: RankOptions = .init(),
    name: String? = nil
  ) -> RankRows where S.Element == Record {
    rank(RankQuestion(question, levels: levels, options: options), records: records, name: name)
  }
  /// Evaluate text records without wrapping each string in Record.text.
  public func rank<S: Sequence & Sendable>(
    _ question: RankQuestion, records: S, name: String? = nil
  ) -> RankRows where S.Element == String {
    rank(question, records: TextRecords(records), name: name)
  }
  /// Evaluate text records without wrapping each string in Record.text.
  public func rank<S: AsyncSequence & Sendable>(
    _ question: RankQuestion, records: S, name: String? = nil
  ) -> RankRows where S.Element == String {
    rank(question, records: records.map { Record.text($0) }, name: name)
  }
  /// Evaluate text records using a question and options.
  public func rank<S: Sequence & Sendable>(
    _ question: String, levels: [Choice], records: S, options: RankOptions = .init(),
    name: String? = nil
  ) -> RankRows where S.Element == String {
    rank(RankQuestion(question, levels: levels, options: options), records: records, name: name)
  }
  /// Evaluate text records using a question and options.
  public func rank<S: AsyncSequence & Sendable>(
    _ question: String, levels: [Choice], records: S, options: RankOptions = .init(),
    name: String? = nil
  ) -> RankRows where S.Element == String {
    rank(RankQuestion(question, levels: levels, options: options), records: records, name: name)
  }
}
