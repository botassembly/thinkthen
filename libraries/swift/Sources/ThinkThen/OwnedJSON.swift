import Foundation
import CThinkThen

public struct JSONConversionError: Error, Sendable, CustomStringConvertible {
    public let description: String
    public init(_ description: String) { self.description = description }
}

public struct JSONNumber: Sendable, Equatable {
    public let rawValue: String
    public init(_ rawValue: String) throws {
        let parsed = try JSONValue.parse(Data(rawValue.utf8))
        guard case .number(let number) = parsed.kind, number.rawValue == rawValue else {
            throw JSONConversionError("Expected one JSON number")
        }
        self.rawValue = rawValue
    }
    init(unchecked rawValue: String) { self.rawValue = rawValue }
}

public protocol JSONRepresentable: Sendable {
    static func read(_ json: JSONValue) throws -> Self
    var json: JSONValue { get }
}

public struct JSONMember: Sendable, Equatable {
    public let key: String
    public let value: JSONValue
    public init(_ key: String, _ value: JSONValue) { self.key = key; self.value = value }
}

public indirect enum JSONKind: Sendable, Equatable {
    case null, bool(Bool), string(String), number(JSONNumber)
    case array([JSONValue]), object([String: JSONValue]), orderedObject([JSONMember])
}

public struct JSONValue: JSONRepresentable, Equatable {
    public let kind: JSONKind
    private let original: Data?
    private init(_ kind: JSONKind, original: Data? = nil) { self.kind = kind; self.original = original }
    public static let null = JSONValue(.null)
    public static func bool(_ value: Bool) -> JSONValue { JSONValue(.bool(value)) }
    public static func string(_ value: String) -> JSONValue { JSONValue(.string(value)) }
    public static func number(_ value: JSONNumber) -> JSONValue { JSONValue(.number(value)) }
    public static func array(_ value: [JSONValue]) -> JSONValue { JSONValue(.array(value)) }
    public static func object(_ value: [String: JSONValue]) -> JSONValue { JSONValue(.object(value)) }
    public static func orderedObject(_ value: [JSONMember]) -> JSONValue { JSONValue(.orderedObject(value)) }
    public static func == (left: JSONValue, right: JSONValue) -> Bool { left.kind == right.kind }
    public static func read(_ json: JSONValue) throws -> JSONValue { json }
    public var json: JSONValue { self }
    public static func parse(_ data: Data) throws -> JSONValue {
        try requireNativePackage()
        var owner: OpaquePointer?
        try data.withUnsafeBytes { raw in
            try sessionCheck(thinkthen_value_parse(raw.bindMemory(to: CChar.self).baseAddress, raw.count, &owner))
        }
        guard let owner else { throw JSONConversionError("Missing native value") }
        defer { thinkthen_value_free(owner) }
        let copied = try nativeValue(owner)
        return JSONValue(copied.kind, original: data)
    }
    public func data() throws -> Data {
        func text(_ value: JSONValue) throws -> String {
            if let original = value.original { return String(decoding: original, as: UTF8.self) }
            switch value.kind {
            case .null: return "null"
            case .bool(let value): return value ? "true" : "false"
            case .string(let value): return String(decoding: try JSONEncoder().encode(value), as: UTF8.self)
            case .number(let value):
                _ = try JSONNumber(value.rawValue)
                return value.rawValue
            case .array(let values): return "[" + (try values.map(text)).joined(separator: ",") + "]"
            case .object(let values):
                return "{" + (try values.map { key, value in
                    try text(.string(key)) + ":" + text(value)
                }).joined(separator: ",") + "}"
            case .orderedObject(let values):
                return "{" + (try values.map { try text(.string($0.key)) + ":" + text($0.value) }).joined(separator: ",") + "}"
            }
        }
        return Data(try text(self).utf8)
    }
}

func nativeString(_ value: thinkthen_complete_utf8_v1) -> String {
    String(decoding: UnsafeRawBufferPointer(start: value.data, count: value.len), as: UTF8.self)
}

func nativeValue(_ owner: OpaquePointer) throws -> JSONValue {
    var view: UnsafePointer<thinkthen_complete_json_v1>?
    try sessionCheck(thinkthen_value_view(owner, &view))
    guard let view else { throw JSONConversionError("Missing native value view") }
    return try JSONValue.copyNative(view)
}

extension JSONValue {
    static func copyNative(_ pointer: UnsafePointer<thinkthen_complete_json_v1>) throws -> JSONValue {
        let value = pointer.pointee
        switch value.kind {
        case UInt32(THINKTHEN_COMPLETE_JSON_NULL_V1): return .null
        case UInt32(THINKTHEN_COMPLETE_JSON_BOOLEAN_V1): return .bool(value.data.boolean != 0)
        case UInt32(THINKTHEN_COMPLETE_JSON_NUMBER_V1): return .number(JSONNumber(unchecked: nativeString(value.data.number)))
        case UInt32(THINKTHEN_COMPLETE_JSON_STRING_V1): return .string(nativeString(value.data.string))
        case UInt32(THINKTHEN_COMPLETE_JSON_ARRAY_V1):
            let values = value.data.array
            return .array(try (0..<values.len).map {
                guard let child = values.data?[$0] else { throw JSONConversionError("Missing native array value") }
                return try copyNative(child)
            })
        case UInt32(THINKTHEN_COMPLETE_JSON_OBJECT_V1):
            let values = value.data.object
            return .orderedObject(try (0..<values.len).map {
                guard let member = values.data?[$0], let child = member.value else { throw JSONConversionError("Missing native object value") }
                return JSONMember(nativeString(member.name), try copyNative(child))
            })
        default: throw JSONConversionError("Unknown native value kind")
        }
    }
}

public enum Presence<Value: Sendable>: Sendable {
    case absent, null, value(Value)
    public var value: Value? { if case .value(let value) = self { return value }; return nil }
    public var isPresent: Bool { if case .absent = self { return false }; return true }
}

func jsonObject(_ json: JSONValue) throws -> [String: JSONValue] {
    switch json.kind {
    case .object(let object): return object
    case .orderedObject(let members): return members.reduce(into: [:]) { $0[$1.key] = $1.value }
    default: throw JSONConversionError("Expected JSON object")
    }
}
func jsonRequired(_ object: [String: JSONValue], _ key: String) throws -> JSONValue {
    guard let value = object[key] else { throw JSONConversionError("Missing JSON member: " + key) }
    return value
}
func readPresence<T: JSONRepresentable>(_ object: [String: JSONValue], _ key: String, _ type: T.Type) throws -> Presence<T> {
    guard let value = object[key] else { return .absent }
    if value == .null { return .null }
    return .value(try T.read(value))
}
func writePresence<T: JSONRepresentable>(_ value: Presence<T>, _ key: String, _ object: inout [String: JSONValue]) {
    switch value { case .absent: break; case .null: object[key] = .null; case .value(let value): object[key] = value.json }
}

extension String: JSONRepresentable {
    public static func read(_ json: JSONValue) throws -> String {
        guard case .string(let value) = json.kind else { throw JSONConversionError("Expected JSON string") }; return value
    }
    public var json: JSONValue { .string(self) }
}
extension Bool: JSONRepresentable {
    public static func read(_ json: JSONValue) throws -> Bool {
        guard case .bool(let value) = json.kind else { throw JSONConversionError("Expected JSON boolean") }; return value
    }
    public var json: JSONValue { .bool(self) }
}
extension Int64: JSONRepresentable {
    public static func read(_ json: JSONValue) throws -> Int64 {
        guard case .number(let value) = json.kind, let parsed = Int64(value.rawValue) else { throw JSONConversionError("Expected signed JSON integer") }; return parsed
    }
    public var json: JSONValue { .number(JSONNumber(unchecked: String(self))) }
}
extension UInt64: JSONRepresentable {
    public static func read(_ json: JSONValue) throws -> UInt64 {
        guard case .number(let value) = json.kind, let parsed = UInt64(value.rawValue) else { throw JSONConversionError("Expected unsigned JSON integer") }; return parsed
    }
    public var json: JSONValue { .number(JSONNumber(unchecked: String(self))) }
}
extension Double: JSONRepresentable {
    public static func read(_ json: JSONValue) throws -> Double {
        guard case .number(let value) = json.kind, let parsed = Double(value.rawValue), parsed.isFinite else { throw JSONConversionError("Expected finite JSON number") }; return parsed
    }
    public var json: JSONValue { .number(JSONNumber(unchecked: String(self))) }
}
extension Optional: JSONRepresentable where Wrapped: JSONRepresentable {
    public static func read(_ json: JSONValue) throws -> Self { json == .null ? nil : try Wrapped.read(json) }
    public var json: JSONValue { self?.json ?? .null }
}
extension Array: JSONRepresentable where Element: JSONRepresentable {
    public static func read(_ json: JSONValue) throws -> Self {
        guard case .array(let values) = json.kind else { throw JSONConversionError("Expected JSON array") }; return try values.map(Element.read)
    }
    public var json: JSONValue { .array(map(\.json)) }
}
extension Dictionary: JSONRepresentable where Key == String, Value: JSONRepresentable {
    public static func read(_ json: JSONValue) throws -> Self { try jsonObject(json).mapValues(Value.read) }
    public var json: JSONValue { .object(mapValues(\.json)) }
}
