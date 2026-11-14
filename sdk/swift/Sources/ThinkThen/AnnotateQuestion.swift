import Foundation

public struct AnnotateQuestion: SavedQuestion, Equatable, Hashable {
  public static let function = FunctionName.annotate
  public func encode(to encoder: any Encoder) throws { try definition.encode(to: encoder) }
  public let definition: JSONValue
  public init(_ questions: AnnotationQuestions, field: String? = nil) {
    definition = questionDefinition(
      "annotate", ["questions": questions.definition, "field": json(field)])
  }
}
public struct AnnotationShape<Item: Sendable>: FunctionShape {
  public typealias Answer = Annotations<Item>
  public typealias Detail = NoDetail
  public static var function: FunctionName { .annotate }
}
public typealias AnnotateRow<Item: Sendable> = Row<AnnotationShape<Item>, Item>
public typealias AnnotateRows<Item: Sendable> = Rows<AnnotateRow<Item>>
