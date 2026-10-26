import Foundation

extension Client {
  public func rank<S: Sequence & Sendable>(
    _ question: RankQuestion, records: S, name: String? = nil
  ) -> RankRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(RankQuestion.self)
    }
  }
  public func rank<S: AsyncSequence & Sendable>(
    _ question: RankQuestion, records: S, name: String? = nil
  ) -> RankRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(RankQuestion.self)
    }
  }
  public func rank<S: Sequence & Sendable>(
    _ question: String, levels: [String], records: S, options: RankOptions = .init(),
    name: String? = nil
  ) -> RankRows where S.Element == Record {
    rank(RankQuestion(question, levels: levels, options: options), records: records, name: name)
  }
  public func rank<S: AsyncSequence & Sendable>(
    _ question: String, levels: [String], records: S, options: RankOptions = .init(),
    name: String? = nil
  ) -> RankRows where S.Element == Record {
    rank(RankQuestion(question, levels: levels, options: options), records: records, name: name)
  }
}
