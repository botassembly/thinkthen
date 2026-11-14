import Foundation

// Swift has no negative generic constraint for Encodable & Sendable except Array.
// Check the host value at the one-record boundary, including generic forwarding;
// a collection used as a record in records: remains the engine's responsibility.
private protocol RecordArray {}
extension Array: RecordArray {}
func requireSingleRecord<T>(_ record: T, function: FunctionName) throws {
  if record is any RecordArray {
    throw UsageError(
      code: .usageArgument,
      message: "\(function.rawValue) takes one record; use records: for an array of records")
  }
}
