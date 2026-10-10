// Generated from the shared Rust graph. Do not edit.
import Foundation
public struct InputAuthoredChoose: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `batch`: Presence<InputAuthoredChooseBatch>
    public let `choose`: InputAuthoredQuestionText
    public let `contextSchema`: Presence<InputAuthoredInputDeclaration>
    public let `itemSchema`: Presence<InputAuthoredInputDeclaration>
    public let `model`: Presence<InputAuthoredName>
    public let `name`: Presence<String>
    public let `on`: Presence<InputAuthoredPointers>
    public let `options`: Presence<InputAuthoredOptions>
    public let `profile`: Presence<InputAuthoredProfile>
    public let `threshold`: Presence<InputAuthoredCut>
    public let `wordingVersion`: Presence<Int64>
    public init(extensions: [String: JSONValue] = [:], `batch`: Presence<InputAuthoredChooseBatch> = .absent, `choose`: InputAuthoredQuestionText, `contextSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `itemSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `model`: Presence<InputAuthoredName> = .absent, `name`: Presence<String> = .absent, `on`: Presence<InputAuthoredPointers> = .absent, `options`: Presence<InputAuthoredOptions> = .absent, `profile`: Presence<InputAuthoredProfile> = .absent, `threshold`: Presence<InputAuthoredCut> = .absent, `wordingVersion`: Presence<Int64> = .absent) { self.extensions = extensions; self.`batch` = `batch`; self.`choose` = `choose`; self.`contextSchema` = `contextSchema`; self.`itemSchema` = `itemSchema`; self.`model` = `model`; self.`name` = `name`; self.`on` = `on`; self.`options` = `options`; self.`profile` = `profile`; self.`threshold` = `threshold`; self.`wordingVersion` = `wordingVersion` }
    public static func read(_ json: JSONValue) throws -> InputAuthoredChoose {
        let object = try jsonObject(json)
        let known: Set<String> = ["batch", "choose", "context_schema", "item_schema", "model", "name", "on", "options", "profile", "threshold", "wording_version"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, batch: try readPresence(object, "batch", InputAuthoredChooseBatch.self), choose: try InputAuthoredQuestionText.read(jsonRequired(object, "choose")), contextSchema: try readPresence(object, "context_schema", InputAuthoredInputDeclaration.self), itemSchema: try readPresence(object, "item_schema", InputAuthoredInputDeclaration.self), model: try readPresence(object, "model", InputAuthoredName.self), name: try readPresence(object, "name", String.self), on: try readPresence(object, "on", InputAuthoredPointers.self), options: try readPresence(object, "options", InputAuthoredOptions.self), profile: try readPresence(object, "profile", InputAuthoredProfile.self), threshold: try readPresence(object, "threshold", InputAuthoredCut.self), wordingVersion: try readPresence(object, "wording_version", Int64.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["batch", "choose", "context_schema", "item_schema", "model", "name", "on", "options", "profile", "threshold", "wording_version"]
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
public indirect enum InputAuthoredChooseBatch: JSONRepresentable {
    case alternative0(String)
    case alternative1(Int64)
    public static func read(_ json: JSONValue) throws -> InputAuthoredChooseBatch {
        if let value = try? String.read(json) { return .alternative0(value) }
        if let value = try? Int64.read(json) { return .alternative1(value) }
        throw JSONConversionError("Unknown InputAuthoredChooseBatch value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public indirect enum InputAuthoredCriterion: JSONRepresentable {
    case alternative0(String)
    case alternative1([String: JSONValue])
    case alternative2([JSONValue])
    case alternative3(JSONValue)
    public static func read(_ json: JSONValue) throws -> InputAuthoredCriterion {
        if let value = try? String.read(json) { return .alternative0(value) }
        if let value = try? [String: JSONValue].read(json) { return .alternative1(value) }
        if let value = try? [JSONValue].read(json) { return .alternative2(value) }
        if json == .null { return .alternative3(.null) }
        throw JSONConversionError("Unknown InputAuthoredCriterion value")
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
public indirect enum InputAuthoredCut: JSONRepresentable {
    case alternative0(Double)
    case alternative1(String)
    public static func read(_ json: JSONValue) throws -> InputAuthoredCut {
        if let value = try? Double.read(json) { return .alternative0(value) }
        if let value = try? String.read(json) { return .alternative1(value) }
        throw JSONConversionError("Unknown InputAuthoredCut value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public struct InputAuthoredDecide: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `batch`: Presence<InputAuthoredDecideBatch>
    public let `contextSchema`: Presence<InputAuthoredInputDeclaration>
    public let `decide`: InputAuthoredQuestionText
    public let `false`: Presence<InputAuthoredCriterion>
    public let `itemSchema`: Presence<InputAuthoredInputDeclaration>
    public let `model`: Presence<InputAuthoredName>
    public let `name`: Presence<String>
    public let `on`: Presence<InputAuthoredPointers>
    public let `profile`: Presence<InputAuthoredProfile>
    public let `threshold`: Presence<InputAuthoredThreshold>
    public let `true`: Presence<InputAuthoredCriterion>
    public let `wordingVersion`: Presence<Int64>
    public init(extensions: [String: JSONValue] = [:], `batch`: Presence<InputAuthoredDecideBatch> = .absent, `contextSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `decide`: InputAuthoredQuestionText, `false`: Presence<InputAuthoredCriterion> = .absent, `itemSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `model`: Presence<InputAuthoredName> = .absent, `name`: Presence<String> = .absent, `on`: Presence<InputAuthoredPointers> = .absent, `profile`: Presence<InputAuthoredProfile> = .absent, `threshold`: Presence<InputAuthoredThreshold> = .absent, `true`: Presence<InputAuthoredCriterion> = .absent, `wordingVersion`: Presence<Int64> = .absent) { self.extensions = extensions; self.`batch` = `batch`; self.`contextSchema` = `contextSchema`; self.`decide` = `decide`; self.`false` = `false`; self.`itemSchema` = `itemSchema`; self.`model` = `model`; self.`name` = `name`; self.`on` = `on`; self.`profile` = `profile`; self.`threshold` = `threshold`; self.`true` = `true`; self.`wordingVersion` = `wordingVersion` }
    public static func read(_ json: JSONValue) throws -> InputAuthoredDecide {
        let object = try jsonObject(json)
        let known: Set<String> = ["batch", "context_schema", "decide", "false", "item_schema", "model", "name", "on", "profile", "threshold", "true", "wording_version"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, batch: try readPresence(object, "batch", InputAuthoredDecideBatch.self), contextSchema: try readPresence(object, "context_schema", InputAuthoredInputDeclaration.self), decide: try InputAuthoredQuestionText.read(jsonRequired(object, "decide")), false: try readPresence(object, "false", InputAuthoredCriterion.self), itemSchema: try readPresence(object, "item_schema", InputAuthoredInputDeclaration.self), model: try readPresence(object, "model", InputAuthoredName.self), name: try readPresence(object, "name", String.self), on: try readPresence(object, "on", InputAuthoredPointers.self), profile: try readPresence(object, "profile", InputAuthoredProfile.self), threshold: try readPresence(object, "threshold", InputAuthoredThreshold.self), true: try readPresence(object, "true", InputAuthoredCriterion.self), wordingVersion: try readPresence(object, "wording_version", Int64.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["batch", "context_schema", "decide", "false", "item_schema", "model", "name", "on", "profile", "threshold", "true", "wording_version"]
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
public indirect enum InputAuthoredDecideBatch: JSONRepresentable {
    case alternative0(String)
    case alternative1(Int64)
    public static func read(_ json: JSONValue) throws -> InputAuthoredDecideBatch {
        if let value = try? String.read(json) { return .alternative0(value) }
        if let value = try? Int64.read(json) { return .alternative1(value) }
        throw JSONConversionError("Unknown InputAuthoredDecideBatch value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public indirect enum InputAuthoredDescription: JSONRepresentable {
    case alternative0(String)
    case alternative1([String: JSONValue])
    case alternative2([JSONValue])
    case alternative3(JSONValue)
    public static func read(_ json: JSONValue) throws -> InputAuthoredDescription {
        if let value = try? String.read(json) { return .alternative0(value) }
        if let value = try? [String: JSONValue].read(json) { return .alternative1(value) }
        if let value = try? [JSONValue].read(json) { return .alternative2(value) }
        if json == .null { return .alternative3(.null) }
        throw JSONConversionError("Unknown InputAuthoredDescription value")
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
public struct InputAuthoredFind: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `contextSchema`: Presence<InputAuthoredInputDeclaration>
    public let `find`: InputAuthoredQuestionText
    public let `itemSchema`: Presence<InputAuthoredInputDeclaration>
    public let `model`: Presence<InputAuthoredName>
    public let `name`: Presence<String>
    public let `on`: Presence<InputAuthoredPointers>
    public let `profile`: Presence<InputAuthoredProfile>
    public let `wordingVersion`: Presence<Int64>
    public init(extensions: [String: JSONValue] = [:], `contextSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `find`: InputAuthoredQuestionText, `itemSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `model`: Presence<InputAuthoredName> = .absent, `name`: Presence<String> = .absent, `on`: Presence<InputAuthoredPointers> = .absent, `profile`: Presence<InputAuthoredProfile> = .absent, `wordingVersion`: Presence<Int64> = .absent) { self.extensions = extensions; self.`contextSchema` = `contextSchema`; self.`find` = `find`; self.`itemSchema` = `itemSchema`; self.`model` = `model`; self.`name` = `name`; self.`on` = `on`; self.`profile` = `profile`; self.`wordingVersion` = `wordingVersion` }
    public static func read(_ json: JSONValue) throws -> InputAuthoredFind {
        let object = try jsonObject(json)
        let known: Set<String> = ["context_schema", "find", "item_schema", "model", "name", "on", "profile", "wording_version"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, contextSchema: try readPresence(object, "context_schema", InputAuthoredInputDeclaration.self), find: try InputAuthoredQuestionText.read(jsonRequired(object, "find")), itemSchema: try readPresence(object, "item_schema", InputAuthoredInputDeclaration.self), model: try readPresence(object, "model", InputAuthoredName.self), name: try readPresence(object, "name", String.self), on: try readPresence(object, "on", InputAuthoredPointers.self), profile: try readPresence(object, "profile", InputAuthoredProfile.self), wordingVersion: try readPresence(object, "wording_version", Int64.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["context_schema", "find", "item_schema", "model", "name", "on", "profile", "wording_version"]
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
public indirect enum InputAuthoredInputDeclaration: JSONRepresentable {
    case `string`(InputAuthoredInputDeclarationString)
    case `object`(InputAuthoredInputDeclarationObject)
    public static func read(_ json: JSONValue) throws -> InputAuthoredInputDeclaration {
        let object = try jsonObject(json)
        if object["type"] == .string("string") { return .`string`(try InputAuthoredInputDeclarationString.read(json)) }
        if object["type"] == .string("object") { return .`object`(try InputAuthoredInputDeclarationObject.read(json)) }
        throw JSONConversionError("Unknown InputAuthoredInputDeclaration alternative")
    }
    public var json: JSONValue {
        switch self {
        case .`string`(let value): return value.json
        case .`object`(let value): return value.json
        }
    }
}
public struct InputAuthoredInputDeclarationObject: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `properties`: [String: InputAuthoredInputProperty]
    public let `required`: Presence<[String]>
    public let `type`: String
    public init(extensions: [String: JSONValue] = [:], `properties`: [String: InputAuthoredInputProperty], `required`: Presence<[String]> = .absent, `type`: String = "object") { self.extensions = extensions; self.`properties` = `properties`; self.`required` = `required`; self.`type` = `type` }
    public static func read(_ json: JSONValue) throws -> InputAuthoredInputDeclarationObject {
        let object = try jsonObject(json)
        let known: Set<String> = ["properties", "required", "type"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, properties: try [String: InputAuthoredInputProperty].read(jsonRequired(object, "properties")), required: try readPresence(object, "required", [String].self), type: try String.read(jsonRequired(object, "type")))
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
public struct InputAuthoredInputDeclarationString: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `type`: String
    public init(extensions: [String: JSONValue] = [:], `type`: String = "string") { self.extensions = extensions; self.`type` = `type` }
    public static func read(_ json: JSONValue) throws -> InputAuthoredInputDeclarationString {
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
public indirect enum InputAuthoredInputProperty: JSONRepresentable {
    case `string`(InputAuthoredInputPropertyString)
    case `number`(InputAuthoredInputPropertyNumber)
    case `boolean`(InputAuthoredInputPropertyBoolean)
    case `array`(InputAuthoredInputPropertyArray)
    public static func read(_ json: JSONValue) throws -> InputAuthoredInputProperty {
        let object = try jsonObject(json)
        if object["type"] == .string("string") { return .`string`(try InputAuthoredInputPropertyString.read(json)) }
        if object["type"] == .string("number") { return .`number`(try InputAuthoredInputPropertyNumber.read(json)) }
        if object["type"] == .string("boolean") { return .`boolean`(try InputAuthoredInputPropertyBoolean.read(json)) }
        if object["type"] == .string("array") { return .`array`(try InputAuthoredInputPropertyArray.read(json)) }
        throw JSONConversionError("Unknown InputAuthoredInputProperty alternative")
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
public struct InputAuthoredInputPropertyArray: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `items`: InputAuthoredInputPropertyArrayItems
    public let `type`: String
    public init(extensions: [String: JSONValue] = [:], `items`: InputAuthoredInputPropertyArrayItems, `type`: String = "array") { self.extensions = extensions; self.`items` = `items`; self.`type` = `type` }
    public static func read(_ json: JSONValue) throws -> InputAuthoredInputPropertyArray {
        let object = try jsonObject(json)
        let known: Set<String> = ["items", "type"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, items: try InputAuthoredInputPropertyArrayItems.read(jsonRequired(object, "items")), type: try String.read(jsonRequired(object, "type")))
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
public struct InputAuthoredInputPropertyArrayItems: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `type`: String
    public init(extensions: [String: JSONValue] = [:], `type`: String = "string") { self.extensions = extensions; self.`type` = `type` }
    public static func read(_ json: JSONValue) throws -> InputAuthoredInputPropertyArrayItems {
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
public struct InputAuthoredInputPropertyBoolean: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `type`: String
    public init(extensions: [String: JSONValue] = [:], `type`: String = "boolean") { self.extensions = extensions; self.`type` = `type` }
    public static func read(_ json: JSONValue) throws -> InputAuthoredInputPropertyBoolean {
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
public struct InputAuthoredInputPropertyNumber: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `type`: String
    public init(extensions: [String: JSONValue] = [:], `type`: String = "number") { self.extensions = extensions; self.`type` = `type` }
    public static func read(_ json: JSONValue) throws -> InputAuthoredInputPropertyNumber {
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
public struct InputAuthoredInputPropertyString: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `type`: String
    public init(extensions: [String: JSONValue] = [:], `type`: String = "string") { self.extensions = extensions; self.`type` = `type` }
    public static func read(_ json: JSONValue) throws -> InputAuthoredInputPropertyString {
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
public indirect enum InputAuthoredLabels: JSONRepresentable {
    case alternative0([InputAuthoredName])
    case alternative1([String: InputAuthoredDescription])
    public static func read(_ json: JSONValue) throws -> InputAuthoredLabels {
        if let value = try? [InputAuthoredName].read(json) { return .alternative0(value) }
        if let value = try? [String: InputAuthoredDescription].read(json) { return .alternative1(value) }
        throw JSONConversionError("Unknown InputAuthoredLabels value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public indirect enum InputAuthoredLevels: JSONRepresentable {
    case alternative0([InputAuthoredName])
    case alternative1([String: InputAuthoredCriterion])
    public static func read(_ json: JSONValue) throws -> InputAuthoredLevels {
        if let value = try? [InputAuthoredName].read(json) { return .alternative0(value) }
        if let value = try? [String: InputAuthoredCriterion].read(json) { return .alternative1(value) }
        throw JSONConversionError("Unknown InputAuthoredLevels value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public typealias InputAuthoredName = String
public indirect enum InputAuthoredOptions: JSONRepresentable {
    case alternative0([InputAuthoredName])
    case alternative1([String: InputAuthoredDescription])
    public static func read(_ json: JSONValue) throws -> InputAuthoredOptions {
        if let value = try? [InputAuthoredName].read(json) { return .alternative0(value) }
        if let value = try? [String: InputAuthoredDescription].read(json) { return .alternative1(value) }
        throw JSONConversionError("Unknown InputAuthoredOptions value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public indirect enum InputAuthoredPointers: JSONRepresentable {
    case alternative0(String)
    case alternative1([String])
    public static func read(_ json: JSONValue) throws -> InputAuthoredPointers {
        if let value = try? String.read(json) { return .alternative0(value) }
        if let value = try? [String].read(json) { return .alternative1(value) }
        throw JSONConversionError("Unknown InputAuthoredPointers value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public typealias InputAuthoredProfile = String
public indirect enum InputAuthoredQuestionText: JSONRepresentable {
    case alternative0(String)
    case alternative1([String: JSONValue])
    case alternative2([JSONValue])
    public static func read(_ json: JSONValue) throws -> InputAuthoredQuestionText {
        if let value = try? String.read(json) { return .alternative0(value) }
        if let value = try? [String: JSONValue].read(json) { return .alternative1(value) }
        if let value = try? [JSONValue].read(json) { return .alternative2(value) }
        throw JSONConversionError("Unknown InputAuthoredQuestionText value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        case .alternative2(let value): return value.json
        }
    }
}
public struct InputAuthoredRelate: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `contextSchema`: Presence<InputAuthoredInputDeclaration>
    public let `itemSchema`: Presence<InputAuthoredInputDeclaration>
    public let `model`: Presence<InputAuthoredName>
    public let `name`: Presence<String>
    public let `profile`: Presence<InputAuthoredProfile>
    public let `relate`: InputAuthoredRelateRelate
    public let `threshold`: Presence<InputAuthoredCut>
    public let `version`: Int64
    public let `wordingVersion`: Presence<Int64>
    public init(extensions: [String: JSONValue] = [:], `contextSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `itemSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `model`: Presence<InputAuthoredName> = .absent, `name`: Presence<String> = .absent, `profile`: Presence<InputAuthoredProfile> = .absent, `relate`: InputAuthoredRelateRelate, `threshold`: Presence<InputAuthoredCut> = .absent, `version`: Int64 = 1, `wordingVersion`: Presence<Int64> = .absent) { self.extensions = extensions; self.`contextSchema` = `contextSchema`; self.`itemSchema` = `itemSchema`; self.`model` = `model`; self.`name` = `name`; self.`profile` = `profile`; self.`relate` = `relate`; self.`threshold` = `threshold`; self.`version` = `version`; self.`wordingVersion` = `wordingVersion` }
    public static func read(_ json: JSONValue) throws -> InputAuthoredRelate {
        let object = try jsonObject(json)
        let known: Set<String> = ["context_schema", "item_schema", "model", "name", "profile", "relate", "threshold", "version", "wording_version"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, contextSchema: try readPresence(object, "context_schema", InputAuthoredInputDeclaration.self), itemSchema: try readPresence(object, "item_schema", InputAuthoredInputDeclaration.self), model: try readPresence(object, "model", InputAuthoredName.self), name: try readPresence(object, "name", String.self), profile: try readPresence(object, "profile", InputAuthoredProfile.self), relate: try InputAuthoredRelateRelate.read(jsonRequired(object, "relate")), threshold: try readPresence(object, "threshold", InputAuthoredCut.self), version: try Int64.read(jsonRequired(object, "version")), wordingVersion: try readPresence(object, "wording_version", Int64.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["context_schema", "item_schema", "model", "name", "profile", "relate", "threshold", "version", "wording_version"]
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
public struct InputAuthoredRelateRelate: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `fields`: Presence<InputAuthoredRelateRelateFields>
    public let `relations`: [InputAuthoredRelation]
    public init(extensions: [String: JSONValue] = [:], `fields`: Presence<InputAuthoredRelateRelateFields> = .absent, `relations`: [InputAuthoredRelation]) { self.extensions = extensions; self.`fields` = `fields`; self.`relations` = `relations` }
    public static func read(_ json: JSONValue) throws -> InputAuthoredRelateRelate {
        let object = try jsonObject(json)
        let known: Set<String> = ["fields", "relations"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, fields: try readPresence(object, "fields", InputAuthoredRelateRelateFields.self), relations: try [InputAuthoredRelation].read(jsonRequired(object, "relations")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["fields", "relations"]
        var object = extensions.filter { !known.contains($0.key) }
        writePresence(`fields`, "fields", &object)
        object["relations"] = `relations`.json
        return .object(object)
    }
}
public struct InputAuthoredRelateRelateFields: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `name`: String
    public init(extensions: [String: JSONValue] = [:], `kind`: String, `name`: String) { self.extensions = extensions; self.`kind` = `kind`; self.`name` = `name` }
    public static func read(_ json: JSONValue) throws -> InputAuthoredRelateRelateFields {
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
public struct InputAuthoredRelation: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `either`: Presence<Bool>
    public let `name`: InputAuthoredName
    public let `reads`: Presence<InputAuthoredName>
    public let `single`: Presence<Bool>
    public let `source`: Presence<InputAuthoredName>
    public let `target`: Presence<InputAuthoredName>
    public init(extensions: [String: JSONValue] = [:], `either`: Presence<Bool> = .absent, `name`: InputAuthoredName, `reads`: Presence<InputAuthoredName> = .absent, `single`: Presence<Bool> = .absent, `source`: Presence<InputAuthoredName> = .absent, `target`: Presence<InputAuthoredName> = .absent) { self.extensions = extensions; self.`either` = `either`; self.`name` = `name`; self.`reads` = `reads`; self.`single` = `single`; self.`source` = `source`; self.`target` = `target` }
    public static func read(_ json: JSONValue) throws -> InputAuthoredRelation {
        let object = try jsonObject(json)
        let known: Set<String> = ["either", "name", "reads", "single", "source", "target"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, either: try readPresence(object, "either", Bool.self), name: try InputAuthoredName.read(jsonRequired(object, "name")), reads: try readPresence(object, "reads", InputAuthoredName.self), single: try readPresence(object, "single", Bool.self), source: try readPresence(object, "source", InputAuthoredName.self), target: try readPresence(object, "target", InputAuthoredName.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public struct InputAuthoredScore: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `batch`: Presence<InputAuthoredScoreBatch>
    public let `contextSchema`: Presence<InputAuthoredInputDeclaration>
    public let `itemSchema`: Presence<InputAuthoredInputDeclaration>
    public let `levels`: Presence<InputAuthoredLevels>
    public let `model`: Presence<InputAuthoredName>
    public let `name`: Presence<String>
    public let `on`: Presence<InputAuthoredPointers>
    public let `profile`: Presence<InputAuthoredProfile>
    public let `score`: InputAuthoredQuestionText
    public let `wordingVersion`: Presence<Int64>
    public init(extensions: [String: JSONValue] = [:], `batch`: Presence<InputAuthoredScoreBatch> = .absent, `contextSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `itemSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `levels`: Presence<InputAuthoredLevels> = .absent, `model`: Presence<InputAuthoredName> = .absent, `name`: Presence<String> = .absent, `on`: Presence<InputAuthoredPointers> = .absent, `profile`: Presence<InputAuthoredProfile> = .absent, `score`: InputAuthoredQuestionText, `wordingVersion`: Presence<Int64> = .absent) { self.extensions = extensions; self.`batch` = `batch`; self.`contextSchema` = `contextSchema`; self.`itemSchema` = `itemSchema`; self.`levels` = `levels`; self.`model` = `model`; self.`name` = `name`; self.`on` = `on`; self.`profile` = `profile`; self.`score` = `score`; self.`wordingVersion` = `wordingVersion` }
    public static func read(_ json: JSONValue) throws -> InputAuthoredScore {
        let object = try jsonObject(json)
        let known: Set<String> = ["batch", "context_schema", "item_schema", "levels", "model", "name", "on", "profile", "score", "wording_version"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, batch: try readPresence(object, "batch", InputAuthoredScoreBatch.self), contextSchema: try readPresence(object, "context_schema", InputAuthoredInputDeclaration.self), itemSchema: try readPresence(object, "item_schema", InputAuthoredInputDeclaration.self), levels: try readPresence(object, "levels", InputAuthoredLevels.self), model: try readPresence(object, "model", InputAuthoredName.self), name: try readPresence(object, "name", String.self), on: try readPresence(object, "on", InputAuthoredPointers.self), profile: try readPresence(object, "profile", InputAuthoredProfile.self), score: try InputAuthoredQuestionText.read(jsonRequired(object, "score")), wordingVersion: try readPresence(object, "wording_version", Int64.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["batch", "context_schema", "item_schema", "levels", "model", "name", "on", "profile", "score", "wording_version"]
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
public indirect enum InputAuthoredScoreBatch: JSONRepresentable {
    case alternative0(String)
    case alternative1(Int64)
    public static func read(_ json: JSONValue) throws -> InputAuthoredScoreBatch {
        if let value = try? String.read(json) { return .alternative0(value) }
        if let value = try? Int64.read(json) { return .alternative1(value) }
        throw JSONConversionError("Unknown InputAuthoredScoreBatch value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public struct InputAuthoredTag: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `batch`: Presence<InputAuthoredTagBatch>
    public let `contextSchema`: Presence<InputAuthoredInputDeclaration>
    public let `itemSchema`: Presence<InputAuthoredInputDeclaration>
    public let `labels`: Presence<InputAuthoredLabels>
    public let `model`: Presence<InputAuthoredName>
    public let `name`: Presence<String>
    public let `on`: Presence<InputAuthoredPointers>
    public let `profile`: Presence<InputAuthoredProfile>
    public let `tag`: InputAuthoredQuestionText
    public let `threshold`: Presence<InputAuthoredCut>
    public let `wordingVersion`: Presence<Int64>
    public init(extensions: [String: JSONValue] = [:], `batch`: Presence<InputAuthoredTagBatch> = .absent, `contextSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `itemSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `labels`: Presence<InputAuthoredLabels> = .absent, `model`: Presence<InputAuthoredName> = .absent, `name`: Presence<String> = .absent, `on`: Presence<InputAuthoredPointers> = .absent, `profile`: Presence<InputAuthoredProfile> = .absent, `tag`: InputAuthoredQuestionText, `threshold`: Presence<InputAuthoredCut> = .absent, `wordingVersion`: Presence<Int64> = .absent) { self.extensions = extensions; self.`batch` = `batch`; self.`contextSchema` = `contextSchema`; self.`itemSchema` = `itemSchema`; self.`labels` = `labels`; self.`model` = `model`; self.`name` = `name`; self.`on` = `on`; self.`profile` = `profile`; self.`tag` = `tag`; self.`threshold` = `threshold`; self.`wordingVersion` = `wordingVersion` }
    public static func read(_ json: JSONValue) throws -> InputAuthoredTag {
        let object = try jsonObject(json)
        let known: Set<String> = ["batch", "context_schema", "item_schema", "labels", "model", "name", "on", "profile", "tag", "threshold", "wording_version"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, batch: try readPresence(object, "batch", InputAuthoredTagBatch.self), contextSchema: try readPresence(object, "context_schema", InputAuthoredInputDeclaration.self), itemSchema: try readPresence(object, "item_schema", InputAuthoredInputDeclaration.self), labels: try readPresence(object, "labels", InputAuthoredLabels.self), model: try readPresence(object, "model", InputAuthoredName.self), name: try readPresence(object, "name", String.self), on: try readPresence(object, "on", InputAuthoredPointers.self), profile: try readPresence(object, "profile", InputAuthoredProfile.self), tag: try InputAuthoredQuestionText.read(jsonRequired(object, "tag")), threshold: try readPresence(object, "threshold", InputAuthoredCut.self), wordingVersion: try readPresence(object, "wording_version", Int64.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["batch", "context_schema", "item_schema", "labels", "model", "name", "on", "profile", "tag", "threshold", "wording_version"]
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
public indirect enum InputAuthoredTagBatch: JSONRepresentable {
    case alternative0(String)
    case alternative1(Int64)
    public static func read(_ json: JSONValue) throws -> InputAuthoredTagBatch {
        if let value = try? String.read(json) { return .alternative0(value) }
        if let value = try? Int64.read(json) { return .alternative1(value) }
        throw JSONConversionError("Unknown InputAuthoredTagBatch value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public indirect enum InputAuthoredThreshold: JSONRepresentable {
    case alternative0(Double)
    case alternative1(String)
    public static func read(_ json: JSONValue) throws -> InputAuthoredThreshold {
        if let value = try? Double.read(json) { return .alternative0(value) }
        if let value = try? String.read(json) { return .alternative1(value) }
        throw JSONConversionError("Unknown InputAuthoredThreshold value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public indirect enum InputCacheDocument: JSONRepresentable {
    case alternative0(String)
    case alternative1(InputDisabledCache)
    public static func read(_ json: JSONValue) throws -> InputCacheDocument {
        if let value = try? String.read(json) { return .alternative0(value) }
        if let value = try? InputDisabledCache.read(json) { return .alternative1(value) }
        throw JSONConversionError("Unknown InputCacheDocument value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public indirect enum InputContextSchema: JSONRepresentable {
    case alternative0(String)
    case alternative1([String: JSONValue])
    public static func read(_ json: JSONValue) throws -> InputContextSchema {
        if let value = try? String.read(json) { return .alternative0(value) }
        if let value = try? [String: JSONValue].read(json) { return .alternative1(value) }
        throw JSONConversionError("Unknown InputContextSchema value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public typealias InputDisabledCache = Bool
public struct InputEngineSettings: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `backend`: Presence<String>
    public let `baseUrl`: Presence<String>
    public let `batch`: Presence<InputRequestBatch>
    public let `cache`: Presence<InputCacheDocument>
    public let `maxEstimatedInputTokensTotal`: Presence<InputEngineSettingsMaxEstimatedInputTokensTotal>
    public let `maxRequestBytes`: Presence<UInt64>
    public let `maxRequests`: Presence<InputEngineSettingsMaxRequests>
    public let `maxRequestsTotal`: Presence<InputEngineSettingsMaxRequestsTotal>
    public let `maxRetries`: Presence<UInt64>
    public let `model`: Presence<String>
    public let `profile`: Presence<String>
    public let `proxy`: Presence<JSONValue>
    public let `record`: Presence<String>
    public let `refreshCache`: Presence<Bool>
    public let `replay`: Presence<String>
    public let `throttle`: Presence<UInt64>
    public let `timeout`: Presence<UInt64>
    public let `usdPerMillionInput`: Presence<String>
    public let `usdPerMillionOutput`: Presence<String>
    public init(extensions: [String: JSONValue] = [:], `backend`: Presence<String> = .absent, `baseUrl`: Presence<String> = .absent, `batch`: Presence<InputRequestBatch> = .absent, `cache`: Presence<InputCacheDocument> = .absent, `maxEstimatedInputTokensTotal`: Presence<InputEngineSettingsMaxEstimatedInputTokensTotal> = .absent, `maxRequestBytes`: Presence<UInt64> = .absent, `maxRequests`: Presence<InputEngineSettingsMaxRequests> = .absent, `maxRequestsTotal`: Presence<InputEngineSettingsMaxRequestsTotal> = .absent, `maxRetries`: Presence<UInt64> = .absent, `model`: Presence<String> = .absent, `profile`: Presence<String> = .absent, `proxy`: Presence<JSONValue> = .absent, `record`: Presence<String> = .absent, `refreshCache`: Presence<Bool> = .absent, `replay`: Presence<String> = .absent, `throttle`: Presence<UInt64> = .absent, `timeout`: Presence<UInt64> = .absent, `usdPerMillionInput`: Presence<String> = .absent, `usdPerMillionOutput`: Presence<String> = .absent) { self.extensions = extensions; self.`backend` = `backend`; self.`baseUrl` = `baseUrl`; self.`batch` = `batch`; self.`cache` = `cache`; self.`maxEstimatedInputTokensTotal` = `maxEstimatedInputTokensTotal`; self.`maxRequestBytes` = `maxRequestBytes`; self.`maxRequests` = `maxRequests`; self.`maxRequestsTotal` = `maxRequestsTotal`; self.`maxRetries` = `maxRetries`; self.`model` = `model`; self.`profile` = `profile`; self.`proxy` = `proxy`; self.`record` = `record`; self.`refreshCache` = `refreshCache`; self.`replay` = `replay`; self.`throttle` = `throttle`; self.`timeout` = `timeout`; self.`usdPerMillionInput` = `usdPerMillionInput`; self.`usdPerMillionOutput` = `usdPerMillionOutput` }
    public static func read(_ json: JSONValue) throws -> InputEngineSettings {
        let object = try jsonObject(json)
        let known: Set<String> = ["backend", "base_url", "batch", "cache", "max_estimated_input_tokens_total", "max_request_bytes", "max_requests", "max_requests_total", "max_retries", "model", "profile", "proxy", "record", "refresh_cache", "replay", "throttle", "timeout", "usd_per_million_input", "usd_per_million_output"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, backend: try readPresence(object, "backend", String.self), baseUrl: try readPresence(object, "base_url", String.self), batch: try readPresence(object, "batch", InputRequestBatch.self), cache: try readPresence(object, "cache", InputCacheDocument.self), maxEstimatedInputTokensTotal: try readPresence(object, "max_estimated_input_tokens_total", InputEngineSettingsMaxEstimatedInputTokensTotal.self), maxRequestBytes: try readPresence(object, "max_request_bytes", UInt64.self), maxRequests: try readPresence(object, "max_requests", InputEngineSettingsMaxRequests.self), maxRequestsTotal: try readPresence(object, "max_requests_total", InputEngineSettingsMaxRequestsTotal.self), maxRetries: try readPresence(object, "max_retries", UInt64.self), model: try readPresence(object, "model", String.self), profile: try readPresence(object, "profile", String.self), proxy: try readPresence(object, "proxy", JSONValue.self), record: try readPresence(object, "record", String.self), refreshCache: try readPresence(object, "refresh_cache", Bool.self), replay: try readPresence(object, "replay", String.self), throttle: try readPresence(object, "throttle", UInt64.self), timeout: try readPresence(object, "timeout", UInt64.self), usdPerMillionInput: try readPresence(object, "usd_per_million_input", String.self), usdPerMillionOutput: try readPresence(object, "usd_per_million_output", String.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["backend", "base_url", "batch", "cache", "max_estimated_input_tokens_total", "max_request_bytes", "max_requests", "max_requests_total", "max_retries", "model", "profile", "proxy", "record", "refresh_cache", "replay", "throttle", "timeout", "usd_per_million_input", "usd_per_million_output"]
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
public indirect enum InputEngineSettingsMaxEstimatedInputTokensTotal: JSONRepresentable {
    case alternative0(UInt64)
    case alternative1(JSONValue)
    public static func read(_ json: JSONValue) throws -> InputEngineSettingsMaxEstimatedInputTokensTotal {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        if json == .null { return .alternative1(.null) }
        throw JSONConversionError("Unknown InputEngineSettingsMaxEstimatedInputTokensTotal value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public indirect enum InputEngineSettingsMaxRequests: JSONRepresentable {
    case alternative0(UInt64)
    case alternative1(JSONValue)
    public static func read(_ json: JSONValue) throws -> InputEngineSettingsMaxRequests {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        if json == .null { return .alternative1(.null) }
        throw JSONConversionError("Unknown InputEngineSettingsMaxRequests value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public indirect enum InputEngineSettingsMaxRequestsTotal: JSONRepresentable {
    case alternative0(UInt64)
    case alternative1(JSONValue)
    public static func read(_ json: JSONValue) throws -> InputEngineSettingsMaxRequestsTotal {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        if json == .null { return .alternative1(.null) }
        throw JSONConversionError("Unknown InputEngineSettingsMaxRequestsTotal value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public indirect enum InputImageMedia: JSONRepresentable {
    case alternative0(String)
    case alternative1(String)
    public static func read(_ json: JSONValue) throws -> InputImageMedia {
        if let value = try? String.read(json) { return .alternative0(value) }
        if let value = try? String.read(json) { return .alternative1(value) }
        throw JSONConversionError("Unknown InputImageMedia value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public struct InputOptionSchema: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `description`: Presence<JSONValue>
    public let `name`: String
    public init(extensions: [String: JSONValue] = [:], `description`: Presence<JSONValue> = .absent, `name`: String) { self.extensions = extensions; self.`description` = `description`; self.`name` = `name` }
    public static func read(_ json: JSONValue) throws -> InputOptionSchema {
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
public indirect enum InputReaderMedia: JSONRepresentable {
    case alternative0(String)
    case alternative1(String)
    public static func read(_ json: JSONValue) throws -> InputReaderMedia {
        if let value = try? String.read(json) { return .alternative0(value) }
        if let value = try? String.read(json) { return .alternative1(value) }
        throw JSONConversionError("Unknown InputReaderMedia value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public indirect enum InputRecognitionExample: JSONRepresentable {
    case alternative0(String)
    case alternative1(InputRecognitionExampleText)
    public static func read(_ json: JSONValue) throws -> InputRecognitionExample {
        if let value = try? String.read(json) { return .alternative0(value) }
        if let value = try? InputRecognitionExampleText.read(json) { return .alternative1(value) }
        throw JSONConversionError("Unknown InputRecognitionExample value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public struct InputRecognitionExampleEntity: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `end`: UInt64
    public let `kind`: String
    public let `start`: UInt64
    public init(extensions: [String: JSONValue] = [:], `end`: UInt64, `kind`: String, `start`: UInt64) { self.extensions = extensions; self.`end` = `end`; self.`kind` = `kind`; self.`start` = `start` }
    public static func read(_ json: JSONValue) throws -> InputRecognitionExampleEntity {
        let object = try jsonObject(json)
        let known: Set<String> = ["end", "kind", "start"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, end: try UInt64.read(jsonRequired(object, "end")), kind: try String.read(jsonRequired(object, "kind")), start: try UInt64.read(jsonRequired(object, "start")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["end", "kind", "start"]
        var object = extensions.filter { !known.contains($0.key) }
        object["end"] = `end`.json
        object["kind"] = `kind`.json
        object["start"] = `start`.json
        return .object(object)
    }
}
public struct InputRecognitionExampleText: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `entities`: [InputRecognitionExampleEntity]
    public let `kinds`: Presence<[String]>
    public let `text`: String
    public init(extensions: [String: JSONValue] = [:], `entities`: [InputRecognitionExampleEntity], `kinds`: Presence<[String]> = .absent, `text`: String) { self.extensions = extensions; self.`entities` = `entities`; self.`kinds` = `kinds`; self.`text` = `text` }
    public static func read(_ json: JSONValue) throws -> InputRecognitionExampleText {
        let object = try jsonObject(json)
        let known: Set<String> = ["entities", "kinds", "text"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, entities: try [InputRecognitionExampleEntity].read(jsonRequired(object, "entities")), kinds: try readPresence(object, "kinds", [String].self), text: try String.read(jsonRequired(object, "text")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["entities", "kinds", "text"]
        var object = extensions.filter { !known.contains($0.key) }
        object["entities"] = `entities`.json
        writePresence(`kinds`, "kinds", &object)
        object["text"] = `text`.json
        return .object(object)
    }
}
public indirect enum InputRecognitionMode: JSONRepresentable {
    case alternative0(String)
    case alternative1(String)
    public static func read(_ json: JSONValue) throws -> InputRecognitionMode {
        if let value = try? String.read(json) { return .alternative0(value) }
        if let value = try? String.read(json) { return .alternative1(value) }
        throw JSONConversionError("Unknown InputRecognitionMode value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public struct InputRecognitionSeedSpan: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `end`: UInt64
    public let `kind`: Presence<String>
    public let `start`: UInt64
    public init(extensions: [String: JSONValue] = [:], `end`: UInt64, `kind`: Presence<String> = .absent, `start`: UInt64) { self.extensions = extensions; self.`end` = `end`; self.`kind` = `kind`; self.`start` = `start` }
    public static func read(_ json: JSONValue) throws -> InputRecognitionSeedSpan {
        let object = try jsonObject(json)
        let known: Set<String> = ["end", "kind", "start"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, end: try UInt64.read(jsonRequired(object, "end")), kind: try readPresence(object, "kind", String.self), start: try UInt64.read(jsonRequired(object, "start")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["end", "kind", "start"]
        var object = extensions.filter { !known.contains($0.key) }
        object["end"] = `end`.json
        writePresence(`kind`, "kind", &object)
        object["start"] = `start`.json
        return .object(object)
    }
}
public struct InputRecognitionStageContext: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `boundary`: Presence<String>
    public let `kindEdge`: Presence<String>
    public let `relation`: Presence<String>
    public init(extensions: [String: JSONValue] = [:], `boundary`: Presence<String> = .absent, `kindEdge`: Presence<String> = .absent, `relation`: Presence<String> = .absent) { self.extensions = extensions; self.`boundary` = `boundary`; self.`kindEdge` = `kindEdge`; self.`relation` = `relation` }
    public static func read(_ json: JSONValue) throws -> InputRecognitionStageContext {
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
public struct InputRequest: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `call`: InputRequestCall
    public let `schema`: InputRequestVersion
    public init(extensions: [String: JSONValue] = [:], `call`: InputRequestCall, `schema`: InputRequestVersion) { self.extensions = extensions; self.`call` = `call`; self.`schema` = `schema` }
    public static func read(_ json: JSONValue) throws -> InputRequest {
        let object = try jsonObject(json)
        let known: Set<String> = ["call", "schema"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, call: try InputRequestCall.read(jsonRequired(object, "call")), schema: try InputRequestVersion.read(jsonRequired(object, "schema")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["call", "schema"]
        var object = extensions.filter { !known.contains($0.key) }
        object["call"] = `call`.json
        object["schema"] = `schema`.json
        return .object(object)
    }
}
public indirect enum InputRequestBatch: JSONRepresentable {
    case alternative0(UInt64)
    case alternative1(String)
    public static func read(_ json: JSONValue) throws -> InputRequestBatch {
        if let value = try? UInt64.read(json) { return .alternative0(value) }
        if let value = try? String.read(json) { return .alternative1(value) }
        throw JSONConversionError("Unknown InputRequestBatch value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public indirect enum InputRequestCall: JSONRepresentable {
    case `decide`(InputRequestCallDecide)
    case `choose`(InputRequestCallChoose)
    case `tag`(InputRequestCallTag)
    case `score`(InputRequestCallScore)
    case `filter`(InputRequestCallFilter)
    case `rank`(InputRequestCallRank)
    case `find`(InputRequestCallFind)
    case `annotate`(InputRequestCallAnnotate)
    case `recognize`(InputRequestCallRecognize)
    case `relate`(InputRequestCallRelate)
    public static func read(_ json: JSONValue) throws -> InputRequestCall {
        let object = try jsonObject(json)
        if object["function"] == .string("decide") { return .`decide`(try InputRequestCallDecide.read(json)) }
        if object["function"] == .string("choose") { return .`choose`(try InputRequestCallChoose.read(json)) }
        if object["function"] == .string("tag") { return .`tag`(try InputRequestCallTag.read(json)) }
        if object["function"] == .string("score") { return .`score`(try InputRequestCallScore.read(json)) }
        if object["function"] == .string("filter") { return .`filter`(try InputRequestCallFilter.read(json)) }
        if object["function"] == .string("rank") { return .`rank`(try InputRequestCallRank.read(json)) }
        if object["function"] == .string("find") { return .`find`(try InputRequestCallFind.read(json)) }
        if object["function"] == .string("annotate") { return .`annotate`(try InputRequestCallAnnotate.read(json)) }
        if object["function"] == .string("recognize") { return .`recognize`(try InputRequestCallRecognize.read(json)) }
        if object["function"] == .string("relate") { return .`relate`(try InputRequestCallRelate.read(json)) }
        throw JSONConversionError("Unknown InputRequestCall alternative")
    }
    public var json: JSONValue {
        switch self {
        case .`decide`(let value): return value.json
        case .`choose`(let value): return value.json
        case .`tag`(let value): return value.json
        case .`score`(let value): return value.json
        case .`filter`(let value): return value.json
        case .`rank`(let value): return value.json
        case .`find`(let value): return value.json
        case .`annotate`(let value): return value.json
        case .`recognize`(let value): return value.json
        case .`relate`(let value): return value.json
        }
    }
}
public struct InputRequestCallAnnotate: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `function`: String
    public let `input`: InputRequestInput
    public let `options`: Presence<InputRequestOptions>
    public let `question`: InputRequestQuestion
    public init(extensions: [String: JSONValue] = [:], `function`: String = "annotate", `input`: InputRequestInput, `options`: Presence<InputRequestOptions> = .absent, `question`: InputRequestQuestion) { self.extensions = extensions; self.`function` = `function`; self.`input` = `input`; self.`options` = `options`; self.`question` = `question` }
    public static func read(_ json: JSONValue) throws -> InputRequestCallAnnotate {
        let object = try jsonObject(json)
        let known: Set<String> = ["function", "input", "options", "question"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, function: try String.read(jsonRequired(object, "function")), input: try InputRequestInput.read(jsonRequired(object, "input")), options: try readPresence(object, "options", InputRequestOptions.self), question: try InputRequestQuestion.read(jsonRequired(object, "question")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public struct InputRequestCallChoose: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `function`: String
    public let `input`: InputRequestInput
    public let `options`: Presence<InputRequestOptions>
    public let `question`: InputRequestQuestion
    public init(extensions: [String: JSONValue] = [:], `function`: String = "choose", `input`: InputRequestInput, `options`: Presence<InputRequestOptions> = .absent, `question`: InputRequestQuestion) { self.extensions = extensions; self.`function` = `function`; self.`input` = `input`; self.`options` = `options`; self.`question` = `question` }
    public static func read(_ json: JSONValue) throws -> InputRequestCallChoose {
        let object = try jsonObject(json)
        let known: Set<String> = ["function", "input", "options", "question"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, function: try String.read(jsonRequired(object, "function")), input: try InputRequestInput.read(jsonRequired(object, "input")), options: try readPresence(object, "options", InputRequestOptions.self), question: try InputRequestQuestion.read(jsonRequired(object, "question")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public struct InputRequestCallDecide: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `function`: String
    public let `input`: InputRequestInput
    public let `options`: Presence<InputRequestOptions>
    public let `question`: InputRequestQuestion
    public init(extensions: [String: JSONValue] = [:], `function`: String = "decide", `input`: InputRequestInput, `options`: Presence<InputRequestOptions> = .absent, `question`: InputRequestQuestion) { self.extensions = extensions; self.`function` = `function`; self.`input` = `input`; self.`options` = `options`; self.`question` = `question` }
    public static func read(_ json: JSONValue) throws -> InputRequestCallDecide {
        let object = try jsonObject(json)
        let known: Set<String> = ["function", "input", "options", "question"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, function: try String.read(jsonRequired(object, "function")), input: try InputRequestInput.read(jsonRequired(object, "input")), options: try readPresence(object, "options", InputRequestOptions.self), question: try InputRequestQuestion.read(jsonRequired(object, "question")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public struct InputRequestCallFilter: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `function`: String
    public let `input`: InputRequestInput
    public let `options`: Presence<InputRequestOptions>
    public let `question`: InputRequestQuestion
    public init(extensions: [String: JSONValue] = [:], `function`: String = "filter", `input`: InputRequestInput, `options`: Presence<InputRequestOptions> = .absent, `question`: InputRequestQuestion) { self.extensions = extensions; self.`function` = `function`; self.`input` = `input`; self.`options` = `options`; self.`question` = `question` }
    public static func read(_ json: JSONValue) throws -> InputRequestCallFilter {
        let object = try jsonObject(json)
        let known: Set<String> = ["function", "input", "options", "question"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, function: try String.read(jsonRequired(object, "function")), input: try InputRequestInput.read(jsonRequired(object, "input")), options: try readPresence(object, "options", InputRequestOptions.self), question: try InputRequestQuestion.read(jsonRequired(object, "question")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public struct InputRequestCallFind: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `function`: String
    public let `input`: InputRequestInput
    public let `options`: Presence<InputRequestOptions>
    public let `question`: InputRequestQuestion
    public init(extensions: [String: JSONValue] = [:], `function`: String = "find", `input`: InputRequestInput, `options`: Presence<InputRequestOptions> = .absent, `question`: InputRequestQuestion) { self.extensions = extensions; self.`function` = `function`; self.`input` = `input`; self.`options` = `options`; self.`question` = `question` }
    public static func read(_ json: JSONValue) throws -> InputRequestCallFind {
        let object = try jsonObject(json)
        let known: Set<String> = ["function", "input", "options", "question"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, function: try String.read(jsonRequired(object, "function")), input: try InputRequestInput.read(jsonRequired(object, "input")), options: try readPresence(object, "options", InputRequestOptions.self), question: try InputRequestQuestion.read(jsonRequired(object, "question")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public struct InputRequestCallRank: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `function`: String
    public let `input`: InputRequestInput
    public let `options`: Presence<InputRequestOptions>
    public let `question`: InputRequestQuestion
    public init(extensions: [String: JSONValue] = [:], `function`: String = "rank", `input`: InputRequestInput, `options`: Presence<InputRequestOptions> = .absent, `question`: InputRequestQuestion) { self.extensions = extensions; self.`function` = `function`; self.`input` = `input`; self.`options` = `options`; self.`question` = `question` }
    public static func read(_ json: JSONValue) throws -> InputRequestCallRank {
        let object = try jsonObject(json)
        let known: Set<String> = ["function", "input", "options", "question"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, function: try String.read(jsonRequired(object, "function")), input: try InputRequestInput.read(jsonRequired(object, "input")), options: try readPresence(object, "options", InputRequestOptions.self), question: try InputRequestQuestion.read(jsonRequired(object, "question")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public struct InputRequestCallRecognize: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `function`: String
    public let `input`: InputRequestInput
    public let `options`: Presence<InputRequestOptions>
    public let `question`: InputRequestQuestion
    public init(extensions: [String: JSONValue] = [:], `function`: String = "recognize", `input`: InputRequestInput, `options`: Presence<InputRequestOptions> = .absent, `question`: InputRequestQuestion) { self.extensions = extensions; self.`function` = `function`; self.`input` = `input`; self.`options` = `options`; self.`question` = `question` }
    public static func read(_ json: JSONValue) throws -> InputRequestCallRecognize {
        let object = try jsonObject(json)
        let known: Set<String> = ["function", "input", "options", "question"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, function: try String.read(jsonRequired(object, "function")), input: try InputRequestInput.read(jsonRequired(object, "input")), options: try readPresence(object, "options", InputRequestOptions.self), question: try InputRequestQuestion.read(jsonRequired(object, "question")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public struct InputRequestCallRelate: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `function`: String
    public let `input`: InputRequestInput
    public let `options`: Presence<InputRequestOptions>
    public let `question`: InputRequestQuestion
    public init(extensions: [String: JSONValue] = [:], `function`: String = "relate", `input`: InputRequestInput, `options`: Presence<InputRequestOptions> = .absent, `question`: InputRequestQuestion) { self.extensions = extensions; self.`function` = `function`; self.`input` = `input`; self.`options` = `options`; self.`question` = `question` }
    public static func read(_ json: JSONValue) throws -> InputRequestCallRelate {
        let object = try jsonObject(json)
        let known: Set<String> = ["function", "input", "options", "question"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, function: try String.read(jsonRequired(object, "function")), input: try InputRequestInput.read(jsonRequired(object, "input")), options: try readPresence(object, "options", InputRequestOptions.self), question: try InputRequestQuestion.read(jsonRequired(object, "question")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public struct InputRequestCallScore: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `function`: String
    public let `input`: InputRequestInput
    public let `options`: Presence<InputRequestOptions>
    public let `question`: InputRequestQuestion
    public init(extensions: [String: JSONValue] = [:], `function`: String = "score", `input`: InputRequestInput, `options`: Presence<InputRequestOptions> = .absent, `question`: InputRequestQuestion) { self.extensions = extensions; self.`function` = `function`; self.`input` = `input`; self.`options` = `options`; self.`question` = `question` }
    public static func read(_ json: JSONValue) throws -> InputRequestCallScore {
        let object = try jsonObject(json)
        let known: Set<String> = ["function", "input", "options", "question"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, function: try String.read(jsonRequired(object, "function")), input: try InputRequestInput.read(jsonRequired(object, "input")), options: try readPresence(object, "options", InputRequestOptions.self), question: try InputRequestQuestion.read(jsonRequired(object, "question")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public struct InputRequestCallTag: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `function`: String
    public let `input`: InputRequestInput
    public let `options`: Presence<InputRequestOptions>
    public let `question`: InputRequestQuestion
    public init(extensions: [String: JSONValue] = [:], `function`: String = "tag", `input`: InputRequestInput, `options`: Presence<InputRequestOptions> = .absent, `question`: InputRequestQuestion) { self.extensions = extensions; self.`function` = `function`; self.`input` = `input`; self.`options` = `options`; self.`question` = `question` }
    public static func read(_ json: JSONValue) throws -> InputRequestCallTag {
        let object = try jsonObject(json)
        let known: Set<String> = ["function", "input", "options", "question"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, function: try String.read(jsonRequired(object, "function")), input: try InputRequestInput.read(jsonRequired(object, "input")), options: try readPresence(object, "options", InputRequestOptions.self), question: try InputRequestQuestion.read(jsonRequired(object, "question")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public indirect enum InputRequestDefinition: JSONRepresentable {
    case `alternative0`(InputRequestDefinitionFieldsDecide)
    case `alternative1`(InputRequestDefinitionFieldsChoose)
    case `alternative2`(InputRequestDefinitionFieldsTag)
    case `alternative3`(InputRequestDefinitionFieldsScore)
    case `alternative4`(InputRequestDefinitionFieldsRelateVersion)
    case `alternative5`(InputRequestDefinitionFieldsFind)
    case `alternative6`(InputRequestDefinitionFieldsRecognizeVersion)
    case `alternative7`(InputRequestDefinitionFieldsQuestionsVersion)
    public static func read(_ json: JSONValue) throws -> InputRequestDefinition {
        let object = try jsonObject(json)
        if object["decide"] != nil && object["choose"] == nil && object["find"] == nil && object["questions"] == nil && object["recognize"] == nil && object["relate"] == nil && object["score"] == nil && object["tag"] == nil && object["version"] == nil { return .`alternative0`(try InputRequestDefinitionFieldsDecide.read(json)) }
        if object["choose"] != nil && object["decide"] == nil && object["find"] == nil && object["questions"] == nil && object["recognize"] == nil && object["relate"] == nil && object["score"] == nil && object["tag"] == nil && object["version"] == nil { return .`alternative1`(try InputRequestDefinitionFieldsChoose.read(json)) }
        if object["tag"] != nil && object["choose"] == nil && object["decide"] == nil && object["find"] == nil && object["questions"] == nil && object["recognize"] == nil && object["relate"] == nil && object["score"] == nil && object["version"] == nil { return .`alternative2`(try InputRequestDefinitionFieldsTag.read(json)) }
        if object["score"] != nil && object["choose"] == nil && object["decide"] == nil && object["find"] == nil && object["questions"] == nil && object["recognize"] == nil && object["relate"] == nil && object["tag"] == nil && object["version"] == nil { return .`alternative3`(try InputRequestDefinitionFieldsScore.read(json)) }
        if object["relate"] != nil && object["version"] != nil && object["choose"] == nil && object["decide"] == nil && object["find"] == nil && object["questions"] == nil && object["recognize"] == nil && object["score"] == nil && object["tag"] == nil { return .`alternative4`(try InputRequestDefinitionFieldsRelateVersion.read(json)) }
        if object["find"] != nil && object["choose"] == nil && object["decide"] == nil && object["questions"] == nil && object["recognize"] == nil && object["relate"] == nil && object["score"] == nil && object["tag"] == nil && object["version"] == nil { return .`alternative5`(try InputRequestDefinitionFieldsFind.read(json)) }
        if object["recognize"] != nil && object["version"] != nil && object["choose"] == nil && object["decide"] == nil && object["find"] == nil && object["questions"] == nil && object["relate"] == nil && object["score"] == nil && object["tag"] == nil { return .`alternative6`(try InputRequestDefinitionFieldsRecognizeVersion.read(json)) }
        if object["questions"] != nil && object["version"] != nil && object["choose"] == nil && object["decide"] == nil && object["find"] == nil && object["recognize"] == nil && object["relate"] == nil && object["score"] == nil && object["tag"] == nil { return .`alternative7`(try InputRequestDefinitionFieldsQuestionsVersion.read(json)) }
        throw JSONConversionError("Unknown InputRequestDefinition alternative")
    }
    public var json: JSONValue {
        switch self {
        case .`alternative0`(let value): return value.json
        case .`alternative1`(let value): return value.json
        case .`alternative2`(let value): return value.json
        case .`alternative3`(let value): return value.json
        case .`alternative4`(let value): return value.json
        case .`alternative5`(let value): return value.json
        case .`alternative6`(let value): return value.json
        case .`alternative7`(let value): return value.json
        }
    }
}
public indirect enum InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties: JSONRepresentable {
    case `alternative0`(InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide)
    case `alternative1`(InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose)
    case `alternative2`(InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag)
    case `alternative3`(InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore)
    public static func read(_ json: JSONValue) throws -> InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties {
        let object = try jsonObject(json)
        if object["decide"] != nil && object["choose"] == nil && object["score"] == nil && object["tag"] == nil { return .`alternative0`(try InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide.read(json)) }
        if object["choose"] != nil && object["decide"] == nil && object["score"] == nil && object["tag"] == nil { return .`alternative1`(try InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose.read(json)) }
        if object["tag"] != nil && object["choose"] == nil && object["decide"] == nil && object["score"] == nil { return .`alternative2`(try InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag.read(json)) }
        if object["score"] != nil && object["choose"] == nil && object["decide"] == nil && object["tag"] == nil { return .`alternative3`(try InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore.read(json)) }
        throw JSONConversionError("Unknown InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties alternative")
    }
    public var json: JSONValue {
        switch self {
        case .`alternative0`(let value): return value.json
        case .`alternative1`(let value): return value.json
        case .`alternative2`(let value): return value.json
        case .`alternative3`(let value): return value.json
        }
    }
}
public struct InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `choose`: InputAuthoredQuestionText
    public let `contextSchema`: Presence<InputAuthoredInputDeclaration>
    public let `itemSchema`: Presence<InputAuthoredInputDeclaration>
    public let `name`: Presence<String>
    public let `on`: Presence<InputAuthoredPointers>
    public let `options`: Presence<InputAuthoredOptions>
    public let `threshold`: Presence<InputAuthoredCut>
    public let `wordingVersion`: Presence<Int64>
    public init(extensions: [String: JSONValue] = [:], `choose`: InputAuthoredQuestionText, `contextSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `itemSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `name`: Presence<String> = .absent, `on`: Presence<InputAuthoredPointers> = .absent, `options`: Presence<InputAuthoredOptions> = .absent, `threshold`: Presence<InputAuthoredCut> = .absent, `wordingVersion`: Presence<Int64> = .absent) { self.extensions = extensions; self.`choose` = `choose`; self.`contextSchema` = `contextSchema`; self.`itemSchema` = `itemSchema`; self.`name` = `name`; self.`on` = `on`; self.`options` = `options`; self.`threshold` = `threshold`; self.`wordingVersion` = `wordingVersion` }
    public static func read(_ json: JSONValue) throws -> InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose {
        let object = try jsonObject(json)
        let known: Set<String> = ["choose", "context_schema", "item_schema", "name", "on", "options", "threshold", "wording_version"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, choose: try InputAuthoredQuestionText.read(jsonRequired(object, "choose")), contextSchema: try readPresence(object, "context_schema", InputAuthoredInputDeclaration.self), itemSchema: try readPresence(object, "item_schema", InputAuthoredInputDeclaration.self), name: try readPresence(object, "name", String.self), on: try readPresence(object, "on", InputAuthoredPointers.self), options: try readPresence(object, "options", InputAuthoredOptions.self), threshold: try readPresence(object, "threshold", InputAuthoredCut.self), wordingVersion: try readPresence(object, "wording_version", Int64.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["choose", "context_schema", "item_schema", "name", "on", "options", "threshold", "wording_version"]
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
public struct InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `contextSchema`: Presence<InputAuthoredInputDeclaration>
    public let `decide`: InputAuthoredQuestionText
    public let `false`: Presence<InputAuthoredCriterion>
    public let `itemSchema`: Presence<InputAuthoredInputDeclaration>
    public let `name`: Presence<String>
    public let `on`: Presence<InputAuthoredPointers>
    public let `threshold`: Presence<InputAuthoredThreshold>
    public let `true`: Presence<InputAuthoredCriterion>
    public let `wordingVersion`: Presence<Int64>
    public init(extensions: [String: JSONValue] = [:], `contextSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `decide`: InputAuthoredQuestionText, `false`: Presence<InputAuthoredCriterion> = .absent, `itemSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `name`: Presence<String> = .absent, `on`: Presence<InputAuthoredPointers> = .absent, `threshold`: Presence<InputAuthoredThreshold> = .absent, `true`: Presence<InputAuthoredCriterion> = .absent, `wordingVersion`: Presence<Int64> = .absent) { self.extensions = extensions; self.`contextSchema` = `contextSchema`; self.`decide` = `decide`; self.`false` = `false`; self.`itemSchema` = `itemSchema`; self.`name` = `name`; self.`on` = `on`; self.`threshold` = `threshold`; self.`true` = `true`; self.`wordingVersion` = `wordingVersion` }
    public static func read(_ json: JSONValue) throws -> InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide {
        let object = try jsonObject(json)
        let known: Set<String> = ["context_schema", "decide", "false", "item_schema", "name", "on", "threshold", "true", "wording_version"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, contextSchema: try readPresence(object, "context_schema", InputAuthoredInputDeclaration.self), decide: try InputAuthoredQuestionText.read(jsonRequired(object, "decide")), false: try readPresence(object, "false", InputAuthoredCriterion.self), itemSchema: try readPresence(object, "item_schema", InputAuthoredInputDeclaration.self), name: try readPresence(object, "name", String.self), on: try readPresence(object, "on", InputAuthoredPointers.self), threshold: try readPresence(object, "threshold", InputAuthoredThreshold.self), true: try readPresence(object, "true", InputAuthoredCriterion.self), wordingVersion: try readPresence(object, "wording_version", Int64.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["context_schema", "decide", "false", "item_schema", "name", "on", "threshold", "true", "wording_version"]
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
public struct InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `contextSchema`: Presence<InputAuthoredInputDeclaration>
    public let `itemSchema`: Presence<InputAuthoredInputDeclaration>
    public let `levels`: Presence<InputAuthoredLevels>
    public let `name`: Presence<String>
    public let `on`: Presence<InputAuthoredPointers>
    public let `score`: InputAuthoredQuestionText
    public let `wordingVersion`: Presence<Int64>
    public init(extensions: [String: JSONValue] = [:], `contextSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `itemSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `levels`: Presence<InputAuthoredLevels> = .absent, `name`: Presence<String> = .absent, `on`: Presence<InputAuthoredPointers> = .absent, `score`: InputAuthoredQuestionText, `wordingVersion`: Presence<Int64> = .absent) { self.extensions = extensions; self.`contextSchema` = `contextSchema`; self.`itemSchema` = `itemSchema`; self.`levels` = `levels`; self.`name` = `name`; self.`on` = `on`; self.`score` = `score`; self.`wordingVersion` = `wordingVersion` }
    public static func read(_ json: JSONValue) throws -> InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore {
        let object = try jsonObject(json)
        let known: Set<String> = ["context_schema", "item_schema", "levels", "name", "on", "score", "wording_version"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, contextSchema: try readPresence(object, "context_schema", InputAuthoredInputDeclaration.self), itemSchema: try readPresence(object, "item_schema", InputAuthoredInputDeclaration.self), levels: try readPresence(object, "levels", InputAuthoredLevels.self), name: try readPresence(object, "name", String.self), on: try readPresence(object, "on", InputAuthoredPointers.self), score: try InputAuthoredQuestionText.read(jsonRequired(object, "score")), wordingVersion: try readPresence(object, "wording_version", Int64.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["context_schema", "item_schema", "levels", "name", "on", "score", "wording_version"]
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
public struct InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `contextSchema`: Presence<InputAuthoredInputDeclaration>
    public let `itemSchema`: Presence<InputAuthoredInputDeclaration>
    public let `labels`: Presence<InputAuthoredLabels>
    public let `name`: Presence<String>
    public let `on`: Presence<InputAuthoredPointers>
    public let `tag`: InputAuthoredQuestionText
    public let `threshold`: Presence<InputAuthoredCut>
    public let `wordingVersion`: Presence<Int64>
    public init(extensions: [String: JSONValue] = [:], `contextSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `itemSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `labels`: Presence<InputAuthoredLabels> = .absent, `name`: Presence<String> = .absent, `on`: Presence<InputAuthoredPointers> = .absent, `tag`: InputAuthoredQuestionText, `threshold`: Presence<InputAuthoredCut> = .absent, `wordingVersion`: Presence<Int64> = .absent) { self.extensions = extensions; self.`contextSchema` = `contextSchema`; self.`itemSchema` = `itemSchema`; self.`labels` = `labels`; self.`name` = `name`; self.`on` = `on`; self.`tag` = `tag`; self.`threshold` = `threshold`; self.`wordingVersion` = `wordingVersion` }
    public static func read(_ json: JSONValue) throws -> InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag {
        let object = try jsonObject(json)
        let known: Set<String> = ["context_schema", "item_schema", "labels", "name", "on", "tag", "threshold", "wording_version"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, contextSchema: try readPresence(object, "context_schema", InputAuthoredInputDeclaration.self), itemSchema: try readPresence(object, "item_schema", InputAuthoredInputDeclaration.self), labels: try readPresence(object, "labels", InputAuthoredLabels.self), name: try readPresence(object, "name", String.self), on: try readPresence(object, "on", InputAuthoredPointers.self), tag: try InputAuthoredQuestionText.read(jsonRequired(object, "tag")), threshold: try readPresence(object, "threshold", InputAuthoredCut.self), wordingVersion: try readPresence(object, "wording_version", Int64.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["context_schema", "item_schema", "labels", "name", "on", "tag", "threshold", "wording_version"]
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
public struct InputRequestDefinitionFieldsChoose: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `batch`: Presence<InputRequestDefinitionFieldsChooseBatch>
    public let `choose`: InputAuthoredQuestionText
    public let `contextSchema`: Presence<InputAuthoredInputDeclaration>
    public let `itemSchema`: Presence<InputAuthoredInputDeclaration>
    public let `model`: Presence<InputAuthoredName>
    public let `name`: Presence<String>
    public let `on`: Presence<InputAuthoredPointers>
    public let `options`: Presence<InputAuthoredOptions>
    public let `profile`: Presence<InputAuthoredProfile>
    public let `threshold`: Presence<InputAuthoredCut>
    public let `wordingVersion`: Presence<Int64>
    public init(extensions: [String: JSONValue] = [:], `batch`: Presence<InputRequestDefinitionFieldsChooseBatch> = .absent, `choose`: InputAuthoredQuestionText, `contextSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `itemSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `model`: Presence<InputAuthoredName> = .absent, `name`: Presence<String> = .absent, `on`: Presence<InputAuthoredPointers> = .absent, `options`: Presence<InputAuthoredOptions> = .absent, `profile`: Presence<InputAuthoredProfile> = .absent, `threshold`: Presence<InputAuthoredCut> = .absent, `wordingVersion`: Presence<Int64> = .absent) { self.extensions = extensions; self.`batch` = `batch`; self.`choose` = `choose`; self.`contextSchema` = `contextSchema`; self.`itemSchema` = `itemSchema`; self.`model` = `model`; self.`name` = `name`; self.`on` = `on`; self.`options` = `options`; self.`profile` = `profile`; self.`threshold` = `threshold`; self.`wordingVersion` = `wordingVersion` }
    public static func read(_ json: JSONValue) throws -> InputRequestDefinitionFieldsChoose {
        let object = try jsonObject(json)
        let known: Set<String> = ["batch", "choose", "context_schema", "item_schema", "model", "name", "on", "options", "profile", "threshold", "wording_version"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, batch: try readPresence(object, "batch", InputRequestDefinitionFieldsChooseBatch.self), choose: try InputAuthoredQuestionText.read(jsonRequired(object, "choose")), contextSchema: try readPresence(object, "context_schema", InputAuthoredInputDeclaration.self), itemSchema: try readPresence(object, "item_schema", InputAuthoredInputDeclaration.self), model: try readPresence(object, "model", InputAuthoredName.self), name: try readPresence(object, "name", String.self), on: try readPresence(object, "on", InputAuthoredPointers.self), options: try readPresence(object, "options", InputAuthoredOptions.self), profile: try readPresence(object, "profile", InputAuthoredProfile.self), threshold: try readPresence(object, "threshold", InputAuthoredCut.self), wordingVersion: try readPresence(object, "wording_version", Int64.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["batch", "choose", "context_schema", "item_schema", "model", "name", "on", "options", "profile", "threshold", "wording_version"]
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
public indirect enum InputRequestDefinitionFieldsChooseBatch: JSONRepresentable {
    case alternative0(String)
    case alternative1(Int64)
    public static func read(_ json: JSONValue) throws -> InputRequestDefinitionFieldsChooseBatch {
        if let value = try? String.read(json) { return .alternative0(value) }
        if let value = try? Int64.read(json) { return .alternative1(value) }
        throw JSONConversionError("Unknown InputRequestDefinitionFieldsChooseBatch value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public struct InputRequestDefinitionFieldsDecide: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `batch`: Presence<InputRequestDefinitionFieldsDecideBatch>
    public let `contextSchema`: Presence<InputAuthoredInputDeclaration>
    public let `decide`: InputAuthoredQuestionText
    public let `false`: Presence<InputAuthoredCriterion>
    public let `itemSchema`: Presence<InputAuthoredInputDeclaration>
    public let `model`: Presence<InputAuthoredName>
    public let `name`: Presence<String>
    public let `on`: Presence<InputAuthoredPointers>
    public let `profile`: Presence<InputAuthoredProfile>
    public let `threshold`: Presence<InputAuthoredThreshold>
    public let `true`: Presence<InputAuthoredCriterion>
    public let `wordingVersion`: Presence<Int64>
    public init(extensions: [String: JSONValue] = [:], `batch`: Presence<InputRequestDefinitionFieldsDecideBatch> = .absent, `contextSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `decide`: InputAuthoredQuestionText, `false`: Presence<InputAuthoredCriterion> = .absent, `itemSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `model`: Presence<InputAuthoredName> = .absent, `name`: Presence<String> = .absent, `on`: Presence<InputAuthoredPointers> = .absent, `profile`: Presence<InputAuthoredProfile> = .absent, `threshold`: Presence<InputAuthoredThreshold> = .absent, `true`: Presence<InputAuthoredCriterion> = .absent, `wordingVersion`: Presence<Int64> = .absent) { self.extensions = extensions; self.`batch` = `batch`; self.`contextSchema` = `contextSchema`; self.`decide` = `decide`; self.`false` = `false`; self.`itemSchema` = `itemSchema`; self.`model` = `model`; self.`name` = `name`; self.`on` = `on`; self.`profile` = `profile`; self.`threshold` = `threshold`; self.`true` = `true`; self.`wordingVersion` = `wordingVersion` }
    public static func read(_ json: JSONValue) throws -> InputRequestDefinitionFieldsDecide {
        let object = try jsonObject(json)
        let known: Set<String> = ["batch", "context_schema", "decide", "false", "item_schema", "model", "name", "on", "profile", "threshold", "true", "wording_version"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, batch: try readPresence(object, "batch", InputRequestDefinitionFieldsDecideBatch.self), contextSchema: try readPresence(object, "context_schema", InputAuthoredInputDeclaration.self), decide: try InputAuthoredQuestionText.read(jsonRequired(object, "decide")), false: try readPresence(object, "false", InputAuthoredCriterion.self), itemSchema: try readPresence(object, "item_schema", InputAuthoredInputDeclaration.self), model: try readPresence(object, "model", InputAuthoredName.self), name: try readPresence(object, "name", String.self), on: try readPresence(object, "on", InputAuthoredPointers.self), profile: try readPresence(object, "profile", InputAuthoredProfile.self), threshold: try readPresence(object, "threshold", InputAuthoredThreshold.self), true: try readPresence(object, "true", InputAuthoredCriterion.self), wordingVersion: try readPresence(object, "wording_version", Int64.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["batch", "context_schema", "decide", "false", "item_schema", "model", "name", "on", "profile", "threshold", "true", "wording_version"]
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
public indirect enum InputRequestDefinitionFieldsDecideBatch: JSONRepresentable {
    case alternative0(String)
    case alternative1(Int64)
    public static func read(_ json: JSONValue) throws -> InputRequestDefinitionFieldsDecideBatch {
        if let value = try? String.read(json) { return .alternative0(value) }
        if let value = try? Int64.read(json) { return .alternative1(value) }
        throw JSONConversionError("Unknown InputRequestDefinitionFieldsDecideBatch value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public struct InputRequestDefinitionFieldsFind: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `contextSchema`: Presence<InputAuthoredInputDeclaration>
    public let `find`: InputAuthoredQuestionText
    public let `itemSchema`: Presence<InputAuthoredInputDeclaration>
    public let `model`: Presence<InputAuthoredName>
    public let `name`: Presence<String>
    public let `on`: Presence<InputAuthoredPointers>
    public let `profile`: Presence<InputAuthoredProfile>
    public let `wordingVersion`: Presence<Int64>
    public init(extensions: [String: JSONValue] = [:], `contextSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `find`: InputAuthoredQuestionText, `itemSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `model`: Presence<InputAuthoredName> = .absent, `name`: Presence<String> = .absent, `on`: Presence<InputAuthoredPointers> = .absent, `profile`: Presence<InputAuthoredProfile> = .absent, `wordingVersion`: Presence<Int64> = .absent) { self.extensions = extensions; self.`contextSchema` = `contextSchema`; self.`find` = `find`; self.`itemSchema` = `itemSchema`; self.`model` = `model`; self.`name` = `name`; self.`on` = `on`; self.`profile` = `profile`; self.`wordingVersion` = `wordingVersion` }
    public static func read(_ json: JSONValue) throws -> InputRequestDefinitionFieldsFind {
        let object = try jsonObject(json)
        let known: Set<String> = ["context_schema", "find", "item_schema", "model", "name", "on", "profile", "wording_version"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, contextSchema: try readPresence(object, "context_schema", InputAuthoredInputDeclaration.self), find: try InputAuthoredQuestionText.read(jsonRequired(object, "find")), itemSchema: try readPresence(object, "item_schema", InputAuthoredInputDeclaration.self), model: try readPresence(object, "model", InputAuthoredName.self), name: try readPresence(object, "name", String.self), on: try readPresence(object, "on", InputAuthoredPointers.self), profile: try readPresence(object, "profile", InputAuthoredProfile.self), wordingVersion: try readPresence(object, "wording_version", Int64.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["context_schema", "find", "item_schema", "model", "name", "on", "profile", "wording_version"]
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
public struct InputRequestDefinitionFieldsQuestionsVersion: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `batch`: Presence<JSONValue>
    public let `profile`: Presence<InputAuthoredProfile>
    public let `questions`: [String: InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties]
    public let `threshold`: Presence<InputAuthoredThreshold>
    public let `version`: Int64
    public init(extensions: [String: JSONValue] = [:], `batch`: Presence<JSONValue> = .absent, `profile`: Presence<InputAuthoredProfile> = .absent, `questions`: [String: InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties], `threshold`: Presence<InputAuthoredThreshold> = .absent, `version`: Int64 = 1) { self.extensions = extensions; self.`batch` = `batch`; self.`profile` = `profile`; self.`questions` = `questions`; self.`threshold` = `threshold`; self.`version` = `version` }
    public static func read(_ json: JSONValue) throws -> InputRequestDefinitionFieldsQuestionsVersion {
        let object = try jsonObject(json)
        let known: Set<String> = ["batch", "profile", "questions", "threshold", "version"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, batch: try readPresence(object, "batch", JSONValue.self), profile: try readPresence(object, "profile", InputAuthoredProfile.self), questions: try [String: InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties].read(jsonRequired(object, "questions")), threshold: try readPresence(object, "threshold", InputAuthoredThreshold.self), version: try Int64.read(jsonRequired(object, "version")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public struct InputRequestDefinitionFieldsRecognizeVersion: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `contextSchema`: Presence<InputAuthoredInputDeclaration>
    public let `itemSchema`: Presence<InputAuthoredInputDeclaration>
    public let `model`: Presence<InputAuthoredName>
    public let `name`: Presence<String>
    public let `on`: Presence<InputAuthoredPointers>
    public let `profile`: Presence<InputAuthoredProfile>
    public let `recognize`: InputRequestDefinitionFieldsRecognizeVersionRecognize
    public let `relationThreshold`: Presence<InputAuthoredCut>
    public let `threshold`: Presence<InputAuthoredCut>
    public let `version`: Int64
    public let `wordingVersion`: Presence<Int64>
    public init(extensions: [String: JSONValue] = [:], `contextSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `itemSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `model`: Presence<InputAuthoredName> = .absent, `name`: Presence<String> = .absent, `on`: Presence<InputAuthoredPointers> = .absent, `profile`: Presence<InputAuthoredProfile> = .absent, `recognize`: InputRequestDefinitionFieldsRecognizeVersionRecognize, `relationThreshold`: Presence<InputAuthoredCut> = .absent, `threshold`: Presence<InputAuthoredCut> = .absent, `version`: Int64 = 1, `wordingVersion`: Presence<Int64> = .absent) { self.extensions = extensions; self.`contextSchema` = `contextSchema`; self.`itemSchema` = `itemSchema`; self.`model` = `model`; self.`name` = `name`; self.`on` = `on`; self.`profile` = `profile`; self.`recognize` = `recognize`; self.`relationThreshold` = `relationThreshold`; self.`threshold` = `threshold`; self.`version` = `version`; self.`wordingVersion` = `wordingVersion` }
    public static func read(_ json: JSONValue) throws -> InputRequestDefinitionFieldsRecognizeVersion {
        let object = try jsonObject(json)
        let known: Set<String> = ["context_schema", "item_schema", "model", "name", "on", "profile", "recognize", "relation_threshold", "threshold", "version", "wording_version"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, contextSchema: try readPresence(object, "context_schema", InputAuthoredInputDeclaration.self), itemSchema: try readPresence(object, "item_schema", InputAuthoredInputDeclaration.self), model: try readPresence(object, "model", InputAuthoredName.self), name: try readPresence(object, "name", String.self), on: try readPresence(object, "on", InputAuthoredPointers.self), profile: try readPresence(object, "profile", InputAuthoredProfile.self), recognize: try InputRequestDefinitionFieldsRecognizeVersionRecognize.read(jsonRequired(object, "recognize")), relationThreshold: try readPresence(object, "relation_threshold", InputAuthoredCut.self), threshold: try readPresence(object, "threshold", InputAuthoredCut.self), version: try Int64.read(jsonRequired(object, "version")), wordingVersion: try readPresence(object, "wording_version", Int64.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["context_schema", "item_schema", "model", "name", "on", "profile", "recognize", "relation_threshold", "threshold", "version", "wording_version"]
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
public struct InputRequestDefinitionFieldsRecognizeVersionRecognize: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `entityDefinition`: Presence<InputAuthoredQuestionText>
    public let `instructions`: Presence<InputAuthoredQuestionText>
    public let `kinds`: Presence<[String: InputAuthoredDescription]>
    public let `mode`: Presence<InputRecognitionMode>
    public let `relations`: Presence<[InputAuthoredRelation]>
    public let `snippetPieces`: Presence<UInt64>
    public let `stageContext`: Presence<InputRecognitionStageContext>
    public init(extensions: [String: JSONValue] = [:], `entityDefinition`: Presence<InputAuthoredQuestionText> = .absent, `instructions`: Presence<InputAuthoredQuestionText> = .absent, `kinds`: Presence<[String: InputAuthoredDescription]> = .absent, `mode`: Presence<InputRecognitionMode> = .absent, `relations`: Presence<[InputAuthoredRelation]> = .absent, `snippetPieces`: Presence<UInt64> = .absent, `stageContext`: Presence<InputRecognitionStageContext> = .absent) { self.extensions = extensions; self.`entityDefinition` = `entityDefinition`; self.`instructions` = `instructions`; self.`kinds` = `kinds`; self.`mode` = `mode`; self.`relations` = `relations`; self.`snippetPieces` = `snippetPieces`; self.`stageContext` = `stageContext` }
    public static func read(_ json: JSONValue) throws -> InputRequestDefinitionFieldsRecognizeVersionRecognize {
        let object = try jsonObject(json)
        let known: Set<String> = ["entity_definition", "instructions", "kinds", "mode", "relations", "snippet_pieces", "stage_context"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, entityDefinition: try readPresence(object, "entity_definition", InputAuthoredQuestionText.self), instructions: try readPresence(object, "instructions", InputAuthoredQuestionText.self), kinds: try readPresence(object, "kinds", [String: InputAuthoredDescription].self), mode: try readPresence(object, "mode", InputRecognitionMode.self), relations: try readPresence(object, "relations", [InputAuthoredRelation].self), snippetPieces: try readPresence(object, "snippet_pieces", UInt64.self), stageContext: try readPresence(object, "stage_context", InputRecognitionStageContext.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["entity_definition", "instructions", "kinds", "mode", "relations", "snippet_pieces", "stage_context"]
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
public struct InputRequestDefinitionFieldsRelateVersion: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `contextSchema`: Presence<InputAuthoredInputDeclaration>
    public let `itemSchema`: Presence<InputAuthoredInputDeclaration>
    public let `model`: Presence<InputAuthoredName>
    public let `name`: Presence<String>
    public let `profile`: Presence<InputAuthoredProfile>
    public let `relate`: InputRequestDefinitionFieldsRelateVersionRelate
    public let `threshold`: Presence<InputAuthoredCut>
    public let `version`: Int64
    public let `wordingVersion`: Presence<Int64>
    public init(extensions: [String: JSONValue] = [:], `contextSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `itemSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `model`: Presence<InputAuthoredName> = .absent, `name`: Presence<String> = .absent, `profile`: Presence<InputAuthoredProfile> = .absent, `relate`: InputRequestDefinitionFieldsRelateVersionRelate, `threshold`: Presence<InputAuthoredCut> = .absent, `version`: Int64 = 1, `wordingVersion`: Presence<Int64> = .absent) { self.extensions = extensions; self.`contextSchema` = `contextSchema`; self.`itemSchema` = `itemSchema`; self.`model` = `model`; self.`name` = `name`; self.`profile` = `profile`; self.`relate` = `relate`; self.`threshold` = `threshold`; self.`version` = `version`; self.`wordingVersion` = `wordingVersion` }
    public static func read(_ json: JSONValue) throws -> InputRequestDefinitionFieldsRelateVersion {
        let object = try jsonObject(json)
        let known: Set<String> = ["context_schema", "item_schema", "model", "name", "profile", "relate", "threshold", "version", "wording_version"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, contextSchema: try readPresence(object, "context_schema", InputAuthoredInputDeclaration.self), itemSchema: try readPresence(object, "item_schema", InputAuthoredInputDeclaration.self), model: try readPresence(object, "model", InputAuthoredName.self), name: try readPresence(object, "name", String.self), profile: try readPresence(object, "profile", InputAuthoredProfile.self), relate: try InputRequestDefinitionFieldsRelateVersionRelate.read(jsonRequired(object, "relate")), threshold: try readPresence(object, "threshold", InputAuthoredCut.self), version: try Int64.read(jsonRequired(object, "version")), wordingVersion: try readPresence(object, "wording_version", Int64.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["context_schema", "item_schema", "model", "name", "profile", "relate", "threshold", "version", "wording_version"]
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
public struct InputRequestDefinitionFieldsRelateVersionRelate: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `fields`: Presence<InputRequestDefinitionFieldsRelateVersionRelateFields>
    public let `relations`: [InputAuthoredRelation]
    public init(extensions: [String: JSONValue] = [:], `fields`: Presence<InputRequestDefinitionFieldsRelateVersionRelateFields> = .absent, `relations`: [InputAuthoredRelation]) { self.extensions = extensions; self.`fields` = `fields`; self.`relations` = `relations` }
    public static func read(_ json: JSONValue) throws -> InputRequestDefinitionFieldsRelateVersionRelate {
        let object = try jsonObject(json)
        let known: Set<String> = ["fields", "relations"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, fields: try readPresence(object, "fields", InputRequestDefinitionFieldsRelateVersionRelateFields.self), relations: try [InputAuthoredRelation].read(jsonRequired(object, "relations")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["fields", "relations"]
        var object = extensions.filter { !known.contains($0.key) }
        writePresence(`fields`, "fields", &object)
        object["relations"] = `relations`.json
        return .object(object)
    }
}
public struct InputRequestDefinitionFieldsRelateVersionRelateFields: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `name`: String
    public init(extensions: [String: JSONValue] = [:], `kind`: String, `name`: String) { self.extensions = extensions; self.`kind` = `kind`; self.`name` = `name` }
    public static func read(_ json: JSONValue) throws -> InputRequestDefinitionFieldsRelateVersionRelateFields {
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
public struct InputRequestDefinitionFieldsScore: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `batch`: Presence<InputRequestDefinitionFieldsScoreBatch>
    public let `contextSchema`: Presence<InputAuthoredInputDeclaration>
    public let `itemSchema`: Presence<InputAuthoredInputDeclaration>
    public let `levels`: Presence<InputAuthoredLevels>
    public let `model`: Presence<InputAuthoredName>
    public let `name`: Presence<String>
    public let `on`: Presence<InputAuthoredPointers>
    public let `profile`: Presence<InputAuthoredProfile>
    public let `score`: InputAuthoredQuestionText
    public let `wordingVersion`: Presence<Int64>
    public init(extensions: [String: JSONValue] = [:], `batch`: Presence<InputRequestDefinitionFieldsScoreBatch> = .absent, `contextSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `itemSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `levels`: Presence<InputAuthoredLevels> = .absent, `model`: Presence<InputAuthoredName> = .absent, `name`: Presence<String> = .absent, `on`: Presence<InputAuthoredPointers> = .absent, `profile`: Presence<InputAuthoredProfile> = .absent, `score`: InputAuthoredQuestionText, `wordingVersion`: Presence<Int64> = .absent) { self.extensions = extensions; self.`batch` = `batch`; self.`contextSchema` = `contextSchema`; self.`itemSchema` = `itemSchema`; self.`levels` = `levels`; self.`model` = `model`; self.`name` = `name`; self.`on` = `on`; self.`profile` = `profile`; self.`score` = `score`; self.`wordingVersion` = `wordingVersion` }
    public static func read(_ json: JSONValue) throws -> InputRequestDefinitionFieldsScore {
        let object = try jsonObject(json)
        let known: Set<String> = ["batch", "context_schema", "item_schema", "levels", "model", "name", "on", "profile", "score", "wording_version"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, batch: try readPresence(object, "batch", InputRequestDefinitionFieldsScoreBatch.self), contextSchema: try readPresence(object, "context_schema", InputAuthoredInputDeclaration.self), itemSchema: try readPresence(object, "item_schema", InputAuthoredInputDeclaration.self), levels: try readPresence(object, "levels", InputAuthoredLevels.self), model: try readPresence(object, "model", InputAuthoredName.self), name: try readPresence(object, "name", String.self), on: try readPresence(object, "on", InputAuthoredPointers.self), profile: try readPresence(object, "profile", InputAuthoredProfile.self), score: try InputAuthoredQuestionText.read(jsonRequired(object, "score")), wordingVersion: try readPresence(object, "wording_version", Int64.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["batch", "context_schema", "item_schema", "levels", "model", "name", "on", "profile", "score", "wording_version"]
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
public indirect enum InputRequestDefinitionFieldsScoreBatch: JSONRepresentable {
    case alternative0(String)
    case alternative1(Int64)
    public static func read(_ json: JSONValue) throws -> InputRequestDefinitionFieldsScoreBatch {
        if let value = try? String.read(json) { return .alternative0(value) }
        if let value = try? Int64.read(json) { return .alternative1(value) }
        throw JSONConversionError("Unknown InputRequestDefinitionFieldsScoreBatch value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public struct InputRequestDefinitionFieldsTag: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `batch`: Presence<InputRequestDefinitionFieldsTagBatch>
    public let `contextSchema`: Presence<InputAuthoredInputDeclaration>
    public let `itemSchema`: Presence<InputAuthoredInputDeclaration>
    public let `labels`: Presence<InputAuthoredLabels>
    public let `model`: Presence<InputAuthoredName>
    public let `name`: Presence<String>
    public let `on`: Presence<InputAuthoredPointers>
    public let `profile`: Presence<InputAuthoredProfile>
    public let `tag`: InputAuthoredQuestionText
    public let `threshold`: Presence<InputAuthoredCut>
    public let `wordingVersion`: Presence<Int64>
    public init(extensions: [String: JSONValue] = [:], `batch`: Presence<InputRequestDefinitionFieldsTagBatch> = .absent, `contextSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `itemSchema`: Presence<InputAuthoredInputDeclaration> = .absent, `labels`: Presence<InputAuthoredLabels> = .absent, `model`: Presence<InputAuthoredName> = .absent, `name`: Presence<String> = .absent, `on`: Presence<InputAuthoredPointers> = .absent, `profile`: Presence<InputAuthoredProfile> = .absent, `tag`: InputAuthoredQuestionText, `threshold`: Presence<InputAuthoredCut> = .absent, `wordingVersion`: Presence<Int64> = .absent) { self.extensions = extensions; self.`batch` = `batch`; self.`contextSchema` = `contextSchema`; self.`itemSchema` = `itemSchema`; self.`labels` = `labels`; self.`model` = `model`; self.`name` = `name`; self.`on` = `on`; self.`profile` = `profile`; self.`tag` = `tag`; self.`threshold` = `threshold`; self.`wordingVersion` = `wordingVersion` }
    public static func read(_ json: JSONValue) throws -> InputRequestDefinitionFieldsTag {
        let object = try jsonObject(json)
        let known: Set<String> = ["batch", "context_schema", "item_schema", "labels", "model", "name", "on", "profile", "tag", "threshold", "wording_version"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, batch: try readPresence(object, "batch", InputRequestDefinitionFieldsTagBatch.self), contextSchema: try readPresence(object, "context_schema", InputAuthoredInputDeclaration.self), itemSchema: try readPresence(object, "item_schema", InputAuthoredInputDeclaration.self), labels: try readPresence(object, "labels", InputAuthoredLabels.self), model: try readPresence(object, "model", InputAuthoredName.self), name: try readPresence(object, "name", String.self), on: try readPresence(object, "on", InputAuthoredPointers.self), profile: try readPresence(object, "profile", InputAuthoredProfile.self), tag: try InputAuthoredQuestionText.read(jsonRequired(object, "tag")), threshold: try readPresence(object, "threshold", InputAuthoredCut.self), wordingVersion: try readPresence(object, "wording_version", Int64.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["batch", "context_schema", "item_schema", "labels", "model", "name", "on", "profile", "tag", "threshold", "wording_version"]
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
public indirect enum InputRequestDefinitionFieldsTagBatch: JSONRepresentable {
    case alternative0(String)
    case alternative1(Int64)
    public static func read(_ json: JSONValue) throws -> InputRequestDefinitionFieldsTagBatch {
        if let value = try? String.read(json) { return .alternative0(value) }
        if let value = try? Int64.read(json) { return .alternative1(value) }
        throw JSONConversionError("Unknown InputRequestDefinitionFieldsTagBatch value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public indirect enum InputRequestFraming: JSONRepresentable {
    case alternative0(String)
    case alternative1(String)
    case alternative2(String)
    case alternative3(String)
    case alternative4(String)
    public static func read(_ json: JSONValue) throws -> InputRequestFraming {
        if let value = try? String.read(json) { return .alternative0(value) }
        if let value = try? String.read(json) { return .alternative1(value) }
        if let value = try? String.read(json) { return .alternative2(value) }
        if let value = try? String.read(json) { return .alternative3(value) }
        if let value = try? String.read(json) { return .alternative4(value) }
        throw JSONConversionError("Unknown InputRequestFraming value")
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
public indirect enum InputRequestImage: JSONRepresentable {
    case `file`(InputRequestImageFile)
    case `bytes`(InputRequestImageBytes)
    public static func read(_ json: JSONValue) throws -> InputRequestImage {
        let object = try jsonObject(json)
        if object["kind"] == .string("file") { return .`file`(try InputRequestImageFile.read(json)) }
        if object["kind"] == .string("bytes") { return .`bytes`(try InputRequestImageBytes.read(json)) }
        throw JSONConversionError("Unknown InputRequestImage alternative")
    }
    public var json: JSONValue {
        switch self {
        case .`file`(let value): return value.json
        case .`bytes`(let value): return value.json
        }
    }
}
public struct InputRequestImageBytes: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `bytes`: String
    public let `kind`: String
    public let `media`: InputImageMedia
    public init(extensions: [String: JSONValue] = [:], `bytes`: String, `kind`: String = "bytes", `media`: InputImageMedia) { self.extensions = extensions; self.`bytes` = `bytes`; self.`kind` = `kind`; self.`media` = `media` }
    public static func read(_ json: JSONValue) throws -> InputRequestImageBytes {
        let object = try jsonObject(json)
        let known: Set<String> = ["bytes", "kind", "media"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, bytes: try String.read(jsonRequired(object, "bytes")), kind: try String.read(jsonRequired(object, "kind")), media: try InputImageMedia.read(jsonRequired(object, "media")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["bytes", "kind", "media"]
        var object = extensions.filter { !known.contains($0.key) }
        object["bytes"] = `bytes`.json
        object["kind"] = `kind`.json
        object["media"] = `media`.json
        return .object(object)
    }
}
public struct InputRequestImageFile: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `media`: Presence<InputImageMedia>
    public let `path`: String
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "file", `media`: Presence<InputImageMedia> = .absent, `path`: String) { self.extensions = extensions; self.`kind` = `kind`; self.`media` = `media`; self.`path` = `path` }
    public static func read(_ json: JSONValue) throws -> InputRequestImageFile {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "media", "path"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), media: try readPresence(object, "media", InputImageMedia.self), path: try String.read(jsonRequired(object, "path")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "media", "path"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        writePresence(`media`, "media", &object)
        object["path"] = `path`.json
        return .object(object)
    }
}
public indirect enum InputRequestInput: JSONRepresentable {
    case `text`(InputRequestInputText)
    case `json`(InputRequestInputJson)
    case `records`(InputRequestInputRecords)
    case `units`(InputRequestInputUnits)
    case `entities`(InputRequestInputEntities)
    case `source`(InputRequestInputSource)
    case `feed`(InputRequestInputFeed)
    public static func read(_ json: JSONValue) throws -> InputRequestInput {
        let object = try jsonObject(json)
        if object["kind"] == .string("text") { return .`text`(try InputRequestInputText.read(json)) }
        if object["kind"] == .string("json") { return .`json`(try InputRequestInputJson.read(json)) }
        if object["kind"] == .string("records") { return .`records`(try InputRequestInputRecords.read(json)) }
        if object["kind"] == .string("units") { return .`units`(try InputRequestInputUnits.read(json)) }
        if object["kind"] == .string("entities") { return .`entities`(try InputRequestInputEntities.read(json)) }
        if object["kind"] == .string("source") { return .`source`(try InputRequestInputSource.read(json)) }
        if object["kind"] == .string("feed") { return .`feed`(try InputRequestInputFeed.read(json)) }
        throw JSONConversionError("Unknown InputRequestInput alternative")
    }
    public var json: JSONValue {
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
public struct InputRequestInputEntities: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `items`: [InputRequestItem]
    public let `kind`: String
    public init(extensions: [String: JSONValue] = [:], `items`: [InputRequestItem], `kind`: String = "entities") { self.extensions = extensions; self.`items` = `items`; self.`kind` = `kind` }
    public static func read(_ json: JSONValue) throws -> InputRequestInputEntities {
        let object = try jsonObject(json)
        let known: Set<String> = ["items", "kind"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, items: try [InputRequestItem].read(jsonRequired(object, "items")), kind: try String.read(jsonRequired(object, "kind")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["items", "kind"]
        var object = extensions.filter { !known.contains($0.key) }
        object["items"] = `items`.json
        object["kind"] = `kind`.json
        return .object(object)
    }
}
public struct InputRequestInputFeed: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `framing`: Presence<InputRequestFraming>
    public let `images`: Presence<[InputRequestImage]>
    public let `kind`: String
    public let `name`: String
    public let `reading`: Presence<InputRequestReader>
    public init(extensions: [String: JSONValue] = [:], `framing`: Presence<InputRequestFraming> = .absent, `images`: Presence<[InputRequestImage]> = .absent, `kind`: String = "feed", `name`: String, `reading`: Presence<InputRequestReader> = .absent) { self.extensions = extensions; self.`framing` = `framing`; self.`images` = `images`; self.`kind` = `kind`; self.`name` = `name`; self.`reading` = `reading` }
    public static func read(_ json: JSONValue) throws -> InputRequestInputFeed {
        let object = try jsonObject(json)
        let known: Set<String> = ["framing", "images", "kind", "name", "reading"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, framing: try readPresence(object, "framing", InputRequestFraming.self), images: try readPresence(object, "images", [InputRequestImage].self), kind: try String.read(jsonRequired(object, "kind")), name: try String.read(jsonRequired(object, "name")), reading: try readPresence(object, "reading", InputRequestReader.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public struct InputRequestInputJson: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `images`: Presence<[InputRequestImage]>
    public let `kind`: String
    public let `value`: JSONValue
    public init(extensions: [String: JSONValue] = [:], `images`: Presence<[InputRequestImage]> = .absent, `kind`: String = "json", `value`: JSONValue) { self.extensions = extensions; self.`images` = `images`; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> InputRequestInputJson {
        let object = try jsonObject(json)
        let known: Set<String> = ["images", "kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, images: try readPresence(object, "images", [InputRequestImage].self), kind: try String.read(jsonRequired(object, "kind")), value: try JSONValue.read(jsonRequired(object, "value")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["images", "kind", "value"]
        var object = extensions.filter { !known.contains($0.key) }
        writePresence(`images`, "images", &object)
        object["kind"] = `kind`.json
        object["value"] = `value`.json
        return .object(object)
    }
}
public struct InputRequestInputRecords: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `items`: [InputRequestItem]
    public let `kind`: String
    public init(extensions: [String: JSONValue] = [:], `items`: [InputRequestItem], `kind`: String = "records") { self.extensions = extensions; self.`items` = `items`; self.`kind` = `kind` }
    public static func read(_ json: JSONValue) throws -> InputRequestInputRecords {
        let object = try jsonObject(json)
        let known: Set<String> = ["items", "kind"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, items: try [InputRequestItem].read(jsonRequired(object, "items")), kind: try String.read(jsonRequired(object, "kind")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["items", "kind"]
        var object = extensions.filter { !known.contains($0.key) }
        object["items"] = `items`.json
        object["kind"] = `kind`.json
        return .object(object)
    }
}
public struct InputRequestInputSource: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `source`: InputRequestSource
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "source", `source`: InputRequestSource) { self.extensions = extensions; self.`kind` = `kind`; self.`source` = `source` }
    public static func read(_ json: JSONValue) throws -> InputRequestInputSource {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "source"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), source: try InputRequestSource.read(jsonRequired(object, "source")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "source"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        object["source"] = `source`.json
        return .object(object)
    }
}
public struct InputRequestInputText: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `images`: Presence<[InputRequestImage]>
    public let `kind`: String
    public let `text`: String
    public init(extensions: [String: JSONValue] = [:], `images`: Presence<[InputRequestImage]> = .absent, `kind`: String = "text", `text`: String) { self.extensions = extensions; self.`images` = `images`; self.`kind` = `kind`; self.`text` = `text` }
    public static func read(_ json: JSONValue) throws -> InputRequestInputText {
        let object = try jsonObject(json)
        let known: Set<String> = ["images", "kind", "text"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, images: try readPresence(object, "images", [InputRequestImage].self), kind: try String.read(jsonRequired(object, "kind")), text: try String.read(jsonRequired(object, "text")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["images", "kind", "text"]
        var object = extensions.filter { !known.contains($0.key) }
        writePresence(`images`, "images", &object)
        object["kind"] = `kind`.json
        object["text"] = `text`.json
        return .object(object)
    }
}
public struct InputRequestInputUnits: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `items`: [InputRequestItem]
    public let `kind`: String
    public init(extensions: [String: JSONValue] = [:], `items`: [InputRequestItem], `kind`: String = "units") { self.extensions = extensions; self.`items` = `items`; self.`kind` = `kind` }
    public static func read(_ json: JSONValue) throws -> InputRequestInputUnits {
        let object = try jsonObject(json)
        let known: Set<String> = ["items", "kind"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, items: try [InputRequestItem].read(jsonRequired(object, "items")), kind: try String.read(jsonRequired(object, "kind")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["items", "kind"]
        var object = extensions.filter { !known.contains($0.key) }
        object["items"] = `items`.json
        object["kind"] = `kind`.json
        return .object(object)
    }
}
public struct InputRequestItem: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `context`: Presence<InputContextSchema>
    public let `examples`: Presence<[InputRecognitionExample]>
    public let `images`: Presence<[InputRequestImage]>
    public let `options`: Presence<[InputOptionSchema]>
    public let `original`: Presence<InputRequestOriginal>
    public let `seedSpans`: Presence<[InputRecognitionSeedSpan]>
    public init(extensions: [String: JSONValue] = [:], `context`: Presence<InputContextSchema> = .absent, `examples`: Presence<[InputRecognitionExample]> = .absent, `images`: Presence<[InputRequestImage]> = .absent, `options`: Presence<[InputOptionSchema]> = .absent, `original`: Presence<InputRequestOriginal> = .absent, `seedSpans`: Presence<[InputRecognitionSeedSpan]> = .absent) { self.extensions = extensions; self.`context` = `context`; self.`examples` = `examples`; self.`images` = `images`; self.`options` = `options`; self.`original` = `original`; self.`seedSpans` = `seedSpans` }
    public static func read(_ json: JSONValue) throws -> InputRequestItem {
        let object = try jsonObject(json)
        let known: Set<String> = ["context", "examples", "images", "options", "original", "seed_spans"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, context: try readPresence(object, "context", InputContextSchema.self), examples: try readPresence(object, "examples", [InputRecognitionExample].self), images: try readPresence(object, "images", [InputRequestImage].self), options: try readPresence(object, "options", [InputOptionSchema].self), original: try readPresence(object, "original", InputRequestOriginal.self), seedSpans: try readPresence(object, "seed_spans", [InputRecognitionSeedSpan].self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public struct InputRequestOptions: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `attempts`: Presence<Bool>
    public let `batch`: Presence<InputRequestBatch>
    public let `context`: Presence<String>
    public let `contextField`: Presence<String>
    public let `deadlineMs`: Presence<Int64>
    public let `details`: Presence<Bool>
    public let `examples`: Presence<[InputRecognitionExample]>
    public let `examplesField`: Presence<String>
    public let `field`: Presence<[String]>
    public let `filesOnly`: Presence<Bool>
    public let `maxRequestsTotal`: Presence<UInt64>
    public let `mode`: Presence<InputRecognitionMode>
    public let `model`: Presence<String>
    public let `none`: Presence<Bool>
    public let `optionsField`: Presence<String>
    public let `relationThreshold`: Presence<InputRequestThreshold>
    public let `seedSpans`: Presence<[InputRecognitionSeedSpan]>
    public let `seedSpansField`: Presence<String>
    public let `snippetPieces`: Presence<UInt64>
    public let `stageContext`: Presence<InputRecognitionStageContext>
    public let `threshold`: Presence<InputRequestThreshold>
    public let `top`: Presence<UInt64>
    public init(extensions: [String: JSONValue] = [:], `attempts`: Presence<Bool> = .absent, `batch`: Presence<InputRequestBatch> = .absent, `context`: Presence<String> = .absent, `contextField`: Presence<String> = .absent, `deadlineMs`: Presence<Int64> = .absent, `details`: Presence<Bool> = .absent, `examples`: Presence<[InputRecognitionExample]> = .absent, `examplesField`: Presence<String> = .absent, `field`: Presence<[String]> = .absent, `filesOnly`: Presence<Bool> = .absent, `maxRequestsTotal`: Presence<UInt64> = .absent, `mode`: Presence<InputRecognitionMode> = .absent, `model`: Presence<String> = .absent, `none`: Presence<Bool> = .absent, `optionsField`: Presence<String> = .absent, `relationThreshold`: Presence<InputRequestThreshold> = .absent, `seedSpans`: Presence<[InputRecognitionSeedSpan]> = .absent, `seedSpansField`: Presence<String> = .absent, `snippetPieces`: Presence<UInt64> = .absent, `stageContext`: Presence<InputRecognitionStageContext> = .absent, `threshold`: Presence<InputRequestThreshold> = .absent, `top`: Presence<UInt64> = .absent) { self.extensions = extensions; self.`attempts` = `attempts`; self.`batch` = `batch`; self.`context` = `context`; self.`contextField` = `contextField`; self.`deadlineMs` = `deadlineMs`; self.`details` = `details`; self.`examples` = `examples`; self.`examplesField` = `examplesField`; self.`field` = `field`; self.`filesOnly` = `filesOnly`; self.`maxRequestsTotal` = `maxRequestsTotal`; self.`mode` = `mode`; self.`model` = `model`; self.`none` = `none`; self.`optionsField` = `optionsField`; self.`relationThreshold` = `relationThreshold`; self.`seedSpans` = `seedSpans`; self.`seedSpansField` = `seedSpansField`; self.`snippetPieces` = `snippetPieces`; self.`stageContext` = `stageContext`; self.`threshold` = `threshold`; self.`top` = `top` }
    public static func read(_ json: JSONValue) throws -> InputRequestOptions {
        let object = try jsonObject(json)
        let known: Set<String> = ["attempts", "batch", "context", "context_field", "deadline_ms", "details", "examples", "examples_field", "field", "files_only", "max_requests_total", "mode", "model", "none", "options_field", "relation_threshold", "seed_spans", "seed_spans_field", "snippet_pieces", "stage_context", "threshold", "top"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, attempts: try readPresence(object, "attempts", Bool.self), batch: try readPresence(object, "batch", InputRequestBatch.self), context: try readPresence(object, "context", String.self), contextField: try readPresence(object, "context_field", String.self), deadlineMs: try readPresence(object, "deadline_ms", Int64.self), details: try readPresence(object, "details", Bool.self), examples: try readPresence(object, "examples", [InputRecognitionExample].self), examplesField: try readPresence(object, "examples_field", String.self), field: try readPresence(object, "field", [String].self), filesOnly: try readPresence(object, "files_only", Bool.self), maxRequestsTotal: try readPresence(object, "max_requests_total", UInt64.self), mode: try readPresence(object, "mode", InputRecognitionMode.self), model: try readPresence(object, "model", String.self), none: try readPresence(object, "none", Bool.self), optionsField: try readPresence(object, "options_field", String.self), relationThreshold: try readPresence(object, "relation_threshold", InputRequestThreshold.self), seedSpans: try readPresence(object, "seed_spans", [InputRecognitionSeedSpan].self), seedSpansField: try readPresence(object, "seed_spans_field", String.self), snippetPieces: try readPresence(object, "snippet_pieces", UInt64.self), stageContext: try readPresence(object, "stage_context", InputRecognitionStageContext.self), threshold: try readPresence(object, "threshold", InputRequestThreshold.self), top: try readPresence(object, "top", UInt64.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["attempts", "batch", "context", "context_field", "deadline_ms", "details", "examples", "examples_field", "field", "files_only", "max_requests_total", "mode", "model", "none", "options_field", "relation_threshold", "seed_spans", "seed_spans_field", "snippet_pieces", "stage_context", "threshold", "top"]
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
public indirect enum InputRequestOriginal: JSONRepresentable {
    case `text`(InputRequestOriginalText)
    case `json`(InputRequestOriginalJson)
    public static func read(_ json: JSONValue) throws -> InputRequestOriginal {
        let object = try jsonObject(json)
        if object["kind"] == .string("text") { return .`text`(try InputRequestOriginalText.read(json)) }
        if object["kind"] == .string("json") { return .`json`(try InputRequestOriginalJson.read(json)) }
        throw JSONConversionError("Unknown InputRequestOriginal alternative")
    }
    public var json: JSONValue {
        switch self {
        case .`text`(let value): return value.json
        case .`json`(let value): return value.json
        }
    }
}
public struct InputRequestOriginalJson: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `value`: JSONValue
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "json", `value`: JSONValue) { self.extensions = extensions; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> InputRequestOriginalJson {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), value: try JSONValue.read(jsonRequired(object, "value")))
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
public struct InputRequestOriginalText: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `text`: String
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "text", `text`: String) { self.extensions = extensions; self.`kind` = `kind`; self.`text` = `text` }
    public static func read(_ json: JSONValue) throws -> InputRequestOriginalText {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "text"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), text: try String.read(jsonRequired(object, "text")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "text"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        object["text"] = `text`.json
        return .object(object)
    }
}
public indirect enum InputRequestQuestion: JSONRepresentable {
    case `text`(InputRequestQuestionText)
    case `definition`(InputRequestQuestionDefinition)
    case `file`(InputRequestQuestionFile)
    case `name`(InputRequestQuestionName)
    case `reference`(InputRequestQuestionReference)
    public static func read(_ json: JSONValue) throws -> InputRequestQuestion {
        let object = try jsonObject(json)
        if object["kind"] == .string("text") { return .`text`(try InputRequestQuestionText.read(json)) }
        if object["kind"] == .string("definition") { return .`definition`(try InputRequestQuestionDefinition.read(json)) }
        if object["kind"] == .string("file") { return .`file`(try InputRequestQuestionFile.read(json)) }
        if object["kind"] == .string("name") { return .`name`(try InputRequestQuestionName.read(json)) }
        if object["kind"] == .string("reference") { return .`reference`(try InputRequestQuestionReference.read(json)) }
        throw JSONConversionError("Unknown InputRequestQuestion alternative")
    }
    public var json: JSONValue {
        switch self {
        case .`text`(let value): return value.json
        case .`definition`(let value): return value.json
        case .`file`(let value): return value.json
        case .`name`(let value): return value.json
        case .`reference`(let value): return value.json
        }
    }
}
public struct InputRequestQuestionDefinition: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `value`: InputRequestDefinition
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "definition", `value`: InputRequestDefinition) { self.extensions = extensions; self.`kind` = `kind`; self.`value` = `value` }
    public static func read(_ json: JSONValue) throws -> InputRequestQuestionDefinition {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "value"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), value: try InputRequestDefinition.read(jsonRequired(object, "value")))
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
public struct InputRequestQuestionFile: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `path`: String
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "file", `path`: String) { self.extensions = extensions; self.`kind` = `kind`; self.`path` = `path` }
    public static func read(_ json: JSONValue) throws -> InputRequestQuestionFile {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "path"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), path: try String.read(jsonRequired(object, "path")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "path"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        object["path"] = `path`.json
        return .object(object)
    }
}
public struct InputRequestQuestionName: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `name`: String
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "name", `name`: String) { self.extensions = extensions; self.`kind` = `kind`; self.`name` = `name` }
    public static func read(_ json: JSONValue) throws -> InputRequestQuestionName {
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
public struct InputRequestQuestionReference: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `reference`: String
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "reference", `reference`: String) { self.extensions = extensions; self.`kind` = `kind`; self.`reference` = `reference` }
    public static func read(_ json: JSONValue) throws -> InputRequestQuestionReference {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "reference"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), reference: try String.read(jsonRequired(object, "reference")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "reference"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        object["reference"] = `reference`.json
        return .object(object)
    }
}
public struct InputRequestQuestionText: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `text`: String
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "text", `text`: String) { self.extensions = extensions; self.`kind` = `kind`; self.`text` = `text` }
    public static func read(_ json: JSONValue) throws -> InputRequestQuestionText {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "text"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), text: try String.read(jsonRequired(object, "text")))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "text"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        object["text"] = `text`.json
        return .object(object)
    }
}
public struct InputRequestReader: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `unit`: Presence<InputSourceUnit>
    public let `window`: Presence<UInt64>
    public init(extensions: [String: JSONValue] = [:], `unit`: Presence<InputSourceUnit> = .absent, `window`: Presence<UInt64> = .absent) { self.extensions = extensions; self.`unit` = `unit`; self.`window` = `window` }
    public static func read(_ json: JSONValue) throws -> InputRequestReader {
        let object = try jsonObject(json)
        let known: Set<String> = ["unit", "window"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, unit: try readPresence(object, "unit", InputSourceUnit.self), window: try readPresence(object, "window", UInt64.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["unit", "window"]
        var object = extensions.filter { !known.contains($0.key) }
        writePresence(`unit`, "unit", &object)
        writePresence(`window`, "window", &object)
        return .object(object)
    }
}
public indirect enum InputRequestReaderFailure: JSONRepresentable {
    case `io`(InputRequestReaderFailureIo)
    case `utf8`(InputRequestReaderFailureUtf8)
    case `invalidInput`(InputRequestReaderFailureInvalidInput)
    public static func read(_ json: JSONValue) throws -> InputRequestReaderFailure {
        let object = try jsonObject(json)
        if object["kind"] == .string("io") { return .`io`(try InputRequestReaderFailureIo.read(json)) }
        if object["kind"] == .string("utf8") { return .`utf8`(try InputRequestReaderFailureUtf8.read(json)) }
        if object["kind"] == .string("invalid_input") { return .`invalidInput`(try InputRequestReaderFailureInvalidInput.read(json)) }
        throw JSONConversionError("Unknown InputRequestReaderFailure alternative")
    }
    public var json: JSONValue {
        switch self {
        case .`io`(let value): return value.json
        case .`utf8`(let value): return value.json
        case .`invalidInput`(let value): return value.json
        }
    }
}
public struct InputRequestReaderFailureInvalidInput: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `location`: Presence<InputSessionSourceLocation>
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "invalid_input", `location`: Presence<InputSessionSourceLocation> = .absent) { self.extensions = extensions; self.`kind` = `kind`; self.`location` = `location` }
    public static func read(_ json: JSONValue) throws -> InputRequestReaderFailureInvalidInput {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "location"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), location: try readPresence(object, "location", InputSessionSourceLocation.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "location"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        writePresence(`location`, "location", &object)
        return .object(object)
    }
}
public struct InputRequestReaderFailureIo: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `location`: Presence<InputSessionSourceLocation>
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "io", `location`: Presence<InputSessionSourceLocation> = .absent) { self.extensions = extensions; self.`kind` = `kind`; self.`location` = `location` }
    public static func read(_ json: JSONValue) throws -> InputRequestReaderFailureIo {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "location"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), location: try readPresence(object, "location", InputSessionSourceLocation.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "location"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        writePresence(`location`, "location", &object)
        return .object(object)
    }
}
public struct InputRequestReaderFailureUtf8: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `kind`: String
    public let `location`: Presence<InputSessionSourceLocation>
    public init(extensions: [String: JSONValue] = [:], `kind`: String = "utf8", `location`: Presence<InputSessionSourceLocation> = .absent) { self.extensions = extensions; self.`kind` = `kind`; self.`location` = `location` }
    public static func read(_ json: JSONValue) throws -> InputRequestReaderFailureUtf8 {
        let object = try jsonObject(json)
        let known: Set<String> = ["kind", "location"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, kind: try String.read(jsonRequired(object, "kind")), location: try readPresence(object, "location", InputSessionSourceLocation.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["kind", "location"]
        var object = extensions.filter { !known.contains($0.key) }
        object["kind"] = `kind`.json
        writePresence(`location`, "location", &object)
        return .object(object)
    }
}
public struct InputRequestSessionDescriptor: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `item`: InputRequestItem
    public let `location`: Presence<InputSessionSourceLocation>
    public init(extensions: [String: JSONValue] = [:], `item`: InputRequestItem, `location`: Presence<InputSessionSourceLocation> = .absent) { self.extensions = extensions; self.`item` = `item`; self.`location` = `location` }
    public static func read(_ json: JSONValue) throws -> InputRequestSessionDescriptor {
        let object = try jsonObject(json)
        let known: Set<String> = ["item", "location"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, item: try InputRequestItem.read(jsonRequired(object, "item")), location: try readPresence(object, "location", InputSessionSourceLocation.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
        if let _sourceJSON { return _sourceJSON }
        let known: Set<String> = ["item", "location"]
        var object = extensions.filter { !known.contains($0.key) }
        object["item"] = `item`.json
        writePresence(`location`, "location", &object)
        return .object(object)
    }
}
public struct InputRequestSource: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `framing`: Presence<InputRequestFraming>
    public let `media`: Presence<InputReaderMedia>
    public let `paths`: [String]
    public let `reading`: Presence<InputRequestReader>
    public init(extensions: [String: JSONValue] = [:], `framing`: Presence<InputRequestFraming> = .absent, `media`: Presence<InputReaderMedia> = .absent, `paths`: [String], `reading`: Presence<InputRequestReader> = .absent) { self.extensions = extensions; self.`framing` = `framing`; self.`media` = `media`; self.`paths` = `paths`; self.`reading` = `reading` }
    public static func read(_ json: JSONValue) throws -> InputRequestSource {
        let object = try jsonObject(json)
        let known: Set<String> = ["framing", "media", "paths", "reading"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, framing: try readPresence(object, "framing", InputRequestFraming.self), media: try readPresence(object, "media", InputReaderMedia.self), paths: try [String].read(jsonRequired(object, "paths")), reading: try readPresence(object, "reading", InputRequestReader.self))
        value._sourceJSON = json
        return value
    }
    public var json: JSONValue {
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
public indirect enum InputRequestThreshold: JSONRepresentable {
    case alternative0(Double)
    case alternative1(String)
    public static func read(_ json: JSONValue) throws -> InputRequestThreshold {
        if let value = try? Double.read(json) { return .alternative0(value) }
        if let value = try? String.read(json) { return .alternative1(value) }
        throw JSONConversionError("Unknown InputRequestThreshold value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        }
    }
}
public indirect enum InputRequestVersion: JSONRepresentable {
    case alternative0(String)
    public static func read(_ json: JSONValue) throws -> InputRequestVersion {
        if let value = try? String.read(json) { return .alternative0(value) }
        throw JSONConversionError("Unknown InputRequestVersion value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        }
    }
}
public struct InputSessionSourceLocation: JSONRepresentable {
    public let extensions: [String: JSONValue]
    private var _sourceJSON: JSONValue? = nil
    public let `file`: String
    public let `firstLine`: Presence<UInt64>
    public let `lastLine`: Presence<UInt64>
    public init(extensions: [String: JSONValue] = [:], `file`: String, `firstLine`: Presence<UInt64> = .absent, `lastLine`: Presence<UInt64> = .absent) { self.extensions = extensions; self.`file` = `file`; self.`firstLine` = `firstLine`; self.`lastLine` = `lastLine` }
    public static func read(_ json: JSONValue) throws -> InputSessionSourceLocation {
        let object = try jsonObject(json)
        let known: Set<String> = ["file", "first_line", "last_line"]
        var value = Self(extensions: object.filter { !known.contains($0.key) }, file: try String.read(jsonRequired(object, "file")), firstLine: try readPresence(object, "first_line", UInt64.self), lastLine: try readPresence(object, "last_line", UInt64.self))
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
public indirect enum InputSourceUnit: JSONRepresentable {
    case alternative0(String)
    case alternative1(String)
    case alternative2(String)
    public static func read(_ json: JSONValue) throws -> InputSourceUnit {
        if let value = try? String.read(json) { return .alternative0(value) }
        if let value = try? String.read(json) { return .alternative1(value) }
        if let value = try? String.read(json) { return .alternative2(value) }
        throw JSONConversionError("Unknown InputSourceUnit value")
    }
    public var json: JSONValue {
        switch self {
        case .alternative0(let value): return value.json
        case .alternative1(let value): return value.json
        case .alternative2(let value): return value.json
        }
    }
}
public let ownedRequestVersion = "thinkthen.request/1"
