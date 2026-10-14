import CThinkThen
// Generated from the shared Rust graph. Do not edit.
import Foundation

public struct Annotation: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `answerId`: AnswerId
  public let `answers`: [String: AnnotationMember]
  public let `file`: Presence<String>
  public let `firstLine`: Presence<UInt64>
  public let `index`: Presence<UInt64>
  public let `input`: DataValue
  public let `lastLine`: Presence<UInt64>
  public let `meta`: Meta
  public let `position`: Presence<Position>
  public let `schema`: ReceiptVersion
  public let `source`: Presence<PhysicalSource>
  public let `value`: AnnotatedRow
  public init(
    extensions: [String: DataValue] = [:], `answerId`: AnswerId,
    `answers`: [String: AnnotationMember], `file`: Presence<String> = .absent,
    `firstLine`: Presence<UInt64> = .absent, `index`: Presence<UInt64> = .absent,
    `input`: DataValue, `lastLine`: Presence<UInt64> = .absent, `meta`: Meta,
    `position`: Presence<Position> = .absent, `schema`: ReceiptVersion,
    `source`: Presence<PhysicalSource> = .absent, `value`: AnnotatedRow
  ) {
    self.extensions = extensions
    self.`answerId` = `answerId`
    self.`answers` = `answers`
    self.`file` = `file`
    self.`firstLine` = `firstLine`
    self.`index` = `index`
    self.`input` = `input`
    self.`lastLine` = `lastLine`
    self.`meta` = `meta`
    self.`position` = `position`
    self.`schema` = `schema`
    self.`source` = `source`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> Annotation {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "answer_id", "answers", "file", "first_line", "index", "input", "last_line", "meta",
      "position", "schema", "source", "value",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      answerId: try AnswerId.read(jsonRequired(object, "answer_id")),
      answers: try [String: AnnotationMember].read(jsonRequired(object, "answers")),
      file: try readPresence(object, "file", String.self),
      firstLine: try readPresence(object, "first_line", UInt64.self),
      index: try readPresence(object, "index", UInt64.self),
      input: try DataValue.read(jsonRequired(object, "input")),
      lastLine: try readPresence(object, "last_line", UInt64.self),
      meta: try Meta.read(jsonRequired(object, "meta")),
      position: try readPresence(object, "position", Position.self),
      schema: try ReceiptVersion.read(jsonRequired(object, "schema")),
      source: try readPresence(object, "source", PhysicalSource.self),
      value: try AnnotatedRow.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "answer_id", "answers", "file", "first_line", "index", "input", "last_line", "meta",
      "position", "schema", "source", "value",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    object["answer_id"] = `answerId`.json
    object["answers"] = `answers`.json
    writePresence(`file`, "file", &object)
    writePresence(`firstLine`, "first_line", &object)
    writePresence(`index`, "index", &object)
    object["input"] = `input`.json
    writePresence(`lastLine`, "last_line", &object)
    object["meta"] = `meta`.json
    writePresence(`position`, "position", &object)
    object["schema"] = `schema`.json
    writePresence(`source`, "source", &object)
    object["value"] = `value`.json
    return .object(object)
  }
}
public indirect enum AnnotationMember: ValueRepresentable {
  case `answerId`(AnnotationMemberAnswerId)
  case `failureId`(AnnotationMemberFailureId)
  static func read(_ json: DataValue) throws -> AnnotationMember {
    let object = try jsonObject(json)
    if object["answer_id"] != nil && object["answer_id"] != .null {
      return .`answerId`(try AnnotationMemberAnswerId.read(json))
    }
    if object["failure_id"] != nil && object["failure_id"] != .null {
      return .`failureId`(try AnnotationMemberFailureId.read(json))
    }
    throw ValueConversionError("Unknown AnnotationMember alternative")
  }
  var json: DataValue {
    switch self {
    case .`answerId`(let value): return value.json
    case .`failureId`(let value): return value.json
    }
  }
}
public struct AnnotationMemberAnswerId: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `answer`: Answer
  public let `answerId`: AnswerId
  public let `observations`: [Observation]
  public let `question`: QuestionDocument
  public let `questionSources`: [QuestionSource]
  public let `request`: String
  public let `threshold`: ReceiptThreshold?
  public let `usage`: Presence<Usage>
  public let `value`: Value
  public init(
    extensions: [String: DataValue] = [:], `answer`: Answer, `answerId`: AnswerId,
    `observations`: [Observation], `question`: QuestionDocument,
    `questionSources`: [QuestionSource], `request`: String, `threshold`: ReceiptThreshold?,
    `usage`: Presence<Usage> = .absent, `value`: Value
  ) {
    self.extensions = extensions
    self.`answer` = `answer`
    self.`answerId` = `answerId`
    self.`observations` = `observations`
    self.`question` = `question`
    self.`questionSources` = `questionSources`
    self.`request` = `request`
    self.`threshold` = `threshold`
    self.`usage` = `usage`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> AnnotationMemberAnswerId {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "answer", "answer_id", "observations", "question", "question_sources", "request", "threshold",
      "usage", "value",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      answer: try Answer.read(jsonRequired(object, "answer")),
      answerId: try AnswerId.read(jsonRequired(object, "answer_id")),
      observations: try [Observation].read(jsonRequired(object, "observations")),
      question: try QuestionDocument.read(jsonRequired(object, "question")),
      questionSources: try [QuestionSource].read(jsonRequired(object, "question_sources")),
      request: try String.read(jsonRequired(object, "request")),
      threshold: try ReceiptThreshold?.read(jsonRequired(object, "threshold")),
      usage: try readPresence(object, "usage", Usage.self),
      value: try Value.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "answer", "answer_id", "observations", "question", "question_sources", "request", "threshold",
      "usage", "value",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    object["answer"] = `answer`.json
    object["answer_id"] = `answerId`.json
    object["observations"] = `observations`.json
    object["question"] = `question`.json
    object["question_sources"] = `questionSources`.json
    object["request"] = `request`.json
    object["threshold"] = `threshold`.json
    writePresence(`usage`, "usage", &object)
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct AnnotationMemberFailureId: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `failure`: Failure
  public let `failureId`: FailureId
  public let `observations`: [Observation]
  public let `question`: QuestionDocument
  public let `questionSources`: [QuestionSource]
  public let `request`: String
  public let `threshold`: ReceiptThreshold?
  public let `usage`: Presence<Usage>
  public init(
    extensions: [String: DataValue] = [:], `failure`: Failure, `failureId`: FailureId,
    `observations`: [Observation], `question`: QuestionDocument,
    `questionSources`: [QuestionSource], `request`: String, `threshold`: ReceiptThreshold?,
    `usage`: Presence<Usage> = .absent
  ) {
    self.extensions = extensions
    self.`failure` = `failure`
    self.`failureId` = `failureId`
    self.`observations` = `observations`
    self.`question` = `question`
    self.`questionSources` = `questionSources`
    self.`request` = `request`
    self.`threshold` = `threshold`
    self.`usage` = `usage`
  }
  static func read(_ json: DataValue) throws -> AnnotationMemberFailureId {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "failure", "failure_id", "observations", "question", "question_sources", "request",
      "threshold", "usage",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      failure: try Failure.read(jsonRequired(object, "failure")),
      failureId: try FailureId.read(jsonRequired(object, "failure_id")),
      observations: try [Observation].read(jsonRequired(object, "observations")),
      question: try QuestionDocument.read(jsonRequired(object, "question")),
      questionSources: try [QuestionSource].read(jsonRequired(object, "question_sources")),
      request: try String.read(jsonRequired(object, "request")),
      threshold: try ReceiptThreshold?.read(jsonRequired(object, "threshold")),
      usage: try readPresence(object, "usage", Usage.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "failure", "failure_id", "observations", "question", "question_sources", "request",
      "threshold", "usage",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    object["failure"] = `failure`.json
    object["failure_id"] = `failureId`.json
    object["observations"] = `observations`.json
    object["question"] = `question`.json
    object["question_sources"] = `questionSources`.json
    object["request"] = `request`.json
    object["threshold"] = `threshold`.json
    writePresence(`usage`, "usage", &object)
    return .object(object)
  }
}
public indirect enum AnnotationValue: ValueRepresentable {
  case `decision`(AnnotationValueDecision)
  case `choice`(AnnotationValueChoice)
  case `score`(AnnotationValueScore)
  case `tags`(AnnotationValueTags)
  case `failed`(AnnotationValueFailed)
  static func read(_ json: DataValue) throws -> AnnotationValue {
    let object = try jsonObject(json)
    if object["kind"] == .string("decision") {
      return .`decision`(try AnnotationValueDecision.read(json))
    }
    if object["kind"] == .string("choice") {
      return .`choice`(try AnnotationValueChoice.read(json))
    }
    if object["kind"] == .string("score") { return .`score`(try AnnotationValueScore.read(json)) }
    if object["kind"] == .string("tags") { return .`tags`(try AnnotationValueTags.read(json)) }
    if object["kind"] == .string("failed") {
      return .`failed`(try AnnotationValueFailed.read(json))
    }
    throw ValueConversionError("Unknown AnnotationValue alternative")
  }
  var json: DataValue {
    switch self {
    case .`decision`(let value): return value.json
    case .`choice`(let value): return value.json
    case .`score`(let value): return value.json
    case .`tags`(let value): return value.json
    case .`failed`(let value): return value.json
    }
  }
}
public struct AnnotationValueChoice: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `value`: String?
  public init(extensions: [String: DataValue] = [:], `kind`: String = "choice", `value`: String?) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> AnnotationValueChoice {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try String?.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct AnnotationValueDecision: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `value`: Bool?
  public init(extensions: [String: DataValue] = [:], `kind`: String = "decision", `value`: Bool?) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> AnnotationValueDecision {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try Bool?.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct AnnotationValueFailed: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `value`: Failure
  public init(extensions: [String: DataValue] = [:], `kind`: String = "failed", `value`: Failure) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> AnnotationValueFailed {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try Failure.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct AnnotationValueScore: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `value`: Double
  public init(extensions: [String: DataValue] = [:], `kind`: String = "score", `value`: Double) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> AnnotationValueScore {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try Double.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct AnnotationValueTags: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `value`: [String]
  public init(extensions: [String: DataValue] = [:], `kind`: String = "tags", `value`: [String]) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> AnnotationValueTags {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try [String].read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public typealias AnswerId = String
public struct Answers: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `questions`: [RelationMember]
  public init(extensions: [String: DataValue] = [:], `questions`: [RelationMember]) {
    self.extensions = extensions
    self.`questions` = `questions`
  }
  static func read(_ json: DataValue) throws -> Answers {
    let object = try jsonObject(json)
    let known: Set<String> = ["questions"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      questions: try [RelationMember].read(jsonRequired(object, "questions")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["questions"]
    var object = extensions.filter { !known.contains($0.key) }
    object["questions"] = `questions`.json
    return .object(object)
  }
}
public struct AtomicArrayOfString: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `answer`: Answer
  public let `answerId`: AnswerId
  public let `images`: Presence<[ReceiptImage]>
  public let `index`: Presence<UInt64>
  public let `input`: Presence<DataValue>
  public let `members`: Presence<[RankMember]>
  public let `meta`: Meta
  public let `question`: QuestionDocument
  public let `questionName`: Presence<String>
  public let `schema`: ReceiptVersion
  public let `source`: Presence<PhysicalSource>
  public let `threshold`: ReceiptThreshold?
  public let `value`: [String]
  public init(
    extensions: [String: DataValue] = [:], `answer`: Answer, `answerId`: AnswerId,
    `images`: Presence<[ReceiptImage]> = .absent, `index`: Presence<UInt64> = .absent,
    `input`: Presence<DataValue> = .absent, `members`: Presence<[RankMember]> = .absent,
    `meta`: Meta, `question`: QuestionDocument, `questionName`: Presence<String> = .absent,
    `schema`: ReceiptVersion, `source`: Presence<PhysicalSource> = .absent,
    `threshold`: ReceiptThreshold?, `value`: [String]
  ) {
    self.extensions = extensions
    self.`answer` = `answer`
    self.`answerId` = `answerId`
    self.`images` = `images`
    self.`index` = `index`
    self.`input` = `input`
    self.`members` = `members`
    self.`meta` = `meta`
    self.`question` = `question`
    self.`questionName` = `questionName`
    self.`schema` = `schema`
    self.`source` = `source`
    self.`threshold` = `threshold`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> AtomicArrayOfString {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "answer", "answer_id", "images", "index", "input", "members", "meta", "question",
      "question_name", "schema", "source", "threshold", "value",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      answer: try Answer.read(jsonRequired(object, "answer")),
      answerId: try AnswerId.read(jsonRequired(object, "answer_id")),
      images: try readPresence(object, "images", [ReceiptImage].self),
      index: try readPresence(object, "index", UInt64.self),
      input: try readPresence(object, "input", DataValue.self),
      members: try readPresence(object, "members", [RankMember].self),
      meta: try Meta.read(jsonRequired(object, "meta")),
      question: try QuestionDocument.read(jsonRequired(object, "question")),
      questionName: try readPresence(object, "question_name", String.self),
      schema: try ReceiptVersion.read(jsonRequired(object, "schema")),
      source: try readPresence(object, "source", PhysicalSource.self),
      threshold: try ReceiptThreshold?.read(jsonRequired(object, "threshold")),
      value: try [String].read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "answer", "answer_id", "images", "index", "input", "members", "meta", "question",
      "question_name", "schema", "source", "threshold", "value",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    object["answer"] = `answer`.json
    object["answer_id"] = `answerId`.json
    writePresence(`images`, "images", &object)
    writePresence(`index`, "index", &object)
    writePresence(`input`, "input", &object)
    writePresence(`members`, "members", &object)
    object["meta"] = `meta`.json
    object["question"] = `question`.json
    writePresence(`questionName`, "question_name", &object)
    object["schema"] = `schema`.json
    writePresence(`source`, "source", &object)
    object["threshold"] = `threshold`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct AtomicDecideValue: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `answer`: Answer
  public let `answerId`: AnswerId
  public let `images`: Presence<[ReceiptImage]>
  public let `index`: Presence<UInt64>
  public let `input`: Presence<DataValue>
  public let `members`: Presence<[RankMember]>
  public let `meta`: Meta
  public let `question`: QuestionDocument
  public let `questionName`: Presence<String>
  public let `schema`: ReceiptVersion
  public let `source`: Presence<PhysicalSource>
  public let `threshold`: ReceiptThreshold?
  public let `value`: DecideValue
  public init(
    extensions: [String: DataValue] = [:], `answer`: Answer, `answerId`: AnswerId,
    `images`: Presence<[ReceiptImage]> = .absent, `index`: Presence<UInt64> = .absent,
    `input`: Presence<DataValue> = .absent, `members`: Presence<[RankMember]> = .absent,
    `meta`: Meta, `question`: QuestionDocument, `questionName`: Presence<String> = .absent,
    `schema`: ReceiptVersion, `source`: Presence<PhysicalSource> = .absent,
    `threshold`: ReceiptThreshold?, `value`: DecideValue
  ) {
    self.extensions = extensions
    self.`answer` = `answer`
    self.`answerId` = `answerId`
    self.`images` = `images`
    self.`index` = `index`
    self.`input` = `input`
    self.`members` = `members`
    self.`meta` = `meta`
    self.`question` = `question`
    self.`questionName` = `questionName`
    self.`schema` = `schema`
    self.`source` = `source`
    self.`threshold` = `threshold`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> AtomicDecideValue {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "answer", "answer_id", "images", "index", "input", "members", "meta", "question",
      "question_name", "schema", "source", "threshold", "value",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      answer: try Answer.read(jsonRequired(object, "answer")),
      answerId: try AnswerId.read(jsonRequired(object, "answer_id")),
      images: try readPresence(object, "images", [ReceiptImage].self),
      index: try readPresence(object, "index", UInt64.self),
      input: try readPresence(object, "input", DataValue.self),
      members: try readPresence(object, "members", [RankMember].self),
      meta: try Meta.read(jsonRequired(object, "meta")),
      question: try QuestionDocument.read(jsonRequired(object, "question")),
      questionName: try readPresence(object, "question_name", String.self),
      schema: try ReceiptVersion.read(jsonRequired(object, "schema")),
      source: try readPresence(object, "source", PhysicalSource.self),
      threshold: try ReceiptThreshold?.read(jsonRequired(object, "threshold")),
      value: try DecideValue.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "answer", "answer_id", "images", "index", "input", "members", "meta", "question",
      "question_name", "schema", "source", "threshold", "value",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    object["answer"] = `answer`.json
    object["answer_id"] = `answerId`.json
    writePresence(`images`, "images", &object)
    writePresence(`index`, "index", &object)
    writePresence(`input`, "input", &object)
    writePresence(`members`, "members", &object)
    object["meta"] = `meta`.json
    object["question"] = `question`.json
    writePresence(`questionName`, "question_name", &object)
    object["schema"] = `schema`.json
    writePresence(`source`, "source", &object)
    object["threshold"] = `threshold`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct AtomicNonZeroUsize: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `answer`: Answer
  public let `answerId`: AnswerId
  public let `images`: Presence<[ReceiptImage]>
  public let `index`: Presence<UInt64>
  public let `input`: Presence<DataValue>
  public let `members`: Presence<[RankMember]>
  public let `meta`: Meta
  public let `question`: QuestionDocument
  public let `questionName`: Presence<String>
  public let `schema`: ReceiptVersion
  public let `source`: Presence<PhysicalSource>
  public let `threshold`: ReceiptThreshold?
  public let `value`: UInt64
  public init(
    extensions: [String: DataValue] = [:], `answer`: Answer, `answerId`: AnswerId,
    `images`: Presence<[ReceiptImage]> = .absent, `index`: Presence<UInt64> = .absent,
    `input`: Presence<DataValue> = .absent, `members`: Presence<[RankMember]> = .absent,
    `meta`: Meta, `question`: QuestionDocument, `questionName`: Presence<String> = .absent,
    `schema`: ReceiptVersion, `source`: Presence<PhysicalSource> = .absent,
    `threshold`: ReceiptThreshold?, `value`: UInt64
  ) {
    self.extensions = extensions
    self.`answer` = `answer`
    self.`answerId` = `answerId`
    self.`images` = `images`
    self.`index` = `index`
    self.`input` = `input`
    self.`members` = `members`
    self.`meta` = `meta`
    self.`question` = `question`
    self.`questionName` = `questionName`
    self.`schema` = `schema`
    self.`source` = `source`
    self.`threshold` = `threshold`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> AtomicNonZeroUsize {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "answer", "answer_id", "images", "index", "input", "members", "meta", "question",
      "question_name", "schema", "source", "threshold", "value",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      answer: try Answer.read(jsonRequired(object, "answer")),
      answerId: try AnswerId.read(jsonRequired(object, "answer_id")),
      images: try readPresence(object, "images", [ReceiptImage].self),
      index: try readPresence(object, "index", UInt64.self),
      input: try readPresence(object, "input", DataValue.self),
      members: try readPresence(object, "members", [RankMember].self),
      meta: try Meta.read(jsonRequired(object, "meta")),
      question: try QuestionDocument.read(jsonRequired(object, "question")),
      questionName: try readPresence(object, "question_name", String.self),
      schema: try ReceiptVersion.read(jsonRequired(object, "schema")),
      source: try readPresence(object, "source", PhysicalSource.self),
      threshold: try ReceiptThreshold?.read(jsonRequired(object, "threshold")),
      value: try UInt64.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "answer", "answer_id", "images", "index", "input", "members", "meta", "question",
      "question_name", "schema", "source", "threshold", "value",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    object["answer"] = `answer`.json
    object["answer_id"] = `answerId`.json
    writePresence(`images`, "images", &object)
    writePresence(`index`, "index", &object)
    writePresence(`input`, "input", &object)
    writePresence(`members`, "members", &object)
    object["meta"] = `meta`.json
    object["question"] = `question`.json
    writePresence(`questionName`, "question_name", &object)
    object["schema"] = `schema`.json
    writePresence(`source`, "source", &object)
    object["threshold"] = `threshold`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct AtomicNullableString: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `answer`: Answer
  public let `answerId`: AnswerId
  public let `images`: Presence<[ReceiptImage]>
  public let `index`: Presence<UInt64>
  public let `input`: Presence<DataValue>
  public let `members`: Presence<[RankMember]>
  public let `meta`: Meta
  public let `question`: QuestionDocument
  public let `questionName`: Presence<String>
  public let `schema`: ReceiptVersion
  public let `source`: Presence<PhysicalSource>
  public let `threshold`: ReceiptThreshold?
  public let `value`: String?
  public init(
    extensions: [String: DataValue] = [:], `answer`: Answer, `answerId`: AnswerId,
    `images`: Presence<[ReceiptImage]> = .absent, `index`: Presence<UInt64> = .absent,
    `input`: Presence<DataValue> = .absent, `members`: Presence<[RankMember]> = .absent,
    `meta`: Meta, `question`: QuestionDocument, `questionName`: Presence<String> = .absent,
    `schema`: ReceiptVersion, `source`: Presence<PhysicalSource> = .absent,
    `threshold`: ReceiptThreshold?, `value`: String?
  ) {
    self.extensions = extensions
    self.`answer` = `answer`
    self.`answerId` = `answerId`
    self.`images` = `images`
    self.`index` = `index`
    self.`input` = `input`
    self.`members` = `members`
    self.`meta` = `meta`
    self.`question` = `question`
    self.`questionName` = `questionName`
    self.`schema` = `schema`
    self.`source` = `source`
    self.`threshold` = `threshold`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> AtomicNullableString {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "answer", "answer_id", "images", "index", "input", "members", "meta", "question",
      "question_name", "schema", "source", "threshold", "value",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      answer: try Answer.read(jsonRequired(object, "answer")),
      answerId: try AnswerId.read(jsonRequired(object, "answer_id")),
      images: try readPresence(object, "images", [ReceiptImage].self),
      index: try readPresence(object, "index", UInt64.self),
      input: try readPresence(object, "input", DataValue.self),
      members: try readPresence(object, "members", [RankMember].self),
      meta: try Meta.read(jsonRequired(object, "meta")),
      question: try QuestionDocument.read(jsonRequired(object, "question")),
      questionName: try readPresence(object, "question_name", String.self),
      schema: try ReceiptVersion.read(jsonRequired(object, "schema")),
      source: try readPresence(object, "source", PhysicalSource.self),
      threshold: try ReceiptThreshold?.read(jsonRequired(object, "threshold")),
      value: try String?.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "answer", "answer_id", "images", "index", "input", "members", "meta", "question",
      "question_name", "schema", "source", "threshold", "value",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    object["answer"] = `answer`.json
    object["answer_id"] = `answerId`.json
    writePresence(`images`, "images", &object)
    writePresence(`index`, "index", &object)
    writePresence(`input`, "input", &object)
    writePresence(`members`, "members", &object)
    object["meta"] = `meta`.json
    object["question"] = `question`.json
    writePresence(`questionName`, "question_name", &object)
    object["schema"] = `schema`.json
    writePresence(`source`, "source", &object)
    object["threshold"] = `threshold`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct AtomicBoolean: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `answer`: Answer
  public let `answerId`: AnswerId
  public let `images`: Presence<[ReceiptImage]>
  public let `index`: Presence<UInt64>
  public let `input`: Presence<DataValue>
  public let `members`: Presence<[RankMember]>
  public let `meta`: Meta
  public let `question`: QuestionDocument
  public let `questionName`: Presence<String>
  public let `schema`: ReceiptVersion
  public let `source`: Presence<PhysicalSource>
  public let `threshold`: ReceiptThreshold?
  public let `value`: Bool
  public init(
    extensions: [String: DataValue] = [:], `answer`: Answer, `answerId`: AnswerId,
    `images`: Presence<[ReceiptImage]> = .absent, `index`: Presence<UInt64> = .absent,
    `input`: Presence<DataValue> = .absent, `members`: Presence<[RankMember]> = .absent,
    `meta`: Meta, `question`: QuestionDocument, `questionName`: Presence<String> = .absent,
    `schema`: ReceiptVersion, `source`: Presence<PhysicalSource> = .absent,
    `threshold`: ReceiptThreshold?, `value`: Bool
  ) {
    self.extensions = extensions
    self.`answer` = `answer`
    self.`answerId` = `answerId`
    self.`images` = `images`
    self.`index` = `index`
    self.`input` = `input`
    self.`members` = `members`
    self.`meta` = `meta`
    self.`question` = `question`
    self.`questionName` = `questionName`
    self.`schema` = `schema`
    self.`source` = `source`
    self.`threshold` = `threshold`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> AtomicBoolean {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "answer", "answer_id", "images", "index", "input", "members", "meta", "question",
      "question_name", "schema", "source", "threshold", "value",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      answer: try Answer.read(jsonRequired(object, "answer")),
      answerId: try AnswerId.read(jsonRequired(object, "answer_id")),
      images: try readPresence(object, "images", [ReceiptImage].self),
      index: try readPresence(object, "index", UInt64.self),
      input: try readPresence(object, "input", DataValue.self),
      members: try readPresence(object, "members", [RankMember].self),
      meta: try Meta.read(jsonRequired(object, "meta")),
      question: try QuestionDocument.read(jsonRequired(object, "question")),
      questionName: try readPresence(object, "question_name", String.self),
      schema: try ReceiptVersion.read(jsonRequired(object, "schema")),
      source: try readPresence(object, "source", PhysicalSource.self),
      threshold: try ReceiptThreshold?.read(jsonRequired(object, "threshold")),
      value: try Bool.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "answer", "answer_id", "images", "index", "input", "members", "meta", "question",
      "question_name", "schema", "source", "threshold", "value",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    object["answer"] = `answer`.json
    object["answer_id"] = `answerId`.json
    writePresence(`images`, "images", &object)
    writePresence(`index`, "index", &object)
    writePresence(`input`, "input", &object)
    writePresence(`members`, "members", &object)
    object["meta"] = `meta`.json
    object["question"] = `question`.json
    writePresence(`questionName`, "question_name", &object)
    object["schema"] = `schema`.json
    writePresence(`source`, "source", &object)
    object["threshold"] = `threshold`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct AtomicDouble: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `answer`: Answer
  public let `answerId`: AnswerId
  public let `images`: Presence<[ReceiptImage]>
  public let `index`: Presence<UInt64>
  public let `input`: Presence<DataValue>
  public let `members`: Presence<[RankMember]>
  public let `meta`: Meta
  public let `question`: QuestionDocument
  public let `questionName`: Presence<String>
  public let `schema`: ReceiptVersion
  public let `source`: Presence<PhysicalSource>
  public let `threshold`: ReceiptThreshold?
  public let `value`: Double
  public init(
    extensions: [String: DataValue] = [:], `answer`: Answer, `answerId`: AnswerId,
    `images`: Presence<[ReceiptImage]> = .absent, `index`: Presence<UInt64> = .absent,
    `input`: Presence<DataValue> = .absent, `members`: Presence<[RankMember]> = .absent,
    `meta`: Meta, `question`: QuestionDocument, `questionName`: Presence<String> = .absent,
    `schema`: ReceiptVersion, `source`: Presence<PhysicalSource> = .absent,
    `threshold`: ReceiptThreshold?, `value`: Double
  ) {
    self.extensions = extensions
    self.`answer` = `answer`
    self.`answerId` = `answerId`
    self.`images` = `images`
    self.`index` = `index`
    self.`input` = `input`
    self.`members` = `members`
    self.`meta` = `meta`
    self.`question` = `question`
    self.`questionName` = `questionName`
    self.`schema` = `schema`
    self.`source` = `source`
    self.`threshold` = `threshold`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> AtomicDouble {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "answer", "answer_id", "images", "index", "input", "members", "meta", "question",
      "question_name", "schema", "source", "threshold", "value",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      answer: try Answer.read(jsonRequired(object, "answer")),
      answerId: try AnswerId.read(jsonRequired(object, "answer_id")),
      images: try readPresence(object, "images", [ReceiptImage].self),
      index: try readPresence(object, "index", UInt64.self),
      input: try readPresence(object, "input", DataValue.self),
      members: try readPresence(object, "members", [RankMember].self),
      meta: try Meta.read(jsonRequired(object, "meta")),
      question: try QuestionDocument.read(jsonRequired(object, "question")),
      questionName: try readPresence(object, "question_name", String.self),
      schema: try ReceiptVersion.read(jsonRequired(object, "schema")),
      source: try readPresence(object, "source", PhysicalSource.self),
      threshold: try ReceiptThreshold?.read(jsonRequired(object, "threshold")),
      value: try Double.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "answer", "answer_id", "images", "index", "input", "members", "meta", "question",
      "question_name", "schema", "source", "threshold", "value",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    object["answer"] = `answer`.json
    object["answer_id"] = `answerId`.json
    writePresence(`images`, "images", &object)
    writePresence(`index`, "index", &object)
    writePresence(`input`, "input", &object)
    writePresence(`members`, "members", &object)
    object["meta"] = `meta`.json
    object["question"] = `question`.json
    writePresence(`questionName`, "question_name", &object)
    object["schema"] = `schema`.json
    writePresence(`source`, "source", &object)
    object["threshold"] = `threshold`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct Attempt: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `ordinal`: UInt64
  public let `outcome`: AttemptOutcome
  public let `requestId`: Presence<String>
  public let `requestSha256`: String
  public let `sdkRequestId`: SdkRequestId
  public let `serverMs`: Presence<UInt64>
  public let `status`: Presence<UInt64>
  public let `wallMs`: UInt64
  public init(
    extensions: [String: DataValue] = [:], `ordinal`: UInt64, `outcome`: AttemptOutcome,
    `requestId`: Presence<String> = .absent, `requestSha256`: String, `sdkRequestId`: SdkRequestId,
    `serverMs`: Presence<UInt64> = .absent, `status`: Presence<UInt64> = .absent, `wallMs`: UInt64
  ) {
    self.extensions = extensions
    self.`ordinal` = `ordinal`
    self.`outcome` = `outcome`
    self.`requestId` = `requestId`
    self.`requestSha256` = `requestSha256`
    self.`sdkRequestId` = `sdkRequestId`
    self.`serverMs` = `serverMs`
    self.`status` = `status`
    self.`wallMs` = `wallMs`
  }
  static func read(_ json: DataValue) throws -> Attempt {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "ordinal", "outcome", "request_id", "request_sha256", "sdk_request_id", "server_ms", "status",
      "wall_ms",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      ordinal: try UInt64.read(jsonRequired(object, "ordinal")),
      outcome: try AttemptOutcome.read(jsonRequired(object, "outcome")),
      requestId: try readPresence(object, "request_id", String.self),
      requestSha256: try String.read(jsonRequired(object, "request_sha256")),
      sdkRequestId: try SdkRequestId.read(jsonRequired(object, "sdk_request_id")),
      serverMs: try readPresence(object, "server_ms", UInt64.self),
      status: try readPresence(object, "status", UInt64.self),
      wallMs: try UInt64.read(jsonRequired(object, "wall_ms")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "ordinal", "outcome", "request_id", "request_sha256", "sdk_request_id", "server_ms", "status",
      "wall_ms",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    object["ordinal"] = `ordinal`.json
    object["outcome"] = `outcome`.json
    writePresence(`requestId`, "request_id", &object)
    object["request_sha256"] = `requestSha256`.json
    object["sdk_request_id"] = `sdkRequestId`.json
    writePresence(`serverMs`, "server_ms", &object)
    writePresence(`status`, "status", &object)
    object["wall_ms"] = `wallMs`.json
    return .object(object)
  }
}
public indirect enum ReceiptBatch: ValueRepresentable {
  case `integer`(UInt64)
  case `string`(String)
  static func read(_ json: DataValue) throws -> ReceiptBatch {
    if let value = try? UInt64.read(json) { return .`integer`(value) }
    if let value = try? String.read(json) { return .`string`(value) }
    throw ValueConversionError("Unknown ReceiptBatch value")
  }
  var json: DataValue {
    switch self {
    case .`integer`(let value): return value.json
    case .`string`(let value): return value.json
    }
  }
}
public enum BoundaryMode: ValueRepresentable {
  case `boundaryOnly`
  case unknown(String)
  static func read(_ json: DataValue) throws -> BoundaryMode {
    let value = try String.read(json)
    switch value {
    case "boundary_only": return .`boundaryOnly`
    default: return .unknown(value)
    }
  }
  var json: DataValue {
    switch self {
    case .`boundaryOnly`: return .string("boundary_only")
    case .unknown(let value): return .string(value)
    }
  }
}
public struct BoundaryOdds: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `pieces`: [PieceOdds]
  public let `proposals`: [BoundaryProposal]
  public init(
    extensions: [String: DataValue] = [:], `pieces`: [PieceOdds], `proposals`: [BoundaryProposal]
  ) {
    self.extensions = extensions
    self.`pieces` = `pieces`
    self.`proposals` = `proposals`
  }
  static func read(_ json: DataValue) throws -> BoundaryOdds {
    let object = try jsonObject(json)
    let known: Set<String> = ["pieces", "proposals"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      pieces: try [PieceOdds].read(jsonRequired(object, "pieces")),
      proposals: try [BoundaryProposal].read(jsonRequired(object, "proposals")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["pieces", "proposals"]
    var object = extensions.filter { !known.contains($0.key) }
    object["pieces"] = `pieces`.json
    object["proposals"] = `proposals`.json
    return .object(object)
  }
}
public struct BoundaryProposal: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `end`: UInt64
  public let `length`: UInt64
  public let `probability`: Double
  public let `start`: UInt64
  public let `text`: String
  public init(
    extensions: [String: DataValue] = [:], `end`: UInt64, `length`: UInt64, `probability`: Double,
    `start`: UInt64, `text`: String
  ) {
    self.extensions = extensions
    self.`end` = `end`
    self.`length` = `length`
    self.`probability` = `probability`
    self.`start` = `start`
    self.`text` = `text`
  }
  static func read(_ json: DataValue) throws -> BoundaryProposal {
    let object = try jsonObject(json)
    let known: Set<String> = ["end", "length", "probability", "start", "text"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      end: try UInt64.read(jsonRequired(object, "end")),
      length: try UInt64.read(jsonRequired(object, "length")),
      probability: try Double.read(jsonRequired(object, "probability")),
      start: try UInt64.read(jsonRequired(object, "start")),
      text: try String.read(jsonRequired(object, "text")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["end", "length", "probability", "start", "text"]
    var object = extensions.filter { !known.contains($0.key) }
    object["end"] = `end`.json
    object["length"] = `length`.json
    object["probability"] = `probability`.json
    object["start"] = `start`.json
    object["text"] = `text`.json
    return .object(object)
  }
}
public struct CallError: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `error`: FailureDetail
  public let `facts`: Presence<Facts>
  public init(
    extensions: [String: DataValue] = [:], `error`: FailureDetail,
    `facts`: Presence<Facts> = .absent
  ) {
    self.extensions = extensions
    self.`error` = `error`
    self.`facts` = `facts`
  }
  static func read(_ json: DataValue) throws -> CallError {
    let object = try jsonObject(json)
    let known: Set<String> = ["error", "facts"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      error: try FailureDetail.read(jsonRequired(object, "error")),
      facts: try readPresence(object, "facts", Facts.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["error", "facts"]
    var object = extensions.filter { !known.contains($0.key) }
    object["error"] = `error`.json
    writePresence(`facts`, "facts", &object)
    return .object(object)
  }
}
public typealias CallId = String
public indirect enum DecideValue: ValueRepresentable {
  case `booleanOrNull`(Bool?)
  case `jsonValue`(DataValue)
  static func read(_ json: DataValue) throws -> DecideValue {
    if let value = try? Bool?.read(json) { return .`booleanOrNull`(value) }
    if let value = try? DataValue.read(json) { return .`jsonValue`(value) }
    throw ValueConversionError("Unknown DecideValue value")
  }
  var json: DataValue {
    switch self {
    case .`booleanOrNull`(let value): return value.json
    case .`jsonValue`(let value): return value.json
    }
  }
}
public struct EntityDocument: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `name`: String
  public init(extensions: [String: DataValue] = [:], `kind`: String, `name`: String) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`name` = `name`
  }
  static func read(_ json: DataValue) throws -> EntityDocument {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "name"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      name: try String.read(jsonRequired(object, "name")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "name"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["name"] = `name`.json
    return .object(object)
  }
}
public struct FailureDetail: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `estimatedInputDenial`: Presence<EstimatedInputDenial>
  public let `kind`: FailureKind
  public let `message`: String
  public let `retryable`: Bool
  public let `sendBudgetDenial`: Presence<SendBudgetDenial>
  public let `stopped`: Stopped
  public init(
    extensions: [String: DataValue] = [:],
    `estimatedInputDenial`: Presence<EstimatedInputDenial> = .absent, `kind`: FailureKind,
    `message`: String, `retryable`: Bool, `sendBudgetDenial`: Presence<SendBudgetDenial> = .absent,
    `stopped`: Stopped
  ) {
    self.extensions = extensions
    self.`estimatedInputDenial` = `estimatedInputDenial`
    self.`kind` = `kind`
    self.`message` = `message`
    self.`retryable` = `retryable`
    self.`sendBudgetDenial` = `sendBudgetDenial`
    self.`stopped` = `stopped`
  }
  static func read(_ json: DataValue) throws -> FailureDetail {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "estimated_input_denial", "kind", "message", "retryable", "send_budget_denial", "stopped",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      estimatedInputDenial: try readPresence(
        object, "estimated_input_denial", EstimatedInputDenial.self),
      kind: try FailureKind.read(jsonRequired(object, "kind")),
      message: try String.read(jsonRequired(object, "message")),
      retryable: try Bool.read(jsonRequired(object, "retryable")),
      sendBudgetDenial: try readPresence(object, "send_budget_denial", SendBudgetDenial.self),
      stopped: try Stopped.read(jsonRequired(object, "stopped")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "estimated_input_denial", "kind", "message", "retryable", "send_budget_denial", "stopped",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`estimatedInputDenial`, "estimated_input_denial", &object)
    object["kind"] = `kind`.json
    object["message"] = `message`.json
    object["retryable"] = `retryable`.json
    writePresence(`sendBudgetDenial`, "send_budget_denial", &object)
    object["stopped"] = `stopped`.json
    return .object(object)
  }
}
public indirect enum EstimatedInputDenial: ValueRepresentable {
  case `initialRequest`(EstimatedInputDenialInitialRequest)
  case `additionalRequest`(EstimatedInputDenialAdditionalRequest)
  case `retry`(EstimatedInputDenialRetry)
  static func read(_ json: DataValue) throws -> EstimatedInputDenial {
    let object = try jsonObject(json)
    if object["kind"] == .string("initial_request") {
      return .`initialRequest`(try EstimatedInputDenialInitialRequest.read(json))
    }
    if object["kind"] == .string("additional_request") {
      return .`additionalRequest`(try EstimatedInputDenialAdditionalRequest.read(json))
    }
    if object["kind"] == .string("retry") {
      return .`retry`(try EstimatedInputDenialRetry.read(json))
    }
    throw ValueConversionError("Unknown EstimatedInputDenial alternative")
  }
  var json: DataValue {
    switch self {
    case .`initialRequest`(let value): return value.json
    case .`additionalRequest`(let value): return value.json
    case .`retry`(let value): return value.json
    }
  }
}
public struct EstimatedInputDenialAdditionalRequest: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `limit`: UInt64
  public init(
    extensions: [String: DataValue] = [:], `kind`: String = "additional_request", `limit`: UInt64
  ) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`limit` = `limit`
  }
  static func read(_ json: DataValue) throws -> EstimatedInputDenialAdditionalRequest {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "limit"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      limit: try UInt64.read(jsonRequired(object, "limit")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "limit"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["limit"] = `limit`.json
    return .object(object)
  }
}
public struct EstimatedInputDenialInitialRequest: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `limit`: UInt64
  public init(
    extensions: [String: DataValue] = [:], `kind`: String = "initial_request", `limit`: UInt64
  ) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`limit` = `limit`
  }
  static func read(_ json: DataValue) throws -> EstimatedInputDenialInitialRequest {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "limit"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      limit: try UInt64.read(jsonRequired(object, "limit")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "limit"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["limit"] = `limit`.json
    return .object(object)
  }
}
public struct EstimatedInputDenialRetry: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `lastStatus`: UInt64
  public let `limit`: UInt64
  public init(
    extensions: [String: DataValue] = [:], `kind`: String = "retry", `lastStatus`: UInt64,
    `limit`: UInt64
  ) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`lastStatus` = `lastStatus`
    self.`limit` = `limit`
  }
  static func read(_ json: DataValue) throws -> EstimatedInputDenialRetry {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "last_status", "limit"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      lastStatus: try UInt64.read(jsonRequired(object, "last_status")),
      limit: try UInt64.read(jsonRequired(object, "limit")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "last_status", "limit"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["last_status"] = `lastStatus`.json
    object["limit"] = `limit`.json
    return .object(object)
  }
}
public struct Facts: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `attempts`: Presence<[Attempt]>
  public let `cacheAnswers`: UInt64
  public let `callId`: CallId
  public let `estimatedCostUsd`: Presence<String>
  public let `heldModelMismatch`: Presence<Bool>
  public let `inputTokens`: Presence<UInt64>
  public let `largestRequestBytes`: UInt64
  public let `largestRequestEstimatedInputTokens`: UInt64?
  public let `model`: Presence<String>
  public let `outputTokens`: Presence<UInt64>
  public let `records`: UInt64
  public let `requestsSent`: UInt64
  public let `seconds`: Double
  public let `tokenEstimateMethod`: String
  public let `usagePersistence`: Presence<PersistenceObservation>
  public init(
    extensions: [String: DataValue] = [:], `attempts`: Presence<[Attempt]> = .absent,
    `cacheAnswers`: UInt64, `callId`: CallId, `estimatedCostUsd`: Presence<String> = .absent,
    `heldModelMismatch`: Presence<Bool> = .absent, `inputTokens`: Presence<UInt64> = .absent,
    `largestRequestBytes`: UInt64, `largestRequestEstimatedInputTokens`: UInt64?,
    `model`: Presence<String> = .absent, `outputTokens`: Presence<UInt64> = .absent,
    `records`: UInt64, `requestsSent`: UInt64, `seconds`: Double, `tokenEstimateMethod`: String,
    `usagePersistence`: Presence<PersistenceObservation> = .absent
  ) {
    self.extensions = extensions
    self.`attempts` = `attempts`
    self.`cacheAnswers` = `cacheAnswers`
    self.`callId` = `callId`
    self.`estimatedCostUsd` = `estimatedCostUsd`
    self.`heldModelMismatch` = `heldModelMismatch`
    self.`inputTokens` = `inputTokens`
    self.`largestRequestBytes` = `largestRequestBytes`
    self.`largestRequestEstimatedInputTokens` = `largestRequestEstimatedInputTokens`
    self.`model` = `model`
    self.`outputTokens` = `outputTokens`
    self.`records` = `records`
    self.`requestsSent` = `requestsSent`
    self.`seconds` = `seconds`
    self.`tokenEstimateMethod` = `tokenEstimateMethod`
    self.`usagePersistence` = `usagePersistence`
  }
  static func read(_ json: DataValue) throws -> Facts {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "attempts", "cache_answers", "call_id", "estimated_cost_usd", "held_model_mismatch",
      "input_tokens", "largest_request_bytes", "largest_request_estimated_input_tokens", "model",
      "output_tokens", "records", "requests_sent", "seconds", "token_estimate_method",
      "usage_persistence",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      attempts: try readPresence(object, "attempts", [Attempt].self),
      cacheAnswers: try UInt64.read(jsonRequired(object, "cache_answers")),
      callId: try CallId.read(jsonRequired(object, "call_id")),
      estimatedCostUsd: try readPresence(object, "estimated_cost_usd", String.self),
      heldModelMismatch: try readPresence(object, "held_model_mismatch", Bool.self),
      inputTokens: try readPresence(object, "input_tokens", UInt64.self),
      largestRequestBytes: try UInt64.read(jsonRequired(object, "largest_request_bytes")),
      largestRequestEstimatedInputTokens: try UInt64?.read(
        jsonRequired(object, "largest_request_estimated_input_tokens")),
      model: try readPresence(object, "model", String.self),
      outputTokens: try readPresence(object, "output_tokens", UInt64.self),
      records: try UInt64.read(jsonRequired(object, "records")),
      requestsSent: try UInt64.read(jsonRequired(object, "requests_sent")),
      seconds: try Double.read(jsonRequired(object, "seconds")),
      tokenEstimateMethod: try String.read(jsonRequired(object, "token_estimate_method")),
      usagePersistence: try readPresence(object, "usage_persistence", PersistenceObservation.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "attempts", "cache_answers", "call_id", "estimated_cost_usd", "held_model_mismatch",
      "input_tokens", "largest_request_bytes", "largest_request_estimated_input_tokens", "model",
      "output_tokens", "records", "requests_sent", "seconds", "token_estimate_method",
      "usage_persistence",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`attempts`, "attempts", &object)
    object["cache_answers"] = `cacheAnswers`.json
    object["call_id"] = `callId`.json
    writePresence(`estimatedCostUsd`, "estimated_cost_usd", &object)
    writePresence(`heldModelMismatch`, "held_model_mismatch", &object)
    writePresence(`inputTokens`, "input_tokens", &object)
    object["largest_request_bytes"] = `largestRequestBytes`.json
    object["largest_request_estimated_input_tokens"] = `largestRequestEstimatedInputTokens`.json
    writePresence(`model`, "model", &object)
    writePresence(`outputTokens`, "output_tokens", &object)
    object["records"] = `records`.json
    object["requests_sent"] = `requestsSent`.json
    object["seconds"] = `seconds`.json
    object["token_estimate_method"] = `tokenEstimateMethod`.json
    writePresence(`usagePersistence`, "usage_persistence", &object)
    return .object(object)
  }
}
public typealias FailureId = String
public struct Find: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `answer`: FindAnswer
  public let `answerId`: AnswerId
  public let `candidates`: Presence<[FindCandidate]>
  public let `file`: Presence<String>
  public let `firstLine`: Presence<UInt64>
  public let `index`: UInt64?
  public let `lastLine`: Presence<UInt64>
  public let `meta`: Meta
  public let `position`: Presence<Position>
  public let `question`: FindQuestionDocument
  public let `schema`: ReceiptVersion
  public let `threshold`: DataValue
  public let `value`: DataValue
  public init(
    extensions: [String: DataValue] = [:], `answer`: FindAnswer, `answerId`: AnswerId,
    `candidates`: Presence<[FindCandidate]> = .absent, `file`: Presence<String> = .absent,
    `firstLine`: Presence<UInt64> = .absent, `index`: UInt64?,
    `lastLine`: Presence<UInt64> = .absent, `meta`: Meta, `position`: Presence<Position> = .absent,
    `question`: FindQuestionDocument, `schema`: ReceiptVersion, `threshold`: DataValue,
    `value`: DataValue
  ) {
    self.extensions = extensions
    self.`answer` = `answer`
    self.`answerId` = `answerId`
    self.`candidates` = `candidates`
    self.`file` = `file`
    self.`firstLine` = `firstLine`
    self.`index` = `index`
    self.`lastLine` = `lastLine`
    self.`meta` = `meta`
    self.`position` = `position`
    self.`question` = `question`
    self.`schema` = `schema`
    self.`threshold` = `threshold`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> Find {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "answer", "answer_id", "candidates", "file", "first_line", "index", "last_line", "meta",
      "position", "question", "schema", "threshold", "value",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      answer: try FindAnswer.read(jsonRequired(object, "answer")),
      answerId: try AnswerId.read(jsonRequired(object, "answer_id")),
      candidates: try readPresence(object, "candidates", [FindCandidate].self),
      file: try readPresence(object, "file", String.self),
      firstLine: try readPresence(object, "first_line", UInt64.self),
      index: try UInt64?.read(jsonRequired(object, "index")),
      lastLine: try readPresence(object, "last_line", UInt64.self),
      meta: try Meta.read(jsonRequired(object, "meta")),
      position: try readPresence(object, "position", Position.self),
      question: try FindQuestionDocument.read(jsonRequired(object, "question")),
      schema: try ReceiptVersion.read(jsonRequired(object, "schema")),
      threshold: try DataValue.read(jsonRequired(object, "threshold")),
      value: try DataValue.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "answer", "answer_id", "candidates", "file", "first_line", "index", "last_line", "meta",
      "position", "question", "schema", "threshold", "value",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    object["answer"] = `answer`.json
    object["answer_id"] = `answerId`.json
    writePresence(`candidates`, "candidates", &object)
    writePresence(`file`, "file", &object)
    writePresence(`firstLine`, "first_line", &object)
    object["index"] = `index`.json
    writePresence(`lastLine`, "last_line", &object)
    object["meta"] = `meta`.json
    writePresence(`position`, "position", &object)
    object["question"] = `question`.json
    object["schema"] = `schema`.json
    object["threshold"] = `threshold`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct FindCandidate: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `index`: UInt64?
  public let `input`: DataValue
  public let `probability`: Double
  public let `source`: Presence<PhysicalSource>
  public init(
    extensions: [String: DataValue] = [:], `index`: UInt64?, `input`: DataValue,
    `probability`: Double, `source`: Presence<PhysicalSource> = .absent
  ) {
    self.extensions = extensions
    self.`index` = `index`
    self.`input` = `input`
    self.`probability` = `probability`
    self.`source` = `source`
  }
  static func read(_ json: DataValue) throws -> FindCandidate {
    let object = try jsonObject(json)
    let known: Set<String> = ["index", "input", "probability", "source"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      index: try UInt64?.read(jsonRequired(object, "index")),
      input: try DataValue.read(jsonRequired(object, "input")),
      probability: try Double.read(jsonRequired(object, "probability")),
      source: try readPresence(object, "source", PhysicalSource.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["index", "input", "probability", "source"]
    var object = extensions.filter { !known.contains($0.key) }
    object["index"] = `index`.json
    object["input"] = `input`.json
    object["probability"] = `probability`.json
    writePresence(`source`, "source", &object)
    return .object(object)
  }
}
public struct ReceiptImage: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `base64`: String
  public let `height`: UInt64
  public let `media`: ReceiptImageMedia
  public let `width`: UInt64
  public init(
    extensions: [String: DataValue] = [:], `base64`: String, `height`: UInt64,
    `media`: ReceiptImageMedia, `width`: UInt64
  ) {
    self.extensions = extensions
    self.`base64` = `base64`
    self.`height` = `height`
    self.`media` = `media`
    self.`width` = `width`
  }
  static func read(_ json: DataValue) throws -> ReceiptImage {
    let object = try jsonObject(json)
    let known: Set<String> = ["base64", "height", "media", "width"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      base64: try String.read(jsonRequired(object, "base64")),
      height: try UInt64.read(jsonRequired(object, "height")),
      media: try ReceiptImageMedia.read(jsonRequired(object, "media")),
      width: try UInt64.read(jsonRequired(object, "width")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["base64", "height", "media", "width"]
    var object = extensions.filter { !known.contains($0.key) }
    object["base64"] = `base64`.json
    object["height"] = `height`.json
    object["media"] = `media`.json
    object["width"] = `width`.json
    return .object(object)
  }
}
public enum ReceiptImageMedia: ValueRepresentable {
  case `imageJpeg`
  case `imagePng`
  case unknown(String)
  static func read(_ json: DataValue) throws -> ReceiptImageMedia {
    let value = try String.read(json)
    switch value {
    case "image/jpeg": return .`imageJpeg`
    case "image/png": return .`imagePng`
    default: return .unknown(value)
    }
  }
  var json: DataValue {
    switch self {
    case .`imageJpeg`: return .string("image/jpeg")
    case .`imagePng`: return .string("image/png")
    case .unknown(let value): return .string(value)
    }
  }
}
public struct ImmediateFacts: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `attempts`: Presence<[Attempt]>
  public let `cacheAnswers`: UInt64
  public let `callId`: Presence<CallId>
  public let `estimatedCostUsd`: Presence<String>
  public let `heldModelMismatch`: Presence<Bool>
  public let `inputTokens`: Presence<UInt64>
  public let `largestRequestBytes`: UInt64
  public let `largestRequestEstimatedInputTokens`: UInt64?
  public let `model`: Presence<String>
  public let `outputTokens`: Presence<UInt64>
  public let `records`: UInt64
  public let `requestsSent`: UInt64
  public let `seconds`: Double
  public let `tokenEstimateMethod`: String
  public let `usagePersistence`: Presence<PersistenceObservation>
  public init(
    extensions: [String: DataValue] = [:], `attempts`: Presence<[Attempt]> = .absent,
    `cacheAnswers`: UInt64, `callId`: Presence<CallId> = .absent,
    `estimatedCostUsd`: Presence<String> = .absent, `heldModelMismatch`: Presence<Bool> = .absent,
    `inputTokens`: Presence<UInt64> = .absent, `largestRequestBytes`: UInt64,
    `largestRequestEstimatedInputTokens`: UInt64?, `model`: Presence<String> = .absent,
    `outputTokens`: Presence<UInt64> = .absent, `records`: UInt64, `requestsSent`: UInt64,
    `seconds`: Double, `tokenEstimateMethod`: String,
    `usagePersistence`: Presence<PersistenceObservation> = .absent
  ) {
    self.extensions = extensions
    self.`attempts` = `attempts`
    self.`cacheAnswers` = `cacheAnswers`
    self.`callId` = `callId`
    self.`estimatedCostUsd` = `estimatedCostUsd`
    self.`heldModelMismatch` = `heldModelMismatch`
    self.`inputTokens` = `inputTokens`
    self.`largestRequestBytes` = `largestRequestBytes`
    self.`largestRequestEstimatedInputTokens` = `largestRequestEstimatedInputTokens`
    self.`model` = `model`
    self.`outputTokens` = `outputTokens`
    self.`records` = `records`
    self.`requestsSent` = `requestsSent`
    self.`seconds` = `seconds`
    self.`tokenEstimateMethod` = `tokenEstimateMethod`
    self.`usagePersistence` = `usagePersistence`
  }
  static func read(_ json: DataValue) throws -> ImmediateFacts {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "attempts", "cache_answers", "call_id", "estimated_cost_usd", "held_model_mismatch",
      "input_tokens", "largest_request_bytes", "largest_request_estimated_input_tokens", "model",
      "output_tokens", "records", "requests_sent", "seconds", "token_estimate_method",
      "usage_persistence",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      attempts: try readPresence(object, "attempts", [Attempt].self),
      cacheAnswers: try UInt64.read(jsonRequired(object, "cache_answers")),
      callId: try readPresence(object, "call_id", CallId.self),
      estimatedCostUsd: try readPresence(object, "estimated_cost_usd", String.self),
      heldModelMismatch: try readPresence(object, "held_model_mismatch", Bool.self),
      inputTokens: try readPresence(object, "input_tokens", UInt64.self),
      largestRequestBytes: try UInt64.read(jsonRequired(object, "largest_request_bytes")),
      largestRequestEstimatedInputTokens: try UInt64?.read(
        jsonRequired(object, "largest_request_estimated_input_tokens")),
      model: try readPresence(object, "model", String.self),
      outputTokens: try readPresence(object, "output_tokens", UInt64.self),
      records: try UInt64.read(jsonRequired(object, "records")),
      requestsSent: try UInt64.read(jsonRequired(object, "requests_sent")),
      seconds: try Double.read(jsonRequired(object, "seconds")),
      tokenEstimateMethod: try String.read(jsonRequired(object, "token_estimate_method")),
      usagePersistence: try readPresence(object, "usage_persistence", PersistenceObservation.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "attempts", "cache_answers", "call_id", "estimated_cost_usd", "held_model_mismatch",
      "input_tokens", "largest_request_bytes", "largest_request_estimated_input_tokens", "model",
      "output_tokens", "records", "requests_sent", "seconds", "token_estimate_method",
      "usage_persistence",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`attempts`, "attempts", &object)
    object["cache_answers"] = `cacheAnswers`.json
    writePresence(`callId`, "call_id", &object)
    writePresence(`estimatedCostUsd`, "estimated_cost_usd", &object)
    writePresence(`heldModelMismatch`, "held_model_mismatch", &object)
    writePresence(`inputTokens`, "input_tokens", &object)
    object["largest_request_bytes"] = `largestRequestBytes`.json
    object["largest_request_estimated_input_tokens"] = `largestRequestEstimatedInputTokens`.json
    writePresence(`model`, "model", &object)
    writePresence(`outputTokens`, "output_tokens", &object)
    object["records"] = `records`.json
    object["requests_sent"] = `requestsSent`.json
    object["seconds"] = `seconds`.json
    object["token_estimate_method"] = `tokenEstimateMethod`.json
    writePresence(`usagePersistence`, "usage_persistence", &object)
    return .object(object)
  }
}
public indirect enum InputDeclaration: ValueRepresentable {
  case `string`(InputDeclarationStringRoot)
  case `object`(InputDeclarationObjectRoot)
  static func read(_ json: DataValue) throws -> InputDeclaration {
    let object = try jsonObject(json)
    if object["type"] == .string("string") {
      return .`string`(try InputDeclarationStringRoot.read(json))
    }
    if object["type"] == .string("object") {
      return .`object`(try InputDeclarationObjectRoot.read(json))
    }
    throw ValueConversionError("Unknown InputDeclaration alternative")
  }
  var json: DataValue {
    switch self {
    case .`string`(let value): return value.json
    case .`object`(let value): return value.json
    }
  }
}
public struct InputDeclarationObjectRoot: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `properties`: [String: InputPropertyType]
  public let `required`: Presence<[String]>
  public let `type`: ObjectType
  public init(
    extensions: [String: DataValue] = [:], `properties`: [String: InputPropertyType],
    `required`: Presence<[String]> = .absent, `type`: ObjectType
  ) {
    self.extensions = extensions
    self.`properties` = `properties`
    self.`required` = `required`
    self.`type` = `type`
  }
  static func read(_ json: DataValue) throws -> InputDeclarationObjectRoot {
    let object = try jsonObject(json)
    let known: Set<String> = ["properties", "required", "type"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      properties: try [String: InputPropertyType].read(jsonRequired(object, "properties")),
      required: try readPresence(object, "required", [String].self),
      type: try ObjectType.read(jsonRequired(object, "type")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["properties", "required", "type"]
    var object = extensions.filter { !known.contains($0.key) }
    object["properties"] = `properties`.json
    writePresence(`required`, "required", &object)
    object["type"] = `type`.json
    return .object(object)
  }
}
public struct InputDeclarationStringRoot: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `type`: StringType
  public init(extensions: [String: DataValue] = [:], `type`: StringType) {
    self.extensions = extensions
    self.`type` = `type`
  }
  static func read(_ json: DataValue) throws -> InputDeclarationStringRoot {
    let object = try jsonObject(json)
    let known: Set<String> = ["type"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      type: try StringType.read(jsonRequired(object, "type")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["type"]
    var object = extensions.filter { !known.contains($0.key) }
    object["type"] = `type`.json
    return .object(object)
  }
}
public indirect enum InputPropertyType: ValueRepresentable {
  case `string`(InputPropertyTypeString)
  case `number`(InputPropertyTypeNumber)
  case `boolean`(InputPropertyTypeBoolean)
  case `array`(InputPropertyTypeArray)
  static func read(_ json: DataValue) throws -> InputPropertyType {
    let object = try jsonObject(json)
    if object["type"] == .string("string") {
      return .`string`(try InputPropertyTypeString.read(json))
    }
    if object["type"] == .string("number") {
      return .`number`(try InputPropertyTypeNumber.read(json))
    }
    if object["type"] == .string("boolean") {
      return .`boolean`(try InputPropertyTypeBoolean.read(json))
    }
    if object["type"] == .string("array") { return .`array`(try InputPropertyTypeArray.read(json)) }
    throw ValueConversionError("Unknown InputPropertyType alternative")
  }
  var json: DataValue {
    switch self {
    case .`string`(let value): return value.json
    case .`number`(let value): return value.json
    case .`boolean`(let value): return value.json
    case .`array`(let value): return value.json
    }
  }
}
public struct InputPropertyTypeArray: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `items`: StringRoot
  public let `type`: String
  public init(extensions: [String: DataValue] = [:], `items`: StringRoot, `type`: String = "array")
  {
    self.extensions = extensions
    self.`items` = `items`
    self.`type` = `type`
  }
  static func read(_ json: DataValue) throws -> InputPropertyTypeArray {
    let object = try jsonObject(json)
    let known: Set<String> = ["items", "type"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      items: try StringRoot.read(jsonRequired(object, "items")),
      type: try String.read(jsonRequired(object, "type")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["items", "type"]
    var object = extensions.filter { !known.contains($0.key) }
    object["items"] = `items`.json
    object["type"] = `type`.json
    return .object(object)
  }
}
public struct InputPropertyTypeBoolean: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `type`: String
  public init(extensions: [String: DataValue] = [:], `type`: String = "boolean") {
    self.extensions = extensions
    self.`type` = `type`
  }
  static func read(_ json: DataValue) throws -> InputPropertyTypeBoolean {
    let object = try jsonObject(json)
    let known: Set<String> = ["type"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      type: try String.read(jsonRequired(object, "type")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["type"]
    var object = extensions.filter { !known.contains($0.key) }
    object["type"] = `type`.json
    return .object(object)
  }
}
public struct InputPropertyTypeNumber: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `type`: String
  public init(extensions: [String: DataValue] = [:], `type`: String = "number") {
    self.extensions = extensions
    self.`type` = `type`
  }
  static func read(_ json: DataValue) throws -> InputPropertyTypeNumber {
    let object = try jsonObject(json)
    let known: Set<String> = ["type"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      type: try String.read(jsonRequired(object, "type")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["type"]
    var object = extensions.filter { !known.contains($0.key) }
    object["type"] = `type`.json
    return .object(object)
  }
}
public struct InputPropertyTypeString: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `type`: String
  public init(extensions: [String: DataValue] = [:], `type`: String = "string") {
    self.extensions = extensions
    self.`type` = `type`
  }
  static func read(_ json: DataValue) throws -> InputPropertyTypeString {
    let object = try jsonObject(json)
    let known: Set<String> = ["type"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      type: try String.read(jsonRequired(object, "type")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["type"]
    var object = extensions.filter { !known.contains($0.key) }
    object["type"] = `type`.json
    return .object(object)
  }
}
public struct Label: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `description`: Presence<DataValue>
  public let `name`: String
  public init(
    extensions: [String: DataValue] = [:], `description`: Presence<DataValue> = .absent,
    `name`: String
  ) {
    self.extensions = extensions
    self.`description` = `description`
    self.`name` = `name`
  }
  static func read(_ json: DataValue) throws -> Label {
    let object = try jsonObject(json)
    let known: Set<String> = ["description", "name"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      description: try readPresence(object, "description", DataValue.self),
      name: try String.read(jsonRequired(object, "name")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["description", "name"]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`description`, "description", &object)
    object["name"] = `name`.json
    return .object(object)
  }
}
public struct Meta: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `answeredBy`: Presence<String>
  public let `attempts`: Presence<[Attempt]>
  public let `batchSetting`: Presence<BatchSetting>
  public let `batchWarning`: Presence<BatchWarning>
  public let `cached`: Bool
  public let `contextSha256`: Presence<String>
  public let `failedQuestions`: UInt64
  public let `model`: String
  public let `observations`: [Observation]
  public let `origin`: Origin?
  public let `profileWarning`: Presence<ProfileWarning>
  public let `questionSha256`: Presence<String>
  public let `questionSources`: [QuestionSource]
  public let `questionsSha256`: Presence<String>
  public let `requests`: [String]
  public let `requestsSent`: UInt64
  public let `tool`: String
  public let `url`: String
  public let `usage`: Presence<Usage>
  public init(
    extensions: [String: DataValue] = [:], `answeredBy`: Presence<String> = .absent,
    `attempts`: Presence<[Attempt]> = .absent, `batchSetting`: Presence<BatchSetting> = .absent,
    `batchWarning`: Presence<BatchWarning> = .absent, `cached`: Bool,
    `contextSha256`: Presence<String> = .absent, `failedQuestions`: UInt64, `model`: String,
    `observations`: [Observation], `origin`: Origin?,
    `profileWarning`: Presence<ProfileWarning> = .absent,
    `questionSha256`: Presence<String> = .absent, `questionSources`: [QuestionSource],
    `questionsSha256`: Presence<String> = .absent, `requests`: [String], `requestsSent`: UInt64,
    `tool`: String, `url`: String, `usage`: Presence<Usage> = .absent
  ) {
    self.extensions = extensions
    self.`answeredBy` = `answeredBy`
    self.`attempts` = `attempts`
    self.`batchSetting` = `batchSetting`
    self.`batchWarning` = `batchWarning`
    self.`cached` = `cached`
    self.`contextSha256` = `contextSha256`
    self.`failedQuestions` = `failedQuestions`
    self.`model` = `model`
    self.`observations` = `observations`
    self.`origin` = `origin`
    self.`profileWarning` = `profileWarning`
    self.`questionSha256` = `questionSha256`
    self.`questionSources` = `questionSources`
    self.`questionsSha256` = `questionsSha256`
    self.`requests` = `requests`
    self.`requestsSent` = `requestsSent`
    self.`tool` = `tool`
    self.`url` = `url`
    self.`usage` = `usage`
  }
  static func read(_ json: DataValue) throws -> Meta {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "answered_by", "attempts", "batch_setting", "batch_warning", "cached", "context_sha256",
      "failed_questions", "model", "observations", "origin", "profile_warning", "question_sha256",
      "question_sources", "questions_sha256", "requests", "requests_sent", "tool", "url", "usage",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      answeredBy: try readPresence(object, "answered_by", String.self),
      attempts: try readPresence(object, "attempts", [Attempt].self),
      batchSetting: try readPresence(object, "batch_setting", BatchSetting.self),
      batchWarning: try readPresence(object, "batch_warning", BatchWarning.self),
      cached: try Bool.read(jsonRequired(object, "cached")),
      contextSha256: try readPresence(object, "context_sha256", String.self),
      failedQuestions: try UInt64.read(jsonRequired(object, "failed_questions")),
      model: try String.read(jsonRequired(object, "model")),
      observations: try [Observation].read(jsonRequired(object, "observations")),
      origin: try Origin?.read(jsonRequired(object, "origin")),
      profileWarning: try readPresence(object, "profile_warning", ProfileWarning.self),
      questionSha256: try readPresence(object, "question_sha256", String.self),
      questionSources: try [QuestionSource].read(jsonRequired(object, "question_sources")),
      questionsSha256: try readPresence(object, "questions_sha256", String.self),
      requests: try [String].read(jsonRequired(object, "requests")),
      requestsSent: try UInt64.read(jsonRequired(object, "requests_sent")),
      tool: try String.read(jsonRequired(object, "tool")),
      url: try String.read(jsonRequired(object, "url")),
      usage: try readPresence(object, "usage", Usage.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "answered_by", "attempts", "batch_setting", "batch_warning", "cached", "context_sha256",
      "failed_questions", "model", "observations", "origin", "profile_warning", "question_sha256",
      "question_sources", "questions_sha256", "requests", "requests_sent", "tool", "url", "usage",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`answeredBy`, "answered_by", &object)
    writePresence(`attempts`, "attempts", &object)
    writePresence(`batchSetting`, "batch_setting", &object)
    writePresence(`batchWarning`, "batch_warning", &object)
    object["cached"] = `cached`.json
    writePresence(`contextSha256`, "context_sha256", &object)
    object["failed_questions"] = `failedQuestions`.json
    object["model"] = `model`.json
    object["observations"] = `observations`.json
    object["origin"] = `origin`.json
    writePresence(`profileWarning`, "profile_warning", &object)
    writePresence(`questionSha256`, "question_sha256", &object)
    object["question_sources"] = `questionSources`.json
    writePresence(`questionsSha256`, "questions_sha256", &object)
    object["requests"] = `requests`.json
    object["requests_sent"] = `requestsSent`.json
    object["tool"] = `tool`.json
    object["url"] = `url`.json
    writePresence(`usage`, "usage", &object)
    return .object(object)
  }
}
public struct ObjectRoot: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `properties`: [String: InputPropertyType]
  public let `required`: Presence<[String]>
  public let `type`: ObjectType
  public init(
    extensions: [String: DataValue] = [:], `properties`: [String: InputPropertyType],
    `required`: Presence<[String]> = .absent, `type`: ObjectType
  ) {
    self.extensions = extensions
    self.`properties` = `properties`
    self.`required` = `required`
    self.`type` = `type`
  }
  static func read(_ json: DataValue) throws -> ObjectRoot {
    let object = try jsonObject(json)
    let known: Set<String> = ["properties", "required", "type"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      properties: try [String: InputPropertyType].read(jsonRequired(object, "properties")),
      required: try readPresence(object, "required", [String].self),
      type: try ObjectType.read(jsonRequired(object, "type")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["properties", "required", "type"]
    var object = extensions.filter { !known.contains($0.key) }
    object["properties"] = `properties`.json
    writePresence(`required`, "required", &object)
    object["type"] = `type`.json
    return .object(object)
  }
}
public enum ObjectType: ValueRepresentable {
  case `object`
  case unknown(String)
  static func read(_ json: DataValue) throws -> ObjectType {
    let value = try String.read(json)
    switch value {
    case "object": return .`object`
    default: return .unknown(value)
    }
  }
  var json: DataValue {
    switch self {
    case .`object`: return .string("object")
    case .unknown(let value): return .string(value)
    }
  }
}
public indirect enum Observation: ValueRepresentable {
  case `observationId`(ObservationObservationId)
  case `failureId`(ObservationFailureId)
  static func read(_ json: DataValue) throws -> Observation {
    let object = try jsonObject(json)
    if object["observation_id"] != nil && object["observation_id"] != .null {
      return .`observationId`(try ObservationObservationId.read(json))
    }
    if object["failure_id"] != nil && object["failure_id"] != .null {
      return .`failureId`(try ObservationFailureId.read(json))
    }
    throw ValueConversionError("Unknown Observation alternative")
  }
  var json: DataValue {
    switch self {
    case .`observationId`(let value): return value.json
    case .`failureId`(let value): return value.json
    }
  }
}
public typealias ObservationId = String
public struct ObservationFailureId: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `failureId`: FailureId
  public init(extensions: [String: DataValue] = [:], `failureId`: FailureId) {
    self.extensions = extensions
    self.`failureId` = `failureId`
  }
  static func read(_ json: DataValue) throws -> ObservationFailureId {
    let object = try jsonObject(json)
    let known: Set<String> = ["failure_id"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      failureId: try FailureId.read(jsonRequired(object, "failure_id")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["failure_id"]
    var object = extensions.filter { !known.contains($0.key) }
    object["failure_id"] = `failureId`.json
    return .object(object)
  }
}
public struct ObservationObservationId: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `observationId`: ObservationId
  public init(extensions: [String: DataValue] = [:], `observationId`: ObservationId) {
    self.extensions = extensions
    self.`observationId` = `observationId`
  }
  static func read(_ json: DataValue) throws -> ObservationObservationId {
    let object = try jsonObject(json)
    let known: Set<String> = ["observation_id"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      observationId: try ObservationId.read(jsonRequired(object, "observation_id")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["observation_id"]
    var object = extensions.filter { !known.contains($0.key) }
    object["observation_id"] = `observationId`.json
    return .object(object)
  }
}
public enum Origin: ValueRepresentable {
  case `live`
  case `cache`
  case `replay`
  case `proxy`
  case `memory`
  case unknown(String)
  static func read(_ json: DataValue) throws -> Origin {
    let value = try String.read(json)
    switch value {
    case "live": return .`live`
    case "cache": return .`cache`
    case "replay": return .`replay`
    case "proxy": return .`proxy`
    case "memory": return .`memory`
    default: return .unknown(value)
    }
  }
  var json: DataValue {
    switch self {
    case .`live`: return .string("live")
    case .`cache`: return .string("cache")
    case .`replay`: return .string("replay")
    case .`proxy`: return .string("proxy")
    case .`memory`: return .string("memory")
    case .unknown(let value): return .string(value)
    }
  }
}
public struct PersistenceObservation: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `advice`: Presence<String>
  public let `observedAt`: String
  public let `state`: UsagePersistence
  public init(
    extensions: [String: DataValue] = [:], `advice`: Presence<String> = .absent,
    `observedAt`: String, `state`: UsagePersistence
  ) {
    self.extensions = extensions
    self.`advice` = `advice`
    self.`observedAt` = `observedAt`
    self.`state` = `state`
  }
  static func read(_ json: DataValue) throws -> PersistenceObservation {
    let object = try jsonObject(json)
    let known: Set<String> = ["advice", "observed_at", "state"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      advice: try readPresence(object, "advice", String.self),
      observedAt: try String.read(jsonRequired(object, "observed_at")),
      state: try UsagePersistence.read(jsonRequired(object, "state")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["advice", "observed_at", "state"]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`advice`, "advice", &object)
    object["observed_at"] = `observedAt`.json
    object["state"] = `state`.json
    return .object(object)
  }
}
public struct PhysicalSource: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `file`: String
  public let `firstLine`: Presence<UInt64>
  public let `lastLine`: Presence<UInt64>
  public init(
    extensions: [String: DataValue] = [:], `file`: String, `firstLine`: Presence<UInt64> = .absent,
    `lastLine`: Presence<UInt64> = .absent
  ) {
    self.extensions = extensions
    self.`file` = `file`
    self.`firstLine` = `firstLine`
    self.`lastLine` = `lastLine`
  }
  static func read(_ json: DataValue) throws -> PhysicalSource {
    let object = try jsonObject(json)
    let known: Set<String> = ["file", "first_line", "last_line"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      file: try String.read(jsonRequired(object, "file")),
      firstLine: try readPresence(object, "first_line", UInt64.self),
      lastLine: try readPresence(object, "last_line", UInt64.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["file", "first_line", "last_line"]
    var object = extensions.filter { !known.contains($0.key) }
    object["file"] = `file`.json
    writePresence(`firstLine`, "first_line", &object)
    writePresence(`lastLine`, "last_line", &object)
    return .object(object)
  }
}
public struct Position: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `file`: String?
  public let `first`: Presence<UInt64>
  public let `images`: Presence<[String]>
  public let `last`: Presence<UInt64>
  public init(
    extensions: [String: DataValue] = [:], `file`: String?, `first`: Presence<UInt64> = .absent,
    `images`: Presence<[String]> = .absent, `last`: Presence<UInt64> = .absent
  ) {
    self.extensions = extensions
    self.`file` = `file`
    self.`first` = `first`
    self.`images` = `images`
    self.`last` = `last`
  }
  static func read(_ json: DataValue) throws -> Position {
    let object = try jsonObject(json)
    let known: Set<String> = ["file", "first", "images", "last"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      file: try String?.read(jsonRequired(object, "file")),
      first: try readPresence(object, "first", UInt64.self),
      images: try readPresence(object, "images", [String].self),
      last: try readPresence(object, "last", UInt64.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["file", "first", "images", "last"]
    var object = extensions.filter { !known.contains($0.key) }
    object["file"] = `file`.json
    writePresence(`first`, "first", &object)
    writePresence(`images`, "images", &object)
    writePresence(`last`, "last", &object)
    return .object(object)
  }
}
public typealias ReceiptQuestionName = String
public struct QuestionSource: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `answeredBy`: String
  public let `batchSize`: Presence<UInt64>
  public let `origin`: Origin
  public init(
    extensions: [String: DataValue] = [:], `answeredBy`: String,
    `batchSize`: Presence<UInt64> = .absent, `origin`: Origin
  ) {
    self.extensions = extensions
    self.`answeredBy` = `answeredBy`
    self.`batchSize` = `batchSize`
    self.`origin` = `origin`
  }
  static func read(_ json: DataValue) throws -> QuestionSource {
    let object = try jsonObject(json)
    let known: Set<String> = ["answered_by", "batch_size", "origin"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      answeredBy: try String.read(jsonRequired(object, "answered_by")),
      batchSize: try readPresence(object, "batch_size", UInt64.self),
      origin: try Origin.read(jsonRequired(object, "origin")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["answered_by", "batch_size", "origin"]
    var object = extensions.filter { !known.contains($0.key) }
    object["answered_by"] = `answeredBy`.json
    writePresence(`batchSize`, "batch_size", &object)
    object["origin"] = `origin`.json
    return .object(object)
  }
}
public struct RankMember: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `name`: String
  public let `result`: RankMemberResult
  public init(extensions: [String: DataValue] = [:], `name`: String, `result`: RankMemberResult) {
    self.extensions = extensions
    self.`name` = `name`
    self.`result` = `result`
  }
  static func read(_ json: DataValue) throws -> RankMember {
    let object = try jsonObject(json)
    let known: Set<String> = ["name", "result"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      name: try String.read(jsonRequired(object, "name")),
      result: try RankMemberResult.read(jsonRequired(object, "result")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["name", "result"]
    var object = extensions.filter { !known.contains($0.key) }
    object["name"] = `name`.json
    object["result"] = `result`.json
    return .object(object)
  }
}
public struct RankMemberResult: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `answer`: Answer
  public let `answerId`: AnswerId
  public let `images`: Presence<[ReceiptImage]>
  public let `meta`: Meta
  public let `question`: QuestionDocument
  public let `schema`: ReceiptVersion
  public let `source`: Presence<PhysicalSource>
  public let `threshold`: DataValue
  public let `value`: UInt64
  public init(
    extensions: [String: DataValue] = [:], `answer`: Answer, `answerId`: AnswerId,
    `images`: Presence<[ReceiptImage]> = .absent, `meta`: Meta, `question`: QuestionDocument,
    `schema`: ReceiptVersion, `source`: Presence<PhysicalSource> = .absent, `threshold`: DataValue,
    `value`: UInt64
  ) {
    self.extensions = extensions
    self.`answer` = `answer`
    self.`answerId` = `answerId`
    self.`images` = `images`
    self.`meta` = `meta`
    self.`question` = `question`
    self.`schema` = `schema`
    self.`source` = `source`
    self.`threshold` = `threshold`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> RankMemberResult {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "answer", "answer_id", "images", "meta", "question", "schema", "source", "threshold", "value",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      answer: try Answer.read(jsonRequired(object, "answer")),
      answerId: try AnswerId.read(jsonRequired(object, "answer_id")),
      images: try readPresence(object, "images", [ReceiptImage].self),
      meta: try Meta.read(jsonRequired(object, "meta")),
      question: try QuestionDocument.read(jsonRequired(object, "question")),
      schema: try ReceiptVersion.read(jsonRequired(object, "schema")),
      source: try readPresence(object, "source", PhysicalSource.self),
      threshold: try DataValue.read(jsonRequired(object, "threshold")),
      value: try UInt64.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "answer", "answer_id", "images", "meta", "question", "schema", "source", "threshold", "value",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    object["answer"] = `answer`.json
    object["answer_id"] = `answerId`.json
    writePresence(`images`, "images", &object)
    object["meta"] = `meta`.json
    object["question"] = `question`.json
    object["schema"] = `schema`.json
    writePresence(`source`, "source", &object)
    object["threshold"] = `threshold`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct Recognition: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `answer`: RecognitionOdds
  public let `answerId`: AnswerId
  public let `file`: Presence<String>
  public let `firstLine`: Presence<UInt64>
  public let `index`: Presence<UInt64>
  public let `input`: Presence<DataValue>
  public let `lastLine`: Presence<UInt64>
  public let `meta`: Meta
  public let `position`: Presence<Position>
  public let `question`: RecognizeQuestionDocument
  public let `schema`: ReceiptVersion
  public let `source`: Presence<PhysicalSource>
  public let `value`: Recognize
  public init(
    extensions: [String: DataValue] = [:], `answer`: RecognitionOdds, `answerId`: AnswerId,
    `file`: Presence<String> = .absent, `firstLine`: Presence<UInt64> = .absent,
    `index`: Presence<UInt64> = .absent, `input`: Presence<DataValue> = .absent,
    `lastLine`: Presence<UInt64> = .absent, `meta`: Meta, `position`: Presence<Position> = .absent,
    `question`: RecognizeQuestionDocument, `schema`: ReceiptVersion,
    `source`: Presence<PhysicalSource> = .absent, `value`: Recognize
  ) {
    self.extensions = extensions
    self.`answer` = `answer`
    self.`answerId` = `answerId`
    self.`file` = `file`
    self.`firstLine` = `firstLine`
    self.`index` = `index`
    self.`input` = `input`
    self.`lastLine` = `lastLine`
    self.`meta` = `meta`
    self.`position` = `position`
    self.`question` = `question`
    self.`schema` = `schema`
    self.`source` = `source`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> Recognition {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "answer", "answer_id", "file", "first_line", "index", "input", "last_line", "meta",
      "position", "question", "schema", "source", "value",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      answer: try RecognitionOdds.read(jsonRequired(object, "answer")),
      answerId: try AnswerId.read(jsonRequired(object, "answer_id")),
      file: try readPresence(object, "file", String.self),
      firstLine: try readPresence(object, "first_line", UInt64.self),
      index: try readPresence(object, "index", UInt64.self),
      input: try readPresence(object, "input", DataValue.self),
      lastLine: try readPresence(object, "last_line", UInt64.self),
      meta: try Meta.read(jsonRequired(object, "meta")),
      position: try readPresence(object, "position", Position.self),
      question: try RecognizeQuestionDocument.read(jsonRequired(object, "question")),
      schema: try ReceiptVersion.read(jsonRequired(object, "schema")),
      source: try readPresence(object, "source", PhysicalSource.self),
      value: try Recognize.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "answer", "answer_id", "file", "first_line", "index", "input", "last_line", "meta",
      "position", "question", "schema", "source", "value",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    object["answer"] = `answer`.json
    object["answer_id"] = `answerId`.json
    writePresence(`file`, "file", &object)
    writePresence(`firstLine`, "first_line", &object)
    writePresence(`index`, "index", &object)
    writePresence(`input`, "input", &object)
    writePresence(`lastLine`, "last_line", &object)
    object["meta"] = `meta`.json
    writePresence(`position`, "position", &object)
    object["question"] = `question`.json
    object["schema"] = `schema`.json
    writePresence(`source`, "source", &object)
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct RecognitionEdgeDocument: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `either`: Bool
  public let `probability`: Double
  public let `relation`: String
  public let `source`: Entity
  public let `target`: Entity
  public init(
    extensions: [String: DataValue] = [:], `either`: Bool, `probability`: Double,
    `relation`: String, `source`: Entity, `target`: Entity
  ) {
    self.extensions = extensions
    self.`either` = `either`
    self.`probability` = `probability`
    self.`relation` = `relation`
    self.`source` = `source`
    self.`target` = `target`
  }
  static func read(_ json: DataValue) throws -> RecognitionEdgeDocument {
    let object = try jsonObject(json)
    let known: Set<String> = ["either", "probability", "relation", "source", "target"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      either: try Bool.read(jsonRequired(object, "either")),
      probability: try Double.read(jsonRequired(object, "probability")),
      relation: try String.read(jsonRequired(object, "relation")),
      source: try Entity.read(jsonRequired(object, "source")),
      target: try Entity.read(jsonRequired(object, "target")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["either", "probability", "relation", "source", "target"]
    var object = extensions.filter { !known.contains($0.key) }
    object["either"] = `either`.json
    object["probability"] = `probability`.json
    object["relation"] = `relation`.json
    object["source"] = `source`.json
    object["target"] = `target`.json
    return .object(object)
  }
}
public enum ReceiptRecognitionMode: ValueRepresentable {
  case `whole`
  case `boundaryOnly`
  case unknown(String)
  static func read(_ json: DataValue) throws -> ReceiptRecognitionMode {
    let value = try String.read(json)
    switch value {
    case "whole": return .`whole`
    case "boundary_only": return .`boundaryOnly`
    default: return .unknown(value)
    }
  }
  var json: DataValue {
    switch self {
    case .`whole`: return .string("whole")
    case .`boundaryOnly`: return .string("boundary_only")
    case .unknown(let value): return .string(value)
    }
  }
}
public indirect enum RecognitionOdds: ValueRepresentable {
  case `namesPairsPiecesProposals`(RecognitionOddsRecognizeAnswer)
  case `piecesProposals`(RecognitionOddsBoundaryOdds)
  static func read(_ json: DataValue) throws -> RecognitionOdds {
    let object = try jsonObject(json)
    if object["names"] != nil && object["pairs"] != nil && object["pieces"] != nil
      && object["proposals"] != nil
    {
      return .`namesPairsPiecesProposals`(try RecognitionOddsRecognizeAnswer.read(json))
    }
    if object["pieces"] != nil && object["proposals"] != nil && object["names"] == nil
      && object["pairs"] == nil
    {
      return .`piecesProposals`(try RecognitionOddsBoundaryOdds.read(json))
    }
    throw ValueConversionError("Unknown RecognitionOdds alternative")
  }
  var json: DataValue {
    switch self {
    case .`namesPairsPiecesProposals`(let value): return value.json
    case .`piecesProposals`(let value): return value.json
    }
  }
}
public struct RecognitionOddsBoundaryOdds: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `pieces`: [PieceOdds]
  public let `proposals`: [BoundaryProposal]
  public init(
    extensions: [String: DataValue] = [:], `pieces`: [PieceOdds], `proposals`: [BoundaryProposal]
  ) {
    self.extensions = extensions
    self.`pieces` = `pieces`
    self.`proposals` = `proposals`
  }
  static func read(_ json: DataValue) throws -> RecognitionOddsBoundaryOdds {
    let object = try jsonObject(json)
    let known: Set<String> = ["pieces", "proposals"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      pieces: try [PieceOdds].read(jsonRequired(object, "pieces")),
      proposals: try [BoundaryProposal].read(jsonRequired(object, "proposals")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["pieces", "proposals"]
    var object = extensions.filter { !known.contains($0.key) }
    object["pieces"] = `pieces`.json
    object["proposals"] = `proposals`.json
    return .object(object)
  }
}
public struct RecognitionOddsRecognizeAnswer: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `names`: [NameOdds]
  public let `pairs`: [PairOdds]
  public let `pieces`: [PieceOdds]
  public let `proposals`: [RecognitionProposal]
  public init(
    extensions: [String: DataValue] = [:], `names`: [NameOdds], `pairs`: [PairOdds],
    `pieces`: [PieceOdds], `proposals`: [RecognitionProposal]
  ) {
    self.extensions = extensions
    self.`names` = `names`
    self.`pairs` = `pairs`
    self.`pieces` = `pieces`
    self.`proposals` = `proposals`
  }
  static func read(_ json: DataValue) throws -> RecognitionOddsRecognizeAnswer {
    let object = try jsonObject(json)
    let known: Set<String> = ["names", "pairs", "pieces", "proposals"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      names: try [NameOdds].read(jsonRequired(object, "names")),
      pairs: try [PairOdds].read(jsonRequired(object, "pairs")),
      pieces: try [PieceOdds].read(jsonRequired(object, "pieces")),
      proposals: try [RecognitionProposal].read(jsonRequired(object, "proposals")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["names", "pairs", "pieces", "proposals"]
    var object = extensions.filter { !known.contains($0.key) }
    object["names"] = `names`.json
    object["pairs"] = `pairs`.json
    object["pieces"] = `pieces`.json
    object["proposals"] = `proposals`.json
    return .object(object)
  }
}
public struct RecognitionProposal: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `end`: UInt64
  public let `kept`: Bool
  public let `kind`: Presence<String>
  public let `selected`: Presence<Place>
  public let `spanProbability`: Double
  public let `start`: UInt64
  public let `strength`: Presence<Double>
  public init(
    extensions: [String: DataValue] = [:], `end`: UInt64, `kept`: Bool,
    `kind`: Presence<String> = .absent, `selected`: Presence<Place> = .absent,
    `spanProbability`: Double, `start`: UInt64, `strength`: Presence<Double> = .absent
  ) {
    self.extensions = extensions
    self.`end` = `end`
    self.`kept` = `kept`
    self.`kind` = `kind`
    self.`selected` = `selected`
    self.`spanProbability` = `spanProbability`
    self.`start` = `start`
    self.`strength` = `strength`
  }
  static func read(_ json: DataValue) throws -> RecognitionProposal {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "end", "kept", "kind", "selected", "span_probability", "start", "strength",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      end: try UInt64.read(jsonRequired(object, "end")),
      kept: try Bool.read(jsonRequired(object, "kept")),
      kind: try readPresence(object, "kind", String.self),
      selected: try readPresence(object, "selected", Place.self),
      spanProbability: try Double.read(jsonRequired(object, "span_probability")),
      start: try UInt64.read(jsonRequired(object, "start")),
      strength: try readPresence(object, "strength", Double.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "end", "kept", "kind", "selected", "span_probability", "start", "strength",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    object["end"] = `end`.json
    object["kept"] = `kept`.json
    writePresence(`kind`, "kind", &object)
    writePresence(`selected`, "selected", &object)
    object["span_probability"] = `spanProbability`.json
    object["start"] = `start`.json
    writePresence(`strength`, "strength", &object)
    return .object(object)
  }
}
public struct ReceiptRecognitionStageContext: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `boundary`: Presence<String>
  public let `kindEdge`: Presence<String>
  public let `relation`: Presence<String>
  public init(
    extensions: [String: DataValue] = [:], `boundary`: Presence<String> = .absent,
    `kindEdge`: Presence<String> = .absent, `relation`: Presence<String> = .absent
  ) {
    self.extensions = extensions
    self.`boundary` = `boundary`
    self.`kindEdge` = `kindEdge`
    self.`relation` = `relation`
  }
  static func read(_ json: DataValue) throws -> ReceiptRecognitionStageContext {
    let object = try jsonObject(json)
    let known: Set<String> = ["boundary", "kind_edge", "relation"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      boundary: try readPresence(object, "boundary", String.self),
      kindEdge: try readPresence(object, "kind_edge", String.self),
      relation: try readPresence(object, "relation", String.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["boundary", "kind_edge", "relation"]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`boundary`, "boundary", &object)
    writePresence(`kindEdge`, "kind_edge", &object)
    writePresence(`relation`, "relation", &object)
    return .object(object)
  }
}
public struct Relation: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `answer`: Answers
  public let `answerId`: AnswerId
  public let `file`: Presence<String>
  public let `firstLine`: Presence<UInt64>
  public let `index`: Presence<UInt64>
  public let `input`: Presence<DataValue>
  public let `inputSources`: Presence<[SessionInputSource]>
  public let `lastLine`: Presence<UInt64>
  public let `meta`: Meta
  public let `position`: Presence<Position>
  public let `question`: RelateQuestionDocument
  public let `schema`: ReceiptVersion
  public let `value`: [RelatedEntityEdge]
  public init(
    extensions: [String: DataValue] = [:], `answer`: Answers, `answerId`: AnswerId,
    `file`: Presence<String> = .absent, `firstLine`: Presence<UInt64> = .absent,
    `index`: Presence<UInt64> = .absent, `input`: Presence<DataValue> = .absent,
    `inputSources`: Presence<[SessionInputSource]> = .absent,
    `lastLine`: Presence<UInt64> = .absent, `meta`: Meta, `position`: Presence<Position> = .absent,
    `question`: RelateQuestionDocument, `schema`: ReceiptVersion, `value`: [RelatedEntityEdge]
  ) {
    self.extensions = extensions
    self.`answer` = `answer`
    self.`answerId` = `answerId`
    self.`file` = `file`
    self.`firstLine` = `firstLine`
    self.`index` = `index`
    self.`input` = `input`
    self.`inputSources` = `inputSources`
    self.`lastLine` = `lastLine`
    self.`meta` = `meta`
    self.`position` = `position`
    self.`question` = `question`
    self.`schema` = `schema`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> Relation {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "answer", "answer_id", "file", "first_line", "index", "input", "input_sources", "last_line",
      "meta", "position", "question", "schema", "value",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      answer: try Answers.read(jsonRequired(object, "answer")),
      answerId: try AnswerId.read(jsonRequired(object, "answer_id")),
      file: try readPresence(object, "file", String.self),
      firstLine: try readPresence(object, "first_line", UInt64.self),
      index: try readPresence(object, "index", UInt64.self),
      input: try readPresence(object, "input", DataValue.self),
      inputSources: try readPresence(object, "input_sources", [SessionInputSource].self),
      lastLine: try readPresence(object, "last_line", UInt64.self),
      meta: try Meta.read(jsonRequired(object, "meta")),
      position: try readPresence(object, "position", Position.self),
      question: try RelateQuestionDocument.read(jsonRequired(object, "question")),
      schema: try ReceiptVersion.read(jsonRequired(object, "schema")),
      value: try [RelatedEntityEdge].read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "answer", "answer_id", "file", "first_line", "index", "input", "input_sources", "last_line",
      "meta", "position", "question", "schema", "value",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    object["answer"] = `answer`.json
    object["answer_id"] = `answerId`.json
    writePresence(`file`, "file", &object)
    writePresence(`firstLine`, "first_line", &object)
    writePresence(`index`, "index", &object)
    writePresence(`input`, "input", &object)
    writePresence(`inputSources`, "input_sources", &object)
    writePresence(`lastLine`, "last_line", &object)
    object["meta"] = `meta`.json
    writePresence(`position`, "position", &object)
    object["question"] = `question`.json
    object["schema"] = `schema`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public enum RelationDirection: ValueRepresentable {
  case `sourceToTarget`
  case `either`
  case unknown(String)
  static func read(_ json: DataValue) throws -> RelationDirection {
    let value = try String.read(json)
    switch value {
    case "source_to_target": return .`sourceToTarget`
    case "either": return .`either`
    default: return .unknown(value)
    }
  }
  var json: DataValue {
    switch self {
    case .`sourceToTarget`: return .string("source_to_target")
    case .`either`: return .string("either")
    case .unknown(let value): return .string(value)
    }
  }
}
public indirect enum RelationEndpoint: ValueRepresentable {
  case `kindName`(RelationEndpointRelatedEntity)
  case `fileKindNameOrdinalRecord`(RelationEndpointSource)
  static func read(_ json: DataValue) throws -> RelationEndpoint {
    let object = try jsonObject(json)
    if object["kind"] != nil && object["name"] != nil && object["file"] == nil
      && object["ordinal"] == nil && object["record"] == nil
    {
      return .`kindName`(try RelationEndpointRelatedEntity.read(json))
    }
    if object["file"] != nil && object["kind"] != nil && object["name"] != nil
      && object["ordinal"] != nil && object["record"] != nil
    {
      return .`fileKindNameOrdinalRecord`(try RelationEndpointSource.read(json))
    }
    throw ValueConversionError("Unknown RelationEndpoint alternative")
  }
  var json: DataValue {
    switch self {
    case .`kindName`(let value): return value.json
    case .`fileKindNameOrdinalRecord`(let value): return value.json
    }
  }
}
public struct RelationEndpointRelatedEntity: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `name`: String
  public init(extensions: [String: DataValue] = [:], `kind`: String, `name`: String) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`name` = `name`
  }
  static func read(_ json: DataValue) throws -> RelationEndpointRelatedEntity {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "name"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      name: try String.read(jsonRequired(object, "name")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "name"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["name"] = `name`.json
    return .object(object)
  }
}
public struct RelationEndpointSource: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `file`: String?
  public let `firstLine`: Presence<UInt64>
  public let `kind`: String
  public let `lastLine`: Presence<UInt64>
  public let `name`: String
  public let `ordinal`: UInt64
  public let `record`: DataValue
  public init(
    extensions: [String: DataValue] = [:], `file`: String?, `firstLine`: Presence<UInt64> = .absent,
    `kind`: String, `lastLine`: Presence<UInt64> = .absent, `name`: String, `ordinal`: UInt64,
    `record`: DataValue
  ) {
    self.extensions = extensions
    self.`file` = `file`
    self.`firstLine` = `firstLine`
    self.`kind` = `kind`
    self.`lastLine` = `lastLine`
    self.`name` = `name`
    self.`ordinal` = `ordinal`
    self.`record` = `record`
  }
  static func read(_ json: DataValue) throws -> RelationEndpointSource {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "file", "first_line", "kind", "last_line", "name", "ordinal", "record",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      file: try String?.read(jsonRequired(object, "file")),
      firstLine: try readPresence(object, "first_line", UInt64.self),
      kind: try String.read(jsonRequired(object, "kind")),
      lastLine: try readPresence(object, "last_line", UInt64.self),
      name: try String.read(jsonRequired(object, "name")),
      ordinal: try UInt64.read(jsonRequired(object, "ordinal")),
      record: try DataValue.read(jsonRequired(object, "record")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "file", "first_line", "kind", "last_line", "name", "ordinal", "record",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    object["file"] = `file`.json
    writePresence(`firstLine`, "first_line", &object)
    object["kind"] = `kind`.json
    writePresence(`lastLine`, "last_line", &object)
    object["name"] = `name`.json
    object["ordinal"] = `ordinal`.json
    object["record"] = `record`.json
    return .object(object)
  }
}
public indirect enum RelationMember: ValueRepresentable {
  case `answerId`(RelationMemberAnswerId)
  case `failureId`(RelationMemberFailureId)
  static func read(_ json: DataValue) throws -> RelationMember {
    let object = try jsonObject(json)
    if object["answer_id"] != nil && object["answer_id"] != .null {
      return .`answerId`(try RelationMemberAnswerId.read(json))
    }
    if object["failure_id"] != nil && object["failure_id"] != .null {
      return .`failureId`(try RelationMemberFailureId.read(json))
    }
    throw ValueConversionError("Unknown RelationMember alternative")
  }
  var json: DataValue {
    switch self {
    case .`answerId`(let value): return value.json
    case .`failureId`(let value): return value.json
    }
  }
}
public struct RelationMemberAnswerId: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `direction`: RelationDirection
  public let `method`: RelationMethod
  public let `observations`: [Observation]
  public let `question`: QuestionDocument
  public let `questionSources`: [QuestionSource]
  public let `reads`: String
  public let `relation`: String
  public let `request`: String
  public let `source`: RelatedEntity
  public let `target`: RelatedEntity?
  public let `threshold`: ReceiptThreshold
  public let `usage`: Presence<Usage>
  public let `accepted`: Bool
  public let `answer`: Answer
  public let `answerId`: AnswerId
  public let `probability`: Double
  public init(
    extensions: [String: DataValue] = [:], `direction`: RelationDirection, `method`: RelationMethod,
    `observations`: [Observation], `question`: QuestionDocument,
    `questionSources`: [QuestionSource], `reads`: String, `relation`: String, `request`: String,
    `source`: RelatedEntity, `target`: RelatedEntity?, `threshold`: ReceiptThreshold,
    `usage`: Presence<Usage> = .absent, `accepted`: Bool, `answer`: Answer, `answerId`: AnswerId,
    `probability`: Double
  ) {
    self.extensions = extensions
    self.`direction` = `direction`
    self.`method` = `method`
    self.`observations` = `observations`
    self.`question` = `question`
    self.`questionSources` = `questionSources`
    self.`reads` = `reads`
    self.`relation` = `relation`
    self.`request` = `request`
    self.`source` = `source`
    self.`target` = `target`
    self.`threshold` = `threshold`
    self.`usage` = `usage`
    self.`accepted` = `accepted`
    self.`answer` = `answer`
    self.`answerId` = `answerId`
    self.`probability` = `probability`
  }
  static func read(_ json: DataValue) throws -> RelationMemberAnswerId {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "direction", "method", "observations", "question", "question_sources", "reads", "relation",
      "request", "source", "target", "threshold", "usage", "accepted", "answer", "answer_id",
      "probability",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      direction: try RelationDirection.read(jsonRequired(object, "direction")),
      method: try RelationMethod.read(jsonRequired(object, "method")),
      observations: try [Observation].read(jsonRequired(object, "observations")),
      question: try QuestionDocument.read(jsonRequired(object, "question")),
      questionSources: try [QuestionSource].read(jsonRequired(object, "question_sources")),
      reads: try String.read(jsonRequired(object, "reads")),
      relation: try String.read(jsonRequired(object, "relation")),
      request: try String.read(jsonRequired(object, "request")),
      source: try RelatedEntity.read(jsonRequired(object, "source")),
      target: try RelatedEntity?.read(jsonRequired(object, "target")),
      threshold: try ReceiptThreshold.read(jsonRequired(object, "threshold")),
      usage: try readPresence(object, "usage", Usage.self),
      accepted: try Bool.read(jsonRequired(object, "accepted")),
      answer: try Answer.read(jsonRequired(object, "answer")),
      answerId: try AnswerId.read(jsonRequired(object, "answer_id")),
      probability: try Double.read(jsonRequired(object, "probability")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "direction", "method", "observations", "question", "question_sources", "reads", "relation",
      "request", "source", "target", "threshold", "usage", "accepted", "answer", "answer_id",
      "probability",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    object["direction"] = `direction`.json
    object["method"] = `method`.json
    object["observations"] = `observations`.json
    object["question"] = `question`.json
    object["question_sources"] = `questionSources`.json
    object["reads"] = `reads`.json
    object["relation"] = `relation`.json
    object["request"] = `request`.json
    object["source"] = `source`.json
    object["target"] = `target`.json
    object["threshold"] = `threshold`.json
    writePresence(`usage`, "usage", &object)
    object["accepted"] = `accepted`.json
    object["answer"] = `answer`.json
    object["answer_id"] = `answerId`.json
    object["probability"] = `probability`.json
    return .object(object)
  }
  public func sourceName() throws -> Presence<String> {
    switch try readPresence(jsonObject(self.json), "source", DataValue.self) {
    case .absent: return .absent
    case .null: return .null
    case .value(let object): return try readPresence(jsonObject(object), "name", String.self)
    }
  }
}
public struct RelationMemberFailureId: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `direction`: RelationDirection
  public let `method`: RelationMethod
  public let `observations`: [Observation]
  public let `question`: QuestionDocument
  public let `questionSources`: [QuestionSource]
  public let `reads`: String
  public let `relation`: String
  public let `request`: String
  public let `source`: RelatedEntity
  public let `target`: RelatedEntity?
  public let `threshold`: ReceiptThreshold
  public let `usage`: Presence<Usage>
  public let `failure`: Failure
  public let `failureId`: FailureId
  public init(
    extensions: [String: DataValue] = [:], `direction`: RelationDirection, `method`: RelationMethod,
    `observations`: [Observation], `question`: QuestionDocument,
    `questionSources`: [QuestionSource], `reads`: String, `relation`: String, `request`: String,
    `source`: RelatedEntity, `target`: RelatedEntity?, `threshold`: ReceiptThreshold,
    `usage`: Presence<Usage> = .absent, `failure`: Failure, `failureId`: FailureId
  ) {
    self.extensions = extensions
    self.`direction` = `direction`
    self.`method` = `method`
    self.`observations` = `observations`
    self.`question` = `question`
    self.`questionSources` = `questionSources`
    self.`reads` = `reads`
    self.`relation` = `relation`
    self.`request` = `request`
    self.`source` = `source`
    self.`target` = `target`
    self.`threshold` = `threshold`
    self.`usage` = `usage`
    self.`failure` = `failure`
    self.`failureId` = `failureId`
  }
  static func read(_ json: DataValue) throws -> RelationMemberFailureId {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "direction", "method", "observations", "question", "question_sources", "reads", "relation",
      "request", "source", "target", "threshold", "usage", "failure", "failure_id",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      direction: try RelationDirection.read(jsonRequired(object, "direction")),
      method: try RelationMethod.read(jsonRequired(object, "method")),
      observations: try [Observation].read(jsonRequired(object, "observations")),
      question: try QuestionDocument.read(jsonRequired(object, "question")),
      questionSources: try [QuestionSource].read(jsonRequired(object, "question_sources")),
      reads: try String.read(jsonRequired(object, "reads")),
      relation: try String.read(jsonRequired(object, "relation")),
      request: try String.read(jsonRequired(object, "request")),
      source: try RelatedEntity.read(jsonRequired(object, "source")),
      target: try RelatedEntity?.read(jsonRequired(object, "target")),
      threshold: try ReceiptThreshold.read(jsonRequired(object, "threshold")),
      usage: try readPresence(object, "usage", Usage.self),
      failure: try Failure.read(jsonRequired(object, "failure")),
      failureId: try FailureId.read(jsonRequired(object, "failure_id")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "direction", "method", "observations", "question", "question_sources", "reads", "relation",
      "request", "source", "target", "threshold", "usage", "failure", "failure_id",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    object["direction"] = `direction`.json
    object["method"] = `method`.json
    object["observations"] = `observations`.json
    object["question"] = `question`.json
    object["question_sources"] = `questionSources`.json
    object["reads"] = `reads`.json
    object["relation"] = `relation`.json
    object["request"] = `request`.json
    object["source"] = `source`.json
    object["target"] = `target`.json
    object["threshold"] = `threshold`.json
    writePresence(`usage`, "usage", &object)
    object["failure"] = `failure`.json
    object["failure_id"] = `failureId`.json
    return .object(object)
  }
  public func sourceName() throws -> Presence<String> {
    switch try readPresence(jsonObject(self.json), "source", DataValue.self) {
    case .absent: return .absent
    case .null: return .null
    case .value(let object): return try readPresence(jsonObject(object), "name", String.self)
    }
  }
}
public enum RelationMethod: ValueRepresentable {
  case `yesNo`
  case `choice`
  case unknown(String)
  static func read(_ json: DataValue) throws -> RelationMethod {
    let value = try String.read(json)
    switch value {
    case "yes_no": return .`yesNo`
    case "choice": return .`choice`
    default: return .unknown(value)
    }
  }
  var json: DataValue {
    switch self {
    case .`yesNo`: return .string("yes_no")
    case .`choice`: return .string("choice")
    case .unknown(let value): return .string(value)
    }
  }
}
public enum ReceiptFunction: ValueRepresentable {
  case `decide`
  case `choose`
  case `tag`
  case `score`
  case `filter`
  case `rank`
  case `find`
  case `annotate`
  case `recognize`
  case `relate`
  case unknown(String)
  static func read(_ json: DataValue) throws -> ReceiptFunction {
    let value = try String.read(json)
    switch value {
    case "decide": return .`decide`
    case "choose": return .`choose`
    case "tag": return .`tag`
    case "score": return .`score`
    case "filter": return .`filter`
    case "rank": return .`rank`
    case "find": return .`find`
    case "annotate": return .`annotate`
    case "recognize": return .`recognize`
    case "relate": return .`relate`
    default: return .unknown(value)
    }
  }
  var json: DataValue {
    switch self {
    case .`decide`: return .string("decide")
    case .`choose`: return .string("choose")
    case .`tag`: return .string("tag")
    case .`score`: return .string("score")
    case .`filter`: return .string("filter")
    case .`rank`: return .string("rank")
    case .`find`: return .string("find")
    case .`annotate`: return .string("annotate")
    case .`recognize`: return .string("recognize")
    case .`relate`: return .string("relate")
    case .unknown(let value): return .string(value)
    }
  }
}
public typealias SdkRequestId = String
public indirect enum SendBudgetDenial: ValueRepresentable {
  case `beforeFirstSend`(SendBudgetDenialBeforeFirstSend)
  case `beforeAdditionalSend`(SendBudgetDenialBeforeAdditionalSend)
  case `beforeRetry`(SendBudgetDenialBeforeRetry)
  static func read(_ json: DataValue) throws -> SendBudgetDenial {
    let object = try jsonObject(json)
    if object["kind"] == .string("before_first_send") {
      return .`beforeFirstSend`(try SendBudgetDenialBeforeFirstSend.read(json))
    }
    if object["kind"] == .string("before_additional_send") {
      return .`beforeAdditionalSend`(try SendBudgetDenialBeforeAdditionalSend.read(json))
    }
    if object["kind"] == .string("before_retry") {
      return .`beforeRetry`(try SendBudgetDenialBeforeRetry.read(json))
    }
    throw ValueConversionError("Unknown SendBudgetDenial alternative")
  }
  var json: DataValue {
    switch self {
    case .`beforeFirstSend`(let value): return value.json
    case .`beforeAdditionalSend`(let value): return value.json
    case .`beforeRetry`(let value): return value.json
    }
  }
}
public struct SendBudgetDenialBeforeAdditionalSend: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public init(extensions: [String: DataValue] = [:], `kind`: String = "before_additional_send") {
    self.extensions = extensions
    self.`kind` = `kind`
  }
  static func read(_ json: DataValue) throws -> SendBudgetDenialBeforeAdditionalSend {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    return .object(object)
  }
}
public struct SendBudgetDenialBeforeFirstSend: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public init(extensions: [String: DataValue] = [:], `kind`: String = "before_first_send") {
    self.extensions = extensions
    self.`kind` = `kind`
  }
  static func read(_ json: DataValue) throws -> SendBudgetDenialBeforeFirstSend {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    return .object(object)
  }
}
public struct SendBudgetDenialBeforeRetry: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `lastStatus`: UInt64
  public init(
    extensions: [String: DataValue] = [:], `kind`: String = "before_retry", `lastStatus`: UInt64
  ) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`lastStatus` = `lastStatus`
  }
  static func read(_ json: DataValue) throws -> SendBudgetDenialBeforeRetry {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "last_status"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      lastStatus: try UInt64.read(jsonRequired(object, "last_status")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "last_status"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["last_status"] = `lastStatus`.json
    return .object(object)
  }
}
public enum StopCause: ValueRepresentable {
  case `usage`
  case `local`
  case `noKey`
  case `transport`
  case `status`
  case `tooLarge`
  case `reply`
  case `backend`
  case `cancelled`
  case `deadline`
  case `defect`
  case unknown(String)
  static func read(_ json: DataValue) throws -> StopCause {
    let value = try String.read(json)
    switch value {
    case "usage": return .`usage`
    case "local": return .`local`
    case "no_key": return .`noKey`
    case "transport": return .`transport`
    case "status": return .`status`
    case "too_large": return .`tooLarge`
    case "reply": return .`reply`
    case "backend": return .`backend`
    case "cancelled": return .`cancelled`
    case "deadline": return .`deadline`
    case "defect": return .`defect`
    default: return .unknown(value)
    }
  }
  var json: DataValue {
    switch self {
    case .`usage`: return .string("usage")
    case .`local`: return .string("local")
    case .`noKey`: return .string("no_key")
    case .`transport`: return .string("transport")
    case .`status`: return .string("status")
    case .`tooLarge`: return .string("too_large")
    case .`reply`: return .string("reply")
    case .`backend`: return .string("backend")
    case .`cancelled`: return .string("cancelled")
    case .`deadline`: return .string("deadline")
    case .`defect`: return .string("defect")
    case .unknown(let value): return .string(value)
    }
  }
}
public struct Stopped: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `at`: Presence<UInt64>
  public let `cause`: StopCause
  public let `retryable`: Bool
  public let `status`: Presence<UInt64>
  public init(
    extensions: [String: DataValue] = [:], `at`: Presence<UInt64> = .absent, `cause`: StopCause,
    `retryable`: Bool, `status`: Presence<UInt64> = .absent
  ) {
    self.extensions = extensions
    self.`at` = `at`
    self.`cause` = `cause`
    self.`retryable` = `retryable`
    self.`status` = `status`
  }
  static func read(_ json: DataValue) throws -> Stopped {
    let object = try jsonObject(json)
    let known: Set<String> = ["at", "cause", "retryable", "status"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      at: try readPresence(object, "at", UInt64.self),
      cause: try StopCause.read(jsonRequired(object, "cause")),
      retryable: try Bool.read(jsonRequired(object, "retryable")),
      status: try readPresence(object, "status", UInt64.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["at", "cause", "retryable", "status"]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`at`, "at", &object)
    object["cause"] = `cause`.json
    object["retryable"] = `retryable`.json
    writePresence(`status`, "status", &object)
    return .object(object)
  }
}
public struct StringRoot: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `type`: StringType
  public init(extensions: [String: DataValue] = [:], `type`: StringType) {
    self.extensions = extensions
    self.`type` = `type`
  }
  static func read(_ json: DataValue) throws -> StringRoot {
    let object = try jsonObject(json)
    let known: Set<String> = ["type"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      type: try StringType.read(jsonRequired(object, "type")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["type"]
    var object = extensions.filter { !known.contains($0.key) }
    object["type"] = `type`.json
    return .object(object)
  }
}
public enum StringType: ValueRepresentable {
  case `string`
  case unknown(String)
  static func read(_ json: DataValue) throws -> StringType {
    let value = try String.read(json)
    switch value {
    case "string": return .`string`
    default: return .unknown(value)
    }
  }
  var json: DataValue {
    switch self {
    case .`string`: return .string("string")
    case .unknown(let value): return .string(value)
    }
  }
}
public struct Usage: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `inputTokens`: Presence<UInt64>
  public let `outputTokens`: Presence<UInt64>
  public init(
    extensions: [String: DataValue] = [:], `inputTokens`: Presence<UInt64> = .absent,
    `outputTokens`: Presence<UInt64> = .absent
  ) {
    self.extensions = extensions
    self.`inputTokens` = `inputTokens`
    self.`outputTokens` = `outputTokens`
  }
  static func read(_ json: DataValue) throws -> Usage {
    let object = try jsonObject(json)
    let known: Set<String> = ["input_tokens", "output_tokens"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      inputTokens: try readPresence(object, "input_tokens", UInt64.self),
      outputTokens: try readPresence(object, "output_tokens", UInt64.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["input_tokens", "output_tokens"]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`inputTokens`, "input_tokens", &object)
    writePresence(`outputTokens`, "output_tokens", &object)
    return .object(object)
  }
}
public enum UsagePersistence: ValueRepresentable {
  case `disabled`
  case `pending`
  case `written`
  case `failed`
  case unknown(String)
  static func read(_ json: DataValue) throws -> UsagePersistence {
    let value = try String.read(json)
    switch value {
    case "disabled": return .`disabled`
    case "pending": return .`pending`
    case "written": return .`written`
    case "failed": return .`failed`
    default: return .unknown(value)
    }
  }
  var json: DataValue {
    switch self {
    case .`disabled`: return .string("disabled")
    case .`pending`: return .string("pending")
    case .`written`: return .string("written")
    case .`failed`: return .string("failed")
    case .unknown(let value): return .string(value)
    }
  }
}
public enum Verb: ValueRepresentable {
  case `recognize`
  case unknown(String)
  static func read(_ json: DataValue) throws -> Verb {
    let value = try String.read(json)
    switch value {
    case "recognize": return .`recognize`
    default: return .unknown(value)
    }
  }
  var json: DataValue {
    switch self {
    case .`recognize`: return .string("recognize")
    case .unknown(let value): return .string(value)
    }
  }
}
public enum ReceiptVersion: ValueRepresentable {
  case `thinkthenResult_2`
  case unknown(String)
  static func read(_ json: DataValue) throws -> ReceiptVersion {
    let value = try String.read(json)
    switch value {
    case "thinkthen.result/2": return .`thinkthenResult_2`
    default: return .unknown(value)
    }
  }
  var json: DataValue {
    switch self {
    case .`thinkthenResult_2`: return .string("thinkthen.result/2")
    case .unknown(let value): return .string(value)
    }
  }
}
public typealias WordingVersion = UInt64
public indirect enum AnnotatedField: ValueRepresentable {
  case `boolean`(Bool)
  case `null`
  case `string`(String)
  case `list`([String])
  case `number`(Double)
  case `completefailed`(Failed)
  static func read(_ json: DataValue) throws -> AnnotatedField {
    if let value = try? Bool.read(json) { return .`boolean`(value) }
    if json == .null { return .`null` }
    if let value = try? String.read(json) { return .`string`(value) }
    if let value = try? [String].read(json) { return .`list`(value) }
    if let value = try? Double.read(json) { return .`number`(value) }
    if let value = try? Failed.read(json) { return .`completefailed`(value) }
    throw ValueConversionError("Unknown AnnotatedField value")
  }
  var json: DataValue {
    switch self {
    case .`boolean`(let value): return value.json
    case .`null`: return .null
    case .`string`(let value): return value.json
    case .`list`(let value): return value.json
    case .`number`(let value): return value.json
    case .`completefailed`(let value): return value.json
    }
  }
}
public typealias AnnotatedRow = [String: AnnotatedField]
public indirect enum Answer: ValueRepresentable {
  case `yesNo`(AnswerYesNo)
  case `choice`(AnswerChoice)
  case `tag`(AnswerTag)
  case `score`(AnswerScore)
  static func read(_ json: DataValue) throws -> Answer {
    let object = try jsonObject(json)
    if object["kind"] == .string("yes_no") { return .`yesNo`(try AnswerYesNo.read(json)) }
    if object["kind"] == .string("choice") { return .`choice`(try AnswerChoice.read(json)) }
    if object["kind"] == .string("tag") { return .`tag`(try AnswerTag.read(json)) }
    if object["kind"] == .string("score") { return .`score`(try AnswerScore.read(json)) }
    throw ValueConversionError("Unknown Answer alternative")
  }
  var json: DataValue {
    switch self {
    case .`yesNo`(let value): return value.json
    case .`choice`(let value): return value.json
    case .`tag`(let value): return value.json
    case .`score`(let value): return value.json
    }
  }
}
public struct AnswerChoice: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `confidence`: Presence<Double>
  public let `kind`: String
  public let `pick`: String
  public let `probabilities`: [String: Double]
  public init(
    extensions: [String: DataValue] = [:], `confidence`: Presence<Double> = .absent,
    `kind`: String = "choice", `pick`: String, `probabilities`: [String: Double]
  ) {
    self.extensions = extensions
    self.`confidence` = `confidence`
    self.`kind` = `kind`
    self.`pick` = `pick`
    self.`probabilities` = `probabilities`
  }
  static func read(_ json: DataValue) throws -> AnswerChoice {
    let object = try jsonObject(json)
    let known: Set<String> = ["confidence", "kind", "pick", "probabilities"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      confidence: try readPresence(object, "confidence", Double.self),
      kind: try String.read(jsonRequired(object, "kind")),
      pick: try String.read(jsonRequired(object, "pick")),
      probabilities: try [String: Double].read(jsonRequired(object, "probabilities")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["confidence", "kind", "pick", "probabilities"]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`confidence`, "confidence", &object)
    object["kind"] = `kind`.json
    object["pick"] = `pick`.json
    object["probabilities"] = `probabilities`.json
    return .object(object)
  }
}
public struct AnswerScore: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `confidence`: Presence<Double>
  public let `kind`: String
  public let `level`: String
  public let `probabilities`: [String: Double]
  public init(
    extensions: [String: DataValue] = [:], `confidence`: Presence<Double> = .absent,
    `kind`: String = "score", `level`: String, `probabilities`: [String: Double]
  ) {
    self.extensions = extensions
    self.`confidence` = `confidence`
    self.`kind` = `kind`
    self.`level` = `level`
    self.`probabilities` = `probabilities`
  }
  static func read(_ json: DataValue) throws -> AnswerScore {
    let object = try jsonObject(json)
    let known: Set<String> = ["confidence", "kind", "level", "probabilities"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      confidence: try readPresence(object, "confidence", Double.self),
      kind: try String.read(jsonRequired(object, "kind")),
      level: try String.read(jsonRequired(object, "level")),
      probabilities: try [String: Double].read(jsonRequired(object, "probabilities")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["confidence", "kind", "level", "probabilities"]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`confidence`, "confidence", &object)
    object["kind"] = `kind`.json
    object["level"] = `level`.json
    object["probabilities"] = `probabilities`.json
    return .object(object)
  }
}
public struct AnswerTag: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `probabilities`: [String: Double]
  public init(
    extensions: [String: DataValue] = [:], `kind`: String = "tag", `probabilities`: [String: Double]
  ) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`probabilities` = `probabilities`
  }
  static func read(_ json: DataValue) throws -> AnswerTag {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "probabilities"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      probabilities: try [String: Double].read(jsonRequired(object, "probabilities")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "probabilities"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["probabilities"] = `probabilities`.json
    return .object(object)
  }
}
public struct AnswerYesNo: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `probability`: Double
  public init(
    extensions: [String: DataValue] = [:], `kind`: String = "yes_no", `probability`: Double
  ) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`probability` = `probability`
  }
  static func read(_ json: DataValue) throws -> AnswerYesNo {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "probability"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      probability: try Double.read(jsonRequired(object, "probability")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "probability"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["probability"] = `probability`.json
    return .object(object)
  }
}
public enum AttemptOutcome: ValueRepresentable {
  case `ok`
  case `status`
  case `transport`
  case unknown(String)
  static func read(_ json: DataValue) throws -> AttemptOutcome {
    let value = try String.read(json)
    switch value {
    case "ok": return .`ok`
    case "status": return .`status`
    case "transport": return .`transport`
    default: return .unknown(value)
    }
  }
  var json: DataValue {
    switch self {
    case .`ok`: return .string("ok")
    case .`status`: return .string("status")
    case .`transport`: return .string("transport")
    case .unknown(let value): return .string(value)
    }
  }
}
public indirect enum BatchSetting: ValueRepresentable {
  case `integer`(UInt64)
  case `string`(String)
  static func read(_ json: DataValue) throws -> BatchSetting {
    if let value = try? UInt64.read(json) { return .`integer`(value) }
    if let value = try? String.read(json) { return .`string`(value) }
    throw ValueConversionError("Unknown BatchSetting value")
  }
  var json: DataValue {
    switch self {
    case .`integer`(let value): return value.json
    case .`string`(let value): return value.json
    }
  }
}
public struct BatchWarning: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `running`: BatchSetting
  public let `tunedFor`: BatchSetting
  public init(
    extensions: [String: DataValue] = [:], `running`: BatchSetting, `tunedFor`: BatchSetting
  ) {
    self.extensions = extensions
    self.`running` = `running`
    self.`tunedFor` = `tunedFor`
  }
  static func read(_ json: DataValue) throws -> BatchWarning {
    let object = try jsonObject(json)
    let known: Set<String> = ["running", "tuned_for"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      running: try BatchSetting.read(jsonRequired(object, "running")),
      tunedFor: try BatchSetting.read(jsonRequired(object, "tuned_for")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["running", "tuned_for"]
    var object = extensions.filter { !known.contains($0.key) }
    object["running"] = `running`.json
    object["tuned_for"] = `tunedFor`.json
    return .object(object)
  }
}
public struct Entity: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `end`: UInt64
  public let `file`: Presence<String>
  public let `firstLine`: Presence<UInt64>
  public let `kind`: String
  public let `lastLine`: Presence<UInt64>
  public let `length`: UInt64
  public let `start`: UInt64
  public let `strength`: Double
  public let `text`: String
  public init(
    extensions: [String: DataValue] = [:], `end`: UInt64, `file`: Presence<String> = .absent,
    `firstLine`: Presence<UInt64> = .absent, `kind`: String, `lastLine`: Presence<UInt64> = .absent,
    `length`: UInt64, `start`: UInt64, `strength`: Double, `text`: String
  ) {
    self.extensions = extensions
    self.`end` = `end`
    self.`file` = `file`
    self.`firstLine` = `firstLine`
    self.`kind` = `kind`
    self.`lastLine` = `lastLine`
    self.`length` = `length`
    self.`start` = `start`
    self.`strength` = `strength`
    self.`text` = `text`
  }
  static func read(_ json: DataValue) throws -> Entity {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "end", "file", "first_line", "kind", "last_line", "length", "start", "strength", "text",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      end: try UInt64.read(jsonRequired(object, "end")),
      file: try readPresence(object, "file", String.self),
      firstLine: try readPresence(object, "first_line", UInt64.self),
      kind: try String.read(jsonRequired(object, "kind")),
      lastLine: try readPresence(object, "last_line", UInt64.self),
      length: try UInt64.read(jsonRequired(object, "length")),
      start: try UInt64.read(jsonRequired(object, "start")),
      strength: try Double.read(jsonRequired(object, "strength")),
      text: try String.read(jsonRequired(object, "text")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "end", "file", "first_line", "kind", "last_line", "length", "start", "strength", "text",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    object["end"] = `end`.json
    writePresence(`file`, "file", &object)
    writePresence(`firstLine`, "first_line", &object)
    object["kind"] = `kind`.json
    writePresence(`lastLine`, "last_line", &object)
    object["length"] = `length`.json
    object["start"] = `start`.json
    object["strength"] = `strength`.json
    object["text"] = `text`.json
    return .object(object)
  }
}
public struct EntityEdge: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `either`: Presence<Bool>
  public let `probability`: Double
  public let `relation`: String
  public let `source`: Entity
  public let `target`: Entity
  public init(
    extensions: [String: DataValue] = [:], `either`: Presence<Bool> = .absent,
    `probability`: Double, `relation`: String, `source`: Entity, `target`: Entity
  ) {
    self.extensions = extensions
    self.`either` = `either`
    self.`probability` = `probability`
    self.`relation` = `relation`
    self.`source` = `source`
    self.`target` = `target`
  }
  static func read(_ json: DataValue) throws -> EntityEdge {
    let object = try jsonObject(json)
    let known: Set<String> = ["either", "probability", "relation", "source", "target"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      either: try readPresence(object, "either", Bool.self),
      probability: try Double.read(jsonRequired(object, "probability")),
      relation: try String.read(jsonRequired(object, "relation")),
      source: try Entity.read(jsonRequired(object, "source")),
      target: try Entity.read(jsonRequired(object, "target")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["either", "probability", "relation", "source", "target"]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`either`, "either", &object)
    object["probability"] = `probability`.json
    object["relation"] = `relation`.json
    object["source"] = `source`.json
    object["target"] = `target`.json
    return .object(object)
  }
}
public struct Failed: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `failed`: Failure
  public init(extensions: [String: DataValue] = [:], `failed`: Failure) {
    self.extensions = extensions
    self.`failed` = `failed`
  }
  static func read(_ json: DataValue) throws -> Failed {
    let object = try jsonObject(json)
    let known: Set<String> = ["failed"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      failed: try Failure.read(jsonRequired(object, "failed")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["failed"]
    var object = extensions.filter { !known.contains($0.key) }
    object["failed"] = `failed`.json
    return .object(object)
  }
}
public struct Failure: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `cause`: FailureCause
  public let `kind`: String
  public init(extensions: [String: DataValue] = [:], `cause`: FailureCause, `kind`: String) {
    self.extensions = extensions
    self.`cause` = `cause`
    self.`kind` = `kind`
  }
  static func read(_ json: DataValue) throws -> Failure {
    let object = try jsonObject(json)
    let known: Set<String> = ["cause", "kind"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      cause: try FailureCause.read(jsonRequired(object, "cause")),
      kind: try String.read(jsonRequired(object, "kind")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["cause", "kind"]
    var object = extensions.filter { !known.contains($0.key) }
    object["cause"] = `cause`.json
    object["kind"] = `kind`.json
    return .object(object)
  }
}
public enum FailureCause: ValueRepresentable {
  case `missingAnswer`
  case `wrongKind`
  case `missingProbability`
  case `invalidProbability`
  case `invalidDistribution`
  case `unexpectedProbability`
  case unknown(String)
  static func read(_ json: DataValue) throws -> FailureCause {
    let value = try String.read(json)
    switch value {
    case "missing_answer": return .`missingAnswer`
    case "wrong_kind": return .`wrongKind`
    case "missing_probability": return .`missingProbability`
    case "invalid_probability": return .`invalidProbability`
    case "invalid_distribution": return .`invalidDistribution`
    case "unexpected_probability": return .`unexpectedProbability`
    default: return .unknown(value)
    }
  }
  var json: DataValue {
    switch self {
    case .`missingAnswer`: return .string("missing_answer")
    case .`wrongKind`: return .string("wrong_kind")
    case .`missingProbability`: return .string("missing_probability")
    case .`invalidProbability`: return .string("invalid_probability")
    case .`invalidDistribution`: return .string("invalid_distribution")
    case .`unexpectedProbability`: return .string("unexpected_probability")
    case .unknown(let value): return .string(value)
    }
  }
}
public enum FailureKind: ValueRepresentable {
  case `usage`
  case `backend`
  case `local`
  case `cancelled`
  case `deadline`
  case `defect`
  case unknown(String)
  static func read(_ json: DataValue) throws -> FailureKind {
    let value = try String.read(json)
    switch value {
    case "usage": return .`usage`
    case "backend": return .`backend`
    case "local": return .`local`
    case "cancelled": return .`cancelled`
    case "deadline": return .`deadline`
    case "defect": return .`defect`
    default: return .unknown(value)
    }
  }
  var json: DataValue {
    switch self {
    case .`usage`: return .string("usage")
    case .`backend`: return .string("backend")
    case .`local`: return .string("local")
    case .`cancelled`: return .string("cancelled")
    case .`deadline`: return .string("deadline")
    case .`defect`: return .string("defect")
    case .unknown(let value): return .string(value)
    }
  }
}
public struct FindAnswer: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `confidence`: Presence<Double>
  public let `kind`: String
  public let `pick`: String
  public let `probabilities`: [String: Double]
  public init(
    extensions: [String: DataValue] = [:], `confidence`: Presence<Double> = .absent,
    `kind`: String = "find", `pick`: String, `probabilities`: [String: Double]
  ) {
    self.extensions = extensions
    self.`confidence` = `confidence`
    self.`kind` = `kind`
    self.`pick` = `pick`
    self.`probabilities` = `probabilities`
  }
  static func read(_ json: DataValue) throws -> FindAnswer {
    let object = try jsonObject(json)
    let known: Set<String> = ["confidence", "kind", "pick", "probabilities"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      confidence: try readPresence(object, "confidence", Double.self),
      kind: try String.read(jsonRequired(object, "kind")),
      pick: try String.read(jsonRequired(object, "pick")),
      probabilities: try [String: Double].read(jsonRequired(object, "probabilities")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["confidence", "kind", "pick", "probabilities"]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`confidence`, "confidence", &object)
    object["kind"] = `kind`.json
    object["pick"] = `pick`.json
    object["probabilities"] = `probabilities`.json
    return .object(object)
  }
}
public struct FindQuestionDocument: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `batch`: Presence<ReceiptBatch>
  public let `contextSchema`: Presence<InputDeclaration>
  public let `itemSchema`: Presence<InputDeclaration>
  public let `labelDetails`: Presence<[Label]>
  public let `model`: Presence<String>
  public let `name`: Presence<ReceiptQuestionName>
  public let `none`: Bool
  public let `on`: Presence<[String]>
  public let `profile`: Presence<String>
  public let `text`: DataValue
  public let `verb`: String
  public let `wordingVersion`: Presence<WordingVersion>
  public init(
    extensions: [String: DataValue] = [:], `batch`: Presence<ReceiptBatch> = .absent,
    `contextSchema`: Presence<InputDeclaration> = .absent,
    `itemSchema`: Presence<InputDeclaration> = .absent, `labelDetails`: Presence<[Label]> = .absent,
    `model`: Presence<String> = .absent, `name`: Presence<ReceiptQuestionName> = .absent,
    `none`: Bool, `on`: Presence<[String]> = .absent, `profile`: Presence<String> = .absent,
    `text`: DataValue, `verb`: String = "find", `wordingVersion`: Presence<WordingVersion> = .absent
  ) {
    self.extensions = extensions
    self.`batch` = `batch`
    self.`contextSchema` = `contextSchema`
    self.`itemSchema` = `itemSchema`
    self.`labelDetails` = `labelDetails`
    self.`model` = `model`
    self.`name` = `name`
    self.`none` = `none`
    self.`on` = `on`
    self.`profile` = `profile`
    self.`text` = `text`
    self.`verb` = `verb`
    self.`wordingVersion` = `wordingVersion`
  }
  static func read(_ json: DataValue) throws -> FindQuestionDocument {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "batch", "context_schema", "item_schema", "label_details", "model", "name", "none", "on",
      "profile", "text", "verb", "wording_version",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      batch: try readPresence(object, "batch", ReceiptBatch.self),
      contextSchema: try readPresence(object, "context_schema", InputDeclaration.self),
      itemSchema: try readPresence(object, "item_schema", InputDeclaration.self),
      labelDetails: try readPresence(object, "label_details", [Label].self),
      model: try readPresence(object, "model", String.self),
      name: try readPresence(object, "name", ReceiptQuestionName.self),
      none: try Bool.read(jsonRequired(object, "none")),
      on: try readPresence(object, "on", [String].self),
      profile: try readPresence(object, "profile", String.self),
      text: try DataValue.read(jsonRequired(object, "text")),
      verb: try String.read(jsonRequired(object, "verb")),
      wordingVersion: try readPresence(object, "wording_version", WordingVersion.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "batch", "context_schema", "item_schema", "label_details", "model", "name", "none", "on",
      "profile", "text", "verb", "wording_version",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`batch`, "batch", &object)
    writePresence(`contextSchema`, "context_schema", &object)
    writePresence(`itemSchema`, "item_schema", &object)
    writePresence(`labelDetails`, "label_details", &object)
    writePresence(`model`, "model", &object)
    writePresence(`name`, "name", &object)
    object["none"] = `none`.json
    writePresence(`on`, "on", &object)
    writePresence(`profile`, "profile", &object)
    object["text"] = `text`.json
    object["verb"] = `verb`.json
    writePresence(`wordingVersion`, "wording_version", &object)
    return .object(object)
  }
}
public struct NameOdds: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `edges`: [String: Double]?
  public let `end`: UInt64
  public let `kinds`: [String: Double]?
  public let `start`: UInt64
  public init(
    extensions: [String: DataValue] = [:], `edges`: [String: Double]?, `end`: UInt64,
    `kinds`: [String: Double]?, `start`: UInt64
  ) {
    self.extensions = extensions
    self.`edges` = `edges`
    self.`end` = `end`
    self.`kinds` = `kinds`
    self.`start` = `start`
  }
  static func read(_ json: DataValue) throws -> NameOdds {
    let object = try jsonObject(json)
    let known: Set<String> = ["edges", "end", "kinds", "start"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      edges: try [String: Double]?.read(jsonRequired(object, "edges")),
      end: try UInt64.read(jsonRequired(object, "end")),
      kinds: try [String: Double]?.read(jsonRequired(object, "kinds")),
      start: try UInt64.read(jsonRequired(object, "start")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["edges", "end", "kinds", "start"]
    var object = extensions.filter { !known.contains($0.key) }
    object["edges"] = `edges`.json
    object["end"] = `end`.json
    object["kinds"] = `kinds`.json
    object["start"] = `start`.json
    return .object(object)
  }
}
public struct PairOdds: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `probability`: Double
  public let `relation`: String
  public let `source`: Place
  public let `target`: Place
  public init(
    extensions: [String: DataValue] = [:], `probability`: Double, `relation`: String,
    `source`: Place, `target`: Place
  ) {
    self.extensions = extensions
    self.`probability` = `probability`
    self.`relation` = `relation`
    self.`source` = `source`
    self.`target` = `target`
  }
  static func read(_ json: DataValue) throws -> PairOdds {
    let object = try jsonObject(json)
    let known: Set<String> = ["probability", "relation", "source", "target"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      probability: try Double.read(jsonRequired(object, "probability")),
      relation: try String.read(jsonRequired(object, "relation")),
      source: try Place.read(jsonRequired(object, "source")),
      target: try Place.read(jsonRequired(object, "target")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["probability", "relation", "source", "target"]
    var object = extensions.filter { !known.contains($0.key) }
    object["probability"] = `probability`.json
    object["relation"] = `relation`.json
    object["source"] = `source`.json
    object["target"] = `target`.json
    return .object(object)
  }
}
public struct PieceOdds: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `end`: UInt64
  public let `start`: UInt64
  public let `tags`: [String: Double]
  public init(
    extensions: [String: DataValue] = [:], `end`: UInt64, `start`: UInt64, `tags`: [String: Double]
  ) {
    self.extensions = extensions
    self.`end` = `end`
    self.`start` = `start`
    self.`tags` = `tags`
  }
  static func read(_ json: DataValue) throws -> PieceOdds {
    let object = try jsonObject(json)
    let known: Set<String> = ["end", "start", "tags"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      end: try UInt64.read(jsonRequired(object, "end")),
      start: try UInt64.read(jsonRequired(object, "start")),
      tags: try [String: Double].read(jsonRequired(object, "tags")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["end", "start", "tags"]
    var object = extensions.filter { !known.contains($0.key) }
    object["end"] = `end`.json
    object["start"] = `start`.json
    object["tags"] = `tags`.json
    return .object(object)
  }
}
public struct Place: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `end`: UInt64
  public let `start`: UInt64
  public init(extensions: [String: DataValue] = [:], `end`: UInt64, `start`: UInt64) {
    self.extensions = extensions
    self.`end` = `end`
    self.`start` = `start`
  }
  static func read(_ json: DataValue) throws -> Place {
    let object = try jsonObject(json)
    let known: Set<String> = ["end", "start"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      end: try UInt64.read(jsonRequired(object, "end")),
      start: try UInt64.read(jsonRequired(object, "start")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["end", "start"]
    var object = extensions.filter { !known.contains($0.key) }
    object["end"] = `end`.json
    object["start"] = `start`.json
    return .object(object)
  }
}
public struct Plan: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `estimatedBytes`: UInt64
  public let `estimatedInputTokens`: TokenBand
  public let `firstBodyUtf8`: String?
  public let `largestRequestBytes`: UInt64
  public let `largestRequestEstimatedInputTokens`: UInt64
  public let `records`: UInt64
  public let `requests`: UInt64
  public let `tokenEstimateMethod`: String
  public let `upperBound`: Bool
  public init(
    extensions: [String: DataValue] = [:], `estimatedBytes`: UInt64,
    `estimatedInputTokens`: TokenBand, `firstBodyUtf8`: String?, `largestRequestBytes`: UInt64,
    `largestRequestEstimatedInputTokens`: UInt64, `records`: UInt64, `requests`: UInt64,
    `tokenEstimateMethod`: String, `upperBound`: Bool
  ) {
    self.extensions = extensions
    self.`estimatedBytes` = `estimatedBytes`
    self.`estimatedInputTokens` = `estimatedInputTokens`
    self.`firstBodyUtf8` = `firstBodyUtf8`
    self.`largestRequestBytes` = `largestRequestBytes`
    self.`largestRequestEstimatedInputTokens` = `largestRequestEstimatedInputTokens`
    self.`records` = `records`
    self.`requests` = `requests`
    self.`tokenEstimateMethod` = `tokenEstimateMethod`
    self.`upperBound` = `upperBound`
  }
  static func read(_ json: DataValue) throws -> Plan {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "estimated_bytes", "estimated_input_tokens", "first_body_utf8", "largest_request_bytes",
      "largest_request_estimated_input_tokens", "records", "requests", "token_estimate_method",
      "upper_bound",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      estimatedBytes: try UInt64.read(jsonRequired(object, "estimated_bytes")),
      estimatedInputTokens: try TokenBand.read(jsonRequired(object, "estimated_input_tokens")),
      firstBodyUtf8: try String?.read(jsonRequired(object, "first_body_utf8")),
      largestRequestBytes: try UInt64.read(jsonRequired(object, "largest_request_bytes")),
      largestRequestEstimatedInputTokens: try UInt64.read(
        jsonRequired(object, "largest_request_estimated_input_tokens")),
      records: try UInt64.read(jsonRequired(object, "records")),
      requests: try UInt64.read(jsonRequired(object, "requests")),
      tokenEstimateMethod: try String.read(jsonRequired(object, "token_estimate_method")),
      upperBound: try Bool.read(jsonRequired(object, "upper_bound")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "estimated_bytes", "estimated_input_tokens", "first_body_utf8", "largest_request_bytes",
      "largest_request_estimated_input_tokens", "records", "requests", "token_estimate_method",
      "upper_bound",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    object["estimated_bytes"] = `estimatedBytes`.json
    object["estimated_input_tokens"] = `estimatedInputTokens`.json
    object["first_body_utf8"] = `firstBodyUtf8`.json
    object["largest_request_bytes"] = `largestRequestBytes`.json
    object["largest_request_estimated_input_tokens"] = `largestRequestEstimatedInputTokens`.json
    object["records"] = `records`.json
    object["requests"] = `requests`.json
    object["token_estimate_method"] = `tokenEstimateMethod`.json
    object["upper_bound"] = `upperBound`.json
    return .object(object)
  }
}
public struct ProfileWarning: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `running`: String
  public let `tunedFor`: String
  public init(extensions: [String: DataValue] = [:], `running`: String, `tunedFor`: String) {
    self.extensions = extensions
    self.`running` = `running`
    self.`tunedFor` = `tunedFor`
  }
  static func read(_ json: DataValue) throws -> ProfileWarning {
    let object = try jsonObject(json)
    let known: Set<String> = ["running", "tuned_for"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      running: try String.read(jsonRequired(object, "running")),
      tunedFor: try String.read(jsonRequired(object, "tuned_for")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["running", "tuned_for"]
    var object = extensions.filter { !known.contains($0.key) }
    object["running"] = `running`.json
    object["tuned_for"] = `tunedFor`.json
    return .object(object)
  }
}
public indirect enum QuestionDocument: ValueRepresentable {
  case `decide`(QuestionDocumentDecide)
  case `choose`(QuestionDocumentChoose)
  case `tag`(QuestionDocumentTag)
  case `score`(QuestionDocumentScore)
  static func read(_ json: DataValue) throws -> QuestionDocument {
    let object = try jsonObject(json)
    if object["verb"] == .string("decide") {
      return .`decide`(try QuestionDocumentDecide.read(json))
    }
    if object["verb"] == .string("choose") {
      return .`choose`(try QuestionDocumentChoose.read(json))
    }
    if object["verb"] == .string("tag") { return .`tag`(try QuestionDocumentTag.read(json)) }
    if object["verb"] == .string("score") { return .`score`(try QuestionDocumentScore.read(json)) }
    throw ValueConversionError("Unknown QuestionDocument alternative")
  }
  var json: DataValue {
    switch self {
    case .`decide`(let value): return value.json
    case .`choose`(let value): return value.json
    case .`tag`(let value): return value.json
    case .`score`(let value): return value.json
    }
  }
}
public struct QuestionDocumentChoose: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `batch`: Presence<ReceiptBatch>
  public let `contextSchema`: Presence<InputDeclaration>
  public let `itemSchema`: Presence<InputDeclaration>
  public let `labelDetails`: Presence<[Label]>
  public let `model`: Presence<String>
  public let `name`: Presence<ReceiptQuestionName>
  public let `on`: Presence<[String]>
  public let `profile`: Presence<String>
  public let `wordingVersion`: Presence<WordingVersion>
  public let `options`: [String]
  public let `text`: DataValue
  public let `verb`: String
  public init(
    extensions: [String: DataValue] = [:], `batch`: Presence<ReceiptBatch> = .absent,
    `contextSchema`: Presence<InputDeclaration> = .absent,
    `itemSchema`: Presence<InputDeclaration> = .absent, `labelDetails`: Presence<[Label]> = .absent,
    `model`: Presence<String> = .absent, `name`: Presence<ReceiptQuestionName> = .absent,
    `on`: Presence<[String]> = .absent, `profile`: Presence<String> = .absent,
    `wordingVersion`: Presence<WordingVersion> = .absent, `options`: [String], `text`: DataValue,
    `verb`: String = "choose"
  ) {
    self.extensions = extensions
    self.`batch` = `batch`
    self.`contextSchema` = `contextSchema`
    self.`itemSchema` = `itemSchema`
    self.`labelDetails` = `labelDetails`
    self.`model` = `model`
    self.`name` = `name`
    self.`on` = `on`
    self.`profile` = `profile`
    self.`wordingVersion` = `wordingVersion`
    self.`options` = `options`
    self.`text` = `text`
    self.`verb` = `verb`
  }
  static func read(_ json: DataValue) throws -> QuestionDocumentChoose {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "batch", "context_schema", "item_schema", "label_details", "model", "name", "on", "profile",
      "wording_version", "options", "text", "verb",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      batch: try readPresence(object, "batch", ReceiptBatch.self),
      contextSchema: try readPresence(object, "context_schema", InputDeclaration.self),
      itemSchema: try readPresence(object, "item_schema", InputDeclaration.self),
      labelDetails: try readPresence(object, "label_details", [Label].self),
      model: try readPresence(object, "model", String.self),
      name: try readPresence(object, "name", ReceiptQuestionName.self),
      on: try readPresence(object, "on", [String].self),
      profile: try readPresence(object, "profile", String.self),
      wordingVersion: try readPresence(object, "wording_version", WordingVersion.self),
      options: try [String].read(jsonRequired(object, "options")),
      text: try DataValue.read(jsonRequired(object, "text")),
      verb: try String.read(jsonRequired(object, "verb")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "batch", "context_schema", "item_schema", "label_details", "model", "name", "on", "profile",
      "wording_version", "options", "text", "verb",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`batch`, "batch", &object)
    writePresence(`contextSchema`, "context_schema", &object)
    writePresence(`itemSchema`, "item_schema", &object)
    writePresence(`labelDetails`, "label_details", &object)
    writePresence(`model`, "model", &object)
    writePresence(`name`, "name", &object)
    writePresence(`on`, "on", &object)
    writePresence(`profile`, "profile", &object)
    writePresence(`wordingVersion`, "wording_version", &object)
    object["options"] = `options`.json
    object["text"] = `text`.json
    object["verb"] = `verb`.json
    return .object(object)
  }
}
public struct QuestionDocumentDecide: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `batch`: Presence<ReceiptBatch>
  public let `contextSchema`: Presence<InputDeclaration>
  public let `itemSchema`: Presence<InputDeclaration>
  public let `labelDetails`: Presence<[Label]>
  public let `model`: Presence<String>
  public let `name`: Presence<ReceiptQuestionName>
  public let `on`: Presence<[String]>
  public let `profile`: Presence<String>
  public let `wordingVersion`: Presence<WordingVersion>
  public let `false`: Presence<DataValue>
  public let `text`: DataValue
  public let `true`: Presence<DataValue>
  public let `verb`: String
  public init(
    extensions: [String: DataValue] = [:], `batch`: Presence<ReceiptBatch> = .absent,
    `contextSchema`: Presence<InputDeclaration> = .absent,
    `itemSchema`: Presence<InputDeclaration> = .absent, `labelDetails`: Presence<[Label]> = .absent,
    `model`: Presence<String> = .absent, `name`: Presence<ReceiptQuestionName> = .absent,
    `on`: Presence<[String]> = .absent, `profile`: Presence<String> = .absent,
    `wordingVersion`: Presence<WordingVersion> = .absent, `false`: Presence<DataValue> = .absent,
    `text`: DataValue, `true`: Presence<DataValue> = .absent, `verb`: String = "decide"
  ) {
    self.extensions = extensions
    self.`batch` = `batch`
    self.`contextSchema` = `contextSchema`
    self.`itemSchema` = `itemSchema`
    self.`labelDetails` = `labelDetails`
    self.`model` = `model`
    self.`name` = `name`
    self.`on` = `on`
    self.`profile` = `profile`
    self.`wordingVersion` = `wordingVersion`
    self.`false` = `false`
    self.`text` = `text`
    self.`true` = `true`
    self.`verb` = `verb`
  }
  static func read(_ json: DataValue) throws -> QuestionDocumentDecide {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "batch", "context_schema", "item_schema", "label_details", "model", "name", "on", "profile",
      "wording_version", "false", "text", "true", "verb",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      batch: try readPresence(object, "batch", ReceiptBatch.self),
      contextSchema: try readPresence(object, "context_schema", InputDeclaration.self),
      itemSchema: try readPresence(object, "item_schema", InputDeclaration.self),
      labelDetails: try readPresence(object, "label_details", [Label].self),
      model: try readPresence(object, "model", String.self),
      name: try readPresence(object, "name", ReceiptQuestionName.self),
      on: try readPresence(object, "on", [String].self),
      profile: try readPresence(object, "profile", String.self),
      wordingVersion: try readPresence(object, "wording_version", WordingVersion.self),
      false: try readPresence(object, "false", DataValue.self),
      text: try DataValue.read(jsonRequired(object, "text")),
      true: try readPresence(object, "true", DataValue.self),
      verb: try String.read(jsonRequired(object, "verb")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "batch", "context_schema", "item_schema", "label_details", "model", "name", "on", "profile",
      "wording_version", "false", "text", "true", "verb",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`batch`, "batch", &object)
    writePresence(`contextSchema`, "context_schema", &object)
    writePresence(`itemSchema`, "item_schema", &object)
    writePresence(`labelDetails`, "label_details", &object)
    writePresence(`model`, "model", &object)
    writePresence(`name`, "name", &object)
    writePresence(`on`, "on", &object)
    writePresence(`profile`, "profile", &object)
    writePresence(`wordingVersion`, "wording_version", &object)
    writePresence(`false`, "false", &object)
    object["text"] = `text`.json
    writePresence(`true`, "true", &object)
    object["verb"] = `verb`.json
    return .object(object)
  }
}
public struct QuestionDocumentScore: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `batch`: Presence<ReceiptBatch>
  public let `contextSchema`: Presence<InputDeclaration>
  public let `itemSchema`: Presence<InputDeclaration>
  public let `labelDetails`: Presence<[Label]>
  public let `model`: Presence<String>
  public let `name`: Presence<ReceiptQuestionName>
  public let `on`: Presence<[String]>
  public let `profile`: Presence<String>
  public let `wordingVersion`: Presence<WordingVersion>
  public let `levels`: [String]
  public let `text`: DataValue
  public let `verb`: String
  public init(
    extensions: [String: DataValue] = [:], `batch`: Presence<ReceiptBatch> = .absent,
    `contextSchema`: Presence<InputDeclaration> = .absent,
    `itemSchema`: Presence<InputDeclaration> = .absent, `labelDetails`: Presence<[Label]> = .absent,
    `model`: Presence<String> = .absent, `name`: Presence<ReceiptQuestionName> = .absent,
    `on`: Presence<[String]> = .absent, `profile`: Presence<String> = .absent,
    `wordingVersion`: Presence<WordingVersion> = .absent, `levels`: [String], `text`: DataValue,
    `verb`: String = "score"
  ) {
    self.extensions = extensions
    self.`batch` = `batch`
    self.`contextSchema` = `contextSchema`
    self.`itemSchema` = `itemSchema`
    self.`labelDetails` = `labelDetails`
    self.`model` = `model`
    self.`name` = `name`
    self.`on` = `on`
    self.`profile` = `profile`
    self.`wordingVersion` = `wordingVersion`
    self.`levels` = `levels`
    self.`text` = `text`
    self.`verb` = `verb`
  }
  static func read(_ json: DataValue) throws -> QuestionDocumentScore {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "batch", "context_schema", "item_schema", "label_details", "model", "name", "on", "profile",
      "wording_version", "levels", "text", "verb",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      batch: try readPresence(object, "batch", ReceiptBatch.self),
      contextSchema: try readPresence(object, "context_schema", InputDeclaration.self),
      itemSchema: try readPresence(object, "item_schema", InputDeclaration.self),
      labelDetails: try readPresence(object, "label_details", [Label].self),
      model: try readPresence(object, "model", String.self),
      name: try readPresence(object, "name", ReceiptQuestionName.self),
      on: try readPresence(object, "on", [String].self),
      profile: try readPresence(object, "profile", String.self),
      wordingVersion: try readPresence(object, "wording_version", WordingVersion.self),
      levels: try [String].read(jsonRequired(object, "levels")),
      text: try DataValue.read(jsonRequired(object, "text")),
      verb: try String.read(jsonRequired(object, "verb")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "batch", "context_schema", "item_schema", "label_details", "model", "name", "on", "profile",
      "wording_version", "levels", "text", "verb",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`batch`, "batch", &object)
    writePresence(`contextSchema`, "context_schema", &object)
    writePresence(`itemSchema`, "item_schema", &object)
    writePresence(`labelDetails`, "label_details", &object)
    writePresence(`model`, "model", &object)
    writePresence(`name`, "name", &object)
    writePresence(`on`, "on", &object)
    writePresence(`profile`, "profile", &object)
    writePresence(`wordingVersion`, "wording_version", &object)
    object["levels"] = `levels`.json
    object["text"] = `text`.json
    object["verb"] = `verb`.json
    return .object(object)
  }
}
public struct QuestionDocumentTag: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `batch`: Presence<ReceiptBatch>
  public let `contextSchema`: Presence<InputDeclaration>
  public let `itemSchema`: Presence<InputDeclaration>
  public let `labelDetails`: Presence<[Label]>
  public let `model`: Presence<String>
  public let `name`: Presence<ReceiptQuestionName>
  public let `on`: Presence<[String]>
  public let `profile`: Presence<String>
  public let `wordingVersion`: Presence<WordingVersion>
  public let `labels`: [String]
  public let `text`: DataValue
  public let `verb`: String
  public init(
    extensions: [String: DataValue] = [:], `batch`: Presence<ReceiptBatch> = .absent,
    `contextSchema`: Presence<InputDeclaration> = .absent,
    `itemSchema`: Presence<InputDeclaration> = .absent, `labelDetails`: Presence<[Label]> = .absent,
    `model`: Presence<String> = .absent, `name`: Presence<ReceiptQuestionName> = .absent,
    `on`: Presence<[String]> = .absent, `profile`: Presence<String> = .absent,
    `wordingVersion`: Presence<WordingVersion> = .absent, `labels`: [String], `text`: DataValue,
    `verb`: String = "tag"
  ) {
    self.extensions = extensions
    self.`batch` = `batch`
    self.`contextSchema` = `contextSchema`
    self.`itemSchema` = `itemSchema`
    self.`labelDetails` = `labelDetails`
    self.`model` = `model`
    self.`name` = `name`
    self.`on` = `on`
    self.`profile` = `profile`
    self.`wordingVersion` = `wordingVersion`
    self.`labels` = `labels`
    self.`text` = `text`
    self.`verb` = `verb`
  }
  static func read(_ json: DataValue) throws -> QuestionDocumentTag {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "batch", "context_schema", "item_schema", "label_details", "model", "name", "on", "profile",
      "wording_version", "labels", "text", "verb",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      batch: try readPresence(object, "batch", ReceiptBatch.self),
      contextSchema: try readPresence(object, "context_schema", InputDeclaration.self),
      itemSchema: try readPresence(object, "item_schema", InputDeclaration.self),
      labelDetails: try readPresence(object, "label_details", [Label].self),
      model: try readPresence(object, "model", String.self),
      name: try readPresence(object, "name", ReceiptQuestionName.self),
      on: try readPresence(object, "on", [String].self),
      profile: try readPresence(object, "profile", String.self),
      wordingVersion: try readPresence(object, "wording_version", WordingVersion.self),
      labels: try [String].read(jsonRequired(object, "labels")),
      text: try DataValue.read(jsonRequired(object, "text")),
      verb: try String.read(jsonRequired(object, "verb")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "batch", "context_schema", "item_schema", "label_details", "model", "name", "on", "profile",
      "wording_version", "labels", "text", "verb",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`batch`, "batch", &object)
    writePresence(`contextSchema`, "context_schema", &object)
    writePresence(`itemSchema`, "item_schema", &object)
    writePresence(`labelDetails`, "label_details", &object)
    writePresence(`model`, "model", &object)
    writePresence(`name`, "name", &object)
    writePresence(`on`, "on", &object)
    writePresence(`profile`, "profile", &object)
    writePresence(`wordingVersion`, "wording_version", &object)
    object["labels"] = `labels`.json
    object["text"] = `text`.json
    object["verb"] = `verb`.json
    return .object(object)
  }
}
public indirect enum Recognize: ValueRepresentable {
  case `entities`(RecognizeEntities)
  case `modeProposals`(RecognizeModeProposals)
  static func read(_ json: DataValue) throws -> Recognize {
    let object = try jsonObject(json)
    if object["entities"] != nil && object["mode"] == nil && object["proposals"] == nil {
      return .`entities`(try RecognizeEntities.read(json))
    }
    if object["mode"] != nil && object["proposals"] != nil && object["entities"] == nil {
      return .`modeProposals`(try RecognizeModeProposals.read(json))
    }
    throw ValueConversionError("Unknown Recognize alternative")
  }
  var json: DataValue {
    switch self {
    case .`entities`(let value): return value.json
    case .`modeProposals`(let value): return value.json
    }
  }
}
public struct RecognizeAnswer: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `names`: [NameOdds]
  public let `pairs`: [PairOdds]
  public let `pieces`: [PieceOdds]
  public let `proposals`: [RecognitionProposal]
  public init(
    extensions: [String: DataValue] = [:], `names`: [NameOdds], `pairs`: [PairOdds],
    `pieces`: [PieceOdds], `proposals`: [RecognitionProposal]
  ) {
    self.extensions = extensions
    self.`names` = `names`
    self.`pairs` = `pairs`
    self.`pieces` = `pieces`
    self.`proposals` = `proposals`
  }
  static func read(_ json: DataValue) throws -> RecognizeAnswer {
    let object = try jsonObject(json)
    let known: Set<String> = ["names", "pairs", "pieces", "proposals"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      names: try [NameOdds].read(jsonRequired(object, "names")),
      pairs: try [PairOdds].read(jsonRequired(object, "pairs")),
      pieces: try [PieceOdds].read(jsonRequired(object, "pieces")),
      proposals: try [RecognitionProposal].read(jsonRequired(object, "proposals")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["names", "pairs", "pieces", "proposals"]
    var object = extensions.filter { !known.contains($0.key) }
    object["names"] = `names`.json
    object["pairs"] = `pairs`.json
    object["pieces"] = `pieces`.json
    object["proposals"] = `proposals`.json
    return .object(object)
  }
}
public struct RecognizeQuestionDocument: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `batch`: Presence<ReceiptBatch>
  public let `contextSchema`: Presence<InputDeclaration>
  public let `entityDefinition`: Presence<DataValue>
  public let `instructions`: Presence<DataValue>
  public let `itemSchema`: Presence<InputDeclaration>
  public let `kinds`: [String: DataValue]
  public let `labelDetails`: Presence<[Label]>
  public let `mode`: Presence<ReceiptRecognitionMode>
  public let `model`: Presence<String>
  public let `name`: Presence<ReceiptQuestionName>
  public let `on`: Presence<[String]>
  public let `profile`: Presence<String>
  public let `relationThreshold`: Presence<ReceiptThreshold>
  public let `relations`: Presence<[RelationRule]>
  public let `snippetPieces`: Presence<UInt64>
  public let `stageContext`: Presence<ReceiptRecognitionStageContext>
  public let `threshold`: ReceiptThreshold
  public let `verb`: Verb
  public let `wordingVersion`: Presence<WordingVersion>
  public init(
    extensions: [String: DataValue] = [:], `batch`: Presence<ReceiptBatch> = .absent,
    `contextSchema`: Presence<InputDeclaration> = .absent,
    `entityDefinition`: Presence<DataValue> = .absent,
    `instructions`: Presence<DataValue> = .absent,
    `itemSchema`: Presence<InputDeclaration> = .absent, `kinds`: [String: DataValue],
    `labelDetails`: Presence<[Label]> = .absent, `mode`: Presence<ReceiptRecognitionMode> = .absent,
    `model`: Presence<String> = .absent, `name`: Presence<ReceiptQuestionName> = .absent,
    `on`: Presence<[String]> = .absent, `profile`: Presence<String> = .absent,
    `relationThreshold`: Presence<ReceiptThreshold> = .absent,
    `relations`: Presence<[RelationRule]> = .absent, `snippetPieces`: Presence<UInt64> = .absent,
    `stageContext`: Presence<ReceiptRecognitionStageContext> = .absent,
    `threshold`: ReceiptThreshold, `verb`: Verb,
    `wordingVersion`: Presence<WordingVersion> = .absent
  ) {
    self.extensions = extensions
    self.`batch` = `batch`
    self.`contextSchema` = `contextSchema`
    self.`entityDefinition` = `entityDefinition`
    self.`instructions` = `instructions`
    self.`itemSchema` = `itemSchema`
    self.`kinds` = `kinds`
    self.`labelDetails` = `labelDetails`
    self.`mode` = `mode`
    self.`model` = `model`
    self.`name` = `name`
    self.`on` = `on`
    self.`profile` = `profile`
    self.`relationThreshold` = `relationThreshold`
    self.`relations` = `relations`
    self.`snippetPieces` = `snippetPieces`
    self.`stageContext` = `stageContext`
    self.`threshold` = `threshold`
    self.`verb` = `verb`
    self.`wordingVersion` = `wordingVersion`
  }
  static func read(_ json: DataValue) throws -> RecognizeQuestionDocument {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "batch", "context_schema", "entity_definition", "instructions", "item_schema", "kinds",
      "label_details", "mode", "model", "name", "on", "profile", "relation_threshold", "relations",
      "snippet_pieces", "stage_context", "threshold", "verb", "wording_version",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      batch: try readPresence(object, "batch", ReceiptBatch.self),
      contextSchema: try readPresence(object, "context_schema", InputDeclaration.self),
      entityDefinition: try readPresence(object, "entity_definition", DataValue.self),
      instructions: try readPresence(object, "instructions", DataValue.self),
      itemSchema: try readPresence(object, "item_schema", InputDeclaration.self),
      kinds: try [String: DataValue].read(jsonRequired(object, "kinds")),
      labelDetails: try readPresence(object, "label_details", [Label].self),
      mode: try readPresence(object, "mode", ReceiptRecognitionMode.self),
      model: try readPresence(object, "model", String.self),
      name: try readPresence(object, "name", ReceiptQuestionName.self),
      on: try readPresence(object, "on", [String].self),
      profile: try readPresence(object, "profile", String.self),
      relationThreshold: try readPresence(object, "relation_threshold", ReceiptThreshold.self),
      relations: try readPresence(object, "relations", [RelationRule].self),
      snippetPieces: try readPresence(object, "snippet_pieces", UInt64.self),
      stageContext: try readPresence(object, "stage_context", ReceiptRecognitionStageContext.self),
      threshold: try ReceiptThreshold.read(jsonRequired(object, "threshold")),
      verb: try Verb.read(jsonRequired(object, "verb")),
      wordingVersion: try readPresence(object, "wording_version", WordingVersion.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "batch", "context_schema", "entity_definition", "instructions", "item_schema", "kinds",
      "label_details", "mode", "model", "name", "on", "profile", "relation_threshold", "relations",
      "snippet_pieces", "stage_context", "threshold", "verb", "wording_version",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`batch`, "batch", &object)
    writePresence(`contextSchema`, "context_schema", &object)
    writePresence(`entityDefinition`, "entity_definition", &object)
    writePresence(`instructions`, "instructions", &object)
    writePresence(`itemSchema`, "item_schema", &object)
    object["kinds"] = `kinds`.json
    writePresence(`labelDetails`, "label_details", &object)
    writePresence(`mode`, "mode", &object)
    writePresence(`model`, "model", &object)
    writePresence(`name`, "name", &object)
    writePresence(`on`, "on", &object)
    writePresence(`profile`, "profile", &object)
    writePresence(`relationThreshold`, "relation_threshold", &object)
    writePresence(`relations`, "relations", &object)
    writePresence(`snippetPieces`, "snippet_pieces", &object)
    writePresence(`stageContext`, "stage_context", &object)
    object["threshold"] = `threshold`.json
    object["verb"] = `verb`.json
    writePresence(`wordingVersion`, "wording_version", &object)
    return .object(object)
  }
}
public struct RecognizeEntities: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `entities`: [Entity]
  public let `relations`: Presence<[EntityEdge]>
  public init(
    extensions: [String: DataValue] = [:], `entities`: [Entity],
    `relations`: Presence<[EntityEdge]> = .absent
  ) {
    self.extensions = extensions
    self.`entities` = `entities`
    self.`relations` = `relations`
  }
  static func read(_ json: DataValue) throws -> RecognizeEntities {
    let object = try jsonObject(json)
    let known: Set<String> = ["entities", "relations"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      entities: try [Entity].read(jsonRequired(object, "entities")),
      relations: try readPresence(object, "relations", [EntityEdge].self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["entities", "relations"]
    var object = extensions.filter { !known.contains($0.key) }
    object["entities"] = `entities`.json
    writePresence(`relations`, "relations", &object)
    return .object(object)
  }
}
public struct RecognizeModeProposals: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `mode`: BoundaryMode
  public let `proposals`: [BoundaryProposal]
  public init(
    extensions: [String: DataValue] = [:], `mode`: BoundaryMode, `proposals`: [BoundaryProposal]
  ) {
    self.extensions = extensions
    self.`mode` = `mode`
    self.`proposals` = `proposals`
  }
  static func read(_ json: DataValue) throws -> RecognizeModeProposals {
    let object = try jsonObject(json)
    let known: Set<String> = ["mode", "proposals"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      mode: try BoundaryMode.read(jsonRequired(object, "mode")),
      proposals: try [BoundaryProposal].read(jsonRequired(object, "proposals")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["mode", "proposals"]
    var object = extensions.filter { !known.contains($0.key) }
    object["mode"] = `mode`.json
    object["proposals"] = `proposals`.json
    return .object(object)
  }
}
public struct ReceiptRelateFields: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `name`: String
  public init(extensions: [String: DataValue] = [:], `kind`: String, `name`: String) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`name` = `name`
  }
  static func read(_ json: DataValue) throws -> ReceiptRelateFields {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "name"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      name: try String.read(jsonRequired(object, "name")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "name"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["name"] = `name`.json
    return .object(object)
  }
}
public struct RelateQuestionDocument: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `batch`: Presence<ReceiptBatch>
  public let `contextSchema`: Presence<InputDeclaration>
  public let `fields`: ReceiptRelateFields?
  public let `itemSchema`: Presence<InputDeclaration>
  public let `labelDetails`: Presence<[Label]>
  public let `model`: Presence<String>
  public let `name`: Presence<ReceiptQuestionName>
  public let `on`: Presence<[String]>
  public let `profile`: Presence<String>
  public let `relations`: [RelationRule]
  public let `threshold`: ReceiptThreshold
  public let `verb`: String
  public let `wordingVersion`: Presence<WordingVersion>
  public init(
    extensions: [String: DataValue] = [:], `batch`: Presence<ReceiptBatch> = .absent,
    `contextSchema`: Presence<InputDeclaration> = .absent, `fields`: ReceiptRelateFields?,
    `itemSchema`: Presence<InputDeclaration> = .absent, `labelDetails`: Presence<[Label]> = .absent,
    `model`: Presence<String> = .absent, `name`: Presence<ReceiptQuestionName> = .absent,
    `on`: Presence<[String]> = .absent, `profile`: Presence<String> = .absent,
    `relations`: [RelationRule], `threshold`: ReceiptThreshold, `verb`: String = "relate",
    `wordingVersion`: Presence<WordingVersion> = .absent
  ) {
    self.extensions = extensions
    self.`batch` = `batch`
    self.`contextSchema` = `contextSchema`
    self.`fields` = `fields`
    self.`itemSchema` = `itemSchema`
    self.`labelDetails` = `labelDetails`
    self.`model` = `model`
    self.`name` = `name`
    self.`on` = `on`
    self.`profile` = `profile`
    self.`relations` = `relations`
    self.`threshold` = `threshold`
    self.`verb` = `verb`
    self.`wordingVersion` = `wordingVersion`
  }
  static func read(_ json: DataValue) throws -> RelateQuestionDocument {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "batch", "context_schema", "fields", "item_schema", "label_details", "model", "name", "on",
      "profile", "relations", "threshold", "verb", "wording_version",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      batch: try readPresence(object, "batch", ReceiptBatch.self),
      contextSchema: try readPresence(object, "context_schema", InputDeclaration.self),
      fields: try ReceiptRelateFields?.read(jsonRequired(object, "fields")),
      itemSchema: try readPresence(object, "item_schema", InputDeclaration.self),
      labelDetails: try readPresence(object, "label_details", [Label].self),
      model: try readPresence(object, "model", String.self),
      name: try readPresence(object, "name", ReceiptQuestionName.self),
      on: try readPresence(object, "on", [String].self),
      profile: try readPresence(object, "profile", String.self),
      relations: try [RelationRule].read(jsonRequired(object, "relations")),
      threshold: try ReceiptThreshold.read(jsonRequired(object, "threshold")),
      verb: try String.read(jsonRequired(object, "verb")),
      wordingVersion: try readPresence(object, "wording_version", WordingVersion.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "batch", "context_schema", "fields", "item_schema", "label_details", "model", "name", "on",
      "profile", "relations", "threshold", "verb", "wording_version",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`batch`, "batch", &object)
    writePresence(`contextSchema`, "context_schema", &object)
    object["fields"] = `fields`.json
    writePresence(`itemSchema`, "item_schema", &object)
    writePresence(`labelDetails`, "label_details", &object)
    writePresence(`model`, "model", &object)
    writePresence(`name`, "name", &object)
    writePresence(`on`, "on", &object)
    writePresence(`profile`, "profile", &object)
    object["relations"] = `relations`.json
    object["threshold"] = `threshold`.json
    object["verb"] = `verb`.json
    writePresence(`wordingVersion`, "wording_version", &object)
    return .object(object)
  }
}
public struct RelatedEntity: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `name`: String
  public init(extensions: [String: DataValue] = [:], `kind`: String, `name`: String) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`name` = `name`
  }
  static func read(_ json: DataValue) throws -> RelatedEntity {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "name"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      name: try String.read(jsonRequired(object, "name")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "name"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["name"] = `name`.json
    return .object(object)
  }
}
public struct RelatedEntityEdge: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `either`: Presence<Bool>
  public let `probability`: Double
  public let `relation`: String
  public let `source`: RelationEndpoint
  public let `target`: RelationEndpoint
  public init(
    extensions: [String: DataValue] = [:], `either`: Presence<Bool> = .absent,
    `probability`: Double, `relation`: String, `source`: RelationEndpoint,
    `target`: RelationEndpoint
  ) {
    self.extensions = extensions
    self.`either` = `either`
    self.`probability` = `probability`
    self.`relation` = `relation`
    self.`source` = `source`
    self.`target` = `target`
  }
  static func read(_ json: DataValue) throws -> RelatedEntityEdge {
    let object = try jsonObject(json)
    let known: Set<String> = ["either", "probability", "relation", "source", "target"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      either: try readPresence(object, "either", Bool.self),
      probability: try Double.read(jsonRequired(object, "probability")),
      relation: try String.read(jsonRequired(object, "relation")),
      source: try RelationEndpoint.read(jsonRequired(object, "source")),
      target: try RelationEndpoint.read(jsonRequired(object, "target")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["either", "probability", "relation", "source", "target"]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`either`, "either", &object)
    object["probability"] = `probability`.json
    object["relation"] = `relation`.json
    object["source"] = `source`.json
    object["target"] = `target`.json
    return .object(object)
  }
  public func sourceName() throws -> Presence<String> {
    switch try readPresence(jsonObject(self.json), "source", DataValue.self) {
    case .absent: return .absent
    case .null: return .null
    case .value(let object): return try readPresence(jsonObject(object), "name", String.self)
    }
  }
  public func targetName() throws -> Presence<String> {
    switch try readPresence(jsonObject(self.json), "target", DataValue.self) {
    case .absent: return .absent
    case .null: return .null
    case .value(let object): return try readPresence(jsonObject(object), "name", String.self)
    }
  }
}
public struct RelationRule: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `either`: Bool
  public let `name`: String
  public let `reads`: String
  public let `single`: Presence<Bool>
  public let `source`: String
  public let `target`: String
  public init(
    extensions: [String: DataValue] = [:], `either`: Bool, `name`: String, `reads`: String,
    `single`: Presence<Bool> = .absent, `source`: String, `target`: String
  ) {
    self.extensions = extensions
    self.`either` = `either`
    self.`name` = `name`
    self.`reads` = `reads`
    self.`single` = `single`
    self.`source` = `source`
    self.`target` = `target`
  }
  static func read(_ json: DataValue) throws -> RelationRule {
    let object = try jsonObject(json)
    let known: Set<String> = ["either", "name", "reads", "single", "source", "target"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      either: try Bool.read(jsonRequired(object, "either")),
      name: try String.read(jsonRequired(object, "name")),
      reads: try String.read(jsonRequired(object, "reads")),
      single: try readPresence(object, "single", Bool.self),
      source: try String.read(jsonRequired(object, "source")),
      target: try String.read(jsonRequired(object, "target")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["either", "name", "reads", "single", "source", "target"]
    var object = extensions.filter { !known.contains($0.key) }
    object["either"] = `either`.json
    object["name"] = `name`.json
    object["reads"] = `reads`.json
    writePresence(`single`, "single", &object)
    object["source"] = `source`.json
    object["target"] = `target`.json
    return .object(object)
  }
}
public struct SessionAnnotation: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `name`: String
  public let `value`: AnnotationValue
  public init(extensions: [String: DataValue] = [:], `name`: String, `value`: AnnotationValue) {
    self.extensions = extensions
    self.`name` = `name`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> SessionAnnotation {
    let object = try jsonObject(json)
    let known: Set<String> = ["name", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      name: try String.read(jsonRequired(object, "name")),
      value: try AnnotationValue.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["name", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["name"] = `name`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionInputSource: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `index`: UInt64
  public let `source`: PhysicalSource
  public init(extensions: [String: DataValue] = [:], `index`: UInt64, `source`: PhysicalSource) {
    self.extensions = extensions
    self.`index` = `index`
    self.`source` = `source`
  }
  static func read(_ json: DataValue) throws -> SessionInputSource {
    let object = try jsonObject(json)
    let known: Set<String> = ["index", "source"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      index: try UInt64.read(jsonRequired(object, "index")),
      source: try PhysicalSource.read(jsonRequired(object, "source")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["index", "source"]
    var object = extensions.filter { !known.contains($0.key) }
    object["index"] = `index`.json
    object["source"] = `source`.json
    return .object(object)
  }
}
public indirect enum SessionJudgment: ValueRepresentable {
  case `decision`(SessionJudgmentDecision)
  case `choice`(SessionJudgmentChoice)
  case `score`(SessionJudgmentScore)
  case `tags`(SessionJudgmentTags)
  static func read(_ json: DataValue) throws -> SessionJudgment {
    let object = try jsonObject(json)
    if object["kind"] == .string("decision") {
      return .`decision`(try SessionJudgmentDecision.read(json))
    }
    if object["kind"] == .string("choice") {
      return .`choice`(try SessionJudgmentChoice.read(json))
    }
    if object["kind"] == .string("score") { return .`score`(try SessionJudgmentScore.read(json)) }
    if object["kind"] == .string("tags") { return .`tags`(try SessionJudgmentTags.read(json)) }
    throw ValueConversionError("Unknown SessionJudgment alternative")
  }
  var json: DataValue {
    switch self {
    case .`decision`(let value): return value.json
    case .`choice`(let value): return value.json
    case .`score`(let value): return value.json
    case .`tags`(let value): return value.json
    }
  }
}
public struct SessionJudgmentChoice: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `value`: String?
  public init(extensions: [String: DataValue] = [:], `kind`: String = "choice", `value`: String?) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> SessionJudgmentChoice {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try String?.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionJudgmentDecision: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `value`: Bool?
  public init(extensions: [String: DataValue] = [:], `kind`: String = "decision", `value`: Bool?) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> SessionJudgmentDecision {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try Bool?.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionJudgmentScore: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `value`: Double
  public init(extensions: [String: DataValue] = [:], `kind`: String = "score", `value`: Double) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> SessionJudgmentScore {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try Double.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionJudgmentTags: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `value`: [String]
  public init(extensions: [String: DataValue] = [:], `kind`: String = "tags", `value`: [String]) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> SessionJudgmentTags {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try [String].read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionNamedProbability: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `name`: String
  public let `probability`: Double
  public init(extensions: [String: DataValue] = [:], `name`: String, `probability`: Double) {
    self.extensions = extensions
    self.`name` = `name`
    self.`probability` = `probability`
  }
  static func read(_ json: DataValue) throws -> SessionNamedProbability {
    let object = try jsonObject(json)
    let known: Set<String> = ["name", "probability"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      name: try String.read(jsonRequired(object, "name")),
      probability: try Double.read(jsonRequired(object, "probability")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["name", "probability"]
    var object = extensions.filter { !known.contains($0.key) }
    object["name"] = `name`.json
    object["probability"] = `probability`.json
    return .object(object)
  }
}
public indirect enum SessionObservation: ValueRepresentable {
  case `question`(SessionObservationQuestion)
  case `row`(SessionObservationRow)
  static func read(_ json: DataValue) throws -> SessionObservation {
    let object = try jsonObject(json)
    if object["kind"] == .string("question") {
      return .`question`(try SessionObservationQuestion.read(json))
    }
    if object["kind"] == .string("row") { return .`row`(try SessionObservationRow.read(json)) }
    throw ValueConversionError("Unknown SessionObservation alternative")
  }
  var json: DataValue {
    switch self {
    case .`question`(let value): return value.json
    case .`row`(let value): return value.json
    }
  }
}
public struct SessionObservationQuestion: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `detail`: SessionQuestionDetail
  public let `index`: UInt64
  public let `kind`: String
  public let `member`: Presence<String>
  public let `position`: UInt64
  public let `stage`: Presence<String>
  public init(
    extensions: [String: DataValue] = [:], `detail`: SessionQuestionDetail, `index`: UInt64,
    `kind`: String = "question", `member`: Presence<String> = .absent, `position`: UInt64,
    `stage`: Presence<String> = .absent
  ) {
    self.extensions = extensions
    self.`detail` = `detail`
    self.`index` = `index`
    self.`kind` = `kind`
    self.`member` = `member`
    self.`position` = `position`
    self.`stage` = `stage`
  }
  static func read(_ json: DataValue) throws -> SessionObservationQuestion {
    let object = try jsonObject(json)
    let known: Set<String> = ["detail", "index", "kind", "member", "position", "stage"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      detail: try SessionQuestionDetail.read(jsonRequired(object, "detail")),
      index: try UInt64.read(jsonRequired(object, "index")),
      kind: try String.read(jsonRequired(object, "kind")),
      member: try readPresence(object, "member", String.self),
      position: try UInt64.read(jsonRequired(object, "position")),
      stage: try readPresence(object, "stage", String.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["detail", "index", "kind", "member", "position", "stage"]
    var object = extensions.filter { !known.contains($0.key) }
    object["detail"] = `detail`.json
    object["index"] = `index`.json
    object["kind"] = `kind`.json
    writePresence(`member`, "member", &object)
    object["position"] = `position`.json
    writePresence(`stage`, "stage", &object)
    return .object(object)
  }
}
public struct SessionObservationRow: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `index`: UInt64
  public let `kind`: String
  public let `value`: SessionObservedRow
  public init(
    extensions: [String: DataValue] = [:], `index`: UInt64, `kind`: String = "row",
    `value`: SessionObservedRow
  ) {
    self.extensions = extensions
    self.`index` = `index`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> SessionObservationRow {
    let object = try jsonObject(json)
    let known: Set<String> = ["index", "kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      index: try UInt64.read(jsonRequired(object, "index")),
      kind: try String.read(jsonRequired(object, "kind")),
      value: try SessionObservedRow.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["index", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["index"] = `index`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public indirect enum SessionObservedRow: ValueRepresentable {
  case `judgment`(SessionObservedRowJudgment)
  case `annotated`(SessionObservedRowAnnotated)
  case `recognized`(SessionObservedRowRecognized)
  case `find`(SessionObservedRowFind)
  case `relations`(SessionObservedRowRelations)
  static func read(_ json: DataValue) throws -> SessionObservedRow {
    let object = try jsonObject(json)
    if object["kind"] == .string("judgment") {
      return .`judgment`(try SessionObservedRowJudgment.read(json))
    }
    if object["kind"] == .string("annotated") {
      return .`annotated`(try SessionObservedRowAnnotated.read(json))
    }
    if object["kind"] == .string("recognized") {
      return .`recognized`(try SessionObservedRowRecognized.read(json))
    }
    if object["kind"] == .string("find") { return .`find`(try SessionObservedRowFind.read(json)) }
    if object["kind"] == .string("relations") {
      return .`relations`(try SessionObservedRowRelations.read(json))
    }
    throw ValueConversionError("Unknown SessionObservedRow alternative")
  }
  var json: DataValue {
    switch self {
    case .`judgment`(let value): return value.json
    case .`annotated`(let value): return value.json
    case .`recognized`(let value): return value.json
    case .`find`(let value): return value.json
    case .`relations`(let value): return value.json
    }
  }
}
public struct SessionObservedRowAnnotated: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `value`: [SessionAnnotation]
  public init(
    extensions: [String: DataValue] = [:], `kind`: String = "annotated",
    `value`: [SessionAnnotation]
  ) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> SessionObservedRowAnnotated {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try [SessionAnnotation].read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionObservedRowFind: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `value`: UInt64?
  public init(extensions: [String: DataValue] = [:], `kind`: String = "find", `value`: UInt64?) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> SessionObservedRowFind {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try UInt64?.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionObservedRowJudgment: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `value`: SessionJudgment
  public init(
    extensions: [String: DataValue] = [:], `kind`: String = "judgment", `value`: SessionJudgment
  ) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> SessionObservedRowJudgment {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try SessionJudgment.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionObservedRowRecognized: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `value`: SessionRecognition
  public init(
    extensions: [String: DataValue] = [:], `kind`: String = "recognized",
    `value`: SessionRecognition
  ) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> SessionObservedRowRecognized {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try SessionRecognition.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionObservedRowRelations: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `value`: [SessionRelationEdge]
  public init(
    extensions: [String: DataValue] = [:], `kind`: String = "relations",
    `value`: [SessionRelationEdge]
  ) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> SessionObservedRowRelations {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try [SessionRelationEdge].read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public indirect enum SessionPacket: ValueRepresentable {
  case `decideRow`(SessionPacketDecideRow)
  case `chooseRow`(SessionPacketChooseRow)
  case `tagRow`(SessionPacketTagRow)
  case `scoreRow`(SessionPacketScoreRow)
  case `filterRow`(SessionPacketFilterRow)
  case `annotateRow`(SessionPacketAnnotateRow)
  case `decideAggregate`(SessionPacketDecideAggregate)
  case `chooseAggregate`(SessionPacketChooseAggregate)
  case `tagAggregate`(SessionPacketTagAggregate)
  case `scoreAggregate`(SessionPacketScoreAggregate)
  case `filterAggregate`(SessionPacketFilterAggregate)
  case `rankAggregate`(SessionPacketRankAggregate)
  case `findAggregate`(SessionPacketFindAggregate)
  case `annotateAggregate`(SessionPacketAnnotateAggregate)
  case `recognizeAggregate`(SessionPacketRecognizeAggregate)
  case `relateAggregate`(SessionPacketRelateAggregate)
  case `observation`(SessionPacketObservation)
  case `terminal`(SessionPacketTerminal)
  static func read(_ json: DataValue) throws -> SessionPacket {
    let object = try jsonObject(json)
    if object["function"] == .string("decide") && object["kind"] == .string("row") {
      return .`decideRow`(try SessionPacketDecideRow.read(json))
    }
    if object["function"] == .string("choose") && object["kind"] == .string("row") {
      return .`chooseRow`(try SessionPacketChooseRow.read(json))
    }
    if object["function"] == .string("tag") && object["kind"] == .string("row") {
      return .`tagRow`(try SessionPacketTagRow.read(json))
    }
    if object["function"] == .string("score") && object["kind"] == .string("row") {
      return .`scoreRow`(try SessionPacketScoreRow.read(json))
    }
    if object["function"] == .string("filter") && object["kind"] == .string("row") {
      return .`filterRow`(try SessionPacketFilterRow.read(json))
    }
    if object["function"] == .string("annotate") && object["kind"] == .string("row") {
      return .`annotateRow`(try SessionPacketAnnotateRow.read(json))
    }
    if object["function"] == .string("decide") && object["kind"] == .string("aggregate") {
      return .`decideAggregate`(try SessionPacketDecideAggregate.read(json))
    }
    if object["function"] == .string("choose") && object["kind"] == .string("aggregate") {
      return .`chooseAggregate`(try SessionPacketChooseAggregate.read(json))
    }
    if object["function"] == .string("tag") && object["kind"] == .string("aggregate") {
      return .`tagAggregate`(try SessionPacketTagAggregate.read(json))
    }
    if object["function"] == .string("score") && object["kind"] == .string("aggregate") {
      return .`scoreAggregate`(try SessionPacketScoreAggregate.read(json))
    }
    if object["function"] == .string("filter") && object["kind"] == .string("aggregate") {
      return .`filterAggregate`(try SessionPacketFilterAggregate.read(json))
    }
    if object["function"] == .string("rank") && object["kind"] == .string("aggregate") {
      return .`rankAggregate`(try SessionPacketRankAggregate.read(json))
    }
    if object["function"] == .string("find") && object["kind"] == .string("aggregate") {
      return .`findAggregate`(try SessionPacketFindAggregate.read(json))
    }
    if object["function"] == .string("annotate") && object["kind"] == .string("aggregate") {
      return .`annotateAggregate`(try SessionPacketAnnotateAggregate.read(json))
    }
    if object["function"] == .string("recognize") && object["kind"] == .string("aggregate") {
      return .`recognizeAggregate`(try SessionPacketRecognizeAggregate.read(json))
    }
    if object["function"] == .string("relate") && object["kind"] == .string("aggregate") {
      return .`relateAggregate`(try SessionPacketRelateAggregate.read(json))
    }
    if object["kind"] == .string("observation") {
      return .`observation`(try SessionPacketObservation.read(json))
    }
    if object["kind"] == .string("terminal") {
      return .`terminal`(try SessionPacketTerminal.read(json))
    }
    throw ValueConversionError("Unknown SessionPacket alternative")
  }
  var json: DataValue {
    switch self {
    case .`decideRow`(let value): return value.json
    case .`chooseRow`(let value): return value.json
    case .`tagRow`(let value): return value.json
    case .`scoreRow`(let value): return value.json
    case .`filterRow`(let value): return value.json
    case .`annotateRow`(let value): return value.json
    case .`decideAggregate`(let value): return value.json
    case .`chooseAggregate`(let value): return value.json
    case .`tagAggregate`(let value): return value.json
    case .`scoreAggregate`(let value): return value.json
    case .`filterAggregate`(let value): return value.json
    case .`rankAggregate`(let value): return value.json
    case .`findAggregate`(let value): return value.json
    case .`annotateAggregate`(let value): return value.json
    case .`recognizeAggregate`(let value): return value.json
    case .`relateAggregate`(let value): return value.json
    case .`observation`(let value): return value.json
    case .`terminal`(let value): return value.json
    }
  }
}
public struct SessionPacketAnnotateAggregate: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `function`: String
  public let `kind`: String
  public let `value`: [Annotation]
  public init(
    extensions: [String: DataValue] = [:], `function`: String = "annotate",
    `kind`: String = "aggregate", `value`: [Annotation]
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> SessionPacketAnnotateAggregate {
    let object = try jsonObject(json)
    let known: Set<String> = ["function", "kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      function: try String.read(jsonRequired(object, "function")),
      kind: try String.read(jsonRequired(object, "kind")),
      value: try [Annotation].read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketAnnotateRow: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `function`: String
  public let `kind`: String
  public let `value`: Annotation
  public init(
    extensions: [String: DataValue] = [:], `function`: String = "annotate", `kind`: String = "row",
    `value`: Annotation
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> SessionPacketAnnotateRow {
    let object = try jsonObject(json)
    let known: Set<String> = ["function", "kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      function: try String.read(jsonRequired(object, "function")),
      kind: try String.read(jsonRequired(object, "kind")),
      value: try Annotation.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketChooseAggregate: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `function`: String
  public let `kind`: String
  public let `value`: [AtomicNullableString]
  public init(
    extensions: [String: DataValue] = [:], `function`: String = "choose",
    `kind`: String = "aggregate", `value`: [AtomicNullableString]
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> SessionPacketChooseAggregate {
    let object = try jsonObject(json)
    let known: Set<String> = ["function", "kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      function: try String.read(jsonRequired(object, "function")),
      kind: try String.read(jsonRequired(object, "kind")),
      value: try [AtomicNullableString].read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketChooseRow: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `function`: String
  public let `kind`: String
  public let `value`: AtomicNullableString
  public init(
    extensions: [String: DataValue] = [:], `function`: String = "choose", `kind`: String = "row",
    `value`: AtomicNullableString
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> SessionPacketChooseRow {
    let object = try jsonObject(json)
    let known: Set<String> = ["function", "kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      function: try String.read(jsonRequired(object, "function")),
      kind: try String.read(jsonRequired(object, "kind")),
      value: try AtomicNullableString.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketDecideAggregate: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `function`: String
  public let `kind`: String
  public let `value`: [AtomicDecideValue]
  public init(
    extensions: [String: DataValue] = [:], `function`: String = "decide",
    `kind`: String = "aggregate", `value`: [AtomicDecideValue]
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> SessionPacketDecideAggregate {
    let object = try jsonObject(json)
    let known: Set<String> = ["function", "kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      function: try String.read(jsonRequired(object, "function")),
      kind: try String.read(jsonRequired(object, "kind")),
      value: try [AtomicDecideValue].read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketDecideRow: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `function`: String
  public let `kind`: String
  public let `value`: AtomicDecideValue
  public init(
    extensions: [String: DataValue] = [:], `function`: String = "decide", `kind`: String = "row",
    `value`: AtomicDecideValue
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> SessionPacketDecideRow {
    let object = try jsonObject(json)
    let known: Set<String> = ["function", "kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      function: try String.read(jsonRequired(object, "function")),
      kind: try String.read(jsonRequired(object, "kind")),
      value: try AtomicDecideValue.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketFilterAggregate: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `function`: String
  public let `kind`: String
  public let `value`: [AtomicBoolean]
  public init(
    extensions: [String: DataValue] = [:], `function`: String = "filter",
    `kind`: String = "aggregate", `value`: [AtomicBoolean]
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> SessionPacketFilterAggregate {
    let object = try jsonObject(json)
    let known: Set<String> = ["function", "kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      function: try String.read(jsonRequired(object, "function")),
      kind: try String.read(jsonRequired(object, "kind")),
      value: try [AtomicBoolean].read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketFilterRow: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `function`: String
  public let `kind`: String
  public let `value`: AtomicBoolean
  public init(
    extensions: [String: DataValue] = [:], `function`: String = "filter", `kind`: String = "row",
    `value`: AtomicBoolean
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> SessionPacketFilterRow {
    let object = try jsonObject(json)
    let known: Set<String> = ["function", "kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      function: try String.read(jsonRequired(object, "function")),
      kind: try String.read(jsonRequired(object, "kind")),
      value: try AtomicBoolean.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketFindAggregate: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `function`: String
  public let `kind`: String
  public let `value`: Find
  public init(
    extensions: [String: DataValue] = [:], `function`: String = "find",
    `kind`: String = "aggregate", `value`: Find
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> SessionPacketFindAggregate {
    let object = try jsonObject(json)
    let known: Set<String> = ["function", "kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      function: try String.read(jsonRequired(object, "function")),
      kind: try String.read(jsonRequired(object, "kind")),
      value: try Find.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketObservation: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `function`: ReceiptFunction
  public let `kind`: String
  public let `value`: SessionObservation
  public init(
    extensions: [String: DataValue] = [:], `function`: ReceiptFunction,
    `kind`: String = "observation", `value`: SessionObservation
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> SessionPacketObservation {
    let object = try jsonObject(json)
    let known: Set<String> = ["function", "kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      function: try ReceiptFunction.read(jsonRequired(object, "function")),
      kind: try String.read(jsonRequired(object, "kind")),
      value: try SessionObservation.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketRankAggregate: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `function`: String
  public let `kind`: String
  public let `value`: [AtomicNonZeroUsize]
  public init(
    extensions: [String: DataValue] = [:], `function`: String = "rank",
    `kind`: String = "aggregate", `value`: [AtomicNonZeroUsize]
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> SessionPacketRankAggregate {
    let object = try jsonObject(json)
    let known: Set<String> = ["function", "kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      function: try String.read(jsonRequired(object, "function")),
      kind: try String.read(jsonRequired(object, "kind")),
      value: try [AtomicNonZeroUsize].read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketRecognizeAggregate: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `function`: String
  public let `kind`: String
  public let `value`: [Recognition]
  public init(
    extensions: [String: DataValue] = [:], `function`: String = "recognize",
    `kind`: String = "aggregate", `value`: [Recognition]
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> SessionPacketRecognizeAggregate {
    let object = try jsonObject(json)
    let known: Set<String> = ["function", "kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      function: try String.read(jsonRequired(object, "function")),
      kind: try String.read(jsonRequired(object, "kind")),
      value: try [Recognition].read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketRelateAggregate: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `function`: String
  public let `kind`: String
  public let `value`: Relation
  public init(
    extensions: [String: DataValue] = [:], `function`: String = "relate",
    `kind`: String = "aggregate", `value`: Relation
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> SessionPacketRelateAggregate {
    let object = try jsonObject(json)
    let known: Set<String> = ["function", "kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      function: try String.read(jsonRequired(object, "function")),
      kind: try String.read(jsonRequired(object, "kind")),
      value: try Relation.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketScoreAggregate: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `function`: String
  public let `kind`: String
  public let `value`: [AtomicDouble]
  public init(
    extensions: [String: DataValue] = [:], `function`: String = "score",
    `kind`: String = "aggregate", `value`: [AtomicDouble]
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> SessionPacketScoreAggregate {
    let object = try jsonObject(json)
    let known: Set<String> = ["function", "kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      function: try String.read(jsonRequired(object, "function")),
      kind: try String.read(jsonRequired(object, "kind")),
      value: try [AtomicDouble].read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketScoreRow: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `function`: String
  public let `kind`: String
  public let `value`: AtomicDouble
  public init(
    extensions: [String: DataValue] = [:], `function`: String = "score", `kind`: String = "row",
    `value`: AtomicDouble
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> SessionPacketScoreRow {
    let object = try jsonObject(json)
    let known: Set<String> = ["function", "kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      function: try String.read(jsonRequired(object, "function")),
      kind: try String.read(jsonRequired(object, "kind")),
      value: try AtomicDouble.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketTagAggregate: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `function`: String
  public let `kind`: String
  public let `value`: [AtomicArrayOfString]
  public init(
    extensions: [String: DataValue] = [:], `function`: String = "tag", `kind`: String = "aggregate",
    `value`: [AtomicArrayOfString]
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> SessionPacketTagAggregate {
    let object = try jsonObject(json)
    let known: Set<String> = ["function", "kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      function: try String.read(jsonRequired(object, "function")),
      kind: try String.read(jsonRequired(object, "kind")),
      value: try [AtomicArrayOfString].read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketTagRow: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `function`: String
  public let `kind`: String
  public let `value`: AtomicArrayOfString
  public init(
    extensions: [String: DataValue] = [:], `function`: String = "tag", `kind`: String = "row",
    `value`: AtomicArrayOfString
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> SessionPacketTagRow {
    let object = try jsonObject(json)
    let known: Set<String> = ["function", "kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      function: try String.read(jsonRequired(object, "function")),
      kind: try String.read(jsonRequired(object, "kind")),
      value: try AtomicArrayOfString.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketTerminal: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `facts`: Presence<Facts>
  public let `failure`: Presence<CallError>
  public let `kind`: String
  public init(
    extensions: [String: DataValue] = [:], `facts`: Presence<Facts> = .absent,
    `failure`: Presence<CallError> = .absent, `kind`: String = "terminal"
  ) {
    self.extensions = extensions
    self.`facts` = `facts`
    self.`failure` = `failure`
    self.`kind` = `kind`
  }
  static func read(_ json: DataValue) throws -> SessionPacketTerminal {
    let object = try jsonObject(json)
    let known: Set<String> = ["facts", "failure", "kind"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      facts: try readPresence(object, "facts", Facts.self),
      failure: try readPresence(object, "failure", CallError.self),
      kind: try String.read(jsonRequired(object, "kind")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["facts", "failure", "kind"]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`facts`, "facts", &object)
    writePresence(`failure`, "failure", &object)
    object["kind"] = `kind`.json
    return .object(object)
  }
}
public indirect enum SessionProbabilities: ValueRepresentable {
  case `yesNo`(SessionProbabilitiesYesNo)
  case `named`(SessionProbabilitiesNamed)
  static func read(_ json: DataValue) throws -> SessionProbabilities {
    let object = try jsonObject(json)
    if object["kind"] == .string("yes_no") {
      return .`yesNo`(try SessionProbabilitiesYesNo.read(json))
    }
    if object["kind"] == .string("named") {
      return .`named`(try SessionProbabilitiesNamed.read(json))
    }
    throw ValueConversionError("Unknown SessionProbabilities alternative")
  }
  var json: DataValue {
    switch self {
    case .`yesNo`(let value): return value.json
    case .`named`(let value): return value.json
    }
  }
}
public struct SessionProbabilitiesNamed: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `value`: [SessionNamedProbability]
  public init(
    extensions: [String: DataValue] = [:], `kind`: String = "named",
    `value`: [SessionNamedProbability]
  ) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> SessionProbabilitiesNamed {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try [SessionNamedProbability].read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionProbabilitiesYesNo: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `value`: Double
  public init(extensions: [String: DataValue] = [:], `kind`: String = "yes_no", `value`: Double) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> SessionProbabilitiesYesNo {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try Double.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionQuestionDetail: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `answerId`: Presence<AnswerId>
  public let `cached`: Bool
  public let `confidence`: Presence<Double>
  public let `failedQuestions`: UInt64
  public let `failure`: Presence<Failure>
  public let `failureId`: Presence<FailureId>
  public let `input`: Presence<DataValue>
  public let `inputSource`: Presence<PhysicalSource>
  public let `inputSources`: [SessionInputSource]
  public let `inputs`: [DataValue]
  public let `model`: String
  public let `observations`: [Observation]
  public let `probabilities`: Presence<SessionProbabilities>
  public let `question`: QuestionDocument
  public let `questionSha256`: String
  public let `questionSources`: [QuestionSource]
  public let `rawPick`: Presence<String>
  public let `reportedUsage`: Presence<Usage>
  public let `requests`: [String]
  public let `requestsSent`: UInt64
  public let `threshold`: Presence<ReceiptThreshold>
  public let `url`: String
  public let `usage`: Presence<TokenUsage>
  public let `value`: Presence<Value>
  public init(
    extensions: [String: DataValue] = [:], `answerId`: Presence<AnswerId> = .absent, `cached`: Bool,
    `confidence`: Presence<Double> = .absent, `failedQuestions`: UInt64,
    `failure`: Presence<Failure> = .absent, `failureId`: Presence<FailureId> = .absent,
    `input`: Presence<DataValue> = .absent, `inputSource`: Presence<PhysicalSource> = .absent,
    `inputSources`: [SessionInputSource], `inputs`: [DataValue], `model`: String,
    `observations`: [Observation], `probabilities`: Presence<SessionProbabilities> = .absent,
    `question`: QuestionDocument, `questionSha256`: String, `questionSources`: [QuestionSource],
    `rawPick`: Presence<String> = .absent, `reportedUsage`: Presence<Usage> = .absent,
    `requests`: [String], `requestsSent`: UInt64, `threshold`: Presence<ReceiptThreshold> = .absent,
    `url`: String, `usage`: Presence<TokenUsage> = .absent, `value`: Presence<Value> = .absent
  ) {
    self.extensions = extensions
    self.`answerId` = `answerId`
    self.`cached` = `cached`
    self.`confidence` = `confidence`
    self.`failedQuestions` = `failedQuestions`
    self.`failure` = `failure`
    self.`failureId` = `failureId`
    self.`input` = `input`
    self.`inputSource` = `inputSource`
    self.`inputSources` = `inputSources`
    self.`inputs` = `inputs`
    self.`model` = `model`
    self.`observations` = `observations`
    self.`probabilities` = `probabilities`
    self.`question` = `question`
    self.`questionSha256` = `questionSha256`
    self.`questionSources` = `questionSources`
    self.`rawPick` = `rawPick`
    self.`reportedUsage` = `reportedUsage`
    self.`requests` = `requests`
    self.`requestsSent` = `requestsSent`
    self.`threshold` = `threshold`
    self.`url` = `url`
    self.`usage` = `usage`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> SessionQuestionDetail {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "answer_id", "cached", "confidence", "failed_questions", "failure", "failure_id", "input",
      "input_source", "input_sources", "inputs", "model", "observations", "probabilities",
      "question", "question_sha256", "question_sources", "raw_pick", "reported_usage", "requests",
      "requests_sent", "threshold", "url", "usage", "value",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      answerId: try readPresence(object, "answer_id", AnswerId.self),
      cached: try Bool.read(jsonRequired(object, "cached")),
      confidence: try readPresence(object, "confidence", Double.self),
      failedQuestions: try UInt64.read(jsonRequired(object, "failed_questions")),
      failure: try readPresence(object, "failure", Failure.self),
      failureId: try readPresence(object, "failure_id", FailureId.self),
      input: try readPresence(object, "input", DataValue.self),
      inputSource: try readPresence(object, "input_source", PhysicalSource.self),
      inputSources: try [SessionInputSource].read(jsonRequired(object, "input_sources")),
      inputs: try [DataValue].read(jsonRequired(object, "inputs")),
      model: try String.read(jsonRequired(object, "model")),
      observations: try [Observation].read(jsonRequired(object, "observations")),
      probabilities: try readPresence(object, "probabilities", SessionProbabilities.self),
      question: try QuestionDocument.read(jsonRequired(object, "question")),
      questionSha256: try String.read(jsonRequired(object, "question_sha256")),
      questionSources: try [QuestionSource].read(jsonRequired(object, "question_sources")),
      rawPick: try readPresence(object, "raw_pick", String.self),
      reportedUsage: try readPresence(object, "reported_usage", Usage.self),
      requests: try [String].read(jsonRequired(object, "requests")),
      requestsSent: try UInt64.read(jsonRequired(object, "requests_sent")),
      threshold: try readPresence(object, "threshold", ReceiptThreshold.self),
      url: try String.read(jsonRequired(object, "url")),
      usage: try readPresence(object, "usage", TokenUsage.self),
      value: try readPresence(object, "value", Value.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "answer_id", "cached", "confidence", "failed_questions", "failure", "failure_id", "input",
      "input_source", "input_sources", "inputs", "model", "observations", "probabilities",
      "question", "question_sha256", "question_sources", "raw_pick", "reported_usage", "requests",
      "requests_sent", "threshold", "url", "usage", "value",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`answerId`, "answer_id", &object)
    object["cached"] = `cached`.json
    writePresence(`confidence`, "confidence", &object)
    object["failed_questions"] = `failedQuestions`.json
    writePresence(`failure`, "failure", &object)
    writePresence(`failureId`, "failure_id", &object)
    writePresence(`input`, "input", &object)
    writePresence(`inputSource`, "input_source", &object)
    object["input_sources"] = `inputSources`.json
    object["inputs"] = `inputs`.json
    object["model"] = `model`.json
    object["observations"] = `observations`.json
    writePresence(`probabilities`, "probabilities", &object)
    object["question"] = `question`.json
    object["question_sha256"] = `questionSha256`.json
    object["question_sources"] = `questionSources`.json
    writePresence(`rawPick`, "raw_pick", &object)
    writePresence(`reportedUsage`, "reported_usage", &object)
    object["requests"] = `requests`.json
    object["requests_sent"] = `requestsSent`.json
    writePresence(`threshold`, "threshold", &object)
    object["url"] = `url`.json
    writePresence(`usage`, "usage", &object)
    writePresence(`value`, "value", &object)
    return .object(object)
  }
}
public struct SessionRecognition: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `entities`: [Entity]
  public let `mode`: ReceiptRecognitionMode
  public let `proposals`: Presence<[BoundaryProposal]>
  public let `relations`: Presence<[RecognitionEdgeDocument]>
  public init(
    extensions: [String: DataValue] = [:], `entities`: [Entity], `mode`: ReceiptRecognitionMode,
    `proposals`: Presence<[BoundaryProposal]> = .absent,
    `relations`: Presence<[RecognitionEdgeDocument]> = .absent
  ) {
    self.extensions = extensions
    self.`entities` = `entities`
    self.`mode` = `mode`
    self.`proposals` = `proposals`
    self.`relations` = `relations`
  }
  static func read(_ json: DataValue) throws -> SessionRecognition {
    let object = try jsonObject(json)
    let known: Set<String> = ["entities", "mode", "proposals", "relations"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      entities: try [Entity].read(jsonRequired(object, "entities")),
      mode: try ReceiptRecognitionMode.read(jsonRequired(object, "mode")),
      proposals: try readPresence(object, "proposals", [BoundaryProposal].self),
      relations: try readPresence(object, "relations", [RecognitionEdgeDocument].self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["entities", "mode", "proposals", "relations"]
    var object = extensions.filter { !known.contains($0.key) }
    object["entities"] = `entities`.json
    object["mode"] = `mode`.json
    writePresence(`proposals`, "proposals", &object)
    writePresence(`relations`, "relations", &object)
    return .object(object)
  }
}
public struct SessionRelationEdge: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `either`: Bool
  public let `probability`: Double
  public let `relation`: String
  public let `source`: EntityDocument
  public let `target`: EntityDocument
  public init(
    extensions: [String: DataValue] = [:], `either`: Bool, `probability`: Double,
    `relation`: String, `source`: EntityDocument, `target`: EntityDocument
  ) {
    self.extensions = extensions
    self.`either` = `either`
    self.`probability` = `probability`
    self.`relation` = `relation`
    self.`source` = `source`
    self.`target` = `target`
  }
  static func read(_ json: DataValue) throws -> SessionRelationEdge {
    let object = try jsonObject(json)
    let known: Set<String> = ["either", "probability", "relation", "source", "target"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      either: try Bool.read(jsonRequired(object, "either")),
      probability: try Double.read(jsonRequired(object, "probability")),
      relation: try String.read(jsonRequired(object, "relation")),
      source: try EntityDocument.read(jsonRequired(object, "source")),
      target: try EntityDocument.read(jsonRequired(object, "target")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["either", "probability", "relation", "source", "target"]
    var object = extensions.filter { !known.contains($0.key) }
    object["either"] = `either`.json
    object["probability"] = `probability`.json
    object["relation"] = `relation`.json
    object["source"] = `source`.json
    object["target"] = `target`.json
    return .object(object)
  }
  public func sourceName() throws -> Presence<String> {
    switch try readPresence(jsonObject(self.json), "source", DataValue.self) {
    case .absent: return .absent
    case .null: return .null
    case .value(let object): return try readPresence(jsonObject(object), "name", String.self)
    }
  }
  public func targetName() throws -> Presence<String> {
    switch try readPresence(jsonObject(self.json), "target", DataValue.self) {
    case .absent: return .absent
    case .null: return .null
    case .value(let object): return try readPresence(jsonObject(object), "name", String.self)
    }
  }
}
public struct SourceRelationEndpoint: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `file`: String?
  public let `firstLine`: Presence<UInt64>
  public let `kind`: String
  public let `lastLine`: Presence<UInt64>
  public let `name`: String
  public let `ordinal`: UInt64
  public let `record`: DataValue
  public init(
    extensions: [String: DataValue] = [:], `file`: String?, `firstLine`: Presence<UInt64> = .absent,
    `kind`: String, `lastLine`: Presence<UInt64> = .absent, `name`: String, `ordinal`: UInt64,
    `record`: DataValue
  ) {
    self.extensions = extensions
    self.`file` = `file`
    self.`firstLine` = `firstLine`
    self.`kind` = `kind`
    self.`lastLine` = `lastLine`
    self.`name` = `name`
    self.`ordinal` = `ordinal`
    self.`record` = `record`
  }
  static func read(_ json: DataValue) throws -> SourceRelationEndpoint {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "file", "first_line", "kind", "last_line", "name", "ordinal", "record",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      file: try String?.read(jsonRequired(object, "file")),
      firstLine: try readPresence(object, "first_line", UInt64.self),
      kind: try String.read(jsonRequired(object, "kind")),
      lastLine: try readPresence(object, "last_line", UInt64.self),
      name: try String.read(jsonRequired(object, "name")),
      ordinal: try UInt64.read(jsonRequired(object, "ordinal")),
      record: try DataValue.read(jsonRequired(object, "record")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "file", "first_line", "kind", "last_line", "name", "ordinal", "record",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    object["file"] = `file`.json
    writePresence(`firstLine`, "first_line", &object)
    object["kind"] = `kind`.json
    writePresence(`lastLine`, "last_line", &object)
    object["name"] = `name`.json
    object["ordinal"] = `ordinal`.json
    object["record"] = `record`.json
    return .object(object)
  }
}
public indirect enum ReceiptThreshold: ValueRepresentable {
  case `number`(Double)
  case `string`(String)
  static func read(_ json: DataValue) throws -> ReceiptThreshold {
    if let value = try? Double.read(json) { return .`number`(value) }
    if let value = try? String.read(json) { return .`string`(value) }
    throw ValueConversionError("Unknown ReceiptThreshold value")
  }
  var json: DataValue {
    switch self {
    case .`number`(let value): return value.json
    case .`string`(let value): return value.json
    }
  }
}
public struct TokenBand: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `lower`: UInt64
  public let `upper`: UInt64
  public init(extensions: [String: DataValue] = [:], `lower`: UInt64, `upper`: UInt64) {
    self.extensions = extensions
    self.`lower` = `lower`
    self.`upper` = `upper`
  }
  static func read(_ json: DataValue) throws -> TokenBand {
    let object = try jsonObject(json)
    let known: Set<String> = ["lower", "upper"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      lower: try UInt64.read(jsonRequired(object, "lower")),
      upper: try UInt64.read(jsonRequired(object, "upper")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["lower", "upper"]
    var object = extensions.filter { !known.contains($0.key) }
    object["lower"] = `lower`.json
    object["upper"] = `upper`.json
    return .object(object)
  }
}
public struct TokenUsage: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `inputTokens`: UInt64
  public let `outputTokens`: UInt64
  public init(extensions: [String: DataValue] = [:], `inputTokens`: UInt64, `outputTokens`: UInt64)
  {
    self.extensions = extensions
    self.`inputTokens` = `inputTokens`
    self.`outputTokens` = `outputTokens`
  }
  static func read(_ json: DataValue) throws -> TokenUsage {
    let object = try jsonObject(json)
    let known: Set<String> = ["input_tokens", "output_tokens"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      inputTokens: try UInt64.read(jsonRequired(object, "input_tokens")),
      outputTokens: try UInt64.read(jsonRequired(object, "output_tokens")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["input_tokens", "output_tokens"]
    var object = extensions.filter { !known.contains($0.key) }
    object["input_tokens"] = `inputTokens`.json
    object["output_tokens"] = `outputTokens`.json
    return .object(object)
  }
}
public indirect enum Value: ValueRepresentable {
  case `boolean`(Bool)
  case `null`
  case `string`(String)
  case `list`([String])
  case `number`(Double)
  static func read(_ json: DataValue) throws -> Value {
    if let value = try? Bool.read(json) { return .`boolean`(value) }
    if json == .null { return .`null` }
    if let value = try? String.read(json) { return .`string`(value) }
    if let value = try? [String].read(json) { return .`list`(value) }
    if let value = try? Double.read(json) { return .`number`(value) }
    throw ValueConversionError("Unknown Value value")
  }
  var json: DataValue {
    switch self {
    case .`boolean`(let value): return value.json
    case .`null`: return .null
    case .`string`(let value): return value.json
    case .`list`(let value): return value.json
    case .`number`(let value): return value.json
    }
  }
}
extension SessionPacket {
  public func decideAnswer() throws -> [AtomicDecideValue] {
    if case .`decideAggregate`(let packet) = self { return packet.value }
    throw ValueConversionError("Expected decide aggregate packet")
  }
  public func chooseAnswer() throws -> [AtomicNullableString] {
    if case .`chooseAggregate`(let packet) = self { return packet.value }
    throw ValueConversionError("Expected choose aggregate packet")
  }
  public func tagAnswer() throws -> [AtomicArrayOfString] {
    if case .`tagAggregate`(let packet) = self { return packet.value }
    throw ValueConversionError("Expected tag aggregate packet")
  }
  public func scoreAnswer() throws -> [AtomicDouble] {
    if case .`scoreAggregate`(let packet) = self { return packet.value }
    throw ValueConversionError("Expected score aggregate packet")
  }
  public func filterAnswer() throws -> [AtomicBoolean] {
    if case .`filterAggregate`(let packet) = self { return packet.value }
    throw ValueConversionError("Expected filter aggregate packet")
  }
  public func rankAnswer() throws -> [AtomicNonZeroUsize] {
    if case .`rankAggregate`(let packet) = self { return packet.value }
    throw ValueConversionError("Expected rank aggregate packet")
  }
  public func findAnswer() throws -> Find {
    if case .`findAggregate`(let packet) = self { return packet.value }
    throw ValueConversionError("Expected find aggregate packet")
  }
  public func annotateAnswer() throws -> [Annotation] {
    if case .`annotateAggregate`(let packet) = self { return packet.value }
    throw ValueConversionError("Expected annotate aggregate packet")
  }
  public func recognizeAnswer() throws -> [Recognition] {
    if case .`recognizeAggregate`(let packet) = self { return packet.value }
    throw ValueConversionError("Expected recognize aggregate packet")
  }
  public func relateAnswer() throws -> Relation {
    if case .`relateAggregate`(let packet) = self { return packet.value }
    throw ValueConversionError("Expected relate aggregate packet")
  }
}
public struct AnswerCardinalityError: Error, Sendable {
  public let function: String
  public let count: Int
  public let call: Call
}
private func answerPath(_ json: DataValue, _ path: [String]) throws -> DataValue? {
  var value: DataValue? = json
  for key in path {
    guard let object = value else { return nil }
    value = try jsonObject(object)[key]
  }
  return value
}
private func answerProjection(_ value: DataValue, _ member: String) throws -> DataValue {
  guard case .array(let rows) = value.kind else {
    throw ValueConversionError("Expected answer collection")
  }
  return .array(try rows.map { try jsonRequired(jsonObject($0), member) })
}
extension Call {
  private func answerJSON(_ function: String, _ judgment: String) throws -> DataValue {
    guard terminal.kind == "terminal" else {
      throw AnswerCardinalityError(function: function, count: 0, call: self)
    }
    if case .null = terminal.failure {
      throw ValueConversionError("Terminal failure must be an object")
    }
    if case .value(let failure) = terminal.failure {
      throw SessionFailure(failure: failure, call: self)
    }
    let collection =
      function == "decide" || function == "choose" || function == "tag" || function == "score"
      || function == "filter" || function == "rank" || function == "annotate"
      || function == "recognize"
    var values: [DataValue] = []
    for packet in packets {
      guard try answerPath(packet.json, ["function"]) == .string(function) else { continue }
      if judgment.isEmpty {
        let kind = try answerPath(packet.json, ["kind"])
        let row = collection && kind == .string("row")
        guard row || kind == .string("aggregate"),
          let value = try answerPath(packet.json, ["value"])
        else { continue }
        values.append(row ? .array([value]) : value)
      } else {
        guard try answerPath(packet.json, ["kind"]) == .string("observation") else { continue }
        guard try answerPath(packet.json, ["value", "kind"]) == .string("row") else { continue }
        guard try answerPath(packet.json, ["value", "value", "kind"]) == .string("judgment") else {
          continue
        }
        guard let carrier = try answerPath(packet.json, ["value", "value", "value"]) else {
          continue
        }
        guard try answerPath(carrier, ["kind"]) == .string(judgment),
          let value = try answerPath(carrier, ["value"])
        else { continue }
        values.append(value)
      }
    }
    if judgment.isEmpty && collection {
      var combined: [DataValue] = []
      for value in values {
        guard case .array(let items) = value.kind else {
          throw ValueConversionError("Expected aggregate collection")
        }
        combined.append(contentsOf: items)
      }
      return .array(combined)
    }
    guard values.count == 1 else {
      throw AnswerCardinalityError(function: function, count: values.count, call: self)
    }
    return values[0]
  }
  public func decideAnswer() throws -> Bool? {
    try Optional<Bool>.read(answerJSON("decide", "decision"))
  }
  public func chooseAnswer() throws -> String? {
    try Optional<String>.read(answerJSON("choose", "choice"))
  }
  public func tagAnswer() throws -> [String] { try [String].read(answerJSON("tag", "tags")) }
  public func scoreAnswer() throws -> Double { try Double.read(answerJSON("score", "score")) }
  public func filterAnswer() throws -> [DataValue] {
    try [DataValue].read(answerProjection(answerJSON("filter", ""), "input"))
  }
  public func rankAnswer() throws -> [UInt64] {
    try [UInt64].read(answerProjection(answerJSON("rank", ""), "index"))
  }
  public func findAnswer() throws -> Find { try Find.read(answerJSON("find", "")) }
  public func annotateAnswer() throws -> [Annotation] {
    try [Annotation].read(answerJSON("annotate", ""))
  }
  public func recognizeAnswer() throws -> [Recognition] {
    try [Recognition].read(answerJSON("recognize", ""))
  }
  public func relateAnswer() throws -> Relation { try Relation.read(answerJSON("relate", "")) }
}
extension Client {
  public func decideAnswer(_ question: Question, input: Input, options: Presence<Options> = .absent)
    async throws -> Bool?
  {
    let call = try await decide(question, input: input, options: options)
    return try call.decideAnswer()
  }
  public func chooseAnswer(_ question: Question, input: Input, options: Presence<Options> = .absent)
    async throws -> String?
  {
    let call = try await choose(question, input: input, options: options)
    return try call.chooseAnswer()
  }
  public func tagAnswer(_ question: Question, input: Input, options: Presence<Options> = .absent)
    async throws -> [String]
  {
    let call = try await tag(question, input: input, options: options)
    return try call.tagAnswer()
  }
  public func scoreAnswer(_ question: Question, input: Input, options: Presence<Options> = .absent)
    async throws -> Double
  {
    let call = try await score(question, input: input, options: options)
    return try call.scoreAnswer()
  }
  public func filterAnswer(_ question: Question, input: Input, options: Presence<Options> = .absent)
    async throws -> [DataValue]
  {
    let call = try await filter(question, input: input, options: options)
    return try call.filterAnswer()
  }
  public func rankAnswer(_ question: Question, input: Input, options: Presence<Options> = .absent)
    async throws -> [UInt64]
  {
    let call = try await rank(question, input: input, options: options)
    return try call.rankAnswer()
  }
  public func findAnswer(_ question: Question, input: Input, options: Presence<Options> = .absent)
    async throws -> Find
  {
    let call = try await find(question, input: input, options: options)
    return try call.findAnswer()
  }
  public func annotateAnswer(
    _ question: Question, input: Input, options: Presence<Options> = .absent
  ) async throws -> [Annotation] {
    let call = try await annotate(question, input: input, options: options)
    return try call.annotateAnswer()
  }
  public func recognizeAnswer(
    _ question: Question, input: Input, options: Presence<Options> = .absent
  ) async throws -> [Recognition] {
    let call = try await recognize(question, input: input, options: options)
    return try call.recognizeAnswer()
  }
  public func relateAnswer(_ question: Question, input: Input, options: Presence<Options> = .absent)
    async throws -> Relation
  {
    let call = try await relate(question, input: input, options: options)
    return try call.relateAnswer()
  }
}

func nativeElements<T>(_ pointer: UnsafePointer<T>?, _ count: Int) throws -> [T] {
  if count == 0 { return [] }
  guard let pointer else { throw ValueConversionError("Missing native collection") }
  return Array(UnsafeBufferPointer(start: pointer, count: count))
}
func copyNativeLeaf(_ pointer: UnsafePointer<thinkthen_complete_json_v1>?) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native leaf") }
  return try DataValue.copyNative(pointer)
}
func copyNativeExtensions(_ value: thinkthen_complete_extensions_v1) throws -> [ValueMember] {
  try nativeElements(value.data, value.len).map {
    ValueMember(nativeString($0.name), try copyNativeLeaf($0.value))
  }
}

func nativeValue(_ owner: OpaquePointer) throws -> DataValue {
  var view: UnsafePointer<thinkthen_complete_json_v1>?
  try sessionCheck(thinkthen_value_view(owner, &view))
  guard let view else { throw ValueConversionError("Missing native value view") }
  return try DataValue.copyNative(view)
}

private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_session_packet_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return try copyNativeGraph(value.data.`decide_row`)
  case 2: return try copyNativeGraph(value.data.`choose_row`)
  case 3: return try copyNativeGraph(value.data.`tag_row`)
  case 4: return try copyNativeGraph(value.data.`score_row`)
  case 5: return try copyNativeGraph(value.data.`filter_row`)
  case 6: return try copyNativeGraph(value.data.`annotate_row`)
  case 7: return try copyNativeGraph(value.data.`decide_aggregate`)
  case 8: return try copyNativeGraph(value.data.`choose_aggregate`)
  case 9: return try copyNativeGraph(value.data.`tag_aggregate`)
  case 10: return try copyNativeGraph(value.data.`score_aggregate`)
  case 11: return try copyNativeGraph(value.data.`filter_aggregate`)
  case 12: return try copyNativeGraph(value.data.`rank_aggregate`)
  case 13: return try copyNativeGraph(value.data.`find_aggregate`)
  case 14: return try copyNativeGraph(value.data.`annotate_aggregate`)
  case 15: return try copyNativeGraph(value.data.`recognize_aggregate`)
  case 16: return try copyNativeGraph(value.data.`relate_aggregate`)
  case 17: return try copyNativeGraph(value.data.`observation`)
  case 18: return try copyNativeGraph(value.data.`terminal`)
  default: throw ValueConversionError("Unknown native packet alternative")
  }
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_session_packet_terminal_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  switch value.`facts`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("facts", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("facts", try copyNativeGraph(value.`facts`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`failure`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("failure", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("failure", try copyNativeGraph(value.`failure`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_call_error_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("error", try copyNativeGraph(value.`error`)))
  switch value.`facts`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("facts", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("facts", try copyNativeGraph(value.`facts`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_facts_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  switch value.`attempts`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("attempts", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "attempts",
        .array(
          try nativeElements(value.`attempts`.value.data, value.`attempts`.value.len).map { item in
            try copyNativeGraph(item)
          })))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(
    ValueMember("cache_answers", .number(NumberLiteral(unchecked: String(value.`cache_answers`)))))
  members.append(ValueMember("call_id", try copyNativeGraph(value.`call_id`)))
  switch value.`estimated_cost_usd`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("estimated_cost_usd", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("estimated_cost_usd", .string(nativeString(value.`estimated_cost_usd`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`held_model_mismatch`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("held_model_mismatch", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("held_model_mismatch", .bool(value.`held_model_mismatch`.value != 0)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`input_tokens`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("input_tokens", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "input_tokens", .number(NumberLiteral(unchecked: String(value.`input_tokens`.value)))))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(
    ValueMember(
      "largest_request_bytes",
      .number(NumberLiteral(unchecked: String(value.`largest_request_bytes`)))))
  switch value.`largest_request_estimated_input_tokens`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("largest_request_estimated_input_tokens", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "largest_request_estimated_input_tokens",
        .number(
          NumberLiteral(unchecked: String(value.`largest_request_estimated_input_tokens`.value)))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`model`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("model", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("model", .string(nativeString(value.`model`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`output_tokens`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("output_tokens", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "output_tokens", .number(NumberLiteral(unchecked: String(value.`output_tokens`.value)))))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("records", .number(NumberLiteral(unchecked: String(value.`records`)))))
  members.append(
    ValueMember("requests_sent", .number(NumberLiteral(unchecked: String(value.`requests_sent`)))))
  members.append(ValueMember("seconds", .number(NumberLiteral(unchecked: String(value.`seconds`)))))
  members.append(
    ValueMember("token_estimate_method", .string(nativeString(value.`token_estimate_method`))))
  switch value.`usage_persistence`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("usage_persistence", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("usage_persistence", try copyNativeGraph(value.`usage_persistence`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_persistence_observation_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  switch value.`advice`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("advice", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("advice", .string(nativeString(value.`advice`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("observed_at", .string(nativeString(value.`observed_at`))))
  members.append(ValueMember("state", try copyNativeGraph(value.`state`)))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_usage_persistence_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return .string("disabled")
  case 2: return .string("pending")
  case 3: return .string("written")
  case 4: return .string("failed")
  default: throw ValueConversionError("Unknown native enum value")
  }
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_call_id_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  return .string(nativeString(value.value))
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_attempt_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("ordinal", .number(NumberLiteral(unchecked: String(value.`ordinal`)))))
  members.append(ValueMember("outcome", try copyNativeGraph(value.`outcome`)))
  switch value.`request_id`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("request_id", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("request_id", .string(nativeString(value.`request_id`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("request_sha256", .string(nativeString(value.`request_sha256`))))
  members.append(ValueMember("sdk_request_id", try copyNativeGraph(value.`sdk_request_id`)))
  switch value.`server_ms`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("server_ms", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("server_ms", .number(NumberLiteral(unchecked: String(value.`server_ms`.value)))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`status`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("status", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("status", .number(NumberLiteral(unchecked: String(value.`status`.value)))))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("wall_ms", .number(NumberLiteral(unchecked: String(value.`wall_ms`)))))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_sdk_request_id_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  return .string(nativeString(value.value))
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_attempt_outcome_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return .string("ok")
  case 2: return .string("status")
  case 3: return .string("transport")
  default: throw ValueConversionError("Unknown native enum value")
  }
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_error_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  switch value.`estimated_input_denial`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("estimated_input_denial", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "estimated_input_denial", try copyNativeGraph(value.`estimated_input_denial`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("kind", try copyNativeGraph(value.`kind`)))
  members.append(ValueMember("message", .string(nativeString(value.`message`))))
  members.append(ValueMember("retryable", .bool(value.`retryable` != 0)))
  switch value.`send_budget_denial`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("send_budget_denial", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("send_budget_denial", try copyNativeGraph(value.`send_budget_denial`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("stopped", try copyNativeGraph(value.`stopped`)))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_stopped_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  switch value.`at`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("at", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("at", .number(NumberLiteral(unchecked: String(value.`at`.value)))))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("cause", try copyNativeGraph(value.`cause`)))
  members.append(ValueMember("retryable", .bool(value.`retryable` != 0)))
  switch value.`status`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("status", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("status", .number(NumberLiteral(unchecked: String(value.`status`.value)))))
  default: throw ValueConversionError("Unknown native presence")
  }
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_stop_cause_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return .string("usage")
  case 2: return .string("local")
  case 3: return .string("no_key")
  case 4: return .string("transport")
  case 5: return .string("status")
  case 6: return .string("too_large")
  case 7: return .string("reply")
  case 8: return .string("backend")
  case 9: return .string("cancelled")
  case 10: return .string("deadline")
  case 11: return .string("defect")
  default: throw ValueConversionError("Unknown native enum value")
  }
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_send_denial_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return try copyNativeGraph(value.data.`before_first_send`)
  case 2: return try copyNativeGraph(value.data.`before_additional_send`)
  case 3: return try copyNativeGraph(value.data.`before_retry`)
  default: throw ValueConversionError("Unknown native packet alternative")
  }
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_send_denial_before_retry_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(
    ValueMember("last_status", .number(NumberLiteral(unchecked: String(value.`last_status`)))))
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_send_denial_before_additional_send_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_send_denial_before_first_send_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_failure_kind_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return .string("usage")
  case 2: return .string("backend")
  case 3: return .string("local")
  case 4: return .string("cancelled")
  case 5: return .string("deadline")
  case 6: return .string("defect")
  default: throw ValueConversionError("Unknown native enum value")
  }
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_input_denial_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return try copyNativeGraph(value.data.`initial_request`)
  case 2: return try copyNativeGraph(value.data.`additional_request`)
  case 3: return try copyNativeGraph(value.data.`retry`)
  default: throw ValueConversionError("Unknown native packet alternative")
  }
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_input_denial_retry_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(
    ValueMember("last_status", .number(NumberLiteral(unchecked: String(value.`last_status`)))))
  members.append(ValueMember("limit", .number(NumberLiteral(unchecked: String(value.`limit`)))))
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_input_denial_additional_request_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(ValueMember("limit", .number(NumberLiteral(unchecked: String(value.`limit`)))))
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_input_denial_initial_request_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(ValueMember("limit", .number(NumberLiteral(unchecked: String(value.`limit`)))))
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_session_packet_observation_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("function", try copyNativeGraph(value.`function`)))
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(ValueMember("value", try copyNativeGraph(value.`value`)))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_session_observation_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return try copyNativeGraph(value.data.`question`)
  case 2: return try copyNativeGraph(value.data.`row`)
  default: throw ValueConversionError("Unknown native packet alternative")
  }
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_session_observation_row_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("index", .number(NumberLiteral(unchecked: String(value.`index`)))))
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(ValueMember("value", try copyNativeGraph(value.`value`)))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_session_observed_row_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return try copyNativeGraph(value.data.`judgment`)
  case 2: return try copyNativeGraph(value.data.`annotated`)
  case 3: return try copyNativeGraph(value.data.`recognized`)
  case 4: return try copyNativeGraph(value.data.`find`)
  case 5: return try copyNativeGraph(value.data.`relations`)
  default: throw ValueConversionError("Unknown native packet alternative")
  }
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_session_observed_row_relations_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(
    ValueMember(
      "value",
      .array(
        try nativeElements(value.`value`.data, value.`value`.len).map { item in
          try copyNativeGraph(item)
        })))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_session_relation_edge_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("either", .bool(value.`either` != 0)))
  members.append(
    ValueMember("probability", .number(NumberLiteral(unchecked: String(value.`probability`)))))
  members.append(ValueMember("relation", .string(nativeString(value.`relation`))))
  members.append(ValueMember("source", try copyNativeGraph(value.`source`)))
  members.append(ValueMember("target", try copyNativeGraph(value.`target`)))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_entity_document_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(ValueMember("name", .string(nativeString(value.`name`))))
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_session_observed_row_find_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  switch value.`value`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("value", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("value", .number(NumberLiteral(unchecked: String(value.`value`.value)))))
  default: throw ValueConversionError("Unknown native presence")
  }
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_session_observed_row_recognized_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(ValueMember("value", try copyNativeGraph(value.`value`)))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_session_recognition_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(
    ValueMember(
      "entities",
      .array(
        try nativeElements(value.`entities`.data, value.`entities`.len).map { item in
          try copyNativeGraph(item)
        })))
  members.append(ValueMember("mode", try copyNativeGraph(value.`mode`)))
  switch value.`proposals`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("proposals", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "proposals",
        .array(
          try nativeElements(value.`proposals`.value.data, value.`proposals`.value.len).map {
            item in try copyNativeGraph(item)
          })))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`relations`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("relations", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "relations",
        .array(
          try nativeElements(value.`relations`.value.data, value.`relations`.value.len).map {
            item in try copyNativeGraph(item)
          })))
  default: throw ValueConversionError("Unknown native presence")
  }
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_recognition_edge_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("either", .bool(value.`either` != 0)))
  members.append(
    ValueMember("probability", .number(NumberLiteral(unchecked: String(value.`probability`)))))
  members.append(ValueMember("relation", .string(nativeString(value.`relation`))))
  members.append(ValueMember("source", try copyNativeGraph(value.`source`)))
  members.append(ValueMember("target", try copyNativeGraph(value.`target`)))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_entity_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("end", .number(NumberLiteral(unchecked: String(value.`end`)))))
  switch value.`file`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("file", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("file", .string(nativeString(value.`file`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`first_line`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("first_line", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("first_line", .number(NumberLiteral(unchecked: String(value.`first_line`.value))))
    )
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  switch value.`last_line`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("last_line", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("last_line", .number(NumberLiteral(unchecked: String(value.`last_line`.value)))))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("length", .number(NumberLiteral(unchecked: String(value.`length`)))))
  members.append(ValueMember("start", .number(NumberLiteral(unchecked: String(value.`start`)))))
  members.append(
    ValueMember("strength", .number(NumberLiteral(unchecked: String(value.`strength`)))))
  members.append(ValueMember("text", .string(nativeString(value.`text`))))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_boundary_proposal_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("end", .number(NumberLiteral(unchecked: String(value.`end`)))))
  members.append(ValueMember("length", .number(NumberLiteral(unchecked: String(value.`length`)))))
  members.append(
    ValueMember("probability", .number(NumberLiteral(unchecked: String(value.`probability`)))))
  members.append(ValueMember("start", .number(NumberLiteral(unchecked: String(value.`start`)))))
  members.append(ValueMember("text", .string(nativeString(value.`text`))))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_recognition_mode_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return .string("whole")
  case 2: return .string("boundary_only")
  default: throw ValueConversionError("Unknown native enum value")
  }
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_session_observed_row_annotated_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(
    ValueMember(
      "value",
      .array(
        try nativeElements(value.`value`.data, value.`value`.len).map { item in
          try copyNativeGraph(item)
        })))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_session_annotation_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("name", .string(nativeString(value.`name`))))
  members.append(ValueMember("value", try copyNativeGraph(value.`value`)))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_annotation_value_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return try copyNativeGraph(value.data.`decision`)
  case 2: return try copyNativeGraph(value.data.`choice`)
  case 3: return try copyNativeGraph(value.data.`score`)
  case 4: return try copyNativeGraph(value.data.`tags`)
  case 5: return try copyNativeGraph(value.data.`failed`)
  default: throw ValueConversionError("Unknown native packet alternative")
  }
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_annotation_value_failed_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(ValueMember("value", try copyNativeGraph(value.`value`)))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_failure_field_kind_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return .string("backend")
  default: throw ValueConversionError("Unknown native enum value")
  }
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_failure_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("cause", try copyNativeGraph(value.`cause`)))
  members.append(ValueMember("kind", try copyNativeGraph(value.`kind`)))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_failure_cause_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return .string("missing_answer")
  case 2: return .string("wrong_kind")
  case 3: return .string("missing_probability")
  case 4: return .string("invalid_probability")
  case 5: return .string("invalid_distribution")
  case 6: return .string("unexpected_probability")
  default: throw ValueConversionError("Unknown native enum value")
  }
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_annotation_value_tags_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(
    ValueMember(
      "value",
      .array(
        try nativeElements(value.`value`.data, value.`value`.len).map { item in
          .string(nativeString(item))
        })))
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_annotation_value_score_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(ValueMember("value", .number(NumberLiteral(unchecked: String(value.`value`)))))
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_annotation_value_choice_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  switch value.`value`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("value", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("value", .string(nativeString(value.`value`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_annotation_value_decision_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  switch value.`value`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("value", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("value", .bool(value.`value`.value != 0)))
  default: throw ValueConversionError("Unknown native presence")
  }
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_session_observed_row_judgment_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(ValueMember("value", try copyNativeGraph(value.`value`)))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_session_judgment_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return try copyNativeGraph(value.data.`decision`)
  case 2: return try copyNativeGraph(value.data.`choice`)
  case 3: return try copyNativeGraph(value.data.`score`)
  case 4: return try copyNativeGraph(value.data.`tags`)
  default: throw ValueConversionError("Unknown native packet alternative")
  }
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_session_judgment_tags_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(
    ValueMember(
      "value",
      .array(
        try nativeElements(value.`value`.data, value.`value`.len).map { item in
          .string(nativeString(item))
        })))
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_session_judgment_score_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(ValueMember("value", .number(NumberLiteral(unchecked: String(value.`value`)))))
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_session_judgment_choice_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  switch value.`value`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("value", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("value", .string(nativeString(value.`value`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_session_judgment_decision_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  switch value.`value`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("value", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("value", .bool(value.`value`.value != 0)))
  default: throw ValueConversionError("Unknown native presence")
  }
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_session_observation_question_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("detail", try copyNativeGraph(value.`detail`)))
  members.append(ValueMember("index", .number(NumberLiteral(unchecked: String(value.`index`)))))
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  switch value.`member`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("member", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("member", .string(nativeString(value.`member`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(
    ValueMember("position", .number(NumberLiteral(unchecked: String(value.`position`)))))
  switch value.`stage`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("stage", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("stage", .string(nativeString(value.`stage`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_session_question_detail_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  switch value.`answer_id`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("answer_id", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("answer_id", try copyNativeGraph(value.`answer_id`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("cached", .bool(value.`cached` != 0)))
  switch value.`confidence`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("confidence", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("confidence", .number(NumberLiteral(unchecked: String(value.`confidence`.value))))
    )
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(
    ValueMember(
      "failed_questions", .number(NumberLiteral(unchecked: String(value.`failed_questions`)))))
  switch value.`failure`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("failure", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("failure", try copyNativeGraph(value.`failure`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`failure_id`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("failure_id", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("failure_id", try copyNativeGraph(value.`failure_id`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`input`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("input", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("input", try copyNativeLeaf(value.`input`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`input_source`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("input_source", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("input_source", try copyNativeGraph(value.`input_source`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(
    ValueMember(
      "input_sources",
      .array(
        try nativeElements(value.`input_sources`.data, value.`input_sources`.len).map { item in
          try copyNativeGraph(item)
        })))
  members.append(
    ValueMember(
      "inputs",
      .array(
        try nativeElements(value.`inputs`.data, value.`inputs`.len).map { item in
          try copyNativeLeaf(item)
        })))
  members.append(ValueMember("model", .string(nativeString(value.`model`))))
  members.append(
    ValueMember(
      "observations",
      .array(
        try nativeElements(value.`observations`.data, value.`observations`.len).map { item in
          try copyNativeGraph(item)
        })))
  switch value.`probabilities`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("probabilities", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("probabilities", try copyNativeGraph(value.`probabilities`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("question", try copyNativeGraph(value.`question`)))
  members.append(ValueMember("question_sha256", .string(nativeString(value.`question_sha256`))))
  members.append(
    ValueMember(
      "question_sources",
      .array(
        try nativeElements(value.`question_sources`.data, value.`question_sources`.len).map {
          item in try copyNativeGraph(item)
        })))
  switch value.`raw_pick`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("raw_pick", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("raw_pick", .string(nativeString(value.`raw_pick`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`reported_usage`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("reported_usage", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("reported_usage", try copyNativeGraph(value.`reported_usage`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(
    ValueMember(
      "requests",
      .array(
        try nativeElements(value.`requests`.data, value.`requests`.len).map { item in
          .string(nativeString(item))
        })))
  members.append(
    ValueMember("requests_sent", .number(NumberLiteral(unchecked: String(value.`requests_sent`)))))
  switch value.`threshold`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("threshold", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("threshold", try copyNativeGraph(value.`threshold`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("url", .string(nativeString(value.`url`))))
  switch value.`usage`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("usage", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("usage", try copyNativeGraph(value.`usage`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`value`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("value", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("value", try copyNativeGraph(value.`value`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_value_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return .bool(value.data.`boolean` != 0)
  case 2: return .null
  case 3: return .string(nativeString(value.data.`string`))
  case 4:
    return .array(
      try nativeElements(value.data.`array`.data, value.data.`array`.len).map { item in
        .string(nativeString(item))
      })
  case 5: return .number(NumberLiteral(unchecked: String(value.data.`number`)))
  default: throw ValueConversionError("Unknown native value alternative")
  }
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_token_usage_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(
    ValueMember("input_tokens", .number(NumberLiteral(unchecked: String(value.`input_tokens`)))))
  members.append(
    ValueMember("output_tokens", .number(NumberLiteral(unchecked: String(value.`output_tokens`)))))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_threshold_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return .number(NumberLiteral(unchecked: String(value.data.`number`)))
  case 2: return .string(nativeString(value.data.`string`))
  default: throw ValueConversionError("Unknown native value alternative")
  }
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_usage_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  switch value.`input_tokens`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("input_tokens", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "input_tokens", .number(NumberLiteral(unchecked: String(value.`input_tokens`.value)))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`output_tokens`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("output_tokens", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "output_tokens", .number(NumberLiteral(unchecked: String(value.`output_tokens`.value)))))
  default: throw ValueConversionError("Unknown native presence")
  }
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_question_source_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("answered_by", .string(nativeString(value.`answered_by`))))
  switch value.`batch_size`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("batch_size", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("batch_size", .number(NumberLiteral(unchecked: String(value.`batch_size`.value))))
    )
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("origin", try copyNativeGraph(value.`origin`)))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_origin_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return .string("live")
  case 2: return .string("cache")
  case 3: return .string("replay")
  case 4: return .string("proxy")
  case 5: return .string("memory")
  default: throw ValueConversionError("Unknown native enum value")
  }
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_question_document_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return try copyNativeGraph(value.data.`decide`)
  case 2: return try copyNativeGraph(value.data.`choose`)
  case 3: return try copyNativeGraph(value.data.`tag`)
  case 4: return try copyNativeGraph(value.data.`score`)
  default: throw ValueConversionError("Unknown native packet alternative")
  }
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_question_document_score_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  switch value.`batch`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("batch", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("batch", try copyNativeGraph(value.`batch`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`context_schema`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("context_schema", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("context_schema", try copyNativeGraph(value.`context_schema`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`item_schema`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("item_schema", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("item_schema", try copyNativeGraph(value.`item_schema`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`label_details`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("label_details", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "label_details",
        .array(
          try nativeElements(value.`label_details`.value.data, value.`label_details`.value.len).map
          { item in try copyNativeGraph(item) })))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`model`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("model", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("model", .string(nativeString(value.`model`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`name`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("name", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("name", try copyNativeGraph(value.`name`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`on`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("on", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "on",
        .array(
          try nativeElements(value.`on`.value.data, value.`on`.value.len).map { item in
            .string(nativeString(item))
          })))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`profile`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("profile", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("profile", .string(nativeString(value.`profile`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`wording_version`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("wording_version", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("wording_version", try copyNativeGraph(value.`wording_version`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(
    ValueMember(
      "levels",
      .array(
        try nativeElements(value.`levels`.data, value.`levels`.len).map { item in
          .string(nativeString(item))
        })))
  switch value.`text`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("text", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("text", try copyNativeLeaf(value.`text`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("verb", .string(nativeString(value.`verb`))))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_wording_version_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  return .number(NumberLiteral(unchecked: String(value.value)))
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_question_name_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  return .string(nativeString(value.value))
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_label_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  switch value.`description`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("description", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("description", try copyNativeLeaf(value.`description`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("name", .string(nativeString(value.`name`))))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_input_declaration_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return try copyNativeGraph(value.data.`string_root`)
  case 2: return try copyNativeGraph(value.data.`object_root`)
  default: throw ValueConversionError("Unknown native packet alternative")
  }
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_input_declaration_object_root_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(
    ValueMember(
      "properties",
      .orderedObject(
        try nativeElements(value.`properties`.data, value.`properties`.len).map { item in
          ValueMember(nativeString(item.name), try copyNativeGraph(item.value))
        })))
  switch value.`required`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("required", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "required",
        .array(
          try nativeElements(value.`required`.value.data, value.`required`.value.len).map { item in
            .string(nativeString(item))
          })))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("type", try copyNativeGraph(value.`type`)))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_object_type_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return .string("object")
  default: throw ValueConversionError("Unknown native enum value")
  }
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_input_property_type_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return try copyNativeGraph(value.data.`string`)
  case 2: return try copyNativeGraph(value.data.`number`)
  case 3: return try copyNativeGraph(value.data.`boolean`)
  case 4: return try copyNativeGraph(value.data.`array`)
  default: throw ValueConversionError("Unknown native packet alternative")
  }
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_input_property_type_array_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("items", try copyNativeGraph(value.`items`)))
  members.append(ValueMember("type", .string(nativeString(value.`type`))))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_string_root_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("type", try copyNativeGraph(value.`type`)))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_string_type_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return .string("string")
  default: throw ValueConversionError("Unknown native enum value")
  }
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_input_property_type_boolean_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("type", .string(nativeString(value.`type`))))
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_input_property_type_number_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("type", .string(nativeString(value.`type`))))
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_input_property_type_string_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("type", .string(nativeString(value.`type`))))
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_input_declaration_string_root_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("type", try copyNativeGraph(value.`type`)))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_batch_string_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return .string("max")
  default: throw ValueConversionError("Unknown native enum value")
  }
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_batch_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return .number(NumberLiteral(unchecked: String(value.data.`integer`)))
  case 2: return try copyNativeGraph(value.data.`string`)
  default: throw ValueConversionError("Unknown native value alternative")
  }
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_question_document_tag_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  switch value.`batch`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("batch", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("batch", try copyNativeGraph(value.`batch`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`context_schema`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("context_schema", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("context_schema", try copyNativeGraph(value.`context_schema`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`item_schema`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("item_schema", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("item_schema", try copyNativeGraph(value.`item_schema`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`label_details`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("label_details", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "label_details",
        .array(
          try nativeElements(value.`label_details`.value.data, value.`label_details`.value.len).map
          { item in try copyNativeGraph(item) })))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`model`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("model", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("model", .string(nativeString(value.`model`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`name`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("name", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("name", try copyNativeGraph(value.`name`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`on`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("on", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "on",
        .array(
          try nativeElements(value.`on`.value.data, value.`on`.value.len).map { item in
            .string(nativeString(item))
          })))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`profile`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("profile", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("profile", .string(nativeString(value.`profile`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`wording_version`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("wording_version", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("wording_version", try copyNativeGraph(value.`wording_version`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(
    ValueMember(
      "labels",
      .array(
        try nativeElements(value.`labels`.data, value.`labels`.len).map { item in
          .string(nativeString(item))
        })))
  switch value.`text`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("text", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("text", try copyNativeLeaf(value.`text`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("verb", .string(nativeString(value.`verb`))))
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_question_document_choose_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  switch value.`batch`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("batch", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("batch", try copyNativeGraph(value.`batch`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`context_schema`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("context_schema", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("context_schema", try copyNativeGraph(value.`context_schema`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`item_schema`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("item_schema", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("item_schema", try copyNativeGraph(value.`item_schema`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`label_details`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("label_details", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "label_details",
        .array(
          try nativeElements(value.`label_details`.value.data, value.`label_details`.value.len).map
          { item in try copyNativeGraph(item) })))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`model`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("model", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("model", .string(nativeString(value.`model`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`name`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("name", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("name", try copyNativeGraph(value.`name`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`on`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("on", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "on",
        .array(
          try nativeElements(value.`on`.value.data, value.`on`.value.len).map { item in
            .string(nativeString(item))
          })))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`profile`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("profile", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("profile", .string(nativeString(value.`profile`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`wording_version`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("wording_version", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("wording_version", try copyNativeGraph(value.`wording_version`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(
    ValueMember(
      "options",
      .array(
        try nativeElements(value.`options`.data, value.`options`.len).map { item in
          .string(nativeString(item))
        })))
  switch value.`text`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("text", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("text", try copyNativeLeaf(value.`text`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("verb", .string(nativeString(value.`verb`))))
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_question_document_decide_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  switch value.`batch`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("batch", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("batch", try copyNativeGraph(value.`batch`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`context_schema`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("context_schema", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("context_schema", try copyNativeGraph(value.`context_schema`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`item_schema`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("item_schema", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("item_schema", try copyNativeGraph(value.`item_schema`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`label_details`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("label_details", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "label_details",
        .array(
          try nativeElements(value.`label_details`.value.data, value.`label_details`.value.len).map
          { item in try copyNativeGraph(item) })))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`model`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("model", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("model", .string(nativeString(value.`model`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`name`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("name", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("name", try copyNativeGraph(value.`name`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`on`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("on", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "on",
        .array(
          try nativeElements(value.`on`.value.data, value.`on`.value.len).map { item in
            .string(nativeString(item))
          })))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`profile`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("profile", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("profile", .string(nativeString(value.`profile`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`wording_version`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("wording_version", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("wording_version", try copyNativeGraph(value.`wording_version`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`false_`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("false", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("false", try copyNativeLeaf(value.`false_`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`text`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("text", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("text", try copyNativeLeaf(value.`text`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`true_`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("true", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("true", try copyNativeLeaf(value.`true_`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("verb", .string(nativeString(value.`verb`))))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_session_probabilities_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return try copyNativeGraph(value.data.`yes_no`)
  case 2: return try copyNativeGraph(value.data.`named`)
  default: throw ValueConversionError("Unknown native packet alternative")
  }
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_session_probabilities_named_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(
    ValueMember(
      "value",
      .array(
        try nativeElements(value.`value`.data, value.`value`.len).map { item in
          try copyNativeGraph(item)
        })))
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_session_named_probability_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("name", .string(nativeString(value.`name`))))
  members.append(
    ValueMember("probability", .number(NumberLiteral(unchecked: String(value.`probability`)))))
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_session_probabilities_yes_no_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(ValueMember("value", .number(NumberLiteral(unchecked: String(value.`value`)))))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_observation_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return try copyNativeGraph(value.data.`observation_id`)
  case 2: return try copyNativeGraph(value.data.`failure_id`)
  default: throw ValueConversionError("Unknown native packet alternative")
  }
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_observation_failure_id_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("failure_id", try copyNativeGraph(value.`failure_id`)))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_failure_id_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  return .string(nativeString(value.value))
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_observation_observation_id_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("observation_id", try copyNativeGraph(value.`observation_id`)))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_observation_id_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  return .string(nativeString(value.value))
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_session_input_source_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("index", .number(NumberLiteral(unchecked: String(value.`index`)))))
  members.append(ValueMember("source", try copyNativeGraph(value.`source`)))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_physical_source_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("file", .string(nativeString(value.`file`))))
  switch value.`first_line`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("first_line", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("first_line", .number(NumberLiteral(unchecked: String(value.`first_line`.value))))
    )
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`last_line`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("last_line", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("last_line", .number(NumberLiteral(unchecked: String(value.`last_line`.value)))))
  default: throw ValueConversionError("Unknown native presence")
  }
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_answer_id_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  return .string(nativeString(value.value))
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_request_function_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return .string("decide")
  case 2: return .string("choose")
  case 3: return .string("tag")
  case 4: return .string("score")
  case 5: return .string("filter")
  case 6: return .string("rank")
  case 7: return .string("find")
  case 8: return .string("annotate")
  case 9: return .string("recognize")
  case 10: return .string("relate")
  default: throw ValueConversionError("Unknown native enum value")
  }
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_session_packet_relate_aggregate_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("function", .string(nativeString(value.`function`))))
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(ValueMember("value", try copyNativeGraph(value.`value`)))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_relation_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("answer", try copyNativeGraph(value.`answer`)))
  members.append(ValueMember("answer_id", try copyNativeGraph(value.`answer_id`)))
  switch value.`file`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("file", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("file", .string(nativeString(value.`file`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`first_line`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("first_line", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("first_line", .number(NumberLiteral(unchecked: String(value.`first_line`.value))))
    )
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`index`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("index", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("index", .number(NumberLiteral(unchecked: String(value.`index`.value)))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`input`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("input", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("input", try copyNativeLeaf(value.`input`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`input_sources`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("input_sources", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "input_sources",
        .array(
          try nativeElements(value.`input_sources`.value.data, value.`input_sources`.value.len).map
          { item in try copyNativeGraph(item) })))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`last_line`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("last_line", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("last_line", .number(NumberLiteral(unchecked: String(value.`last_line`.value)))))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("meta", try copyNativeGraph(value.`meta`)))
  switch value.`position`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("position", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("position", try copyNativeGraph(value.`position`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("question", try copyNativeGraph(value.`question`)))
  members.append(ValueMember("schema", try copyNativeGraph(value.`schema`)))
  members.append(
    ValueMember(
      "value",
      .array(
        try nativeElements(value.`value`.data, value.`value`.len).map { item in
          try copyNativeGraph(item)
        })))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_related_entity_edge_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  switch value.`either`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("either", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("either", .bool(value.`either`.value != 0)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(
    ValueMember("probability", .number(NumberLiteral(unchecked: String(value.`probability`)))))
  members.append(ValueMember("relation", .string(nativeString(value.`relation`))))
  members.append(ValueMember("source", try copyNativeGraph(value.`source`)))
  members.append(ValueMember("target", try copyNativeGraph(value.`target`)))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_relation_endpoint_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return try copyNativeGraph(value.data.`related_entity`)
  case 2: return try copyNativeGraph(value.data.`source`)
  default: throw ValueConversionError("Unknown native packet alternative")
  }
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_relation_endpoint_source_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  switch value.`file`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("file", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("file", .string(nativeString(value.`file`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`first_line`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("first_line", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("first_line", .number(NumberLiteral(unchecked: String(value.`first_line`.value))))
    )
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  switch value.`last_line`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("last_line", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("last_line", .number(NumberLiteral(unchecked: String(value.`last_line`.value)))))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("name", .string(nativeString(value.`name`))))
  members.append(ValueMember("ordinal", .number(NumberLiteral(unchecked: String(value.`ordinal`)))))
  switch value.`record`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("record", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("record", try copyNativeLeaf(value.`record`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_relation_endpoint_related_entity_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(ValueMember("name", .string(nativeString(value.`name`))))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_version_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return .string("thinkthen.result/2")
  default: throw ValueConversionError("Unknown native enum value")
  }
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_relate_question_document_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  switch value.`batch`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("batch", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("batch", try copyNativeGraph(value.`batch`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`context_schema`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("context_schema", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("context_schema", try copyNativeGraph(value.`context_schema`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`fields`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("fields", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("fields", try copyNativeGraph(value.`fields`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`item_schema`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("item_schema", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("item_schema", try copyNativeGraph(value.`item_schema`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`label_details`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("label_details", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "label_details",
        .array(
          try nativeElements(value.`label_details`.value.data, value.`label_details`.value.len).map
          { item in try copyNativeGraph(item) })))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`model`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("model", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("model", .string(nativeString(value.`model`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`name`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("name", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("name", try copyNativeGraph(value.`name`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`on`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("on", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "on",
        .array(
          try nativeElements(value.`on`.value.data, value.`on`.value.len).map { item in
            .string(nativeString(item))
          })))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`profile`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("profile", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("profile", .string(nativeString(value.`profile`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(
    ValueMember(
      "relations",
      .array(
        try nativeElements(value.`relations`.data, value.`relations`.len).map { item in
          try copyNativeGraph(item)
        })))
  members.append(ValueMember("threshold", try copyNativeGraph(value.`threshold`)))
  members.append(ValueMember("verb", .string(nativeString(value.`verb`))))
  switch value.`wording_version`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("wording_version", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("wording_version", try copyNativeGraph(value.`wording_version`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_relation_rule_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("either", .bool(value.`either` != 0)))
  members.append(ValueMember("name", .string(nativeString(value.`name`))))
  members.append(ValueMember("reads", .string(nativeString(value.`reads`))))
  switch value.`single`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("single", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("single", .bool(value.`single`.value != 0)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("source", .string(nativeString(value.`source`))))
  members.append(ValueMember("target", .string(nativeString(value.`target`))))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_relate_fields_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(ValueMember("name", .string(nativeString(value.`name`))))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_position_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  switch value.`file`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("file", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("file", .string(nativeString(value.`file`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`first`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("first", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("first", .number(NumberLiteral(unchecked: String(value.`first`.value)))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`images`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("images", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "images",
        .array(
          try nativeElements(value.`images`.value.data, value.`images`.value.len).map { item in
            .string(nativeString(item))
          })))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`last`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("last", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("last", .number(NumberLiteral(unchecked: String(value.`last`.value)))))
  default: throw ValueConversionError("Unknown native presence")
  }
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_meta_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  switch value.`answered_by`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("answered_by", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("answered_by", .string(nativeString(value.`answered_by`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`attempts`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("attempts", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "attempts",
        .array(
          try nativeElements(value.`attempts`.value.data, value.`attempts`.value.len).map { item in
            try copyNativeGraph(item)
          })))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`batch_setting`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("batch_setting", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("batch_setting", try copyNativeGraph(value.`batch_setting`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`batch_warning`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("batch_warning", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("batch_warning", try copyNativeGraph(value.`batch_warning`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("cached", .bool(value.`cached` != 0)))
  switch value.`context_sha256`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("context_sha256", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("context_sha256", .string(nativeString(value.`context_sha256`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(
    ValueMember(
      "failed_questions", .number(NumberLiteral(unchecked: String(value.`failed_questions`)))))
  members.append(ValueMember("model", .string(nativeString(value.`model`))))
  members.append(
    ValueMember(
      "observations",
      .array(
        try nativeElements(value.`observations`.data, value.`observations`.len).map { item in
          try copyNativeGraph(item)
        })))
  switch value.`origin`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("origin", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("origin", try copyNativeGraph(value.`origin`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`profile_warning`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("profile_warning", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("profile_warning", try copyNativeGraph(value.`profile_warning`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`question_sha256`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("question_sha256", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("question_sha256", .string(nativeString(value.`question_sha256`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(
    ValueMember(
      "question_sources",
      .array(
        try nativeElements(value.`question_sources`.data, value.`question_sources`.len).map {
          item in try copyNativeGraph(item)
        })))
  switch value.`questions_sha256`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("questions_sha256", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("questions_sha256", .string(nativeString(value.`questions_sha256`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(
    ValueMember(
      "requests",
      .array(
        try nativeElements(value.`requests`.data, value.`requests`.len).map { item in
          .string(nativeString(item))
        })))
  members.append(
    ValueMember("requests_sent", .number(NumberLiteral(unchecked: String(value.`requests_sent`)))))
  members.append(ValueMember("tool", .string(nativeString(value.`tool`))))
  members.append(ValueMember("url", .string(nativeString(value.`url`))))
  switch value.`usage`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("usage", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("usage", try copyNativeGraph(value.`usage`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_profile_warning_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("running", .string(nativeString(value.`running`))))
  members.append(ValueMember("tuned_for", .string(nativeString(value.`tuned_for`))))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_batch_warning_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("running", try copyNativeGraph(value.`running`)))
  members.append(ValueMember("tuned_for", try copyNativeGraph(value.`tuned_for`)))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_batch_setting_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return .number(NumberLiteral(unchecked: String(value.data.`integer`)))
  case 2: return .string(nativeString(value.data.`string`))
  default: throw ValueConversionError("Unknown native value alternative")
  }
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_answers_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(
    ValueMember(
      "questions",
      .array(
        try nativeElements(value.`questions`.data, value.`questions`.len).map { item in
          try copyNativeGraph(item)
        })))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_relation_member_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return try copyNativeGraph(value.data.`answer_id`)
  case 2: return try copyNativeGraph(value.data.`failure_id`)
  default: throw ValueConversionError("Unknown native packet alternative")
  }
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_relation_member_failure_id_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("direction", try copyNativeGraph(value.`direction`)))
  members.append(ValueMember("method", try copyNativeGraph(value.`method`)))
  members.append(
    ValueMember(
      "observations",
      .array(
        try nativeElements(value.`observations`.data, value.`observations`.len).map { item in
          try copyNativeGraph(item)
        })))
  members.append(ValueMember("question", try copyNativeGraph(value.`question`)))
  members.append(
    ValueMember(
      "question_sources",
      .array(
        try nativeElements(value.`question_sources`.data, value.`question_sources`.len).map {
          item in try copyNativeGraph(item)
        })))
  members.append(ValueMember("reads", .string(nativeString(value.`reads`))))
  members.append(ValueMember("relation", .string(nativeString(value.`relation`))))
  members.append(ValueMember("request", .string(nativeString(value.`request`))))
  members.append(ValueMember("source", try copyNativeGraph(value.`source`)))
  switch value.`target`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("target", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("target", try copyNativeGraph(value.`target`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("threshold", try copyNativeGraph(value.`threshold`)))
  switch value.`usage`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("usage", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("usage", try copyNativeGraph(value.`usage`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("failure", try copyNativeGraph(value.`failure`)))
  members.append(ValueMember("failure_id", try copyNativeGraph(value.`failure_id`)))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_related_entity_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(ValueMember("name", .string(nativeString(value.`name`))))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_relation_method_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return .string("yes_no")
  case 2: return .string("choice")
  default: throw ValueConversionError("Unknown native enum value")
  }
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_relation_direction_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return .string("source_to_target")
  case 2: return .string("either")
  default: throw ValueConversionError("Unknown native enum value")
  }
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_relation_member_answer_id_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("direction", try copyNativeGraph(value.`direction`)))
  members.append(ValueMember("method", try copyNativeGraph(value.`method`)))
  members.append(
    ValueMember(
      "observations",
      .array(
        try nativeElements(value.`observations`.data, value.`observations`.len).map { item in
          try copyNativeGraph(item)
        })))
  members.append(ValueMember("question", try copyNativeGraph(value.`question`)))
  members.append(
    ValueMember(
      "question_sources",
      .array(
        try nativeElements(value.`question_sources`.data, value.`question_sources`.len).map {
          item in try copyNativeGraph(item)
        })))
  members.append(ValueMember("reads", .string(nativeString(value.`reads`))))
  members.append(ValueMember("relation", .string(nativeString(value.`relation`))))
  members.append(ValueMember("request", .string(nativeString(value.`request`))))
  members.append(ValueMember("source", try copyNativeGraph(value.`source`)))
  switch value.`target`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("target", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("target", try copyNativeGraph(value.`target`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("threshold", try copyNativeGraph(value.`threshold`)))
  switch value.`usage`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("usage", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("usage", try copyNativeGraph(value.`usage`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("accepted", .bool(value.`accepted` != 0)))
  members.append(ValueMember("answer", try copyNativeGraph(value.`answer`)))
  members.append(ValueMember("answer_id", try copyNativeGraph(value.`answer_id`)))
  members.append(
    ValueMember("probability", .number(NumberLiteral(unchecked: String(value.`probability`)))))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_answer_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return try copyNativeGraph(value.data.`yes_no`)
  case 2: return try copyNativeGraph(value.data.`choice`)
  case 3: return try copyNativeGraph(value.data.`tag`)
  case 4: return try copyNativeGraph(value.data.`score`)
  default: throw ValueConversionError("Unknown native packet alternative")
  }
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_answer_score_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  switch value.`confidence`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("confidence", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("confidence", .number(NumberLiteral(unchecked: String(value.`confidence`.value))))
    )
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(ValueMember("level", .string(nativeString(value.`level`))))
  members.append(
    ValueMember(
      "probabilities",
      .orderedObject(
        try nativeElements(value.`probabilities`.data, value.`probabilities`.len).map { item in
          ValueMember(
            nativeString(item.name), .number(NumberLiteral(unchecked: String(item.value))))
        })))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_answer_tag_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(
    ValueMember(
      "probabilities",
      .orderedObject(
        try nativeElements(value.`probabilities`.data, value.`probabilities`.len).map { item in
          ValueMember(
            nativeString(item.name), .number(NumberLiteral(unchecked: String(item.value))))
        })))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_answer_choice_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  switch value.`confidence`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("confidence", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("confidence", .number(NumberLiteral(unchecked: String(value.`confidence`.value))))
    )
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(ValueMember("pick", .string(nativeString(value.`pick`))))
  members.append(
    ValueMember(
      "probabilities",
      .orderedObject(
        try nativeElements(value.`probabilities`.data, value.`probabilities`.len).map { item in
          ValueMember(
            nativeString(item.name), .number(NumberLiteral(unchecked: String(item.value))))
        })))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_answer_yes_no_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(
    ValueMember("probability", .number(NumberLiteral(unchecked: String(value.`probability`)))))
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_session_packet_recognize_aggregate_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("function", .string(nativeString(value.`function`))))
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(
    ValueMember(
      "value",
      .array(
        try nativeElements(value.`value`.data, value.`value`.len).map { item in
          try copyNativeGraph(item)
        })))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_recognition_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("answer", try copyNativeGraph(value.`answer`)))
  members.append(ValueMember("answer_id", try copyNativeGraph(value.`answer_id`)))
  switch value.`file`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("file", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("file", .string(nativeString(value.`file`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`first_line`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("first_line", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("first_line", .number(NumberLiteral(unchecked: String(value.`first_line`.value))))
    )
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`index`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("index", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("index", .number(NumberLiteral(unchecked: String(value.`index`.value)))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`input`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("input", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("input", try copyNativeLeaf(value.`input`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`last_line`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("last_line", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("last_line", .number(NumberLiteral(unchecked: String(value.`last_line`.value)))))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("meta", try copyNativeGraph(value.`meta`)))
  switch value.`position`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("position", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("position", try copyNativeGraph(value.`position`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("question", try copyNativeGraph(value.`question`)))
  members.append(ValueMember("schema", try copyNativeGraph(value.`schema`)))
  switch value.`source`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("source", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("source", try copyNativeGraph(value.`source`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("value", try copyNativeGraph(value.`value`)))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_recognize_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return try copyNativeGraph(value.data.`entities`)
  case 2: return try copyNativeGraph(value.data.`mode_proposals`)
  default: throw ValueConversionError("Unknown native packet alternative")
  }
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_recognize_mode_proposals_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("mode", try copyNativeGraph(value.`mode`)))
  members.append(
    ValueMember(
      "proposals",
      .array(
        try nativeElements(value.`proposals`.data, value.`proposals`.len).map { item in
          try copyNativeGraph(item)
        })))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_boundary_mode_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return .string("boundary_only")
  default: throw ValueConversionError("Unknown native enum value")
  }
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_recognize_entities_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(
    ValueMember(
      "entities",
      .array(
        try nativeElements(value.`entities`.data, value.`entities`.len).map { item in
          try copyNativeGraph(item)
        })))
  switch value.`relations`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("relations", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "relations",
        .array(
          try nativeElements(value.`relations`.value.data, value.`relations`.value.len).map {
            item in try copyNativeGraph(item)
          })))
  default: throw ValueConversionError("Unknown native presence")
  }
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_entity_edge_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  switch value.`either`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("either", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("either", .bool(value.`either`.value != 0)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(
    ValueMember("probability", .number(NumberLiteral(unchecked: String(value.`probability`)))))
  members.append(ValueMember("relation", .string(nativeString(value.`relation`))))
  members.append(ValueMember("source", try copyNativeGraph(value.`source`)))
  members.append(ValueMember("target", try copyNativeGraph(value.`target`)))
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_recognize_question_document_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  switch value.`batch`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("batch", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("batch", try copyNativeGraph(value.`batch`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`context_schema`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("context_schema", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("context_schema", try copyNativeGraph(value.`context_schema`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`entity_definition`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("entity_definition", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("entity_definition", try copyNativeLeaf(value.`entity_definition`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`instructions`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("instructions", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("instructions", try copyNativeLeaf(value.`instructions`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`item_schema`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("item_schema", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("item_schema", try copyNativeGraph(value.`item_schema`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(
    ValueMember(
      "kinds",
      .orderedObject(
        try nativeElements(value.`kinds`.data, value.`kinds`.len).map { item in
          ValueMember(nativeString(item.name), try copyNativeLeaf(item.value))
        })))
  switch value.`label_details`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("label_details", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "label_details",
        .array(
          try nativeElements(value.`label_details`.value.data, value.`label_details`.value.len).map
          { item in try copyNativeGraph(item) })))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`mode`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("mode", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("mode", try copyNativeGraph(value.`mode`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`model`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("model", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("model", .string(nativeString(value.`model`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`name`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("name", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("name", try copyNativeGraph(value.`name`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`on`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("on", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "on",
        .array(
          try nativeElements(value.`on`.value.data, value.`on`.value.len).map { item in
            .string(nativeString(item))
          })))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`profile`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("profile", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("profile", .string(nativeString(value.`profile`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`relation_threshold`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("relation_threshold", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("relation_threshold", try copyNativeGraph(value.`relation_threshold`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`relations`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("relations", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "relations",
        .array(
          try nativeElements(value.`relations`.value.data, value.`relations`.value.len).map {
            item in try copyNativeGraph(item)
          })))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`snippet_pieces`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("snippet_pieces", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "snippet_pieces", .number(NumberLiteral(unchecked: String(value.`snippet_pieces`.value)))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`stage_context`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("stage_context", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("stage_context", try copyNativeGraph(value.`stage_context`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("threshold", try copyNativeGraph(value.`threshold`)))
  members.append(ValueMember("verb", try copyNativeGraph(value.`verb`)))
  switch value.`wording_version`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("wording_version", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("wording_version", try copyNativeGraph(value.`wording_version`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_verb_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return .string("recognize")
  default: throw ValueConversionError("Unknown native enum value")
  }
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_recognition_stage_context_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  switch value.`boundary`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("boundary", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("boundary", .string(nativeString(value.`boundary`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`kind_edge`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("kind_edge", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("kind_edge", .string(nativeString(value.`kind_edge`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`relation`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("relation", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("relation", .string(nativeString(value.`relation`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_recognition_odds_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return try copyNativeGraph(value.data.`recognize_answer`)
  case 2: return try copyNativeGraph(value.data.`boundary_odds`)
  default: throw ValueConversionError("Unknown native packet alternative")
  }
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_recognition_odds_boundary_odds_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(
    ValueMember(
      "pieces",
      .array(
        try nativeElements(value.`pieces`.data, value.`pieces`.len).map { item in
          try copyNativeGraph(item)
        })))
  members.append(
    ValueMember(
      "proposals",
      .array(
        try nativeElements(value.`proposals`.data, value.`proposals`.len).map { item in
          try copyNativeGraph(item)
        })))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_piece_odds_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("end", .number(NumberLiteral(unchecked: String(value.`end`)))))
  members.append(ValueMember("start", .number(NumberLiteral(unchecked: String(value.`start`)))))
  members.append(
    ValueMember(
      "tags",
      .orderedObject(
        try nativeElements(value.`tags`.data, value.`tags`.len).map { item in
          ValueMember(
            nativeString(item.name), .number(NumberLiteral(unchecked: String(item.value))))
        })))
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_recognition_odds_recognize_answer_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(
    ValueMember(
      "names",
      .array(
        try nativeElements(value.`names`.data, value.`names`.len).map { item in
          try copyNativeGraph(item)
        })))
  members.append(
    ValueMember(
      "pairs",
      .array(
        try nativeElements(value.`pairs`.data, value.`pairs`.len).map { item in
          try copyNativeGraph(item)
        })))
  members.append(
    ValueMember(
      "pieces",
      .array(
        try nativeElements(value.`pieces`.data, value.`pieces`.len).map { item in
          try copyNativeGraph(item)
        })))
  members.append(
    ValueMember(
      "proposals",
      .array(
        try nativeElements(value.`proposals`.data, value.`proposals`.len).map { item in
          try copyNativeGraph(item)
        })))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_recognition_proposal_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("end", .number(NumberLiteral(unchecked: String(value.`end`)))))
  members.append(ValueMember("kept", .bool(value.`kept` != 0)))
  switch value.`kind`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("kind", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("kind", .string(nativeString(value.`kind`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`selected`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("selected", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("selected", try copyNativeGraph(value.`selected`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(
    ValueMember(
      "span_probability", .number(NumberLiteral(unchecked: String(value.`span_probability`)))))
  members.append(ValueMember("start", .number(NumberLiteral(unchecked: String(value.`start`)))))
  switch value.`strength`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("strength", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("strength", .number(NumberLiteral(unchecked: String(value.`strength`.value)))))
  default: throw ValueConversionError("Unknown native presence")
  }
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_place_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("end", .number(NumberLiteral(unchecked: String(value.`end`)))))
  members.append(ValueMember("start", .number(NumberLiteral(unchecked: String(value.`start`)))))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_pair_odds_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(
    ValueMember("probability", .number(NumberLiteral(unchecked: String(value.`probability`)))))
  members.append(ValueMember("relation", .string(nativeString(value.`relation`))))
  members.append(ValueMember("source", try copyNativeGraph(value.`source`)))
  members.append(ValueMember("target", try copyNativeGraph(value.`target`)))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_name_odds_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  switch value.`edges`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("edges", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "edges",
        .orderedObject(
          try nativeElements(value.`edges`.value.data, value.`edges`.value.len).map { item in
            ValueMember(
              nativeString(item.name), .number(NumberLiteral(unchecked: String(item.value))))
          })))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("end", .number(NumberLiteral(unchecked: String(value.`end`)))))
  switch value.`kinds`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("kinds", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "kinds",
        .orderedObject(
          try nativeElements(value.`kinds`.value.data, value.`kinds`.value.len).map { item in
            ValueMember(
              nativeString(item.name), .number(NumberLiteral(unchecked: String(item.value))))
          })))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("start", .number(NumberLiteral(unchecked: String(value.`start`)))))
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_session_packet_annotate_aggregate_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("function", .string(nativeString(value.`function`))))
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(
    ValueMember(
      "value",
      .array(
        try nativeElements(value.`value`.data, value.`value`.len).map { item in
          try copyNativeGraph(item)
        })))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_annotation_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("answer_id", try copyNativeGraph(value.`answer_id`)))
  members.append(
    ValueMember(
      "answers",
      .orderedObject(
        try nativeElements(value.`answers`.data, value.`answers`.len).map { item in
          ValueMember(nativeString(item.name), try copyNativeGraph(item.value))
        })))
  switch value.`file`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("file", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("file", .string(nativeString(value.`file`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`first_line`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("first_line", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("first_line", .number(NumberLiteral(unchecked: String(value.`first_line`.value))))
    )
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`index`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("index", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("index", .number(NumberLiteral(unchecked: String(value.`index`.value)))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`input`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("input", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("input", try copyNativeLeaf(value.`input`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`last_line`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("last_line", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("last_line", .number(NumberLiteral(unchecked: String(value.`last_line`.value)))))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("meta", try copyNativeGraph(value.`meta`)))
  switch value.`position`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("position", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("position", try copyNativeGraph(value.`position`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("schema", try copyNativeGraph(value.`schema`)))
  switch value.`source`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("source", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("source", try copyNativeGraph(value.`source`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("value", try copyNativeGraph(value.`value`)))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_annotated_row_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  return .orderedObject(
    try nativeElements(value.value.data, value.value.len).map { item in
      ValueMember(nativeString(item.name), try copyNativeGraph(item.value))
    })
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_annotated_field_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return .bool(value.data.`boolean` != 0)
  case 2: return .null
  case 3: return .string(nativeString(value.data.`string`))
  case 4:
    return .array(
      try nativeElements(value.data.`array`.data, value.data.`array`.len).map { item in
        .string(nativeString(item))
      })
  case 5: return .number(NumberLiteral(unchecked: String(value.data.`number`)))
  case 6: return try copyNativeGraph(value.data.`object`)
  default: throw ValueConversionError("Unknown native value alternative")
  }
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_failed_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("failed", try copyNativeGraph(value.`failed`)))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_annotation_member_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return try copyNativeGraph(value.data.`answer_id`)
  case 2: return try copyNativeGraph(value.data.`failure_id`)
  default: throw ValueConversionError("Unknown native packet alternative")
  }
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_annotation_member_failure_id_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("failure", try copyNativeGraph(value.`failure`)))
  members.append(ValueMember("failure_id", try copyNativeGraph(value.`failure_id`)))
  members.append(
    ValueMember(
      "observations",
      .array(
        try nativeElements(value.`observations`.data, value.`observations`.len).map { item in
          try copyNativeGraph(item)
        })))
  members.append(ValueMember("question", try copyNativeGraph(value.`question`)))
  members.append(
    ValueMember(
      "question_sources",
      .array(
        try nativeElements(value.`question_sources`.data, value.`question_sources`.len).map {
          item in try copyNativeGraph(item)
        })))
  members.append(ValueMember("request", .string(nativeString(value.`request`))))
  switch value.`threshold`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("threshold", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("threshold", try copyNativeGraph(value.`threshold`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`usage`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("usage", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("usage", try copyNativeGraph(value.`usage`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_annotation_member_answer_id_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("answer", try copyNativeGraph(value.`answer`)))
  members.append(ValueMember("answer_id", try copyNativeGraph(value.`answer_id`)))
  members.append(
    ValueMember(
      "observations",
      .array(
        try nativeElements(value.`observations`.data, value.`observations`.len).map { item in
          try copyNativeGraph(item)
        })))
  members.append(ValueMember("question", try copyNativeGraph(value.`question`)))
  members.append(
    ValueMember(
      "question_sources",
      .array(
        try nativeElements(value.`question_sources`.data, value.`question_sources`.len).map {
          item in try copyNativeGraph(item)
        })))
  members.append(ValueMember("request", .string(nativeString(value.`request`))))
  switch value.`threshold`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("threshold", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("threshold", try copyNativeGraph(value.`threshold`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`usage`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("usage", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("usage", try copyNativeGraph(value.`usage`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`value`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("value", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("value", try copyNativeGraph(value.`value`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_session_packet_find_aggregate_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("function", .string(nativeString(value.`function`))))
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(ValueMember("value", try copyNativeGraph(value.`value`)))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_find_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("answer", try copyNativeGraph(value.`answer`)))
  members.append(ValueMember("answer_id", try copyNativeGraph(value.`answer_id`)))
  switch value.`candidates`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("candidates", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "candidates",
        .array(
          try nativeElements(value.`candidates`.value.data, value.`candidates`.value.len).map {
            item in try copyNativeGraph(item)
          })))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`file`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("file", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("file", .string(nativeString(value.`file`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`first_line`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("first_line", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("first_line", .number(NumberLiteral(unchecked: String(value.`first_line`.value))))
    )
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`index`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("index", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("index", .number(NumberLiteral(unchecked: String(value.`index`.value)))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`last_line`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("last_line", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("last_line", .number(NumberLiteral(unchecked: String(value.`last_line`.value)))))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("meta", try copyNativeGraph(value.`meta`)))
  switch value.`position`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("position", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("position", try copyNativeGraph(value.`position`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("question", try copyNativeGraph(value.`question`)))
  members.append(ValueMember("schema", try copyNativeGraph(value.`schema`)))
  switch value.`threshold`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("threshold", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("threshold", try copyNativeLeaf(value.`threshold`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`value`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("value", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("value", try copyNativeLeaf(value.`value`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_find_question_document_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  switch value.`batch`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("batch", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("batch", try copyNativeGraph(value.`batch`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`context_schema`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("context_schema", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("context_schema", try copyNativeGraph(value.`context_schema`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`item_schema`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("item_schema", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("item_schema", try copyNativeGraph(value.`item_schema`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`label_details`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("label_details", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "label_details",
        .array(
          try nativeElements(value.`label_details`.value.data, value.`label_details`.value.len).map
          { item in try copyNativeGraph(item) })))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`model`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("model", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("model", .string(nativeString(value.`model`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`name`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("name", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("name", try copyNativeGraph(value.`name`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("none", .bool(value.`none` != 0)))
  switch value.`on`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("on", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "on",
        .array(
          try nativeElements(value.`on`.value.data, value.`on`.value.len).map { item in
            .string(nativeString(item))
          })))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`profile`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("profile", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("profile", .string(nativeString(value.`profile`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`text`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("text", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("text", try copyNativeLeaf(value.`text`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("verb", .string(nativeString(value.`verb`))))
  switch value.`wording_version`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("wording_version", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("wording_version", try copyNativeGraph(value.`wording_version`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_find_candidate_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  switch value.`index`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("index", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("index", .number(NumberLiteral(unchecked: String(value.`index`.value)))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`input`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("input", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("input", try copyNativeLeaf(value.`input`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(
    ValueMember("probability", .number(NumberLiteral(unchecked: String(value.`probability`)))))
  switch value.`source`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("source", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("source", try copyNativeGraph(value.`source`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_find_answer_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  switch value.`confidence`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("confidence", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("confidence", .number(NumberLiteral(unchecked: String(value.`confidence`.value))))
    )
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(ValueMember("pick", .string(nativeString(value.`pick`))))
  members.append(
    ValueMember(
      "probabilities",
      .orderedObject(
        try nativeElements(value.`probabilities`.data, value.`probabilities`.len).map { item in
          ValueMember(
            nativeString(item.name), .number(NumberLiteral(unchecked: String(item.value))))
        })))
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_session_packet_rank_aggregate_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("function", .string(nativeString(value.`function`))))
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(
    ValueMember(
      "value",
      .array(
        try nativeElements(value.`value`.data, value.`value`.len).map { item in
          try copyNativeGraph(item)
        })))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_atomic_non_zero_usize_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("answer", try copyNativeGraph(value.`answer`)))
  members.append(ValueMember("answer_id", try copyNativeGraph(value.`answer_id`)))
  switch value.`images`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("images", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "images",
        .array(
          try nativeElements(value.`images`.value.data, value.`images`.value.len).map { item in
            try copyNativeGraph(item)
          })))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`index`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("index", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("index", .number(NumberLiteral(unchecked: String(value.`index`.value)))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`input`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("input", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("input", try copyNativeLeaf(value.`input`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`members`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("members", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "members",
        .array(
          try nativeElements(value.`members`.value.data, value.`members`.value.len).map { item in
            try copyNativeGraph(item)
          })))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("meta", try copyNativeGraph(value.`meta`)))
  members.append(ValueMember("question", try copyNativeGraph(value.`question`)))
  switch value.`question_name`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("question_name", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("question_name", .string(nativeString(value.`question_name`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("schema", try copyNativeGraph(value.`schema`)))
  switch value.`source`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("source", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("source", try copyNativeGraph(value.`source`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`threshold`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("threshold", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("threshold", try copyNativeGraph(value.`threshold`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("value", .number(NumberLiteral(unchecked: String(value.`value`)))))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_rank_member_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("name", .string(nativeString(value.`name`))))
  members.append(ValueMember("result", try copyNativeGraph(value.`result`)))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_rank_member_result_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("answer", try copyNativeGraph(value.`answer`)))
  members.append(ValueMember("answer_id", try copyNativeGraph(value.`answer_id`)))
  switch value.`images`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("images", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "images",
        .array(
          try nativeElements(value.`images`.value.data, value.`images`.value.len).map { item in
            try copyNativeGraph(item)
          })))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("meta", try copyNativeGraph(value.`meta`)))
  members.append(ValueMember("question", try copyNativeGraph(value.`question`)))
  members.append(ValueMember("schema", try copyNativeGraph(value.`schema`)))
  switch value.`source`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("source", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("source", try copyNativeGraph(value.`source`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`threshold`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("threshold", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("threshold", try copyNativeLeaf(value.`threshold`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("value", .number(NumberLiteral(unchecked: String(value.`value`)))))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_image_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("base64", .string(nativeString(value.`base64`))))
  members.append(ValueMember("height", .number(NumberLiteral(unchecked: String(value.`height`)))))
  members.append(ValueMember("media", try copyNativeGraph(value.`media`)))
  members.append(ValueMember("width", .number(NumberLiteral(unchecked: String(value.`width`)))))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_image_media_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  switch value.kind {
  case 1: return .string("image/jpeg")
  case 2: return .string("image/png")
  default: throw ValueConversionError("Unknown native enum value")
  }
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_session_packet_filter_aggregate_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("function", .string(nativeString(value.`function`))))
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(
    ValueMember(
      "value",
      .array(
        try nativeElements(value.`value`.data, value.`value`.len).map { item in
          try copyNativeGraph(item)
        })))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_atomic_boolean_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("answer", try copyNativeGraph(value.`answer`)))
  members.append(ValueMember("answer_id", try copyNativeGraph(value.`answer_id`)))
  switch value.`images`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("images", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "images",
        .array(
          try nativeElements(value.`images`.value.data, value.`images`.value.len).map { item in
            try copyNativeGraph(item)
          })))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`index`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("index", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("index", .number(NumberLiteral(unchecked: String(value.`index`.value)))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`input`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("input", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("input", try copyNativeLeaf(value.`input`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`members`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("members", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "members",
        .array(
          try nativeElements(value.`members`.value.data, value.`members`.value.len).map { item in
            try copyNativeGraph(item)
          })))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("meta", try copyNativeGraph(value.`meta`)))
  members.append(ValueMember("question", try copyNativeGraph(value.`question`)))
  switch value.`question_name`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("question_name", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("question_name", .string(nativeString(value.`question_name`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("schema", try copyNativeGraph(value.`schema`)))
  switch value.`source`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("source", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("source", try copyNativeGraph(value.`source`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`threshold`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("threshold", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("threshold", try copyNativeGraph(value.`threshold`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("value", .bool(value.`value` != 0)))
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_session_packet_score_aggregate_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("function", .string(nativeString(value.`function`))))
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(
    ValueMember(
      "value",
      .array(
        try nativeElements(value.`value`.data, value.`value`.len).map { item in
          try copyNativeGraph(item)
        })))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_atomic_double_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("answer", try copyNativeGraph(value.`answer`)))
  members.append(ValueMember("answer_id", try copyNativeGraph(value.`answer_id`)))
  switch value.`images`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("images", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "images",
        .array(
          try nativeElements(value.`images`.value.data, value.`images`.value.len).map { item in
            try copyNativeGraph(item)
          })))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`index`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("index", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("index", .number(NumberLiteral(unchecked: String(value.`index`.value)))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`input`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("input", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("input", try copyNativeLeaf(value.`input`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`members`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("members", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "members",
        .array(
          try nativeElements(value.`members`.value.data, value.`members`.value.len).map { item in
            try copyNativeGraph(item)
          })))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("meta", try copyNativeGraph(value.`meta`)))
  members.append(ValueMember("question", try copyNativeGraph(value.`question`)))
  switch value.`question_name`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("question_name", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("question_name", .string(nativeString(value.`question_name`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("schema", try copyNativeGraph(value.`schema`)))
  switch value.`source`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("source", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("source", try copyNativeGraph(value.`source`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`threshold`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("threshold", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("threshold", try copyNativeGraph(value.`threshold`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("value", .number(NumberLiteral(unchecked: String(value.`value`)))))
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_session_packet_tag_aggregate_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("function", .string(nativeString(value.`function`))))
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(
    ValueMember(
      "value",
      .array(
        try nativeElements(value.`value`.data, value.`value`.len).map { item in
          try copyNativeGraph(item)
        })))
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_atomic_array_of_string_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("answer", try copyNativeGraph(value.`answer`)))
  members.append(ValueMember("answer_id", try copyNativeGraph(value.`answer_id`)))
  switch value.`images`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("images", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "images",
        .array(
          try nativeElements(value.`images`.value.data, value.`images`.value.len).map { item in
            try copyNativeGraph(item)
          })))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`index`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("index", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("index", .number(NumberLiteral(unchecked: String(value.`index`.value)))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`input`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("input", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("input", try copyNativeLeaf(value.`input`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`members`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("members", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "members",
        .array(
          try nativeElements(value.`members`.value.data, value.`members`.value.len).map { item in
            try copyNativeGraph(item)
          })))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("meta", try copyNativeGraph(value.`meta`)))
  members.append(ValueMember("question", try copyNativeGraph(value.`question`)))
  switch value.`question_name`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("question_name", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("question_name", .string(nativeString(value.`question_name`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("schema", try copyNativeGraph(value.`schema`)))
  switch value.`source`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("source", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("source", try copyNativeGraph(value.`source`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`threshold`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("threshold", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("threshold", try copyNativeGraph(value.`threshold`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(
    ValueMember(
      "value",
      .array(
        try nativeElements(value.`value`.data, value.`value`.len).map { item in
          .string(nativeString(item))
        })))
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_session_packet_choose_aggregate_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("function", .string(nativeString(value.`function`))))
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(
    ValueMember(
      "value",
      .array(
        try nativeElements(value.`value`.data, value.`value`.len).map { item in
          try copyNativeGraph(item)
        })))
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_atomic_nullable_string_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("answer", try copyNativeGraph(value.`answer`)))
  members.append(ValueMember("answer_id", try copyNativeGraph(value.`answer_id`)))
  switch value.`images`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("images", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "images",
        .array(
          try nativeElements(value.`images`.value.data, value.`images`.value.len).map { item in
            try copyNativeGraph(item)
          })))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`index`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("index", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("index", .number(NumberLiteral(unchecked: String(value.`index`.value)))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`input`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("input", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("input", try copyNativeLeaf(value.`input`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`members`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("members", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "members",
        .array(
          try nativeElements(value.`members`.value.data, value.`members`.value.len).map { item in
            try copyNativeGraph(item)
          })))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("meta", try copyNativeGraph(value.`meta`)))
  members.append(ValueMember("question", try copyNativeGraph(value.`question`)))
  switch value.`question_name`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("question_name", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("question_name", .string(nativeString(value.`question_name`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("schema", try copyNativeGraph(value.`schema`)))
  switch value.`source`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("source", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("source", try copyNativeGraph(value.`source`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`threshold`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("threshold", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("threshold", try copyNativeGraph(value.`threshold`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`value`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("value", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("value", .string(nativeString(value.`value`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_session_packet_decide_aggregate_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("function", .string(nativeString(value.`function`))))
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(
    ValueMember(
      "value",
      .array(
        try nativeElements(value.`value`.data, value.`value`.len).map { item in
          try copyNativeGraph(item)
        })))
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_atomic_decide_value_v1>?)
  throws -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("answer", try copyNativeGraph(value.`answer`)))
  members.append(ValueMember("answer_id", try copyNativeGraph(value.`answer_id`)))
  switch value.`images`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("images", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "images",
        .array(
          try nativeElements(value.`images`.value.data, value.`images`.value.len).map { item in
            try copyNativeGraph(item)
          })))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`index`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("index", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember("index", .number(NumberLiteral(unchecked: String(value.`index`.value)))))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`input`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("input", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("input", try copyNativeLeaf(value.`input`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`members`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("members", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(
      ValueMember(
        "members",
        .array(
          try nativeElements(value.`members`.value.data, value.`members`.value.len).map { item in
            try copyNativeGraph(item)
          })))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("meta", try copyNativeGraph(value.`meta`)))
  members.append(ValueMember("question", try copyNativeGraph(value.`question`)))
  switch value.`question_name`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1):
    members.append(ValueMember("question_name", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("question_name", .string(nativeString(value.`question_name`.value))))
  default: throw ValueConversionError("Unknown native presence")
  }
  members.append(ValueMember("schema", try copyNativeGraph(value.`schema`)))
  switch value.`source`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("source", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("source", try copyNativeGraph(value.`source`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`threshold`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("threshold", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("threshold", try copyNativeGraph(value.`threshold`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  switch value.`value`.presence {
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_MISSING_V1): break
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_NULL_V1): members.append(ValueMember("value", .null))
  case UInt32(THINKTHEN_COMPLETE_PRESENCE_VALUE_V1):
    members.append(ValueMember("value", try copyNativeGraph(value.`value`.value)))
  default: throw ValueConversionError("Unknown native presence")
  }
  return .orderedObject(members)
}
private func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_decide_value_v1>?) throws
  -> DataValue
{
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  return try copyNativeLeaf(value.value)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_session_packet_annotate_row_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("function", .string(nativeString(value.`function`))))
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(ValueMember("value", try copyNativeGraph(value.`value`)))
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_session_packet_filter_row_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("function", .string(nativeString(value.`function`))))
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(ValueMember("value", try copyNativeGraph(value.`value`)))
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_session_packet_score_row_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("function", .string(nativeString(value.`function`))))
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(ValueMember("value", try copyNativeGraph(value.`value`)))
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_session_packet_tag_row_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("function", .string(nativeString(value.`function`))))
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(ValueMember("value", try copyNativeGraph(value.`value`)))
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_session_packet_choose_row_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("function", .string(nativeString(value.`function`))))
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(ValueMember("value", try copyNativeGraph(value.`value`)))
  return .orderedObject(members)
}
private func copyNativeGraph(
  _ pointer: UnsafePointer<thinkthen_complete_session_packet_decide_row_v1>?
) throws -> DataValue {
  guard let pointer else { throw ValueConversionError("Missing native graph member") }
  let value = pointer.pointee
  var members = try copyNativeExtensions(value.extensions)
  members.append(ValueMember("function", .string(nativeString(value.`function`))))
  members.append(ValueMember("kind", .string(nativeString(value.`kind`))))
  members.append(ValueMember("value", try copyNativeGraph(value.`value`)))
  return .orderedObject(members)
}
func copyNativeGraph(_ pointer: UnsafePointer<thinkthen_complete_session_packet_v1>) throws
  -> DataValue
{
  try copyNativeGraph(Optional(pointer))
}
