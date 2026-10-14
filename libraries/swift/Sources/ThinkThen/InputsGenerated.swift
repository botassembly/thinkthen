// Generated from the shared Rust graph. Do not edit.
import Foundation

public struct OrderedEntry<Value: Sendable>: Sendable {
  public let key: String
  public let value: Value
  public init(_ key: String, _ value: Value) {
    self.key = key
    self.value = value
  }
}
public struct OrderedMap<Value: Sendable>: Sendable {
  public let entries: [OrderedEntry<Value>]
  public init(_ entries: [OrderedEntry<Value>]) { self.entries = entries }
}
extension OrderedMap: ValueRepresentable where Value: ValueRepresentable {
  static func read(_ json: DataValue) throws -> Self {
    let members: [ValueMember]
    switch json.kind {
    case .orderedObject(let value): members = value
    case .object(let value): members = value.map { ValueMember($0.key, $0.value) }
    default: throw ValueConversionError("Expected JSON object")
    }
    return Self(try members.map { OrderedEntry($0.key, try Value.read($0.value)) })
  }
  var json: DataValue { .orderedObject(entries.map { ValueMember($0.key, $0.value.json) }) }
}
public struct AnnotateDefinition: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `batch`: Presence<DataValue>
  public let `profile`: Presence<AuthoredProfile>
  public let `questions`: OrderedMap<AnnotateQuestion>
  public let `threshold`: Presence<AuthoredThreshold>
  public let `version`: Int64
  public init(
    extensions: [String: DataValue] = [:], `batch`: Presence<DataValue> = .absent,
    `profile`: Presence<AuthoredProfile> = .absent, `questions`: OrderedMap<AnnotateQuestion>,
    `threshold`: Presence<AuthoredThreshold> = .absent, `version`: Int64 = 1
  ) {
    self.extensions = extensions
    self.`batch` = `batch`
    self.`profile` = `profile`
    self.`questions` = `questions`
    self.`threshold` = `threshold`
    self.`version` = `version`
  }
  static func read(_ json: DataValue) throws -> AnnotateDefinition {
    let object = try jsonObject(json)
    let known: Set<String> = ["batch", "profile", "questions", "threshold", "version"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      batch: try readPresence(object, "batch", DataValue.self),
      profile: try readPresence(object, "profile", AuthoredProfile.self),
      questions: try OrderedMap<AnnotateQuestion>.read(jsonRequired(object, "questions")),
      threshold: try readPresence(object, "threshold", AuthoredThreshold.self),
      version: try Int64.read(jsonRequired(object, "version")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["batch", "profile", "questions", "threshold", "version"]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`batch`, "batch", &object)
    writePresence(`profile`, "profile", &object)
    object["questions"] = `questions`.json
    writePresence(`threshold`, "threshold", &object)
    object["version"] = `version`.json
    return .object(object)
  }
}
public indirect enum AnnotateQuestion: ValueRepresentable {
  case `decide`(AnnotateQuestionDecide)
  case `choose`(AnnotateQuestionChoose)
  case `tag`(AnnotateQuestionTag)
  case `score`(AnnotateQuestionScore)
  static func read(_ json: DataValue) throws -> AnnotateQuestion {
    let object = try jsonObject(json)
    if object["decide"] != nil && object["choose"] == nil && object["score"] == nil
      && object["tag"] == nil
    {
      return .`decide`(try AnnotateQuestionDecide.read(json))
    }
    if object["choose"] != nil && object["decide"] == nil && object["score"] == nil
      && object["tag"] == nil
    {
      return .`choose`(try AnnotateQuestionChoose.read(json))
    }
    if object["tag"] != nil && object["choose"] == nil && object["decide"] == nil
      && object["score"] == nil
    {
      return .`tag`(try AnnotateQuestionTag.read(json))
    }
    if object["score"] != nil && object["choose"] == nil && object["decide"] == nil
      && object["tag"] == nil
    {
      return .`score`(try AnnotateQuestionScore.read(json))
    }
    throw ValueConversionError("Unknown AnnotateQuestion alternative")
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
public struct AnnotateQuestionChoose: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `choose`: AuthoredQuestionText
  public let `contextSchema`: Presence<AuthoredInputDeclaration>
  public let `itemSchema`: Presence<AuthoredInputDeclaration>
  public let `name`: Presence<String>
  public let `on`: Presence<AuthoredPointers>
  public let `options`: Presence<AuthoredOptions>
  public let `threshold`: Presence<AuthoredCut>
  public let `wordingVersion`: Presence<Int64>
  public init(
    extensions: [String: DataValue] = [:], `choose`: AuthoredQuestionText,
    `contextSchema`: Presence<AuthoredInputDeclaration> = .absent,
    `itemSchema`: Presence<AuthoredInputDeclaration> = .absent, `name`: Presence<String> = .absent,
    `on`: Presence<AuthoredPointers> = .absent, `options`: Presence<AuthoredOptions> = .absent,
    `threshold`: Presence<AuthoredCut> = .absent, `wordingVersion`: Presence<Int64> = .absent
  ) {
    self.extensions = extensions
    self.`choose` = `choose`
    self.`contextSchema` = `contextSchema`
    self.`itemSchema` = `itemSchema`
    self.`name` = `name`
    self.`on` = `on`
    self.`options` = `options`
    self.`threshold` = `threshold`
    self.`wordingVersion` = `wordingVersion`
  }
  static func read(_ json: DataValue) throws -> AnnotateQuestionChoose {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "choose", "context_schema", "item_schema", "name", "on", "options", "threshold",
      "wording_version",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      choose: try AuthoredQuestionText.read(jsonRequired(object, "choose")),
      contextSchema: try readPresence(object, "context_schema", AuthoredInputDeclaration.self),
      itemSchema: try readPresence(object, "item_schema", AuthoredInputDeclaration.self),
      name: try readPresence(object, "name", String.self),
      on: try readPresence(object, "on", AuthoredPointers.self),
      options: try readPresence(object, "options", AuthoredOptions.self),
      threshold: try readPresence(object, "threshold", AuthoredCut.self),
      wordingVersion: try readPresence(object, "wording_version", Int64.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "choose", "context_schema", "item_schema", "name", "on", "options", "threshold",
      "wording_version",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    object["choose"] = `choose`.json
    writePresence(`contextSchema`, "context_schema", &object)
    writePresence(`itemSchema`, "item_schema", &object)
    writePresence(`name`, "name", &object)
    writePresence(`on`, "on", &object)
    writePresence(`options`, "options", &object)
    writePresence(`threshold`, "threshold", &object)
    writePresence(`wordingVersion`, "wording_version", &object)
    return .object(object)
  }
}
public struct AnnotateQuestionDecide: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `contextSchema`: Presence<AuthoredInputDeclaration>
  public let `decide`: AuthoredQuestionText
  public let `false`: Presence<AuthoredCriterion>
  public let `itemSchema`: Presence<AuthoredInputDeclaration>
  public let `name`: Presence<String>
  public let `on`: Presence<AuthoredPointers>
  public let `threshold`: Presence<AuthoredThreshold>
  public let `true`: Presence<AuthoredCriterion>
  public let `wordingVersion`: Presence<Int64>
  public init(
    extensions: [String: DataValue] = [:],
    `contextSchema`: Presence<AuthoredInputDeclaration> = .absent, `decide`: AuthoredQuestionText,
    `false`: Presence<AuthoredCriterion> = .absent,
    `itemSchema`: Presence<AuthoredInputDeclaration> = .absent, `name`: Presence<String> = .absent,
    `on`: Presence<AuthoredPointers> = .absent, `threshold`: Presence<AuthoredThreshold> = .absent,
    `true`: Presence<AuthoredCriterion> = .absent, `wordingVersion`: Presence<Int64> = .absent
  ) {
    self.extensions = extensions
    self.`contextSchema` = `contextSchema`
    self.`decide` = `decide`
    self.`false` = `false`
    self.`itemSchema` = `itemSchema`
    self.`name` = `name`
    self.`on` = `on`
    self.`threshold` = `threshold`
    self.`true` = `true`
    self.`wordingVersion` = `wordingVersion`
  }
  static func read(_ json: DataValue) throws -> AnnotateQuestionDecide {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "context_schema", "decide", "false", "item_schema", "name", "on", "threshold", "true",
      "wording_version",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      contextSchema: try readPresence(object, "context_schema", AuthoredInputDeclaration.self),
      decide: try AuthoredQuestionText.read(jsonRequired(object, "decide")),
      false: try readPresence(object, "false", AuthoredCriterion.self),
      itemSchema: try readPresence(object, "item_schema", AuthoredInputDeclaration.self),
      name: try readPresence(object, "name", String.self),
      on: try readPresence(object, "on", AuthoredPointers.self),
      threshold: try readPresence(object, "threshold", AuthoredThreshold.self),
      true: try readPresence(object, "true", AuthoredCriterion.self),
      wordingVersion: try readPresence(object, "wording_version", Int64.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "context_schema", "decide", "false", "item_schema", "name", "on", "threshold", "true",
      "wording_version",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`contextSchema`, "context_schema", &object)
    object["decide"] = `decide`.json
    writePresence(`false`, "false", &object)
    writePresence(`itemSchema`, "item_schema", &object)
    writePresence(`name`, "name", &object)
    writePresence(`on`, "on", &object)
    writePresence(`threshold`, "threshold", &object)
    writePresence(`true`, "true", &object)
    writePresence(`wordingVersion`, "wording_version", &object)
    return .object(object)
  }
}
public struct AnnotateQuestionScore: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `contextSchema`: Presence<AuthoredInputDeclaration>
  public let `itemSchema`: Presence<AuthoredInputDeclaration>
  public let `levels`: Presence<AuthoredLevels>
  public let `name`: Presence<String>
  public let `on`: Presence<AuthoredPointers>
  public let `score`: AuthoredQuestionText
  public let `wordingVersion`: Presence<Int64>
  public init(
    extensions: [String: DataValue] = [:],
    `contextSchema`: Presence<AuthoredInputDeclaration> = .absent,
    `itemSchema`: Presence<AuthoredInputDeclaration> = .absent,
    `levels`: Presence<AuthoredLevels> = .absent, `name`: Presence<String> = .absent,
    `on`: Presence<AuthoredPointers> = .absent, `score`: AuthoredQuestionText,
    `wordingVersion`: Presence<Int64> = .absent
  ) {
    self.extensions = extensions
    self.`contextSchema` = `contextSchema`
    self.`itemSchema` = `itemSchema`
    self.`levels` = `levels`
    self.`name` = `name`
    self.`on` = `on`
    self.`score` = `score`
    self.`wordingVersion` = `wordingVersion`
  }
  static func read(_ json: DataValue) throws -> AnnotateQuestionScore {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "context_schema", "item_schema", "levels", "name", "on", "score", "wording_version",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      contextSchema: try readPresence(object, "context_schema", AuthoredInputDeclaration.self),
      itemSchema: try readPresence(object, "item_schema", AuthoredInputDeclaration.self),
      levels: try readPresence(object, "levels", AuthoredLevels.self),
      name: try readPresence(object, "name", String.self),
      on: try readPresence(object, "on", AuthoredPointers.self),
      score: try AuthoredQuestionText.read(jsonRequired(object, "score")),
      wordingVersion: try readPresence(object, "wording_version", Int64.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "context_schema", "item_schema", "levels", "name", "on", "score", "wording_version",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`contextSchema`, "context_schema", &object)
    writePresence(`itemSchema`, "item_schema", &object)
    writePresence(`levels`, "levels", &object)
    writePresence(`name`, "name", &object)
    writePresence(`on`, "on", &object)
    object["score"] = `score`.json
    writePresence(`wordingVersion`, "wording_version", &object)
    return .object(object)
  }
}
public struct AnnotateQuestionTag: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `contextSchema`: Presence<AuthoredInputDeclaration>
  public let `itemSchema`: Presence<AuthoredInputDeclaration>
  public let `labels`: Presence<AuthoredLabels>
  public let `name`: Presence<String>
  public let `on`: Presence<AuthoredPointers>
  public let `tag`: AuthoredQuestionText
  public let `threshold`: Presence<AuthoredCut>
  public let `wordingVersion`: Presence<Int64>
  public init(
    extensions: [String: DataValue] = [:],
    `contextSchema`: Presence<AuthoredInputDeclaration> = .absent,
    `itemSchema`: Presence<AuthoredInputDeclaration> = .absent,
    `labels`: Presence<AuthoredLabels> = .absent, `name`: Presence<String> = .absent,
    `on`: Presence<AuthoredPointers> = .absent, `tag`: AuthoredQuestionText,
    `threshold`: Presence<AuthoredCut> = .absent, `wordingVersion`: Presence<Int64> = .absent
  ) {
    self.extensions = extensions
    self.`contextSchema` = `contextSchema`
    self.`itemSchema` = `itemSchema`
    self.`labels` = `labels`
    self.`name` = `name`
    self.`on` = `on`
    self.`tag` = `tag`
    self.`threshold` = `threshold`
    self.`wordingVersion` = `wordingVersion`
  }
  static func read(_ json: DataValue) throws -> AnnotateQuestionTag {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "context_schema", "item_schema", "labels", "name", "on", "tag", "threshold",
      "wording_version",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      contextSchema: try readPresence(object, "context_schema", AuthoredInputDeclaration.self),
      itemSchema: try readPresence(object, "item_schema", AuthoredInputDeclaration.self),
      labels: try readPresence(object, "labels", AuthoredLabels.self),
      name: try readPresence(object, "name", String.self),
      on: try readPresence(object, "on", AuthoredPointers.self),
      tag: try AuthoredQuestionText.read(jsonRequired(object, "tag")),
      threshold: try readPresence(object, "threshold", AuthoredCut.self),
      wordingVersion: try readPresence(object, "wording_version", Int64.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "context_schema", "item_schema", "labels", "name", "on", "tag", "threshold",
      "wording_version",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`contextSchema`, "context_schema", &object)
    writePresence(`itemSchema`, "item_schema", &object)
    writePresence(`labels`, "labels", &object)
    writePresence(`name`, "name", &object)
    writePresence(`on`, "on", &object)
    object["tag"] = `tag`.json
    writePresence(`threshold`, "threshold", &object)
    writePresence(`wordingVersion`, "wording_version", &object)
    return .object(object)
  }
}
public struct AuthoredChoose: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `batch`: Presence<AuthoredChooseBatch>
  public let `choose`: AuthoredQuestionText
  public let `contextSchema`: Presence<AuthoredInputDeclaration>
  public let `itemSchema`: Presence<AuthoredInputDeclaration>
  public let `model`: Presence<AuthoredName>
  public let `name`: Presence<String>
  public let `on`: Presence<AuthoredPointers>
  public let `options`: Presence<AuthoredOptions>
  public let `profile`: Presence<AuthoredProfile>
  public let `threshold`: Presence<AuthoredCut>
  public let `wordingVersion`: Presence<Int64>
  public init(
    extensions: [String: DataValue] = [:], `batch`: Presence<AuthoredChooseBatch> = .absent,
    `choose`: AuthoredQuestionText, `contextSchema`: Presence<AuthoredInputDeclaration> = .absent,
    `itemSchema`: Presence<AuthoredInputDeclaration> = .absent,
    `model`: Presence<AuthoredName> = .absent, `name`: Presence<String> = .absent,
    `on`: Presence<AuthoredPointers> = .absent, `options`: Presence<AuthoredOptions> = .absent,
    `profile`: Presence<AuthoredProfile> = .absent, `threshold`: Presence<AuthoredCut> = .absent,
    `wordingVersion`: Presence<Int64> = .absent
  ) {
    self.extensions = extensions
    self.`batch` = `batch`
    self.`choose` = `choose`
    self.`contextSchema` = `contextSchema`
    self.`itemSchema` = `itemSchema`
    self.`model` = `model`
    self.`name` = `name`
    self.`on` = `on`
    self.`options` = `options`
    self.`profile` = `profile`
    self.`threshold` = `threshold`
    self.`wordingVersion` = `wordingVersion`
  }
  static func read(_ json: DataValue) throws -> AuthoredChoose {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "batch", "choose", "context_schema", "item_schema", "model", "name", "on", "options",
      "profile", "threshold", "wording_version",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      batch: try readPresence(object, "batch", AuthoredChooseBatch.self),
      choose: try AuthoredQuestionText.read(jsonRequired(object, "choose")),
      contextSchema: try readPresence(object, "context_schema", AuthoredInputDeclaration.self),
      itemSchema: try readPresence(object, "item_schema", AuthoredInputDeclaration.self),
      model: try readPresence(object, "model", AuthoredName.self),
      name: try readPresence(object, "name", String.self),
      on: try readPresence(object, "on", AuthoredPointers.self),
      options: try readPresence(object, "options", AuthoredOptions.self),
      profile: try readPresence(object, "profile", AuthoredProfile.self),
      threshold: try readPresence(object, "threshold", AuthoredCut.self),
      wordingVersion: try readPresence(object, "wording_version", Int64.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "batch", "choose", "context_schema", "item_schema", "model", "name", "on", "options",
      "profile", "threshold", "wording_version",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`batch`, "batch", &object)
    object["choose"] = `choose`.json
    writePresence(`contextSchema`, "context_schema", &object)
    writePresence(`itemSchema`, "item_schema", &object)
    writePresence(`model`, "model", &object)
    writePresence(`name`, "name", &object)
    writePresence(`on`, "on", &object)
    writePresence(`options`, "options", &object)
    writePresence(`profile`, "profile", &object)
    writePresence(`threshold`, "threshold", &object)
    writePresence(`wordingVersion`, "wording_version", &object)
    return .object(object)
  }
}
public indirect enum AuthoredChooseBatch: ValueRepresentable {
  case `max`(String)
  case `integer`(Int64)
  static func read(_ json: DataValue) throws -> AuthoredChooseBatch {
    if let value = try? String.read(json) { return .`max`(value) }
    if let value = try? Int64.read(json) { return .`integer`(value) }
    throw ValueConversionError("Unknown AuthoredChooseBatch value")
  }
  var json: DataValue {
    switch self {
    case .`max`(let value): return value.json
    case .`integer`(let value): return value.json
    }
  }
}
public indirect enum AuthoredCriterion: ValueRepresentable {
  case `string`(String)
  case `object`([String: DataValue])
  case `list`([DataValue])
  case `null`
  static func read(_ json: DataValue) throws -> AuthoredCriterion {
    if let value = try? String.read(json) { return .`string`(value) }
    if let value = try? [String: DataValue].read(json) { return .`object`(value) }
    if let value = try? [DataValue].read(json) { return .`list`(value) }
    if json == .null { return .`null` }
    throw ValueConversionError("Unknown AuthoredCriterion value")
  }
  var json: DataValue {
    switch self {
    case .`string`(let value): return value.json
    case .`object`(let value): return value.json
    case .`list`(let value): return value.json
    case .`null`: return .null
    }
  }
}
public indirect enum AuthoredCut: ValueRepresentable {
  case `number`(Double)
  case `string`(String)
  static func read(_ json: DataValue) throws -> AuthoredCut {
    if let value = try? Double.read(json) { return .`number`(value) }
    if let value = try? String.read(json) { return .`string`(value) }
    throw ValueConversionError("Unknown AuthoredCut value")
  }
  var json: DataValue {
    switch self {
    case .`number`(let value): return value.json
    case .`string`(let value): return value.json
    }
  }
}
public struct AuthoredDecide: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `batch`: Presence<AuthoredDecideBatch>
  public let `contextSchema`: Presence<AuthoredInputDeclaration>
  public let `decide`: AuthoredQuestionText
  public let `false`: Presence<AuthoredCriterion>
  public let `itemSchema`: Presence<AuthoredInputDeclaration>
  public let `model`: Presence<AuthoredName>
  public let `name`: Presence<String>
  public let `on`: Presence<AuthoredPointers>
  public let `profile`: Presence<AuthoredProfile>
  public let `threshold`: Presence<AuthoredThreshold>
  public let `true`: Presence<AuthoredCriterion>
  public let `wordingVersion`: Presence<Int64>
  public init(
    extensions: [String: DataValue] = [:], `batch`: Presence<AuthoredDecideBatch> = .absent,
    `contextSchema`: Presence<AuthoredInputDeclaration> = .absent, `decide`: AuthoredQuestionText,
    `false`: Presence<AuthoredCriterion> = .absent,
    `itemSchema`: Presence<AuthoredInputDeclaration> = .absent,
    `model`: Presence<AuthoredName> = .absent, `name`: Presence<String> = .absent,
    `on`: Presence<AuthoredPointers> = .absent, `profile`: Presence<AuthoredProfile> = .absent,
    `threshold`: Presence<AuthoredThreshold> = .absent,
    `true`: Presence<AuthoredCriterion> = .absent, `wordingVersion`: Presence<Int64> = .absent
  ) {
    self.extensions = extensions
    self.`batch` = `batch`
    self.`contextSchema` = `contextSchema`
    self.`decide` = `decide`
    self.`false` = `false`
    self.`itemSchema` = `itemSchema`
    self.`model` = `model`
    self.`name` = `name`
    self.`on` = `on`
    self.`profile` = `profile`
    self.`threshold` = `threshold`
    self.`true` = `true`
    self.`wordingVersion` = `wordingVersion`
  }
  static func read(_ json: DataValue) throws -> AuthoredDecide {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "batch", "context_schema", "decide", "false", "item_schema", "model", "name", "on", "profile",
      "threshold", "true", "wording_version",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      batch: try readPresence(object, "batch", AuthoredDecideBatch.self),
      contextSchema: try readPresence(object, "context_schema", AuthoredInputDeclaration.self),
      decide: try AuthoredQuestionText.read(jsonRequired(object, "decide")),
      false: try readPresence(object, "false", AuthoredCriterion.self),
      itemSchema: try readPresence(object, "item_schema", AuthoredInputDeclaration.self),
      model: try readPresence(object, "model", AuthoredName.self),
      name: try readPresence(object, "name", String.self),
      on: try readPresence(object, "on", AuthoredPointers.self),
      profile: try readPresence(object, "profile", AuthoredProfile.self),
      threshold: try readPresence(object, "threshold", AuthoredThreshold.self),
      true: try readPresence(object, "true", AuthoredCriterion.self),
      wordingVersion: try readPresence(object, "wording_version", Int64.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "batch", "context_schema", "decide", "false", "item_schema", "model", "name", "on", "profile",
      "threshold", "true", "wording_version",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`batch`, "batch", &object)
    writePresence(`contextSchema`, "context_schema", &object)
    object["decide"] = `decide`.json
    writePresence(`false`, "false", &object)
    writePresence(`itemSchema`, "item_schema", &object)
    writePresence(`model`, "model", &object)
    writePresence(`name`, "name", &object)
    writePresence(`on`, "on", &object)
    writePresence(`profile`, "profile", &object)
    writePresence(`threshold`, "threshold", &object)
    writePresence(`true`, "true", &object)
    writePresence(`wordingVersion`, "wording_version", &object)
    return .object(object)
  }
}
public indirect enum AuthoredDecideBatch: ValueRepresentable {
  case `max`(String)
  case `integer`(Int64)
  static func read(_ json: DataValue) throws -> AuthoredDecideBatch {
    if let value = try? String.read(json) { return .`max`(value) }
    if let value = try? Int64.read(json) { return .`integer`(value) }
    throw ValueConversionError("Unknown AuthoredDecideBatch value")
  }
  var json: DataValue {
    switch self {
    case .`max`(let value): return value.json
    case .`integer`(let value): return value.json
    }
  }
}
public indirect enum AuthoredDescription: ValueRepresentable {
  case `string`(String)
  case `object`([String: DataValue])
  case `list`([DataValue])
  case `null`
  static func read(_ json: DataValue) throws -> AuthoredDescription {
    if let value = try? String.read(json) { return .`string`(value) }
    if let value = try? [String: DataValue].read(json) { return .`object`(value) }
    if let value = try? [DataValue].read(json) { return .`list`(value) }
    if json == .null { return .`null` }
    throw ValueConversionError("Unknown AuthoredDescription value")
  }
  var json: DataValue {
    switch self {
    case .`string`(let value): return value.json
    case .`object`(let value): return value.json
    case .`list`(let value): return value.json
    case .`null`: return .null
    }
  }
}
public struct AuthoredFind: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `contextSchema`: Presence<AuthoredInputDeclaration>
  public let `find`: AuthoredQuestionText
  public let `itemSchema`: Presence<AuthoredInputDeclaration>
  public let `model`: Presence<AuthoredName>
  public let `name`: Presence<String>
  public let `on`: Presence<AuthoredPointers>
  public let `profile`: Presence<AuthoredProfile>
  public let `wordingVersion`: Presence<Int64>
  public init(
    extensions: [String: DataValue] = [:],
    `contextSchema`: Presence<AuthoredInputDeclaration> = .absent, `find`: AuthoredQuestionText,
    `itemSchema`: Presence<AuthoredInputDeclaration> = .absent,
    `model`: Presence<AuthoredName> = .absent, `name`: Presence<String> = .absent,
    `on`: Presence<AuthoredPointers> = .absent, `profile`: Presence<AuthoredProfile> = .absent,
    `wordingVersion`: Presence<Int64> = .absent
  ) {
    self.extensions = extensions
    self.`contextSchema` = `contextSchema`
    self.`find` = `find`
    self.`itemSchema` = `itemSchema`
    self.`model` = `model`
    self.`name` = `name`
    self.`on` = `on`
    self.`profile` = `profile`
    self.`wordingVersion` = `wordingVersion`
  }
  static func read(_ json: DataValue) throws -> AuthoredFind {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "context_schema", "find", "item_schema", "model", "name", "on", "profile", "wording_version",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      contextSchema: try readPresence(object, "context_schema", AuthoredInputDeclaration.self),
      find: try AuthoredQuestionText.read(jsonRequired(object, "find")),
      itemSchema: try readPresence(object, "item_schema", AuthoredInputDeclaration.self),
      model: try readPresence(object, "model", AuthoredName.self),
      name: try readPresence(object, "name", String.self),
      on: try readPresence(object, "on", AuthoredPointers.self),
      profile: try readPresence(object, "profile", AuthoredProfile.self),
      wordingVersion: try readPresence(object, "wording_version", Int64.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "context_schema", "find", "item_schema", "model", "name", "on", "profile", "wording_version",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`contextSchema`, "context_schema", &object)
    object["find"] = `find`.json
    writePresence(`itemSchema`, "item_schema", &object)
    writePresence(`model`, "model", &object)
    writePresence(`name`, "name", &object)
    writePresence(`on`, "on", &object)
    writePresence(`profile`, "profile", &object)
    writePresence(`wordingVersion`, "wording_version", &object)
    return .object(object)
  }
}
public indirect enum AuthoredInputDeclaration: ValueRepresentable {
  case `string`(AuthoredInputDeclarationString)
  case `object`(AuthoredInputDeclarationObject)
  static func read(_ json: DataValue) throws -> AuthoredInputDeclaration {
    let object = try jsonObject(json)
    if object["type"] == .string("string") {
      return .`string`(try AuthoredInputDeclarationString.read(json))
    }
    if object["type"] == .string("object") {
      return .`object`(try AuthoredInputDeclarationObject.read(json))
    }
    throw ValueConversionError("Unknown AuthoredInputDeclaration alternative")
  }
  var json: DataValue {
    switch self {
    case .`string`(let value): return value.json
    case .`object`(let value): return value.json
    }
  }
}
public struct AuthoredInputDeclarationObject: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `properties`: [String: AuthoredInputProperty]
  public let `required`: Presence<[String]>
  public let `type`: String
  public init(
    extensions: [String: DataValue] = [:], `properties`: [String: AuthoredInputProperty],
    `required`: Presence<[String]> = .absent, `type`: String = "object"
  ) {
    self.extensions = extensions
    self.`properties` = `properties`
    self.`required` = `required`
    self.`type` = `type`
  }
  static func read(_ json: DataValue) throws -> AuthoredInputDeclarationObject {
    let object = try jsonObject(json)
    let known: Set<String> = ["properties", "required", "type"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      properties: try [String: AuthoredInputProperty].read(jsonRequired(object, "properties")),
      required: try readPresence(object, "required", [String].self),
      type: try String.read(jsonRequired(object, "type")))
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
public struct AuthoredInputDeclarationString: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `type`: String
  public init(extensions: [String: DataValue] = [:], `type`: String = "string") {
    self.extensions = extensions
    self.`type` = `type`
  }
  static func read(_ json: DataValue) throws -> AuthoredInputDeclarationString {
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
public indirect enum AuthoredInputProperty: ValueRepresentable {
  case `string`(AuthoredInputPropertyString)
  case `number`(AuthoredInputPropertyNumber)
  case `boolean`(AuthoredInputPropertyBoolean)
  case `array`(AuthoredInputPropertyArray)
  static func read(_ json: DataValue) throws -> AuthoredInputProperty {
    let object = try jsonObject(json)
    if object["type"] == .string("string") {
      return .`string`(try AuthoredInputPropertyString.read(json))
    }
    if object["type"] == .string("number") {
      return .`number`(try AuthoredInputPropertyNumber.read(json))
    }
    if object["type"] == .string("boolean") {
      return .`boolean`(try AuthoredInputPropertyBoolean.read(json))
    }
    if object["type"] == .string("array") {
      return .`array`(try AuthoredInputPropertyArray.read(json))
    }
    throw ValueConversionError("Unknown AuthoredInputProperty alternative")
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
public struct AuthoredInputPropertyArray: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `items`: AuthoredInputPropertyArrayItems
  public let `type`: String
  public init(
    extensions: [String: DataValue] = [:], `items`: AuthoredInputPropertyArrayItems,
    `type`: String = "array"
  ) {
    self.extensions = extensions
    self.`items` = `items`
    self.`type` = `type`
  }
  static func read(_ json: DataValue) throws -> AuthoredInputPropertyArray {
    let object = try jsonObject(json)
    let known: Set<String> = ["items", "type"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      items: try AuthoredInputPropertyArrayItems.read(jsonRequired(object, "items")),
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
public struct AuthoredInputPropertyArrayItems: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `type`: String
  public init(extensions: [String: DataValue] = [:], `type`: String = "string") {
    self.extensions = extensions
    self.`type` = `type`
  }
  static func read(_ json: DataValue) throws -> AuthoredInputPropertyArrayItems {
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
public struct AuthoredInputPropertyBoolean: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `type`: String
  public init(extensions: [String: DataValue] = [:], `type`: String = "boolean") {
    self.extensions = extensions
    self.`type` = `type`
  }
  static func read(_ json: DataValue) throws -> AuthoredInputPropertyBoolean {
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
public struct AuthoredInputPropertyNumber: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `type`: String
  public init(extensions: [String: DataValue] = [:], `type`: String = "number") {
    self.extensions = extensions
    self.`type` = `type`
  }
  static func read(_ json: DataValue) throws -> AuthoredInputPropertyNumber {
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
public struct AuthoredInputPropertyString: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `type`: String
  public init(extensions: [String: DataValue] = [:], `type`: String = "string") {
    self.extensions = extensions
    self.`type` = `type`
  }
  static func read(_ json: DataValue) throws -> AuthoredInputPropertyString {
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
public indirect enum AuthoredLabels: ValueRepresentable {
  case `list`([AuthoredName])
  case `object`([String: AuthoredDescription])
  static func read(_ json: DataValue) throws -> AuthoredLabels {
    if let value = try? [AuthoredName].read(json) { return .`list`(value) }
    if let value = try? [String: AuthoredDescription].read(json) { return .`object`(value) }
    throw ValueConversionError("Unknown AuthoredLabels value")
  }
  var json: DataValue {
    switch self {
    case .`list`(let value): return value.json
    case .`object`(let value): return value.json
    }
  }
}
public indirect enum AuthoredLevels: ValueRepresentable {
  case `list`([AuthoredName])
  case `object`([String: AuthoredCriterion])
  static func read(_ json: DataValue) throws -> AuthoredLevels {
    if let value = try? [AuthoredName].read(json) { return .`list`(value) }
    if let value = try? [String: AuthoredCriterion].read(json) { return .`object`(value) }
    throw ValueConversionError("Unknown AuthoredLevels value")
  }
  var json: DataValue {
    switch self {
    case .`list`(let value): return value.json
    case .`object`(let value): return value.json
    }
  }
}
public typealias AuthoredName = String
public indirect enum AuthoredOptions: ValueRepresentable {
  case `list`([AuthoredName])
  case `object`(OrderedMap<AuthoredDescription>)
  static func read(_ json: DataValue) throws -> AuthoredOptions {
    if let value = try? [AuthoredName].read(json) { return .`list`(value) }
    if let value = try? OrderedMap<AuthoredDescription>.read(json) { return .`object`(value) }
    throw ValueConversionError("Unknown AuthoredOptions value")
  }
  var json: DataValue {
    switch self {
    case .`list`(let value): return value.json
    case .`object`(let value): return value.json
    }
  }
}
public indirect enum AuthoredPointers: ValueRepresentable {
  case `string`(String)
  case `list`([String])
  static func read(_ json: DataValue) throws -> AuthoredPointers {
    if let value = try? String.read(json) { return .`string`(value) }
    if let value = try? [String].read(json) { return .`list`(value) }
    throw ValueConversionError("Unknown AuthoredPointers value")
  }
  var json: DataValue {
    switch self {
    case .`string`(let value): return value.json
    case .`list`(let value): return value.json
    }
  }
}
public typealias AuthoredProfile = String
public indirect enum AuthoredQuestionText: ValueRepresentable {
  case `string`(String)
  case `object`([String: DataValue])
  case `list`([DataValue])
  static func read(_ json: DataValue) throws -> AuthoredQuestionText {
    if let value = try? String.read(json) { return .`string`(value) }
    if let value = try? [String: DataValue].read(json) { return .`object`(value) }
    if let value = try? [DataValue].read(json) { return .`list`(value) }
    throw ValueConversionError("Unknown AuthoredQuestionText value")
  }
  var json: DataValue {
    switch self {
    case .`string`(let value): return value.json
    case .`object`(let value): return value.json
    case .`list`(let value): return value.json
    }
  }
}
public struct AuthoredRelate: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `contextSchema`: Presence<AuthoredInputDeclaration>
  public let `itemSchema`: Presence<AuthoredInputDeclaration>
  public let `model`: Presence<AuthoredName>
  public let `name`: Presence<String>
  public let `profile`: Presence<AuthoredProfile>
  public let `relate`: RelateQuestion
  public let `threshold`: Presence<AuthoredCut>
  public let `version`: Int64
  public let `wordingVersion`: Presence<Int64>
  public init(
    extensions: [String: DataValue] = [:],
    `contextSchema`: Presence<AuthoredInputDeclaration> = .absent,
    `itemSchema`: Presence<AuthoredInputDeclaration> = .absent,
    `model`: Presence<AuthoredName> = .absent, `name`: Presence<String> = .absent,
    `profile`: Presence<AuthoredProfile> = .absent, `relate`: RelateQuestion,
    `threshold`: Presence<AuthoredCut> = .absent, `version`: Int64 = 1,
    `wordingVersion`: Presence<Int64> = .absent
  ) {
    self.extensions = extensions
    self.`contextSchema` = `contextSchema`
    self.`itemSchema` = `itemSchema`
    self.`model` = `model`
    self.`name` = `name`
    self.`profile` = `profile`
    self.`relate` = `relate`
    self.`threshold` = `threshold`
    self.`version` = `version`
    self.`wordingVersion` = `wordingVersion`
  }
  static func read(_ json: DataValue) throws -> AuthoredRelate {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "context_schema", "item_schema", "model", "name", "profile", "relate", "threshold", "version",
      "wording_version",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      contextSchema: try readPresence(object, "context_schema", AuthoredInputDeclaration.self),
      itemSchema: try readPresence(object, "item_schema", AuthoredInputDeclaration.self),
      model: try readPresence(object, "model", AuthoredName.self),
      name: try readPresence(object, "name", String.self),
      profile: try readPresence(object, "profile", AuthoredProfile.self),
      relate: try RelateQuestion.read(jsonRequired(object, "relate")),
      threshold: try readPresence(object, "threshold", AuthoredCut.self),
      version: try Int64.read(jsonRequired(object, "version")),
      wordingVersion: try readPresence(object, "wording_version", Int64.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "context_schema", "item_schema", "model", "name", "profile", "relate", "threshold", "version",
      "wording_version",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`contextSchema`, "context_schema", &object)
    writePresence(`itemSchema`, "item_schema", &object)
    writePresence(`model`, "model", &object)
    writePresence(`name`, "name", &object)
    writePresence(`profile`, "profile", &object)
    object["relate"] = `relate`.json
    writePresence(`threshold`, "threshold", &object)
    object["version"] = `version`.json
    writePresence(`wordingVersion`, "wording_version", &object)
    return .object(object)
  }
}
public struct AuthoredRelation: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `either`: Presence<Bool>
  public let `name`: AuthoredName
  public let `reads`: Presence<AuthoredName>
  public let `single`: Presence<Bool>
  public let `source`: Presence<AuthoredName>
  public let `target`: Presence<AuthoredName>
  public init(
    extensions: [String: DataValue] = [:], `either`: Presence<Bool> = .absent, `name`: AuthoredName,
    `reads`: Presence<AuthoredName> = .absent, `single`: Presence<Bool> = .absent,
    `source`: Presence<AuthoredName> = .absent, `target`: Presence<AuthoredName> = .absent
  ) {
    self.extensions = extensions
    self.`either` = `either`
    self.`name` = `name`
    self.`reads` = `reads`
    self.`single` = `single`
    self.`source` = `source`
    self.`target` = `target`
  }
  static func read(_ json: DataValue) throws -> AuthoredRelation {
    let object = try jsonObject(json)
    let known: Set<String> = ["either", "name", "reads", "single", "source", "target"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      either: try readPresence(object, "either", Bool.self),
      name: try AuthoredName.read(jsonRequired(object, "name")),
      reads: try readPresence(object, "reads", AuthoredName.self),
      single: try readPresence(object, "single", Bool.self),
      source: try readPresence(object, "source", AuthoredName.self),
      target: try readPresence(object, "target", AuthoredName.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["either", "name", "reads", "single", "source", "target"]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`either`, "either", &object)
    object["name"] = `name`.json
    writePresence(`reads`, "reads", &object)
    writePresence(`single`, "single", &object)
    writePresence(`source`, "source", &object)
    writePresence(`target`, "target", &object)
    return .object(object)
  }
}
public struct AuthoredScore: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `batch`: Presence<AuthoredScoreBatch>
  public let `contextSchema`: Presence<AuthoredInputDeclaration>
  public let `itemSchema`: Presence<AuthoredInputDeclaration>
  public let `levels`: Presence<AuthoredLevels>
  public let `model`: Presence<AuthoredName>
  public let `name`: Presence<String>
  public let `on`: Presence<AuthoredPointers>
  public let `profile`: Presence<AuthoredProfile>
  public let `score`: AuthoredQuestionText
  public let `wordingVersion`: Presence<Int64>
  public init(
    extensions: [String: DataValue] = [:], `batch`: Presence<AuthoredScoreBatch> = .absent,
    `contextSchema`: Presence<AuthoredInputDeclaration> = .absent,
    `itemSchema`: Presence<AuthoredInputDeclaration> = .absent,
    `levels`: Presence<AuthoredLevels> = .absent, `model`: Presence<AuthoredName> = .absent,
    `name`: Presence<String> = .absent, `on`: Presence<AuthoredPointers> = .absent,
    `profile`: Presence<AuthoredProfile> = .absent, `score`: AuthoredQuestionText,
    `wordingVersion`: Presence<Int64> = .absent
  ) {
    self.extensions = extensions
    self.`batch` = `batch`
    self.`contextSchema` = `contextSchema`
    self.`itemSchema` = `itemSchema`
    self.`levels` = `levels`
    self.`model` = `model`
    self.`name` = `name`
    self.`on` = `on`
    self.`profile` = `profile`
    self.`score` = `score`
    self.`wordingVersion` = `wordingVersion`
  }
  static func read(_ json: DataValue) throws -> AuthoredScore {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "batch", "context_schema", "item_schema", "levels", "model", "name", "on", "profile", "score",
      "wording_version",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      batch: try readPresence(object, "batch", AuthoredScoreBatch.self),
      contextSchema: try readPresence(object, "context_schema", AuthoredInputDeclaration.self),
      itemSchema: try readPresence(object, "item_schema", AuthoredInputDeclaration.self),
      levels: try readPresence(object, "levels", AuthoredLevels.self),
      model: try readPresence(object, "model", AuthoredName.self),
      name: try readPresence(object, "name", String.self),
      on: try readPresence(object, "on", AuthoredPointers.self),
      profile: try readPresence(object, "profile", AuthoredProfile.self),
      score: try AuthoredQuestionText.read(jsonRequired(object, "score")),
      wordingVersion: try readPresence(object, "wording_version", Int64.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "batch", "context_schema", "item_schema", "levels", "model", "name", "on", "profile", "score",
      "wording_version",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`batch`, "batch", &object)
    writePresence(`contextSchema`, "context_schema", &object)
    writePresence(`itemSchema`, "item_schema", &object)
    writePresence(`levels`, "levels", &object)
    writePresence(`model`, "model", &object)
    writePresence(`name`, "name", &object)
    writePresence(`on`, "on", &object)
    writePresence(`profile`, "profile", &object)
    object["score"] = `score`.json
    writePresence(`wordingVersion`, "wording_version", &object)
    return .object(object)
  }
}
public indirect enum AuthoredScoreBatch: ValueRepresentable {
  case `max`(String)
  case `integer`(Int64)
  static func read(_ json: DataValue) throws -> AuthoredScoreBatch {
    if let value = try? String.read(json) { return .`max`(value) }
    if let value = try? Int64.read(json) { return .`integer`(value) }
    throw ValueConversionError("Unknown AuthoredScoreBatch value")
  }
  var json: DataValue {
    switch self {
    case .`max`(let value): return value.json
    case .`integer`(let value): return value.json
    }
  }
}
public struct AuthoredTag: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `batch`: Presence<AuthoredTagBatch>
  public let `contextSchema`: Presence<AuthoredInputDeclaration>
  public let `itemSchema`: Presence<AuthoredInputDeclaration>
  public let `labels`: Presence<AuthoredLabels>
  public let `model`: Presence<AuthoredName>
  public let `name`: Presence<String>
  public let `on`: Presence<AuthoredPointers>
  public let `profile`: Presence<AuthoredProfile>
  public let `tag`: AuthoredQuestionText
  public let `threshold`: Presence<AuthoredCut>
  public let `wordingVersion`: Presence<Int64>
  public init(
    extensions: [String: DataValue] = [:], `batch`: Presence<AuthoredTagBatch> = .absent,
    `contextSchema`: Presence<AuthoredInputDeclaration> = .absent,
    `itemSchema`: Presence<AuthoredInputDeclaration> = .absent,
    `labels`: Presence<AuthoredLabels> = .absent, `model`: Presence<AuthoredName> = .absent,
    `name`: Presence<String> = .absent, `on`: Presence<AuthoredPointers> = .absent,
    `profile`: Presence<AuthoredProfile> = .absent, `tag`: AuthoredQuestionText,
    `threshold`: Presence<AuthoredCut> = .absent, `wordingVersion`: Presence<Int64> = .absent
  ) {
    self.extensions = extensions
    self.`batch` = `batch`
    self.`contextSchema` = `contextSchema`
    self.`itemSchema` = `itemSchema`
    self.`labels` = `labels`
    self.`model` = `model`
    self.`name` = `name`
    self.`on` = `on`
    self.`profile` = `profile`
    self.`tag` = `tag`
    self.`threshold` = `threshold`
    self.`wordingVersion` = `wordingVersion`
  }
  static func read(_ json: DataValue) throws -> AuthoredTag {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "batch", "context_schema", "item_schema", "labels", "model", "name", "on", "profile", "tag",
      "threshold", "wording_version",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      batch: try readPresence(object, "batch", AuthoredTagBatch.self),
      contextSchema: try readPresence(object, "context_schema", AuthoredInputDeclaration.self),
      itemSchema: try readPresence(object, "item_schema", AuthoredInputDeclaration.self),
      labels: try readPresence(object, "labels", AuthoredLabels.self),
      model: try readPresence(object, "model", AuthoredName.self),
      name: try readPresence(object, "name", String.self),
      on: try readPresence(object, "on", AuthoredPointers.self),
      profile: try readPresence(object, "profile", AuthoredProfile.self),
      tag: try AuthoredQuestionText.read(jsonRequired(object, "tag")),
      threshold: try readPresence(object, "threshold", AuthoredCut.self),
      wordingVersion: try readPresence(object, "wording_version", Int64.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "batch", "context_schema", "item_schema", "labels", "model", "name", "on", "profile", "tag",
      "threshold", "wording_version",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`batch`, "batch", &object)
    writePresence(`contextSchema`, "context_schema", &object)
    writePresence(`itemSchema`, "item_schema", &object)
    writePresence(`labels`, "labels", &object)
    writePresence(`model`, "model", &object)
    writePresence(`name`, "name", &object)
    writePresence(`on`, "on", &object)
    writePresence(`profile`, "profile", &object)
    object["tag"] = `tag`.json
    writePresence(`threshold`, "threshold", &object)
    writePresence(`wordingVersion`, "wording_version", &object)
    return .object(object)
  }
}
public indirect enum AuthoredTagBatch: ValueRepresentable {
  case `max`(String)
  case `integer`(Int64)
  static func read(_ json: DataValue) throws -> AuthoredTagBatch {
    if let value = try? String.read(json) { return .`max`(value) }
    if let value = try? Int64.read(json) { return .`integer`(value) }
    throw ValueConversionError("Unknown AuthoredTagBatch value")
  }
  var json: DataValue {
    switch self {
    case .`max`(let value): return value.json
    case .`integer`(let value): return value.json
    }
  }
}
public indirect enum AuthoredThreshold: ValueRepresentable {
  case `number`(Double)
  case `string`(String)
  static func read(_ json: DataValue) throws -> AuthoredThreshold {
    if let value = try? Double.read(json) { return .`number`(value) }
    if let value = try? String.read(json) { return .`string`(value) }
    throw ValueConversionError("Unknown AuthoredThreshold value")
  }
  var json: DataValue {
    switch self {
    case .`number`(let value): return value.json
    case .`string`(let value): return value.json
    }
  }
}
public indirect enum CacheDocument: ValueRepresentable {
  case `string`(String)
  case `disabledCache`(DisabledCache)
  static func read(_ json: DataValue) throws -> CacheDocument {
    if let value = try? String.read(json) { return .`string`(value) }
    if let value = try? DisabledCache.read(json) { return .`disabledCache`(value) }
    throw ValueConversionError("Unknown CacheDocument value")
  }
  var json: DataValue {
    switch self {
    case .`string`(let value): return value.json
    case .`disabledCache`(let value): return value.json
    }
  }
}
public indirect enum ContextSchema: ValueRepresentable {
  case `string`(String)
  case `object`([String: DataValue])
  static func read(_ json: DataValue) throws -> ContextSchema {
    if let value = try? String.read(json) { return .`string`(value) }
    if let value = try? [String: DataValue].read(json) { return .`object`(value) }
    throw ValueConversionError("Unknown ContextSchema value")
  }
  var json: DataValue {
    switch self {
    case .`string`(let value): return value.json
    case .`object`(let value): return value.json
    }
  }
}
public typealias DisabledCache = Bool
public struct EngineSettings: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `backend`: Presence<String>
  public let `baseUrl`: Presence<String>
  public let `batch`: Presence<Batch>
  public let `cache`: Presence<CacheDocument>
  public let `maxEstimatedInputTokensTotal`: Presence<UInt64?>
  public let `maxRequestBytes`: Presence<UInt64>
  public let `maxRequests`: Presence<UInt64?>
  public let `maxRequestsTotal`: Presence<UInt64?>
  public let `maxRetries`: Presence<UInt64>
  public let `model`: Presence<String>
  public let `profile`: Presence<String>
  public let `proxy`: Presence<DataValue>
  public let `record`: Presence<String>
  public let `refreshCache`: Presence<Bool>
  public let `replay`: Presence<String>
  public let `throttle`: Presence<UInt64>
  public let `timeout`: Presence<UInt64>
  public let `usdPerMillionInput`: Presence<String>
  public let `usdPerMillionOutput`: Presence<String>
  public init(
    extensions: [String: DataValue] = [:], `backend`: Presence<String> = .absent,
    `baseUrl`: Presence<String> = .absent, `batch`: Presence<Batch> = .absent,
    `cache`: Presence<CacheDocument> = .absent,
    `maxEstimatedInputTokensTotal`: Presence<UInt64?> = .absent,
    `maxRequestBytes`: Presence<UInt64> = .absent, `maxRequests`: Presence<UInt64?> = .absent,
    `maxRequestsTotal`: Presence<UInt64?> = .absent, `maxRetries`: Presence<UInt64> = .absent,
    `model`: Presence<String> = .absent, `profile`: Presence<String> = .absent,
    `proxy`: Presence<DataValue> = .absent, `record`: Presence<String> = .absent,
    `refreshCache`: Presence<Bool> = .absent, `replay`: Presence<String> = .absent,
    `throttle`: Presence<UInt64> = .absent, `timeout`: Presence<UInt64> = .absent,
    `usdPerMillionInput`: Presence<String> = .absent,
    `usdPerMillionOutput`: Presence<String> = .absent
  ) {
    self.extensions = extensions
    self.`backend` = `backend`
    self.`baseUrl` = `baseUrl`
    self.`batch` = `batch`
    self.`cache` = `cache`
    self.`maxEstimatedInputTokensTotal` = `maxEstimatedInputTokensTotal`
    self.`maxRequestBytes` = `maxRequestBytes`
    self.`maxRequests` = `maxRequests`
    self.`maxRequestsTotal` = `maxRequestsTotal`
    self.`maxRetries` = `maxRetries`
    self.`model` = `model`
    self.`profile` = `profile`
    self.`proxy` = `proxy`
    self.`record` = `record`
    self.`refreshCache` = `refreshCache`
    self.`replay` = `replay`
    self.`throttle` = `throttle`
    self.`timeout` = `timeout`
    self.`usdPerMillionInput` = `usdPerMillionInput`
    self.`usdPerMillionOutput` = `usdPerMillionOutput`
  }
  static func read(_ json: DataValue) throws -> EngineSettings {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "backend", "base_url", "batch", "cache", "max_estimated_input_tokens_total",
      "max_request_bytes", "max_requests", "max_requests_total", "max_retries", "model", "profile",
      "proxy", "record", "refresh_cache", "replay", "throttle", "timeout", "usd_per_million_input",
      "usd_per_million_output",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      backend: try readPresence(object, "backend", String.self),
      baseUrl: try readPresence(object, "base_url", String.self),
      batch: try readPresence(object, "batch", Batch.self),
      cache: try readPresence(object, "cache", CacheDocument.self),
      maxEstimatedInputTokensTotal: try readPresence(
        object, "max_estimated_input_tokens_total", UInt64?.self),
      maxRequestBytes: try readPresence(object, "max_request_bytes", UInt64.self),
      maxRequests: try readPresence(object, "max_requests", UInt64?.self),
      maxRequestsTotal: try readPresence(object, "max_requests_total", UInt64?.self),
      maxRetries: try readPresence(object, "max_retries", UInt64.self),
      model: try readPresence(object, "model", String.self),
      profile: try readPresence(object, "profile", String.self),
      proxy: try readPresence(object, "proxy", DataValue.self),
      record: try readPresence(object, "record", String.self),
      refreshCache: try readPresence(object, "refresh_cache", Bool.self),
      replay: try readPresence(object, "replay", String.self),
      throttle: try readPresence(object, "throttle", UInt64.self),
      timeout: try readPresence(object, "timeout", UInt64.self),
      usdPerMillionInput: try readPresence(object, "usd_per_million_input", String.self),
      usdPerMillionOutput: try readPresence(object, "usd_per_million_output", String.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "backend", "base_url", "batch", "cache", "max_estimated_input_tokens_total",
      "max_request_bytes", "max_requests", "max_requests_total", "max_retries", "model", "profile",
      "proxy", "record", "refresh_cache", "replay", "throttle", "timeout", "usd_per_million_input",
      "usd_per_million_output",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`backend`, "backend", &object)
    writePresence(`baseUrl`, "base_url", &object)
    writePresence(`batch`, "batch", &object)
    writePresence(`cache`, "cache", &object)
    writePresence(`maxEstimatedInputTokensTotal`, "max_estimated_input_tokens_total", &object)
    writePresence(`maxRequestBytes`, "max_request_bytes", &object)
    writePresence(`maxRequests`, "max_requests", &object)
    writePresence(`maxRequestsTotal`, "max_requests_total", &object)
    writePresence(`maxRetries`, "max_retries", &object)
    writePresence(`model`, "model", &object)
    writePresence(`profile`, "profile", &object)
    writePresence(`proxy`, "proxy", &object)
    writePresence(`record`, "record", &object)
    writePresence(`refreshCache`, "refresh_cache", &object)
    writePresence(`replay`, "replay", &object)
    writePresence(`throttle`, "throttle", &object)
    writePresence(`timeout`, "timeout", &object)
    writePresence(`usdPerMillionInput`, "usd_per_million_input", &object)
    writePresence(`usdPerMillionOutput`, "usd_per_million_output", &object)
    return .object(object)
  }
}
public enum ImageMedia: ValueRepresentable {
  case `imageJpeg`
  case `imagePng`
  static func read(_ json: DataValue) throws -> ImageMedia {
    let value = try String.read(json)
    switch value {
    case "image/jpeg": return .`imageJpeg`
    case "image/png": return .`imagePng`
    default: throw ValueConversionError("Unknown ImageMedia value")
    }
  }
  var json: DataValue {
    switch self {
    case .`imageJpeg`: return .string("image/jpeg")
    case .`imagePng`: return .string("image/png")
    }
  }
}
public struct OptionSchema: ValueRepresentable {
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
  static func read(_ json: DataValue) throws -> OptionSchema {
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
public enum ReaderMedia: ValueRepresentable {
  case `text`
  case `image`
  static func read(_ json: DataValue) throws -> ReaderMedia {
    let value = try String.read(json)
    switch value {
    case "text": return .`text`
    case "image": return .`image`
    default: throw ValueConversionError("Unknown ReaderMedia value")
    }
  }
  var json: DataValue {
    switch self {
    case .`text`: return .string("text")
    case .`image`: return .string("image")
    }
  }
}
public indirect enum RecognitionExample: ValueRepresentable {
  case `string`(String)
  case `recognitionExampleText`(RecognitionExampleText)
  static func read(_ json: DataValue) throws -> RecognitionExample {
    if let value = try? String.read(json) { return .`string`(value) }
    if let value = try? RecognitionExampleText.read(json) {
      return .`recognitionExampleText`(value)
    }
    throw ValueConversionError("Unknown RecognitionExample value")
  }
  var json: DataValue {
    switch self {
    case .`string`(let value): return value.json
    case .`recognitionExampleText`(let value): return value.json
    }
  }
}
public struct RecognitionExampleEntity: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `end`: UInt64
  public let `kind`: String
  public let `start`: UInt64
  public init(extensions: [String: DataValue] = [:], `end`: UInt64, `kind`: String, `start`: UInt64)
  {
    self.extensions = extensions
    self.`end` = `end`
    self.`kind` = `kind`
    self.`start` = `start`
  }
  static func read(_ json: DataValue) throws -> RecognitionExampleEntity {
    let object = try jsonObject(json)
    let known: Set<String> = ["end", "kind", "start"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      end: try UInt64.read(jsonRequired(object, "end")),
      kind: try String.read(jsonRequired(object, "kind")),
      start: try UInt64.read(jsonRequired(object, "start")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["end", "kind", "start"]
    var object = extensions.filter { !known.contains($0.key) }
    object["end"] = `end`.json
    object["kind"] = `kind`.json
    object["start"] = `start`.json
    return .object(object)
  }
}
public struct RecognitionExampleText: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `entities`: [RecognitionExampleEntity]
  public let `kinds`: Presence<[String]>
  public let `text`: String
  public init(
    extensions: [String: DataValue] = [:], `entities`: [RecognitionExampleEntity],
    `kinds`: Presence<[String]> = .absent, `text`: String
  ) {
    self.extensions = extensions
    self.`entities` = `entities`
    self.`kinds` = `kinds`
    self.`text` = `text`
  }
  static func read(_ json: DataValue) throws -> RecognitionExampleText {
    let object = try jsonObject(json)
    let known: Set<String> = ["entities", "kinds", "text"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      entities: try [RecognitionExampleEntity].read(jsonRequired(object, "entities")),
      kinds: try readPresence(object, "kinds", [String].self),
      text: try String.read(jsonRequired(object, "text")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["entities", "kinds", "text"]
    var object = extensions.filter { !known.contains($0.key) }
    object["entities"] = `entities`.json
    writePresence(`kinds`, "kinds", &object)
    object["text"] = `text`.json
    return .object(object)
  }
}
public enum RecognitionMode: ValueRepresentable {
  case `whole`
  case `boundaryOnly`
  static func read(_ json: DataValue) throws -> RecognitionMode {
    let value = try String.read(json)
    switch value {
    case "whole": return .`whole`
    case "boundary_only": return .`boundaryOnly`
    default: throw ValueConversionError("Unknown RecognitionMode value")
    }
  }
  var json: DataValue {
    switch self {
    case .`whole`: return .string("whole")
    case .`boundaryOnly`: return .string("boundary_only")
    }
  }
}
public struct RecognitionSeedSpan: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `end`: UInt64
  public let `kind`: Presence<String>
  public let `start`: UInt64
  public init(
    extensions: [String: DataValue] = [:], `end`: UInt64, `kind`: Presence<String> = .absent,
    `start`: UInt64
  ) {
    self.extensions = extensions
    self.`end` = `end`
    self.`kind` = `kind`
    self.`start` = `start`
  }
  static func read(_ json: DataValue) throws -> RecognitionSeedSpan {
    let object = try jsonObject(json)
    let known: Set<String> = ["end", "kind", "start"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      end: try UInt64.read(jsonRequired(object, "end")),
      kind: try readPresence(object, "kind", String.self),
      start: try UInt64.read(jsonRequired(object, "start")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["end", "kind", "start"]
    var object = extensions.filter { !known.contains($0.key) }
    object["end"] = `end`.json
    writePresence(`kind`, "kind", &object)
    object["start"] = `start`.json
    return .object(object)
  }
}
public struct RecognitionStageContext: ValueRepresentable {
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
  static func read(_ json: DataValue) throws -> RecognitionStageContext {
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
public struct RecognizeDefinition: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `contextSchema`: Presence<AuthoredInputDeclaration>
  public let `itemSchema`: Presence<AuthoredInputDeclaration>
  public let `model`: Presence<AuthoredName>
  public let `name`: Presence<String>
  public let `on`: Presence<AuthoredPointers>
  public let `profile`: Presence<AuthoredProfile>
  public let `recognize`: RecognizeQuestion
  public let `relationThreshold`: Presence<AuthoredCut>
  public let `threshold`: Presence<AuthoredCut>
  public let `version`: Int64
  public let `wordingVersion`: Presence<Int64>
  public init(
    extensions: [String: DataValue] = [:],
    `contextSchema`: Presence<AuthoredInputDeclaration> = .absent,
    `itemSchema`: Presence<AuthoredInputDeclaration> = .absent,
    `model`: Presence<AuthoredName> = .absent, `name`: Presence<String> = .absent,
    `on`: Presence<AuthoredPointers> = .absent, `profile`: Presence<AuthoredProfile> = .absent,
    `recognize`: RecognizeQuestion, `relationThreshold`: Presence<AuthoredCut> = .absent,
    `threshold`: Presence<AuthoredCut> = .absent, `version`: Int64 = 1,
    `wordingVersion`: Presence<Int64> = .absent
  ) {
    self.extensions = extensions
    self.`contextSchema` = `contextSchema`
    self.`itemSchema` = `itemSchema`
    self.`model` = `model`
    self.`name` = `name`
    self.`on` = `on`
    self.`profile` = `profile`
    self.`recognize` = `recognize`
    self.`relationThreshold` = `relationThreshold`
    self.`threshold` = `threshold`
    self.`version` = `version`
    self.`wordingVersion` = `wordingVersion`
  }
  static func read(_ json: DataValue) throws -> RecognizeDefinition {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "context_schema", "item_schema", "model", "name", "on", "profile", "recognize",
      "relation_threshold", "threshold", "version", "wording_version",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      contextSchema: try readPresence(object, "context_schema", AuthoredInputDeclaration.self),
      itemSchema: try readPresence(object, "item_schema", AuthoredInputDeclaration.self),
      model: try readPresence(object, "model", AuthoredName.self),
      name: try readPresence(object, "name", String.self),
      on: try readPresence(object, "on", AuthoredPointers.self),
      profile: try readPresence(object, "profile", AuthoredProfile.self),
      recognize: try RecognizeQuestion.read(jsonRequired(object, "recognize")),
      relationThreshold: try readPresence(object, "relation_threshold", AuthoredCut.self),
      threshold: try readPresence(object, "threshold", AuthoredCut.self),
      version: try Int64.read(jsonRequired(object, "version")),
      wordingVersion: try readPresence(object, "wording_version", Int64.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "context_schema", "item_schema", "model", "name", "on", "profile", "recognize",
      "relation_threshold", "threshold", "version", "wording_version",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`contextSchema`, "context_schema", &object)
    writePresence(`itemSchema`, "item_schema", &object)
    writePresence(`model`, "model", &object)
    writePresence(`name`, "name", &object)
    writePresence(`on`, "on", &object)
    writePresence(`profile`, "profile", &object)
    object["recognize"] = `recognize`.json
    writePresence(`relationThreshold`, "relation_threshold", &object)
    writePresence(`threshold`, "threshold", &object)
    object["version"] = `version`.json
    writePresence(`wordingVersion`, "wording_version", &object)
    return .object(object)
  }
}
public struct RecognizeQuestion: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `entityDefinition`: Presence<AuthoredQuestionText>
  public let `instructions`: Presence<AuthoredQuestionText>
  public let `kinds`: Presence<OrderedMap<AuthoredDescription>>
  public let `mode`: Presence<RecognitionMode>
  public let `relations`: Presence<[AuthoredRelation]>
  public let `snippetPieces`: Presence<UInt64>
  public let `stageContext`: Presence<RecognitionStageContext>
  public init(
    extensions: [String: DataValue] = [:],
    `entityDefinition`: Presence<AuthoredQuestionText> = .absent,
    `instructions`: Presence<AuthoredQuestionText> = .absent,
    `kinds`: Presence<OrderedMap<AuthoredDescription>> = .absent,
    `mode`: Presence<RecognitionMode> = .absent,
    `relations`: Presence<[AuthoredRelation]> = .absent,
    `snippetPieces`: Presence<UInt64> = .absent,
    `stageContext`: Presence<RecognitionStageContext> = .absent
  ) {
    self.extensions = extensions
    self.`entityDefinition` = `entityDefinition`
    self.`instructions` = `instructions`
    self.`kinds` = `kinds`
    self.`mode` = `mode`
    self.`relations` = `relations`
    self.`snippetPieces` = `snippetPieces`
    self.`stageContext` = `stageContext`
  }
  static func read(_ json: DataValue) throws -> RecognizeQuestion {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "entity_definition", "instructions", "kinds", "mode", "relations", "snippet_pieces",
      "stage_context",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      entityDefinition: try readPresence(object, "entity_definition", AuthoredQuestionText.self),
      instructions: try readPresence(object, "instructions", AuthoredQuestionText.self),
      kinds: try readPresence(object, "kinds", OrderedMap<AuthoredDescription>.self),
      mode: try readPresence(object, "mode", RecognitionMode.self),
      relations: try readPresence(object, "relations", [AuthoredRelation].self),
      snippetPieces: try readPresence(object, "snippet_pieces", UInt64.self),
      stageContext: try readPresence(object, "stage_context", RecognitionStageContext.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "entity_definition", "instructions", "kinds", "mode", "relations", "snippet_pieces",
      "stage_context",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`entityDefinition`, "entity_definition", &object)
    writePresence(`instructions`, "instructions", &object)
    writePresence(`kinds`, "kinds", &object)
    writePresence(`mode`, "mode", &object)
    writePresence(`relations`, "relations", &object)
    writePresence(`snippetPieces`, "snippet_pieces", &object)
    writePresence(`stageContext`, "stage_context", &object)
    return .object(object)
  }
}
public struct RelateFields: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `name`: String
  public init(extensions: [String: DataValue] = [:], `kind`: String, `name`: String) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`name` = `name`
  }
  static func read(_ json: DataValue) throws -> RelateFields {
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
public struct RelateQuestion: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `fields`: Presence<RelateFields>
  public let `relations`: [AuthoredRelation]
  public init(
    extensions: [String: DataValue] = [:], `fields`: Presence<RelateFields> = .absent,
    `relations`: [AuthoredRelation]
  ) {
    self.extensions = extensions
    self.`fields` = `fields`
    self.`relations` = `relations`
  }
  static func read(_ json: DataValue) throws -> RelateQuestion {
    let object = try jsonObject(json)
    let known: Set<String> = ["fields", "relations"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      fields: try readPresence(object, "fields", RelateFields.self),
      relations: try [AuthoredRelation].read(jsonRequired(object, "relations")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["fields", "relations"]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`fields`, "fields", &object)
    object["relations"] = `relations`.json
    return .object(object)
  }
}
public struct Request: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `call`: CallRequest
  public let `schema`: Version
  public init(extensions: [String: DataValue] = [:], `call`: CallRequest, `schema`: Version) {
    self.extensions = extensions
    self.`call` = `call`
    self.`schema` = `schema`
  }
  static func read(_ json: DataValue) throws -> Request {
    let object = try jsonObject(json)
    let known: Set<String> = ["call", "schema"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      call: try CallRequest.read(jsonRequired(object, "call")),
      schema: try Version.read(jsonRequired(object, "schema")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["call", "schema"]
    var object = extensions.filter { !known.contains($0.key) }
    object["call"] = `call`.json
    object["schema"] = `schema`.json
    return .object(object)
  }
}
public indirect enum Batch: ValueRepresentable {
  case `integer`(UInt64)
  case `string`(String)
  static func read(_ json: DataValue) throws -> Batch {
    if let value = try? UInt64.read(json) { return .`integer`(value) }
    if let value = try? String.read(json) { return .`string`(value) }
    throw ValueConversionError("Unknown Batch value")
  }
  var json: DataValue {
    switch self {
    case .`integer`(let value): return value.json
    case .`string`(let value): return value.json
    }
  }
}
public struct CallRequest: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `function`: Function
  public let `input`: Input
  public let `options`: Presence<Options>
  public let `question`: Question
  public init(
    extensions: [String: DataValue] = [:], `function`: Function, `input`: Input,
    `options`: Presence<Options> = .absent, `question`: Question
  ) {
    self.extensions = extensions
    self.`function` = `function`
    self.`input` = `input`
    self.`options` = `options`
    self.`question` = `question`
  }
  static func read(_ json: DataValue) throws -> CallRequest {
    let object = try jsonObject(json)
    let known: Set<String> = ["function", "input", "options", "question"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      function: try Function.read(jsonRequired(object, "function")),
      input: try Input.read(jsonRequired(object, "input")),
      options: try readPresence(object, "options", Options.self),
      question: try Question.read(jsonRequired(object, "question")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["function", "input", "options", "question"]
    var object = extensions.filter { !known.contains($0.key) }
    object["function"] = `function`.json
    object["input"] = `input`.json
    writePresence(`options`, "options", &object)
    object["question"] = `question`.json
    return .object(object)
  }
}
public indirect enum Definition: ValueRepresentable {
  case `decide`(AuthoredDecide)
  case `choose`(AuthoredChoose)
  case `tag`(AuthoredTag)
  case `score`(AuthoredScore)
  case `relate`(AuthoredRelate)
  case `find`(AuthoredFind)
  case `recognize`(RecognizeDefinition)
  case `questions`(AnnotateDefinition)
  static func read(_ json: DataValue) throws -> Definition {
    let object = try jsonObject(json)
    if object["decide"] != nil && object["choose"] == nil && object["find"] == nil
      && object["questions"] == nil && object["recognize"] == nil && object["relate"] == nil
      && object["score"] == nil && object["tag"] == nil && object["version"] == nil
    {
      return .`decide`(try AuthoredDecide.read(json))
    }
    if object["choose"] != nil && object["decide"] == nil && object["find"] == nil
      && object["questions"] == nil && object["recognize"] == nil && object["relate"] == nil
      && object["score"] == nil && object["tag"] == nil && object["version"] == nil
    {
      return .`choose`(try AuthoredChoose.read(json))
    }
    if object["tag"] != nil && object["choose"] == nil && object["decide"] == nil
      && object["find"] == nil && object["questions"] == nil && object["recognize"] == nil
      && object["relate"] == nil && object["score"] == nil && object["version"] == nil
    {
      return .`tag`(try AuthoredTag.read(json))
    }
    if object["score"] != nil && object["choose"] == nil && object["decide"] == nil
      && object["find"] == nil && object["questions"] == nil && object["recognize"] == nil
      && object["relate"] == nil && object["tag"] == nil && object["version"] == nil
    {
      return .`score`(try AuthoredScore.read(json))
    }
    if object["relate"] != nil && object["version"] != nil && object["choose"] == nil
      && object["decide"] == nil && object["find"] == nil && object["questions"] == nil
      && object["recognize"] == nil && object["score"] == nil && object["tag"] == nil
    {
      return .`relate`(try AuthoredRelate.read(json))
    }
    if object["find"] != nil && object["choose"] == nil && object["decide"] == nil
      && object["questions"] == nil && object["recognize"] == nil && object["relate"] == nil
      && object["score"] == nil && object["tag"] == nil && object["version"] == nil
    {
      return .`find`(try AuthoredFind.read(json))
    }
    if object["recognize"] != nil && object["version"] != nil && object["choose"] == nil
      && object["decide"] == nil && object["find"] == nil && object["questions"] == nil
      && object["relate"] == nil && object["score"] == nil && object["tag"] == nil
    {
      return .`recognize`(try RecognizeDefinition.read(json))
    }
    if object["questions"] != nil && object["version"] != nil && object["choose"] == nil
      && object["decide"] == nil && object["find"] == nil && object["recognize"] == nil
      && object["relate"] == nil && object["score"] == nil && object["tag"] == nil
    {
      return .`questions`(try AnnotateDefinition.read(json))
    }
    throw ValueConversionError("Unknown Definition alternative")
  }
  var json: DataValue {
    switch self {
    case .`decide`(let value): return value.json
    case .`choose`(let value): return value.json
    case .`tag`(let value): return value.json
    case .`score`(let value): return value.json
    case .`relate`(let value): return value.json
    case .`find`(let value): return value.json
    case .`recognize`(let value): return value.json
    case .`questions`(let value): return value.json
    }
  }
}
public enum Framing: ValueRepresentable {
  case `document`
  case `lines`
  case `jsonl`
  case `csv`
  case `tsv`
  static func read(_ json: DataValue) throws -> Framing {
    let value = try String.read(json)
    switch value {
    case "document": return .`document`
    case "lines": return .`lines`
    case "jsonl": return .`jsonl`
    case "csv": return .`csv`
    case "tsv": return .`tsv`
    default: throw ValueConversionError("Unknown Framing value")
    }
  }
  var json: DataValue {
    switch self {
    case .`document`: return .string("document")
    case .`lines`: return .string("lines")
    case .`jsonl`: return .string("jsonl")
    case .`csv`: return .string("csv")
    case .`tsv`: return .string("tsv")
    }
  }
}
public enum Function: ValueRepresentable {
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
  static func read(_ json: DataValue) throws -> Function {
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
    default: throw ValueConversionError("Unknown Function value")
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
    }
  }
}
public indirect enum Image: ValueRepresentable {
  case `file`(ImageFile)
  case `bytes`(ImageBytes)
  static func read(_ json: DataValue) throws -> Image {
    let object = try jsonObject(json)
    if object["kind"] == .string("file") { return .`file`(try ImageFile.read(json)) }
    if object["kind"] == .string("bytes") { return .`bytes`(try ImageBytes.read(json)) }
    throw ValueConversionError("Unknown Image alternative")
  }
  var json: DataValue {
    switch self {
    case .`file`(let value): return value.json
    case .`bytes`(let value): return value.json
    }
  }
}
public struct ImageBytes: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `bytes`: String
  public let `kind`: String
  public let `media`: ImageMedia
  public init(
    extensions: [String: DataValue] = [:], `bytes`: String, `kind`: String = "bytes",
    `media`: ImageMedia
  ) {
    self.extensions = extensions
    self.`bytes` = `bytes`
    self.`kind` = `kind`
    self.`media` = `media`
  }
  static func read(_ json: DataValue) throws -> ImageBytes {
    let object = try jsonObject(json)
    let known: Set<String> = ["bytes", "kind", "media"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      bytes: try String.read(jsonRequired(object, "bytes")),
      kind: try String.read(jsonRequired(object, "kind")),
      media: try ImageMedia.read(jsonRequired(object, "media")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["bytes", "kind", "media"]
    var object = extensions.filter { !known.contains($0.key) }
    object["bytes"] = `bytes`.json
    object["kind"] = `kind`.json
    object["media"] = `media`.json
    return .object(object)
  }
}
public struct ImageFile: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `media`: Presence<ImageMedia>
  public let `path`: String
  public init(
    extensions: [String: DataValue] = [:], `kind`: String = "file",
    `media`: Presence<ImageMedia> = .absent, `path`: String
  ) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`media` = `media`
    self.`path` = `path`
  }
  static func read(_ json: DataValue) throws -> ImageFile {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "media", "path"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      media: try readPresence(object, "media", ImageMedia.self),
      path: try String.read(jsonRequired(object, "path")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "media", "path"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    writePresence(`media`, "media", &object)
    object["path"] = `path`.json
    return .object(object)
  }
}
public indirect enum Input: ValueRepresentable {
  case `text`(InputText)
  case `json`(InputJson)
  case `records`(InputRecords)
  case `units`(InputUnits)
  case `entities`(InputEntities)
  case `source`(InputSource)
  case `feed`(InputFeed)
  static func read(_ json: DataValue) throws -> Input {
    let object = try jsonObject(json)
    if object["kind"] == .string("text") { return .`text`(try InputText.read(json)) }
    if object["kind"] == .string("json") { return .`json`(try InputJson.read(json)) }
    if object["kind"] == .string("records") { return .`records`(try InputRecords.read(json)) }
    if object["kind"] == .string("units") { return .`units`(try InputUnits.read(json)) }
    if object["kind"] == .string("entities") { return .`entities`(try InputEntities.read(json)) }
    if object["kind"] == .string("source") { return .`source`(try InputSource.read(json)) }
    if object["kind"] == .string("feed") { return .`feed`(try InputFeed.read(json)) }
    throw ValueConversionError("Unknown Input alternative")
  }
  var json: DataValue {
    switch self {
    case .`text`(let value): return value.json
    case .`json`(let value): return value.json
    case .`records`(let value): return value.json
    case .`units`(let value): return value.json
    case .`entities`(let value): return value.json
    case .`source`(let value): return value.json
    case .`feed`(let value): return value.json
    }
  }
}
public struct InputEntities: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `items`: [Item]
  public let `kind`: String
  public init(extensions: [String: DataValue] = [:], `items`: [Item], `kind`: String = "entities") {
    self.extensions = extensions
    self.`items` = `items`
    self.`kind` = `kind`
  }
  static func read(_ json: DataValue) throws -> InputEntities {
    let object = try jsonObject(json)
    let known: Set<String> = ["items", "kind"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      items: try [Item].read(jsonRequired(object, "items")),
      kind: try String.read(jsonRequired(object, "kind")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["items", "kind"]
    var object = extensions.filter { !known.contains($0.key) }
    object["items"] = `items`.json
    object["kind"] = `kind`.json
    return .object(object)
  }
}
public struct InputFeed: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `framing`: Presence<Framing>
  public let `images`: Presence<[Image]>
  public let `kind`: String
  public let `name`: String
  public let `reading`: Presence<Reader>
  public init(
    extensions: [String: DataValue] = [:], `framing`: Presence<Framing> = .absent,
    `images`: Presence<[Image]> = .absent, `kind`: String = "feed", `name`: String,
    `reading`: Presence<Reader> = .absent
  ) {
    self.extensions = extensions
    self.`framing` = `framing`
    self.`images` = `images`
    self.`kind` = `kind`
    self.`name` = `name`
    self.`reading` = `reading`
  }
  static func read(_ json: DataValue) throws -> InputFeed {
    let object = try jsonObject(json)
    let known: Set<String> = ["framing", "images", "kind", "name", "reading"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      framing: try readPresence(object, "framing", Framing.self),
      images: try readPresence(object, "images", [Image].self),
      kind: try String.read(jsonRequired(object, "kind")),
      name: try String.read(jsonRequired(object, "name")),
      reading: try readPresence(object, "reading", Reader.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["framing", "images", "kind", "name", "reading"]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`framing`, "framing", &object)
    writePresence(`images`, "images", &object)
    object["kind"] = `kind`.json
    object["name"] = `name`.json
    writePresence(`reading`, "reading", &object)
    return .object(object)
  }
}
public struct InputJson: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `images`: Presence<[Image]>
  public let `kind`: String
  public let `value`: DataValue
  public init(
    extensions: [String: DataValue] = [:], `images`: Presence<[Image]> = .absent,
    `kind`: String = "json", `value`: DataValue
  ) {
    self.extensions = extensions
    self.`images` = `images`
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> InputJson {
    let object = try jsonObject(json)
    let known: Set<String> = ["images", "kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      images: try readPresence(object, "images", [Image].self),
      kind: try String.read(jsonRequired(object, "kind")),
      value: try DataValue.read(jsonRequired(object, "value")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["images", "kind", "value"]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`images`, "images", &object)
    object["kind"] = `kind`.json
    object["value"] = `value`.json
    return .object(object)
  }
}
public struct InputRecords: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `items`: [Item]
  public let `kind`: String
  public init(extensions: [String: DataValue] = [:], `items`: [Item], `kind`: String = "records") {
    self.extensions = extensions
    self.`items` = `items`
    self.`kind` = `kind`
  }
  static func read(_ json: DataValue) throws -> InputRecords {
    let object = try jsonObject(json)
    let known: Set<String> = ["items", "kind"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      items: try [Item].read(jsonRequired(object, "items")),
      kind: try String.read(jsonRequired(object, "kind")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["items", "kind"]
    var object = extensions.filter { !known.contains($0.key) }
    object["items"] = `items`.json
    object["kind"] = `kind`.json
    return .object(object)
  }
}
public struct InputSource: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `source`: Source
  public init(extensions: [String: DataValue] = [:], `kind`: String = "source", `source`: Source) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`source` = `source`
  }
  static func read(_ json: DataValue) throws -> InputSource {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "source"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      source: try Source.read(jsonRequired(object, "source")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "source"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["source"] = `source`.json
    return .object(object)
  }
}
public struct InputText: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `images`: Presence<[Image]>
  public let `kind`: String
  public let `text`: String
  public init(
    extensions: [String: DataValue] = [:], `images`: Presence<[Image]> = .absent,
    `kind`: String = "text", `text`: String
  ) {
    self.extensions = extensions
    self.`images` = `images`
    self.`kind` = `kind`
    self.`text` = `text`
  }
  static func read(_ json: DataValue) throws -> InputText {
    let object = try jsonObject(json)
    let known: Set<String> = ["images", "kind", "text"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      images: try readPresence(object, "images", [Image].self),
      kind: try String.read(jsonRequired(object, "kind")),
      text: try String.read(jsonRequired(object, "text")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["images", "kind", "text"]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`images`, "images", &object)
    object["kind"] = `kind`.json
    object["text"] = `text`.json
    return .object(object)
  }
}
public struct InputUnits: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `items`: [Item]
  public let `kind`: String
  public init(extensions: [String: DataValue] = [:], `items`: [Item], `kind`: String = "units") {
    self.extensions = extensions
    self.`items` = `items`
    self.`kind` = `kind`
  }
  static func read(_ json: DataValue) throws -> InputUnits {
    let object = try jsonObject(json)
    let known: Set<String> = ["items", "kind"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      items: try [Item].read(jsonRequired(object, "items")),
      kind: try String.read(jsonRequired(object, "kind")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["items", "kind"]
    var object = extensions.filter { !known.contains($0.key) }
    object["items"] = `items`.json
    object["kind"] = `kind`.json
    return .object(object)
  }
}
public struct Item: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `context`: Presence<ContextSchema>
  public let `examples`: Presence<[RecognitionExample]>
  public let `images`: Presence<[Image]>
  public let `options`: Presence<[OptionSchema]>
  public let `original`: Presence<Original>
  public let `seedSpans`: Presence<[RecognitionSeedSpan]>
  public init(
    extensions: [String: DataValue] = [:], `context`: Presence<ContextSchema> = .absent,
    `examples`: Presence<[RecognitionExample]> = .absent, `images`: Presence<[Image]> = .absent,
    `options`: Presence<[OptionSchema]> = .absent, `original`: Presence<Original> = .absent,
    `seedSpans`: Presence<[RecognitionSeedSpan]> = .absent
  ) {
    self.extensions = extensions
    self.`context` = `context`
    self.`examples` = `examples`
    self.`images` = `images`
    self.`options` = `options`
    self.`original` = `original`
    self.`seedSpans` = `seedSpans`
  }
  static func read(_ json: DataValue) throws -> Item {
    let object = try jsonObject(json)
    let known: Set<String> = ["context", "examples", "images", "options", "original", "seed_spans"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      context: try readPresence(object, "context", ContextSchema.self),
      examples: try readPresence(object, "examples", [RecognitionExample].self),
      images: try readPresence(object, "images", [Image].self),
      options: try readPresence(object, "options", [OptionSchema].self),
      original: try readPresence(object, "original", Original.self),
      seedSpans: try readPresence(object, "seed_spans", [RecognitionSeedSpan].self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["context", "examples", "images", "options", "original", "seed_spans"]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`context`, "context", &object)
    writePresence(`examples`, "examples", &object)
    writePresence(`images`, "images", &object)
    writePresence(`options`, "options", &object)
    writePresence(`original`, "original", &object)
    writePresence(`seedSpans`, "seed_spans", &object)
    return .object(object)
  }
}
public struct Options: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `attempts`: Presence<Bool>
  public let `batch`: Presence<Batch>
  public let `context`: Presence<String>
  public let `contextField`: Presence<String>
  public let `deadlineMs`: Presence<Int64>
  public let `details`: Presence<Bool>
  public let `examples`: Presence<[RecognitionExample]>
  public let `examplesField`: Presence<String>
  public let `field`: Presence<[String]>
  public let `filesOnly`: Presence<Bool>
  public let `maxRequestsTotal`: Presence<UInt64>
  public let `mode`: Presence<RecognitionMode>
  public let `model`: Presence<String>
  public let `none`: Presence<Bool>
  public let `optionsField`: Presence<String>
  public let `relationThreshold`: Presence<Threshold>
  public let `seedSpans`: Presence<[RecognitionSeedSpan]>
  public let `seedSpansField`: Presence<String>
  public let `snippetPieces`: Presence<UInt64>
  public let `stageContext`: Presence<RecognitionStageContext>
  public let `threshold`: Presence<Threshold>
  public let `top`: Presence<UInt64>
  public init(
    extensions: [String: DataValue] = [:], `attempts`: Presence<Bool> = .absent,
    `batch`: Presence<Batch> = .absent, `context`: Presence<String> = .absent,
    `contextField`: Presence<String> = .absent, `deadlineMs`: Presence<Int64> = .absent,
    `details`: Presence<Bool> = .absent, `examples`: Presence<[RecognitionExample]> = .absent,
    `examplesField`: Presence<String> = .absent, `field`: Presence<[String]> = .absent,
    `filesOnly`: Presence<Bool> = .absent, `maxRequestsTotal`: Presence<UInt64> = .absent,
    `mode`: Presence<RecognitionMode> = .absent, `model`: Presence<String> = .absent,
    `none`: Presence<Bool> = .absent, `optionsField`: Presence<String> = .absent,
    `relationThreshold`: Presence<Threshold> = .absent,
    `seedSpans`: Presence<[RecognitionSeedSpan]> = .absent,
    `seedSpansField`: Presence<String> = .absent, `snippetPieces`: Presence<UInt64> = .absent,
    `stageContext`: Presence<RecognitionStageContext> = .absent,
    `threshold`: Presence<Threshold> = .absent, `top`: Presence<UInt64> = .absent
  ) {
    self.extensions = extensions
    self.`attempts` = `attempts`
    self.`batch` = `batch`
    self.`context` = `context`
    self.`contextField` = `contextField`
    self.`deadlineMs` = `deadlineMs`
    self.`details` = `details`
    self.`examples` = `examples`
    self.`examplesField` = `examplesField`
    self.`field` = `field`
    self.`filesOnly` = `filesOnly`
    self.`maxRequestsTotal` = `maxRequestsTotal`
    self.`mode` = `mode`
    self.`model` = `model`
    self.`none` = `none`
    self.`optionsField` = `optionsField`
    self.`relationThreshold` = `relationThreshold`
    self.`seedSpans` = `seedSpans`
    self.`seedSpansField` = `seedSpansField`
    self.`snippetPieces` = `snippetPieces`
    self.`stageContext` = `stageContext`
    self.`threshold` = `threshold`
    self.`top` = `top`
  }
  static func read(_ json: DataValue) throws -> Options {
    let object = try jsonObject(json)
    let known: Set<String> = [
      "attempts", "batch", "context", "context_field", "deadline_ms", "details", "examples",
      "examples_field", "field", "files_only", "max_requests_total", "mode", "model", "none",
      "options_field", "relation_threshold", "seed_spans", "seed_spans_field", "snippet_pieces",
      "stage_context", "threshold", "top",
    ]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      attempts: try readPresence(object, "attempts", Bool.self),
      batch: try readPresence(object, "batch", Batch.self),
      context: try readPresence(object, "context", String.self),
      contextField: try readPresence(object, "context_field", String.self),
      deadlineMs: try readPresence(object, "deadline_ms", Int64.self),
      details: try readPresence(object, "details", Bool.self),
      examples: try readPresence(object, "examples", [RecognitionExample].self),
      examplesField: try readPresence(object, "examples_field", String.self),
      field: try readPresence(object, "field", [String].self),
      filesOnly: try readPresence(object, "files_only", Bool.self),
      maxRequestsTotal: try readPresence(object, "max_requests_total", UInt64.self),
      mode: try readPresence(object, "mode", RecognitionMode.self),
      model: try readPresence(object, "model", String.self),
      none: try readPresence(object, "none", Bool.self),
      optionsField: try readPresence(object, "options_field", String.self),
      relationThreshold: try readPresence(object, "relation_threshold", Threshold.self),
      seedSpans: try readPresence(object, "seed_spans", [RecognitionSeedSpan].self),
      seedSpansField: try readPresence(object, "seed_spans_field", String.self),
      snippetPieces: try readPresence(object, "snippet_pieces", UInt64.self),
      stageContext: try readPresence(object, "stage_context", RecognitionStageContext.self),
      threshold: try readPresence(object, "threshold", Threshold.self),
      top: try readPresence(object, "top", UInt64.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = [
      "attempts", "batch", "context", "context_field", "deadline_ms", "details", "examples",
      "examples_field", "field", "files_only", "max_requests_total", "mode", "model", "none",
      "options_field", "relation_threshold", "seed_spans", "seed_spans_field", "snippet_pieces",
      "stage_context", "threshold", "top",
    ]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`attempts`, "attempts", &object)
    writePresence(`batch`, "batch", &object)
    writePresence(`context`, "context", &object)
    writePresence(`contextField`, "context_field", &object)
    writePresence(`deadlineMs`, "deadline_ms", &object)
    writePresence(`details`, "details", &object)
    writePresence(`examples`, "examples", &object)
    writePresence(`examplesField`, "examples_field", &object)
    writePresence(`field`, "field", &object)
    writePresence(`filesOnly`, "files_only", &object)
    writePresence(`maxRequestsTotal`, "max_requests_total", &object)
    writePresence(`mode`, "mode", &object)
    writePresence(`model`, "model", &object)
    writePresence(`none`, "none", &object)
    writePresence(`optionsField`, "options_field", &object)
    writePresence(`relationThreshold`, "relation_threshold", &object)
    writePresence(`seedSpans`, "seed_spans", &object)
    writePresence(`seedSpansField`, "seed_spans_field", &object)
    writePresence(`snippetPieces`, "snippet_pieces", &object)
    writePresence(`stageContext`, "stage_context", &object)
    writePresence(`threshold`, "threshold", &object)
    writePresence(`top`, "top", &object)
    return .object(object)
  }
}
public indirect enum Original: ValueRepresentable {
  case `text`(OriginalText)
  case `json`(OriginalJson)
  static func read(_ json: DataValue) throws -> Original {
    let object = try jsonObject(json)
    if object["kind"] == .string("text") { return .`text`(try OriginalText.read(json)) }
    if object["kind"] == .string("json") { return .`json`(try OriginalJson.read(json)) }
    throw ValueConversionError("Unknown Original alternative")
  }
  var json: DataValue {
    switch self {
    case .`text`(let value): return value.json
    case .`json`(let value): return value.json
    }
  }
}
public struct OriginalJson: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `value`: DataValue
  public init(extensions: [String: DataValue] = [:], `kind`: String = "json", `value`: DataValue) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> OriginalJson {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try DataValue.read(jsonRequired(object, "value")))
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
public struct OriginalText: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `text`: String
  public init(extensions: [String: DataValue] = [:], `kind`: String = "text", `text`: String) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`text` = `text`
  }
  static func read(_ json: DataValue) throws -> OriginalText {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "text"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      text: try String.read(jsonRequired(object, "text")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "text"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["text"] = `text`.json
    return .object(object)
  }
}
public indirect enum Question: ValueRepresentable {
  case `text`(QuestionText)
  case `definition`(QuestionDefinition)
  case `file`(QuestionFile)
  case `name`(QuestionName)
  case `reference`(QuestionReference)
  static func read(_ json: DataValue) throws -> Question {
    let object = try jsonObject(json)
    if object["kind"] == .string("text") { return .`text`(try QuestionText.read(json)) }
    if object["kind"] == .string("definition") {
      return .`definition`(try QuestionDefinition.read(json))
    }
    if object["kind"] == .string("file") { return .`file`(try QuestionFile.read(json)) }
    if object["kind"] == .string("name") { return .`name`(try QuestionName.read(json)) }
    if object["kind"] == .string("reference") {
      return .`reference`(try QuestionReference.read(json))
    }
    throw ValueConversionError("Unknown Question alternative")
  }
  var json: DataValue {
    switch self {
    case .`text`(let value): return value.json
    case .`definition`(let value): return value.json
    case .`file`(let value): return value.json
    case .`name`(let value): return value.json
    case .`reference`(let value): return value.json
    }
  }
}
public struct QuestionDefinition: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `value`: Definition
  public init(
    extensions: [String: DataValue] = [:], `kind`: String = "definition", `value`: Definition
  ) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`value` = `value`
  }
  static func read(_ json: DataValue) throws -> QuestionDefinition {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "value"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      value: try Definition.read(jsonRequired(object, "value")))
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
public struct QuestionFile: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `path`: String
  public init(extensions: [String: DataValue] = [:], `kind`: String = "file", `path`: String) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`path` = `path`
  }
  static func read(_ json: DataValue) throws -> QuestionFile {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "path"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      path: try String.read(jsonRequired(object, "path")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "path"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["path"] = `path`.json
    return .object(object)
  }
}
public struct QuestionName: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `name`: String
  public init(extensions: [String: DataValue] = [:], `kind`: String = "name", `name`: String) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`name` = `name`
  }
  static func read(_ json: DataValue) throws -> QuestionName {
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
public struct QuestionReference: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `reference`: String
  public init(
    extensions: [String: DataValue] = [:], `kind`: String = "reference", `reference`: String
  ) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`reference` = `reference`
  }
  static func read(_ json: DataValue) throws -> QuestionReference {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "reference"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      reference: try String.read(jsonRequired(object, "reference")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "reference"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["reference"] = `reference`.json
    return .object(object)
  }
}
public struct QuestionText: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `text`: String
  public init(extensions: [String: DataValue] = [:], `kind`: String = "text", `text`: String) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`text` = `text`
  }
  static func read(_ json: DataValue) throws -> QuestionText {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "text"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      text: try String.read(jsonRequired(object, "text")))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "text"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    object["text"] = `text`.json
    return .object(object)
  }
}
public struct Reader: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `unit`: Presence<SourceUnit>
  public let `window`: Presence<UInt64>
  public init(
    extensions: [String: DataValue] = [:], `unit`: Presence<SourceUnit> = .absent,
    `window`: Presence<UInt64> = .absent
  ) {
    self.extensions = extensions
    self.`unit` = `unit`
    self.`window` = `window`
  }
  static func read(_ json: DataValue) throws -> Reader {
    let object = try jsonObject(json)
    let known: Set<String> = ["unit", "window"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      unit: try readPresence(object, "unit", SourceUnit.self),
      window: try readPresence(object, "window", UInt64.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["unit", "window"]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`unit`, "unit", &object)
    writePresence(`window`, "window", &object)
    return .object(object)
  }
}
public indirect enum ReaderFailure: ValueRepresentable {
  case `producer`(ReaderFailureProducer)
  case `io`(ReaderFailureIo)
  case `utf8`(ReaderFailureUtf8)
  case `invalidInput`(ReaderFailureInvalidInput)
  static func read(_ json: DataValue) throws -> ReaderFailure {
    let object = try jsonObject(json)
    if object["kind"] == .string("producer") {
      return .`producer`(try ReaderFailureProducer.read(json))
    }
    if object["kind"] == .string("io") { return .`io`(try ReaderFailureIo.read(json)) }
    if object["kind"] == .string("utf8") { return .`utf8`(try ReaderFailureUtf8.read(json)) }
    if object["kind"] == .string("invalid_input") {
      return .`invalidInput`(try ReaderFailureInvalidInput.read(json))
    }
    throw ValueConversionError("Unknown ReaderFailure alternative")
  }
  var json: DataValue {
    switch self {
    case .`producer`(let value): return value.json
    case .`io`(let value): return value.json
    case .`utf8`(let value): return value.json
    case .`invalidInput`(let value): return value.json
    }
  }
}
public struct ReaderFailureInvalidInput: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `location`: Presence<SessionSourceLocation>
  public init(
    extensions: [String: DataValue] = [:], `kind`: String = "invalid_input",
    `location`: Presence<SessionSourceLocation> = .absent
  ) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`location` = `location`
  }
  static func read(_ json: DataValue) throws -> ReaderFailureInvalidInput {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "location"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      location: try readPresence(object, "location", SessionSourceLocation.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "location"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    writePresence(`location`, "location", &object)
    return .object(object)
  }
}
public struct ReaderFailureIo: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `location`: Presence<SessionSourceLocation>
  public init(
    extensions: [String: DataValue] = [:], `kind`: String = "io",
    `location`: Presence<SessionSourceLocation> = .absent
  ) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`location` = `location`
  }
  static func read(_ json: DataValue) throws -> ReaderFailureIo {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "location"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      location: try readPresence(object, "location", SessionSourceLocation.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "location"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    writePresence(`location`, "location", &object)
    return .object(object)
  }
}
public struct ReaderFailureProducer: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public init(extensions: [String: DataValue] = [:], `kind`: String = "producer") {
    self.extensions = extensions
    self.`kind` = `kind`
  }
  static func read(_ json: DataValue) throws -> ReaderFailureProducer {
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
public struct ReaderFailureUtf8: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `kind`: String
  public let `location`: Presence<SessionSourceLocation>
  public init(
    extensions: [String: DataValue] = [:], `kind`: String = "utf8",
    `location`: Presence<SessionSourceLocation> = .absent
  ) {
    self.extensions = extensions
    self.`kind` = `kind`
    self.`location` = `location`
  }
  static func read(_ json: DataValue) throws -> ReaderFailureUtf8 {
    let object = try jsonObject(json)
    let known: Set<String> = ["kind", "location"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      kind: try String.read(jsonRequired(object, "kind")),
      location: try readPresence(object, "location", SessionSourceLocation.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["kind", "location"]
    var object = extensions.filter { !known.contains($0.key) }
    object["kind"] = `kind`.json
    writePresence(`location`, "location", &object)
    return .object(object)
  }
}
public struct SessionDescriptor: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `item`: Item
  public let `location`: Presence<SessionSourceLocation>
  public init(
    extensions: [String: DataValue] = [:], `item`: Item,
    `location`: Presence<SessionSourceLocation> = .absent
  ) {
    self.extensions = extensions
    self.`item` = `item`
    self.`location` = `location`
  }
  static func read(_ json: DataValue) throws -> SessionDescriptor {
    let object = try jsonObject(json)
    let known: Set<String> = ["item", "location"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      item: try Item.read(jsonRequired(object, "item")),
      location: try readPresence(object, "location", SessionSourceLocation.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["item", "location"]
    var object = extensions.filter { !known.contains($0.key) }
    object["item"] = `item`.json
    writePresence(`location`, "location", &object)
    return .object(object)
  }
}
public struct Source: ValueRepresentable {
  public let extensions: [String: DataValue]
  private var _sourceJSON: DataValue? = nil
  public let `framing`: Presence<Framing>
  public let `media`: Presence<ReaderMedia>
  public let `paths`: [String]
  public let `reading`: Presence<Reader>
  public init(
    extensions: [String: DataValue] = [:], `framing`: Presence<Framing> = .absent,
    `media`: Presence<ReaderMedia> = .absent, `paths`: [String],
    `reading`: Presence<Reader> = .absent
  ) {
    self.extensions = extensions
    self.`framing` = `framing`
    self.`media` = `media`
    self.`paths` = `paths`
    self.`reading` = `reading`
  }
  static func read(_ json: DataValue) throws -> Source {
    let object = try jsonObject(json)
    let known: Set<String> = ["framing", "media", "paths", "reading"]
    var value = Self(
      extensions: object.filter { !known.contains($0.key) },
      framing: try readPresence(object, "framing", Framing.self),
      media: try readPresence(object, "media", ReaderMedia.self),
      paths: try [String].read(jsonRequired(object, "paths")),
      reading: try readPresence(object, "reading", Reader.self))
    value._sourceJSON = json
    return value
  }
  var json: DataValue {
    if let _sourceJSON { return _sourceJSON }
    let known: Set<String> = ["framing", "media", "paths", "reading"]
    var object = extensions.filter { !known.contains($0.key) }
    writePresence(`framing`, "framing", &object)
    writePresence(`media`, "media", &object)
    object["paths"] = `paths`.json
    writePresence(`reading`, "reading", &object)
    return .object(object)
  }
}
public indirect enum Threshold: ValueRepresentable {
  case `number`(Double)
  case `string`(String)
  static func read(_ json: DataValue) throws -> Threshold {
    if let value = try? Double.read(json) { return .`number`(value) }
    if let value = try? String.read(json) { return .`string`(value) }
    throw ValueConversionError("Unknown Threshold value")
  }
  var json: DataValue {
    switch self {
    case .`number`(let value): return value.json
    case .`string`(let value): return value.json
    }
  }
}
public enum Version: ValueRepresentable {
  case `thinkthenRequest_1`
  static func read(_ json: DataValue) throws -> Version {
    let value = try String.read(json)
    switch value {
    case "thinkthen.request/1": return .`thinkthenRequest_1`
    default: throw ValueConversionError("Unknown Version value")
    }
  }
  var json: DataValue {
    switch self {
    case .`thinkthenRequest_1`: return .string("thinkthen.request/1")
    }
  }
}
public struct SessionSourceLocation: ValueRepresentable {
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
  static func read(_ json: DataValue) throws -> SessionSourceLocation {
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
public enum SourceUnit: ValueRepresentable {
  case `line`
  case `window`
  case `file`
  static func read(_ json: DataValue) throws -> SourceUnit {
    let value = try String.read(json)
    switch value {
    case "line": return .`line`
    case "window": return .`window`
    case "file": return .`file`
    default: throw ValueConversionError("Unknown SourceUnit value")
    }
  }
  var json: DataValue {
    switch self {
    case .`line`: return .string("line")
    case .`window`: return .string("window")
    case .`file`: return .string("file")
    }
  }
}
extension Client {
  public func decide(_ question: Question, input: Input, options: Presence<Options> = .absent)
    async throws -> Call
  {
    try await execute(
      Request(
        call: CallRequest(function: .`decide`, input: input, options: options, question: question),
        schema: .`thinkthenRequest_1`))
  }
  public func choose(_ question: Question, input: Input, options: Presence<Options> = .absent)
    async throws -> Call
  {
    try await execute(
      Request(
        call: CallRequest(function: .`choose`, input: input, options: options, question: question),
        schema: .`thinkthenRequest_1`))
  }
  public func tag(_ question: Question, input: Input, options: Presence<Options> = .absent)
    async throws -> Call
  {
    try await execute(
      Request(
        call: CallRequest(function: .`tag`, input: input, options: options, question: question),
        schema: .`thinkthenRequest_1`))
  }
  public func score(_ question: Question, input: Input, options: Presence<Options> = .absent)
    async throws -> Call
  {
    try await execute(
      Request(
        call: CallRequest(function: .`score`, input: input, options: options, question: question),
        schema: .`thinkthenRequest_1`))
  }
  public func filter(_ question: Question, input: Input, options: Presence<Options> = .absent)
    async throws -> Call
  {
    try await execute(
      Request(
        call: CallRequest(function: .`filter`, input: input, options: options, question: question),
        schema: .`thinkthenRequest_1`))
  }
  public func rank(_ question: Question, input: Input, options: Presence<Options> = .absent)
    async throws -> Call
  {
    try await execute(
      Request(
        call: CallRequest(function: .`rank`, input: input, options: options, question: question),
        schema: .`thinkthenRequest_1`))
  }
  public func find(_ question: Question, input: Input, options: Presence<Options> = .absent)
    async throws -> Call
  {
    try await execute(
      Request(
        call: CallRequest(function: .`find`, input: input, options: options, question: question),
        schema: .`thinkthenRequest_1`))
  }
  public func annotate(_ question: Question, input: Input, options: Presence<Options> = .absent)
    async throws -> Call
  {
    try await execute(
      Request(
        call: CallRequest(
          function: .`annotate`, input: input, options: options, question: question),
        schema: .`thinkthenRequest_1`))
  }
  public func recognize(_ question: Question, input: Input, options: Presence<Options> = .absent)
    async throws -> Call
  {
    try await execute(
      Request(
        call: CallRequest(
          function: .`recognize`, input: input, options: options, question: question),
        schema: .`thinkthenRequest_1`))
  }
  public func relate(_ question: Question, input: Input, options: Presence<Options> = .absent)
    async throws -> Call
  {
    try await execute(
      Request(
        call: CallRequest(function: .`relate`, input: input, options: options, question: question),
        schema: .`thinkthenRequest_1`))
  }
}
public let ownedRequestVersion = "thinkthen.request/1"
