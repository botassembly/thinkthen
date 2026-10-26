import Foundation

public struct RecognizeOptions: Sendable, Codable, Equatable, Hashable {
  public let evidence: String?
  public let threshold: Double?
  public init(evidence: String? = nil, threshold: Double? = nil) {
    self.evidence = evidence
    self.threshold = threshold
  }
}
public struct RecognizeQuestion: FunctionShape, Codable, Equatable, Hashable {
  public typealias Answer = [RecognizedName]
  public typealias Detail = RecognizeDetail
  public static let function = FunctionName.recognize
  public let kinds: [String]
  public let options: RecognizeOptions
  public init(kinds: [String], options: RecognizeOptions = .init()) {
    self.kinds = kinds
    self.options = options
  }
  enum CodingKeys: String, CodingKey { case kinds }
  public init(from decoder: any Decoder) throws {
    try checkFunction(Self.function, from: decoder)
    let c = try decoder.container(keyedBy: CodingKeys.self)
    kinds = try c.decode([String].self, forKey: .kinds)
    options = try RecognizeOptions(from: decoder)
  }
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.container(keyedBy: CodingKeys.self)
    try c.encode(kinds, forKey: .kinds)
    try options.encode(to: encoder)
    var tag = encoder.container(keyedBy: FunctionKey.self)
    try tag.encode(Self.function, forKey: .function)
  }
}
public typealias RecognizeRow = Row<RecognizeQuestion>
public typealias RecognizeRows = Rows<RecognizeRow>
