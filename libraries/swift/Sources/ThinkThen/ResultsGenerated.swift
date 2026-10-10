// Generated from the shared Rust graph. Do not edit.
import Foundation
public struct OwnedAnnotation: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `answerId`: OwnedAnswerId
    public let `answers`: [String: OwnedAnnotationMember]
    public let `file`: Presence<OwnedAnnotationFile>
    public let `firstLine`: Presence<OwnedAnnotationFirstLine>
    public let `index`: Presence<OwnedAnnotationIndex>
    public let `input`: JSONValue
    public let `lastLine`: Presence<OwnedAnnotationLastLine>
    public let `meta`: OwnedMeta
    public let `position`: Presence<OwnedPosition>
    public let `schema`: OwnedVersion
    public let `source`: Presence<OwnedPhysicalSource>
    public let `value`: OwnedAnnotatedRow
    public init(extensions: [String: JSONValue] = [:], `answerId`: OwnedAnswerId, `answers`: [String: OwnedAnnotationMember], `file`: Presence<OwnedAnnotationFile> = .absent, `firstLine`: Presence<OwnedAnnotationFirstLine> = .absent, `index`: Presence<OwnedAnnotationIndex> = .absent, `input`: JSONValue, `lastLine`: Presence<OwnedAnnotationLastLine> = .absent, `meta`: OwnedMeta, `position`: Presence<OwnedPosition> = .absent, `schema`: OwnedVersion, `source`: Presence<OwnedPhysicalSource> = .absent, `value`: OwnedAnnotatedRow) { self.extensions = extensions; self.`answerId` = `answerId`; self.`answers` = `answers`; self.`file` = `file`; self.`firstLine` = `firstLine`; self.`index` = `index`; self.`input` = `input`; self.`lastLine` = `lastLine`; self.`meta` = `meta`; self.`position` = `position`; self.`schema` = `schema`; self.`source` = `source`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedAnnotation {
        let object = try jsonObject(json)
        let known: Set<String> = ["answer_id", "answers", "file", "first_line", "index", "input", "last_line", "meta", "position", "schema", "source", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, answerId: try OwnedAnswerId.read(jsonRequired(object, "answer_id")), answers: try [String: OwnedAnnotationMember].read(jsonRequired(object, "answers")), file: try readPresence(object, "file", OwnedAnnotationFile.self), firstLine: try readPresence(object, "first_line", OwnedAnnotationFirstLine.self), index: try readPresence(object, "index", OwnedAnnotationIndex.self), input: try JSONValue.read(jsonRequired(object, "input")), lastLine: try readPresence(object, "last_line", OwnedAnnotationLastLine.self), meta: try OwnedMeta.read(jsonRequired(object, "meta")), position: try readPresence(object, "position", OwnedPosition.self), schema: try OwnedVersion.read(jsonRequired(object, "schema")), source: try readPresence(object, "source", OwnedPhysicalSource.self), value: try OwnedAnnotatedRow.read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["answer_id", "answers", "file", "first_line", "index", "input", "last_line", "meta", "position", "schema", "source", "value"]
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
public indirect enum OwnedAnnotationMember: JSONRepresentable {
    case `answerId`(OwnedAnnotationMemberAnswerId)
    case `failureId`(OwnedAnnotationMemberFailureId)
    public static func read(_ json: JSONValue) throws -> OwnedAnnotationMember {
        let object = try jsonObject(json)
        if object["answer_id"] != nil && object["answer_id"] != .null { return .`answerId`(try OwnedAnnotationMemberAnswerId.read(json)) }
        if object["failure_id"] != nil && object["failure_id"] != .null { return .`failureId`(try OwnedAnnotationMemberFailureId.read(json)) }
        throw JSONConversionError("Unknown OwnedAnnotationMember alternative")
    }
    public var json: JSONValue {
        switch self {
        case .`answerId`(let value): return value.json
        case .`failureId`(let value): return value.json
        }
    }
}
public struct OwnedAnnotationMemberAnswerId: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `answer`: OwnedAnswer
    public let `answerId`: OwnedAnswerId
    public let `observations`: [OwnedObservation]
    public let `question`: OwnedReadableQuestion
    public let `questionSources`: [OwnedQuestionSource]
    public let `request`: String
    public let `threshold`: OwnedThreshold?
    public let `usage`: Presence<OwnedUsage>
    public let `value`: OwnedValue
    public init(extensions: [String: JSONValue] = [:], `answer`: OwnedAnswer, `answerId`: OwnedAnswerId, `observations`: [OwnedObservation], `question`: OwnedReadableQuestion, `questionSources`: [OwnedQuestionSource], `request`: String, `threshold`: OwnedThreshold?, `usage`: Presence<OwnedUsage> = .absent, `value`: OwnedValue) { self.extensions = extensions; self.`answer` = `answer`; self.`answerId` = `answerId`; self.`observations` = `observations`; self.`question` = `question`; self.`questionSources` = `questionSources`; self.`request` = `request`; self.`threshold` = `threshold`; self.`usage` = `usage`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedAnnotationMemberAnswerId {
        let object = try jsonObject(json)
        let known: Set<String> = ["answer", "answer_id", "observations", "question", "question_sources", "request", "threshold", "usage", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, answer: try OwnedAnswer.read(jsonRequired(object, "answer")), answerId: try OwnedAnswerId.read(jsonRequired(object, "answer_id")), observations: try [OwnedObservation].read(jsonRequired(object, "observations")), question: try OwnedReadableQuestion.read(jsonRequired(object, "question")), questionSources: try [OwnedQuestionSource].read(jsonRequired(object, "question_sources")), request: try String.read(jsonRequired(object, "request")), threshold: try OwnedThreshold?.read(jsonRequired(object, "threshold")), usage: try readPresence(object, "usage", OwnedUsage.self), value: try OwnedValue.read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["answer", "answer_id", "observations", "question", "question_sources", "request", "threshold", "usage", "value"]
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
public struct OwnedAnnotationMemberFailureId: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `failure`: OwnedFailure
    public let `failureId`: OwnedFailureId
    public let `observations`: [OwnedObservation]
    public let `question`: OwnedReadableQuestion
    public let `questionSources`: [OwnedQuestionSource]
    public let `request`: String
    public let `threshold`: OwnedThreshold?
    public let `usage`: Presence<OwnedUsage>
    public init(extensions: [String: JSONValue] = [:], `failure`: OwnedFailure, `failureId`: OwnedFailureId, `observations`: [OwnedObservation], `question`: OwnedReadableQuestion, `questionSources`: [OwnedQuestionSource], `request`: String, `threshold`: OwnedThreshold?, `usage`: Presence<OwnedUsage> = .absent) { self.extensions = extensions; self.`failure` = `failure`; self.`failureId` = `failureId`; self.`observations` = `observations`; self.`question` = `question`; self.`questionSources` = `questionSources`; self.`request` = `request`; self.`threshold` = `threshold`; self.`usage` = `usage` }
    public static func read(_ json: JSONValue) throws -> OwnedAnnotationMemberFailureId {
        let object = try jsonObject(json)
        let known: Set<String> = ["failure", "failure_id", "observations", "question", "question_sources", "request", "threshold", "usage"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, failure: try OwnedFailure.read(jsonRequired(object, "failure")), failureId: try OwnedFailureId.read(jsonRequired(object, "failure_id")), observations: try [OwnedObservation].read(jsonRequired(object, "observations")), question: try OwnedReadableQuestion.read(jsonRequired(object, "question")), questionSources: try [OwnedQuestionSource].read(jsonRequired(object, "question_sources")), request: try String.read(jsonRequired(object, "request")), threshold: try OwnedThreshold?.read(jsonRequired(object, "threshold")), usage: try readPresence(object, "usage", OwnedUsage.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["failure", "failure_id", "observations", "question", "question_sources", "request", "threshold", "usage"]
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
public indirect enum OwnedAnnotationValue: JSONRepresentable {
    case `decision`(OwnedAnnotationValueDecision)
    case `choice`(OwnedAnnotationValueChoice)
    case `score`(OwnedAnnotationValueScore)
    case `tags`(OwnedAnnotationValueTags)
    case `failed`(OwnedAnnotationValueFailed)
    public static func read(_ json: JSONValue) throws -> OwnedAnnotationValue {
        let object = try jsonObject(json)
        if object["kind"] == .string("decision") { return .`decision`(try OwnedAnnotationValueDecision.read(json)) }
        if object["kind"] == .string("choice") { return .`choice`(try OwnedAnnotationValueChoice.read(json)) }
        if object["kind"] == .string("score") { return .`score`(try OwnedAnnotationValueScore.read(json)) }
        if object["kind"] == .string("tags") { return .`tags`(try OwnedAnnotationValueTags.read(json)) }
        if object["kind"] == .string("failed") { return .`failed`(try OwnedAnnotationValueFailed.read(json)) }
        throw JSONConversionError("Unknown OwnedAnnotationValue alternative")
    }
    public var json: JSONValue {
        switch self {
        case .`decision`(let value): return value.json
        case .`choice`(let value): return value.json
        case .`score`(let value): return value.json
        case .`tags`(let value): return value.json
        case .`failed`(let value): return value.json
        }
    }
}
public struct OwnedAnnotationValueChoice: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `value`: OwnedAnnotationValueChoiceValue
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "choice", `value`: OwnedAnnotationValueChoiceValue) { self.extensions = extensions; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedAnnotationValueChoice {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), value: try OwnedAnnotationValueChoiceValue.read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public indirect enum OwnedAnnotationValueChoiceValue: JSONRepresentable {
    case alternative0(String)
    case alternative1(JSONValue)
    public static func read(_ json: JSONValue) throws -> OwnedAnnotationValueChoiceValue {
        if let value = try? String.read(json) { return .alternative0(value) }
        if json == .null { return .alternative1(.null) }
        throw JSONConversionError("Unknown OwnedAnnotationValueChoiceValue value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public struct OwnedAnnotationValueDecision: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `value`: OwnedAnnotationValueDecisionValue
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "decision", `value`: OwnedAnnotationValueDecisionValue) { self.extensions = extensions; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedAnnotationValueDecision {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), value: try OwnedAnnotationValueDecisionValue.read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public indirect enum OwnedAnnotationValueDecisionValue: JSONRepresentable {
    case alternative0(Bool)
    case alternative1(JSONValue)
    public static func read(_ json: JSONValue) throws -> OwnedAnnotationValueDecisionValue {
        if let value = try? Bool.read(json) { return .alternative0(value) }
        if json == .null { return .alternative1(.null) }
        throw JSONConversionError("Unknown OwnedAnnotationValueDecisionValue value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public struct OwnedAnnotationValueFailed: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `value`: OwnedFailure
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "failed", `value`: OwnedFailure) { self.extensions = extensions; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedAnnotationValueFailed {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), value: try OwnedFailure.read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public struct OwnedAnnotationValueScore: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `value`: Double
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "score", `value`: Double) { self.extensions = extensions; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedAnnotationValueScore {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), value: try Double.read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public struct OwnedAnnotationValueTags: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `value`: [String]
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "tags", `value`: [String]) { self.extensions = extensions; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedAnnotationValueTags {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), value: try [String].read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public indirect enum OwnedAnnotationFile: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedAnnotationFile {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedAnnotationFile value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedAnnotationFirstLine: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedAnnotationFirstLine {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedAnnotationFirstLine value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedAnnotationIndex: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedAnnotationIndex {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedAnnotationIndex value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedAnnotationLastLine: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedAnnotationLastLine {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedAnnotationLastLine value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public typealias OwnedAnswerId = String
public struct OwnedAnswers: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `questions`: [OwnedRelationMember]
    public init(extensions: [String: JSONValue] = [:], `questions`: [OwnedRelationMember]) { self.extensions = extensions; self.`questions` = `questions` }
    public static func read(_ json: JSONValue) throws -> OwnedAnswers {
        let object = try jsonObject(json)
        let known: Set<String> = ["questions"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, questions: try [OwnedRelationMember].read(jsonRequired(object, "questions")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["questions"]
        var object = extensions.filter { !known.contains($0.key) }
        object["questions"] = `questions`.json
        return .object(object)
    }
}
public struct OwnedAtomicArrayOfString: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `answer`: OwnedAnswer
    public let `answerId`: OwnedAnswerId
    public let `images`: Presence<OwnedAtomicArrayOfStringImages>
    public let `index`: Presence<OwnedAtomicArrayOfStringIndex>
    public let `input`: Presence<JSONValue>
    public let `members`: Presence<OwnedAtomicArrayOfStringMembers>
    public let `meta`: OwnedMeta
    public let `question`: OwnedReadableQuestion
    public let `questionName`: Presence<OwnedAtomicArrayOfStringQuestionName>
    public let `schema`: OwnedVersion
    public let `source`: Presence<OwnedPhysicalSource>
    public let `threshold`: OwnedThreshold?
    public let `value`: [String]
    public init(extensions: [String: JSONValue] = [:], `answer`: OwnedAnswer, `answerId`: OwnedAnswerId, `images`: Presence<OwnedAtomicArrayOfStringImages> = .absent, `index`: Presence<OwnedAtomicArrayOfStringIndex> = .absent, `input`: Presence<JSONValue> = .absent, `members`: Presence<OwnedAtomicArrayOfStringMembers> = .absent, `meta`: OwnedMeta, `question`: OwnedReadableQuestion, `questionName`: Presence<OwnedAtomicArrayOfStringQuestionName> = .absent, `schema`: OwnedVersion, `source`: Presence<OwnedPhysicalSource> = .absent, `threshold`: OwnedThreshold?, `value`: [String]) { self.extensions = extensions; self.`answer` = `answer`; self.`answerId` = `answerId`; self.`images` = `images`; self.`index` = `index`; self.`input` = `input`; self.`members` = `members`; self.`meta` = `meta`; self.`question` = `question`; self.`questionName` = `questionName`; self.`schema` = `schema`; self.`source` = `source`; self.`threshold` = `threshold`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedAtomicArrayOfString {
        let object = try jsonObject(json)
        let known: Set<String> = ["answer", "answer_id", "images", "index", "input", "members", "meta", "question", "question_name", "schema", "source", "threshold", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, answer: try OwnedAnswer.read(jsonRequired(object, "answer")), answerId: try OwnedAnswerId.read(jsonRequired(object, "answer_id")), images: try readPresence(object, "images", OwnedAtomicArrayOfStringImages.self), index: try readPresence(object, "index", OwnedAtomicArrayOfStringIndex.self), input: try readPresence(object, "input", JSONValue.self), members: try readPresence(object, "members", OwnedAtomicArrayOfStringMembers.self), meta: try OwnedMeta.read(jsonRequired(object, "meta")), question: try OwnedReadableQuestion.read(jsonRequired(object, "question")), questionName: try readPresence(object, "question_name", OwnedAtomicArrayOfStringQuestionName.self), schema: try OwnedVersion.read(jsonRequired(object, "schema")), source: try readPresence(object, "source", OwnedPhysicalSource.self), threshold: try OwnedThreshold?.read(jsonRequired(object, "threshold")), value: try [String].read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["answer", "answer_id", "images", "index", "input", "members", "meta", "question", "question_name", "schema", "source", "threshold", "value"]
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
public indirect enum OwnedAtomicArrayOfStringImages: JSONRepresentable {
    case alternative0([OwnedImage])
    public static func read(_ json: JSONValue) throws -> OwnedAtomicArrayOfStringImages {
        if let value = try? [OwnedImage].read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedAtomicArrayOfStringImages value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedAtomicArrayOfStringIndex: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedAtomicArrayOfStringIndex {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedAtomicArrayOfStringIndex value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedAtomicArrayOfStringMembers: JSONRepresentable {
    case alternative0([OwnedRankMember])
    public static func read(_ json: JSONValue) throws -> OwnedAtomicArrayOfStringMembers {
        if let value = try? [OwnedRankMember].read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedAtomicArrayOfStringMembers value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedAtomicArrayOfStringQuestionName: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedAtomicArrayOfStringQuestionName {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedAtomicArrayOfStringQuestionName value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public struct OwnedAtomicDecideValue: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `answer`: OwnedAnswer
    public let `answerId`: OwnedAnswerId
    public let `images`: Presence<OwnedAtomicDecideValueImages>
    public let `index`: Presence<OwnedAtomicDecideValueIndex>
    public let `input`: Presence<JSONValue>
    public let `members`: Presence<OwnedAtomicDecideValueMembers>
    public let `meta`: OwnedMeta
    public let `question`: OwnedReadableQuestion
    public let `questionName`: Presence<OwnedAtomicDecideValueQuestionName>
    public let `schema`: OwnedVersion
    public let `source`: Presence<OwnedPhysicalSource>
    public let `threshold`: OwnedThreshold?
    public let `value`: OwnedDecideValue
    public init(extensions: [String: JSONValue] = [:], `answer`: OwnedAnswer, `answerId`: OwnedAnswerId, `images`: Presence<OwnedAtomicDecideValueImages> = .absent, `index`: Presence<OwnedAtomicDecideValueIndex> = .absent, `input`: Presence<JSONValue> = .absent, `members`: Presence<OwnedAtomicDecideValueMembers> = .absent, `meta`: OwnedMeta, `question`: OwnedReadableQuestion, `questionName`: Presence<OwnedAtomicDecideValueQuestionName> = .absent, `schema`: OwnedVersion, `source`: Presence<OwnedPhysicalSource> = .absent, `threshold`: OwnedThreshold?, `value`: OwnedDecideValue) { self.extensions = extensions; self.`answer` = `answer`; self.`answerId` = `answerId`; self.`images` = `images`; self.`index` = `index`; self.`input` = `input`; self.`members` = `members`; self.`meta` = `meta`; self.`question` = `question`; self.`questionName` = `questionName`; self.`schema` = `schema`; self.`source` = `source`; self.`threshold` = `threshold`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedAtomicDecideValue {
        let object = try jsonObject(json)
        let known: Set<String> = ["answer", "answer_id", "images", "index", "input", "members", "meta", "question", "question_name", "schema", "source", "threshold", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, answer: try OwnedAnswer.read(jsonRequired(object, "answer")), answerId: try OwnedAnswerId.read(jsonRequired(object, "answer_id")), images: try readPresence(object, "images", OwnedAtomicDecideValueImages.self), index: try readPresence(object, "index", OwnedAtomicDecideValueIndex.self), input: try readPresence(object, "input", JSONValue.self), members: try readPresence(object, "members", OwnedAtomicDecideValueMembers.self), meta: try OwnedMeta.read(jsonRequired(object, "meta")), question: try OwnedReadableQuestion.read(jsonRequired(object, "question")), questionName: try readPresence(object, "question_name", OwnedAtomicDecideValueQuestionName.self), schema: try OwnedVersion.read(jsonRequired(object, "schema")), source: try readPresence(object, "source", OwnedPhysicalSource.self), threshold: try OwnedThreshold?.read(jsonRequired(object, "threshold")), value: try OwnedDecideValue.read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["answer", "answer_id", "images", "index", "input", "members", "meta", "question", "question_name", "schema", "source", "threshold", "value"]
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
public indirect enum OwnedAtomicDecideValueImages: JSONRepresentable {
    case alternative0([OwnedImage])
    public static func read(_ json: JSONValue) throws -> OwnedAtomicDecideValueImages {
        if let value = try? [OwnedImage].read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedAtomicDecideValueImages value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedAtomicDecideValueIndex: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedAtomicDecideValueIndex {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedAtomicDecideValueIndex value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedAtomicDecideValueMembers: JSONRepresentable {
    case alternative0([OwnedRankMember])
    public static func read(_ json: JSONValue) throws -> OwnedAtomicDecideValueMembers {
        if let value = try? [OwnedRankMember].read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedAtomicDecideValueMembers value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedAtomicDecideValueQuestionName: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedAtomicDecideValueQuestionName {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedAtomicDecideValueQuestionName value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public struct OwnedAtomicNonZeroUsize: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `answer`: OwnedAnswer
    public let `answerId`: OwnedAnswerId
    public let `images`: Presence<OwnedAtomicNonZeroUsizeImages>
    public let `index`: Presence<OwnedAtomicNonZeroUsizeIndex>
    public let `input`: Presence<JSONValue>
    public let `members`: Presence<OwnedAtomicNonZeroUsizeMembers>
    public let `meta`: OwnedMeta
    public let `question`: OwnedReadableQuestion
    public let `questionName`: Presence<OwnedAtomicNonZeroUsizeQuestionName>
    public let `schema`: OwnedVersion
    public let `source`: Presence<OwnedPhysicalSource>
    public let `threshold`: OwnedThreshold?
    public let `value`: UInt64
    public init(extensions: [String: JSONValue] = [:], `answer`: OwnedAnswer, `answerId`: OwnedAnswerId, `images`: Presence<OwnedAtomicNonZeroUsizeImages> = .absent, `index`: Presence<OwnedAtomicNonZeroUsizeIndex> = .absent, `input`: Presence<JSONValue> = .absent, `members`: Presence<OwnedAtomicNonZeroUsizeMembers> = .absent, `meta`: OwnedMeta, `question`: OwnedReadableQuestion, `questionName`: Presence<OwnedAtomicNonZeroUsizeQuestionName> = .absent, `schema`: OwnedVersion, `source`: Presence<OwnedPhysicalSource> = .absent, `threshold`: OwnedThreshold?, `value`: UInt64) { self.extensions = extensions; self.`answer` = `answer`; self.`answerId` = `answerId`; self.`images` = `images`; self.`index` = `index`; self.`input` = `input`; self.`members` = `members`; self.`meta` = `meta`; self.`question` = `question`; self.`questionName` = `questionName`; self.`schema` = `schema`; self.`source` = `source`; self.`threshold` = `threshold`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedAtomicNonZeroUsize {
        let object = try jsonObject(json)
        let known: Set<String> = ["answer", "answer_id", "images", "index", "input", "members", "meta", "question", "question_name", "schema", "source", "threshold", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, answer: try OwnedAnswer.read(jsonRequired(object, "answer")), answerId: try OwnedAnswerId.read(jsonRequired(object, "answer_id")), images: try readPresence(object, "images", OwnedAtomicNonZeroUsizeImages.self), index: try readPresence(object, "index", OwnedAtomicNonZeroUsizeIndex.self), input: try readPresence(object, "input", JSONValue.self), members: try readPresence(object, "members", OwnedAtomicNonZeroUsizeMembers.self), meta: try OwnedMeta.read(jsonRequired(object, "meta")), question: try OwnedReadableQuestion.read(jsonRequired(object, "question")), questionName: try readPresence(object, "question_name", OwnedAtomicNonZeroUsizeQuestionName.self), schema: try OwnedVersion.read(jsonRequired(object, "schema")), source: try readPresence(object, "source", OwnedPhysicalSource.self), threshold: try OwnedThreshold?.read(jsonRequired(object, "threshold")), value: try UInt64.read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["answer", "answer_id", "images", "index", "input", "members", "meta", "question", "question_name", "schema", "source", "threshold", "value"]
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
public indirect enum OwnedAtomicNonZeroUsizeImages: JSONRepresentable {
    case alternative0([OwnedImage])
    public static func read(_ json: JSONValue) throws -> OwnedAtomicNonZeroUsizeImages {
        if let value = try? [OwnedImage].read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedAtomicNonZeroUsizeImages value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedAtomicNonZeroUsizeIndex: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedAtomicNonZeroUsizeIndex {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedAtomicNonZeroUsizeIndex value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedAtomicNonZeroUsizeMembers: JSONRepresentable {
    case alternative0([OwnedRankMember])
    public static func read(_ json: JSONValue) throws -> OwnedAtomicNonZeroUsizeMembers {
        if let value = try? [OwnedRankMember].read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedAtomicNonZeroUsizeMembers value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedAtomicNonZeroUsizeQuestionName: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedAtomicNonZeroUsizeQuestionName {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedAtomicNonZeroUsizeQuestionName value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public struct OwnedAtomicNullableString: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `answer`: OwnedAnswer
    public let `answerId`: OwnedAnswerId
    public let `images`: Presence<OwnedAtomicNullableStringImages>
    public let `index`: Presence<OwnedAtomicNullableStringIndex>
    public let `input`: Presence<JSONValue>
    public let `members`: Presence<OwnedAtomicNullableStringMembers>
    public let `meta`: OwnedMeta
    public let `question`: OwnedReadableQuestion
    public let `questionName`: Presence<OwnedAtomicNullableStringQuestionName>
    public let `schema`: OwnedVersion
    public let `source`: Presence<OwnedPhysicalSource>
    public let `threshold`: OwnedThreshold?
    public let `value`: OwnedAtomicNullableStringValue
    public init(extensions: [String: JSONValue] = [:], `answer`: OwnedAnswer, `answerId`: OwnedAnswerId, `images`: Presence<OwnedAtomicNullableStringImages> = .absent, `index`: Presence<OwnedAtomicNullableStringIndex> = .absent, `input`: Presence<JSONValue> = .absent, `members`: Presence<OwnedAtomicNullableStringMembers> = .absent, `meta`: OwnedMeta, `question`: OwnedReadableQuestion, `questionName`: Presence<OwnedAtomicNullableStringQuestionName> = .absent, `schema`: OwnedVersion, `source`: Presence<OwnedPhysicalSource> = .absent, `threshold`: OwnedThreshold?, `value`: OwnedAtomicNullableStringValue) { self.extensions = extensions; self.`answer` = `answer`; self.`answerId` = `answerId`; self.`images` = `images`; self.`index` = `index`; self.`input` = `input`; self.`members` = `members`; self.`meta` = `meta`; self.`question` = `question`; self.`questionName` = `questionName`; self.`schema` = `schema`; self.`source` = `source`; self.`threshold` = `threshold`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedAtomicNullableString {
        let object = try jsonObject(json)
        let known: Set<String> = ["answer", "answer_id", "images", "index", "input", "members", "meta", "question", "question_name", "schema", "source", "threshold", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, answer: try OwnedAnswer.read(jsonRequired(object, "answer")), answerId: try OwnedAnswerId.read(jsonRequired(object, "answer_id")), images: try readPresence(object, "images", OwnedAtomicNullableStringImages.self), index: try readPresence(object, "index", OwnedAtomicNullableStringIndex.self), input: try readPresence(object, "input", JSONValue.self), members: try readPresence(object, "members", OwnedAtomicNullableStringMembers.self), meta: try OwnedMeta.read(jsonRequired(object, "meta")), question: try OwnedReadableQuestion.read(jsonRequired(object, "question")), questionName: try readPresence(object, "question_name", OwnedAtomicNullableStringQuestionName.self), schema: try OwnedVersion.read(jsonRequired(object, "schema")), source: try readPresence(object, "source", OwnedPhysicalSource.self), threshold: try OwnedThreshold?.read(jsonRequired(object, "threshold")), value: try OwnedAtomicNullableStringValue.read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["answer", "answer_id", "images", "index", "input", "members", "meta", "question", "question_name", "schema", "source", "threshold", "value"]
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
public indirect enum OwnedAtomicNullableStringImages: JSONRepresentable {
    case alternative0([OwnedImage])
    public static func read(_ json: JSONValue) throws -> OwnedAtomicNullableStringImages {
        if let value = try? [OwnedImage].read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedAtomicNullableStringImages value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedAtomicNullableStringIndex: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedAtomicNullableStringIndex {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedAtomicNullableStringIndex value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedAtomicNullableStringMembers: JSONRepresentable {
    case alternative0([OwnedRankMember])
    public static func read(_ json: JSONValue) throws -> OwnedAtomicNullableStringMembers {
        if let value = try? [OwnedRankMember].read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedAtomicNullableStringMembers value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedAtomicNullableStringQuestionName: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedAtomicNullableStringQuestionName {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedAtomicNullableStringQuestionName value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedAtomicNullableStringValue: JSONRepresentable {
    case alternative0(String)
    case alternative1(JSONValue)
    public static func read(_ json: JSONValue) throws -> OwnedAtomicNullableStringValue {
        if let value = try? String.read(json) { return .alternative0(value) }
        if json == .null { return .alternative1(.null) }
        throw JSONConversionError("Unknown OwnedAtomicNullableStringValue value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public struct OwnedAtomicBoolean: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `answer`: OwnedAnswer
    public let `answerId`: OwnedAnswerId
    public let `images`: Presence<OwnedAtomicBooleanImages>
    public let `index`: Presence<OwnedAtomicBooleanIndex>
    public let `input`: Presence<JSONValue>
    public let `members`: Presence<OwnedAtomicBooleanMembers>
    public let `meta`: OwnedMeta
    public let `question`: OwnedReadableQuestion
    public let `questionName`: Presence<OwnedAtomicBooleanQuestionName>
    public let `schema`: OwnedVersion
    public let `source`: Presence<OwnedPhysicalSource>
    public let `threshold`: OwnedThreshold?
    public let `value`: Bool
    public init(extensions: [String: JSONValue] = [:], `answer`: OwnedAnswer, `answerId`: OwnedAnswerId, `images`: Presence<OwnedAtomicBooleanImages> = .absent, `index`: Presence<OwnedAtomicBooleanIndex> = .absent, `input`: Presence<JSONValue> = .absent, `members`: Presence<OwnedAtomicBooleanMembers> = .absent, `meta`: OwnedMeta, `question`: OwnedReadableQuestion, `questionName`: Presence<OwnedAtomicBooleanQuestionName> = .absent, `schema`: OwnedVersion, `source`: Presence<OwnedPhysicalSource> = .absent, `threshold`: OwnedThreshold?, `value`: Bool) { self.extensions = extensions; self.`answer` = `answer`; self.`answerId` = `answerId`; self.`images` = `images`; self.`index` = `index`; self.`input` = `input`; self.`members` = `members`; self.`meta` = `meta`; self.`question` = `question`; self.`questionName` = `questionName`; self.`schema` = `schema`; self.`source` = `source`; self.`threshold` = `threshold`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedAtomicBoolean {
        let object = try jsonObject(json)
        let known: Set<String> = ["answer", "answer_id", "images", "index", "input", "members", "meta", "question", "question_name", "schema", "source", "threshold", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, answer: try OwnedAnswer.read(jsonRequired(object, "answer")), answerId: try OwnedAnswerId.read(jsonRequired(object, "answer_id")), images: try readPresence(object, "images", OwnedAtomicBooleanImages.self), index: try readPresence(object, "index", OwnedAtomicBooleanIndex.self), input: try readPresence(object, "input", JSONValue.self), members: try readPresence(object, "members", OwnedAtomicBooleanMembers.self), meta: try OwnedMeta.read(jsonRequired(object, "meta")), question: try OwnedReadableQuestion.read(jsonRequired(object, "question")), questionName: try readPresence(object, "question_name", OwnedAtomicBooleanQuestionName.self), schema: try OwnedVersion.read(jsonRequired(object, "schema")), source: try readPresence(object, "source", OwnedPhysicalSource.self), threshold: try OwnedThreshold?.read(jsonRequired(object, "threshold")), value: try Bool.read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["answer", "answer_id", "images", "index", "input", "members", "meta", "question", "question_name", "schema", "source", "threshold", "value"]
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
public indirect enum OwnedAtomicBooleanImages: JSONRepresentable {
    case alternative0([OwnedImage])
    public static func read(_ json: JSONValue) throws -> OwnedAtomicBooleanImages {
        if let value = try? [OwnedImage].read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedAtomicBooleanImages value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedAtomicBooleanIndex: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedAtomicBooleanIndex {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedAtomicBooleanIndex value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedAtomicBooleanMembers: JSONRepresentable {
    case alternative0([OwnedRankMember])
    public static func read(_ json: JSONValue) throws -> OwnedAtomicBooleanMembers {
        if let value = try? [OwnedRankMember].read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedAtomicBooleanMembers value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedAtomicBooleanQuestionName: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedAtomicBooleanQuestionName {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedAtomicBooleanQuestionName value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public struct OwnedAtomicDouble: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `answer`: OwnedAnswer
    public let `answerId`: OwnedAnswerId
    public let `images`: Presence<OwnedAtomicDoubleImages>
    public let `index`: Presence<OwnedAtomicDoubleIndex>
    public let `input`: Presence<JSONValue>
    public let `members`: Presence<OwnedAtomicDoubleMembers>
    public let `meta`: OwnedMeta
    public let `question`: OwnedReadableQuestion
    public let `questionName`: Presence<OwnedAtomicDoubleQuestionName>
    public let `schema`: OwnedVersion
    public let `source`: Presence<OwnedPhysicalSource>
    public let `threshold`: OwnedThreshold?
    public let `value`: Double
    public init(extensions: [String: JSONValue] = [:], `answer`: OwnedAnswer, `answerId`: OwnedAnswerId, `images`: Presence<OwnedAtomicDoubleImages> = .absent, `index`: Presence<OwnedAtomicDoubleIndex> = .absent, `input`: Presence<JSONValue> = .absent, `members`: Presence<OwnedAtomicDoubleMembers> = .absent, `meta`: OwnedMeta, `question`: OwnedReadableQuestion, `questionName`: Presence<OwnedAtomicDoubleQuestionName> = .absent, `schema`: OwnedVersion, `source`: Presence<OwnedPhysicalSource> = .absent, `threshold`: OwnedThreshold?, `value`: Double) { self.extensions = extensions; self.`answer` = `answer`; self.`answerId` = `answerId`; self.`images` = `images`; self.`index` = `index`; self.`input` = `input`; self.`members` = `members`; self.`meta` = `meta`; self.`question` = `question`; self.`questionName` = `questionName`; self.`schema` = `schema`; self.`source` = `source`; self.`threshold` = `threshold`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedAtomicDouble {
        let object = try jsonObject(json)
        let known: Set<String> = ["answer", "answer_id", "images", "index", "input", "members", "meta", "question", "question_name", "schema", "source", "threshold", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, answer: try OwnedAnswer.read(jsonRequired(object, "answer")), answerId: try OwnedAnswerId.read(jsonRequired(object, "answer_id")), images: try readPresence(object, "images", OwnedAtomicDoubleImages.self), index: try readPresence(object, "index", OwnedAtomicDoubleIndex.self), input: try readPresence(object, "input", JSONValue.self), members: try readPresence(object, "members", OwnedAtomicDoubleMembers.self), meta: try OwnedMeta.read(jsonRequired(object, "meta")), question: try OwnedReadableQuestion.read(jsonRequired(object, "question")), questionName: try readPresence(object, "question_name", OwnedAtomicDoubleQuestionName.self), schema: try OwnedVersion.read(jsonRequired(object, "schema")), source: try readPresence(object, "source", OwnedPhysicalSource.self), threshold: try OwnedThreshold?.read(jsonRequired(object, "threshold")), value: try Double.read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["answer", "answer_id", "images", "index", "input", "members", "meta", "question", "question_name", "schema", "source", "threshold", "value"]
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
public indirect enum OwnedAtomicDoubleImages: JSONRepresentable {
    case alternative0([OwnedImage])
    public static func read(_ json: JSONValue) throws -> OwnedAtomicDoubleImages {
        if let value = try? [OwnedImage].read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedAtomicDoubleImages value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedAtomicDoubleIndex: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedAtomicDoubleIndex {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedAtomicDoubleIndex value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedAtomicDoubleMembers: JSONRepresentable {
    case alternative0([OwnedRankMember])
    public static func read(_ json: JSONValue) throws -> OwnedAtomicDoubleMembers {
        if let value = try? [OwnedRankMember].read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedAtomicDoubleMembers value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedAtomicDoubleQuestionName: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedAtomicDoubleQuestionName {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedAtomicDoubleQuestionName value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public struct OwnedAttempt: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `ordinal`: UInt64
    public let `outcome`: OwnedAttemptOutcome
    public let `requestId`: Presence<OwnedAttemptRequestId>
    public let `requestSha256`: String
    public let `sdkRequestId`: OwnedSdkRequestId
    public let `serverMs`: Presence<OwnedAttemptServerMs>
    public let `status`: Presence<OwnedAttemptStatus>
    public let `wallMs`: UInt64
    public init(extensions: [String: JSONValue] = [:], `ordinal`: UInt64, `outcome`: OwnedAttemptOutcome, `requestId`: Presence<OwnedAttemptRequestId> = .absent, `requestSha256`: String, `sdkRequestId`: OwnedSdkRequestId, `serverMs`: Presence<OwnedAttemptServerMs> = .absent, `status`: Presence<OwnedAttemptStatus> = .absent, `wallMs`: UInt64) { self.extensions = extensions; self.`ordinal` = `ordinal`; self.`outcome` = `outcome`; self.`requestId` = `requestId`; self.`requestSha256` = `requestSha256`; self.`sdkRequestId` = `sdkRequestId`; self.`serverMs` = `serverMs`; self.`status` = `status`; self.`wallMs` = `wallMs` }
    public static func read(_ json: JSONValue) throws -> OwnedAttempt {
        let object = try jsonObject(json)
        let known: Set<String> = ["ordinal", "outcome", "request_id", "request_sha256", "sdk_request_id", "server_ms", "status", "wall_ms"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, ordinal: try UInt64.read(jsonRequired(object, "ordinal")), outcome: try OwnedAttemptOutcome.read(jsonRequired(object, "outcome")), requestId: try readPresence(object, "request_id", OwnedAttemptRequestId.self), requestSha256: try String.read(jsonRequired(object, "request_sha256")), sdkRequestId: try OwnedSdkRequestId.read(jsonRequired(object, "sdk_request_id")), serverMs: try readPresence(object, "server_ms", OwnedAttemptServerMs.self), status: try readPresence(object, "status", OwnedAttemptStatus.self), wallMs: try UInt64.read(jsonRequired(object, "wall_ms")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["ordinal", "outcome", "request_id", "request_sha256", "sdk_request_id", "server_ms", "status", "wall_ms"]
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
public indirect enum OwnedAttemptRequestId: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedAttemptRequestId {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedAttemptRequestId value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedAttemptServerMs: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedAttemptServerMs {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedAttemptServerMs value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedAttemptStatus: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedAttemptStatus {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedAttemptStatus value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedBatch: JSONRepresentable {
    case alternative0(UInt64)
    case alternative1(String)
    public static func read(_ json: JSONValue) throws -> OwnedBatch {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        if let value = try? String.read(json) { return .alternative1(value) }
        throw JSONConversionError("Unknown OwnedBatch value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public typealias OwnedBoundaryMode = String
public struct OwnedBoundaryOdds: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `pieces`: [OwnedPieceOdds]
    public let `proposals`: [OwnedBoundaryProposal]
    public init(extensions: [String: JSONValue] = [:], `pieces`: [OwnedPieceOdds], `proposals`: [OwnedBoundaryProposal]) { self.extensions = extensions; self.`pieces` = `pieces`; self.`proposals` = `proposals` }
    public static func read(_ json: JSONValue) throws -> OwnedBoundaryOdds {
        let object = try jsonObject(json)
        let known: Set<String> = ["pieces", "proposals"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, pieces: try [OwnedPieceOdds].read(jsonRequired(object, "pieces")), proposals: try [OwnedBoundaryProposal].read(jsonRequired(object, "proposals")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["pieces", "proposals"]
        var object = extensions.filter { !known.contains($0.key) }
        object["pieces"] = `pieces`.json
        object["proposals"] = `proposals`.json
        return .object(object)
    }
}
public struct OwnedBoundaryProposal: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `end`: UInt64
    public let `length`: UInt64
    public let `probability`: Double
    public let `start`: UInt64
    public let `text`: String
    public init(extensions: [String: JSONValue] = [:], `end`: UInt64, `length`: UInt64, `probability`: Double, `start`: UInt64, `text`: String) { self.extensions = extensions; self.`end` = `end`; self.`length` = `length`; self.`probability` = `probability`; self.`start` = `start`; self.`text` = `text` }
    public static func read(_ json: JSONValue) throws -> OwnedBoundaryProposal {
        let object = try jsonObject(json)
        let known: Set<String> = ["end", "length", "probability", "start", "text"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, end: try UInt64.read(jsonRequired(object, "end")), length: try UInt64.read(jsonRequired(object, "length")), probability: try Double.read(jsonRequired(object, "probability")), start: try UInt64.read(jsonRequired(object, "start")), text: try String.read(jsonRequired(object, "text")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public struct OwnedCallError: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `error`: OwnedError
    public let `facts`: Presence<OwnedFacts>
    public init(extensions: [String: JSONValue] = [:], `error`: OwnedError, `facts`: Presence<OwnedFacts> = .absent) { self.extensions = extensions; self.`error` = `error`; self.`facts` = `facts` }
    public static func read(_ json: JSONValue) throws -> OwnedCallError {
        let object = try jsonObject(json)
        let known: Set<String> = ["error", "facts"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, error: try OwnedError.read(jsonRequired(object, "error")), facts: try readPresence(object, "facts", OwnedFacts.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["error", "facts"]
        var object = extensions.filter { !known.contains($0.key) }
        object["error"] = `error`.json
        writePresence(`facts`, "facts", &object)
        return .object(object)
    }
}
public typealias OwnedCallId = String
public indirect enum OwnedDecideValue: JSONRepresentable {
    case alternative0(OwnedDecideValueAlternative0)
    case alternative1(JSONValue)
    public static func read(_ json: JSONValue) throws -> OwnedDecideValue {
        if let value = try? OwnedDecideValueAlternative0.read(json) { return .alternative0(value) }
        if let value = try? JSONValue.read(json) { return .alternative1(value) }
        throw JSONConversionError("Unknown OwnedDecideValue value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public indirect enum OwnedDecideValueAlternative0: JSONRepresentable {
    case alternative0(Bool)
    case alternative1(JSONValue)
    public static func read(_ json: JSONValue) throws -> OwnedDecideValueAlternative0 {
        if let value = try? Bool.read(json) { return .alternative0(value) }
        if json == .null { return .alternative1(.null) }
        throw JSONConversionError("Unknown OwnedDecideValueAlternative0 value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public struct OwnedEntityDocument: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `name`: String
    public init(extensions: [String: JSONValue] = [:], `kind`: String, `name`: String) { self.extensions = extensions; self.`kind` = `kind`; self.`name` = `name` }
    public static func read(_ json: JSONValue) throws -> OwnedEntityDocument {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "name"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), name: try String.read(jsonRequired(object, "name")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "name"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        object["name"] = `name`.json
        return .object(object)
    }
}
public struct OwnedError: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `estimatedInputDenial`: Presence<OwnedEstimatedInputDenial>
    public let `kind`: OwnedFailureKind
    public let `message`: String
    public let `retryable`: Bool
    public let `sendBudgetDenial`: Presence<OwnedSendBudgetDenial>
    public let `stopped`: OwnedStopped
    public init(extensions: [String: JSONValue] = [:], `estimatedInputDenial`: Presence<OwnedEstimatedInputDenial> = .absent, `kind`: OwnedFailureKind, `message`: String, `retryable`: Bool, `sendBudgetDenial`: Presence<OwnedSendBudgetDenial> = .absent, `stopped`: OwnedStopped) { self.extensions = extensions; self.`estimatedInputDenial` = `estimatedInputDenial`; self.`kind` = `kind`; self.`message` = `message`; self.`retryable` = `retryable`; self.`sendBudgetDenial` = `sendBudgetDenial`; self.`stopped` = `stopped` }
    public static func read(_ json: JSONValue) throws -> OwnedError {
        let object = try jsonObject(json)
        let known: Set<String> = ["estimated_input_denial", "kind", "message", "retryable", "send_budget_denial", "stopped"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, estimatedInputDenial: try readPresence(object, "estimated_input_denial", OwnedEstimatedInputDenial.self), kind: try OwnedFailureKind.read(jsonRequired(object, "kind")), message: try String.read(jsonRequired(object, "message")), retryable: try Bool.read(jsonRequired(object, "retryable")), sendBudgetDenial: try readPresence(object, "send_budget_denial", OwnedSendBudgetDenial.self), stopped: try OwnedStopped.read(jsonRequired(object, "stopped")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["estimated_input_denial", "kind", "message", "retryable", "send_budget_denial", "stopped"]
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
public indirect enum OwnedEstimatedInputDenial: JSONRepresentable {
    case `initialRequest`(OwnedEstimatedInputDenialInitialRequest)
    case `additionalRequest`(OwnedEstimatedInputDenialAdditionalRequest)
    case `retry`(OwnedEstimatedInputDenialRetry)
    public static func read(_ json: JSONValue) throws -> OwnedEstimatedInputDenial {
        let object = try jsonObject(json)
        if object["kind"] == .string("initial_request") { return .`initialRequest`(try OwnedEstimatedInputDenialInitialRequest.read(json)) }
        if object["kind"] == .string("additional_request") { return .`additionalRequest`(try OwnedEstimatedInputDenialAdditionalRequest.read(json)) }
        if object["kind"] == .string("retry") { return .`retry`(try OwnedEstimatedInputDenialRetry.read(json)) }
        throw JSONConversionError("Unknown OwnedEstimatedInputDenial alternative")
    }
    public var json: JSONValue {
        switch self {
        case .`initialRequest`(let value): return value.json
        case .`additionalRequest`(let value): return value.json
        case .`retry`(let value): return value.json
        }
    }
}
public struct OwnedEstimatedInputDenialAdditionalRequest: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `limit`: UInt64
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "additional_request", `limit`: UInt64) { self.extensions = extensions; self.`kind` = `kind`; self.`limit` = `limit` }
    public static func read(_ json: JSONValue) throws -> OwnedEstimatedInputDenialAdditionalRequest {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "limit"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), limit: try UInt64.read(jsonRequired(object, "limit")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "limit"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        object["limit"] = `limit`.json
        return .object(object)
    }
}
public struct OwnedEstimatedInputDenialInitialRequest: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `limit`: UInt64
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "initial_request", `limit`: UInt64) { self.extensions = extensions; self.`kind` = `kind`; self.`limit` = `limit` }
    public static func read(_ json: JSONValue) throws -> OwnedEstimatedInputDenialInitialRequest {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "limit"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), limit: try UInt64.read(jsonRequired(object, "limit")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "limit"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        object["limit"] = `limit`.json
        return .object(object)
    }
}
public struct OwnedEstimatedInputDenialRetry: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `lastStatus`: UInt64
    public let `limit`: UInt64
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "retry", `lastStatus`: UInt64, `limit`: UInt64) { self.extensions = extensions; self.`kind` = `kind`; self.`lastStatus` = `lastStatus`; self.`limit` = `limit` }
    public static func read(_ json: JSONValue) throws -> OwnedEstimatedInputDenialRetry {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "last_status", "limit"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), lastStatus: try UInt64.read(jsonRequired(object, "last_status")), limit: try UInt64.read(jsonRequired(object, "limit")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "last_status", "limit"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        object["last_status"] = `lastStatus`.json
        object["limit"] = `limit`.json
        return .object(object)
    }
}
public struct OwnedFacts: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `attempts`: Presence<OwnedFactsAttempts>
    public let `cacheAnswers`: UInt64
    public let `callId`: OwnedCallId
    public let `estimatedCostUsd`: Presence<OwnedFactsEstimatedCostUsd>
    public let `heldModelMismatch`: Presence<OwnedFactsHeldModelMismatch>
    public let `inputTokens`: Presence<OwnedFactsInputTokens>
    public let `largestRequestBytes`: UInt64
    public let `largestRequestEstimatedInputTokens`: OwnedFactsLargestRequestEstimatedInputTokens
    public let `model`: Presence<OwnedFactsModel>
    public let `outputTokens`: Presence<OwnedFactsOutputTokens>
    public let `records`: UInt64
    public let `requestsSent`: UInt64
    public let `seconds`: Double
    public let `tokenEstimateMethod`: String
    public let `usagePersistence`: Presence<OwnedPersistenceObservation>
    public init(extensions: [String: JSONValue] = [:], `attempts`: Presence<OwnedFactsAttempts> = .absent, `cacheAnswers`: UInt64, `callId`: OwnedCallId, `estimatedCostUsd`: Presence<OwnedFactsEstimatedCostUsd> = .absent, `heldModelMismatch`: Presence<OwnedFactsHeldModelMismatch> = .absent, `inputTokens`: Presence<OwnedFactsInputTokens> = .absent, `largestRequestBytes`: UInt64, `largestRequestEstimatedInputTokens`: OwnedFactsLargestRequestEstimatedInputTokens, `model`: Presence<OwnedFactsModel> = .absent, `outputTokens`: Presence<OwnedFactsOutputTokens> = .absent, `records`: UInt64, `requestsSent`: UInt64, `seconds`: Double, `tokenEstimateMethod`: String, `usagePersistence`: Presence<OwnedPersistenceObservation> = .absent) { self.extensions = extensions; self.`attempts` = `attempts`; self.`cacheAnswers` = `cacheAnswers`; self.`callId` = `callId`; self.`estimatedCostUsd` = `estimatedCostUsd`; self.`heldModelMismatch` = `heldModelMismatch`; self.`inputTokens` = `inputTokens`; self.`largestRequestBytes` = `largestRequestBytes`; self.`largestRequestEstimatedInputTokens` = `largestRequestEstimatedInputTokens`; self.`model` = `model`; self.`outputTokens` = `outputTokens`; self.`records` = `records`; self.`requestsSent` = `requestsSent`; self.`seconds` = `seconds`; self.`tokenEstimateMethod` = `tokenEstimateMethod`; self.`usagePersistence` = `usagePersistence` }
    public static func read(_ json: JSONValue) throws -> OwnedFacts {
        let object = try jsonObject(json)
        let known: Set<String> = ["attempts", "cache_answers", "call_id", "estimated_cost_usd", "held_model_mismatch", "input_tokens", "largest_request_bytes", "largest_request_estimated_input_tokens", "model", "output_tokens", "records", "requests_sent", "seconds", "token_estimate_method", "usage_persistence"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, attempts: try readPresence(object, "attempts", OwnedFactsAttempts.self), cacheAnswers: try UInt64.read(jsonRequired(object, "cache_answers")), callId: try OwnedCallId.read(jsonRequired(object, "call_id")), estimatedCostUsd: try readPresence(object, "estimated_cost_usd", OwnedFactsEstimatedCostUsd.self), heldModelMismatch: try readPresence(object, "held_model_mismatch", OwnedFactsHeldModelMismatch.self), inputTokens: try readPresence(object, "input_tokens", OwnedFactsInputTokens.self), largestRequestBytes: try UInt64.read(jsonRequired(object, "largest_request_bytes")), largestRequestEstimatedInputTokens: try OwnedFactsLargestRequestEstimatedInputTokens.read(jsonRequired(object, "largest_request_estimated_input_tokens")), model: try readPresence(object, "model", OwnedFactsModel.self), outputTokens: try readPresence(object, "output_tokens", OwnedFactsOutputTokens.self), records: try UInt64.read(jsonRequired(object, "records")), requestsSent: try UInt64.read(jsonRequired(object, "requests_sent")), seconds: try Double.read(jsonRequired(object, "seconds")), tokenEstimateMethod: try String.read(jsonRequired(object, "token_estimate_method")), usagePersistence: try readPresence(object, "usage_persistence", OwnedPersistenceObservation.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["attempts", "cache_answers", "call_id", "estimated_cost_usd", "held_model_mismatch", "input_tokens", "largest_request_bytes", "largest_request_estimated_input_tokens", "model", "output_tokens", "records", "requests_sent", "seconds", "token_estimate_method", "usage_persistence"]
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
public indirect enum OwnedFactsAttempts: JSONRepresentable {
    case alternative0([OwnedAttempt])
    public static func read(_ json: JSONValue) throws -> OwnedFactsAttempts {
        if let value = try? [OwnedAttempt].read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedFactsAttempts value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedFactsEstimatedCostUsd: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedFactsEstimatedCostUsd {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedFactsEstimatedCostUsd value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedFactsHeldModelMismatch: JSONRepresentable {
    case alternative0(Bool)
    public static func read(_ json: JSONValue) throws -> OwnedFactsHeldModelMismatch {
        if let value = try? Bool.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedFactsHeldModelMismatch value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedFactsInputTokens: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedFactsInputTokens {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedFactsInputTokens value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedFactsLargestRequestEstimatedInputTokens: JSONRepresentable {
    case alternative0(UInt64)
    case alternative1(JSONValue)
    public static func read(_ json: JSONValue) throws -> OwnedFactsLargestRequestEstimatedInputTokens {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        if json == .null { return .alternative1(.null) }
        throw JSONConversionError("Unknown OwnedFactsLargestRequestEstimatedInputTokens value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public indirect enum OwnedFactsModel: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedFactsModel {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedFactsModel value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedFactsOutputTokens: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedFactsOutputTokens {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedFactsOutputTokens value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public typealias OwnedFailureId = String
public struct OwnedFind: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `answer`: OwnedFindAnswer
    public let `answerId`: OwnedAnswerId
    public let `candidates`: Presence<OwnedFindCandidates>
    public let `file`: Presence<OwnedFindFile>
    public let `firstLine`: Presence<OwnedFindFirstLine>
    public let `index`: OwnedFindIndex
    public let `lastLine`: Presence<OwnedFindLastLine>
    public let `meta`: OwnedMeta
    public let `position`: Presence<OwnedPosition>
    public let `question`: OwnedReadableQuestion2
    public let `schema`: OwnedVersion
    public let `threshold`: JSONValue
    public let `value`: JSONValue
    public init(extensions: [String: JSONValue] = [:], `answer`: OwnedFindAnswer, `answerId`: OwnedAnswerId, `candidates`: Presence<OwnedFindCandidates> = .absent, `file`: Presence<OwnedFindFile> = .absent, `firstLine`: Presence<OwnedFindFirstLine> = .absent, `index`: OwnedFindIndex, `lastLine`: Presence<OwnedFindLastLine> = .absent, `meta`: OwnedMeta, `position`: Presence<OwnedPosition> = .absent, `question`: OwnedReadableQuestion2, `schema`: OwnedVersion, `threshold`: JSONValue, `value`: JSONValue) { self.extensions = extensions; self.`answer` = `answer`; self.`answerId` = `answerId`; self.`candidates` = `candidates`; self.`file` = `file`; self.`firstLine` = `firstLine`; self.`index` = `index`; self.`lastLine` = `lastLine`; self.`meta` = `meta`; self.`position` = `position`; self.`question` = `question`; self.`schema` = `schema`; self.`threshold` = `threshold`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedFind {
        let object = try jsonObject(json)
        let known: Set<String> = ["answer", "answer_id", "candidates", "file", "first_line", "index", "last_line", "meta", "position", "question", "schema", "threshold", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, answer: try OwnedFindAnswer.read(jsonRequired(object, "answer")), answerId: try OwnedAnswerId.read(jsonRequired(object, "answer_id")), candidates: try readPresence(object, "candidates", OwnedFindCandidates.self), file: try readPresence(object, "file", OwnedFindFile.self), firstLine: try readPresence(object, "first_line", OwnedFindFirstLine.self), index: try OwnedFindIndex.read(jsonRequired(object, "index")), lastLine: try readPresence(object, "last_line", OwnedFindLastLine.self), meta: try OwnedMeta.read(jsonRequired(object, "meta")), position: try readPresence(object, "position", OwnedPosition.self), question: try OwnedReadableQuestion2.read(jsonRequired(object, "question")), schema: try OwnedVersion.read(jsonRequired(object, "schema")), threshold: try JSONValue.read(jsonRequired(object, "threshold")), value: try JSONValue.read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["answer", "answer_id", "candidates", "file", "first_line", "index", "last_line", "meta", "position", "question", "schema", "threshold", "value"]
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
public struct OwnedFindCandidate: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `index`: OwnedFindCandidateIndex
    public let `input`: JSONValue
    public let `probability`: Double
    public let `source`: Presence<OwnedPhysicalSource>
    public init(extensions: [String: JSONValue] = [:], `index`: OwnedFindCandidateIndex, `input`: JSONValue, `probability`: Double, `source`: Presence<OwnedPhysicalSource> = .absent) { self.extensions = extensions; self.`index` = `index`; self.`input` = `input`; self.`probability` = `probability`; self.`source` = `source` }
    public static func read(_ json: JSONValue) throws -> OwnedFindCandidate {
        let object = try jsonObject(json)
        let known: Set<String> = ["index", "input", "probability", "source"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, index: try OwnedFindCandidateIndex.read(jsonRequired(object, "index")), input: try JSONValue.read(jsonRequired(object, "input")), probability: try Double.read(jsonRequired(object, "probability")), source: try readPresence(object, "source", OwnedPhysicalSource.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public indirect enum OwnedFindCandidateIndex: JSONRepresentable {
    case alternative0(UInt64)
    case alternative1(JSONValue)
    public static func read(_ json: JSONValue) throws -> OwnedFindCandidateIndex {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        if json == .null { return .alternative1(.null) }
        throw JSONConversionError("Unknown OwnedFindCandidateIndex value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public indirect enum OwnedFindCandidates: JSONRepresentable {
    case alternative0([OwnedFindCandidate])
    public static func read(_ json: JSONValue) throws -> OwnedFindCandidates {
        if let value = try? [OwnedFindCandidate].read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedFindCandidates value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedFindFile: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedFindFile {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedFindFile value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedFindFirstLine: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedFindFirstLine {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedFindFirstLine value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedFindIndex: JSONRepresentable {
    case alternative0(UInt64)
    case alternative1(JSONValue)
    public static func read(_ json: JSONValue) throws -> OwnedFindIndex {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        if json == .null { return .alternative1(.null) }
        throw JSONConversionError("Unknown OwnedFindIndex value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public indirect enum OwnedFindLastLine: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedFindLastLine {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedFindLastLine value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public struct OwnedImage: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `base64`: String
    public let `height`: UInt64
    public let `media`: OwnedImageMedia
    public let `width`: UInt64
    public init(extensions: [String: JSONValue] = [:], `base64`: String, `height`: UInt64, `media`: OwnedImageMedia, `width`: UInt64) { self.extensions = extensions; self.`base64` = `base64`; self.`height` = `height`; self.`media` = `media`; self.`width` = `width` }
    public static func read(_ json: JSONValue) throws -> OwnedImage {
        let object = try jsonObject(json)
        let known: Set<String> = ["base64", "height", "media", "width"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, base64: try String.read(jsonRequired(object, "base64")), height: try UInt64.read(jsonRequired(object, "height")), media: try OwnedImageMedia.read(jsonRequired(object, "media")), width: try UInt64.read(jsonRequired(object, "width")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public indirect enum OwnedImageMedia: JSONRepresentable {
    case alternative0(String)
    case alternative1(String)
    public static func read(_ json: JSONValue) throws -> OwnedImageMedia {
        if let value = try? String.read(json) { return .alternative0(value) }
        if let value = try? String.read(json) { return .alternative1(value) }
        throw JSONConversionError("Unknown OwnedImageMedia value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public indirect enum OwnedInputDeclaration: JSONRepresentable {
    case `string`(OwnedInputDeclarationString)
    case `object`(OwnedInputDeclarationObject)
    public static func read(_ json: JSONValue) throws -> OwnedInputDeclaration {
        let object = try jsonObject(json)
        if object["type"] == .string("string") { return .`string`(try OwnedInputDeclarationString.read(json)) }
        if object["type"] == .string("object") { return .`object`(try OwnedInputDeclarationObject.read(json)) }
        throw JSONConversionError("Unknown OwnedInputDeclaration alternative")
    }
    public var json: JSONValue {
        switch self {
        case .`string`(let value): return value.json
        case .`object`(let value): return value.json
        }
    }
}
public struct OwnedInputDeclarationObject: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `properties`: [String: OwnedInputPropertyType]
    public let `required`: Presence<OwnedInputDeclarationObjectRequired>
    public let `type`: OwnedObjectType
    public init(extensions: [String: JSONValue] = [:], `properties`: [String: OwnedInputPropertyType], `required`: Presence<OwnedInputDeclarationObjectRequired> = .absent, `type`: OwnedObjectType) { self.extensions = extensions; self.`properties` = `properties`; self.`required` = `required`; self.`type` = `type` }
    public static func read(_ json: JSONValue) throws -> OwnedInputDeclarationObject {
        let object = try jsonObject(json)
        let known: Set<String> = ["properties", "required", "type"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, properties: try [String: OwnedInputPropertyType].read(jsonRequired(object, "properties")), required: try readPresence(object, "required", OwnedInputDeclarationObjectRequired.self), type: try OwnedObjectType.read(jsonRequired(object, "type")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["properties", "required", "type"]
        var object = extensions.filter { !known.contains($0.key) }
        object["properties"] = `properties`.json
        writePresence(`required`, "required", &object)
        object["type"] = `type`.json
        return .object(object)
    }
}
public indirect enum OwnedInputDeclarationObjectRequired: JSONRepresentable {
    case alternative0([String])
    public static func read(_ json: JSONValue) throws -> OwnedInputDeclarationObjectRequired {
        if let value = try? [String].read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedInputDeclarationObjectRequired value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public struct OwnedInputDeclarationString: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `type`: OwnedStringType
    public init(extensions: [String: JSONValue] = [:], `type`: OwnedStringType) { self.extensions = extensions; self.`type` = `type` }
    public static func read(_ json: JSONValue) throws -> OwnedInputDeclarationString {
        let object = try jsonObject(json)
        let known: Set<String> = ["type"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, type: try OwnedStringType.read(jsonRequired(object, "type")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["type"]
        var object = extensions.filter { !known.contains($0.key) }
        object["type"] = `type`.json
        return .object(object)
    }
}
public indirect enum OwnedInputPropertyType: JSONRepresentable {
    case `string`(OwnedInputPropertyTypeString)
    case `number`(OwnedInputPropertyTypeNumber)
    case `boolean`(OwnedInputPropertyTypeBoolean)
    case `array`(OwnedInputPropertyTypeArray)
    public static func read(_ json: JSONValue) throws -> OwnedInputPropertyType {
        let object = try jsonObject(json)
        if object["type"] == .string("string") { return .`string`(try OwnedInputPropertyTypeString.read(json)) }
        if object["type"] == .string("number") { return .`number`(try OwnedInputPropertyTypeNumber.read(json)) }
        if object["type"] == .string("boolean") { return .`boolean`(try OwnedInputPropertyTypeBoolean.read(json)) }
        if object["type"] == .string("array") { return .`array`(try OwnedInputPropertyTypeArray.read(json)) }
        throw JSONConversionError("Unknown OwnedInputPropertyType alternative")
    }
    public var json: JSONValue {
        switch self {
        case .`string`(let value): return value.json
        case .`number`(let value): return value.json
        case .`boolean`(let value): return value.json
        case .`array`(let value): return value.json
        }
    }
}
public struct OwnedInputPropertyTypeArray: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `items`: OwnedStringRoot
    public let `type`: String
    public init(extensions: [String: JSONValue] = [:], `items`: OwnedStringRoot, `type`: String = "array") { self.extensions = extensions; self.`items` = `items`; self.`type` = `type` }
    public static func read(_ json: JSONValue) throws -> OwnedInputPropertyTypeArray {
        let object = try jsonObject(json)
        let known: Set<String> = ["items", "type"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, items: try OwnedStringRoot.read(jsonRequired(object, "items")), type: try String.read(jsonRequired(object, "type")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["items", "type"]
        var object = extensions.filter { !known.contains($0.key) }
        object["items"] = `items`.json
        object["type"] = `type`.json
        return .object(object)
    }
}
public struct OwnedInputPropertyTypeBoolean: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `type`: String
    public init(extensions: [String: JSONValue] = [:], `type`: String = "boolean") { self.extensions = extensions; self.`type` = `type` }
    public static func read(_ json: JSONValue) throws -> OwnedInputPropertyTypeBoolean {
        let object = try jsonObject(json)
        let known: Set<String> = ["type"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, type: try String.read(jsonRequired(object, "type")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["type"]
        var object = extensions.filter { !known.contains($0.key) }
        object["type"] = `type`.json
        return .object(object)
    }
}
public struct OwnedInputPropertyTypeNumber: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `type`: String
    public init(extensions: [String: JSONValue] = [:], `type`: String = "number") { self.extensions = extensions; self.`type` = `type` }
    public static func read(_ json: JSONValue) throws -> OwnedInputPropertyTypeNumber {
        let object = try jsonObject(json)
        let known: Set<String> = ["type"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, type: try String.read(jsonRequired(object, "type")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["type"]
        var object = extensions.filter { !known.contains($0.key) }
        object["type"] = `type`.json
        return .object(object)
    }
}
public struct OwnedInputPropertyTypeString: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `type`: String
    public init(extensions: [String: JSONValue] = [:], `type`: String = "string") { self.extensions = extensions; self.`type` = `type` }
    public static func read(_ json: JSONValue) throws -> OwnedInputPropertyTypeString {
        let object = try jsonObject(json)
        let known: Set<String> = ["type"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, type: try String.read(jsonRequired(object, "type")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["type"]
        var object = extensions.filter { !known.contains($0.key) }
        object["type"] = `type`.json
        return .object(object)
    }
}
public struct OwnedLabel: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `description`: Presence<JSONValue>
    public let `name`: String
    public init(extensions: [String: JSONValue] = [:], `description`: Presence<JSONValue> = .absent, `name`: String) { self.extensions = extensions; self.`description` = `description`; self.`name` = `name` }
    public static func read(_ json: JSONValue) throws -> OwnedLabel {
        let object = try jsonObject(json)
        let known: Set<String> = ["description", "name"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, description: try readPresence(object, "description", JSONValue.self), name: try String.read(jsonRequired(object, "name")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["description", "name"]
        var object = extensions.filter { !known.contains($0.key) }
        writePresence(`description`, "description", &object)
        object["name"] = `name`.json
        return .object(object)
    }
}
public struct OwnedMeta: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `answeredBy`: Presence<OwnedMetaAnsweredBy>
    public let `attempts`: Presence<OwnedMetaAttempts>
    public let `batchSetting`: Presence<OwnedBatchSetting>
    public let `batchWarning`: Presence<OwnedBatchWarning>
    public let `cached`: Bool
    public let `contextSha256`: Presence<OwnedMetaContextSha256>
    public let `failedQuestions`: UInt64
    public let `model`: String
    public let `observations`: [OwnedObservation]
    public let `origin`: OwnedOrigin?
    public let `profileWarning`: Presence<OwnedProfileWarning>
    public let `questionSha256`: Presence<OwnedMetaQuestionSha256>
    public let `questionSources`: [OwnedQuestionSource]
    public let `questionsSha256`: Presence<OwnedMetaQuestionsSha256>
    public let `requests`: [String]
    public let `requestsSent`: UInt64
    public let `tool`: String
    public let `url`: String
    public let `usage`: Presence<OwnedUsage>
    public init(extensions: [String: JSONValue] = [:], `answeredBy`: Presence<OwnedMetaAnsweredBy> = .absent, `attempts`: Presence<OwnedMetaAttempts> = .absent, `batchSetting`: Presence<OwnedBatchSetting> = .absent, `batchWarning`: Presence<OwnedBatchWarning> = .absent, `cached`: Bool, `contextSha256`: Presence<OwnedMetaContextSha256> = .absent, `failedQuestions`: UInt64, `model`: String, `observations`: [OwnedObservation], `origin`: OwnedOrigin?, `profileWarning`: Presence<OwnedProfileWarning> = .absent, `questionSha256`: Presence<OwnedMetaQuestionSha256> = .absent, `questionSources`: [OwnedQuestionSource], `questionsSha256`: Presence<OwnedMetaQuestionsSha256> = .absent, `requests`: [String], `requestsSent`: UInt64, `tool`: String, `url`: String, `usage`: Presence<OwnedUsage> = .absent) { self.extensions = extensions; self.`answeredBy` = `answeredBy`; self.`attempts` = `attempts`; self.`batchSetting` = `batchSetting`; self.`batchWarning` = `batchWarning`; self.`cached` = `cached`; self.`contextSha256` = `contextSha256`; self.`failedQuestions` = `failedQuestions`; self.`model` = `model`; self.`observations` = `observations`; self.`origin` = `origin`; self.`profileWarning` = `profileWarning`; self.`questionSha256` = `questionSha256`; self.`questionSources` = `questionSources`; self.`questionsSha256` = `questionsSha256`; self.`requests` = `requests`; self.`requestsSent` = `requestsSent`; self.`tool` = `tool`; self.`url` = `url`; self.`usage` = `usage` }
    public static func read(_ json: JSONValue) throws -> OwnedMeta {
        let object = try jsonObject(json)
        let known: Set<String> = ["answered_by", "attempts", "batch_setting", "batch_warning", "cached", "context_sha256", "failed_questions", "model", "observations", "origin", "profile_warning", "question_sha256", "question_sources", "questions_sha256", "requests", "requests_sent", "tool", "url", "usage"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, answeredBy: try readPresence(object, "answered_by", OwnedMetaAnsweredBy.self), attempts: try readPresence(object, "attempts", OwnedMetaAttempts.self), batchSetting: try readPresence(object, "batch_setting", OwnedBatchSetting.self), batchWarning: try readPresence(object, "batch_warning", OwnedBatchWarning.self), cached: try Bool.read(jsonRequired(object, "cached")), contextSha256: try readPresence(object, "context_sha256", OwnedMetaContextSha256.self), failedQuestions: try UInt64.read(jsonRequired(object, "failed_questions")), model: try String.read(jsonRequired(object, "model")), observations: try [OwnedObservation].read(jsonRequired(object, "observations")), origin: try OwnedOrigin?.read(jsonRequired(object, "origin")), profileWarning: try readPresence(object, "profile_warning", OwnedProfileWarning.self), questionSha256: try readPresence(object, "question_sha256", OwnedMetaQuestionSha256.self), questionSources: try [OwnedQuestionSource].read(jsonRequired(object, "question_sources")), questionsSha256: try readPresence(object, "questions_sha256", OwnedMetaQuestionsSha256.self), requests: try [String].read(jsonRequired(object, "requests")), requestsSent: try UInt64.read(jsonRequired(object, "requests_sent")), tool: try String.read(jsonRequired(object, "tool")), url: try String.read(jsonRequired(object, "url")), usage: try readPresence(object, "usage", OwnedUsage.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["answered_by", "attempts", "batch_setting", "batch_warning", "cached", "context_sha256", "failed_questions", "model", "observations", "origin", "profile_warning", "question_sha256", "question_sources", "questions_sha256", "requests", "requests_sent", "tool", "url", "usage"]
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
public indirect enum OwnedMetaAnsweredBy: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedMetaAnsweredBy {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedMetaAnsweredBy value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedMetaAttempts: JSONRepresentable {
    case alternative0([OwnedAttempt])
    public static func read(_ json: JSONValue) throws -> OwnedMetaAttempts {
        if let value = try? [OwnedAttempt].read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedMetaAttempts value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedMetaContextSha256: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedMetaContextSha256 {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedMetaContextSha256 value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedMetaQuestionSha256: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedMetaQuestionSha256 {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedMetaQuestionSha256 value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedMetaQuestionsSha256: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedMetaQuestionsSha256 {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedMetaQuestionsSha256 value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public struct OwnedObjectRoot: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `properties`: [String: OwnedInputPropertyType]
    public let `required`: Presence<OwnedObjectRootRequired>
    public let `type`: OwnedObjectType
    public init(extensions: [String: JSONValue] = [:], `properties`: [String: OwnedInputPropertyType], `required`: Presence<OwnedObjectRootRequired> = .absent, `type`: OwnedObjectType) { self.extensions = extensions; self.`properties` = `properties`; self.`required` = `required`; self.`type` = `type` }
    public static func read(_ json: JSONValue) throws -> OwnedObjectRoot {
        let object = try jsonObject(json)
        let known: Set<String> = ["properties", "required", "type"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, properties: try [String: OwnedInputPropertyType].read(jsonRequired(object, "properties")), required: try readPresence(object, "required", OwnedObjectRootRequired.self), type: try OwnedObjectType.read(jsonRequired(object, "type")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["properties", "required", "type"]
        var object = extensions.filter { !known.contains($0.key) }
        object["properties"] = `properties`.json
        writePresence(`required`, "required", &object)
        object["type"] = `type`.json
        return .object(object)
    }
}
public indirect enum OwnedObjectRootRequired: JSONRepresentable {
    case alternative0([String])
    public static func read(_ json: JSONValue) throws -> OwnedObjectRootRequired {
        if let value = try? [String].read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedObjectRootRequired value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public typealias OwnedObjectType = String
public indirect enum OwnedObservation: JSONRepresentable {
    case `observationId`(OwnedObservationObservationId)
    case `failureId`(OwnedObservationFailureId)
    public static func read(_ json: JSONValue) throws -> OwnedObservation {
        let object = try jsonObject(json)
        if object["observation_id"] != nil && object["observation_id"] != .null { return .`observationId`(try OwnedObservationObservationId.read(json)) }
        if object["failure_id"] != nil && object["failure_id"] != .null { return .`failureId`(try OwnedObservationFailureId.read(json)) }
        throw JSONConversionError("Unknown OwnedObservation alternative")
    }
    public var json: JSONValue {
        switch self {
        case .`observationId`(let value): return value.json
        case .`failureId`(let value): return value.json
        }
    }
}
public typealias OwnedObservationId = String
public struct OwnedObservationFailureId: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `failureId`: OwnedFailureId
    public init(extensions: [String: JSONValue] = [:], `failureId`: OwnedFailureId) { self.extensions = extensions; self.`failureId` = `failureId` }
    public static func read(_ json: JSONValue) throws -> OwnedObservationFailureId {
        let object = try jsonObject(json)
        let known: Set<String> = ["failure_id"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, failureId: try OwnedFailureId.read(jsonRequired(object, "failure_id")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["failure_id"]
        var object = extensions.filter { !known.contains($0.key) }
        object["failure_id"] = `failureId`.json
        return .object(object)
    }
}
public struct OwnedObservationObservationId: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `observationId`: OwnedObservationId
    public init(extensions: [String: JSONValue] = [:], `observationId`: OwnedObservationId) { self.extensions = extensions; self.`observationId` = `observationId` }
    public static func read(_ json: JSONValue) throws -> OwnedObservationObservationId {
        let object = try jsonObject(json)
        let known: Set<String> = ["observation_id"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, observationId: try OwnedObservationId.read(jsonRequired(object, "observation_id")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["observation_id"]
        var object = extensions.filter { !known.contains($0.key) }
        object["observation_id"] = `observationId`.json
        return .object(object)
    }
}
public indirect enum OwnedOrigin: JSONRepresentable {
    case alternative0(String)
    case alternative1(String)
    case alternative2(String)
    case alternative3(String)
    case alternative4(String)
    public static func read(_ json: JSONValue) throws -> OwnedOrigin {
        if let value = try? String.read(json) { return .alternative0(value) }
        if let value = try? String.read(json) { return .alternative1(value) }
        if let value = try? String.read(json) { return .alternative2(value) }
        if let value = try? String.read(json) { return .alternative3(value) }
        if let value = try? String.read(json) { return .alternative4(value) }
        throw JSONConversionError("Unknown OwnedOrigin value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        case .alternative2(let value): return value.json
        case .alternative3(let value): return value.json
        case .alternative4(let value): return value.json
        }
    }
}
public struct OwnedPersistenceObservation: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `advice`: Presence<OwnedPersistenceObservationAdvice>
    public let `observedAt`: String
    public let `state`: OwnedUsagePersistence
    public init(extensions: [String: JSONValue] = [:], `advice`: Presence<OwnedPersistenceObservationAdvice> = .absent, `observedAt`: String, `state`: OwnedUsagePersistence) { self.extensions = extensions; self.`advice` = `advice`; self.`observedAt` = `observedAt`; self.`state` = `state` }
    public static func read(_ json: JSONValue) throws -> OwnedPersistenceObservation {
        let object = try jsonObject(json)
        let known: Set<String> = ["advice", "observed_at", "state"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, advice: try readPresence(object, "advice", OwnedPersistenceObservationAdvice.self), observedAt: try String.read(jsonRequired(object, "observed_at")), state: try OwnedUsagePersistence.read(jsonRequired(object, "state")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["advice", "observed_at", "state"]
        var object = extensions.filter { !known.contains($0.key) }
        writePresence(`advice`, "advice", &object)
        object["observed_at"] = `observedAt`.json
        object["state"] = `state`.json
        return .object(object)
    }
}
public indirect enum OwnedPersistenceObservationAdvice: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedPersistenceObservationAdvice {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedPersistenceObservationAdvice value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public struct OwnedPhysicalSource: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `file`: String
    public let `firstLine`: Presence<OwnedPhysicalSourceFirstLine>
    public let `lastLine`: Presence<OwnedPhysicalSourceLastLine>
    public init(extensions: [String: JSONValue] = [:], `file`: String, `firstLine`: Presence<OwnedPhysicalSourceFirstLine> = .absent, `lastLine`: Presence<OwnedPhysicalSourceLastLine> = .absent) { self.extensions = extensions; self.`file` = `file`; self.`firstLine` = `firstLine`; self.`lastLine` = `lastLine` }
    public static func read(_ json: JSONValue) throws -> OwnedPhysicalSource {
        let object = try jsonObject(json)
        let known: Set<String> = ["file", "first_line", "last_line"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, file: try String.read(jsonRequired(object, "file")), firstLine: try readPresence(object, "first_line", OwnedPhysicalSourceFirstLine.self), lastLine: try readPresence(object, "last_line", OwnedPhysicalSourceLastLine.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["file", "first_line", "last_line"]
        var object = extensions.filter { !known.contains($0.key) }
        object["file"] = `file`.json
        writePresence(`firstLine`, "first_line", &object)
        writePresence(`lastLine`, "last_line", &object)
        return .object(object)
    }
}
public indirect enum OwnedPhysicalSourceFirstLine: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedPhysicalSourceFirstLine {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedPhysicalSourceFirstLine value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedPhysicalSourceLastLine: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedPhysicalSourceLastLine {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedPhysicalSourceLastLine value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public struct OwnedPosition: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `file`: OwnedPositionFile
    public let `first`: Presence<OwnedPositionFirst>
    public let `images`: Presence<OwnedPositionImages>
    public let `last`: Presence<OwnedPositionLast>
    public init(extensions: [String: JSONValue] = [:], `file`: OwnedPositionFile, `first`: Presence<OwnedPositionFirst> = .absent, `images`: Presence<OwnedPositionImages> = .absent, `last`: Presence<OwnedPositionLast> = .absent) { self.extensions = extensions; self.`file` = `file`; self.`first` = `first`; self.`images` = `images`; self.`last` = `last` }
    public static func read(_ json: JSONValue) throws -> OwnedPosition {
        let object = try jsonObject(json)
        let known: Set<String> = ["file", "first", "images", "last"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, file: try OwnedPositionFile.read(jsonRequired(object, "file")), first: try readPresence(object, "first", OwnedPositionFirst.self), images: try readPresence(object, "images", OwnedPositionImages.self), last: try readPresence(object, "last", OwnedPositionLast.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public indirect enum OwnedPositionFile: JSONRepresentable {
    case alternative0(String)
    case alternative1(JSONValue)
    public static func read(_ json: JSONValue) throws -> OwnedPositionFile {
        if let value = try? String.read(json) { return .alternative0(value) }
        if json == .null { return .alternative1(.null) }
        throw JSONConversionError("Unknown OwnedPositionFile value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public indirect enum OwnedPositionFirst: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedPositionFirst {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedPositionFirst value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedPositionImages: JSONRepresentable {
    case alternative0([String])
    public static func read(_ json: JSONValue) throws -> OwnedPositionImages {
        if let value = try? [String].read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedPositionImages value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedPositionLast: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedPositionLast {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedPositionLast value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public typealias OwnedQuestionName = String
public struct OwnedQuestionSource: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `answeredBy`: String
    public let `batchSize`: Presence<OwnedQuestionSourceBatchSize>
    public let `origin`: OwnedOrigin
    public init(extensions: [String: JSONValue] = [:], `answeredBy`: String, `batchSize`: Presence<OwnedQuestionSourceBatchSize> = .absent, `origin`: OwnedOrigin) { self.extensions = extensions; self.`answeredBy` = `answeredBy`; self.`batchSize` = `batchSize`; self.`origin` = `origin` }
    public static func read(_ json: JSONValue) throws -> OwnedQuestionSource {
        let object = try jsonObject(json)
        let known: Set<String> = ["answered_by", "batch_size", "origin"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, answeredBy: try String.read(jsonRequired(object, "answered_by")), batchSize: try readPresence(object, "batch_size", OwnedQuestionSourceBatchSize.self), origin: try OwnedOrigin.read(jsonRequired(object, "origin")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["answered_by", "batch_size", "origin"]
        var object = extensions.filter { !known.contains($0.key) }
        object["answered_by"] = `answeredBy`.json
        writePresence(`batchSize`, "batch_size", &object)
        object["origin"] = `origin`.json
        return .object(object)
    }
}
public indirect enum OwnedQuestionSourceBatchSize: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedQuestionSourceBatchSize {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedQuestionSourceBatchSize value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public struct OwnedRankMember: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `name`: String
    public let `result`: OwnedRankMemberResult
    public init(extensions: [String: JSONValue] = [:], `name`: String, `result`: OwnedRankMemberResult) { self.extensions = extensions; self.`name` = `name`; self.`result` = `result` }
    public static func read(_ json: JSONValue) throws -> OwnedRankMember {
        let object = try jsonObject(json)
        let known: Set<String> = ["name", "result"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, name: try String.read(jsonRequired(object, "name")), result: try OwnedRankMemberResult.read(jsonRequired(object, "result")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["name", "result"]
        var object = extensions.filter { !known.contains($0.key) }
        object["name"] = `name`.json
        object["result"] = `result`.json
        return .object(object)
    }
}
public struct OwnedRankMemberResult: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `answer`: OwnedAnswer
    public let `answerId`: OwnedAnswerId
    public let `images`: Presence<OwnedRankMemberResultImages>
    public let `meta`: OwnedMeta
    public let `question`: OwnedReadableQuestion
    public let `schema`: OwnedVersion
    public let `source`: Presence<OwnedPhysicalSource>
    public let `threshold`: JSONValue
    public let `value`: UInt64
    public init(extensions: [String: JSONValue] = [:], `answer`: OwnedAnswer, `answerId`: OwnedAnswerId, `images`: Presence<OwnedRankMemberResultImages> = .absent, `meta`: OwnedMeta, `question`: OwnedReadableQuestion, `schema`: OwnedVersion, `source`: Presence<OwnedPhysicalSource> = .absent, `threshold`: JSONValue, `value`: UInt64) { self.extensions = extensions; self.`answer` = `answer`; self.`answerId` = `answerId`; self.`images` = `images`; self.`meta` = `meta`; self.`question` = `question`; self.`schema` = `schema`; self.`source` = `source`; self.`threshold` = `threshold`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedRankMemberResult {
        let object = try jsonObject(json)
        let known: Set<String> = ["answer", "answer_id", "images", "meta", "question", "schema", "source", "threshold", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, answer: try OwnedAnswer.read(jsonRequired(object, "answer")), answerId: try OwnedAnswerId.read(jsonRequired(object, "answer_id")), images: try readPresence(object, "images", OwnedRankMemberResultImages.self), meta: try OwnedMeta.read(jsonRequired(object, "meta")), question: try OwnedReadableQuestion.read(jsonRequired(object, "question")), schema: try OwnedVersion.read(jsonRequired(object, "schema")), source: try readPresence(object, "source", OwnedPhysicalSource.self), threshold: try JSONValue.read(jsonRequired(object, "threshold")), value: try UInt64.read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["answer", "answer_id", "images", "meta", "question", "schema", "source", "threshold", "value"]
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
public indirect enum OwnedRankMemberResultImages: JSONRepresentable {
    case alternative0([OwnedImage])
    public static func read(_ json: JSONValue) throws -> OwnedRankMemberResultImages {
        if let value = try? [OwnedImage].read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedRankMemberResultImages value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedReadableQuestion: JSONRepresentable {
    case `decide`(OwnedReadableQuestionDecide)
    case `choose`(OwnedReadableQuestionChoose)
    case `tag`(OwnedReadableQuestionTag)
    case `score`(OwnedReadableQuestionScore)
    public static func read(_ json: JSONValue) throws -> OwnedReadableQuestion {
        let object = try jsonObject(json)
        if object["verb"] == .string("decide") { return .`decide`(try OwnedReadableQuestionDecide.read(json)) }
        if object["verb"] == .string("choose") { return .`choose`(try OwnedReadableQuestionChoose.read(json)) }
        if object["verb"] == .string("tag") { return .`tag`(try OwnedReadableQuestionTag.read(json)) }
        if object["verb"] == .string("score") { return .`score`(try OwnedReadableQuestionScore.read(json)) }
        throw JSONConversionError("Unknown OwnedReadableQuestion alternative")
    }
    public var json: JSONValue {
        switch self {
        case .`decide`(let value): return value.json
        case .`choose`(let value): return value.json
        case .`tag`(let value): return value.json
        case .`score`(let value): return value.json
        }
    }
}
public struct OwnedReadableQuestion2: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `batch`: Presence<OwnedBatch>
    public let `contextSchema`: Presence<OwnedInputDeclaration>
    public let `itemSchema`: Presence<OwnedInputDeclaration>
    public let `labelDetails`: Presence<OwnedReadableQuestion2LabelDetails>
    public let `model`: Presence<OwnedReadableQuestion2Model>
    public let `name`: Presence<OwnedQuestionName>
    public let `none`: Bool
    public let `on`: Presence<[String]>
    public let `profile`: Presence<OwnedReadableQuestion2Profile>
    public let `text`: JSONValue
    public let `verb`: String
    public let `wordingVersion`: Presence<OwnedWordingVersion>
    public init(extensions: [String: JSONValue] = [:], `batch`: Presence<OwnedBatch> = .absent, `contextSchema`: Presence<OwnedInputDeclaration> = .absent, `itemSchema`: Presence<OwnedInputDeclaration> = .absent, `labelDetails`: Presence<OwnedReadableQuestion2LabelDetails> = .absent, `model`: Presence<OwnedReadableQuestion2Model> = .absent, `name`: Presence<OwnedQuestionName> = .absent, `none`: Bool, `on`: Presence<[String]> = .absent, `profile`: Presence<OwnedReadableQuestion2Profile> = .absent, `text`: JSONValue, `verb`: String = "find", `wordingVersion`: Presence<OwnedWordingVersion> = .absent) { self.extensions = extensions; self.`batch` = `batch`; self.`contextSchema` = `contextSchema`; self.`itemSchema` = `itemSchema`; self.`labelDetails` = `labelDetails`; self.`model` = `model`; self.`name` = `name`; self.`none` = `none`; self.`on` = `on`; self.`profile` = `profile`; self.`text` = `text`; self.`verb` = `verb`; self.`wordingVersion` = `wordingVersion` }
    public static func read(_ json: JSONValue) throws -> OwnedReadableQuestion2 {
        let object = try jsonObject(json)
        let known: Set<String> = ["batch", "context_schema", "item_schema", "label_details", "model", "name", "none", "on", "profile", "text", "verb", "wording_version"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, batch: try readPresence(object, "batch", OwnedBatch.self), contextSchema: try readPresence(object, "context_schema", OwnedInputDeclaration.self), itemSchema: try readPresence(object, "item_schema", OwnedInputDeclaration.self), labelDetails: try readPresence(object, "label_details", OwnedReadableQuestion2LabelDetails.self), model: try readPresence(object, "model", OwnedReadableQuestion2Model.self), name: try readPresence(object, "name", OwnedQuestionName.self), none: try Bool.read(jsonRequired(object, "none")), on: try readPresence(object, "on", [String].self), profile: try readPresence(object, "profile", OwnedReadableQuestion2Profile.self), text: try JSONValue.read(jsonRequired(object, "text")), verb: try String.read(jsonRequired(object, "verb")), wordingVersion: try readPresence(object, "wording_version", OwnedWordingVersion.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["batch", "context_schema", "item_schema", "label_details", "model", "name", "none", "on", "profile", "text", "verb", "wording_version"]
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
public indirect enum OwnedReadableQuestion2LabelDetails: JSONRepresentable {
    case alternative0([OwnedLabel])
    public static func read(_ json: JSONValue) throws -> OwnedReadableQuestion2LabelDetails {
        if let value = try? [OwnedLabel].read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedReadableQuestion2LabelDetails value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedReadableQuestion2Model: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedReadableQuestion2Model {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedReadableQuestion2Model value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedReadableQuestion2Profile: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedReadableQuestion2Profile {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedReadableQuestion2Profile value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public struct OwnedReadableQuestion3: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `batch`: Presence<OwnedBatch>
    public let `contextSchema`: Presence<OwnedInputDeclaration>
    public let `entityDefinition`: Presence<JSONValue>
    public let `instructions`: Presence<JSONValue>
    public let `itemSchema`: Presence<OwnedInputDeclaration>
    public let `kinds`: [String: JSONValue]
    public let `labelDetails`: Presence<OwnedReadableQuestion3LabelDetails>
    public let `mode`: Presence<OwnedRecognitionMode>
    public let `model`: Presence<OwnedReadableQuestion3Model>
    public let `name`: Presence<OwnedQuestionName>
    public let `on`: Presence<[String]>
    public let `profile`: Presence<OwnedReadableQuestion3Profile>
    public let `relationThreshold`: Presence<OwnedThreshold>
    public let `relations`: Presence<OwnedReadableQuestion3Relations>
    public let `snippetPieces`: Presence<UInt64>
    public let `stageContext`: Presence<OwnedRecognitionStageContext>
    public let `threshold`: OwnedThreshold
    public let `verb`: OwnedVerb
    public let `wordingVersion`: Presence<OwnedWordingVersion>
    public init(extensions: [String: JSONValue] = [:], `batch`: Presence<OwnedBatch> = .absent, `contextSchema`: Presence<OwnedInputDeclaration> = .absent, `entityDefinition`: Presence<JSONValue> = .absent, `instructions`: Presence<JSONValue> = .absent, `itemSchema`: Presence<OwnedInputDeclaration> = .absent, `kinds`: [String: JSONValue], `labelDetails`: Presence<OwnedReadableQuestion3LabelDetails> = .absent, `mode`: Presence<OwnedRecognitionMode> = .absent, `model`: Presence<OwnedReadableQuestion3Model> = .absent, `name`: Presence<OwnedQuestionName> = .absent, `on`: Presence<[String]> = .absent, `profile`: Presence<OwnedReadableQuestion3Profile> = .absent, `relationThreshold`: Presence<OwnedThreshold> = .absent, `relations`: Presence<OwnedReadableQuestion3Relations> = .absent, `snippetPieces`: Presence<UInt64> = .absent, `stageContext`: Presence<OwnedRecognitionStageContext> = .absent, `threshold`: OwnedThreshold, `verb`: OwnedVerb, `wordingVersion`: Presence<OwnedWordingVersion> = .absent) { self.extensions = extensions; self.`batch` = `batch`; self.`contextSchema` = `contextSchema`; self.`entityDefinition` = `entityDefinition`; self.`instructions` = `instructions`; self.`itemSchema` = `itemSchema`; self.`kinds` = `kinds`; self.`labelDetails` = `labelDetails`; self.`mode` = `mode`; self.`model` = `model`; self.`name` = `name`; self.`on` = `on`; self.`profile` = `profile`; self.`relationThreshold` = `relationThreshold`; self.`relations` = `relations`; self.`snippetPieces` = `snippetPieces`; self.`stageContext` = `stageContext`; self.`threshold` = `threshold`; self.`verb` = `verb`; self.`wordingVersion` = `wordingVersion` }
    public static func read(_ json: JSONValue) throws -> OwnedReadableQuestion3 {
        let object = try jsonObject(json)
        let known: Set<String> = ["batch", "context_schema", "entity_definition", "instructions", "item_schema", "kinds", "label_details", "mode", "model", "name", "on", "profile", "relation_threshold", "relations", "snippet_pieces", "stage_context", "threshold", "verb", "wording_version"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, batch: try readPresence(object, "batch", OwnedBatch.self), contextSchema: try readPresence(object, "context_schema", OwnedInputDeclaration.self), entityDefinition: try readPresence(object, "entity_definition", JSONValue.self), instructions: try readPresence(object, "instructions", JSONValue.self), itemSchema: try readPresence(object, "item_schema", OwnedInputDeclaration.self), kinds: try [String: JSONValue].read(jsonRequired(object, "kinds")), labelDetails: try readPresence(object, "label_details", OwnedReadableQuestion3LabelDetails.self), mode: try readPresence(object, "mode", OwnedRecognitionMode.self), model: try readPresence(object, "model", OwnedReadableQuestion3Model.self), name: try readPresence(object, "name", OwnedQuestionName.self), on: try readPresence(object, "on", [String].self), profile: try readPresence(object, "profile", OwnedReadableQuestion3Profile.self), relationThreshold: try readPresence(object, "relation_threshold", OwnedThreshold.self), relations: try readPresence(object, "relations", OwnedReadableQuestion3Relations.self), snippetPieces: try readPresence(object, "snippet_pieces", UInt64.self), stageContext: try readPresence(object, "stage_context", OwnedRecognitionStageContext.self), threshold: try OwnedThreshold.read(jsonRequired(object, "threshold")), verb: try OwnedVerb.read(jsonRequired(object, "verb")), wordingVersion: try readPresence(object, "wording_version", OwnedWordingVersion.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["batch", "context_schema", "entity_definition", "instructions", "item_schema", "kinds", "label_details", "mode", "model", "name", "on", "profile", "relation_threshold", "relations", "snippet_pieces", "stage_context", "threshold", "verb", "wording_version"]
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
public indirect enum OwnedReadableQuestion3LabelDetails: JSONRepresentable {
    case alternative0([OwnedLabel])
    public static func read(_ json: JSONValue) throws -> OwnedReadableQuestion3LabelDetails {
        if let value = try? [OwnedLabel].read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedReadableQuestion3LabelDetails value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedReadableQuestion3Model: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedReadableQuestion3Model {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedReadableQuestion3Model value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedReadableQuestion3Profile: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedReadableQuestion3Profile {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedReadableQuestion3Profile value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedReadableQuestion3Relations: JSONRepresentable {
    case alternative0([OwnedRelationRule])
    public static func read(_ json: JSONValue) throws -> OwnedReadableQuestion3Relations {
        if let value = try? [OwnedRelationRule].read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedReadableQuestion3Relations value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public struct OwnedReadableQuestion4: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `batch`: Presence<OwnedBatch>
    public let `contextSchema`: Presence<OwnedInputDeclaration>
    public let `fields`: OwnedRelateFields?
    public let `itemSchema`: Presence<OwnedInputDeclaration>
    public let `labelDetails`: Presence<OwnedReadableQuestion4LabelDetails>
    public let `model`: Presence<OwnedReadableQuestion4Model>
    public let `name`: Presence<OwnedQuestionName>
    public let `on`: Presence<[String]>
    public let `profile`: Presence<OwnedReadableQuestion4Profile>
    public let `relations`: [OwnedRelationRule]
    public let `threshold`: OwnedThreshold
    public let `verb`: String
    public let `wordingVersion`: Presence<OwnedWordingVersion>
    public init(extensions: [String: JSONValue] = [:], `batch`: Presence<OwnedBatch> = .absent, `contextSchema`: Presence<OwnedInputDeclaration> = .absent, `fields`: OwnedRelateFields?, `itemSchema`: Presence<OwnedInputDeclaration> = .absent, `labelDetails`: Presence<OwnedReadableQuestion4LabelDetails> = .absent, `model`: Presence<OwnedReadableQuestion4Model> = .absent, `name`: Presence<OwnedQuestionName> = .absent, `on`: Presence<[String]> = .absent, `profile`: Presence<OwnedReadableQuestion4Profile> = .absent, `relations`: [OwnedRelationRule], `threshold`: OwnedThreshold, `verb`: String = "relate", `wordingVersion`: Presence<OwnedWordingVersion> = .absent) { self.extensions = extensions; self.`batch` = `batch`; self.`contextSchema` = `contextSchema`; self.`fields` = `fields`; self.`itemSchema` = `itemSchema`; self.`labelDetails` = `labelDetails`; self.`model` = `model`; self.`name` = `name`; self.`on` = `on`; self.`profile` = `profile`; self.`relations` = `relations`; self.`threshold` = `threshold`; self.`verb` = `verb`; self.`wordingVersion` = `wordingVersion` }
    public static func read(_ json: JSONValue) throws -> OwnedReadableQuestion4 {
        let object = try jsonObject(json)
        let known: Set<String> = ["batch", "context_schema", "fields", "item_schema", "label_details", "model", "name", "on", "profile", "relations", "threshold", "verb", "wording_version"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, batch: try readPresence(object, "batch", OwnedBatch.self), contextSchema: try readPresence(object, "context_schema", OwnedInputDeclaration.self), fields: try OwnedRelateFields?.read(jsonRequired(object, "fields")), itemSchema: try readPresence(object, "item_schema", OwnedInputDeclaration.self), labelDetails: try readPresence(object, "label_details", OwnedReadableQuestion4LabelDetails.self), model: try readPresence(object, "model", OwnedReadableQuestion4Model.self), name: try readPresence(object, "name", OwnedQuestionName.self), on: try readPresence(object, "on", [String].self), profile: try readPresence(object, "profile", OwnedReadableQuestion4Profile.self), relations: try [OwnedRelationRule].read(jsonRequired(object, "relations")), threshold: try OwnedThreshold.read(jsonRequired(object, "threshold")), verb: try String.read(jsonRequired(object, "verb")), wordingVersion: try readPresence(object, "wording_version", OwnedWordingVersion.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["batch", "context_schema", "fields", "item_schema", "label_details", "model", "name", "on", "profile", "relations", "threshold", "verb", "wording_version"]
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
public indirect enum OwnedReadableQuestion4LabelDetails: JSONRepresentable {
    case alternative0([OwnedLabel])
    public static func read(_ json: JSONValue) throws -> OwnedReadableQuestion4LabelDetails {
        if let value = try? [OwnedLabel].read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedReadableQuestion4LabelDetails value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedReadableQuestion4Model: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedReadableQuestion4Model {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedReadableQuestion4Model value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedReadableQuestion4Profile: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedReadableQuestion4Profile {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedReadableQuestion4Profile value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public struct OwnedReadableQuestionChoose: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `batch`: Presence<OwnedBatch>
    public let `contextSchema`: Presence<OwnedInputDeclaration>
    public let `itemSchema`: Presence<OwnedInputDeclaration>
    public let `labelDetails`: Presence<OwnedReadableQuestionChooseLabelDetails>
    public let `model`: Presence<OwnedReadableQuestionChooseModel>
    public let `name`: Presence<OwnedQuestionName>
    public let `on`: Presence<[String]>
    public let `profile`: Presence<OwnedReadableQuestionChooseProfile>
    public let `wordingVersion`: Presence<OwnedWordingVersion>
    public let `options`: [String]
    public let `text`: JSONValue
    public let `verb`: String
    public init(extensions: [String: JSONValue] = [:], `batch`: Presence<OwnedBatch> = .absent, `contextSchema`: Presence<OwnedInputDeclaration> = .absent, `itemSchema`: Presence<OwnedInputDeclaration> = .absent, `labelDetails`: Presence<OwnedReadableQuestionChooseLabelDetails> = .absent, `model`: Presence<OwnedReadableQuestionChooseModel> = .absent, `name`: Presence<OwnedQuestionName> = .absent, `on`: Presence<[String]> = .absent, `profile`: Presence<OwnedReadableQuestionChooseProfile> = .absent, `wordingVersion`: Presence<OwnedWordingVersion> = .absent, `options`: [String], `text`: JSONValue, `verb`: String = "choose") { self.extensions = extensions; self.`batch` = `batch`; self.`contextSchema` = `contextSchema`; self.`itemSchema` = `itemSchema`; self.`labelDetails` = `labelDetails`; self.`model` = `model`; self.`name` = `name`; self.`on` = `on`; self.`profile` = `profile`; self.`wordingVersion` = `wordingVersion`; self.`options` = `options`; self.`text` = `text`; self.`verb` = `verb` }
    public static func read(_ json: JSONValue) throws -> OwnedReadableQuestionChoose {
        let object = try jsonObject(json)
        let known: Set<String> = ["batch", "context_schema", "item_schema", "label_details", "model", "name", "on", "profile", "wording_version", "options", "text", "verb"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, batch: try readPresence(object, "batch", OwnedBatch.self), contextSchema: try readPresence(object, "context_schema", OwnedInputDeclaration.self), itemSchema: try readPresence(object, "item_schema", OwnedInputDeclaration.self), labelDetails: try readPresence(object, "label_details", OwnedReadableQuestionChooseLabelDetails.self), model: try readPresence(object, "model", OwnedReadableQuestionChooseModel.self), name: try readPresence(object, "name", OwnedQuestionName.self), on: try readPresence(object, "on", [String].self), profile: try readPresence(object, "profile", OwnedReadableQuestionChooseProfile.self), wordingVersion: try readPresence(object, "wording_version", OwnedWordingVersion.self), options: try [String].read(jsonRequired(object, "options")), text: try JSONValue.read(jsonRequired(object, "text")), verb: try String.read(jsonRequired(object, "verb")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["batch", "context_schema", "item_schema", "label_details", "model", "name", "on", "profile", "wording_version", "options", "text", "verb"]
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
public indirect enum OwnedReadableQuestionChooseLabelDetails: JSONRepresentable {
    case alternative0([OwnedLabel])
    public static func read(_ json: JSONValue) throws -> OwnedReadableQuestionChooseLabelDetails {
        if let value = try? [OwnedLabel].read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedReadableQuestionChooseLabelDetails value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedReadableQuestionChooseModel: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedReadableQuestionChooseModel {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedReadableQuestionChooseModel value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedReadableQuestionChooseProfile: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedReadableQuestionChooseProfile {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedReadableQuestionChooseProfile value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public struct OwnedReadableQuestionDecide: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `batch`: Presence<OwnedBatch>
    public let `contextSchema`: Presence<OwnedInputDeclaration>
    public let `itemSchema`: Presence<OwnedInputDeclaration>
    public let `labelDetails`: Presence<OwnedReadableQuestionDecideLabelDetails>
    public let `model`: Presence<OwnedReadableQuestionDecideModel>
    public let `name`: Presence<OwnedQuestionName>
    public let `on`: Presence<[String]>
    public let `profile`: Presence<OwnedReadableQuestionDecideProfile>
    public let `wordingVersion`: Presence<OwnedWordingVersion>
    public let `false`: Presence<JSONValue>
    public let `text`: JSONValue
    public let `true`: Presence<JSONValue>
    public let `verb`: String
    public init(extensions: [String: JSONValue] = [:], `batch`: Presence<OwnedBatch> = .absent, `contextSchema`: Presence<OwnedInputDeclaration> = .absent, `itemSchema`: Presence<OwnedInputDeclaration> = .absent, `labelDetails`: Presence<OwnedReadableQuestionDecideLabelDetails> = .absent, `model`: Presence<OwnedReadableQuestionDecideModel> = .absent, `name`: Presence<OwnedQuestionName> = .absent, `on`: Presence<[String]> = .absent, `profile`: Presence<OwnedReadableQuestionDecideProfile> = .absent, `wordingVersion`: Presence<OwnedWordingVersion> = .absent, `false`: Presence<JSONValue> = .absent, `text`: JSONValue, `true`: Presence<JSONValue> = .absent, `verb`: String = "decide") { self.extensions = extensions; self.`batch` = `batch`; self.`contextSchema` = `contextSchema`; self.`itemSchema` = `itemSchema`; self.`labelDetails` = `labelDetails`; self.`model` = `model`; self.`name` = `name`; self.`on` = `on`; self.`profile` = `profile`; self.`wordingVersion` = `wordingVersion`; self.`false` = `false`; self.`text` = `text`; self.`true` = `true`; self.`verb` = `verb` }
    public static func read(_ json: JSONValue) throws -> OwnedReadableQuestionDecide {
        let object = try jsonObject(json)
        let known: Set<String> = ["batch", "context_schema", "item_schema", "label_details", "model", "name", "on", "profile", "wording_version", "false", "text", "true", "verb"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, batch: try readPresence(object, "batch", OwnedBatch.self), contextSchema: try readPresence(object, "context_schema", OwnedInputDeclaration.self), itemSchema: try readPresence(object, "item_schema", OwnedInputDeclaration.self), labelDetails: try readPresence(object, "label_details", OwnedReadableQuestionDecideLabelDetails.self), model: try readPresence(object, "model", OwnedReadableQuestionDecideModel.self), name: try readPresence(object, "name", OwnedQuestionName.self), on: try readPresence(object, "on", [String].self), profile: try readPresence(object, "profile", OwnedReadableQuestionDecideProfile.self), wordingVersion: try readPresence(object, "wording_version", OwnedWordingVersion.self), false: try readPresence(object, "false", JSONValue.self), text: try JSONValue.read(jsonRequired(object, "text")), true: try readPresence(object, "true", JSONValue.self), verb: try String.read(jsonRequired(object, "verb")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["batch", "context_schema", "item_schema", "label_details", "model", "name", "on", "profile", "wording_version", "false", "text", "true", "verb"]
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
public indirect enum OwnedReadableQuestionDecideLabelDetails: JSONRepresentable {
    case alternative0([OwnedLabel])
    public static func read(_ json: JSONValue) throws -> OwnedReadableQuestionDecideLabelDetails {
        if let value = try? [OwnedLabel].read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedReadableQuestionDecideLabelDetails value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedReadableQuestionDecideModel: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedReadableQuestionDecideModel {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedReadableQuestionDecideModel value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedReadableQuestionDecideProfile: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedReadableQuestionDecideProfile {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedReadableQuestionDecideProfile value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public struct OwnedReadableQuestionScore: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `batch`: Presence<OwnedBatch>
    public let `contextSchema`: Presence<OwnedInputDeclaration>
    public let `itemSchema`: Presence<OwnedInputDeclaration>
    public let `labelDetails`: Presence<OwnedReadableQuestionScoreLabelDetails>
    public let `model`: Presence<OwnedReadableQuestionScoreModel>
    public let `name`: Presence<OwnedQuestionName>
    public let `on`: Presence<[String]>
    public let `profile`: Presence<OwnedReadableQuestionScoreProfile>
    public let `wordingVersion`: Presence<OwnedWordingVersion>
    public let `levels`: [String]
    public let `text`: JSONValue
    public let `verb`: String
    public init(extensions: [String: JSONValue] = [:], `batch`: Presence<OwnedBatch> = .absent, `contextSchema`: Presence<OwnedInputDeclaration> = .absent, `itemSchema`: Presence<OwnedInputDeclaration> = .absent, `labelDetails`: Presence<OwnedReadableQuestionScoreLabelDetails> = .absent, `model`: Presence<OwnedReadableQuestionScoreModel> = .absent, `name`: Presence<OwnedQuestionName> = .absent, `on`: Presence<[String]> = .absent, `profile`: Presence<OwnedReadableQuestionScoreProfile> = .absent, `wordingVersion`: Presence<OwnedWordingVersion> = .absent, `levels`: [String], `text`: JSONValue, `verb`: String = "score") { self.extensions = extensions; self.`batch` = `batch`; self.`contextSchema` = `contextSchema`; self.`itemSchema` = `itemSchema`; self.`labelDetails` = `labelDetails`; self.`model` = `model`; self.`name` = `name`; self.`on` = `on`; self.`profile` = `profile`; self.`wordingVersion` = `wordingVersion`; self.`levels` = `levels`; self.`text` = `text`; self.`verb` = `verb` }
    public static func read(_ json: JSONValue) throws -> OwnedReadableQuestionScore {
        let object = try jsonObject(json)
        let known: Set<String> = ["batch", "context_schema", "item_schema", "label_details", "model", "name", "on", "profile", "wording_version", "levels", "text", "verb"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, batch: try readPresence(object, "batch", OwnedBatch.self), contextSchema: try readPresence(object, "context_schema", OwnedInputDeclaration.self), itemSchema: try readPresence(object, "item_schema", OwnedInputDeclaration.self), labelDetails: try readPresence(object, "label_details", OwnedReadableQuestionScoreLabelDetails.self), model: try readPresence(object, "model", OwnedReadableQuestionScoreModel.self), name: try readPresence(object, "name", OwnedQuestionName.self), on: try readPresence(object, "on", [String].self), profile: try readPresence(object, "profile", OwnedReadableQuestionScoreProfile.self), wordingVersion: try readPresence(object, "wording_version", OwnedWordingVersion.self), levels: try [String].read(jsonRequired(object, "levels")), text: try JSONValue.read(jsonRequired(object, "text")), verb: try String.read(jsonRequired(object, "verb")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["batch", "context_schema", "item_schema", "label_details", "model", "name", "on", "profile", "wording_version", "levels", "text", "verb"]
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
public indirect enum OwnedReadableQuestionScoreLabelDetails: JSONRepresentable {
    case alternative0([OwnedLabel])
    public static func read(_ json: JSONValue) throws -> OwnedReadableQuestionScoreLabelDetails {
        if let value = try? [OwnedLabel].read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedReadableQuestionScoreLabelDetails value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedReadableQuestionScoreModel: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedReadableQuestionScoreModel {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedReadableQuestionScoreModel value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedReadableQuestionScoreProfile: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedReadableQuestionScoreProfile {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedReadableQuestionScoreProfile value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public struct OwnedReadableQuestionTag: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `batch`: Presence<OwnedBatch>
    public let `contextSchema`: Presence<OwnedInputDeclaration>
    public let `itemSchema`: Presence<OwnedInputDeclaration>
    public let `labelDetails`: Presence<OwnedReadableQuestionTagLabelDetails>
    public let `model`: Presence<OwnedReadableQuestionTagModel>
    public let `name`: Presence<OwnedQuestionName>
    public let `on`: Presence<[String]>
    public let `profile`: Presence<OwnedReadableQuestionTagProfile>
    public let `wordingVersion`: Presence<OwnedWordingVersion>
    public let `labels`: [String]
    public let `text`: JSONValue
    public let `verb`: String
    public init(extensions: [String: JSONValue] = [:], `batch`: Presence<OwnedBatch> = .absent, `contextSchema`: Presence<OwnedInputDeclaration> = .absent, `itemSchema`: Presence<OwnedInputDeclaration> = .absent, `labelDetails`: Presence<OwnedReadableQuestionTagLabelDetails> = .absent, `model`: Presence<OwnedReadableQuestionTagModel> = .absent, `name`: Presence<OwnedQuestionName> = .absent, `on`: Presence<[String]> = .absent, `profile`: Presence<OwnedReadableQuestionTagProfile> = .absent, `wordingVersion`: Presence<OwnedWordingVersion> = .absent, `labels`: [String], `text`: JSONValue, `verb`: String = "tag") { self.extensions = extensions; self.`batch` = `batch`; self.`contextSchema` = `contextSchema`; self.`itemSchema` = `itemSchema`; self.`labelDetails` = `labelDetails`; self.`model` = `model`; self.`name` = `name`; self.`on` = `on`; self.`profile` = `profile`; self.`wordingVersion` = `wordingVersion`; self.`labels` = `labels`; self.`text` = `text`; self.`verb` = `verb` }
    public static func read(_ json: JSONValue) throws -> OwnedReadableQuestionTag {
        let object = try jsonObject(json)
        let known: Set<String> = ["batch", "context_schema", "item_schema", "label_details", "model", "name", "on", "profile", "wording_version", "labels", "text", "verb"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, batch: try readPresence(object, "batch", OwnedBatch.self), contextSchema: try readPresence(object, "context_schema", OwnedInputDeclaration.self), itemSchema: try readPresence(object, "item_schema", OwnedInputDeclaration.self), labelDetails: try readPresence(object, "label_details", OwnedReadableQuestionTagLabelDetails.self), model: try readPresence(object, "model", OwnedReadableQuestionTagModel.self), name: try readPresence(object, "name", OwnedQuestionName.self), on: try readPresence(object, "on", [String].self), profile: try readPresence(object, "profile", OwnedReadableQuestionTagProfile.self), wordingVersion: try readPresence(object, "wording_version", OwnedWordingVersion.self), labels: try [String].read(jsonRequired(object, "labels")), text: try JSONValue.read(jsonRequired(object, "text")), verb: try String.read(jsonRequired(object, "verb")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["batch", "context_schema", "item_schema", "label_details", "model", "name", "on", "profile", "wording_version", "labels", "text", "verb"]
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
public indirect enum OwnedReadableQuestionTagLabelDetails: JSONRepresentable {
    case alternative0([OwnedLabel])
    public static func read(_ json: JSONValue) throws -> OwnedReadableQuestionTagLabelDetails {
        if let value = try? [OwnedLabel].read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedReadableQuestionTagLabelDetails value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedReadableQuestionTagModel: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedReadableQuestionTagModel {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedReadableQuestionTagModel value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedReadableQuestionTagProfile: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedReadableQuestionTagProfile {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedReadableQuestionTagProfile value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public struct OwnedRecognition: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `answer`: OwnedRecognitionOdds
    public let `answerId`: OwnedAnswerId
    public let `file`: Presence<OwnedRecognitionFile>
    public let `firstLine`: Presence<OwnedRecognitionFirstLine>
    public let `index`: Presence<OwnedRecognitionIndex>
    public let `input`: Presence<JSONValue>
    public let `lastLine`: Presence<OwnedRecognitionLastLine>
    public let `meta`: OwnedMeta
    public let `position`: Presence<OwnedPosition>
    public let `question`: OwnedReadableQuestion3
    public let `schema`: OwnedVersion
    public let `source`: Presence<OwnedPhysicalSource>
    public let `value`: OwnedRecognize
    public init(extensions: [String: JSONValue] = [:], `answer`: OwnedRecognitionOdds, `answerId`: OwnedAnswerId, `file`: Presence<OwnedRecognitionFile> = .absent, `firstLine`: Presence<OwnedRecognitionFirstLine> = .absent, `index`: Presence<OwnedRecognitionIndex> = .absent, `input`: Presence<JSONValue> = .absent, `lastLine`: Presence<OwnedRecognitionLastLine> = .absent, `meta`: OwnedMeta, `position`: Presence<OwnedPosition> = .absent, `question`: OwnedReadableQuestion3, `schema`: OwnedVersion, `source`: Presence<OwnedPhysicalSource> = .absent, `value`: OwnedRecognize) { self.extensions = extensions; self.`answer` = `answer`; self.`answerId` = `answerId`; self.`file` = `file`; self.`firstLine` = `firstLine`; self.`index` = `index`; self.`input` = `input`; self.`lastLine` = `lastLine`; self.`meta` = `meta`; self.`position` = `position`; self.`question` = `question`; self.`schema` = `schema`; self.`source` = `source`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedRecognition {
        let object = try jsonObject(json)
        let known: Set<String> = ["answer", "answer_id", "file", "first_line", "index", "input", "last_line", "meta", "position", "question", "schema", "source", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, answer: try OwnedRecognitionOdds.read(jsonRequired(object, "answer")), answerId: try OwnedAnswerId.read(jsonRequired(object, "answer_id")), file: try readPresence(object, "file", OwnedRecognitionFile.self), firstLine: try readPresence(object, "first_line", OwnedRecognitionFirstLine.self), index: try readPresence(object, "index", OwnedRecognitionIndex.self), input: try readPresence(object, "input", JSONValue.self), lastLine: try readPresence(object, "last_line", OwnedRecognitionLastLine.self), meta: try OwnedMeta.read(jsonRequired(object, "meta")), position: try readPresence(object, "position", OwnedPosition.self), question: try OwnedReadableQuestion3.read(jsonRequired(object, "question")), schema: try OwnedVersion.read(jsonRequired(object, "schema")), source: try readPresence(object, "source", OwnedPhysicalSource.self), value: try OwnedRecognize.read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["answer", "answer_id", "file", "first_line", "index", "input", "last_line", "meta", "position", "question", "schema", "source", "value"]
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
public struct OwnedRecognitionEdgeDocument: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `either`: Bool
    public let `probability`: Double
    public let `relation`: String
    public let `source`: OwnedEntity
    public let `target`: OwnedEntity
    public init(extensions: [String: JSONValue] = [:], `either`: Bool, `probability`: Double, `relation`: String, `source`: OwnedEntity, `target`: OwnedEntity) { self.extensions = extensions; self.`either` = `either`; self.`probability` = `probability`; self.`relation` = `relation`; self.`source` = `source`; self.`target` = `target` }
    public static func read(_ json: JSONValue) throws -> OwnedRecognitionEdgeDocument {
        let object = try jsonObject(json)
        let known: Set<String> = ["either", "probability", "relation", "source", "target"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, either: try Bool.read(jsonRequired(object, "either")), probability: try Double.read(jsonRequired(object, "probability")), relation: try String.read(jsonRequired(object, "relation")), source: try OwnedEntity.read(jsonRequired(object, "source")), target: try OwnedEntity.read(jsonRequired(object, "target")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public indirect enum OwnedRecognitionMode: JSONRepresentable {
    case alternative0(String)
    case alternative1(String)
    public static func read(_ json: JSONValue) throws -> OwnedRecognitionMode {
        if let value = try? String.read(json) { return .alternative0(value) }
        if let value = try? String.read(json) { return .alternative1(value) }
        throw JSONConversionError("Unknown OwnedRecognitionMode value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public indirect enum OwnedRecognitionOdds: JSONRepresentable {
    case `alternative0`(OwnedRecognitionOddsFieldsNamesPairsPiecesProposals)
    case `alternative1`(OwnedRecognitionOddsFieldsPiecesProposals)
    public static func read(_ json: JSONValue) throws -> OwnedRecognitionOdds {
        let object = try jsonObject(json)
        if object["names"] != nil && object["pairs"] != nil && object["pieces"] != nil && object["proposals"] != nil { return .`alternative0`(try OwnedRecognitionOddsFieldsNamesPairsPiecesProposals.read(json)) }
        if object["pieces"] != nil && object["proposals"] != nil && object["names"] == nil && object["pairs"] == nil { return .`alternative1`(try OwnedRecognitionOddsFieldsPiecesProposals.read(json)) }
        throw JSONConversionError("Unknown OwnedRecognitionOdds alternative")
    }
    public var json: JSONValue {
        switch self {
        case .`alternative0`(let value): return value.json
        case .`alternative1`(let value): return value.json
        }
    }
}
public struct OwnedRecognitionOddsFieldsNamesPairsPiecesProposals: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `names`: [OwnedNameOdds]
    public let `pairs`: [OwnedPairOdds]
    public let `pieces`: [OwnedPieceOdds]
    public let `proposals`: [OwnedRecognitionProposal]
    public init(extensions: [String: JSONValue] = [:], `names`: [OwnedNameOdds], `pairs`: [OwnedPairOdds], `pieces`: [OwnedPieceOdds], `proposals`: [OwnedRecognitionProposal]) { self.extensions = extensions; self.`names` = `names`; self.`pairs` = `pairs`; self.`pieces` = `pieces`; self.`proposals` = `proposals` }
    public static func read(_ json: JSONValue) throws -> OwnedRecognitionOddsFieldsNamesPairsPiecesProposals {
        let object = try jsonObject(json)
        let known: Set<String> = ["names", "pairs", "pieces", "proposals"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, names: try [OwnedNameOdds].read(jsonRequired(object, "names")), pairs: try [OwnedPairOdds].read(jsonRequired(object, "pairs")), pieces: try [OwnedPieceOdds].read(jsonRequired(object, "pieces")), proposals: try [OwnedRecognitionProposal].read(jsonRequired(object, "proposals")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public struct OwnedRecognitionOddsFieldsPiecesProposals: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `pieces`: [OwnedPieceOdds]
    public let `proposals`: [OwnedBoundaryProposal]
    public init(extensions: [String: JSONValue] = [:], `pieces`: [OwnedPieceOdds], `proposals`: [OwnedBoundaryProposal]) { self.extensions = extensions; self.`pieces` = `pieces`; self.`proposals` = `proposals` }
    public static func read(_ json: JSONValue) throws -> OwnedRecognitionOddsFieldsPiecesProposals {
        let object = try jsonObject(json)
        let known: Set<String> = ["pieces", "proposals"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, pieces: try [OwnedPieceOdds].read(jsonRequired(object, "pieces")), proposals: try [OwnedBoundaryProposal].read(jsonRequired(object, "proposals")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["pieces", "proposals"]
        var object = extensions.filter { !known.contains($0.key) }
        object["pieces"] = `pieces`.json
        object["proposals"] = `proposals`.json
        return .object(object)
    }
}
public struct OwnedRecognitionProposal: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `end`: UInt64
    public let `kept`: Bool
    public let `kind`: Presence<OwnedRecognitionProposalKind>
    public let `selected`: Presence<OwnedPlace>
    public let `spanProbability`: Double
    public let `start`: UInt64
    public let `strength`: Presence<OwnedRecognitionProposalStrength>
    public init(extensions: [String: JSONValue] = [:], `end`: UInt64, `kept`: Bool, `kind`: Presence<OwnedRecognitionProposalKind> = .absent, `selected`: Presence<OwnedPlace> = .absent, `spanProbability`: Double, `start`: UInt64, `strength`: Presence<OwnedRecognitionProposalStrength> = .absent) { self.extensions = extensions; self.`end` = `end`; self.`kept` = `kept`; self.`kind` = `kind`; self.`selected` = `selected`; self.`spanProbability` = `spanProbability`; self.`start` = `start`; self.`strength` = `strength` }
    public static func read(_ json: JSONValue) throws -> OwnedRecognitionProposal {
        let object = try jsonObject(json)
        let known: Set<String> = ["end", "kept", "kind", "selected", "span_probability", "start", "strength"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, end: try UInt64.read(jsonRequired(object, "end")), kept: try Bool.read(jsonRequired(object, "kept")), kind: try readPresence(object, "kind", OwnedRecognitionProposalKind.self), selected: try readPresence(object, "selected", OwnedPlace.self), spanProbability: try Double.read(jsonRequired(object, "span_probability")), start: try UInt64.read(jsonRequired(object, "start")), strength: try readPresence(object, "strength", OwnedRecognitionProposalStrength.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["end", "kept", "kind", "selected", "span_probability", "start", "strength"]
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
public indirect enum OwnedRecognitionProposalKind: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedRecognitionProposalKind {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedRecognitionProposalKind value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedRecognitionProposalStrength: JSONRepresentable {
    case alternative0(Double)
    public static func read(_ json: JSONValue) throws -> OwnedRecognitionProposalStrength {
        if let value = try? Double.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedRecognitionProposalStrength value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public struct OwnedRecognitionStageContext: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `boundary`: Presence<String>
    public let `kindEdge`: Presence<String>
    public let `relation`: Presence<String>
    public init(extensions: [String: JSONValue] = [:], `boundary`: Presence<String> = .absent, `kindEdge`: Presence<String> = .absent, `relation`: Presence<String> = .absent) { self.extensions = extensions; self.`boundary` = `boundary`; self.`kindEdge` = `kindEdge`; self.`relation` = `relation` }
    public static func read(_ json: JSONValue) throws -> OwnedRecognitionStageContext {
        let object = try jsonObject(json)
        let known: Set<String> = ["boundary", "kind_edge", "relation"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, boundary: try readPresence(object, "boundary", String.self), kindEdge: try readPresence(object, "kind_edge", String.self), relation: try readPresence(object, "relation", String.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["boundary", "kind_edge", "relation"]
        var object = extensions.filter { !known.contains($0.key) }
        writePresence(`boundary`, "boundary", &object)
        writePresence(`kindEdge`, "kind_edge", &object)
        writePresence(`relation`, "relation", &object)
        return .object(object)
    }
}
public indirect enum OwnedRecognitionFile: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedRecognitionFile {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedRecognitionFile value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedRecognitionFirstLine: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedRecognitionFirstLine {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedRecognitionFirstLine value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedRecognitionIndex: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedRecognitionIndex {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedRecognitionIndex value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedRecognitionLastLine: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedRecognitionLastLine {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedRecognitionLastLine value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public struct OwnedRelation: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `answer`: OwnedAnswers
    public let `answerId`: OwnedAnswerId
    public let `file`: Presence<OwnedRelationFile>
    public let `firstLine`: Presence<OwnedRelationFirstLine>
    public let `index`: Presence<OwnedRelationIndex>
    public let `input`: Presence<JSONValue>
    public let `inputSources`: Presence<OwnedRelationInputSources>
    public let `lastLine`: Presence<OwnedRelationLastLine>
    public let `meta`: OwnedMeta
    public let `position`: Presence<OwnedPosition>
    public let `question`: OwnedReadableQuestion4
    public let `schema`: OwnedVersion
    public let `value`: [OwnedRelatedEntityEdge]
    public init(extensions: [String: JSONValue] = [:], `answer`: OwnedAnswers, `answerId`: OwnedAnswerId, `file`: Presence<OwnedRelationFile> = .absent, `firstLine`: Presence<OwnedRelationFirstLine> = .absent, `index`: Presence<OwnedRelationIndex> = .absent, `input`: Presence<JSONValue> = .absent, `inputSources`: Presence<OwnedRelationInputSources> = .absent, `lastLine`: Presence<OwnedRelationLastLine> = .absent, `meta`: OwnedMeta, `position`: Presence<OwnedPosition> = .absent, `question`: OwnedReadableQuestion4, `schema`: OwnedVersion, `value`: [OwnedRelatedEntityEdge]) { self.extensions = extensions; self.`answer` = `answer`; self.`answerId` = `answerId`; self.`file` = `file`; self.`firstLine` = `firstLine`; self.`index` = `index`; self.`input` = `input`; self.`inputSources` = `inputSources`; self.`lastLine` = `lastLine`; self.`meta` = `meta`; self.`position` = `position`; self.`question` = `question`; self.`schema` = `schema`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedRelation {
        let object = try jsonObject(json)
        let known: Set<String> = ["answer", "answer_id", "file", "first_line", "index", "input", "input_sources", "last_line", "meta", "position", "question", "schema", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, answer: try OwnedAnswers.read(jsonRequired(object, "answer")), answerId: try OwnedAnswerId.read(jsonRequired(object, "answer_id")), file: try readPresence(object, "file", OwnedRelationFile.self), firstLine: try readPresence(object, "first_line", OwnedRelationFirstLine.self), index: try readPresence(object, "index", OwnedRelationIndex.self), input: try readPresence(object, "input", JSONValue.self), inputSources: try readPresence(object, "input_sources", OwnedRelationInputSources.self), lastLine: try readPresence(object, "last_line", OwnedRelationLastLine.self), meta: try OwnedMeta.read(jsonRequired(object, "meta")), position: try readPresence(object, "position", OwnedPosition.self), question: try OwnedReadableQuestion4.read(jsonRequired(object, "question")), schema: try OwnedVersion.read(jsonRequired(object, "schema")), value: try [OwnedRelatedEntityEdge].read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["answer", "answer_id", "file", "first_line", "index", "input", "input_sources", "last_line", "meta", "position", "question", "schema", "value"]
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
public indirect enum OwnedRelationDirection: JSONRepresentable {
    case alternative0(String)
    case alternative1(String)
    public static func read(_ json: JSONValue) throws -> OwnedRelationDirection {
        if let value = try? String.read(json) { return .alternative0(value) }
        if let value = try? String.read(json) { return .alternative1(value) }
        throw JSONConversionError("Unknown OwnedRelationDirection value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public indirect enum OwnedRelationMember: JSONRepresentable {
    case `answerId`(OwnedRelationMemberAnswerId)
    case `failureId`(OwnedRelationMemberFailureId)
    public static func read(_ json: JSONValue) throws -> OwnedRelationMember {
        let object = try jsonObject(json)
        if object["answer_id"] != nil && object["answer_id"] != .null { return .`answerId`(try OwnedRelationMemberAnswerId.read(json)) }
        if object["failure_id"] != nil && object["failure_id"] != .null { return .`failureId`(try OwnedRelationMemberFailureId.read(json)) }
        throw JSONConversionError("Unknown OwnedRelationMember alternative")
    }
    public var json: JSONValue {
        switch self {
        case .`answerId`(let value): return value.json
        case .`failureId`(let value): return value.json
        }
    }
}
public struct OwnedRelationMemberAnswerId: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `direction`: OwnedRelationDirection
    public let `method`: OwnedRelationMethod
    public let `observations`: [OwnedObservation]
    public let `question`: OwnedReadableQuestion
    public let `questionSources`: [OwnedQuestionSource]
    public let `reads`: String
    public let `relation`: String
    public let `request`: String
    public let `source`: OwnedRelatedEntity
    public let `target`: OwnedRelatedEntity?
    public let `threshold`: OwnedThreshold
    public let `usage`: Presence<OwnedUsage>
    public let `accepted`: Bool
    public let `answer`: OwnedAnswer
    public let `answerId`: OwnedAnswerId
    public let `probability`: Double
    public init(extensions: [String: JSONValue] = [:], `direction`: OwnedRelationDirection, `method`: OwnedRelationMethod, `observations`: [OwnedObservation], `question`: OwnedReadableQuestion, `questionSources`: [OwnedQuestionSource], `reads`: String, `relation`: String, `request`: String, `source`: OwnedRelatedEntity, `target`: OwnedRelatedEntity?, `threshold`: OwnedThreshold, `usage`: Presence<OwnedUsage> = .absent, `accepted`: Bool, `answer`: OwnedAnswer, `answerId`: OwnedAnswerId, `probability`: Double) { self.extensions = extensions; self.`direction` = `direction`; self.`method` = `method`; self.`observations` = `observations`; self.`question` = `question`; self.`questionSources` = `questionSources`; self.`reads` = `reads`; self.`relation` = `relation`; self.`request` = `request`; self.`source` = `source`; self.`target` = `target`; self.`threshold` = `threshold`; self.`usage` = `usage`; self.`accepted` = `accepted`; self.`answer` = `answer`; self.`answerId` = `answerId`; self.`probability` = `probability` }
    public static func read(_ json: JSONValue) throws -> OwnedRelationMemberAnswerId {
        let object = try jsonObject(json)
        let known: Set<String> = ["direction", "method", "observations", "question", "question_sources", "reads", "relation", "request", "source", "target", "threshold", "usage", "accepted", "answer", "answer_id", "probability"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, direction: try OwnedRelationDirection.read(jsonRequired(object, "direction")), method: try OwnedRelationMethod.read(jsonRequired(object, "method")), observations: try [OwnedObservation].read(jsonRequired(object, "observations")), question: try OwnedReadableQuestion.read(jsonRequired(object, "question")), questionSources: try [OwnedQuestionSource].read(jsonRequired(object, "question_sources")), reads: try String.read(jsonRequired(object, "reads")), relation: try String.read(jsonRequired(object, "relation")), request: try String.read(jsonRequired(object, "request")), source: try OwnedRelatedEntity.read(jsonRequired(object, "source")), target: try OwnedRelatedEntity?.read(jsonRequired(object, "target")), threshold: try OwnedThreshold.read(jsonRequired(object, "threshold")), usage: try readPresence(object, "usage", OwnedUsage.self), accepted: try Bool.read(jsonRequired(object, "accepted")), answer: try OwnedAnswer.read(jsonRequired(object, "answer")), answerId: try OwnedAnswerId.read(jsonRequired(object, "answer_id")), probability: try Double.read(jsonRequired(object, "probability")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["direction", "method", "observations", "question", "question_sources", "reads", "relation", "request", "source", "target", "threshold", "usage", "accepted", "answer", "answer_id", "probability"]
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
}
public struct OwnedRelationMemberFailureId: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `direction`: OwnedRelationDirection
    public let `method`: OwnedRelationMethod
    public let `observations`: [OwnedObservation]
    public let `question`: OwnedReadableQuestion
    public let `questionSources`: [OwnedQuestionSource]
    public let `reads`: String
    public let `relation`: String
    public let `request`: String
    public let `source`: OwnedRelatedEntity
    public let `target`: OwnedRelatedEntity?
    public let `threshold`: OwnedThreshold
    public let `usage`: Presence<OwnedUsage>
    public let `failure`: OwnedFailure
    public let `failureId`: OwnedFailureId
    public init(extensions: [String: JSONValue] = [:], `direction`: OwnedRelationDirection, `method`: OwnedRelationMethod, `observations`: [OwnedObservation], `question`: OwnedReadableQuestion, `questionSources`: [OwnedQuestionSource], `reads`: String, `relation`: String, `request`: String, `source`: OwnedRelatedEntity, `target`: OwnedRelatedEntity?, `threshold`: OwnedThreshold, `usage`: Presence<OwnedUsage> = .absent, `failure`: OwnedFailure, `failureId`: OwnedFailureId) { self.extensions = extensions; self.`direction` = `direction`; self.`method` = `method`; self.`observations` = `observations`; self.`question` = `question`; self.`questionSources` = `questionSources`; self.`reads` = `reads`; self.`relation` = `relation`; self.`request` = `request`; self.`source` = `source`; self.`target` = `target`; self.`threshold` = `threshold`; self.`usage` = `usage`; self.`failure` = `failure`; self.`failureId` = `failureId` }
    public static func read(_ json: JSONValue) throws -> OwnedRelationMemberFailureId {
        let object = try jsonObject(json)
        let known: Set<String> = ["direction", "method", "observations", "question", "question_sources", "reads", "relation", "request", "source", "target", "threshold", "usage", "failure", "failure_id"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, direction: try OwnedRelationDirection.read(jsonRequired(object, "direction")), method: try OwnedRelationMethod.read(jsonRequired(object, "method")), observations: try [OwnedObservation].read(jsonRequired(object, "observations")), question: try OwnedReadableQuestion.read(jsonRequired(object, "question")), questionSources: try [OwnedQuestionSource].read(jsonRequired(object, "question_sources")), reads: try String.read(jsonRequired(object, "reads")), relation: try String.read(jsonRequired(object, "relation")), request: try String.read(jsonRequired(object, "request")), source: try OwnedRelatedEntity.read(jsonRequired(object, "source")), target: try OwnedRelatedEntity?.read(jsonRequired(object, "target")), threshold: try OwnedThreshold.read(jsonRequired(object, "threshold")), usage: try readPresence(object, "usage", OwnedUsage.self), failure: try OwnedFailure.read(jsonRequired(object, "failure")), failureId: try OwnedFailureId.read(jsonRequired(object, "failure_id")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["direction", "method", "observations", "question", "question_sources", "reads", "relation", "request", "source", "target", "threshold", "usage", "failure", "failure_id"]
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
}
public indirect enum OwnedRelationMethod: JSONRepresentable {
    case alternative0(String)
    case alternative1(String)
    public static func read(_ json: JSONValue) throws -> OwnedRelationMethod {
        if let value = try? String.read(json) { return .alternative0(value) }
        if let value = try? String.read(json) { return .alternative1(value) }
        throw JSONConversionError("Unknown OwnedRelationMethod value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public indirect enum OwnedRelationFile: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedRelationFile {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedRelationFile value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedRelationFirstLine: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedRelationFirstLine {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedRelationFirstLine value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedRelationIndex: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedRelationIndex {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedRelationIndex value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedRelationInputSources: JSONRepresentable {
    case alternative0([OwnedSessionInputSource])
    public static func read(_ json: JSONValue) throws -> OwnedRelationInputSources {
        if let value = try? [OwnedSessionInputSource].read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedRelationInputSources value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedRelationLastLine: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedRelationLastLine {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedRelationLastLine value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedRequestFunction: JSONRepresentable {
    case alternative0(String)
    case alternative1(String)
    case alternative2(String)
    case alternative3(String)
    case alternative4(String)
    case alternative5(String)
    case alternative6(String)
    case alternative7(String)
    case alternative8(String)
    case alternative9(String)
    public static func read(_ json: JSONValue) throws -> OwnedRequestFunction {
        if let value = try? String.read(json) { return .alternative0(value) }
        if let value = try? String.read(json) { return .alternative1(value) }
        if let value = try? String.read(json) { return .alternative2(value) }
        if let value = try? String.read(json) { return .alternative3(value) }
        if let value = try? String.read(json) { return .alternative4(value) }
        if let value = try? String.read(json) { return .alternative5(value) }
        if let value = try? String.read(json) { return .alternative6(value) }
        if let value = try? String.read(json) { return .alternative7(value) }
        if let value = try? String.read(json) { return .alternative8(value) }
        if let value = try? String.read(json) { return .alternative9(value) }
        throw JSONConversionError("Unknown OwnedRequestFunction value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        case .alternative2(let value): return value.json
        case .alternative3(let value): return value.json
        case .alternative4(let value): return value.json
        case .alternative5(let value): return value.json
        case .alternative6(let value): return value.json
        case .alternative7(let value): return value.json
        case .alternative8(let value): return value.json
        case .alternative9(let value): return value.json
        }
    }
}
public typealias OwnedSdkRequestId = String
public indirect enum OwnedSendBudgetDenial: JSONRepresentable {
    case `beforeFirstSend`(OwnedSendBudgetDenialBeforeFirstSend)
    case `beforeAdditionalSend`(OwnedSendBudgetDenialBeforeAdditionalSend)
    case `beforeRetry`(OwnedSendBudgetDenialBeforeRetry)
    public static func read(_ json: JSONValue) throws -> OwnedSendBudgetDenial {
        let object = try jsonObject(json)
        if object["kind"] == .string("before_first_send") { return .`beforeFirstSend`(try OwnedSendBudgetDenialBeforeFirstSend.read(json)) }
        if object["kind"] == .string("before_additional_send") { return .`beforeAdditionalSend`(try OwnedSendBudgetDenialBeforeAdditionalSend.read(json)) }
        if object["kind"] == .string("before_retry") { return .`beforeRetry`(try OwnedSendBudgetDenialBeforeRetry.read(json)) }
        throw JSONConversionError("Unknown OwnedSendBudgetDenial alternative")
    }
    public var json: JSONValue {
        switch self {
        case .`beforeFirstSend`(let value): return value.json
        case .`beforeAdditionalSend`(let value): return value.json
        case .`beforeRetry`(let value): return value.json
        }
    }
}
public struct OwnedSendBudgetDenialBeforeAdditionalSend: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "before_additional_send") { self.extensions = extensions; self.`kind` = `kind` }
    public static func read(_ json: JSONValue) throws -> OwnedSendBudgetDenialBeforeAdditionalSend {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        return .object(object)
    }
}
public struct OwnedSendBudgetDenialBeforeFirstSend: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "before_first_send") { self.extensions = extensions; self.`kind` = `kind` }
    public static func read(_ json: JSONValue) throws -> OwnedSendBudgetDenialBeforeFirstSend {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        return .object(object)
    }
}
public struct OwnedSendBudgetDenialBeforeRetry: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `lastStatus`: UInt64
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "before_retry", `lastStatus`: UInt64) { self.extensions = extensions; self.`kind` = `kind`; self.`lastStatus` = `lastStatus` }
    public static func read(_ json: JSONValue) throws -> OwnedSendBudgetDenialBeforeRetry {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "last_status"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), lastStatus: try UInt64.read(jsonRequired(object, "last_status")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "last_status"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        object["last_status"] = `lastStatus`.json
        return .object(object)
    }
}
public indirect enum OwnedStopCause: JSONRepresentable {
    case alternative0(String)
    case alternative1(String)
    case alternative2(String)
    case alternative3(String)
    case alternative4(String)
    case alternative5(String)
    case alternative6(String)
    case alternative7(String)
    case alternative8(String)
    case alternative9(String)
    case alternative10(String)
    public static func read(_ json: JSONValue) throws -> OwnedStopCause {
        if let value = try? String.read(json) { return .alternative0(value) }
        if let value = try? String.read(json) { return .alternative1(value) }
        if let value = try? String.read(json) { return .alternative2(value) }
        if let value = try? String.read(json) { return .alternative3(value) }
        if let value = try? String.read(json) { return .alternative4(value) }
        if let value = try? String.read(json) { return .alternative5(value) }
        if let value = try? String.read(json) { return .alternative6(value) }
        if let value = try? String.read(json) { return .alternative7(value) }
        if let value = try? String.read(json) { return .alternative8(value) }
        if let value = try? String.read(json) { return .alternative9(value) }
        if let value = try? String.read(json) { return .alternative10(value) }
        throw JSONConversionError("Unknown OwnedStopCause value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        case .alternative2(let value): return value.json
        case .alternative3(let value): return value.json
        case .alternative4(let value): return value.json
        case .alternative5(let value): return value.json
        case .alternative6(let value): return value.json
        case .alternative7(let value): return value.json
        case .alternative8(let value): return value.json
        case .alternative9(let value): return value.json
        case .alternative10(let value): return value.json
        }
    }
}
public struct OwnedStopped: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `at`: Presence<OwnedStoppedAt>
    public let `cause`: OwnedStopCause
    public let `retryable`: Bool
    public let `status`: Presence<OwnedStoppedStatus>
    public init(extensions: [String: JSONValue] = [:], `at`: Presence<OwnedStoppedAt> = .absent, `cause`: OwnedStopCause, `retryable`: Bool, `status`: Presence<OwnedStoppedStatus> = .absent) { self.extensions = extensions; self.`at` = `at`; self.`cause` = `cause`; self.`retryable` = `retryable`; self.`status` = `status` }
    public static func read(_ json: JSONValue) throws -> OwnedStopped {
        let object = try jsonObject(json)
        let known: Set<String> = ["at", "cause", "retryable", "status"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, at: try readPresence(object, "at", OwnedStoppedAt.self), cause: try OwnedStopCause.read(jsonRequired(object, "cause")), retryable: try Bool.read(jsonRequired(object, "retryable")), status: try readPresence(object, "status", OwnedStoppedStatus.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public indirect enum OwnedStoppedAt: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedStoppedAt {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedStoppedAt value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedStoppedStatus: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedStoppedStatus {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedStoppedStatus value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public struct OwnedStringRoot: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `type`: OwnedStringType
    public init(extensions: [String: JSONValue] = [:], `type`: OwnedStringType) { self.extensions = extensions; self.`type` = `type` }
    public static func read(_ json: JSONValue) throws -> OwnedStringRoot {
        let object = try jsonObject(json)
        let known: Set<String> = ["type"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, type: try OwnedStringType.read(jsonRequired(object, "type")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["type"]
        var object = extensions.filter { !known.contains($0.key) }
        object["type"] = `type`.json
        return .object(object)
    }
}
public typealias OwnedStringType = String
public struct OwnedUsage: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `inputTokens`: Presence<OwnedUsageInputTokens>
    public let `outputTokens`: Presence<OwnedUsageOutputTokens>
    public init(extensions: [String: JSONValue] = [:], `inputTokens`: Presence<OwnedUsageInputTokens> = .absent, `outputTokens`: Presence<OwnedUsageOutputTokens> = .absent) { self.extensions = extensions; self.`inputTokens` = `inputTokens`; self.`outputTokens` = `outputTokens` }
    public static func read(_ json: JSONValue) throws -> OwnedUsage {
        let object = try jsonObject(json)
        let known: Set<String> = ["input_tokens", "output_tokens"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, inputTokens: try readPresence(object, "input_tokens", OwnedUsageInputTokens.self), outputTokens: try readPresence(object, "output_tokens", OwnedUsageOutputTokens.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["input_tokens", "output_tokens"]
        var object = extensions.filter { !known.contains($0.key) }
        writePresence(`inputTokens`, "input_tokens", &object)
        writePresence(`outputTokens`, "output_tokens", &object)
        return .object(object)
    }
}
public indirect enum OwnedUsagePersistence: JSONRepresentable {
    case alternative0(String)
    case alternative1(String)
    case alternative2(String)
    case alternative3(String)
    public static func read(_ json: JSONValue) throws -> OwnedUsagePersistence {
        if let value = try? String.read(json) { return .alternative0(value) }
        if let value = try? String.read(json) { return .alternative1(value) }
        if let value = try? String.read(json) { return .alternative2(value) }
        if let value = try? String.read(json) { return .alternative3(value) }
        throw JSONConversionError("Unknown OwnedUsagePersistence value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        case .alternative2(let value): return value.json
        case .alternative3(let value): return value.json
        }
    }
}
public indirect enum OwnedUsageInputTokens: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedUsageInputTokens {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedUsageInputTokens value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedUsageOutputTokens: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedUsageOutputTokens {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedUsageOutputTokens value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public typealias OwnedVerb = String
public typealias OwnedVersion = String
public typealias OwnedWordingVersion = UInt64
public indirect enum OwnedAnnotatedField: JSONRepresentable {
    case alternative0(Bool)
    case alternative1(JSONValue)
    case alternative2(String)
    case alternative3([String])
    case alternative4(Double)
    case alternative5(OwnedFailed)
    public static func read(_ json: JSONValue) throws -> OwnedAnnotatedField {
        if let value = try? Bool.read(json) { return .alternative0(value) }
        if json == .null { return .alternative1(.null) }
        if let value = try? String.read(json) { return .alternative2(value) }
        if let value = try? [String].read(json) { return .alternative3(value) }
        if let value = try? Double.read(json) { return .alternative4(value) }
        if let value = try? OwnedFailed.read(json) { return .alternative5(value) }
        throw JSONConversionError("Unknown OwnedAnnotatedField value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        case .alternative2(let value): return value.json
        case .alternative3(let value): return value.json
        case .alternative4(let value): return value.json
        case .alternative5(let value): return value.json
        }
    }
}
public typealias OwnedAnnotatedRow = [String: OwnedAnnotatedField]
public indirect enum OwnedAnswer: JSONRepresentable {
    case `yesNo`(OwnedAnswerYesNo)
    case `choice`(OwnedAnswerChoice)
    case `tag`(OwnedAnswerTag)
    case `score`(OwnedAnswerScore)
    public static func read(_ json: JSONValue) throws -> OwnedAnswer {
        let object = try jsonObject(json)
        if object["kind"] == .string("yes_no") { return .`yesNo`(try OwnedAnswerYesNo.read(json)) }
        if object["kind"] == .string("choice") { return .`choice`(try OwnedAnswerChoice.read(json)) }
        if object["kind"] == .string("tag") { return .`tag`(try OwnedAnswerTag.read(json)) }
        if object["kind"] == .string("score") { return .`score`(try OwnedAnswerScore.read(json)) }
        throw JSONConversionError("Unknown OwnedAnswer alternative")
    }
    public var json: JSONValue {
        switch self {
        case .`yesNo`(let value): return value.json
        case .`choice`(let value): return value.json
        case .`tag`(let value): return value.json
        case .`score`(let value): return value.json
        }
    }
}
public struct OwnedAnswerChoice: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `confidence`: Presence<OwnedAnswerChoiceConfidence>
    public let `kind`: String
    public let `pick`: String
    public let `probabilities`: [String: Double]
    public init(extensions: [String: JSONValue] = [:], `confidence`: Presence<OwnedAnswerChoiceConfidence> = .absent, `kind`: String = "choice", `pick`: String, `probabilities`: [String: Double]) { self.extensions = extensions; self.`confidence` = `confidence`; self.`kind` = `kind`; self.`pick` = `pick`; self.`probabilities` = `probabilities` }
    public static func read(_ json: JSONValue) throws -> OwnedAnswerChoice {
        let object = try jsonObject(json)
        let known: Set<String> = ["confidence", "kind", "pick", "probabilities"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, confidence: try readPresence(object, "confidence", OwnedAnswerChoiceConfidence.self), kind: try String.read(jsonRequired(object, "kind")), pick: try String.read(jsonRequired(object, "pick")), probabilities: try [String: Double].read(jsonRequired(object, "probabilities")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public indirect enum OwnedAnswerChoiceConfidence: JSONRepresentable {
    case alternative0(Double)
    public static func read(_ json: JSONValue) throws -> OwnedAnswerChoiceConfidence {
        if let value = try? Double.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedAnswerChoiceConfidence value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public struct OwnedAnswerScore: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `confidence`: Presence<OwnedAnswerScoreConfidence>
    public let `kind`: String
    public let `level`: String
    public let `probabilities`: [String: Double]
    public init(extensions: [String: JSONValue] = [:], `confidence`: Presence<OwnedAnswerScoreConfidence> = .absent, `kind`: String = "score", `level`: String, `probabilities`: [String: Double]) { self.extensions = extensions; self.`confidence` = `confidence`; self.`kind` = `kind`; self.`level` = `level`; self.`probabilities` = `probabilities` }
    public static func read(_ json: JSONValue) throws -> OwnedAnswerScore {
        let object = try jsonObject(json)
        let known: Set<String> = ["confidence", "kind", "level", "probabilities"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, confidence: try readPresence(object, "confidence", OwnedAnswerScoreConfidence.self), kind: try String.read(jsonRequired(object, "kind")), level: try String.read(jsonRequired(object, "level")), probabilities: try [String: Double].read(jsonRequired(object, "probabilities")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public indirect enum OwnedAnswerScoreConfidence: JSONRepresentable {
    case alternative0(Double)
    public static func read(_ json: JSONValue) throws -> OwnedAnswerScoreConfidence {
        if let value = try? Double.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedAnswerScoreConfidence value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public struct OwnedAnswerTag: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `probabilities`: [String: Double]
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "tag", `probabilities`: [String: Double]) { self.extensions = extensions; self.`kind` = `kind`; self.`probabilities` = `probabilities` }
    public static func read(_ json: JSONValue) throws -> OwnedAnswerTag {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "probabilities"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), probabilities: try [String: Double].read(jsonRequired(object, "probabilities")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "probabilities"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        object["probabilities"] = `probabilities`.json
        return .object(object)
    }
}
public struct OwnedAnswerYesNo: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `probability`: Double
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "yes_no", `probability`: Double) { self.extensions = extensions; self.`kind` = `kind`; self.`probability` = `probability` }
    public static func read(_ json: JSONValue) throws -> OwnedAnswerYesNo {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "probability"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), probability: try Double.read(jsonRequired(object, "probability")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "probability"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        object["probability"] = `probability`.json
        return .object(object)
    }
}
public indirect enum OwnedAttemptOutcome: JSONRepresentable {
    case alternative0(String)
    case alternative1(String)
    case alternative2(String)
    public static func read(_ json: JSONValue) throws -> OwnedAttemptOutcome {
        if let value = try? String.read(json) { return .alternative0(value) }
        if let value = try? String.read(json) { return .alternative1(value) }
        if let value = try? String.read(json) { return .alternative2(value) }
        throw JSONConversionError("Unknown OwnedAttemptOutcome value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        case .alternative2(let value): return value.json
        }
    }
}
public indirect enum OwnedBatchSetting: JSONRepresentable {
    case alternative0(UInt64)
    case alternative1(String)
    public static func read(_ json: JSONValue) throws -> OwnedBatchSetting {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        if let value = try? String.read(json) { return .alternative1(value) }
        throw JSONConversionError("Unknown OwnedBatchSetting value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public struct OwnedBatchWarning: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `running`: OwnedBatchSetting
    public let `tunedFor`: OwnedBatchSetting
    public init(extensions: [String: JSONValue] = [:], `running`: OwnedBatchSetting, `tunedFor`: OwnedBatchSetting) { self.extensions = extensions; self.`running` = `running`; self.`tunedFor` = `tunedFor` }
    public static func read(_ json: JSONValue) throws -> OwnedBatchWarning {
        let object = try jsonObject(json)
        let known: Set<String> = ["running", "tuned_for"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, running: try OwnedBatchSetting.read(jsonRequired(object, "running")), tunedFor: try OwnedBatchSetting.read(jsonRequired(object, "tuned_for")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["running", "tuned_for"]
        var object = extensions.filter { !known.contains($0.key) }
        object["running"] = `running`.json
        object["tuned_for"] = `tunedFor`.json
        return .object(object)
    }
}
public struct OwnedEntity: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `end`: UInt64
    public let `file`: Presence<OwnedEntityFile>
    public let `firstLine`: Presence<OwnedEntityFirstLine>
    public let `kind`: String
    public let `lastLine`: Presence<OwnedEntityLastLine>
    public let `length`: UInt64
    public let `start`: UInt64
    public let `strength`: Double
    public let `text`: String
    public init(extensions: [String: JSONValue] = [:], `end`: UInt64, `file`: Presence<OwnedEntityFile> = .absent, `firstLine`: Presence<OwnedEntityFirstLine> = .absent, `kind`: String, `lastLine`: Presence<OwnedEntityLastLine> = .absent, `length`: UInt64, `start`: UInt64, `strength`: Double, `text`: String) { self.extensions = extensions; self.`end` = `end`; self.`file` = `file`; self.`firstLine` = `firstLine`; self.`kind` = `kind`; self.`lastLine` = `lastLine`; self.`length` = `length`; self.`start` = `start`; self.`strength` = `strength`; self.`text` = `text` }
    public static func read(_ json: JSONValue) throws -> OwnedEntity {
        let object = try jsonObject(json)
        let known: Set<String> = ["end", "file", "first_line", "kind", "last_line", "length", "start", "strength", "text"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, end: try UInt64.read(jsonRequired(object, "end")), file: try readPresence(object, "file", OwnedEntityFile.self), firstLine: try readPresence(object, "first_line", OwnedEntityFirstLine.self), kind: try String.read(jsonRequired(object, "kind")), lastLine: try readPresence(object, "last_line", OwnedEntityLastLine.self), length: try UInt64.read(jsonRequired(object, "length")), start: try UInt64.read(jsonRequired(object, "start")), strength: try Double.read(jsonRequired(object, "strength")), text: try String.read(jsonRequired(object, "text")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["end", "file", "first_line", "kind", "last_line", "length", "start", "strength", "text"]
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
public struct OwnedEntityEdge: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `either`: Presence<Bool>
    public let `probability`: Double
    public let `relation`: String
    public let `source`: OwnedEntity
    public let `target`: OwnedEntity
    public init(extensions: [String: JSONValue] = [:], `either`: Presence<Bool> = .absent, `probability`: Double, `relation`: String, `source`: OwnedEntity, `target`: OwnedEntity) { self.extensions = extensions; self.`either` = `either`; self.`probability` = `probability`; self.`relation` = `relation`; self.`source` = `source`; self.`target` = `target` }
    public static func read(_ json: JSONValue) throws -> OwnedEntityEdge {
        let object = try jsonObject(json)
        let known: Set<String> = ["either", "probability", "relation", "source", "target"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, either: try readPresence(object, "either", Bool.self), probability: try Double.read(jsonRequired(object, "probability")), relation: try String.read(jsonRequired(object, "relation")), source: try OwnedEntity.read(jsonRequired(object, "source")), target: try OwnedEntity.read(jsonRequired(object, "target")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public indirect enum OwnedEntityFile: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedEntityFile {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedEntityFile value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedEntityFirstLine: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedEntityFirstLine {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedEntityFirstLine value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedEntityLastLine: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedEntityLastLine {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedEntityLastLine value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public struct OwnedFailed: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `failed`: OwnedFailure
    public init(extensions: [String: JSONValue] = [:], `failed`: OwnedFailure) { self.extensions = extensions; self.`failed` = `failed` }
    public static func read(_ json: JSONValue) throws -> OwnedFailed {
        let object = try jsonObject(json)
        let known: Set<String> = ["failed"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, failed: try OwnedFailure.read(jsonRequired(object, "failed")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["failed"]
        var object = extensions.filter { !known.contains($0.key) }
        object["failed"] = `failed`.json
        return .object(object)
    }
}
public struct OwnedFailure: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `cause`: OwnedFailureCause
    public let `kind`: String
    public init(extensions: [String: JSONValue] = [:], `cause`: OwnedFailureCause, `kind`: String) { self.extensions = extensions; self.`cause` = `cause`; self.`kind` = `kind` }
    public static func read(_ json: JSONValue) throws -> OwnedFailure {
        let object = try jsonObject(json)
        let known: Set<String> = ["cause", "kind"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, cause: try OwnedFailureCause.read(jsonRequired(object, "cause")), kind: try String.read(jsonRequired(object, "kind")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["cause", "kind"]
        var object = extensions.filter { !known.contains($0.key) }
        object["cause"] = `cause`.json
        object["kind"] = `kind`.json
        return .object(object)
    }
}
public indirect enum OwnedFailureCause: JSONRepresentable {
    case alternative0(String)
    case alternative1(String)
    case alternative2(String)
    case alternative3(String)
    case alternative4(String)
    case alternative5(String)
    public static func read(_ json: JSONValue) throws -> OwnedFailureCause {
        if let value = try? String.read(json) { return .alternative0(value) }
        if let value = try? String.read(json) { return .alternative1(value) }
        if let value = try? String.read(json) { return .alternative2(value) }
        if let value = try? String.read(json) { return .alternative3(value) }
        if let value = try? String.read(json) { return .alternative4(value) }
        if let value = try? String.read(json) { return .alternative5(value) }
        throw JSONConversionError("Unknown OwnedFailureCause value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        case .alternative2(let value): return value.json
        case .alternative3(let value): return value.json
        case .alternative4(let value): return value.json
        case .alternative5(let value): return value.json
        }
    }
}
public indirect enum OwnedFailureKind: JSONRepresentable {
    case alternative0(String)
    case alternative1(String)
    case alternative2(String)
    case alternative3(String)
    case alternative4(String)
    case alternative5(String)
    public static func read(_ json: JSONValue) throws -> OwnedFailureKind {
        if let value = try? String.read(json) { return .alternative0(value) }
        if let value = try? String.read(json) { return .alternative1(value) }
        if let value = try? String.read(json) { return .alternative2(value) }
        if let value = try? String.read(json) { return .alternative3(value) }
        if let value = try? String.read(json) { return .alternative4(value) }
        if let value = try? String.read(json) { return .alternative5(value) }
        throw JSONConversionError("Unknown OwnedFailureKind value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        case .alternative2(let value): return value.json
        case .alternative3(let value): return value.json
        case .alternative4(let value): return value.json
        case .alternative5(let value): return value.json
        }
    }
}
public struct OwnedFindAnswer: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `confidence`: Presence<OwnedFindAnswerConfidence>
    public let `kind`: String
    public let `pick`: String
    public let `probabilities`: [String: Double]
    public init(extensions: [String: JSONValue] = [:], `confidence`: Presence<OwnedFindAnswerConfidence> = .absent, `kind`: String = "find", `pick`: String, `probabilities`: [String: Double]) { self.extensions = extensions; self.`confidence` = `confidence`; self.`kind` = `kind`; self.`pick` = `pick`; self.`probabilities` = `probabilities` }
    public static func read(_ json: JSONValue) throws -> OwnedFindAnswer {
        let object = try jsonObject(json)
        let known: Set<String> = ["confidence", "kind", "pick", "probabilities"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, confidence: try readPresence(object, "confidence", OwnedFindAnswerConfidence.self), kind: try String.read(jsonRequired(object, "kind")), pick: try String.read(jsonRequired(object, "pick")), probabilities: try [String: Double].read(jsonRequired(object, "probabilities")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public indirect enum OwnedFindAnswerConfidence: JSONRepresentable {
    case alternative0(Double)
    public static func read(_ json: JSONValue) throws -> OwnedFindAnswerConfidence {
        if let value = try? Double.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedFindAnswerConfidence value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public struct OwnedNameOdds: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `edges`: OwnedNameOddsEdges
    public let `end`: UInt64
    public let `kinds`: OwnedNameOddsKinds
    public let `start`: UInt64
    public init(extensions: [String: JSONValue] = [:], `edges`: OwnedNameOddsEdges, `end`: UInt64, `kinds`: OwnedNameOddsKinds, `start`: UInt64) { self.extensions = extensions; self.`edges` = `edges`; self.`end` = `end`; self.`kinds` = `kinds`; self.`start` = `start` }
    public static func read(_ json: JSONValue) throws -> OwnedNameOdds {
        let object = try jsonObject(json)
        let known: Set<String> = ["edges", "end", "kinds", "start"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, edges: try OwnedNameOddsEdges.read(jsonRequired(object, "edges")), end: try UInt64.read(jsonRequired(object, "end")), kinds: try OwnedNameOddsKinds.read(jsonRequired(object, "kinds")), start: try UInt64.read(jsonRequired(object, "start")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public indirect enum OwnedNameOddsEdges: JSONRepresentable {
    case alternative0([String: Double])
    case alternative1(JSONValue)
    public static func read(_ json: JSONValue) throws -> OwnedNameOddsEdges {
        if let value = try? [String: Double].read(json) { return .alternative0(value) }
        if json == .null { return .alternative1(.null) }
        throw JSONConversionError("Unknown OwnedNameOddsEdges value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public indirect enum OwnedNameOddsKinds: JSONRepresentable {
    case alternative0([String: Double])
    case alternative1(JSONValue)
    public static func read(_ json: JSONValue) throws -> OwnedNameOddsKinds {
        if let value = try? [String: Double].read(json) { return .alternative0(value) }
        if json == .null { return .alternative1(.null) }
        throw JSONConversionError("Unknown OwnedNameOddsKinds value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public struct OwnedPairOdds: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `probability`: Double
    public let `relation`: String
    public let `source`: OwnedPlace
    public let `target`: OwnedPlace
    public init(extensions: [String: JSONValue] = [:], `probability`: Double, `relation`: String, `source`: OwnedPlace, `target`: OwnedPlace) { self.extensions = extensions; self.`probability` = `probability`; self.`relation` = `relation`; self.`source` = `source`; self.`target` = `target` }
    public static func read(_ json: JSONValue) throws -> OwnedPairOdds {
        let object = try jsonObject(json)
        let known: Set<String> = ["probability", "relation", "source", "target"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, probability: try Double.read(jsonRequired(object, "probability")), relation: try String.read(jsonRequired(object, "relation")), source: try OwnedPlace.read(jsonRequired(object, "source")), target: try OwnedPlace.read(jsonRequired(object, "target")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public struct OwnedPieceOdds: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `end`: UInt64
    public let `start`: UInt64
    public let `tags`: [String: Double]
    public init(extensions: [String: JSONValue] = [:], `end`: UInt64, `start`: UInt64, `tags`: [String: Double]) { self.extensions = extensions; self.`end` = `end`; self.`start` = `start`; self.`tags` = `tags` }
    public static func read(_ json: JSONValue) throws -> OwnedPieceOdds {
        let object = try jsonObject(json)
        let known: Set<String> = ["end", "start", "tags"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, end: try UInt64.read(jsonRequired(object, "end")), start: try UInt64.read(jsonRequired(object, "start")), tags: try [String: Double].read(jsonRequired(object, "tags")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["end", "start", "tags"]
        var object = extensions.filter { !known.contains($0.key) }
        object["end"] = `end`.json
        object["start"] = `start`.json
        object["tags"] = `tags`.json
        return .object(object)
    }
}
public struct OwnedPlace: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `end`: UInt64
    public let `start`: UInt64
    public init(extensions: [String: JSONValue] = [:], `end`: UInt64, `start`: UInt64) { self.extensions = extensions; self.`end` = `end`; self.`start` = `start` }
    public static func read(_ json: JSONValue) throws -> OwnedPlace {
        let object = try jsonObject(json)
        let known: Set<String> = ["end", "start"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, end: try UInt64.read(jsonRequired(object, "end")), start: try UInt64.read(jsonRequired(object, "start")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["end", "start"]
        var object = extensions.filter { !known.contains($0.key) }
        object["end"] = `end`.json
        object["start"] = `start`.json
        return .object(object)
    }
}
public struct OwnedPlan: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `estimatedBytes`: UInt64
    public let `estimatedInputTokens`: OwnedTokenBand
    public let `firstBodyUtf8`: OwnedPlanFirstBodyUtf8
    public let `largestRequestBytes`: UInt64
    public let `largestRequestEstimatedInputTokens`: UInt64
    public let `records`: UInt64
    public let `requests`: UInt64
    public let `tokenEstimateMethod`: String
    public let `upperBound`: Bool
    public init(extensions: [String: JSONValue] = [:], `estimatedBytes`: UInt64, `estimatedInputTokens`: OwnedTokenBand, `firstBodyUtf8`: OwnedPlanFirstBodyUtf8, `largestRequestBytes`: UInt64, `largestRequestEstimatedInputTokens`: UInt64, `records`: UInt64, `requests`: UInt64, `tokenEstimateMethod`: String, `upperBound`: Bool) { self.extensions = extensions; self.`estimatedBytes` = `estimatedBytes`; self.`estimatedInputTokens` = `estimatedInputTokens`; self.`firstBodyUtf8` = `firstBodyUtf8`; self.`largestRequestBytes` = `largestRequestBytes`; self.`largestRequestEstimatedInputTokens` = `largestRequestEstimatedInputTokens`; self.`records` = `records`; self.`requests` = `requests`; self.`tokenEstimateMethod` = `tokenEstimateMethod`; self.`upperBound` = `upperBound` }
    public static func read(_ json: JSONValue) throws -> OwnedPlan {
        let object = try jsonObject(json)
        let known: Set<String> = ["estimated_bytes", "estimated_input_tokens", "first_body_utf8", "largest_request_bytes", "largest_request_estimated_input_tokens", "records", "requests", "token_estimate_method", "upper_bound"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, estimatedBytes: try UInt64.read(jsonRequired(object, "estimated_bytes")), estimatedInputTokens: try OwnedTokenBand.read(jsonRequired(object, "estimated_input_tokens")), firstBodyUtf8: try OwnedPlanFirstBodyUtf8.read(jsonRequired(object, "first_body_utf8")), largestRequestBytes: try UInt64.read(jsonRequired(object, "largest_request_bytes")), largestRequestEstimatedInputTokens: try UInt64.read(jsonRequired(object, "largest_request_estimated_input_tokens")), records: try UInt64.read(jsonRequired(object, "records")), requests: try UInt64.read(jsonRequired(object, "requests")), tokenEstimateMethod: try String.read(jsonRequired(object, "token_estimate_method")), upperBound: try Bool.read(jsonRequired(object, "upper_bound")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["estimated_bytes", "estimated_input_tokens", "first_body_utf8", "largest_request_bytes", "largest_request_estimated_input_tokens", "records", "requests", "token_estimate_method", "upper_bound"]
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
public indirect enum OwnedPlanFirstBodyUtf8: JSONRepresentable {
    case alternative0(String)
    case alternative1(JSONValue)
    public static func read(_ json: JSONValue) throws -> OwnedPlanFirstBodyUtf8 {
        if let value = try? String.read(json) { return .alternative0(value) }
        if json == .null { return .alternative1(.null) }
        throw JSONConversionError("Unknown OwnedPlanFirstBodyUtf8 value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public struct OwnedProfileWarning: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `running`: String
    public let `tunedFor`: String
    public init(extensions: [String: JSONValue] = [:], `running`: String, `tunedFor`: String) { self.extensions = extensions; self.`running` = `running`; self.`tunedFor` = `tunedFor` }
    public static func read(_ json: JSONValue) throws -> OwnedProfileWarning {
        let object = try jsonObject(json)
        let known: Set<String> = ["running", "tuned_for"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, running: try String.read(jsonRequired(object, "running")), tunedFor: try String.read(jsonRequired(object, "tuned_for")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["running", "tuned_for"]
        var object = extensions.filter { !known.contains($0.key) }
        object["running"] = `running`.json
        object["tuned_for"] = `tunedFor`.json
        return .object(object)
    }
}
public indirect enum OwnedRecognize: JSONRepresentable {
    case `alternative0`(OwnedRecognizeFieldsEntities)
    case `alternative1`(OwnedRecognizeFieldsModeProposals)
    public static func read(_ json: JSONValue) throws -> OwnedRecognize {
        let object = try jsonObject(json)
        if object["entities"] != nil && object["mode"] == nil && object["proposals"] == nil { return .`alternative0`(try OwnedRecognizeFieldsEntities.read(json)) }
        if object["mode"] != nil && object["proposals"] != nil && object["entities"] == nil { return .`alternative1`(try OwnedRecognizeFieldsModeProposals.read(json)) }
        throw JSONConversionError("Unknown OwnedRecognize alternative")
    }
    public var json: JSONValue {
        switch self {
        case .`alternative0`(let value): return value.json
        case .`alternative1`(let value): return value.json
        }
    }
}
public struct OwnedRecognizeAnswer: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `names`: [OwnedNameOdds]
    public let `pairs`: [OwnedPairOdds]
    public let `pieces`: [OwnedPieceOdds]
    public let `proposals`: [OwnedRecognitionProposal]
    public init(extensions: [String: JSONValue] = [:], `names`: [OwnedNameOdds], `pairs`: [OwnedPairOdds], `pieces`: [OwnedPieceOdds], `proposals`: [OwnedRecognitionProposal]) { self.extensions = extensions; self.`names` = `names`; self.`pairs` = `pairs`; self.`pieces` = `pieces`; self.`proposals` = `proposals` }
    public static func read(_ json: JSONValue) throws -> OwnedRecognizeAnswer {
        let object = try jsonObject(json)
        let known: Set<String> = ["names", "pairs", "pieces", "proposals"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, names: try [OwnedNameOdds].read(jsonRequired(object, "names")), pairs: try [OwnedPairOdds].read(jsonRequired(object, "pairs")), pieces: try [OwnedPieceOdds].read(jsonRequired(object, "pieces")), proposals: try [OwnedRecognitionProposal].read(jsonRequired(object, "proposals")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public struct OwnedRecognizeFieldsEntities: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `entities`: [OwnedEntity]
    public let `relations`: Presence<OwnedRecognizeFieldsEntitiesRelations>
    public init(extensions: [String: JSONValue] = [:], `entities`: [OwnedEntity], `relations`: Presence<OwnedRecognizeFieldsEntitiesRelations> = .absent) { self.extensions = extensions; self.`entities` = `entities`; self.`relations` = `relations` }
    public static func read(_ json: JSONValue) throws -> OwnedRecognizeFieldsEntities {
        let object = try jsonObject(json)
        let known: Set<String> = ["entities", "relations"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, entities: try [OwnedEntity].read(jsonRequired(object, "entities")), relations: try readPresence(object, "relations", OwnedRecognizeFieldsEntitiesRelations.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["entities", "relations"]
        var object = extensions.filter { !known.contains($0.key) }
        object["entities"] = `entities`.json
        writePresence(`relations`, "relations", &object)
        return .object(object)
    }
}
public indirect enum OwnedRecognizeFieldsEntitiesRelations: JSONRepresentable {
    case alternative0([OwnedEntityEdge])
    public static func read(_ json: JSONValue) throws -> OwnedRecognizeFieldsEntitiesRelations {
        if let value = try? [OwnedEntityEdge].read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedRecognizeFieldsEntitiesRelations value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public struct OwnedRecognizeFieldsModeProposals: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `mode`: OwnedBoundaryMode
    public let `proposals`: [OwnedBoundaryProposal]
    public init(extensions: [String: JSONValue] = [:], `mode`: OwnedBoundaryMode, `proposals`: [OwnedBoundaryProposal]) { self.extensions = extensions; self.`mode` = `mode`; self.`proposals` = `proposals` }
    public static func read(_ json: JSONValue) throws -> OwnedRecognizeFieldsModeProposals {
        let object = try jsonObject(json)
        let known: Set<String> = ["mode", "proposals"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, mode: try OwnedBoundaryMode.read(jsonRequired(object, "mode")), proposals: try [OwnedBoundaryProposal].read(jsonRequired(object, "proposals")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["mode", "proposals"]
        var object = extensions.filter { !known.contains($0.key) }
        object["mode"] = `mode`.json
        object["proposals"] = `proposals`.json
        return .object(object)
    }
}
public struct OwnedRelateFields: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `name`: String
    public init(extensions: [String: JSONValue] = [:], `kind`: String, `name`: String) { self.extensions = extensions; self.`kind` = `kind`; self.`name` = `name` }
    public static func read(_ json: JSONValue) throws -> OwnedRelateFields {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "name"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), name: try String.read(jsonRequired(object, "name")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "name"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        object["name"] = `name`.json
        return .object(object)
    }
}
public struct OwnedRelatedEntity: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `name`: String
    public init(extensions: [String: JSONValue] = [:], `kind`: String, `name`: String) { self.extensions = extensions; self.`kind` = `kind`; self.`name` = `name` }
    public static func read(_ json: JSONValue) throws -> OwnedRelatedEntity {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "name"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), name: try String.read(jsonRequired(object, "name")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "name"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        object["name"] = `name`.json
        return .object(object)
    }
}
public struct OwnedRelatedEntityEdge: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `either`: Presence<Bool>
    public let `probability`: Double
    public let `relation`: String
    public let `source`: OwnedRelatedEntityEdgePropertiesSource
    public let `target`: OwnedRelatedEntityEdgePropertiesSource
    public init(extensions: [String: JSONValue] = [:], `either`: Presence<Bool> = .absent, `probability`: Double, `relation`: String, `source`: OwnedRelatedEntityEdgePropertiesSource, `target`: OwnedRelatedEntityEdgePropertiesSource) { self.extensions = extensions; self.`either` = `either`; self.`probability` = `probability`; self.`relation` = `relation`; self.`source` = `source`; self.`target` = `target` }
    public static func read(_ json: JSONValue) throws -> OwnedRelatedEntityEdge {
        let object = try jsonObject(json)
        let known: Set<String> = ["either", "probability", "relation", "source", "target"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, either: try readPresence(object, "either", Bool.self), probability: try Double.read(jsonRequired(object, "probability")), relation: try String.read(jsonRequired(object, "relation")), source: try OwnedRelatedEntityEdgePropertiesSource.read(jsonRequired(object, "source")), target: try OwnedRelatedEntityEdgePropertiesSource.read(jsonRequired(object, "target")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public indirect enum OwnedRelatedEntityEdgePropertiesSource: JSONRepresentable {
    case `alternative0`(OwnedRelatedEntityEdgePropertiesSourceFieldsKindName)
    case `alternative1`(OwnedRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord)
    public static func read(_ json: JSONValue) throws -> OwnedRelatedEntityEdgePropertiesSource {
        let object = try jsonObject(json)
        if object["kind"] != nil && object["name"] != nil && object["file"] == nil && object["ordinal"] == nil && object["record"] == nil { return .`alternative0`(try OwnedRelatedEntityEdgePropertiesSourceFieldsKindName.read(json)) }
        if object["file"] != nil && object["kind"] != nil && object["name"] != nil && object["ordinal"] != nil && object["record"] != nil { return .`alternative1`(try OwnedRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord.read(json)) }
        throw JSONConversionError("Unknown OwnedRelatedEntityEdgePropertiesSource alternative")
    }
    public var json: JSONValue {
        switch self {
        case .`alternative0`(let value): return value.json
        case .`alternative1`(let value): return value.json
        }
    }
}
public struct OwnedRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `file`: OwnedRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecordFile
    public let `firstLine`: Presence<OwnedRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecordFirstLine>
    public let `kind`: String
    public let `lastLine`: Presence<OwnedRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecordLastLine>
    public let `name`: String
    public let `ordinal`: UInt64
    public let `record`: JSONValue
    public init(extensions: [String: JSONValue] = [:], `file`: OwnedRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecordFile, `firstLine`: Presence<OwnedRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecordFirstLine> = .absent, `kind`: String, `lastLine`: Presence<OwnedRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecordLastLine> = .absent, `name`: String, `ordinal`: UInt64, `record`: JSONValue) { self.extensions = extensions; self.`file` = `file`; self.`firstLine` = `firstLine`; self.`kind` = `kind`; self.`lastLine` = `lastLine`; self.`name` = `name`; self.`ordinal` = `ordinal`; self.`record` = `record` }
    public static func read(_ json: JSONValue) throws -> OwnedRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord {
        let object = try jsonObject(json)
        let known: Set<String> = ["file", "first_line", "kind", "last_line", "name", "ordinal", "record"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, file: try OwnedRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecordFile.read(jsonRequired(object, "file")), firstLine: try readPresence(object, "first_line", OwnedRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecordFirstLine.self), kind: try String.read(jsonRequired(object, "kind")), lastLine: try readPresence(object, "last_line", OwnedRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecordLastLine.self), name: try String.read(jsonRequired(object, "name")), ordinal: try UInt64.read(jsonRequired(object, "ordinal")), record: try JSONValue.read(jsonRequired(object, "record")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["file", "first_line", "kind", "last_line", "name", "ordinal", "record"]
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
public indirect enum OwnedRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecordFile: JSONRepresentable {
    case alternative0(String)
    case alternative1(JSONValue)
    public static func read(_ json: JSONValue) throws -> OwnedRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecordFile {
        if let value = try? String.read(json) { return .alternative0(value) }
        if json == .null { return .alternative1(.null) }
        throw JSONConversionError("Unknown OwnedRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecordFile value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public indirect enum OwnedRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecordFirstLine: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecordFirstLine {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecordFirstLine value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecordLastLine: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecordLastLine {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecordLastLine value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public struct OwnedRelatedEntityEdgePropertiesSourceFieldsKindName: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `name`: String
    public init(extensions: [String: JSONValue] = [:], `kind`: String, `name`: String) { self.extensions = extensions; self.`kind` = `kind`; self.`name` = `name` }
    public static func read(_ json: JSONValue) throws -> OwnedRelatedEntityEdgePropertiesSourceFieldsKindName {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "name"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), name: try String.read(jsonRequired(object, "name")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "name"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        object["name"] = `name`.json
        return .object(object)
    }
}
public struct OwnedRelationRule: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `either`: Bool
    public let `name`: String
    public let `reads`: String
    public let `single`: Presence<Bool>
    public let `source`: String
    public let `target`: String
    public init(extensions: [String: JSONValue] = [:], `either`: Bool, `name`: String, `reads`: String, `single`: Presence<Bool> = .absent, `source`: String, `target`: String) { self.extensions = extensions; self.`either` = `either`; self.`name` = `name`; self.`reads` = `reads`; self.`single` = `single`; self.`source` = `source`; self.`target` = `target` }
    public static func read(_ json: JSONValue) throws -> OwnedRelationRule {
        let object = try jsonObject(json)
        let known: Set<String> = ["either", "name", "reads", "single", "source", "target"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, either: try Bool.read(jsonRequired(object, "either")), name: try String.read(jsonRequired(object, "name")), reads: try String.read(jsonRequired(object, "reads")), single: try readPresence(object, "single", Bool.self), source: try String.read(jsonRequired(object, "source")), target: try String.read(jsonRequired(object, "target")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public struct OwnedSessionAnnotation: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `name`: String
    public let `value`: OwnedAnnotationValue
    public init(extensions: [String: JSONValue] = [:], `name`: String, `value`: OwnedAnnotationValue) { self.extensions = extensions; self.`name` = `name`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionAnnotation {
        let object = try jsonObject(json)
        let known: Set<String> = ["name", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, name: try String.read(jsonRequired(object, "name")), value: try OwnedAnnotationValue.read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["name", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["name"] = `name`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public struct OwnedSessionInputSource: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `index`: UInt64
    public let `source`: OwnedPhysicalSource
    public init(extensions: [String: JSONValue] = [:], `index`: UInt64, `source`: OwnedPhysicalSource) { self.extensions = extensions; self.`index` = `index`; self.`source` = `source` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionInputSource {
        let object = try jsonObject(json)
        let known: Set<String> = ["index", "source"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, index: try UInt64.read(jsonRequired(object, "index")), source: try OwnedPhysicalSource.read(jsonRequired(object, "source")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["index", "source"]
        var object = extensions.filter { !known.contains($0.key) }
        object["index"] = `index`.json
        object["source"] = `source`.json
        return .object(object)
    }
}
public indirect enum OwnedSessionJudgment: JSONRepresentable {
    case `decision`(OwnedSessionJudgmentDecision)
    case `choice`(OwnedSessionJudgmentChoice)
    case `score`(OwnedSessionJudgmentScore)
    case `tags`(OwnedSessionJudgmentTags)
    public static func read(_ json: JSONValue) throws -> OwnedSessionJudgment {
        let object = try jsonObject(json)
        if object["kind"] == .string("decision") { return .`decision`(try OwnedSessionJudgmentDecision.read(json)) }
        if object["kind"] == .string("choice") { return .`choice`(try OwnedSessionJudgmentChoice.read(json)) }
        if object["kind"] == .string("score") { return .`score`(try OwnedSessionJudgmentScore.read(json)) }
        if object["kind"] == .string("tags") { return .`tags`(try OwnedSessionJudgmentTags.read(json)) }
        throw JSONConversionError("Unknown OwnedSessionJudgment alternative")
    }
    public var json: JSONValue {
        switch self {
        case .`decision`(let value): return value.json
        case .`choice`(let value): return value.json
        case .`score`(let value): return value.json
        case .`tags`(let value): return value.json
        }
    }
}
public struct OwnedSessionJudgmentChoice: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `value`: OwnedSessionJudgmentChoiceValue
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "choice", `value`: OwnedSessionJudgmentChoiceValue) { self.extensions = extensions; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionJudgmentChoice {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), value: try OwnedSessionJudgmentChoiceValue.read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public indirect enum OwnedSessionJudgmentChoiceValue: JSONRepresentable {
    case alternative0(String)
    case alternative1(JSONValue)
    public static func read(_ json: JSONValue) throws -> OwnedSessionJudgmentChoiceValue {
        if let value = try? String.read(json) { return .alternative0(value) }
        if json == .null { return .alternative1(.null) }
        throw JSONConversionError("Unknown OwnedSessionJudgmentChoiceValue value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public struct OwnedSessionJudgmentDecision: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `value`: OwnedSessionJudgmentDecisionValue
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "decision", `value`: OwnedSessionJudgmentDecisionValue) { self.extensions = extensions; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionJudgmentDecision {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), value: try OwnedSessionJudgmentDecisionValue.read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public indirect enum OwnedSessionJudgmentDecisionValue: JSONRepresentable {
    case alternative0(Bool)
    case alternative1(JSONValue)
    public static func read(_ json: JSONValue) throws -> OwnedSessionJudgmentDecisionValue {
        if let value = try? Bool.read(json) { return .alternative0(value) }
        if json == .null { return .alternative1(.null) }
        throw JSONConversionError("Unknown OwnedSessionJudgmentDecisionValue value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public struct OwnedSessionJudgmentScore: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `value`: Double
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "score", `value`: Double) { self.extensions = extensions; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionJudgmentScore {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), value: try Double.read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public struct OwnedSessionJudgmentTags: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `value`: [String]
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "tags", `value`: [String]) { self.extensions = extensions; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionJudgmentTags {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), value: try [String].read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public struct OwnedSessionNamedProbability: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `name`: String
    public let `probability`: Double
    public init(extensions: [String: JSONValue] = [:], `name`: String, `probability`: Double) { self.extensions = extensions; self.`name` = `name`; self.`probability` = `probability` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionNamedProbability {
        let object = try jsonObject(json)
        let known: Set<String> = ["name", "probability"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, name: try String.read(jsonRequired(object, "name")), probability: try Double.read(jsonRequired(object, "probability")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["name", "probability"]
        var object = extensions.filter { !known.contains($0.key) }
        object["name"] = `name`.json
        object["probability"] = `probability`.json
        return .object(object)
    }
}
public indirect enum OwnedSessionObservation: JSONRepresentable {
    case `question`(OwnedSessionObservationQuestion)
    case `row`(OwnedSessionObservationRow)
    public static func read(_ json: JSONValue) throws -> OwnedSessionObservation {
        let object = try jsonObject(json)
        if object["kind"] == .string("question") { return .`question`(try OwnedSessionObservationQuestion.read(json)) }
        if object["kind"] == .string("row") { return .`row`(try OwnedSessionObservationRow.read(json)) }
        throw JSONConversionError("Unknown OwnedSessionObservation alternative")
    }
    public var json: JSONValue {
        switch self {
        case .`question`(let value): return value.json
        case .`row`(let value): return value.json
        }
    }
}
public struct OwnedSessionObservationQuestion: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `detail`: OwnedSessionQuestionDetail
    public let `index`: UInt64
    public let `kind`: String
    public let `member`: Presence<OwnedSessionObservationQuestionMember>
    public let `position`: UInt64
    public let `stage`: Presence<OwnedSessionObservationQuestionStage>
    public init(extensions: [String: JSONValue] = [:], `detail`: OwnedSessionQuestionDetail, `index`: UInt64, `kind`: String = "question", `member`: Presence<OwnedSessionObservationQuestionMember> = .absent, `position`: UInt64, `stage`: Presence<OwnedSessionObservationQuestionStage> = .absent) { self.extensions = extensions; self.`detail` = `detail`; self.`index` = `index`; self.`kind` = `kind`; self.`member` = `member`; self.`position` = `position`; self.`stage` = `stage` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionObservationQuestion {
        let object = try jsonObject(json)
        let known: Set<String> = ["detail", "index", "kind", "member", "position", "stage"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, detail: try OwnedSessionQuestionDetail.read(jsonRequired(object, "detail")), index: try UInt64.read(jsonRequired(object, "index")), kind: try String.read(jsonRequired(object, "kind")), member: try readPresence(object, "member", OwnedSessionObservationQuestionMember.self), position: try UInt64.read(jsonRequired(object, "position")), stage: try readPresence(object, "stage", OwnedSessionObservationQuestionStage.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public indirect enum OwnedSessionObservationQuestionMember: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedSessionObservationQuestionMember {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedSessionObservationQuestionMember value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedSessionObservationQuestionStage: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedSessionObservationQuestionStage {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedSessionObservationQuestionStage value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public struct OwnedSessionObservationRow: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `index`: UInt64
    public let `kind`: String
    public let `value`: OwnedSessionObservedRow
    public init(extensions: [String: JSONValue] = [:], `index`: UInt64, `kind`: String = "row", `value`: OwnedSessionObservedRow) { self.extensions = extensions; self.`index` = `index`; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionObservationRow {
        let object = try jsonObject(json)
        let known: Set<String> = ["index", "kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, index: try UInt64.read(jsonRequired(object, "index")), kind: try String.read(jsonRequired(object, "kind")), value: try OwnedSessionObservedRow.read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["index", "kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["index"] = `index`.json
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public indirect enum OwnedSessionObservedRow: JSONRepresentable {
    case `judgment`(OwnedSessionObservedRowJudgment)
    case `annotated`(OwnedSessionObservedRowAnnotated)
    case `recognized`(OwnedSessionObservedRowRecognized)
    case `find`(OwnedSessionObservedRowFind)
    case `relations`(OwnedSessionObservedRowRelations)
    public static func read(_ json: JSONValue) throws -> OwnedSessionObservedRow {
        let object = try jsonObject(json)
        if object["kind"] == .string("judgment") { return .`judgment`(try OwnedSessionObservedRowJudgment.read(json)) }
        if object["kind"] == .string("annotated") { return .`annotated`(try OwnedSessionObservedRowAnnotated.read(json)) }
        if object["kind"] == .string("recognized") { return .`recognized`(try OwnedSessionObservedRowRecognized.read(json)) }
        if object["kind"] == .string("find") { return .`find`(try OwnedSessionObservedRowFind.read(json)) }
        if object["kind"] == .string("relations") { return .`relations`(try OwnedSessionObservedRowRelations.read(json)) }
        throw JSONConversionError("Unknown OwnedSessionObservedRow alternative")
    }
    public var json: JSONValue {
        switch self {
        case .`judgment`(let value): return value.json
        case .`annotated`(let value): return value.json
        case .`recognized`(let value): return value.json
        case .`find`(let value): return value.json
        case .`relations`(let value): return value.json
        }
    }
}
public struct OwnedSessionObservedRowAnnotated: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `value`: [OwnedSessionAnnotation]
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "annotated", `value`: [OwnedSessionAnnotation]) { self.extensions = extensions; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionObservedRowAnnotated {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), value: try [OwnedSessionAnnotation].read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public struct OwnedSessionObservedRowFind: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `value`: OwnedSessionObservedRowFindValue
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "find", `value`: OwnedSessionObservedRowFindValue) { self.extensions = extensions; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionObservedRowFind {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), value: try OwnedSessionObservedRowFindValue.read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public indirect enum OwnedSessionObservedRowFindValue: JSONRepresentable {
    case alternative0(UInt64)
    case alternative1(JSONValue)
    public static func read(_ json: JSONValue) throws -> OwnedSessionObservedRowFindValue {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        if json == .null { return .alternative1(.null) }
        throw JSONConversionError("Unknown OwnedSessionObservedRowFindValue value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public struct OwnedSessionObservedRowJudgment: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `value`: OwnedSessionJudgment
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "judgment", `value`: OwnedSessionJudgment) { self.extensions = extensions; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionObservedRowJudgment {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), value: try OwnedSessionJudgment.read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public struct OwnedSessionObservedRowRecognized: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `value`: OwnedSessionRecognition
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "recognized", `value`: OwnedSessionRecognition) { self.extensions = extensions; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionObservedRowRecognized {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), value: try OwnedSessionRecognition.read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public struct OwnedSessionObservedRowRelations: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `value`: [OwnedSessionRelationEdge]
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "relations", `value`: [OwnedSessionRelationEdge]) { self.extensions = extensions; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionObservedRowRelations {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), value: try [OwnedSessionRelationEdge].read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public indirect enum OwnedSessionPacket: JSONRepresentable {
    case `decideRow`(OwnedSessionPacketDecideRow)
    case `chooseRow`(OwnedSessionPacketChooseRow)
    case `tagRow`(OwnedSessionPacketTagRow)
    case `scoreRow`(OwnedSessionPacketScoreRow)
    case `filterRow`(OwnedSessionPacketFilterRow)
    case `annotateRow`(OwnedSessionPacketAnnotateRow)
    case `decideAggregate`(OwnedSessionPacketDecideAggregate)
    case `chooseAggregate`(OwnedSessionPacketChooseAggregate)
    case `tagAggregate`(OwnedSessionPacketTagAggregate)
    case `scoreAggregate`(OwnedSessionPacketScoreAggregate)
    case `filterAggregate`(OwnedSessionPacketFilterAggregate)
    case `rankAggregate`(OwnedSessionPacketRankAggregate)
    case `findAggregate`(OwnedSessionPacketFindAggregate)
    case `annotateAggregate`(OwnedSessionPacketAnnotateAggregate)
    case `recognizeAggregate`(OwnedSessionPacketRecognizeAggregate)
    case `relateAggregate`(OwnedSessionPacketRelateAggregate)
    case `observation`(OwnedSessionPacketObservation)
    case `terminal`(OwnedSessionPacketTerminal)
    public static func read(_ json: JSONValue) throws -> OwnedSessionPacket {
        let object = try jsonObject(json)
        if object["function"] == .string("decide") && object["kind"] == .string("row") { return .`decideRow`(try OwnedSessionPacketDecideRow.read(json)) }
        if object["function"] == .string("choose") && object["kind"] == .string("row") { return .`chooseRow`(try OwnedSessionPacketChooseRow.read(json)) }
        if object["function"] == .string("tag") && object["kind"] == .string("row") { return .`tagRow`(try OwnedSessionPacketTagRow.read(json)) }
        if object["function"] == .string("score") && object["kind"] == .string("row") { return .`scoreRow`(try OwnedSessionPacketScoreRow.read(json)) }
        if object["function"] == .string("filter") && object["kind"] == .string("row") { return .`filterRow`(try OwnedSessionPacketFilterRow.read(json)) }
        if object["function"] == .string("annotate") && object["kind"] == .string("row") { return .`annotateRow`(try OwnedSessionPacketAnnotateRow.read(json)) }
        if object["function"] == .string("decide") && object["kind"] == .string("aggregate") { return .`decideAggregate`(try OwnedSessionPacketDecideAggregate.read(json)) }
        if object["function"] == .string("choose") && object["kind"] == .string("aggregate") { return .`chooseAggregate`(try OwnedSessionPacketChooseAggregate.read(json)) }
        if object["function"] == .string("tag") && object["kind"] == .string("aggregate") { return .`tagAggregate`(try OwnedSessionPacketTagAggregate.read(json)) }
        if object["function"] == .string("score") && object["kind"] == .string("aggregate") { return .`scoreAggregate`(try OwnedSessionPacketScoreAggregate.read(json)) }
        if object["function"] == .string("filter") && object["kind"] == .string("aggregate") { return .`filterAggregate`(try OwnedSessionPacketFilterAggregate.read(json)) }
        if object["function"] == .string("rank") && object["kind"] == .string("aggregate") { return .`rankAggregate`(try OwnedSessionPacketRankAggregate.read(json)) }
        if object["function"] == .string("find") && object["kind"] == .string("aggregate") { return .`findAggregate`(try OwnedSessionPacketFindAggregate.read(json)) }
        if object["function"] == .string("annotate") && object["kind"] == .string("aggregate") { return .`annotateAggregate`(try OwnedSessionPacketAnnotateAggregate.read(json)) }
        if object["function"] == .string("recognize") && object["kind"] == .string("aggregate") { return .`recognizeAggregate`(try OwnedSessionPacketRecognizeAggregate.read(json)) }
        if object["function"] == .string("relate") && object["kind"] == .string("aggregate") { return .`relateAggregate`(try OwnedSessionPacketRelateAggregate.read(json)) }
        if object["kind"] == .string("observation") { return .`observation`(try OwnedSessionPacketObservation.read(json)) }
        if object["kind"] == .string("terminal") { return .`terminal`(try OwnedSessionPacketTerminal.read(json)) }
        throw JSONConversionError("Unknown OwnedSessionPacket alternative")
    }
    public var json: JSONValue {
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
public struct OwnedSessionPacketAnnotateAggregate: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `function`: String
    public let `kind`: String
    public let `value`: [OwnedAnnotation]
    public init(extensions: [String: JSONValue] = [:], `function`: String = "annotate", `kind`: String = "aggregate", `value`: [OwnedAnnotation]) { self.extensions = extensions; self.`function` = `function`; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionPacketAnnotateAggregate {
        let object = try jsonObject(json)
        let known: Set<String> = ["function", "kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, function: try String.read(jsonRequired(object, "function")), kind: try String.read(jsonRequired(object, "kind")), value: try [OwnedAnnotation].read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["function", "kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["function"] = `function`.json
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public struct OwnedSessionPacketAnnotateRow: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `function`: String
    public let `kind`: String
    public let `value`: OwnedAnnotation
    public init(extensions: [String: JSONValue] = [:], `function`: String = "annotate", `kind`: String = "row", `value`: OwnedAnnotation) { self.extensions = extensions; self.`function` = `function`; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionPacketAnnotateRow {
        let object = try jsonObject(json)
        let known: Set<String> = ["function", "kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, function: try String.read(jsonRequired(object, "function")), kind: try String.read(jsonRequired(object, "kind")), value: try OwnedAnnotation.read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["function", "kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["function"] = `function`.json
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public struct OwnedSessionPacketChooseAggregate: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `function`: String
    public let `kind`: String
    public let `value`: [OwnedAtomicNullableString]
    public init(extensions: [String: JSONValue] = [:], `function`: String = "choose", `kind`: String = "aggregate", `value`: [OwnedAtomicNullableString]) { self.extensions = extensions; self.`function` = `function`; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionPacketChooseAggregate {
        let object = try jsonObject(json)
        let known: Set<String> = ["function", "kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, function: try String.read(jsonRequired(object, "function")), kind: try String.read(jsonRequired(object, "kind")), value: try [OwnedAtomicNullableString].read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["function", "kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["function"] = `function`.json
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public struct OwnedSessionPacketChooseRow: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `function`: String
    public let `kind`: String
    public let `value`: OwnedAtomicNullableString
    public init(extensions: [String: JSONValue] = [:], `function`: String = "choose", `kind`: String = "row", `value`: OwnedAtomicNullableString) { self.extensions = extensions; self.`function` = `function`; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionPacketChooseRow {
        let object = try jsonObject(json)
        let known: Set<String> = ["function", "kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, function: try String.read(jsonRequired(object, "function")), kind: try String.read(jsonRequired(object, "kind")), value: try OwnedAtomicNullableString.read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["function", "kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["function"] = `function`.json
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public struct OwnedSessionPacketDecideAggregate: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `function`: String
    public let `kind`: String
    public let `value`: [OwnedAtomicDecideValue]
    public init(extensions: [String: JSONValue] = [:], `function`: String = "decide", `kind`: String = "aggregate", `value`: [OwnedAtomicDecideValue]) { self.extensions = extensions; self.`function` = `function`; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionPacketDecideAggregate {
        let object = try jsonObject(json)
        let known: Set<String> = ["function", "kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, function: try String.read(jsonRequired(object, "function")), kind: try String.read(jsonRequired(object, "kind")), value: try [OwnedAtomicDecideValue].read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["function", "kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["function"] = `function`.json
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public struct OwnedSessionPacketDecideRow: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `function`: String
    public let `kind`: String
    public let `value`: OwnedAtomicDecideValue
    public init(extensions: [String: JSONValue] = [:], `function`: String = "decide", `kind`: String = "row", `value`: OwnedAtomicDecideValue) { self.extensions = extensions; self.`function` = `function`; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionPacketDecideRow {
        let object = try jsonObject(json)
        let known: Set<String> = ["function", "kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, function: try String.read(jsonRequired(object, "function")), kind: try String.read(jsonRequired(object, "kind")), value: try OwnedAtomicDecideValue.read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["function", "kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["function"] = `function`.json
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public struct OwnedSessionPacketFilterAggregate: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `function`: String
    public let `kind`: String
    public let `value`: [OwnedAtomicBoolean]
    public init(extensions: [String: JSONValue] = [:], `function`: String = "filter", `kind`: String = "aggregate", `value`: [OwnedAtomicBoolean]) { self.extensions = extensions; self.`function` = `function`; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionPacketFilterAggregate {
        let object = try jsonObject(json)
        let known: Set<String> = ["function", "kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, function: try String.read(jsonRequired(object, "function")), kind: try String.read(jsonRequired(object, "kind")), value: try [OwnedAtomicBoolean].read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["function", "kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["function"] = `function`.json
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public struct OwnedSessionPacketFilterRow: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `function`: String
    public let `kind`: String
    public let `value`: OwnedAtomicBoolean
    public init(extensions: [String: JSONValue] = [:], `function`: String = "filter", `kind`: String = "row", `value`: OwnedAtomicBoolean) { self.extensions = extensions; self.`function` = `function`; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionPacketFilterRow {
        let object = try jsonObject(json)
        let known: Set<String> = ["function", "kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, function: try String.read(jsonRequired(object, "function")), kind: try String.read(jsonRequired(object, "kind")), value: try OwnedAtomicBoolean.read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["function", "kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["function"] = `function`.json
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public struct OwnedSessionPacketFindAggregate: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `function`: String
    public let `kind`: String
    public let `value`: OwnedFind
    public init(extensions: [String: JSONValue] = [:], `function`: String = "find", `kind`: String = "aggregate", `value`: OwnedFind) { self.extensions = extensions; self.`function` = `function`; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionPacketFindAggregate {
        let object = try jsonObject(json)
        let known: Set<String> = ["function", "kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, function: try String.read(jsonRequired(object, "function")), kind: try String.read(jsonRequired(object, "kind")), value: try OwnedFind.read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["function", "kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["function"] = `function`.json
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public struct OwnedSessionPacketObservation: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `function`: OwnedRequestFunction
    public let `kind`: String
    public let `value`: OwnedSessionObservation
    public init(extensions: [String: JSONValue] = [:], `function`: OwnedRequestFunction, `kind`: String = "observation", `value`: OwnedSessionObservation) { self.extensions = extensions; self.`function` = `function`; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionPacketObservation {
        let object = try jsonObject(json)
        let known: Set<String> = ["function", "kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, function: try OwnedRequestFunction.read(jsonRequired(object, "function")), kind: try String.read(jsonRequired(object, "kind")), value: try OwnedSessionObservation.read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["function", "kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["function"] = `function`.json
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public struct OwnedSessionPacketRankAggregate: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `function`: String
    public let `kind`: String
    public let `value`: [OwnedAtomicNonZeroUsize]
    public init(extensions: [String: JSONValue] = [:], `function`: String = "rank", `kind`: String = "aggregate", `value`: [OwnedAtomicNonZeroUsize]) { self.extensions = extensions; self.`function` = `function`; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionPacketRankAggregate {
        let object = try jsonObject(json)
        let known: Set<String> = ["function", "kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, function: try String.read(jsonRequired(object, "function")), kind: try String.read(jsonRequired(object, "kind")), value: try [OwnedAtomicNonZeroUsize].read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["function", "kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["function"] = `function`.json
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public struct OwnedSessionPacketRecognizeAggregate: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `function`: String
    public let `kind`: String
    public let `value`: [OwnedRecognition]
    public init(extensions: [String: JSONValue] = [:], `function`: String = "recognize", `kind`: String = "aggregate", `value`: [OwnedRecognition]) { self.extensions = extensions; self.`function` = `function`; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionPacketRecognizeAggregate {
        let object = try jsonObject(json)
        let known: Set<String> = ["function", "kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, function: try String.read(jsonRequired(object, "function")), kind: try String.read(jsonRequired(object, "kind")), value: try [OwnedRecognition].read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["function", "kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["function"] = `function`.json
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public struct OwnedSessionPacketRelateAggregate: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `function`: String
    public let `kind`: String
    public let `value`: OwnedRelation
    public init(extensions: [String: JSONValue] = [:], `function`: String = "relate", `kind`: String = "aggregate", `value`: OwnedRelation) { self.extensions = extensions; self.`function` = `function`; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionPacketRelateAggregate {
        let object = try jsonObject(json)
        let known: Set<String> = ["function", "kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, function: try String.read(jsonRequired(object, "function")), kind: try String.read(jsonRequired(object, "kind")), value: try OwnedRelation.read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["function", "kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["function"] = `function`.json
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public struct OwnedSessionPacketScoreAggregate: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `function`: String
    public let `kind`: String
    public let `value`: [OwnedAtomicDouble]
    public init(extensions: [String: JSONValue] = [:], `function`: String = "score", `kind`: String = "aggregate", `value`: [OwnedAtomicDouble]) { self.extensions = extensions; self.`function` = `function`; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionPacketScoreAggregate {
        let object = try jsonObject(json)
        let known: Set<String> = ["function", "kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, function: try String.read(jsonRequired(object, "function")), kind: try String.read(jsonRequired(object, "kind")), value: try [OwnedAtomicDouble].read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["function", "kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["function"] = `function`.json
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public struct OwnedSessionPacketScoreRow: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `function`: String
    public let `kind`: String
    public let `value`: OwnedAtomicDouble
    public init(extensions: [String: JSONValue] = [:], `function`: String = "score", `kind`: String = "row", `value`: OwnedAtomicDouble) { self.extensions = extensions; self.`function` = `function`; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionPacketScoreRow {
        let object = try jsonObject(json)
        let known: Set<String> = ["function", "kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, function: try String.read(jsonRequired(object, "function")), kind: try String.read(jsonRequired(object, "kind")), value: try OwnedAtomicDouble.read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["function", "kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["function"] = `function`.json
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public struct OwnedSessionPacketTagAggregate: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `function`: String
    public let `kind`: String
    public let `value`: [OwnedAtomicArrayOfString]
    public init(extensions: [String: JSONValue] = [:], `function`: String = "tag", `kind`: String = "aggregate", `value`: [OwnedAtomicArrayOfString]) { self.extensions = extensions; self.`function` = `function`; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionPacketTagAggregate {
        let object = try jsonObject(json)
        let known: Set<String> = ["function", "kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, function: try String.read(jsonRequired(object, "function")), kind: try String.read(jsonRequired(object, "kind")), value: try [OwnedAtomicArrayOfString].read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["function", "kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["function"] = `function`.json
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public struct OwnedSessionPacketTagRow: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `function`: String
    public let `kind`: String
    public let `value`: OwnedAtomicArrayOfString
    public init(extensions: [String: JSONValue] = [:], `function`: String = "tag", `kind`: String = "row", `value`: OwnedAtomicArrayOfString) { self.extensions = extensions; self.`function` = `function`; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionPacketTagRow {
        let object = try jsonObject(json)
        let known: Set<String> = ["function", "kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, function: try String.read(jsonRequired(object, "function")), kind: try String.read(jsonRequired(object, "kind")), value: try OwnedAtomicArrayOfString.read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["function", "kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["function"] = `function`.json
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public struct OwnedSessionPacketTerminal: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `facts`: Presence<OwnedFacts>
    public let `failure`: Presence<OwnedCallError>
    public let `kind`: String
    public init(extensions: [String: JSONValue] = [:], `facts`: Presence<OwnedFacts> = .absent, `failure`: Presence<OwnedCallError> = .absent, `kind`: String = "terminal") { self.extensions = extensions; self.`facts` = `facts`; self.`failure` = `failure`; self.`kind` = `kind` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionPacketTerminal {
        let object = try jsonObject(json)
        let known: Set<String> = ["facts", "failure", "kind"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, facts: try readPresence(object, "facts", OwnedFacts.self), failure: try readPresence(object, "failure", OwnedCallError.self), kind: try String.read(jsonRequired(object, "kind")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["facts", "failure", "kind"]
        var object = extensions.filter { !known.contains($0.key) }
        writePresence(`facts`, "facts", &object)
        writePresence(`failure`, "failure", &object)
        object["kind"] = `kind`.json
        return .object(object)
    }
}
public indirect enum OwnedSessionProbabilities: JSONRepresentable {
    case `yesNo`(OwnedSessionProbabilitiesYesNo)
    case `named`(OwnedSessionProbabilitiesNamed)
    public static func read(_ json: JSONValue) throws -> OwnedSessionProbabilities {
        let object = try jsonObject(json)
        if object["kind"] == .string("yes_no") { return .`yesNo`(try OwnedSessionProbabilitiesYesNo.read(json)) }
        if object["kind"] == .string("named") { return .`named`(try OwnedSessionProbabilitiesNamed.read(json)) }
        throw JSONConversionError("Unknown OwnedSessionProbabilities alternative")
    }
    public var json: JSONValue {
        switch self {
        case .`yesNo`(let value): return value.json
        case .`named`(let value): return value.json
        }
    }
}
public struct OwnedSessionProbabilitiesNamed: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `value`: [OwnedSessionNamedProbability]
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "named", `value`: [OwnedSessionNamedProbability]) { self.extensions = extensions; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionProbabilitiesNamed {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), value: try [OwnedSessionNamedProbability].read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public struct OwnedSessionProbabilitiesYesNo: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `value`: Double
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "yes_no", `value`: Double) { self.extensions = extensions; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionProbabilitiesYesNo {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), value: try Double.read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public struct OwnedSessionQuestionDetail: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `answerId`: Presence<OwnedAnswerId>
    public let `cached`: Bool
    public let `confidence`: Presence<OwnedSessionQuestionDetailConfidence>
    public let `failedQuestions`: UInt64
    public let `failure`: Presence<OwnedFailure>
    public let `failureId`: Presence<OwnedFailureId>
    public let `input`: Presence<JSONValue>
    public let `inputSource`: Presence<OwnedPhysicalSource>
    public let `inputSources`: [OwnedSessionInputSource]
    public let `inputs`: [JSONValue]
    public let `model`: String
    public let `observations`: [OwnedObservation]
    public let `probabilities`: Presence<OwnedSessionProbabilities>
    public let `question`: OwnedReadableQuestion
    public let `questionSha256`: String
    public let `questionSources`: [OwnedQuestionSource]
    public let `rawPick`: Presence<OwnedSessionQuestionDetailRawPick>
    public let `reportedUsage`: Presence<OwnedUsage>
    public let `requests`: [String]
    public let `requestsSent`: UInt64
    public let `threshold`: Presence<OwnedThreshold>
    public let `url`: String
    public let `usage`: Presence<OwnedTokenUsage>
    public let `value`: Presence<OwnedValue>
    public init(extensions: [String: JSONValue] = [:], `answerId`: Presence<OwnedAnswerId> = .absent, `cached`: Bool, `confidence`: Presence<OwnedSessionQuestionDetailConfidence> = .absent, `failedQuestions`: UInt64, `failure`: Presence<OwnedFailure> = .absent, `failureId`: Presence<OwnedFailureId> = .absent, `input`: Presence<JSONValue> = .absent, `inputSource`: Presence<OwnedPhysicalSource> = .absent, `inputSources`: [OwnedSessionInputSource], `inputs`: [JSONValue], `model`: String, `observations`: [OwnedObservation], `probabilities`: Presence<OwnedSessionProbabilities> = .absent, `question`: OwnedReadableQuestion, `questionSha256`: String, `questionSources`: [OwnedQuestionSource], `rawPick`: Presence<OwnedSessionQuestionDetailRawPick> = .absent, `reportedUsage`: Presence<OwnedUsage> = .absent, `requests`: [String], `requestsSent`: UInt64, `threshold`: Presence<OwnedThreshold> = .absent, `url`: String, `usage`: Presence<OwnedTokenUsage> = .absent, `value`: Presence<OwnedValue> = .absent) { self.extensions = extensions; self.`answerId` = `answerId`; self.`cached` = `cached`; self.`confidence` = `confidence`; self.`failedQuestions` = `failedQuestions`; self.`failure` = `failure`; self.`failureId` = `failureId`; self.`input` = `input`; self.`inputSource` = `inputSource`; self.`inputSources` = `inputSources`; self.`inputs` = `inputs`; self.`model` = `model`; self.`observations` = `observations`; self.`probabilities` = `probabilities`; self.`question` = `question`; self.`questionSha256` = `questionSha256`; self.`questionSources` = `questionSources`; self.`rawPick` = `rawPick`; self.`reportedUsage` = `reportedUsage`; self.`requests` = `requests`; self.`requestsSent` = `requestsSent`; self.`threshold` = `threshold`; self.`url` = `url`; self.`usage` = `usage`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionQuestionDetail {
        let object = try jsonObject(json)
        let known: Set<String> = ["answer_id", "cached", "confidence", "failed_questions", "failure", "failure_id", "input", "input_source", "input_sources", "inputs", "model", "observations", "probabilities", "question", "question_sha256", "question_sources", "raw_pick", "reported_usage", "requests", "requests_sent", "threshold", "url", "usage", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, answerId: try readPresence(object, "answer_id", OwnedAnswerId.self), cached: try Bool.read(jsonRequired(object, "cached")), confidence: try readPresence(object, "confidence", OwnedSessionQuestionDetailConfidence.self), failedQuestions: try UInt64.read(jsonRequired(object, "failed_questions")), failure: try readPresence(object, "failure", OwnedFailure.self), failureId: try readPresence(object, "failure_id", OwnedFailureId.self), input: try readPresence(object, "input", JSONValue.self), inputSource: try readPresence(object, "input_source", OwnedPhysicalSource.self), inputSources: try [OwnedSessionInputSource].read(jsonRequired(object, "input_sources")), inputs: try [JSONValue].read(jsonRequired(object, "inputs")), model: try String.read(jsonRequired(object, "model")), observations: try [OwnedObservation].read(jsonRequired(object, "observations")), probabilities: try readPresence(object, "probabilities", OwnedSessionProbabilities.self), question: try OwnedReadableQuestion.read(jsonRequired(object, "question")), questionSha256: try String.read(jsonRequired(object, "question_sha256")), questionSources: try [OwnedQuestionSource].read(jsonRequired(object, "question_sources")), rawPick: try readPresence(object, "raw_pick", OwnedSessionQuestionDetailRawPick.self), reportedUsage: try readPresence(object, "reported_usage", OwnedUsage.self), requests: try [String].read(jsonRequired(object, "requests")), requestsSent: try UInt64.read(jsonRequired(object, "requests_sent")), threshold: try readPresence(object, "threshold", OwnedThreshold.self), url: try String.read(jsonRequired(object, "url")), usage: try readPresence(object, "usage", OwnedTokenUsage.self), value: try readPresence(object, "value", OwnedValue.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["answer_id", "cached", "confidence", "failed_questions", "failure", "failure_id", "input", "input_source", "input_sources", "inputs", "model", "observations", "probabilities", "question", "question_sha256", "question_sources", "raw_pick", "reported_usage", "requests", "requests_sent", "threshold", "url", "usage", "value"]
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
public indirect enum OwnedSessionQuestionDetailConfidence: JSONRepresentable {
    case alternative0(Double)
    public static func read(_ json: JSONValue) throws -> OwnedSessionQuestionDetailConfidence {
        if let value = try? Double.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedSessionQuestionDetailConfidence value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedSessionQuestionDetailRawPick: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> OwnedSessionQuestionDetailRawPick {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedSessionQuestionDetailRawPick value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public struct OwnedSessionRecognition: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `entities`: [OwnedEntity]
    public let `mode`: OwnedRecognitionMode
    public let `proposals`: Presence<OwnedSessionRecognitionProposals>
    public let `relations`: Presence<OwnedSessionRecognitionRelations>
    public init(extensions: [String: JSONValue] = [:], `entities`: [OwnedEntity], `mode`: OwnedRecognitionMode, `proposals`: Presence<OwnedSessionRecognitionProposals> = .absent, `relations`: Presence<OwnedSessionRecognitionRelations> = .absent) { self.extensions = extensions; self.`entities` = `entities`; self.`mode` = `mode`; self.`proposals` = `proposals`; self.`relations` = `relations` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionRecognition {
        let object = try jsonObject(json)
        let known: Set<String> = ["entities", "mode", "proposals", "relations"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, entities: try [OwnedEntity].read(jsonRequired(object, "entities")), mode: try OwnedRecognitionMode.read(jsonRequired(object, "mode")), proposals: try readPresence(object, "proposals", OwnedSessionRecognitionProposals.self), relations: try readPresence(object, "relations", OwnedSessionRecognitionRelations.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public indirect enum OwnedSessionRecognitionProposals: JSONRepresentable {
    case alternative0([OwnedBoundaryProposal])
    public static func read(_ json: JSONValue) throws -> OwnedSessionRecognitionProposals {
        if let value = try? [OwnedBoundaryProposal].read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedSessionRecognitionProposals value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedSessionRecognitionRelations: JSONRepresentable {
    case alternative0([OwnedRecognitionEdgeDocument])
    public static func read(_ json: JSONValue) throws -> OwnedSessionRecognitionRelations {
        if let value = try? [OwnedRecognitionEdgeDocument].read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedSessionRecognitionRelations value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public struct OwnedSessionRelationEdge: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `either`: Bool
    public let `probability`: Double
    public let `relation`: String
    public let `source`: OwnedEntityDocument
    public let `target`: OwnedEntityDocument
    public init(extensions: [String: JSONValue] = [:], `either`: Bool, `probability`: Double, `relation`: String, `source`: OwnedEntityDocument, `target`: OwnedEntityDocument) { self.extensions = extensions; self.`either` = `either`; self.`probability` = `probability`; self.`relation` = `relation`; self.`source` = `source`; self.`target` = `target` }
    public static func read(_ json: JSONValue) throws -> OwnedSessionRelationEdge {
        let object = try jsonObject(json)
        let known: Set<String> = ["either", "probability", "relation", "source", "target"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, either: try Bool.read(jsonRequired(object, "either")), probability: try Double.read(jsonRequired(object, "probability")), relation: try String.read(jsonRequired(object, "relation")), source: try OwnedEntityDocument.read(jsonRequired(object, "source")), target: try OwnedEntityDocument.read(jsonRequired(object, "target")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public struct OwnedSourceRelationEndpoint: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `file`: OwnedSourceRelationEndpointFile
    public let `firstLine`: Presence<OwnedSourceRelationEndpointFirstLine>
    public let `kind`: String
    public let `lastLine`: Presence<OwnedSourceRelationEndpointLastLine>
    public let `name`: String
    public let `ordinal`: UInt64
    public let `record`: JSONValue
    public init(extensions: [String: JSONValue] = [:], `file`: OwnedSourceRelationEndpointFile, `firstLine`: Presence<OwnedSourceRelationEndpointFirstLine> = .absent, `kind`: String, `lastLine`: Presence<OwnedSourceRelationEndpointLastLine> = .absent, `name`: String, `ordinal`: UInt64, `record`: JSONValue) { self.extensions = extensions; self.`file` = `file`; self.`firstLine` = `firstLine`; self.`kind` = `kind`; self.`lastLine` = `lastLine`; self.`name` = `name`; self.`ordinal` = `ordinal`; self.`record` = `record` }
    public static func read(_ json: JSONValue) throws -> OwnedSourceRelationEndpoint {
        let object = try jsonObject(json)
        let known: Set<String> = ["file", "first_line", "kind", "last_line", "name", "ordinal", "record"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, file: try OwnedSourceRelationEndpointFile.read(jsonRequired(object, "file")), firstLine: try readPresence(object, "first_line", OwnedSourceRelationEndpointFirstLine.self), kind: try String.read(jsonRequired(object, "kind")), lastLine: try readPresence(object, "last_line", OwnedSourceRelationEndpointLastLine.self), name: try String.read(jsonRequired(object, "name")), ordinal: try UInt64.read(jsonRequired(object, "ordinal")), record: try JSONValue.read(jsonRequired(object, "record")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["file", "first_line", "kind", "last_line", "name", "ordinal", "record"]
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
public indirect enum OwnedSourceRelationEndpointFile: JSONRepresentable {
    case alternative0(String)
    case alternative1(JSONValue)
    public static func read(_ json: JSONValue) throws -> OwnedSourceRelationEndpointFile {
        if let value = try? String.read(json) { return .alternative0(value) }
        if json == .null { return .alternative1(.null) }
        throw JSONConversionError("Unknown OwnedSourceRelationEndpointFile value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public indirect enum OwnedSourceRelationEndpointFirstLine: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedSourceRelationEndpointFirstLine {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedSourceRelationEndpointFirstLine value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedSourceRelationEndpointLastLine: JSONRepresentable {
    case alternative0(UInt64)
    public static func read(_ json: JSONValue) throws -> OwnedSourceRelationEndpointLastLine {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown OwnedSourceRelationEndpointLastLine value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public indirect enum OwnedThreshold: JSONRepresentable {
    case alternative0(Double)
    case alternative1(String)
    public static func read(_ json: JSONValue) throws -> OwnedThreshold {
        if let value = try? Double.read(json) { return .alternative0(value) }
        if let value = try? String.read(json) { return .alternative1(value) }
        throw JSONConversionError("Unknown OwnedThreshold value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public struct OwnedTokenBand: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `lower`: UInt64
    public let `upper`: UInt64
    public init(extensions: [String: JSONValue] = [:], `lower`: UInt64, `upper`: UInt64) { self.extensions = extensions; self.`lower` = `lower`; self.`upper` = `upper` }
    public static func read(_ json: JSONValue) throws -> OwnedTokenBand {
        let object = try jsonObject(json)
        let known: Set<String> = ["lower", "upper"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, lower: try UInt64.read(jsonRequired(object, "lower")), upper: try UInt64.read(jsonRequired(object, "upper")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["lower", "upper"]
        var object = extensions.filter { !known.contains($0.key) }
        object["lower"] = `lower`.json
        object["upper"] = `upper`.json
        return .object(object)
    }
}
public struct OwnedTokenUsage: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `inputTokens`: UInt64
    public let `outputTokens`: UInt64
    public init(extensions: [String: JSONValue] = [:], `inputTokens`: UInt64, `outputTokens`: UInt64) { self.extensions = extensions; self.`inputTokens` = `inputTokens`; self.`outputTokens` = `outputTokens` }
    public static func read(_ json: JSONValue) throws -> OwnedTokenUsage {
        let object = try jsonObject(json)
        let known: Set<String> = ["input_tokens", "output_tokens"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, inputTokens: try UInt64.read(jsonRequired(object, "input_tokens")), outputTokens: try UInt64.read(jsonRequired(object, "output_tokens")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["input_tokens", "output_tokens"]
        var object = extensions.filter { !known.contains($0.key) }
        object["input_tokens"] = `inputTokens`.json
        object["output_tokens"] = `outputTokens`.json
        return .object(object)
    }
}
public indirect enum OwnedValue: JSONRepresentable {
    case alternative0(Bool)
    case alternative1(JSONValue)
    case alternative2(String)
    case alternative3([String])
    case alternative4(Double)
    public static func read(_ json: JSONValue) throws -> OwnedValue {
        if let value = try? Bool.read(json) { return .alternative0(value) }
        if json == .null { return .alternative1(.null) }
        if let value = try? String.read(json) { return .alternative2(value) }
        if let value = try? [String].read(json) { return .alternative3(value) }
        if let value = try? Double.read(json) { return .alternative4(value) }
        throw JSONConversionError("Unknown OwnedValue value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        case .alternative2(let value): return value.json
        case .alternative3(let value): return value.json
        case .alternative4(let value): return value.json
        }
    }
}
