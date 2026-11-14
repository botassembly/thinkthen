import Foundation

// Engine offsets are Unicode scalar counts. Swift views keep indexes into the
// selected source while each leaf retains its canonical numbers for detached encoding.
// As in the shared engine Offset value, equality/hash use canonical positions;
// String.Index is the host projection, not a second semantic coordinate value.
private struct SpanOffsets: Sendable, Equatable, Hashable {
  let lower: Int
  let upper: Int
  let start: String.Index
  let end: String.Index
  init(lower: Int, upper: Int, decoder: any Decoder) throws {
    guard let text = decoder.userInfo[.sourceText] as? String,
      lower >= 0, upper >= lower,
      let start = text.unicodeScalars.index(
        text.unicodeScalars.startIndex, offsetBy: lower, limitedBy: text.unicodeScalars.endIndex),
      let end = text.unicodeScalars.index(
        start, offsetBy: upper - lower, limitedBy: text.unicodeScalars.endIndex)
    else { throw CallError.contract(message: "Native span lies outside the selected source") }
    self.lower = lower
    self.upper = upper
    self.start = start
    self.end = end
  }
  static func == (lhs: Self, rhs: Self) -> Bool { lhs.lower == rhs.lower && lhs.upper == rhs.upper }
  func hash(into hasher: inout Hasher) {
    hasher.combine(lower)
    hasher.combine(upper)
  }
}
public struct TextSpan: Sendable, Codable, Equatable, Hashable {
  public let text: String
  private let offsets: SpanOffsets
  public var start: String.Index { offsets.start }
  public var end: String.Index { offsets.end }
  enum CodingKeys: String, CodingKey { case text, start, end }
  public init(from decoder: any Decoder) throws {
    let c = try decoder.container(keyedBy: CodingKeys.self)
    text = try c.decode(String.self, forKey: .text)
    offsets = try SpanOffsets(
      lower: c.decode(Int.self, forKey: .start), upper: c.decode(Int.self, forKey: .end),
      decoder: decoder)
  }
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.container(keyedBy: CodingKeys.self)
    try c.encode(text, forKey: .text)
    try c.encode(offsets.lower, forKey: .start)
    try c.encode(offsets.upper, forKey: .end)
  }
}
public struct RecognizedName: Sendable, Codable, Equatable, Hashable {
  public let text: String
  private let offsets: SpanOffsets
  public var start: String.Index { offsets.start }
  public var end: String.Index { offsets.end }
  public let kind: String
  public let probability: Double
  enum CodingKeys: String, CodingKey { case text, start, end, kind, probability }
  public init(from decoder: any Decoder) throws {
    let c = try decoder.container(keyedBy: CodingKeys.self)
    text = try c.decode(String.self, forKey: .text)
    offsets = try SpanOffsets(
      lower: c.decode(Int.self, forKey: .start), upper: c.decode(Int.self, forKey: .end),
      decoder: decoder)
    kind = try c.decode(String.self, forKey: .kind)
    probability = try c.decode(Double.self, forKey: .probability)
  }
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.container(keyedBy: CodingKeys.self)
    try c.encode(text, forKey: .text)
    try c.encode(offsets.lower, forKey: .start)
    try c.encode(offsets.upper, forKey: .end)
    try c.encode(kind, forKey: .kind)
    try c.encode(probability, forKey: .probability)
  }
}
public struct RelationName: Sendable, Codable, Equatable, Hashable {
  public let text: String
  private let offsets: SpanOffsets
  public var start: String.Index { offsets.start }
  public var end: String.Index { offsets.end }
  public let kind: String
  enum CodingKeys: String, CodingKey { case text, start, end, kind }
  public init(from decoder: any Decoder) throws {
    let c = try decoder.container(keyedBy: CodingKeys.self)
    text = try c.decode(String.self, forKey: .text)
    offsets = try SpanOffsets(
      lower: c.decode(Int.self, forKey: .start), upper: c.decode(Int.self, forKey: .end),
      decoder: decoder)
    kind = try c.decode(String.self, forKey: .kind)
  }
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.container(keyedBy: CodingKeys.self)
    try c.encode(text, forKey: .text)
    try c.encode(offsets.lower, forKey: .start)
    try c.encode(offsets.upper, forKey: .end)
    try c.encode(kind, forKey: .kind)
  }
}
public struct KindDetail: Sendable, Codable, Equatable, Hashable {
  public let text: String
  private let offsets: SpanOffsets
  public var start: String.Index { offsets.start }
  public var end: String.Index { offsets.end }
  public let kinds: [String: Double]
  enum CodingKeys: String, CodingKey { case text, start, end, kinds }
  public init(from decoder: any Decoder) throws {
    let c = try decoder.container(keyedBy: CodingKeys.self)
    text = try c.decode(String.self, forKey: .text)
    offsets = try SpanOffsets(
      lower: c.decode(Int.self, forKey: .start), upper: c.decode(Int.self, forKey: .end),
      decoder: decoder)
    kinds = try c.decode([String: Double].self, forKey: .kinds)
  }
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.container(keyedBy: CodingKeys.self)
    try c.encode(text, forKey: .text)
    try c.encode(offsets.lower, forKey: .start)
    try c.encode(offsets.upper, forKey: .end)
    try c.encode(kinds, forKey: .kinds)
  }
}
public struct PieceDetail: Sendable, Codable, Equatable, Hashable {
  public let text: String
  private let offsets: SpanOffsets
  public var start: String.Index { offsets.start }
  public var end: String.Index { offsets.end }
  public let tags: [BoundaryTag: Double]
  enum CodingKeys: String, CodingKey { case text, start, end, tags }
  public init(from decoder: any Decoder) throws {
    let c = try decoder.container(keyedBy: CodingKeys.self)
    text = try c.decode(String.self, forKey: .text)
    offsets = try SpanOffsets(
      lower: c.decode(Int.self, forKey: .start), upper: c.decode(Int.self, forKey: .end),
      decoder: decoder)
    let raw = try c.decode([String: Double].self, forKey: .tags)
    var values: [BoundaryTag: Double] = [:]
    for (key, value) in raw {
      guard let tag = BoundaryTag(rawValue: key) else {
        throw CallError.contract(message: "Unknown boundary tag")
      }
      values[tag] = value
    }
    tags = values
  }
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.container(keyedBy: CodingKeys.self)
    try c.encode(text, forKey: .text)
    try c.encode(offsets.lower, forKey: .start)
    try c.encode(offsets.upper, forKey: .end)
    try c.encode(
      Dictionary(uniqueKeysWithValues: tags.map { ($0.key.rawValue, $0.value) }), forKey: .tags)
  }
}
public struct Relation: Sendable, Codable, Equatable, Hashable {
  public let source: RelationName
  public let target: RelationName
  public let relation: String
  public let probability: Double
  public let either: Bool
  enum CodingKeys: String, CodingKey { case source, target, relation, probability, either }
  public init(from decoder: any Decoder) throws {
    let c = try decoder.container(keyedBy: CodingKeys.self)
    source = try c.decode(RelationName.self, forKey: .source)
    target = try c.decode(RelationName.self, forKey: .target)
    relation = try c.decode(String.self, forKey: .relation)
    probability = try c.decode(Double.self, forKey: .probability)
    either = try c.decodeIfPresent(Bool.self, forKey: .either) ?? false
  }
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.container(keyedBy: CodingKeys.self)
    try c.encode(source, forKey: .source)
    try c.encode(target, forKey: .target)
    try c.encode(relation, forKey: .relation)
    try c.encode(probability, forKey: .probability)
    if either { try c.encode(either, forKey: .either) }
  }
}
public struct LabelDetail: Sendable, Codable, Equatable, Hashable {
  public let labels: [String: Double]
}
public struct LevelDetail: Sendable, Codable, Equatable, Hashable { public let levels: [Double] }
public struct RelationDetail: Sendable, Codable, Equatable, Hashable {
  public let names: [RecognizedName]
  public let pairs: [Relation]
}
public struct RecognizeDetail: Sendable, Codable, Equatable, Hashable {
  public let pieces: [PieceDetail]
  public let names: [KindDetail]
}
public enum BoundaryTag: String, Sendable, Codable {
  case begin = "BEGIN"
  case inside = "INSIDE"
  case end = "END"
  case single = "SINGLE"
  case out = "OUT"
}
