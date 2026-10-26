import Foundation

extension Client {
  public func annotate<S: Sequence & Sendable>(
    _ question: AnnotateQuestion, records: S, name: String? = nil
  ) -> AnnotateRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(AnnotateQuestion.self)
    }
  }
  public func annotate<S: AsyncSequence & Sendable>(
    _ question: AnnotateQuestion, records: S, name: String? = nil
  ) -> AnnotateRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(AnnotateQuestion.self)
    }
  }
  public func annotate(_ question: AnnotateQuestion, record: Record, name: String? = nil)
    async throws(ThinkThenError) -> AnnotateRow
  {
    try await single(annotate(question, records: [record], name: name))
  }
  public func annotate<S: Sequence & Sendable>(
    _ questions: AnnotationQuestions, records: S, options: AnnotateOptions = .init(),
    name: String? = nil
  ) -> AnnotateRows where S.Element == Record {
    annotate(AnnotateQuestion(questions, options: options), records: records, name: name)
  }
  public func annotate<S: AsyncSequence & Sendable>(
    _ questions: AnnotationQuestions, records: S, options: AnnotateOptions = .init(),
    name: String? = nil
  ) -> AnnotateRows where S.Element == Record {
    annotate(AnnotateQuestion(questions, options: options), records: records, name: name)
  }
  public func annotate(
    _ questions: AnnotationQuestions, record: Record, options: AnnotateOptions = .init(),
    name: String? = nil
  ) async throws(ThinkThenError) -> AnnotateRow {
    try await annotate(AnnotateQuestion(questions, options: options), record: record, name: name)
  }
}
