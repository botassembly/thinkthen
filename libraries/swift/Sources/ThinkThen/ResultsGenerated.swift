// Generated from the shared Rust graph. Do not edit.
import Foundation

public struct Annotation: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `answerId`: AnswerId
  public let `answers`: [String: AnnotationMember]
  public let `file`: Presence<String>
  public let `firstLine`: Presence<UInt64>
  public let `index`: Presence<UInt64>
  public let `input`: JSONValue
  public let `lastLine`: Presence<UInt64>
  public let `meta`: Meta
  public let `position`: Presence<Position>
  public let `schema`: ReceiptVersion
  public let `source`: Presence<PhysicalSource>
  public let `value`: AnnotatedRow
  public init(
    extensions: [String: JSONValue] = [:], `answerId`: AnswerId,
    `answers`: [String: AnnotationMember], `file`: Presence<String> = .absent,
    `firstLine`: Presence<UInt64> = .absent, `index`: Presence<UInt64> = .absent,
    `input`: JSONValue, `lastLine`: Presence<UInt64> = .absent, `meta`: Meta,
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
  static func read(_ json: JSONValue) throws -> Annotation {
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
      input: try JSONValue.read(jsonRequired(object, "input")),
      lastLine: try readPresence(object, "last_line", UInt64.self),
      meta: try Meta.read(jsonRequired(object, "meta")),
      position: try readPresence(object, "position", Position.self),
      schema: try ReceiptVersion.read(jsonRequired(object, "schema")),
      source: try readPresence(object, "source", PhysicalSource.self),
      value: try AnnotatedRow.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
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
public indirect enum AnnotationMember: JSONRepresentable {
  case `answerId`(AnnotationMemberAnswerId)
  case `failureId`(AnnotationMemberFailureId)
  static func read(_ json: JSONValue) throws -> AnnotationMember {
    let object = try jsonObject(json)
    if object["answer_id"] != nil && object["answer_id"] != .null {
      return .`answerId`(try AnnotationMemberAnswerId.read(json))
    }
    if object["failure_id"] != nil && object["failure_id"] != .null {
      return .`failureId`(try AnnotationMemberFailureId.read(json))
    }
    throw JSONConversionError("Unknown AnnotationMember alternative")
  }
  var json: JSONValue {
    switch self {
    case .`answerId`(let value): return value.json
    case .`failureId`(let value): return value.json
    }
  }
}
public struct AnnotationMemberAnswerId: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
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
    extensions: [String: JSONValue] = [:], `answer`: Answer, `answerId`: AnswerId,
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
  static func read(_ json: JSONValue) throws -> AnnotationMemberAnswerId {
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
  var json: JSONValue {
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
public struct AnnotationMemberFailureId: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `failure`: Failure
  public let `failureId`: FailureId
  public let `observations`: [Observation]
  public let `question`: QuestionDocument
  public let `questionSources`: [QuestionSource]
  public let `request`: String
  public let `threshold`: ReceiptThreshold?
  public let `usage`: Presence<Usage>
  public init(
    extensions: [String: JSONValue] = [:], `failure`: Failure, `failureId`: FailureId,
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
  static func read(_ json: JSONValue) throws -> AnnotationMemberFailureId {
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
  var json: JSONValue {
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
public indirect enum AnnotationValue: JSONRepresentable {
  case `decision`(AnnotationValueDecision)
  case `choice`(AnnotationValueChoice)
  case `score`(AnnotationValueScore)
  case `tags`(AnnotationValueTags)
  case `failed`(AnnotationValueFailed)
  static func read(_ json: JSONValue) throws -> AnnotationValue {
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
    throw JSONConversionError("Unknown AnnotationValue alternative")
  }
  var json: JSONValue {
    switch self {
    case .`decision`(let value): return value.json
    case .`choice`(let value): return value.json
    case .`score`(let value): return value.json
    case .`tags`(let value): return value.json
    case .`failed`(let value): return value.json
    }
  }
}
public struct AnnotationValueChoice: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `kind`: String
  public let `value`: String?
  public init(extensions: [String: JSONValue] = [:], `kind`: String = "choice", `value`: String?) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> AnnotationValueChoice {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try String?.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct AnnotationValueDecision: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `kind`: String
  public let `value`: Bool?
  public init(extensions: [String: JSONValue] = [:], `kind`: String = "decision", `value`: Bool?) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> AnnotationValueDecision {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try Bool?.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct AnnotationValueFailed: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `kind`: String
  public let `value`: Failure
  public init(extensions: [String: JSONValue] = [:], `kind`: String = "failed", `value`: Failure) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> AnnotationValueFailed {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try Failure.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct AnnotationValueScore: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `kind`: String
  public let `value`: Double
  public init(extensions: [String: JSONValue] = [:], `kind`: String = "score", `value`: Double) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> AnnotationValueScore {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try Double.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct AnnotationValueTags: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `kind`: String
  public let `value`: [String]
  public init(extensions: [String: JSONValue] = [:], `kind`: String = "tags", `value`: [String]) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> AnnotationValueTags {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try [String].read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public typealias AnswerId = String
public struct Answers: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `questions`: [RelationMember]
  public init(extensions: [String: JSONValue] = [:], `questions`: [RelationMember]) {
    self.extensions = extensions
    self.`questions` = `questions`
  }
  static func read(_ json: JSONValue) throws -> Answers {
    let object = try jsonObject(json)
    let known: Set<String> = ["questions"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      questions: try [RelationMember].read(jsonRequired(object, "questions")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["questions"]
    var object = extensions.filter { !known.contains($0.key) }
    object["questions"] = `questions`.json
    return .object(object)
  }
}
public struct AtomicArrayOfString: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `answer`: Answer
  public let `answerId`: AnswerId
  public let `images`: Presence<[ReceiptImage]>
  public let `index`: Presence<UInt64>
  public let `input`: Presence<JSONValue>
  public let `members`: Presence<[RankMember]>
  public let `meta`: Meta
  public let `question`: QuestionDocument
  public let `questionName`: Presence<String>
  public let `schema`: ReceiptVersion
  public let `source`: Presence<PhysicalSource>
  public let `threshold`: ReceiptThreshold?
  public let `value`: [String]
  public init(
    extensions: [String: JSONValue] = [:], `answer`: Answer, `answerId`: AnswerId,
    `images`: Presence<[ReceiptImage]> = .absent, `index`: Presence<UInt64> = .absent,
    `input`: Presence<JSONValue> = .absent, `members`: Presence<[RankMember]> = .absent,
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
  static func read(_ json: JSONValue) throws -> AtomicArrayOfString {
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
      input: try readPresence(object, "input", JSONValue.self),
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
  var json: JSONValue {
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
public struct AtomicDecideValue: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `answer`: Answer
  public let `answerId`: AnswerId
  public let `images`: Presence<[ReceiptImage]>
  public let `index`: Presence<UInt64>
  public let `input`: Presence<JSONValue>
  public let `members`: Presence<[RankMember]>
  public let `meta`: Meta
  public let `question`: QuestionDocument
  public let `questionName`: Presence<String>
  public let `schema`: ReceiptVersion
  public let `source`: Presence<PhysicalSource>
  public let `threshold`: ReceiptThreshold?
  public let `value`: DecideValue
  public init(
    extensions: [String: JSONValue] = [:], `answer`: Answer, `answerId`: AnswerId,
    `images`: Presence<[ReceiptImage]> = .absent, `index`: Presence<UInt64> = .absent,
    `input`: Presence<JSONValue> = .absent, `members`: Presence<[RankMember]> = .absent,
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
  static func read(_ json: JSONValue) throws -> AtomicDecideValue {
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
      input: try readPresence(object, "input", JSONValue.self),
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
  var json: JSONValue {
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
public struct AtomicNonZeroUsize: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `answer`: Answer
  public let `answerId`: AnswerId
  public let `images`: Presence<[ReceiptImage]>
  public let `index`: Presence<UInt64>
  public let `input`: Presence<JSONValue>
  public let `members`: Presence<[RankMember]>
  public let `meta`: Meta
  public let `question`: QuestionDocument
  public let `questionName`: Presence<String>
  public let `schema`: ReceiptVersion
  public let `source`: Presence<PhysicalSource>
  public let `threshold`: ReceiptThreshold?
  public let `value`: UInt64
  public init(
    extensions: [String: JSONValue] = [:], `answer`: Answer, `answerId`: AnswerId,
    `images`: Presence<[ReceiptImage]> = .absent, `index`: Presence<UInt64> = .absent,
    `input`: Presence<JSONValue> = .absent, `members`: Presence<[RankMember]> = .absent,
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
  static func read(_ json: JSONValue) throws -> AtomicNonZeroUsize {
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
      input: try readPresence(object, "input", JSONValue.self),
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
  var json: JSONValue {
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
public struct AtomicNullableString: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `answer`: Answer
  public let `answerId`: AnswerId
  public let `images`: Presence<[ReceiptImage]>
  public let `index`: Presence<UInt64>
  public let `input`: Presence<JSONValue>
  public let `members`: Presence<[RankMember]>
  public let `meta`: Meta
  public let `question`: QuestionDocument
  public let `questionName`: Presence<String>
  public let `schema`: ReceiptVersion
  public let `source`: Presence<PhysicalSource>
  public let `threshold`: ReceiptThreshold?
  public let `value`: String?
  public init(
    extensions: [String: JSONValue] = [:], `answer`: Answer, `answerId`: AnswerId,
    `images`: Presence<[ReceiptImage]> = .absent, `index`: Presence<UInt64> = .absent,
    `input`: Presence<JSONValue> = .absent, `members`: Presence<[RankMember]> = .absent,
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
  static func read(_ json: JSONValue) throws -> AtomicNullableString {
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
      input: try readPresence(object, "input", JSONValue.self),
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
  var json: JSONValue {
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
public struct AtomicBoolean: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `answer`: Answer
  public let `answerId`: AnswerId
  public let `images`: Presence<[ReceiptImage]>
  public let `index`: Presence<UInt64>
  public let `input`: Presence<JSONValue>
  public let `members`: Presence<[RankMember]>
  public let `meta`: Meta
  public let `question`: QuestionDocument
  public let `questionName`: Presence<String>
  public let `schema`: ReceiptVersion
  public let `source`: Presence<PhysicalSource>
  public let `threshold`: ReceiptThreshold?
  public let `value`: Bool
  public init(
    extensions: [String: JSONValue] = [:], `answer`: Answer, `answerId`: AnswerId,
    `images`: Presence<[ReceiptImage]> = .absent, `index`: Presence<UInt64> = .absent,
    `input`: Presence<JSONValue> = .absent, `members`: Presence<[RankMember]> = .absent,
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
  static func read(_ json: JSONValue) throws -> AtomicBoolean {
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
      input: try readPresence(object, "input", JSONValue.self),
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
  var json: JSONValue {
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
public struct AtomicDouble: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `answer`: Answer
  public let `answerId`: AnswerId
  public let `images`: Presence<[ReceiptImage]>
  public let `index`: Presence<UInt64>
  public let `input`: Presence<JSONValue>
  public let `members`: Presence<[RankMember]>
  public let `meta`: Meta
  public let `question`: QuestionDocument
  public let `questionName`: Presence<String>
  public let `schema`: ReceiptVersion
  public let `source`: Presence<PhysicalSource>
  public let `threshold`: ReceiptThreshold?
  public let `value`: Double
  public init(
    extensions: [String: JSONValue] = [:], `answer`: Answer, `answerId`: AnswerId,
    `images`: Presence<[ReceiptImage]> = .absent, `index`: Presence<UInt64> = .absent,
    `input`: Presence<JSONValue> = .absent, `members`: Presence<[RankMember]> = .absent,
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
  static func read(_ json: JSONValue) throws -> AtomicDouble {
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
      input: try readPresence(object, "input", JSONValue.self),
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
  var json: JSONValue {
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
public struct Attempt: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `ordinal`: UInt64
  public let `outcome`: AttemptOutcome
  public let `requestId`: Presence<String>
  public let `requestSha256`: String
  public let `sdkRequestId`: SdkRequestId
  public let `serverMs`: Presence<UInt64>
  public let `status`: Presence<UInt64>
  public let `wallMs`: UInt64
  public init(
    extensions: [String: JSONValue] = [:], `ordinal`: UInt64, `outcome`: AttemptOutcome,
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
  static func read(_ json: JSONValue) throws -> Attempt {
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
  var json: JSONValue {
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
public indirect enum ReceiptBatch: JSONRepresentable {
  case `integer`(UInt64)
  case `string`(String)
  static func read(_ json: JSONValue) throws -> ReceiptBatch {
    if let value = try? UInt64.read(json) { return .`integer`(value) }
    if let value = try? String.read(json) { return .`string`(value) }
    throw JSONConversionError("Unknown ReceiptBatch value")
  }
  var json: JSONValue {
    switch self {
    case .`integer`(let value): return value.json
    case .`string`(let value): return value.json
    }
  }
}
public enum BoundaryMode: JSONRepresentable {
  case `boundaryOnly`
  case unknown(String)
  static func read(_ json: JSONValue) throws -> BoundaryMode {
    let value = try String.read(json)
    switch value {
    case "boundary_only": return .`boundaryOnly`
    default: return .unknown(value)
    }
  }
  var json: JSONValue {
    switch self {
    case .`boundaryOnly`: return .string("boundary_only")
    case .unknown(let value): return .string(value)
    }
  }
}
public struct BoundaryOdds: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `pieces`: [PieceOdds]
  public let `proposals`: [BoundaryProposal]
  public init(
    extensions: [String: JSONValue] = [:], `pieces`: [PieceOdds], `proposals`: [BoundaryProposal]
  ) {
    self.extensions = extensions
    self.`pieces` = `pieces`
    self.`proposals` = `proposals`
  }
  static func read(_ json: JSONValue) throws -> BoundaryOdds {
    let object = try jsonObject(json)
    let known: Set<String> = ["pieces", "proposals"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      pieces: try [PieceOdds].read(jsonRequired(object, "pieces")),
      proposals: try [BoundaryProposal].read(jsonRequired(object, "proposals")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["pieces", "proposals"]
    var object = extensions.filter { !known.contains($0.key) }
    object["pieces"] = `pieces`.json
    object["proposals"] = `proposals`.json
    return .object(object)
  }
}
public struct BoundaryProposal: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `end`: UInt64
  public let `length`: UInt64
  public let `probability`: Double
  public let `start`: UInt64
  public let `text`: String
  public init(
    extensions: [String: JSONValue] = [:], `end`: UInt64, `length`: UInt64, `probability`: Double,
    `start`: UInt64, `text`: String
  ) {
    self.extensions = extensions
    self.`end` = `end`
    self.`length` = `length`
    self.`probability` = `probability`
    self.`start` = `start`
    self.`text` = `text`
  }
  static func read(_ json: JSONValue) throws -> BoundaryProposal {
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
  var json: JSONValue {
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
public struct CallError: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `error`: FailureDetail
  public let `facts`: Presence<Facts>
  public init(
    extensions: [String: JSONValue] = [:], `error`: FailureDetail,
    `facts`: Presence<Facts> = .absent
  ) {
    self.extensions = extensions
    self.`error` = `error`
    self.`facts` = `facts`
  }
  static func read(_ json: JSONValue) throws -> CallError {
    let object = try jsonObject(json)
    let known: Set<String> = ["error", "facts"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      error: try FailureDetail.read(jsonRequired(object, "error")),
      facts: try readPresence(object, "facts", Facts.self))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["error", "facts"]
    var object = extensions.filter { !known.contains($0.key) }
    object["error"] = `error`.json
    writePresence(`facts`, "facts", &object)
    return .object(object)
  }
}
public typealias CallId = String
public indirect enum DecideValue: JSONRepresentable {
  case `booleanOrNull`(Bool?)
  case `jsonValue`(JSONValue)
  static func read(_ json: JSONValue) throws -> DecideValue {
    if let value = try? Bool?.read(json) { return .`booleanOrNull`(value) }
    if let value = try? JSONValue.read(json) { return .`jsonValue`(value) }
    throw JSONConversionError("Unknown DecideValue value")
  }
  var json: JSONValue {
    switch self {
    case .`booleanOrNull`(let value): return value.json
    case .`jsonValue`(let value): return value.json
    }
  }
}
public struct EntityDocument: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `kind`: String
  public let `name`: String
  public init(extensions: [String: JSONValue] = [:], `kind`: String, `name`: String) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`name` = `name`
  }
  static func read(_ json: JSONValue) throws -> EntityDocument {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "name"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      name: try String.read(jsonRequired(object, "name")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "name"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["name"] = `name`.json
    return .object(object)
  }
}
public struct FailureDetail: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `estimatedInputDenial`: Presence<EstimatedInputDenial>
  public let `kind`: FailureKind
  public let `message`: String
  public let `retryable`: Bool
  public let `sendBudgetDenial`: Presence<SendBudgetDenial>
  public let `stopped`: Stopped
  public init(
    extensions: [String: JSONValue] = [:],
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
  static func read(_ json: JSONValue) throws -> FailureDetail {
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
  var json: JSONValue {
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
public indirect enum EstimatedInputDenial: JSONRepresentable {
  case `initialRequest`(EstimatedInputDenialInitialRequest)
  case `additionalRequest`(EstimatedInputDenialAdditionalRequest)
  case `retry`(EstimatedInputDenialRetry)
  static func read(_ json: JSONValue) throws -> EstimatedInputDenial {
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
    throw JSONConversionError("Unknown EstimatedInputDenial alternative")
  }
  var json: JSONValue {
    switch self {
    case .`initialRequest`(let value): return value.json
    case .`additionalRequest`(let value): return value.json
    case .`retry`(let value): return value.json
    }
  }
}
public struct EstimatedInputDenialAdditionalRequest: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `kind`: String
  public let `limit`: UInt64
  public init(
    extensions: [String: JSONValue] = [:], `kind`: String = "additional_request", `limit`: UInt64
  ) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`limit` = `limit`
  }
  static func read(_ json: JSONValue) throws -> EstimatedInputDenialAdditionalRequest {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "limit"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      limit: try UInt64.read(jsonRequired(object, "limit")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "limit"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["limit"] = `limit`.json
    return .object(object)
  }
}
public struct EstimatedInputDenialInitialRequest: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `kind`: String
  public let `limit`: UInt64
  public init(
    extensions: [String: JSONValue] = [:], `kind`: String = "initial_request", `limit`: UInt64
  ) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`limit` = `limit`
  }
  static func read(_ json: JSONValue) throws -> EstimatedInputDenialInitialRequest {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "limit"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      limit: try UInt64.read(jsonRequired(object, "limit")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "limit"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["limit"] = `limit`.json
    return .object(object)
  }
}
public struct EstimatedInputDenialRetry: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `kind`: String
  public let `lastStatus`: UInt64
  public let `limit`: UInt64
  public init(
    extensions: [String: JSONValue] = [:], `kind`: String = "retry", `lastStatus`: UInt64,
    `limit`: UInt64
  ) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`lastStatus` = `lastStatus`
    self.`limit` = `limit`
  }
  static func read(_ json: JSONValue) throws -> EstimatedInputDenialRetry {
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
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "last_status", "limit"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["last_status"] = `lastStatus`.json
    object["limit"] = `limit`.json
    return .object(object)
  }
}
public struct Facts: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
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
    extensions: [String: JSONValue] = [:], `attempts`: Presence<[Attempt]> = .absent,
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
  static func read(_ json: JSONValue) throws -> Facts {
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
  var json: JSONValue {
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
public struct Find: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
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
  public let `threshold`: JSONValue
  public let `value`: JSONValue
  public init(
    extensions: [String: JSONValue] = [:], `answer`: FindAnswer, `answerId`: AnswerId,
    `candidates`: Presence<[FindCandidate]> = .absent, `file`: Presence<String> = .absent,
    `firstLine`: Presence<UInt64> = .absent, `index`: UInt64?,
    `lastLine`: Presence<UInt64> = .absent, `meta`: Meta, `position`: Presence<Position> = .absent,
    `question`: FindQuestionDocument, `schema`: ReceiptVersion, `threshold`: JSONValue,
    `value`: JSONValue
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
  static func read(_ json: JSONValue) throws -> Find {
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
      threshold: try JSONValue.read(jsonRequired(object, "threshold")),
      value: try JSONValue.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
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
public struct FindCandidate: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `index`: UInt64?
  public let `input`: JSONValue
  public let `probability`: Double
  public let `source`: Presence<PhysicalSource>
  public init(
    extensions: [String: JSONValue] = [:], `index`: UInt64?, `input`: JSONValue,
    `probability`: Double, `source`: Presence<PhysicalSource> = .absent
  ) {
    self.extensions = extensions
    self.`index` = `index`
    self.`input` = `input`
    self.`probability` = `probability`
    self.`source` = `source`
  }
  static func read(_ json: JSONValue) throws -> FindCandidate {
    let object = try jsonObject(json)
    let known: Set<String> = ["index", "input", "probability", "source"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      index: try UInt64?.read(jsonRequired(object, "index")),
      input: try JSONValue.read(jsonRequired(object, "input")),
      probability: try Double.read(jsonRequired(object, "probability")),
      source: try readPresence(object, "source", PhysicalSource.self))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
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
public struct ReceiptImage: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `base64`: String
  public let `height`: UInt64
  public let `media`: ReceiptImageMedia
  public let `width`: UInt64
  public init(
    extensions: [String: JSONValue] = [:], `base64`: String, `height`: UInt64,
    `media`: ReceiptImageMedia, `width`: UInt64
  ) {
    self.extensions = extensions
    self.`base64` = `base64`
    self.`height` = `height`
    self.`media` = `media`
    self.`width` = `width`
  }
  static func read(_ json: JSONValue) throws -> ReceiptImage {
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
  var json: JSONValue {
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
public enum ReceiptImageMedia: JSONRepresentable {
  case `imageJpeg`
  case `imagePng`
  case unknown(String)
  static func read(_ json: JSONValue) throws -> ReceiptImageMedia {
    let value = try String.read(json)
    switch value {
    case "image/jpeg": return .`imageJpeg`
    case "image/png": return .`imagePng`
    default: return .unknown(value)
    }
  }
  var json: JSONValue {
    switch self {
    case .`imageJpeg`: return .string("image/jpeg")
    case .`imagePng`: return .string("image/png")
    case .unknown(let value): return .string(value)
    }
  }
}
public struct ImmediateFacts: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
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
    extensions: [String: JSONValue] = [:], `attempts`: Presence<[Attempt]> = .absent,
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
  static func read(_ json: JSONValue) throws -> ImmediateFacts {
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
  var json: JSONValue {
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
public indirect enum InputDeclaration: JSONRepresentable {
  case `string`(InputDeclarationStringRoot)
  case `object`(InputDeclarationObjectRoot)
  static func read(_ json: JSONValue) throws -> InputDeclaration {
    let object = try jsonObject(json)
    if object["type"] == .string("string") {
      return .`string`(try InputDeclarationStringRoot.read(json))
    }
    if object["type"] == .string("object") {
      return .`object`(try InputDeclarationObjectRoot.read(json))
    }
    throw JSONConversionError("Unknown InputDeclaration alternative")
  }
  var json: JSONValue {
    switch self {
    case .`string`(let value): return value.json
    case .`object`(let value): return value.json
    }
  }
}
public struct InputDeclarationObjectRoot: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `properties`: [String: InputPropertyType]
  public let `required`: Presence<[String]>
  public let `type`: ObjectType
  public init(
    extensions: [String: JSONValue] = [:], `properties`: [String: InputPropertyType],
    `required`: Presence<[String]> = .absent, `type`: ObjectType
  ) {
    self.extensions = extensions
    self.`properties` = `properties`
    self.`required` = `required`
    self.`type` = `type`
  }
  static func read(_ json: JSONValue) throws -> InputDeclarationObjectRoot {
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
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["properties", "required", "type"]
    var object = extensions.filter { !known.contains($0.key) }
    object["properties"] = `properties`.json
    writePresence(`required`, "required", &object)
    object["type"] = `type`.json
    return .object(object)
  }
}
public struct InputDeclarationStringRoot: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `type`: StringType
  public init(extensions: [String: JSONValue] = [:], `type`: StringType) {
    self.extensions = extensions
    self.`type` = `type`
  }
  static func read(_ json: JSONValue) throws -> InputDeclarationStringRoot {
    let object = try jsonObject(json)
    let known: Set<String> = ["type"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      type: try StringType.read(jsonRequired(object, "type")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["type"]
    var object = extensions.filter { !known.contains($0.key) }
    object["type"] = `type`.json
    return .object(object)
  }
}
public indirect enum InputPropertyType: JSONRepresentable {
  case `string`(InputPropertyTypeString)
  case `number`(InputPropertyTypeNumber)
  case `boolean`(InputPropertyTypeBoolean)
  case `array`(InputPropertyTypeArray)
  static func read(_ json: JSONValue) throws -> InputPropertyType {
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
    throw JSONConversionError("Unknown InputPropertyType alternative")
  }
  var json: JSONValue {
    switch self {
    case .`string`(let value): return value.json
    case .`number`(let value): return value.json
    case .`boolean`(let value): return value.json
    case .`array`(let value): return value.json
    }
  }
}
public struct InputPropertyTypeArray: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `items`: StringRoot
  public let `type`: String
  public init(extensions: [String: JSONValue] = [:], `items`: StringRoot, `type`: String = "array")
  {
    self.extensions = extensions
    self.`items` = `items`
    self.`type` = `type`
  }
  static func read(_ json: JSONValue) throws -> InputPropertyTypeArray {
    let object = try jsonObject(json)
    let known: Set<String> = ["items", "type"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      items: try StringRoot.read(jsonRequired(object, "items")),
      type: try String.read(jsonRequired(object, "type")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["items", "type"]
    var object = extensions.filter { !known.contains($0.key) }
    object["items"] = `items`.json
    object["type"] = `type`.json
    return .object(object)
  }
}
public struct InputPropertyTypeBoolean: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `type`: String
  public init(extensions: [String: JSONValue] = [:], `type`: String = "boolean") {
    self.extensions = extensions
    self.`type` = `type`
  }
  static func read(_ json: JSONValue) throws -> InputPropertyTypeBoolean {
    let object = try jsonObject(json)
    let known: Set<String> = ["type"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      type: try String.read(jsonRequired(object, "type")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["type"]
    var object = extensions.filter { !known.contains($0.key) }
    object["type"] = `type`.json
    return .object(object)
  }
}
public struct InputPropertyTypeNumber: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `type`: String
  public init(extensions: [String: JSONValue] = [:], `type`: String = "number") {
    self.extensions = extensions
    self.`type` = `type`
  }
  static func read(_ json: JSONValue) throws -> InputPropertyTypeNumber {
    let object = try jsonObject(json)
    let known: Set<String> = ["type"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      type: try String.read(jsonRequired(object, "type")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["type"]
    var object = extensions.filter { !known.contains($0.key) }
    object["type"] = `type`.json
    return .object(object)
  }
}
public struct InputPropertyTypeString: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `type`: String
  public init(extensions: [String: JSONValue] = [:], `type`: String = "string") {
    self.extensions = extensions
    self.`type` = `type`
  }
  static func read(_ json: JSONValue) throws -> InputPropertyTypeString {
    let object = try jsonObject(json)
    let known: Set<String> = ["type"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      type: try String.read(jsonRequired(object, "type")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["type"]
    var object = extensions.filter { !known.contains($0.key) }
    object["type"] = `type`.json
    return .object(object)
  }
}
public struct Label: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `description`: Presence<JSONValue>
  public let `name`: String
  public init(
    extensions: [String: JSONValue] = [:], `description`: Presence<JSONValue> = .absent,
    `name`: String
  ) {
    self.extensions = extensions
    self.`description` = `description`
    self.`name` = `name`
  }
  static func read(_ json: JSONValue) throws -> Label {
    let object = try jsonObject(json)
    let known: Set<String> = ["description", "name"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      description: try readPresence(object, "description", JSONValue.self),
      name: try String.read(jsonRequired(object, "name")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["description", "name"]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`description`, "description", &object)
    object["name"] = `name`.json
    return .object(object)
  }
}
public struct Meta: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
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
    extensions: [String: JSONValue] = [:], `answeredBy`: Presence<String> = .absent,
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
  static func read(_ json: JSONValue) throws -> Meta {
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
  var json: JSONValue {
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
public struct ObjectRoot: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `properties`: [String: InputPropertyType]
  public let `required`: Presence<[String]>
  public let `type`: ObjectType
  public init(
    extensions: [String: JSONValue] = [:], `properties`: [String: InputPropertyType],
    `required`: Presence<[String]> = .absent, `type`: ObjectType
  ) {
    self.extensions = extensions
    self.`properties` = `properties`
    self.`required` = `required`
    self.`type` = `type`
  }
  static func read(_ json: JSONValue) throws -> ObjectRoot {
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
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["properties", "required", "type"]
    var object = extensions.filter { !known.contains($0.key) }
    object["properties"] = `properties`.json
    writePresence(`required`, "required", &object)
    object["type"] = `type`.json
    return .object(object)
  }
}
public enum ObjectType: JSONRepresentable {
  case `object`
  case unknown(String)
  static func read(_ json: JSONValue) throws -> ObjectType {
    let value = try String.read(json)
    switch value {
    case "object": return .`object`
    default: return .unknown(value)
    }
  }
  var json: JSONValue {
    switch self {
    case .`object`: return .string("object")
    case .unknown(let value): return .string(value)
    }
  }
}
public indirect enum Observation: JSONRepresentable {
  case `observationId`(ObservationObservationId)
  case `failureId`(ObservationFailureId)
  static func read(_ json: JSONValue) throws -> Observation {
    let object = try jsonObject(json)
    if object["observation_id"] != nil && object["observation_id"] != .null {
      return .`observationId`(try ObservationObservationId.read(json))
    }
    if object["failure_id"] != nil && object["failure_id"] != .null {
      return .`failureId`(try ObservationFailureId.read(json))
    }
    throw JSONConversionError("Unknown Observation alternative")
  }
  var json: JSONValue {
    switch self {
    case .`observationId`(let value): return value.json
    case .`failureId`(let value): return value.json
    }
  }
}
public typealias ObservationId = String
public struct ObservationFailureId: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `failureId`: FailureId
  public init(extensions: [String: JSONValue] = [:], `failureId`: FailureId) {
    self.extensions = extensions
    self.`failureId` = `failureId`
  }
  static func read(_ json: JSONValue) throws -> ObservationFailureId {
    let object = try jsonObject(json)
    let known: Set<String> = ["failure_id"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      failureId: try FailureId.read(jsonRequired(object, "failure_id")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["failure_id"]
    var object = extensions.filter { !known.contains($0.key) }
    object["failure_id"] = `failureId`.json
    return .object(object)
  }
}
public struct ObservationObservationId: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `observationId`: ObservationId
  public init(extensions: [String: JSONValue] = [:], `observationId`: ObservationId) {
    self.extensions = extensions
    self.`observationId` = `observationId`
  }
  static func read(_ json: JSONValue) throws -> ObservationObservationId {
    let object = try jsonObject(json)
    let known: Set<String> = ["observation_id"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      observationId: try ObservationId.read(jsonRequired(object, "observation_id")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["observation_id"]
    var object = extensions.filter { !known.contains($0.key) }
    object["observation_id"] = `observationId`.json
    return .object(object)
  }
}
public enum Origin: JSONRepresentable {
  case `live`
  case `cache`
  case `replay`
  case `proxy`
  case `memory`
  case unknown(String)
  static func read(_ json: JSONValue) throws -> Origin {
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
  var json: JSONValue {
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
public struct PersistenceObservation: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `advice`: Presence<String>
  public let `observedAt`: String
  public let `state`: UsagePersistence
  public init(
    extensions: [String: JSONValue] = [:], `advice`: Presence<String> = .absent,
    `observedAt`: String, `state`: UsagePersistence
  ) {
    self.extensions = extensions
    self.`advice` = `advice`
    self.`observedAt` = `observedAt`
    self.`state` = `state`
  }
  static func read(_ json: JSONValue) throws -> PersistenceObservation {
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
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["advice", "observed_at", "state"]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`advice`, "advice", &object)
    object["observed_at"] = `observedAt`.json
    object["state"] = `state`.json
    return .object(object)
  }
}
public struct PhysicalSource: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `file`: String
  public let `firstLine`: Presence<UInt64>
  public let `lastLine`: Presence<UInt64>
  public init(
    extensions: [String: JSONValue] = [:], `file`: String, `firstLine`: Presence<UInt64> = .absent,
    `lastLine`: Presence<UInt64> = .absent
  ) {
    self.extensions = extensions
    self.`file` = `file`
    self.`firstLine` = `firstLine`
    self.`lastLine` = `lastLine`
  }
  static func read(_ json: JSONValue) throws -> PhysicalSource {
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
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["file", "first_line", "last_line"]
    var object = extensions.filter { !known.contains($0.key) }
    object["file"] = `file`.json
    writePresence(`firstLine`, "first_line", &object)
    writePresence(`lastLine`, "last_line", &object)
    return .object(object)
  }
}
public struct Position: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `file`: String?
  public let `first`: Presence<UInt64>
  public let `images`: Presence<[String]>
  public let `last`: Presence<UInt64>
  public init(
    extensions: [String: JSONValue] = [:], `file`: String?, `first`: Presence<UInt64> = .absent,
    `images`: Presence<[String]> = .absent, `last`: Presence<UInt64> = .absent
  ) {
    self.extensions = extensions
    self.`file` = `file`
    self.`first` = `first`
    self.`images` = `images`
    self.`last` = `last`
  }
  static func read(_ json: JSONValue) throws -> Position {
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
  var json: JSONValue {
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
public struct QuestionSource: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `answeredBy`: String
  public let `batchSize`: Presence<UInt64>
  public let `origin`: Origin
  public init(
    extensions: [String: JSONValue] = [:], `answeredBy`: String,
    `batchSize`: Presence<UInt64> = .absent, `origin`: Origin
  ) {
    self.extensions = extensions
    self.`answeredBy` = `answeredBy`
    self.`batchSize` = `batchSize`
    self.`origin` = `origin`
  }
  static func read(_ json: JSONValue) throws -> QuestionSource {
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
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["answered_by", "batch_size", "origin"]
    var object = extensions.filter { !known.contains($0.key) }
    object["answered_by"] = `answeredBy`.json
    writePresence(`batchSize`, "batch_size", &object)
    object["origin"] = `origin`.json
    return .object(object)
  }
}
public struct RankMember: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `name`: String
  public let `result`: RankMemberResult
  public init(extensions: [String: JSONValue] = [:], `name`: String, `result`: RankMemberResult) {
    self.extensions = extensions
    self.`name` = `name`
    self.`result` = `result`
  }
  static func read(_ json: JSONValue) throws -> RankMember {
    let object = try jsonObject(json)
    let known: Set<String> = ["name", "result"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      name: try String.read(jsonRequired(object, "name")),
      result: try RankMemberResult.read(jsonRequired(object, "result")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["name", "result"]
    var object = extensions.filter { !known.contains($0.key) }
    object["name"] = `name`.json
    object["result"] = `result`.json
    return .object(object)
  }
}
public struct RankMemberResult: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `answer`: Answer
  public let `answerId`: AnswerId
  public let `images`: Presence<[ReceiptImage]>
  public let `meta`: Meta
  public let `question`: QuestionDocument
  public let `schema`: ReceiptVersion
  public let `source`: Presence<PhysicalSource>
  public let `threshold`: JSONValue
  public let `value`: UInt64
  public init(
    extensions: [String: JSONValue] = [:], `answer`: Answer, `answerId`: AnswerId,
    `images`: Presence<[ReceiptImage]> = .absent, `meta`: Meta, `question`: QuestionDocument,
    `schema`: ReceiptVersion, `source`: Presence<PhysicalSource> = .absent, `threshold`: JSONValue,
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
  static func read(_ json: JSONValue) throws -> RankMemberResult {
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
      threshold: try JSONValue.read(jsonRequired(object, "threshold")),
      value: try UInt64.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
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
public struct Recognition: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `answer`: RecognitionOdds
  public let `answerId`: AnswerId
  public let `file`: Presence<String>
  public let `firstLine`: Presence<UInt64>
  public let `index`: Presence<UInt64>
  public let `input`: Presence<JSONValue>
  public let `lastLine`: Presence<UInt64>
  public let `meta`: Meta
  public let `position`: Presence<Position>
  public let `question`: RecognizeQuestionDocument
  public let `schema`: ReceiptVersion
  public let `source`: Presence<PhysicalSource>
  public let `value`: Recognize
  public init(
    extensions: [String: JSONValue] = [:], `answer`: RecognitionOdds, `answerId`: AnswerId,
    `file`: Presence<String> = .absent, `firstLine`: Presence<UInt64> = .absent,
    `index`: Presence<UInt64> = .absent, `input`: Presence<JSONValue> = .absent,
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
  static func read(_ json: JSONValue) throws -> Recognition {
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
      input: try readPresence(object, "input", JSONValue.self),
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
  var json: JSONValue {
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
public struct RecognitionEdgeDocument: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `either`: Bool
  public let `probability`: Double
  public let `relation`: String
  public let `source`: Entity
  public let `target`: Entity
  public init(
    extensions: [String: JSONValue] = [:], `either`: Bool, `probability`: Double,
    `relation`: String, `source`: Entity, `target`: Entity
  ) {
    self.extensions = extensions
    self.`either` = `either`
    self.`probability` = `probability`
    self.`relation` = `relation`
    self.`source` = `source`
    self.`target` = `target`
  }
  static func read(_ json: JSONValue) throws -> RecognitionEdgeDocument {
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
  var json: JSONValue {
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
public enum ReceiptRecognitionMode: JSONRepresentable {
  case `whole`
  case `boundaryOnly`
  case unknown(String)
  static func read(_ json: JSONValue) throws -> ReceiptRecognitionMode {
    let value = try String.read(json)
    switch value {
    case "whole": return .`whole`
    case "boundary_only": return .`boundaryOnly`
    default: return .unknown(value)
    }
  }
  var json: JSONValue {
    switch self {
    case .`whole`: return .string("whole")
    case .`boundaryOnly`: return .string("boundary_only")
    case .unknown(let value): return .string(value)
    }
  }
}
public indirect enum RecognitionOdds: JSONRepresentable {
  case `namesPairsPiecesProposals`(RecognitionOddsRecognizeAnswer)
  case `piecesProposals`(RecognitionOddsBoundaryOdds)
  static func read(_ json: JSONValue) throws -> RecognitionOdds {
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
    throw JSONConversionError("Unknown RecognitionOdds alternative")
  }
  var json: JSONValue {
    switch self {
    case .`namesPairsPiecesProposals`(let value): return value.json
    case .`piecesProposals`(let value): return value.json
    }
  }
}
public struct RecognitionOddsBoundaryOdds: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `pieces`: [PieceOdds]
  public let `proposals`: [BoundaryProposal]
  public init(
    extensions: [String: JSONValue] = [:], `pieces`: [PieceOdds], `proposals`: [BoundaryProposal]
  ) {
    self.extensions = extensions
    self.`pieces` = `pieces`
    self.`proposals` = `proposals`
  }
  static func read(_ json: JSONValue) throws -> RecognitionOddsBoundaryOdds {
    let object = try jsonObject(json)
    let known: Set<String> = ["pieces", "proposals"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      pieces: try [PieceOdds].read(jsonRequired(object, "pieces")),
      proposals: try [BoundaryProposal].read(jsonRequired(object, "proposals")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["pieces", "proposals"]
    var object = extensions.filter { !known.contains($0.key) }
    object["pieces"] = `pieces`.json
    object["proposals"] = `proposals`.json
    return .object(object)
  }
}
public struct RecognitionOddsRecognizeAnswer: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `names`: [NameOdds]
  public let `pairs`: [PairOdds]
  public let `pieces`: [PieceOdds]
  public let `proposals`: [RecognitionProposal]
  public init(
    extensions: [String: JSONValue] = [:], `names`: [NameOdds], `pairs`: [PairOdds],
    `pieces`: [PieceOdds], `proposals`: [RecognitionProposal]
  ) {
    self.extensions = extensions
    self.`names` = `names`
    self.`pairs` = `pairs`
    self.`pieces` = `pieces`
    self.`proposals` = `proposals`
  }
  static func read(_ json: JSONValue) throws -> RecognitionOddsRecognizeAnswer {
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
  var json: JSONValue {
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
public struct RecognitionProposal: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `end`: UInt64
  public let `kept`: Bool
  public let `kind`: Presence<String>
  public let `selected`: Presence<Place>
  public let `spanProbability`: Double
  public let `start`: UInt64
  public let `strength`: Presence<Double>
  public init(
    extensions: [String: JSONValue] = [:], `end`: UInt64, `kept`: Bool,
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
  static func read(_ json: JSONValue) throws -> RecognitionProposal {
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
  var json: JSONValue {
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
public struct ReceiptRecognitionStageContext: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `boundary`: Presence<String>
  public let `kindEdge`: Presence<String>
  public let `relation`: Presence<String>
  public init(
    extensions: [String: JSONValue] = [:], `boundary`: Presence<String> = .absent,
    `kindEdge`: Presence<String> = .absent, `relation`: Presence<String> = .absent
  ) {
    self.extensions = extensions
    self.`boundary` = `boundary`
    self.`kindEdge` = `kindEdge`
    self.`relation` = `relation`
  }
  static func read(_ json: JSONValue) throws -> ReceiptRecognitionStageContext {
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
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["boundary", "kind_edge", "relation"]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`boundary`, "boundary", &object)
    writePresence(`kindEdge`, "kind_edge", &object)
    writePresence(`relation`, "relation", &object)
    return .object(object)
  }
}
public struct Relation: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `answer`: Answers
  public let `answerId`: AnswerId
  public let `file`: Presence<String>
  public let `firstLine`: Presence<UInt64>
  public let `index`: Presence<UInt64>
  public let `input`: Presence<JSONValue>
  public let `inputSources`: Presence<[SessionInputSource]>
  public let `lastLine`: Presence<UInt64>
  public let `meta`: Meta
  public let `position`: Presence<Position>
  public let `question`: RelateQuestionDocument
  public let `schema`: ReceiptVersion
  public let `value`: [RelatedEntityEdge]
  public init(
    extensions: [String: JSONValue] = [:], `answer`: Answers, `answerId`: AnswerId,
    `file`: Presence<String> = .absent, `firstLine`: Presence<UInt64> = .absent,
    `index`: Presence<UInt64> = .absent, `input`: Presence<JSONValue> = .absent,
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
  static func read(_ json: JSONValue) throws -> Relation {
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
      input: try readPresence(object, "input", JSONValue.self),
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
  var json: JSONValue {
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
public enum RelationDirection: JSONRepresentable {
  case `sourceToTarget`
  case `either`
  case unknown(String)
  static func read(_ json: JSONValue) throws -> RelationDirection {
    let value = try String.read(json)
    switch value {
    case "source_to_target": return .`sourceToTarget`
    case "either": return .`either`
    default: return .unknown(value)
    }
  }
  var json: JSONValue {
    switch self {
    case .`sourceToTarget`: return .string("source_to_target")
    case .`either`: return .string("either")
    case .unknown(let value): return .string(value)
    }
  }
}
public indirect enum RelationEndpoint: JSONRepresentable {
  case `kindName`(RelationEndpointRelatedEntity)
  case `fileKindNameOrdinalRecord`(RelationEndpointSource)
  static func read(_ json: JSONValue) throws -> RelationEndpoint {
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
    throw JSONConversionError("Unknown RelationEndpoint alternative")
  }
  var json: JSONValue {
    switch self {
    case .`kindName`(let value): return value.json
    case .`fileKindNameOrdinalRecord`(let value): return value.json
    }
  }
}
public struct RelationEndpointRelatedEntity: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `kind`: String
  public let `name`: String
  public init(extensions: [String: JSONValue] = [:], `kind`: String, `name`: String) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`name` = `name`
  }
  static func read(_ json: JSONValue) throws -> RelationEndpointRelatedEntity {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "name"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      name: try String.read(jsonRequired(object, "name")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "name"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["name"] = `name`.json
    return .object(object)
  }
}
public struct RelationEndpointSource: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `file`: String?
  public let `firstLine`: Presence<UInt64>
  public let `kind`: String
  public let `lastLine`: Presence<UInt64>
  public let `name`: String
  public let `ordinal`: UInt64
  public let `record`: JSONValue
  public init(
    extensions: [String: JSONValue] = [:], `file`: String?, `firstLine`: Presence<UInt64> = .absent,
    `kind`: String, `lastLine`: Presence<UInt64> = .absent, `name`: String, `ordinal`: UInt64,
    `record`: JSONValue
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
  static func read(_ json: JSONValue) throws -> RelationEndpointSource {
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
      record: try JSONValue.read(jsonRequired(object, "record")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
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
public indirect enum RelationMember: JSONRepresentable {
  case `answerId`(RelationMemberAnswerId)
  case `failureId`(RelationMemberFailureId)
  static func read(_ json: JSONValue) throws -> RelationMember {
    let object = try jsonObject(json)
    if object["answer_id"] != nil && object["answer_id"] != .null {
      return .`answerId`(try RelationMemberAnswerId.read(json))
    }
    if object["failure_id"] != nil && object["failure_id"] != .null {
      return .`failureId`(try RelationMemberFailureId.read(json))
    }
    throw JSONConversionError("Unknown RelationMember alternative")
  }
  var json: JSONValue {
    switch self {
    case .`answerId`(let value): return value.json
    case .`failureId`(let value): return value.json
    }
  }
}
public struct RelationMemberAnswerId: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
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
    extensions: [String: JSONValue] = [:], `direction`: RelationDirection, `method`: RelationMethod,
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
  static func read(_ json: JSONValue) throws -> RelationMemberAnswerId {
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
  var json: JSONValue {
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
    switch try readPresence(jsonObject(self.json), "source", JSONValue.self) {
    case .absent: return .absent
    case .null: return .null
    case .value(let object): return try readPresence(jsonObject(object), "name", String.self)
    }
  }
}
public struct RelationMemberFailureId: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
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
    extensions: [String: JSONValue] = [:], `direction`: RelationDirection, `method`: RelationMethod,
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
  static func read(_ json: JSONValue) throws -> RelationMemberFailureId {
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
  var json: JSONValue {
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
    switch try readPresence(jsonObject(self.json), "source", JSONValue.self) {
    case .absent: return .absent
    case .null: return .null
    case .value(let object): return try readPresence(jsonObject(object), "name", String.self)
    }
  }
}
public enum RelationMethod: JSONRepresentable {
  case `yesNo`
  case `choice`
  case unknown(String)
  static func read(_ json: JSONValue) throws -> RelationMethod {
    let value = try String.read(json)
    switch value {
    case "yes_no": return .`yesNo`
    case "choice": return .`choice`
    default: return .unknown(value)
    }
  }
  var json: JSONValue {
    switch self {
    case .`yesNo`: return .string("yes_no")
    case .`choice`: return .string("choice")
    case .unknown(let value): return .string(value)
    }
  }
}
public enum ReceiptFunction: JSONRepresentable {
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
  static func read(_ json: JSONValue) throws -> ReceiptFunction {
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
  var json: JSONValue {
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
public indirect enum SendBudgetDenial: JSONRepresentable {
  case `beforeFirstSend`(SendBudgetDenialBeforeFirstSend)
  case `beforeAdditionalSend`(SendBudgetDenialBeforeAdditionalSend)
  case `beforeRetry`(SendBudgetDenialBeforeRetry)
  static func read(_ json: JSONValue) throws -> SendBudgetDenial {
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
    throw JSONConversionError("Unknown SendBudgetDenial alternative")
  }
  var json: JSONValue {
    switch self {
    case .`beforeFirstSend`(let value): return value.json
    case .`beforeAdditionalSend`(let value): return value.json
    case .`beforeRetry`(let value): return value.json
    }
  }
}
public struct SendBudgetDenialBeforeAdditionalSend: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `kind`: String
  public init(extensions: [String: JSONValue] = [:], `kind`: String = "before_additional_send") {
    self.extensions = extensions
    self.`kind` = `kind`
  }
  static func read(_ json: JSONValue) throws -> SendBudgetDenialBeforeAdditionalSend {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    return .object(object)
  }
}
public struct SendBudgetDenialBeforeFirstSend: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `kind`: String
  public init(extensions: [String: JSONValue] = [:], `kind`: String = "before_first_send") {
    self.extensions = extensions
    self.`kind` = `kind`
  }
  static func read(_ json: JSONValue) throws -> SendBudgetDenialBeforeFirstSend {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    return .object(object)
  }
}
public struct SendBudgetDenialBeforeRetry: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `kind`: String
  public let `lastStatus`: UInt64
  public init(
    extensions: [String: JSONValue] = [:], `kind`: String = "before_retry", `lastStatus`: UInt64
  ) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`lastStatus` = `lastStatus`
  }
  static func read(_ json: JSONValue) throws -> SendBudgetDenialBeforeRetry {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "last_status"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      lastStatus: try UInt64.read(jsonRequired(object, "last_status")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "last_status"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["last_status"] = `lastStatus`.json
    return .object(object)
  }
}
public enum StopCause: JSONRepresentable {
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
  static func read(_ json: JSONValue) throws -> StopCause {
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
  var json: JSONValue {
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
public struct Stopped: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `at`: Presence<UInt64>
  public let `cause`: StopCause
  public let `retryable`: Bool
  public let `status`: Presence<UInt64>
  public init(
    extensions: [String: JSONValue] = [:], `at`: Presence<UInt64> = .absent, `cause`: StopCause,
    `retryable`: Bool, `status`: Presence<UInt64> = .absent
  ) {
    self.extensions = extensions
    self.`at` = `at`
    self.`cause` = `cause`
    self.`retryable` = `retryable`
    self.`status` = `status`
  }
  static func read(_ json: JSONValue) throws -> Stopped {
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
  var json: JSONValue {
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
public struct StringRoot: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `type`: StringType
  public init(extensions: [String: JSONValue] = [:], `type`: StringType) {
    self.extensions = extensions
    self.`type` = `type`
  }
  static func read(_ json: JSONValue) throws -> StringRoot {
    let object = try jsonObject(json)
    let known: Set<String> = ["type"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      type: try StringType.read(jsonRequired(object, "type")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["type"]
    var object = extensions.filter { !known.contains($0.key) }
    object["type"] = `type`.json
    return .object(object)
  }
}
public enum StringType: JSONRepresentable {
  case `string`
  case unknown(String)
  static func read(_ json: JSONValue) throws -> StringType {
    let value = try String.read(json)
    switch value {
    case "string": return .`string`
    default: return .unknown(value)
    }
  }
  var json: JSONValue {
    switch self {
    case .`string`: return .string("string")
    case .unknown(let value): return .string(value)
    }
  }
}
public struct Usage: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `inputTokens`: Presence<UInt64>
  public let `outputTokens`: Presence<UInt64>
  public init(
    extensions: [String: JSONValue] = [:], `inputTokens`: Presence<UInt64> = .absent,
    `outputTokens`: Presence<UInt64> = .absent
  ) {
    self.extensions = extensions
    self.`inputTokens` = `inputTokens`
    self.`outputTokens` = `outputTokens`
  }
  static func read(_ json: JSONValue) throws -> Usage {
    let object = try jsonObject(json)
    let known: Set<String> = ["input_tokens", "output_tokens"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      inputTokens: try readPresence(object, "input_tokens", UInt64.self),
      outputTokens: try readPresence(object, "output_tokens", UInt64.self))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["input_tokens", "output_tokens"]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`inputTokens`, "input_tokens", &object)
    writePresence(`outputTokens`, "output_tokens", &object)
    return .object(object)
  }
}
public enum UsagePersistence: JSONRepresentable {
  case `disabled`
  case `pending`
  case `written`
  case `failed`
  case unknown(String)
  static func read(_ json: JSONValue) throws -> UsagePersistence {
    let value = try String.read(json)
    switch value {
    case "disabled": return .`disabled`
    case "pending": return .`pending`
    case "written": return .`written`
    case "failed": return .`failed`
    default: return .unknown(value)
    }
  }
  var json: JSONValue {
    switch self {
    case .`disabled`: return .string("disabled")
    case .`pending`: return .string("pending")
    case .`written`: return .string("written")
    case .`failed`: return .string("failed")
    case .unknown(let value): return .string(value)
    }
  }
}
public enum Verb: JSONRepresentable {
  case `recognize`
  case unknown(String)
  static func read(_ json: JSONValue) throws -> Verb {
    let value = try String.read(json)
    switch value {
    case "recognize": return .`recognize`
    default: return .unknown(value)
    }
  }
  var json: JSONValue {
    switch self {
    case .`recognize`: return .string("recognize")
    case .unknown(let value): return .string(value)
    }
  }
}
public enum ReceiptVersion: JSONRepresentable {
  case `thinkthenResult_2`
  case unknown(String)
  static func read(_ json: JSONValue) throws -> ReceiptVersion {
    let value = try String.read(json)
    switch value {
    case "thinkthen.result/2": return .`thinkthenResult_2`
    default: return .unknown(value)
    }
  }
  var json: JSONValue {
    switch self {
    case .`thinkthenResult_2`: return .string("thinkthen.result/2")
    case .unknown(let value): return .string(value)
    }
  }
}
public typealias WordingVersion = UInt64
public indirect enum AnnotatedField: JSONRepresentable {
  case `boolean`(Bool)
  case `null`
  case `string`(String)
  case `list`([String])
  case `number`(Double)
  case `completefailed`(Failed)
  static func read(_ json: JSONValue) throws -> AnnotatedField {
    if let value = try? Bool.read(json) { return .`boolean`(value) }
    if json == .null { return .`null` }
    if let value = try? String.read(json) { return .`string`(value) }
    if let value = try? [String].read(json) { return .`list`(value) }
    if let value = try? Double.read(json) { return .`number`(value) }
    if let value = try? Failed.read(json) { return .`completefailed`(value) }
    throw JSONConversionError("Unknown AnnotatedField value")
  }
  var json: JSONValue {
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
public indirect enum Answer: JSONRepresentable {
  case `yesNo`(AnswerYesNo)
  case `choice`(AnswerChoice)
  case `tag`(AnswerTag)
  case `score`(AnswerScore)
  static func read(_ json: JSONValue) throws -> Answer {
    let object = try jsonObject(json)
    if object["kind"] == .string("yes_no") { return .`yesNo`(try AnswerYesNo.read(json)) }
    if object["kind"] == .string("choice") { return .`choice`(try AnswerChoice.read(json)) }
    if object["kind"] == .string("tag") { return .`tag`(try AnswerTag.read(json)) }
    if object["kind"] == .string("score") { return .`score`(try AnswerScore.read(json)) }
    throw JSONConversionError("Unknown Answer alternative")
  }
  var json: JSONValue {
    switch self {
    case .`yesNo`(let value): return value.json
    case .`choice`(let value): return value.json
    case .`tag`(let value): return value.json
    case .`score`(let value): return value.json
    }
  }
}
public struct AnswerChoice: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `confidence`: Presence<Double>
  public let `kind`: String
  public let `pick`: String
  public let `probabilities`: [String: Double]
  public init(
    extensions: [String: JSONValue] = [:], `confidence`: Presence<Double> = .absent,
    `kind`: String = "choice", `pick`: String, `probabilities`: [String: Double]
  ) {
    self.extensions = extensions
    self.`confidence` = `confidence`
    self.`kind` = `kind`
    self.`pick` = `pick`
    self.`probabilities` = `probabilities`
  }
  static func read(_ json: JSONValue) throws -> AnswerChoice {
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
  var json: JSONValue {
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
public struct AnswerScore: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `confidence`: Presence<Double>
  public let `kind`: String
  public let `level`: String
  public let `probabilities`: [String: Double]
  public init(
    extensions: [String: JSONValue] = [:], `confidence`: Presence<Double> = .absent,
    `kind`: String = "score", `level`: String, `probabilities`: [String: Double]
  ) {
    self.extensions = extensions
    self.`confidence` = `confidence`
    self.`kind` = `kind`
    self.`level` = `level`
    self.`probabilities` = `probabilities`
  }
  static func read(_ json: JSONValue) throws -> AnswerScore {
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
  var json: JSONValue {
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
public struct AnswerTag: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `kind`: String
  public let `probabilities`: [String: Double]
  public init(
    extensions: [String: JSONValue] = [:], `kind`: String = "tag", `probabilities`: [String: Double]
  ) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`probabilities` = `probabilities`
  }
  static func read(_ json: JSONValue) throws -> AnswerTag {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "probabilities"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      probabilities: try [String: Double].read(jsonRequired(object, "probabilities")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "probabilities"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["probabilities"] = `probabilities`.json
    return .object(object)
  }
}
public struct AnswerYesNo: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `kind`: String
  public let `probability`: Double
  public init(
    extensions: [String: JSONValue] = [:], `kind`: String = "yes_no", `probability`: Double
  ) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`probability` = `probability`
  }
  static func read(_ json: JSONValue) throws -> AnswerYesNo {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "probability"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      probability: try Double.read(jsonRequired(object, "probability")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "probability"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["probability"] = `probability`.json
    return .object(object)
  }
}
public enum AttemptOutcome: JSONRepresentable {
  case `ok`
  case `status`
  case `transport`
  case unknown(String)
  static func read(_ json: JSONValue) throws -> AttemptOutcome {
    let value = try String.read(json)
    switch value {
    case "ok": return .`ok`
    case "status": return .`status`
    case "transport": return .`transport`
    default: return .unknown(value)
    }
  }
  var json: JSONValue {
    switch self {
    case .`ok`: return .string("ok")
    case .`status`: return .string("status")
    case .`transport`: return .string("transport")
    case .unknown(let value): return .string(value)
    }
  }
}
public indirect enum BatchSetting: JSONRepresentable {
  case `integer`(UInt64)
  case `string`(String)
  static func read(_ json: JSONValue) throws -> BatchSetting {
    if let value = try? UInt64.read(json) { return .`integer`(value) }
    if let value = try? String.read(json) { return .`string`(value) }
    throw JSONConversionError("Unknown BatchSetting value")
  }
  var json: JSONValue {
    switch self {
    case .`integer`(let value): return value.json
    case .`string`(let value): return value.json
    }
  }
}
public struct BatchWarning: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `running`: BatchSetting
  public let `tunedFor`: BatchSetting
  public init(
    extensions: [String: JSONValue] = [:], `running`: BatchSetting, `tunedFor`: BatchSetting
  ) {
    self.extensions = extensions
    self.`running` = `running`
    self.`tunedFor` = `tunedFor`
  }
  static func read(_ json: JSONValue) throws -> BatchWarning {
    let object = try jsonObject(json)
    let known: Set<String> = ["running", "tuned_for"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      running: try BatchSetting.read(jsonRequired(object, "running")),
      tunedFor: try BatchSetting.read(jsonRequired(object, "tuned_for")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["running", "tuned_for"]
    var object = extensions.filter { !known.contains($0.key) }
    object["running"] = `running`.json
    object["tuned_for"] = `tunedFor`.json
    return .object(object)
  }
}
public struct Entity: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
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
    extensions: [String: JSONValue] = [:], `end`: UInt64, `file`: Presence<String> = .absent,
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
  static func read(_ json: JSONValue) throws -> Entity {
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
  var json: JSONValue {
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
public struct EntityEdge: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `either`: Presence<Bool>
  public let `probability`: Double
  public let `relation`: String
  public let `source`: Entity
  public let `target`: Entity
  public init(
    extensions: [String: JSONValue] = [:], `either`: Presence<Bool> = .absent,
    `probability`: Double, `relation`: String, `source`: Entity, `target`: Entity
  ) {
    self.extensions = extensions
    self.`either` = `either`
    self.`probability` = `probability`
    self.`relation` = `relation`
    self.`source` = `source`
    self.`target` = `target`
  }
  static func read(_ json: JSONValue) throws -> EntityEdge {
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
  var json: JSONValue {
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
public struct Failed: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `failed`: Failure
  public init(extensions: [String: JSONValue] = [:], `failed`: Failure) {
    self.extensions = extensions
    self.`failed` = `failed`
  }
  static func read(_ json: JSONValue) throws -> Failed {
    let object = try jsonObject(json)
    let known: Set<String> = ["failed"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      failed: try Failure.read(jsonRequired(object, "failed")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["failed"]
    var object = extensions.filter { !known.contains($0.key) }
    object["failed"] = `failed`.json
    return .object(object)
  }
}
public struct Failure: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `cause`: FailureCause
  public let `kind`: String
  public init(extensions: [String: JSONValue] = [:], `cause`: FailureCause, `kind`: String) {
    self.extensions = extensions
    self.`cause` = `cause`
    self.`kind` = `kind`
  }
  static func read(_ json: JSONValue) throws -> Failure {
    let object = try jsonObject(json)
    let known: Set<String> = ["cause", "kind"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      cause: try FailureCause.read(jsonRequired(object, "cause")),
      kind: try String.read(jsonRequired(object, "kind")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["cause", "kind"]
    var object = extensions.filter { !known.contains($0.key) }
    object["cause"] = `cause`.json
    object["kind"] = `kind`.json
    return .object(object)
  }
}
public enum FailureCause: JSONRepresentable {
  case `missingAnswer`
  case `wrongKind`
  case `missingProbability`
  case `invalidProbability`
  case `invalidDistribution`
  case `unexpectedProbability`
  case unknown(String)
  static func read(_ json: JSONValue) throws -> FailureCause {
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
  var json: JSONValue {
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
public enum FailureKind: JSONRepresentable {
  case `usage`
  case `backend`
  case `local`
  case `cancelled`
  case `deadline`
  case `defect`
  case unknown(String)
  static func read(_ json: JSONValue) throws -> FailureKind {
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
  var json: JSONValue {
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
public struct FindAnswer: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `confidence`: Presence<Double>
  public let `kind`: String
  public let `pick`: String
  public let `probabilities`: [String: Double]
  public init(
    extensions: [String: JSONValue] = [:], `confidence`: Presence<Double> = .absent,
    `kind`: String = "find", `pick`: String, `probabilities`: [String: Double]
  ) {
    self.extensions = extensions
    self.`confidence` = `confidence`
    self.`kind` = `kind`
    self.`pick` = `pick`
    self.`probabilities` = `probabilities`
  }
  static func read(_ json: JSONValue) throws -> FindAnswer {
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
  var json: JSONValue {
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
public struct FindQuestionDocument: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `batch`: Presence<ReceiptBatch>
  public let `contextSchema`: Presence<InputDeclaration>
  public let `itemSchema`: Presence<InputDeclaration>
  public let `labelDetails`: Presence<[Label]>
  public let `model`: Presence<String>
  public let `name`: Presence<ReceiptQuestionName>
  public let `none`: Bool
  public let `on`: Presence<[String]>
  public let `profile`: Presence<String>
  public let `text`: JSONValue
  public let `verb`: String
  public let `wordingVersion`: Presence<WordingVersion>
  public init(
    extensions: [String: JSONValue] = [:], `batch`: Presence<ReceiptBatch> = .absent,
    `contextSchema`: Presence<InputDeclaration> = .absent,
    `itemSchema`: Presence<InputDeclaration> = .absent, `labelDetails`: Presence<[Label]> = .absent,
    `model`: Presence<String> = .absent, `name`: Presence<ReceiptQuestionName> = .absent,
    `none`: Bool, `on`: Presence<[String]> = .absent, `profile`: Presence<String> = .absent,
    `text`: JSONValue, `verb`: String = "find", `wordingVersion`: Presence<WordingVersion> = .absent
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
  static func read(_ json: JSONValue) throws -> FindQuestionDocument {
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
      text: try JSONValue.read(jsonRequired(object, "text")),
      verb: try String.read(jsonRequired(object, "verb")),
      wordingVersion: try readPresence(object, "wording_version", WordingVersion.self))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
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
public struct NameOdds: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `edges`: [String: Double]?
  public let `end`: UInt64
  public let `kinds`: [String: Double]?
  public let `start`: UInt64
  public init(
    extensions: [String: JSONValue] = [:], `edges`: [String: Double]?, `end`: UInt64,
    `kinds`: [String: Double]?, `start`: UInt64
  ) {
    self.extensions = extensions
    self.`edges` = `edges`
    self.`end` = `end`
    self.`kinds` = `kinds`
    self.`start` = `start`
  }
  static func read(_ json: JSONValue) throws -> NameOdds {
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
  var json: JSONValue {
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
public struct PairOdds: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `probability`: Double
  public let `relation`: String
  public let `source`: Place
  public let `target`: Place
  public init(
    extensions: [String: JSONValue] = [:], `probability`: Double, `relation`: String,
    `source`: Place, `target`: Place
  ) {
    self.extensions = extensions
    self.`probability` = `probability`
    self.`relation` = `relation`
    self.`source` = `source`
    self.`target` = `target`
  }
  static func read(_ json: JSONValue) throws -> PairOdds {
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
  var json: JSONValue {
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
public struct PieceOdds: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `end`: UInt64
  public let `start`: UInt64
  public let `tags`: [String: Double]
  public init(
    extensions: [String: JSONValue] = [:], `end`: UInt64, `start`: UInt64, `tags`: [String: Double]
  ) {
    self.extensions = extensions
    self.`end` = `end`
    self.`start` = `start`
    self.`tags` = `tags`
  }
  static func read(_ json: JSONValue) throws -> PieceOdds {
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
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["end", "start", "tags"]
    var object = extensions.filter { !known.contains($0.key) }
    object["end"] = `end`.json
    object["start"] = `start`.json
    object["tags"] = `tags`.json
    return .object(object)
  }
}
public struct Place: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `end`: UInt64
  public let `start`: UInt64
  public init(extensions: [String: JSONValue] = [:], `end`: UInt64, `start`: UInt64) {
    self.extensions = extensions
    self.`end` = `end`
    self.`start` = `start`
  }
  static func read(_ json: JSONValue) throws -> Place {
    let object = try jsonObject(json)
    let known: Set<String> = ["end", "start"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      end: try UInt64.read(jsonRequired(object, "end")),
      start: try UInt64.read(jsonRequired(object, "start")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["end", "start"]
    var object = extensions.filter { !known.contains($0.key) }
    object["end"] = `end`.json
    object["start"] = `start`.json
    return .object(object)
  }
}
public struct Plan: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
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
    extensions: [String: JSONValue] = [:], `estimatedBytes`: UInt64,
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
  static func read(_ json: JSONValue) throws -> Plan {
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
  var json: JSONValue {
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
public struct ProfileWarning: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `running`: String
  public let `tunedFor`: String
  public init(extensions: [String: JSONValue] = [:], `running`: String, `tunedFor`: String) {
    self.extensions = extensions
    self.`running` = `running`
    self.`tunedFor` = `tunedFor`
  }
  static func read(_ json: JSONValue) throws -> ProfileWarning {
    let object = try jsonObject(json)
    let known: Set<String> = ["running", "tuned_for"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      running: try String.read(jsonRequired(object, "running")),
      tunedFor: try String.read(jsonRequired(object, "tuned_for")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["running", "tuned_for"]
    var object = extensions.filter { !known.contains($0.key) }
    object["running"] = `running`.json
    object["tuned_for"] = `tunedFor`.json
    return .object(object)
  }
}
public indirect enum QuestionDocument: JSONRepresentable {
  case `decide`(QuestionDocumentDecide)
  case `choose`(QuestionDocumentChoose)
  case `tag`(QuestionDocumentTag)
  case `score`(QuestionDocumentScore)
  static func read(_ json: JSONValue) throws -> QuestionDocument {
    let object = try jsonObject(json)
    if object["verb"] == .string("decide") {
      return .`decide`(try QuestionDocumentDecide.read(json))
    }
    if object["verb"] == .string("choose") {
      return .`choose`(try QuestionDocumentChoose.read(json))
    }
    if object["verb"] == .string("tag") { return .`tag`(try QuestionDocumentTag.read(json)) }
    if object["verb"] == .string("score") { return .`score`(try QuestionDocumentScore.read(json)) }
    throw JSONConversionError("Unknown QuestionDocument alternative")
  }
  var json: JSONValue {
    switch self {
    case .`decide`(let value): return value.json
    case .`choose`(let value): return value.json
    case .`tag`(let value): return value.json
    case .`score`(let value): return value.json
    }
  }
}
public struct QuestionDocumentChoose: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
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
  public let `text`: JSONValue
  public let `verb`: String
  public init(
    extensions: [String: JSONValue] = [:], `batch`: Presence<ReceiptBatch> = .absent,
    `contextSchema`: Presence<InputDeclaration> = .absent,
    `itemSchema`: Presence<InputDeclaration> = .absent, `labelDetails`: Presence<[Label]> = .absent,
    `model`: Presence<String> = .absent, `name`: Presence<ReceiptQuestionName> = .absent,
    `on`: Presence<[String]> = .absent, `profile`: Presence<String> = .absent,
    `wordingVersion`: Presence<WordingVersion> = .absent, `options`: [String], `text`: JSONValue,
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
  static func read(_ json: JSONValue) throws -> QuestionDocumentChoose {
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
      text: try JSONValue.read(jsonRequired(object, "text")),
      verb: try String.read(jsonRequired(object, "verb")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
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
public struct QuestionDocumentDecide: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `batch`: Presence<ReceiptBatch>
  public let `contextSchema`: Presence<InputDeclaration>
  public let `itemSchema`: Presence<InputDeclaration>
  public let `labelDetails`: Presence<[Label]>
  public let `model`: Presence<String>
  public let `name`: Presence<ReceiptQuestionName>
  public let `on`: Presence<[String]>
  public let `profile`: Presence<String>
  public let `wordingVersion`: Presence<WordingVersion>
  public let `false`: Presence<JSONValue>
  public let `text`: JSONValue
  public let `true`: Presence<JSONValue>
  public let `verb`: String
  public init(
    extensions: [String: JSONValue] = [:], `batch`: Presence<ReceiptBatch> = .absent,
    `contextSchema`: Presence<InputDeclaration> = .absent,
    `itemSchema`: Presence<InputDeclaration> = .absent, `labelDetails`: Presence<[Label]> = .absent,
    `model`: Presence<String> = .absent, `name`: Presence<ReceiptQuestionName> = .absent,
    `on`: Presence<[String]> = .absent, `profile`: Presence<String> = .absent,
    `wordingVersion`: Presence<WordingVersion> = .absent, `false`: Presence<JSONValue> = .absent,
    `text`: JSONValue, `true`: Presence<JSONValue> = .absent, `verb`: String = "decide"
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
  static func read(_ json: JSONValue) throws -> QuestionDocumentDecide {
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
      false: try readPresence(object, "false", JSONValue.self),
      text: try JSONValue.read(jsonRequired(object, "text")),
      true: try readPresence(object, "true", JSONValue.self),
      verb: try String.read(jsonRequired(object, "verb")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
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
public struct QuestionDocumentScore: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
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
  public let `text`: JSONValue
  public let `verb`: String
  public init(
    extensions: [String: JSONValue] = [:], `batch`: Presence<ReceiptBatch> = .absent,
    `contextSchema`: Presence<InputDeclaration> = .absent,
    `itemSchema`: Presence<InputDeclaration> = .absent, `labelDetails`: Presence<[Label]> = .absent,
    `model`: Presence<String> = .absent, `name`: Presence<ReceiptQuestionName> = .absent,
    `on`: Presence<[String]> = .absent, `profile`: Presence<String> = .absent,
    `wordingVersion`: Presence<WordingVersion> = .absent, `levels`: [String], `text`: JSONValue,
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
  static func read(_ json: JSONValue) throws -> QuestionDocumentScore {
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
      text: try JSONValue.read(jsonRequired(object, "text")),
      verb: try String.read(jsonRequired(object, "verb")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
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
public struct QuestionDocumentTag: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
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
  public let `text`: JSONValue
  public let `verb`: String
  public init(
    extensions: [String: JSONValue] = [:], `batch`: Presence<ReceiptBatch> = .absent,
    `contextSchema`: Presence<InputDeclaration> = .absent,
    `itemSchema`: Presence<InputDeclaration> = .absent, `labelDetails`: Presence<[Label]> = .absent,
    `model`: Presence<String> = .absent, `name`: Presence<ReceiptQuestionName> = .absent,
    `on`: Presence<[String]> = .absent, `profile`: Presence<String> = .absent,
    `wordingVersion`: Presence<WordingVersion> = .absent, `labels`: [String], `text`: JSONValue,
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
  static func read(_ json: JSONValue) throws -> QuestionDocumentTag {
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
      text: try JSONValue.read(jsonRequired(object, "text")),
      verb: try String.read(jsonRequired(object, "verb")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
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
public indirect enum Recognize: JSONRepresentable {
  case `entities`(RecognizeEntities)
  case `modeProposals`(RecognizeModeProposals)
  static func read(_ json: JSONValue) throws -> Recognize {
    let object = try jsonObject(json)
    if object["entities"] != nil && object["mode"] == nil && object["proposals"] == nil {
      return .`entities`(try RecognizeEntities.read(json))
    }
    if object["mode"] != nil && object["proposals"] != nil && object["entities"] == nil {
      return .`modeProposals`(try RecognizeModeProposals.read(json))
    }
    throw JSONConversionError("Unknown Recognize alternative")
  }
  var json: JSONValue {
    switch self {
    case .`entities`(let value): return value.json
    case .`modeProposals`(let value): return value.json
    }
  }
}
public struct RecognizeAnswer: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `names`: [NameOdds]
  public let `pairs`: [PairOdds]
  public let `pieces`: [PieceOdds]
  public let `proposals`: [RecognitionProposal]
  public init(
    extensions: [String: JSONValue] = [:], `names`: [NameOdds], `pairs`: [PairOdds],
    `pieces`: [PieceOdds], `proposals`: [RecognitionProposal]
  ) {
    self.extensions = extensions
    self.`names` = `names`
    self.`pairs` = `pairs`
    self.`pieces` = `pieces`
    self.`proposals` = `proposals`
  }
  static func read(_ json: JSONValue) throws -> RecognizeAnswer {
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
  var json: JSONValue {
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
public struct RecognizeQuestionDocument: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `batch`: Presence<ReceiptBatch>
  public let `contextSchema`: Presence<InputDeclaration>
  public let `entityDefinition`: Presence<JSONValue>
  public let `instructions`: Presence<JSONValue>
  public let `itemSchema`: Presence<InputDeclaration>
  public let `kinds`: [String: JSONValue]
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
    extensions: [String: JSONValue] = [:], `batch`: Presence<ReceiptBatch> = .absent,
    `contextSchema`: Presence<InputDeclaration> = .absent,
    `entityDefinition`: Presence<JSONValue> = .absent,
    `instructions`: Presence<JSONValue> = .absent,
    `itemSchema`: Presence<InputDeclaration> = .absent, `kinds`: [String: JSONValue],
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
  static func read(_ json: JSONValue) throws -> RecognizeQuestionDocument {
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
      entityDefinition: try readPresence(object, "entity_definition", JSONValue.self),
      instructions: try readPresence(object, "instructions", JSONValue.self),
      itemSchema: try readPresence(object, "item_schema", InputDeclaration.self),
      kinds: try [String: JSONValue].read(jsonRequired(object, "kinds")),
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
  var json: JSONValue {
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
public struct RecognizeEntities: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `entities`: [Entity]
  public let `relations`: Presence<[EntityEdge]>
  public init(
    extensions: [String: JSONValue] = [:], `entities`: [Entity],
    `relations`: Presence<[EntityEdge]> = .absent
  ) {
    self.extensions = extensions
    self.`entities` = `entities`
    self.`relations` = `relations`
  }
  static func read(_ json: JSONValue) throws -> RecognizeEntities {
    let object = try jsonObject(json)
    let known: Set<String> = ["entities", "relations"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      entities: try [Entity].read(jsonRequired(object, "entities")),
      relations: try readPresence(object, "relations", [EntityEdge].self))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["entities", "relations"]
    var object = extensions.filter { !known.contains($0.key) }
    object["entities"] = `entities`.json
    writePresence(`relations`, "relations", &object)
    return .object(object)
  }
}
public struct RecognizeModeProposals: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `mode`: BoundaryMode
  public let `proposals`: [BoundaryProposal]
  public init(
    extensions: [String: JSONValue] = [:], `mode`: BoundaryMode, `proposals`: [BoundaryProposal]
  ) {
    self.extensions = extensions
    self.`mode` = `mode`
    self.`proposals` = `proposals`
  }
  static func read(_ json: JSONValue) throws -> RecognizeModeProposals {
    let object = try jsonObject(json)
    let known: Set<String> = ["mode", "proposals"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      mode: try BoundaryMode.read(jsonRequired(object, "mode")),
      proposals: try [BoundaryProposal].read(jsonRequired(object, "proposals")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["mode", "proposals"]
    var object = extensions.filter { !known.contains($0.key) }
    object["mode"] = `mode`.json
    object["proposals"] = `proposals`.json
    return .object(object)
  }
}
public struct ReceiptRelateFields: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `kind`: String
  public let `name`: String
  public init(extensions: [String: JSONValue] = [:], `kind`: String, `name`: String) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`name` = `name`
  }
  static func read(_ json: JSONValue) throws -> ReceiptRelateFields {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "name"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      name: try String.read(jsonRequired(object, "name")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "name"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["name"] = `name`.json
    return .object(object)
  }
}
public struct RelateQuestionDocument: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
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
    extensions: [String: JSONValue] = [:], `batch`: Presence<ReceiptBatch> = .absent,
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
  static func read(_ json: JSONValue) throws -> RelateQuestionDocument {
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
  var json: JSONValue {
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
public struct RelatedEntity: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `kind`: String
  public let `name`: String
  public init(extensions: [String: JSONValue] = [:], `kind`: String, `name`: String) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`name` = `name`
  }
  static func read(_ json: JSONValue) throws -> RelatedEntity {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "name"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      name: try String.read(jsonRequired(object, "name")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "name"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["name"] = `name`.json
    return .object(object)
  }
}
public struct RelatedEntityEdge: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `either`: Presence<Bool>
  public let `probability`: Double
  public let `relation`: String
  public let `source`: RelationEndpoint
  public let `target`: RelationEndpoint
  public init(
    extensions: [String: JSONValue] = [:], `either`: Presence<Bool> = .absent,
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
  static func read(_ json: JSONValue) throws -> RelatedEntityEdge {
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
  var json: JSONValue {
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
    switch try readPresence(jsonObject(self.json), "source", JSONValue.self) {
    case .absent: return .absent
    case .null: return .null
    case .value(let object): return try readPresence(jsonObject(object), "name", String.self)
    }
  }
  public func targetName() throws -> Presence<String> {
    switch try readPresence(jsonObject(self.json), "target", JSONValue.self) {
    case .absent: return .absent
    case .null: return .null
    case .value(let object): return try readPresence(jsonObject(object), "name", String.self)
    }
  }
}
public struct RelationRule: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `either`: Bool
  public let `name`: String
  public let `reads`: String
  public let `single`: Presence<Bool>
  public let `source`: String
  public let `target`: String
  public init(
    extensions: [String: JSONValue] = [:], `either`: Bool, `name`: String, `reads`: String,
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
  static func read(_ json: JSONValue) throws -> RelationRule {
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
  var json: JSONValue {
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
public struct SessionAnnotation: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `name`: String
  public let `value`: AnnotationValue
  public init(extensions: [String: JSONValue] = [:], `name`: String, `value`: AnnotationValue) {
    self.extensions = extensions
    self.`name` = `name`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> SessionAnnotation {
    let object = try jsonObject(json)
    let known: Set<String> = ["name", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      name: try String.read(jsonRequired(object, "name")),
      value: try AnnotationValue.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["name", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["name"] = `name`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionInputSource: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `index`: UInt64
  public let `source`: PhysicalSource
  public init(extensions: [String: JSONValue] = [:], `index`: UInt64, `source`: PhysicalSource) {
    self.extensions = extensions
    self.`index` = `index`
    self.`source` = `source`
  }
  static func read(_ json: JSONValue) throws -> SessionInputSource {
    let object = try jsonObject(json)
    let known: Set<String> = ["index", "source"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      index: try UInt64.read(jsonRequired(object, "index")),
      source: try PhysicalSource.read(jsonRequired(object, "source")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["index", "source"]
    var object = extensions.filter { !known.contains($0.key) }
    object["index"] = `index`.json
    object["source"] = `source`.json
    return .object(object)
  }
}
public indirect enum SessionJudgment: JSONRepresentable {
  case `decision`(SessionJudgmentDecision)
  case `choice`(SessionJudgmentChoice)
  case `score`(SessionJudgmentScore)
  case `tags`(SessionJudgmentTags)
  static func read(_ json: JSONValue) throws -> SessionJudgment {
    let object = try jsonObject(json)
    if object["kind"] == .string("decision") {
      return .`decision`(try SessionJudgmentDecision.read(json))
    }
    if object["kind"] == .string("choice") {
      return .`choice`(try SessionJudgmentChoice.read(json))
    }
    if object["kind"] == .string("score") { return .`score`(try SessionJudgmentScore.read(json)) }
    if object["kind"] == .string("tags") { return .`tags`(try SessionJudgmentTags.read(json)) }
    throw JSONConversionError("Unknown SessionJudgment alternative")
  }
  var json: JSONValue {
    switch self {
    case .`decision`(let value): return value.json
    case .`choice`(let value): return value.json
    case .`score`(let value): return value.json
    case .`tags`(let value): return value.json
    }
  }
}
public struct SessionJudgmentChoice: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `kind`: String
  public let `value`: String?
  public init(extensions: [String: JSONValue] = [:], `kind`: String = "choice", `value`: String?) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> SessionJudgmentChoice {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try String?.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionJudgmentDecision: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `kind`: String
  public let `value`: Bool?
  public init(extensions: [String: JSONValue] = [:], `kind`: String = "decision", `value`: Bool?) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> SessionJudgmentDecision {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try Bool?.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionJudgmentScore: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `kind`: String
  public let `value`: Double
  public init(extensions: [String: JSONValue] = [:], `kind`: String = "score", `value`: Double) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> SessionJudgmentScore {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try Double.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionJudgmentTags: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `kind`: String
  public let `value`: [String]
  public init(extensions: [String: JSONValue] = [:], `kind`: String = "tags", `value`: [String]) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> SessionJudgmentTags {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try [String].read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionNamedProbability: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `name`: String
  public let `probability`: Double
  public init(extensions: [String: JSONValue] = [:], `name`: String, `probability`: Double) {
    self.extensions = extensions
    self.`name` = `name`
    self.`probability` = `probability`
  }
  static func read(_ json: JSONValue) throws -> SessionNamedProbability {
    let object = try jsonObject(json)
    let known: Set<String> = ["name", "probability"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      name: try String.read(jsonRequired(object, "name")),
      probability: try Double.read(jsonRequired(object, "probability")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["name", "probability"]
    var object = extensions.filter { !known.contains($0.key) }
    object["name"] = `name`.json
    object["probability"] = `probability`.json
    return .object(object)
  }
}
public indirect enum SessionObservation: JSONRepresentable {
  case `question`(SessionObservationQuestion)
  case `row`(SessionObservationRow)
  static func read(_ json: JSONValue) throws -> SessionObservation {
    let object = try jsonObject(json)
    if object["kind"] == .string("question") {
      return .`question`(try SessionObservationQuestion.read(json))
    }
    if object["kind"] == .string("row") { return .`row`(try SessionObservationRow.read(json)) }
    throw JSONConversionError("Unknown SessionObservation alternative")
  }
  var json: JSONValue {
    switch self {
    case .`question`(let value): return value.json
    case .`row`(let value): return value.json
    }
  }
}
public struct SessionObservationQuestion: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `detail`: SessionQuestionDetail
  public let `index`: UInt64
  public let `kind`: String
  public let `member`: Presence<String>
  public let `position`: UInt64
  public let `stage`: Presence<String>
  public init(
    extensions: [String: JSONValue] = [:], `detail`: SessionQuestionDetail, `index`: UInt64,
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
  static func read(_ json: JSONValue) throws -> SessionObservationQuestion {
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
  var json: JSONValue {
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
public struct SessionObservationRow: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `index`: UInt64
  public let `kind`: String
  public let `value`: SessionObservedRow
  public init(
    extensions: [String: JSONValue] = [:], `index`: UInt64, `kind`: String = "row",
    `value`: SessionObservedRow
  ) {
    self.extensions = extensions
    self.`index` = `index`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> SessionObservationRow {
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
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["index", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["index"] = `index`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public indirect enum SessionObservedRow: JSONRepresentable {
  case `judgment`(SessionObservedRowJudgment)
  case `annotated`(SessionObservedRowAnnotated)
  case `recognized`(SessionObservedRowRecognized)
  case `find`(SessionObservedRowFind)
  case `relations`(SessionObservedRowRelations)
  static func read(_ json: JSONValue) throws -> SessionObservedRow {
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
    throw JSONConversionError("Unknown SessionObservedRow alternative")
  }
  var json: JSONValue {
    switch self {
    case .`judgment`(let value): return value.json
    case .`annotated`(let value): return value.json
    case .`recognized`(let value): return value.json
    case .`find`(let value): return value.json
    case .`relations`(let value): return value.json
    }
  }
}
public struct SessionObservedRowAnnotated: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `kind`: String
  public let `value`: [SessionAnnotation]
  public init(
    extensions: [String: JSONValue] = [:], `kind`: String = "annotated",
    `value`: [SessionAnnotation]
  ) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> SessionObservedRowAnnotated {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try [SessionAnnotation].read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionObservedRowFind: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `kind`: String
  public let `value`: UInt64?
  public init(extensions: [String: JSONValue] = [:], `kind`: String = "find", `value`: UInt64?) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> SessionObservedRowFind {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try UInt64?.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionObservedRowJudgment: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `kind`: String
  public let `value`: SessionJudgment
  public init(
    extensions: [String: JSONValue] = [:], `kind`: String = "judgment", `value`: SessionJudgment
  ) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> SessionObservedRowJudgment {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try SessionJudgment.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionObservedRowRecognized: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `kind`: String
  public let `value`: SessionRecognition
  public init(
    extensions: [String: JSONValue] = [:], `kind`: String = "recognized",
    `value`: SessionRecognition
  ) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> SessionObservedRowRecognized {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try SessionRecognition.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionObservedRowRelations: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `kind`: String
  public let `value`: [SessionRelationEdge]
  public init(
    extensions: [String: JSONValue] = [:], `kind`: String = "relations",
    `value`: [SessionRelationEdge]
  ) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> SessionObservedRowRelations {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try [SessionRelationEdge].read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public indirect enum SessionPacket: JSONRepresentable {
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
  static func read(_ json: JSONValue) throws -> SessionPacket {
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
    throw JSONConversionError("Unknown SessionPacket alternative")
  }
  var json: JSONValue {
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
public struct SessionPacketAnnotateAggregate: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `function`: String
  public let `kind`: String
  public let `value`: [Annotation]
  public init(
    extensions: [String: JSONValue] = [:], `function`: String = "annotate",
    `kind`: String = "aggregate", `value`: [Annotation]
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> SessionPacketAnnotateAggregate {
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
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketAnnotateRow: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `function`: String
  public let `kind`: String
  public let `value`: Annotation
  public init(
    extensions: [String: JSONValue] = [:], `function`: String = "annotate", `kind`: String = "row",
    `value`: Annotation
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> SessionPacketAnnotateRow {
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
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketChooseAggregate: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `function`: String
  public let `kind`: String
  public let `value`: [AtomicNullableString]
  public init(
    extensions: [String: JSONValue] = [:], `function`: String = "choose",
    `kind`: String = "aggregate", `value`: [AtomicNullableString]
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> SessionPacketChooseAggregate {
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
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketChooseRow: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `function`: String
  public let `kind`: String
  public let `value`: AtomicNullableString
  public init(
    extensions: [String: JSONValue] = [:], `function`: String = "choose", `kind`: String = "row",
    `value`: AtomicNullableString
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> SessionPacketChooseRow {
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
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketDecideAggregate: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `function`: String
  public let `kind`: String
  public let `value`: [AtomicDecideValue]
  public init(
    extensions: [String: JSONValue] = [:], `function`: String = "decide",
    `kind`: String = "aggregate", `value`: [AtomicDecideValue]
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> SessionPacketDecideAggregate {
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
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketDecideRow: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `function`: String
  public let `kind`: String
  public let `value`: AtomicDecideValue
  public init(
    extensions: [String: JSONValue] = [:], `function`: String = "decide", `kind`: String = "row",
    `value`: AtomicDecideValue
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> SessionPacketDecideRow {
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
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketFilterAggregate: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `function`: String
  public let `kind`: String
  public let `value`: [AtomicBoolean]
  public init(
    extensions: [String: JSONValue] = [:], `function`: String = "filter",
    `kind`: String = "aggregate", `value`: [AtomicBoolean]
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> SessionPacketFilterAggregate {
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
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketFilterRow: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `function`: String
  public let `kind`: String
  public let `value`: AtomicBoolean
  public init(
    extensions: [String: JSONValue] = [:], `function`: String = "filter", `kind`: String = "row",
    `value`: AtomicBoolean
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> SessionPacketFilterRow {
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
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketFindAggregate: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `function`: String
  public let `kind`: String
  public let `value`: Find
  public init(
    extensions: [String: JSONValue] = [:], `function`: String = "find",
    `kind`: String = "aggregate", `value`: Find
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> SessionPacketFindAggregate {
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
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketObservation: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `function`: ReceiptFunction
  public let `kind`: String
  public let `value`: SessionObservation
  public init(
    extensions: [String: JSONValue] = [:], `function`: ReceiptFunction,
    `kind`: String = "observation", `value`: SessionObservation
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> SessionPacketObservation {
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
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketRankAggregate: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `function`: String
  public let `kind`: String
  public let `value`: [AtomicNonZeroUsize]
  public init(
    extensions: [String: JSONValue] = [:], `function`: String = "rank",
    `kind`: String = "aggregate", `value`: [AtomicNonZeroUsize]
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> SessionPacketRankAggregate {
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
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketRecognizeAggregate: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `function`: String
  public let `kind`: String
  public let `value`: [Recognition]
  public init(
    extensions: [String: JSONValue] = [:], `function`: String = "recognize",
    `kind`: String = "aggregate", `value`: [Recognition]
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> SessionPacketRecognizeAggregate {
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
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketRelateAggregate: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `function`: String
  public let `kind`: String
  public let `value`: Relation
  public init(
    extensions: [String: JSONValue] = [:], `function`: String = "relate",
    `kind`: String = "aggregate", `value`: Relation
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> SessionPacketRelateAggregate {
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
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketScoreAggregate: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `function`: String
  public let `kind`: String
  public let `value`: [AtomicDouble]
  public init(
    extensions: [String: JSONValue] = [:], `function`: String = "score",
    `kind`: String = "aggregate", `value`: [AtomicDouble]
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> SessionPacketScoreAggregate {
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
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketScoreRow: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `function`: String
  public let `kind`: String
  public let `value`: AtomicDouble
  public init(
    extensions: [String: JSONValue] = [:], `function`: String = "score", `kind`: String = "row",
    `value`: AtomicDouble
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> SessionPacketScoreRow {
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
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketTagAggregate: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `function`: String
  public let `kind`: String
  public let `value`: [AtomicArrayOfString]
  public init(
    extensions: [String: JSONValue] = [:], `function`: String = "tag", `kind`: String = "aggregate",
    `value`: [AtomicArrayOfString]
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> SessionPacketTagAggregate {
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
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketTagRow: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `function`: String
  public let `kind`: String
  public let `value`: AtomicArrayOfString
  public init(
    extensions: [String: JSONValue] = [:], `function`: String = "tag", `kind`: String = "row",
    `value`: AtomicArrayOfString
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> SessionPacketTagRow {
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
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionPacketTerminal: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `facts`: Presence<Facts>
  public let `failure`: Presence<CallError>
  public let `kind`: String
  public init(
    extensions: [String: JSONValue] = [:], `facts`: Presence<Facts> = .absent,
    `failure`: Presence<CallError> = .absent, `kind`: String = "terminal"
  ) {
    self.extensions = extensions
    self.`facts` = `facts`
    self.`failure` = `failure`
    self.`kind` = `kind`
  }
  static func read(_ json: JSONValue) throws -> SessionPacketTerminal {
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
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["facts", "failure", "kind"]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`facts`, "facts", &object)
    writePresence(`failure`, "failure", &object)
    object["kind"] = `kind`.json
    return .object(object)
  }
}
public indirect enum SessionProbabilities: JSONRepresentable {
  case `yesNo`(SessionProbabilitiesYesNo)
  case `named`(SessionProbabilitiesNamed)
  static func read(_ json: JSONValue) throws -> SessionProbabilities {
    let object = try jsonObject(json)
    if object["kind"] == .string("yes_no") {
      return .`yesNo`(try SessionProbabilitiesYesNo.read(json))
    }
    if object["kind"] == .string("named") {
      return .`named`(try SessionProbabilitiesNamed.read(json))
    }
    throw JSONConversionError("Unknown SessionProbabilities alternative")
  }
  var json: JSONValue {
    switch self {
    case .`yesNo`(let value): return value.json
    case .`named`(let value): return value.json
    }
  }
}
public struct SessionProbabilitiesNamed: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `kind`: String
  public let `value`: [SessionNamedProbability]
  public init(
    extensions: [String: JSONValue] = [:], `kind`: String = "named",
    `value`: [SessionNamedProbability]
  ) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> SessionProbabilitiesNamed {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try [SessionNamedProbability].read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionProbabilitiesYesNo: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `kind`: String
  public let `value`: Double
  public init(extensions: [String: JSONValue] = [:], `kind`: String = "yes_no", `value`: Double) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: JSONValue) throws -> SessionProbabilitiesYesNo {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try Double.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct SessionQuestionDetail: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `answerId`: Presence<AnswerId>
  public let `cached`: Bool
  public let `confidence`: Presence<Double>
  public let `failedQuestions`: UInt64
  public let `failure`: Presence<Failure>
  public let `failureId`: Presence<FailureId>
  public let `input`: Presence<JSONValue>
  public let `inputSource`: Presence<PhysicalSource>
  public let `inputSources`: [SessionInputSource]
  public let `inputs`: [JSONValue]
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
    extensions: [String: JSONValue] = [:], `answerId`: Presence<AnswerId> = .absent, `cached`: Bool,
    `confidence`: Presence<Double> = .absent, `failedQuestions`: UInt64,
    `failure`: Presence<Failure> = .absent, `failureId`: Presence<FailureId> = .absent,
    `input`: Presence<JSONValue> = .absent, `inputSource`: Presence<PhysicalSource> = .absent,
    `inputSources`: [SessionInputSource], `inputs`: [JSONValue], `model`: String,
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
  static func read(_ json: JSONValue) throws -> SessionQuestionDetail {
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
      input: try readPresence(object, "input", JSONValue.self),
      inputSource: try readPresence(object, "input_source", PhysicalSource.self),
      inputSources: try [SessionInputSource].read(jsonRequired(object, "input_sources")),
      inputs: try [JSONValue].read(jsonRequired(object, "inputs")),
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
  var json: JSONValue {
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
public struct SessionRecognition: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `entities`: [Entity]
  public let `mode`: ReceiptRecognitionMode
  public let `proposals`: Presence<[BoundaryProposal]>
  public let `relations`: Presence<[RecognitionEdgeDocument]>
  public init(
    extensions: [String: JSONValue] = [:], `entities`: [Entity], `mode`: ReceiptRecognitionMode,
    `proposals`: Presence<[BoundaryProposal]> = .absent,
    `relations`: Presence<[RecognitionEdgeDocument]> = .absent
  ) {
    self.extensions = extensions
    self.`entities` = `entities`
    self.`mode` = `mode`
    self.`proposals` = `proposals`
    self.`relations` = `relations`
  }
  static func read(_ json: JSONValue) throws -> SessionRecognition {
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
  var json: JSONValue {
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
public struct SessionRelationEdge: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `either`: Bool
  public let `probability`: Double
  public let `relation`: String
  public let `source`: EntityDocument
  public let `target`: EntityDocument
  public init(
    extensions: [String: JSONValue] = [:], `either`: Bool, `probability`: Double,
    `relation`: String, `source`: EntityDocument, `target`: EntityDocument
  ) {
    self.extensions = extensions
    self.`either` = `either`
    self.`probability` = `probability`
    self.`relation` = `relation`
    self.`source` = `source`
    self.`target` = `target`
  }
  static func read(_ json: JSONValue) throws -> SessionRelationEdge {
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
  var json: JSONValue {
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
    switch try readPresence(jsonObject(self.json), "source", JSONValue.self) {
    case .absent: return .absent
    case .null: return .null
    case .value(let object): return try readPresence(jsonObject(object), "name", String.self)
    }
  }
  public func targetName() throws -> Presence<String> {
    switch try readPresence(jsonObject(self.json), "target", JSONValue.self) {
    case .absent: return .absent
    case .null: return .null
    case .value(let object): return try readPresence(jsonObject(object), "name", String.self)
    }
  }
}
public struct SourceRelationEndpoint: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `file`: String?
  public let `firstLine`: Presence<UInt64>
  public let `kind`: String
  public let `lastLine`: Presence<UInt64>
  public let `name`: String
  public let `ordinal`: UInt64
  public let `record`: JSONValue
  public init(
    extensions: [String: JSONValue] = [:], `file`: String?, `firstLine`: Presence<UInt64> = .absent,
    `kind`: String, `lastLine`: Presence<UInt64> = .absent, `name`: String, `ordinal`: UInt64,
    `record`: JSONValue
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
  static func read(_ json: JSONValue) throws -> SourceRelationEndpoint {
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
      record: try JSONValue.read(jsonRequired(object, "record")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
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
public indirect enum ReceiptThreshold: JSONRepresentable {
  case `number`(Double)
  case `string`(String)
  static func read(_ json: JSONValue) throws -> ReceiptThreshold {
    if let value = try? Double.read(json) { return .`number`(value) }
    if let value = try? String.read(json) { return .`string`(value) }
    throw JSONConversionError("Unknown ReceiptThreshold value")
  }
  var json: JSONValue {
    switch self {
    case .`number`(let value): return value.json
    case .`string`(let value): return value.json
    }
  }
}
public struct TokenBand: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `lower`: UInt64
  public let `upper`: UInt64
  public init(extensions: [String: JSONValue] = [:], `lower`: UInt64, `upper`: UInt64) {
    self.extensions = extensions
    self.`lower` = `lower`
    self.`upper` = `upper`
  }
  static func read(_ json: JSONValue) throws -> TokenBand {
    let object = try jsonObject(json)
    let known: Set<String> = ["lower", "upper"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      lower: try UInt64.read(jsonRequired(object, "lower")),
      upper: try UInt64.read(jsonRequired(object, "upper")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["lower", "upper"]
    var object = extensions.filter { !known.contains($0.key) }
    object["lower"] = `lower`.json
    object["upper"] = `upper`.json
    return .object(object)
  }
}
public struct TokenUsage: JSONRepresentable {
  public let extensions: [String: JSONValue]
  private var _sourceJSON: JSONValue? = nil
  public let `inputTokens`: UInt64
  public let `outputTokens`: UInt64
  public init(extensions: [String: JSONValue] = [:], `inputTokens`: UInt64, `outputTokens`: UInt64)
  {
    self.extensions = extensions
    self.`inputTokens` = `inputTokens`
    self.`outputTokens` = `outputTokens`
  }
  static func read(_ json: JSONValue) throws -> TokenUsage {
    let object = try jsonObject(json)
    let known: Set<String> = ["input_tokens", "output_tokens"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      inputTokens: try UInt64.read(jsonRequired(object, "input_tokens")),
      outputTokens: try UInt64.read(jsonRequired(object, "output_tokens")))
    value._sourceJSON = json
    return value
  }
  var json: JSONValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["input_tokens", "output_tokens"]
    var object = extensions.filter { !known.contains($0.key) }
    object["input_tokens"] = `inputTokens`.json
    object["output_tokens"] = `outputTokens`.json
    return .object(object)
  }
}
public indirect enum Value: JSONRepresentable {
  case `boolean`(Bool)
  case `null`
  case `string`(String)
  case `list`([String])
  case `number`(Double)
  static func read(_ json: JSONValue) throws -> Value {
    if let value = try? Bool.read(json) { return .`boolean`(value) }
    if json == .null { return .`null` }
    if let value = try? String.read(json) { return .`string`(value) }
    if let value = try? [String].read(json) { return .`list`(value) }
    if let value = try? Double.read(json) { return .`number`(value) }
    throw JSONConversionError("Unknown Value value")
  }
  var json: JSONValue {
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
    throw JSONConversionError("Expected decide aggregate packet")
  }
  public func chooseAnswer() throws -> [AtomicNullableString] {
    if case .`chooseAggregate`(let packet) = self { return packet.value }
    throw JSONConversionError("Expected choose aggregate packet")
  }
  public func tagAnswer() throws -> [AtomicArrayOfString] {
    if case .`tagAggregate`(let packet) = self { return packet.value }
    throw JSONConversionError("Expected tag aggregate packet")
  }
  public func scoreAnswer() throws -> [AtomicDouble] {
    if case .`scoreAggregate`(let packet) = self { return packet.value }
    throw JSONConversionError("Expected score aggregate packet")
  }
  public func filterAnswer() throws -> [AtomicBoolean] {
    if case .`filterAggregate`(let packet) = self { return packet.value }
    throw JSONConversionError("Expected filter aggregate packet")
  }
  public func rankAnswer() throws -> [AtomicNonZeroUsize] {
    if case .`rankAggregate`(let packet) = self { return packet.value }
    throw JSONConversionError("Expected rank aggregate packet")
  }
  public func findAnswer() throws -> Find {
    if case .`findAggregate`(let packet) = self { return packet.value }
    throw JSONConversionError("Expected find aggregate packet")
  }
  public func annotateAnswer() throws -> [Annotation] {
    if case .`annotateAggregate`(let packet) = self { return packet.value }
    throw JSONConversionError("Expected annotate aggregate packet")
  }
  public func recognizeAnswer() throws -> [Recognition] {
    if case .`recognizeAggregate`(let packet) = self { return packet.value }
    throw JSONConversionError("Expected recognize aggregate packet")
  }
  public func relateAnswer() throws -> Relation {
    if case .`relateAggregate`(let packet) = self { return packet.value }
    throw JSONConversionError("Expected relate aggregate packet")
  }
}
public struct AnswerCardinalityError: Error, Sendable {
  public let function: String
  public let count: Int
  public let call: Call
}
private func answerPath(_ json: JSONValue, _ path: [String]) throws -> JSONValue? {
  var value: JSONValue? = json
  for key in path {
    guard let object = value else { return nil }
    value = try jsonObject(object)[key]
  }
  return value
}
private func answerProjection(_ value: JSONValue, _ member: String) throws -> JSONValue {
  guard case .array(let rows) = value.kind else {
    throw JSONConversionError("Expected answer collection")
  }
  return .array(try rows.map { try jsonRequired(jsonObject($0), member) })
}
extension Call {
  private func answerJSON(_ function: String, _ judgment: String) throws -> JSONValue {
    guard terminal.kind == "terminal" else {
      throw AnswerCardinalityError(function: function, count: 0, call: self)
    }
    if case .null = terminal.failure {
      throw JSONConversionError("Terminal failure must be an object")
    }
    if case .value(let failure) = terminal.failure {
      throw SessionFailure(failure: failure, call: self)
    }
    let collection =
      function == "decide" || function == "choose" || function == "tag" || function == "score"
      || function == "filter" || function == "rank" || function == "annotate"
      || function == "recognize"
    var values: [JSONValue] = []
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
      var combined: [JSONValue] = []
      for value in values {
        guard case .array(let items) = value.kind else {
          throw JSONConversionError("Expected aggregate collection")
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
  public func filterAnswer() throws -> [JSONValue] {
    try [JSONValue].read(answerProjection(answerJSON("filter", ""), "input"))
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
    async throws -> [JSONValue]
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
