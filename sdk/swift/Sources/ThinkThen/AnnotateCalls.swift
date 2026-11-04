import Foundation

extension Client {
  /// Evaluate a lazy stream of records with annotate.
  public func annotate<S: Sequence & Sendable>(
    _ question: AnnotateQuestion, records: S, name: String? = nil
  ) -> AnnotateRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(AnnotateQuestion.self)
    }
  }
  /// Evaluate a lazy stream of records with annotate.
  public func annotate<S: AsyncSequence & Sendable>(
    _ question: AnnotateQuestion, records: S, name: String? = nil
  ) -> AnnotateRows where S.Element == Record {
    rows(question, source: source(records), name: name) { wire, _ in
      try wire.decode(AnnotateQuestion.self)
    }
  }
  /// Evaluate one record with annotate.
  public func annotate(_ question: AnnotateQuestion, record: Record, name: String? = nil)
    async throws -> AnnotateRow
  {
    try await single(annotate(question, records: [record], name: name))
  }
  /// Evaluate a lazy stream of records with annotate.
  public func annotate<S: Sequence & Sendable>(
    _ questions: AnnotationQuestions, records: S, options: AnnotateOptions = .init(),
    name: String? = nil
  ) -> AnnotateRows where S.Element == Record {
    annotate(AnnotateQuestion(questions, options: options), records: records, name: name)
  }
  /// Evaluate a lazy stream of records with annotate.
  public func annotate<S: AsyncSequence & Sendable>(
    _ questions: AnnotationQuestions, records: S, options: AnnotateOptions = .init(),
    name: String? = nil
  ) -> AnnotateRows where S.Element == Record {
    annotate(AnnotateQuestion(questions, options: options), records: records, name: name)
  }
  /// Evaluate one record with annotate.
  public func annotate(
    _ questions: AnnotationQuestions, record: Record, options: AnnotateOptions = .init(),
    name: String? = nil
  ) async throws -> AnnotateRow {
    try await annotate(AnnotateQuestion(questions, options: options), record: record, name: name)
  }
  /// Evaluate text input using saved annotation questions.
  public func annotate<S: Sequence & Sendable>(
    _ question: AnnotateQuestion, records: S, name: String? = nil
  ) -> AnnotateRows where S.Element == String {
    annotate(question, records: TextRecords(records), name: name)
  }
  /// Evaluate text input without converting each value to Record.text.
  public func annotate<S: Sequence & Sendable>(
    _ questions: AnnotationQuestions, records: S, options: AnnotateOptions = .init(),
    name: String? = nil
  ) -> AnnotateRows where S.Element == String {
    annotate(AnnotateQuestion(questions, options: options), records: records, name: name)
  }
  /// Evaluate text input using saved annotation questions.
  public func annotate<S: AsyncSequence & Sendable>(
    _ question: AnnotateQuestion, records: S, name: String? = nil
  ) -> AnnotateRows where S.Element == String {
    annotate(question, records: records.map { Record.text($0) }, name: name)
  }
  /// Evaluate text input without converting each value to Record.text.
  public func annotate<S: AsyncSequence & Sendable>(
    _ questions: AnnotationQuestions, records: S, options: AnnotateOptions = .init(),
    name: String? = nil
  ) -> AnnotateRows where S.Element == String {
    annotate(AnnotateQuestion(questions, options: options), records: records, name: name)
  }
  /// Evaluate a text value without wrapping it in Record.text.
  public func annotate(_ question: AnnotateQuestion, record: String, name: String? = nil)
    async throws -> AnnotateRow
  {
    try await annotate(question, record: .text(record), name: name)
  }
  /// Evaluate a text value without wrapping it in Record.text.
  public func annotate(
    _ questions: AnnotationQuestions, record: String, options: AnnotateOptions = .init(),
    name: String? = nil
  ) async throws -> AnnotateRow {
    try await annotate(questions, record: .text(record), options: options, name: name)
  }
}
