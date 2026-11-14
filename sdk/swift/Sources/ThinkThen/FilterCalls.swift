import Foundation
import Synchronization

extension Client {
  public func filter<S: Sequence & Sendable>(
    _ question: FilterQuestion, records: S, name: String? = nil
  ) -> Rows<S.Element> where S.Element: Encodable & Sendable {
    keptRecords(evaluate(question, input: source(records), item: S.Element.self, name: name))
  }
  public func filter<S: AsyncSequence & Sendable>(
    _ question: FilterQuestion, records: S, name: String? = nil
  ) -> Rows<S.Element> where S.Element: Encodable & Sendable {
    keptRecords(evaluate(question, input: source(records), item: S.Element.self, name: name))
  }
  public func filter<S: Sequence & Sendable>(
    _ question: String, records: S, yesMeans: String? = nil, noMeans: String? = nil,
    threshold: Double? = nil, field: String? = nil, name: String? = nil
  ) -> Rows<S.Element> where S.Element: Encodable & Sendable {
    filter(
      FilterQuestion(
        question, yesMeans: yesMeans, noMeans: noMeans, threshold: threshold, field: field),
      records: records, name: name)
  }
  public func filter<S: AsyncSequence & Sendable>(
    _ question: String, records: S, yesMeans: String? = nil, noMeans: String? = nil,
    threshold: Double? = nil, field: String? = nil, name: String? = nil
  ) -> Rows<S.Element> where S.Element: Encodable & Sendable {
    filter(
      FilterQuestion(
        question, yesMeans: yesMeans, noMeans: noMeans, threshold: threshold, field: field),
      records: records, name: name)
  }
}

private func keptRecords<T: Sendable>(_ rows: Rows<Row<FilterQuestion, T>>) -> Rows<T> {
  Rows(
    iterator: {
      let iterator = rows.makeAsyncIterator()
      let failures = Mutex<[AnyRow]>([])
      return Rows<T>.AsyncIterator(
        next: {
          do {
            while let result = try await iterator.next() {
              if result.isFailed {
                failures.withLock { $0.append(result.erased) }
              } else if try result.answer {
                return result.item
              }
            }
          } catch let error as UsageError where error.code == .usageThreshold {
            throw UsageError(
              code: error.code, message: error.message + "; use decide(_:records:) for two bars")
          }
          let failed = failures.withLock { rows in
            let saved = rows
            rows.removeAll()
            return saved
          }
          if let first = failed.first?.error { throw Failed(first, rows: failed) }
          return nil
        }, cancel: { rows.cancel() })
    }, cancel: { rows.cancel() })
}
