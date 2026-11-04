import Foundation
import Synchronization

extension Client {
  /// Read every filter outcome with its original record, including unsure and failed rows.
  public func filterResults<S: Sequence & Sendable>(
    _ question: FilterQuestion, records: S, name: String? = nil
  ) -> FilterRows where S.Element == Record {
    rows(question, source: source(records), name: name, retainItems: true) { wire, item in
      guard let item else {
        throw rowABIFailure("Filter row has no original item")
      }
      return FilterResult(item: item, row: try wire.decode(FilterQuestion.self))
    }
  }
  /// Read every filter outcome with its original record, including unsure and failed rows.
  public func filterResults<S: AsyncSequence & Sendable>(
    _ question: FilterQuestion, records: S, name: String? = nil
  ) -> FilterRows where S.Element == Record {
    rows(question, source: source(records), name: name, retainItems: true) { wire, item in
      guard let item else {
        throw rowABIFailure("Filter row has no original item")
      }
      return FilterResult(item: item, row: try wire.decode(FilterQuestion.self))
    }
  }
  /// Read every filter outcome with its original record, including unsure and failed rows.
  public func filterResults<S: Sequence & Sendable>(
    _ question: String, records: S, yesMeans: String? = nil, noMeans: String? = nil,
    options: FilterOptions = .init(), name: String? = nil
  ) -> FilterRows where S.Element == Record {
    filterResults(
      FilterQuestion(question, yesMeans: yesMeans, noMeans: noMeans, options: options),
      records: records, name: name)
  }
  /// Read every filter outcome with its original record, including unsure and failed rows.
  public func filterResults<S: AsyncSequence & Sendable>(
    _ question: String, records: S, yesMeans: String? = nil, noMeans: String? = nil,
    options: FilterOptions = .init(), name: String? = nil
  ) -> FilterRows where S.Element == Record {
    filterResults(
      FilterQuestion(question, yesMeans: yesMeans, noMeans: noMeans, options: options),
      records: records, name: name)
  }
  /// Evaluate text records without wrapping each string in Record.text.
  public func filterResults<S: Sequence & Sendable>(
    _ question: FilterQuestion, records: S, name: String? = nil
  ) -> FilterRows where S.Element == String {
    filterResults(question, records: TextRecords(records), name: name)
  }
  /// Evaluate text records without wrapping each string in Record.text.
  public func filterResults<S: AsyncSequence & Sendable>(
    _ question: FilterQuestion, records: S, name: String? = nil
  ) -> FilterRows where S.Element == String {
    filterResults(question, records: records.map { Record.text($0) }, name: name)
  }
  /// Evaluate text records using a question and options.
  public func filterResults<S: Sequence & Sendable>(
    _ question: String, records: S, yesMeans: String? = nil, noMeans: String? = nil,
    options: FilterOptions = .init(), name: String? = nil
  ) -> FilterRows where S.Element == String {
    filterResults(
      FilterQuestion(question, yesMeans: yesMeans, noMeans: noMeans, options: options),
      records: records, name: name)
  }
  /// Evaluate text records using a question and options.
  public func filterResults<S: AsyncSequence & Sendable>(
    _ question: String, records: S, yesMeans: String? = nil, noMeans: String? = nil,
    options: FilterOptions = .init(), name: String? = nil
  ) -> FilterRows where S.Element == String {
    filterResults(
      FilterQuestion(question, yesMeans: yesMeans, noMeans: noMeans, options: options),
      records: records, name: name)
  }
}

/// Return failed envelopes alongside kept envelopes instead of throwing after iteration.
public enum FilterFailures: Sendable { case row }

extension Client {
  /// Yield original kept records, then throw one error carrying all failed rows.
  public func filter<S: Sequence & Sendable>(
    _ question: FilterQuestion, records: S, name: String? = nil
  ) -> Rows<Record> where S.Element == Record {
    keptRecords(filterResults(question, records: records, name: name))
  }
  /// Yield original kept records; deliver kept records before an aggregate record failure.
  public func filter<S: AsyncSequence & Sendable>(
    _ question: FilterQuestion, records: S, name: String? = nil
  ) -> Rows<Record> where S.Element == Record {
    keptRecords(filterResults(question, records: records, name: name))
  }
  /// Yield original kept records; deliver kept records before an aggregate record failure.
  public func filter<S: Sequence & Sendable>(
    _ question: String, records: S, yesMeans: String? = nil, noMeans: String? = nil,
    options: FilterOptions = .init(), name: String? = nil
  ) -> Rows<Record> where S.Element == Record {
    filter(
      FilterQuestion(question, yesMeans: yesMeans, noMeans: noMeans, options: options),
      records: records, name: name)
  }
  /// Yield original kept records; deliver kept records before an aggregate record failure.
  public func filter<S: AsyncSequence & Sendable>(
    _ question: String, records: S, yesMeans: String? = nil, noMeans: String? = nil,
    options: FilterOptions = .init(), name: String? = nil
  ) -> Rows<Record> where S.Element == Record {
    filter(
      FilterQuestion(question, yesMeans: yesMeans, noMeans: noMeans, options: options),
      records: records, name: name)
  }
  /// Yield original kept records; deliver kept records before an aggregate record failure.
  public func filter<S: Sequence & Sendable>(
    _ question: String, records: S, yesMeans: String? = nil, noMeans: String? = nil,
    options: FilterOptions = .init(), name: String? = nil
  ) -> Rows<String> where S.Element == String {
    textRecords(
      filter(
        FilterQuestion(question, yesMeans: yesMeans, noMeans: noMeans, options: options),
        records: TextRecords(records),
        name: name))
  }
  /// Yield original kept records; deliver kept records before an aggregate record failure.
  public func filter<S: AsyncSequence & Sendable>(
    _ question: String, records: S, yesMeans: String? = nil, noMeans: String? = nil,
    options: FilterOptions = .init(), name: String? = nil
  ) -> Rows<String> where S.Element == String {
    textRecords(
      filter(
        FilterQuestion(question, yesMeans: yesMeans, noMeans: noMeans, options: options),
        records: records.map { Record.text($0) },
        name: name))
  }
  /// Yield kept and failed envelopes; omit unsure and answered nonmatches.
  public func filter<S: Sequence & Sendable>(
    _ question: FilterQuestion, records: S, failures: FilterFailures, name: String? = nil
  ) -> FilterRows where S.Element == Record {
    keptAndFailed(filterResults(question, records: records, name: name))
  }
  /// Yield kept and failed envelopes; omit unsure and answered nonmatches.
  public func filter<S: AsyncSequence & Sendable>(
    _ question: FilterQuestion, records: S, failures: FilterFailures, name: String? = nil
  ) -> FilterRows where S.Element == Record {
    keptAndFailed(filterResults(question, records: records, name: name))
  }
  /// Yield original kept text records and report every failed row after them.
  public func filter<S: Sequence & Sendable>(
    _ question: FilterQuestion, records: S, name: String? = nil
  ) -> Rows<String> where S.Element == String {
    textRecords(filter(question, records: TextRecords(records), name: name))
  }
  /// Yield kept and failed envelopes from text input, omitting nonmatches and unsure rows.
  public func filter<S: Sequence & Sendable>(
    _ question: FilterQuestion, records: S, failures: FilterFailures, name: String? = nil
  ) -> FilterRows where S.Element == String {
    filter(question, records: TextRecords(records), failures: failures, name: name)
  }
  /// Yield kept and failed envelopes instead of throwing an aggregate row failure.
  public func filter<S: Sequence & Sendable>(
    _ question: String, records: S, failures: FilterFailures,
    yesMeans: String? = nil, noMeans: String? = nil,
    options: FilterOptions = .init(), name: String? = nil
  ) -> FilterRows where S.Element == Record {
    filter(
      FilterQuestion(question, yesMeans: yesMeans, noMeans: noMeans, options: options),
      records: records, failures: failures, name: name)
  }
  /// Yield kept and failed envelopes instead of throwing an aggregate row failure.
  public func filter<S: Sequence & Sendable>(
    _ question: String, records: S, failures: FilterFailures,
    yesMeans: String? = nil, noMeans: String? = nil,
    options: FilterOptions = .init(), name: String? = nil
  ) -> FilterRows where S.Element == String {
    filter(
      FilterQuestion(question, yesMeans: yesMeans, noMeans: noMeans, options: options),
      records: records, failures: failures, name: name)
  }
  /// Yield original kept text records and report every failed row after them.
  public func filter<S: AsyncSequence & Sendable>(
    _ question: FilterQuestion, records: S, name: String? = nil
  ) -> Rows<String> where S.Element == String {
    textRecords(filter(question, records: records.map { Record.text($0) }, name: name))
  }
  /// Yield kept and failed envelopes from text input, omitting nonmatches and unsure rows.
  public func filter<S: AsyncSequence & Sendable>(
    _ question: FilterQuestion, records: S, failures: FilterFailures, name: String? = nil
  ) -> FilterRows where S.Element == String {
    filter(question, records: records.map { Record.text($0) }, failures: failures, name: name)
  }
  /// Yield kept and failed envelopes instead of throwing an aggregate row failure.
  public func filter<S: AsyncSequence & Sendable>(
    _ question: String, records: S, failures: FilterFailures,
    yesMeans: String? = nil, noMeans: String? = nil,
    options: FilterOptions = .init(), name: String? = nil
  ) -> FilterRows where S.Element == Record {
    filter(
      FilterQuestion(question, yesMeans: yesMeans, noMeans: noMeans, options: options),
      records: records, failures: failures, name: name)
  }
  /// Yield kept and failed envelopes instead of throwing an aggregate row failure.
  public func filter<S: AsyncSequence & Sendable>(
    _ question: String, records: S, failures: FilterFailures,
    yesMeans: String? = nil, noMeans: String? = nil,
    options: FilterOptions = .init(), name: String? = nil
  ) -> FilterRows where S.Element == String {
    filter(
      FilterQuestion(question, yesMeans: yesMeans, noMeans: noMeans, options: options),
      records: records, failures: failures, name: name)
  }

}

private func keptRecords(_ rows: FilterRows) -> Rows<Record> {
  Rows(
    iterator: {
      let iterator = rows.makeAsyncIterator()
      let failures = Mutex<[FilterRow]>([])
      return Rows<Record>.AsyncIterator(
        next: {
          while let result = try await iterator.next() {
            if result.row.isFailed {
              failures.withLock { $0.append(result.row) }
            } else if result.kept {
              return result.item
            }
          }
          let failed = failures.withLock { rows in
            let saved = rows
            rows.removeAll()
            return saved
          }
          if let first = failed.first?.failure {
            throw RowFailure(
              code: first.code, message: first.message, httpStatus: first.httpStatus,
              failedRows: failed)
          }
          return nil
        }, cancel: { rows.cancel() })
    }, cancel: { rows.cancel() })
}
private func keptAndFailed(_ rows: FilterRows) -> FilterRows {
  Rows(
    iterator: {
      let iterator = rows.makeAsyncIterator()
      return FilterRows.AsyncIterator(
        next: {
          while let result = try await iterator.next() {
            if result.kept || result.row.isFailed { return result }
          }
          return nil
        }, cancel: { rows.cancel() })
    }, cancel: { rows.cancel() })
}

private func textRecords(_ rows: Rows<Record>) -> Rows<String> {
  Rows(
    iterator: {
      let iterator = rows.makeAsyncIterator()
      return Rows<String>.AsyncIterator(
        next: {
          guard let record = try await iterator.next() else { return nil }
          guard case .text(let text) = record else {
            throw rowABIFailure("Text filter changed the original record kind")
          }
          return text
        }, cancel: { rows.cancel() })
    }, cancel: { rows.cancel() })
}
