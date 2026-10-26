import Foundation

public struct AnnotateOptions: Sendable, Codable, Equatable, Hashable {
  public let evidence: String?
  public init(evidence: String? = nil) {
    self.evidence = evidence
  }
}
public struct AnnotateQuestion: FunctionShape, Codable, Equatable, Hashable {
  public typealias Answer = Annotations
  public typealias Detail = NoDetail
  public static let function = FunctionName.annotate
  public let questions: AnnotationQuestions
  public let options: AnnotateOptions
  public init(_ questions: AnnotationQuestions, options: AnnotateOptions = .init()) {
    self.questions = questions
    self.options = options
  }
  enum CodingKeys: String, CodingKey { case questions }
  public init(from decoder: any Decoder) throws {
    try checkFunction(Self.function, from: decoder)
    let c = try decoder.container(keyedBy: CodingKeys.self)
    questions = try c.decode(AnnotationQuestions.self, forKey: .questions)
    options = try AnnotateOptions(from: decoder)
  }
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.container(keyedBy: CodingKeys.self)
    try c.encode(questions, forKey: .questions)
    try options.encode(to: encoder)
    var tag = encoder.container(keyedBy: FunctionKey.self)
    try tag.encode(Self.function, forKey: .function)
  }
}
public typealias AnnotateRow = Row<AnnotateQuestion>
public typealias AnnotateRows = Rows<AnnotateRow>
