import Foundation

/// Owned JSON bytes. Decoding is an optional view with the chosen type's limits.
public struct JSON: Sendable {
  public let utf8: Data
  public init(utf8: Data) { self.utf8 = utf8 }
  public static func encoding<T: Encodable>(_ value: T) throws -> JSON {
    JSON(utf8: try JSONEncoder().encode(value))
  }
  public func decode<T: Decodable>(_ type: T.Type) throws -> T {
    try JSONDecoder().decode(type, from: utf8)
  }
}
public enum Record: Sendable {
  case text(String)
  case json(JSON)
}
public struct Question: Sendable, ExpressibleByStringLiteral {
  let json: Data
  public init(stringLiteral value: String) {
    // Encoding a String has no non-finite values or custom encoder callbacks.
    json = Data("{\"type\":\"inline\",\"value\":".utf8) + Self.quoted(value) + Data("}".utf8)
  }
  public static func inline(_ value: JSON) -> Question {
    Question(json: Data("{\"type\":\"inline\",\"value\":".utf8) + value.utf8 + Data("}".utf8))
  }
  public static func toolbox(_ value: JSON, name: String? = nil) -> Question {
    var bytes = Data("{\"type\":\"toolbox\",\"value\":".utf8) + value.utf8
    if let name { bytes += Data(",\"name\":".utf8) + quoted(name) }
    bytes += Data("}".utf8)
    return Question(json: bytes)
  }
  public static func reference(_ name: String) -> Question {
    Question(json: Data("{\"type\":\"reference\",\"name\":".utf8) + quoted(name) + Data("}".utf8))
  }
  private init(json: Data) { self.json = json }
  private static func quoted(_ value: String) -> Data {
    // JSONSerialization's strings-only array cannot fail JSON admission.
    let encoded = try! JSONSerialization.data(withJSONObject: [value], options: [.fragmentsAllowed])
    return encoded.dropFirst().dropLast()
  }
}
public struct ClientSettings: Encodable, Sendable {
  public let mode: String?, adapter: String?, url: String?, model: String?, keySource: String?
  public let batch: Double?
  public init(
    mode: String? = nil, adapter: String? = nil, url: String? = nil,
    model: String? = nil, keySource: String? = nil, batch: Double? = nil
  ) {
    self.mode = mode
    self.adapter = adapter
    self.url = url
    self.model = model
    self.keySource = keySource
    self.batch = batch
  }
  enum CodingKeys: String, CodingKey {
    case mode, adapter, url, model, batch
    case keySource = "key_source"
  }
}
public enum Threshold: Encodable, Sendable, ExpressibleByFloatLiteral, ExpressibleByIntegerLiteral,
  ExpressibleByStringLiteral
{
  case number(Double)
  case band(String)
  public init(floatLiteral value: Double) { self = .number(value) }
  public init(integerLiteral value: Int) { self = .number(Double(value)) }
  public init(stringLiteral value: String) { self = .band(value) }
  public func encode(to encoder: any Encoder) throws {
    var container = encoder.singleValueContainer()
    switch self {
    case .number(let value): try container.encode(value)
    case .band(let value): try container.encode(value)
    }
  }
}
public struct CallOptions: Encodable, Sendable {
  public let threshold: Threshold?, on: String?
  public init(threshold: Threshold? = nil, on: String? = nil) {
    self.threshold = threshold
    self.on = on
  }
}
public struct EvidenceOptions: Encodable, Sendable {
  public let on: String?
  public init(on: String? = nil) { self.on = on }
}
public struct RankOptions: Encodable, Sendable {
  public let on: String?, top: Double?
  public let levels: [String]?
  public init(on: String? = nil, top: Double? = nil, levels: [String]? = nil) {
    self.on = on
    self.top = top
    self.levels = levels
  }
}
public enum RowStatus: String, Decodable, Sendable { case answered, unsure, failed }
public struct ThinkThenError: Error, Decodable, Sendable {
  public let code: String, message: String
}
public struct NativeLoadError: Error, Sendable { public let message: String }
public enum RowsIterationError: Error, Sendable { case alreadyIterated, concurrentNext }
public struct Row<Answer: Sendable>: Sendable {
  public let id: JSON?, status: RowStatus, answer: Answer?, p: Double?, error: ThinkThenError?
  public let position: Int, kept: Bool, detail: JSON?
}
public struct TextSpan: Codable, Sendable { public let text: String, start: Int, end: Int }
public struct RecognizedName: Codable, Sendable {
  public let text: String, start: Int, end: Int, kind: String, p: Double
}
public struct RelationName: Codable, Sendable {
  public let text: String, start: Int, end: Int, kind: String
}
public struct RelationEdge: Codable, Sendable {
  public let source: RelationName, target: RelationName, relation: String, p: Double, either: Bool?
}
