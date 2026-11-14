import Foundation

extension Client {
  private func annotationRows<T: Encodable & Sendable>(
    _ question: AnnotateQuestion, input: any RecordSource, item: T.Type, name: String?
  ) -> AnnotateRows<T> {
    rows(question, source: input, name: name) { wire, retained in
      guard let original = retained as? T else {
        throw CallError.contract(message: "Form lost its original record")
      }
      return try wire.decode(
        AnnotationShape<T>.self, item: original,
        questions: question.definition)
    }
  }
  public func annotate<S: Sequence & Sendable>(
    _ question: AnnotateQuestion, records: S, name: String? = nil
  ) -> AnnotateRows<S.Element> where S.Element: Encodable & Sendable {
    annotationRows(question, input: source(records), item: S.Element.self, name: name)
  }
  public func annotate<S: AsyncSequence & Sendable>(
    _ question: AnnotateQuestion, records: S, name: String? = nil
  ) -> AnnotateRows<S.Element> where S.Element: Encodable & Sendable {
    annotationRows(question, input: source(records), item: S.Element.self, name: name)
  }
  public func annotate<T: Encodable & Sendable>(
    _ question: AnnotateQuestion, record: T, name: String? = nil
  ) async throws -> AnnotateRow<T> {
    try requireSingleRecord(record, function: .annotate)
    return try await single(annotate(question, records: [record], name: name))
  }
  public func annotate<S: Sequence & Sendable>(
    _ questions: AnnotationQuestions, records: S, field: String? = nil, name: String? = nil
  ) -> AnnotateRows<S.Element> where S.Element: Encodable & Sendable {
    annotate(AnnotateQuestion(questions, field: field), records: records, name: name)
  }
  public func annotate<S: AsyncSequence & Sendable>(
    _ questions: AnnotationQuestions, records: S, field: String? = nil, name: String? = nil
  ) -> AnnotateRows<S.Element> where S.Element: Encodable & Sendable {
    annotate(AnnotateQuestion(questions, field: field), records: records, name: name)
  }
  public func annotate<T: Encodable & Sendable>(
    _ questions: AnnotationQuestions, record: T, field: String? = nil, name: String? = nil
  ) async throws -> AnnotateRow<T> {
    try await annotate(AnnotateQuestion(questions, field: field), record: record, name: name)
  }
}
