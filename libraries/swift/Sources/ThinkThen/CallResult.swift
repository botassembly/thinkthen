/// One owned answer with its native records and final call facts.
public struct CallResult<Value: Sendable, Probability: Sendable, Records: Sendable>: Sendable {
  public let value: Value
  public let probability: Probability
  public let results: Records
  public let facts: Facts
  let call: Call
}
