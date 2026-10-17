import Foundation
import CThinkThen

struct ValueConversionError: Error, Sendable, CustomStringConvertible {
    public let description: String
    public init(_ description: String) { self.description = description }
}

public struct NumberLiteral: Sendable, Equatable {
    public let rawValue: String
    public init(_ rawValue: String) throws {
        let parsed = try DataValue.parse(Data(rawValue.utf8))
        guard case .number(let number) = parsed.kind, number.rawValue == rawValue else {
            throw ValueConversionError("Expected one JSON number")
        }
        self.rawValue = rawValue
    }
    init(unchecked rawValue: String) { self.rawValue = rawValue }
}

protocol ValueRepresentable: Sendable {
    static func read(_ json: DataValue) throws -> Self
    var json: DataValue { get }
}

public struct ValueMember: Sendable, Equatable {
    public let key: String
    public let value: DataValue
    public init(_ key: String, _ value: DataValue) { self.key = key; self.value = value }
}

public indirect enum ValueKind: Sendable, Equatable {
    case null, bool(Bool), string(String), number(NumberLiteral)
    case array([DataValue]), object([String: DataValue]), orderedObject([ValueMember])
}

public struct DataValue: ValueRepresentable, Equatable {
    public let kind: ValueKind
    private let original: Data?
    private init(_ kind: ValueKind, original: Data? = nil) { self.kind = kind; self.original = original }
    public static let null = DataValue(.null)
    public static func bool(_ value: Bool) -> DataValue { DataValue(.bool(value)) }
    public static func string(_ value: String) -> DataValue { DataValue(.string(value)) }
    public static func number(_ value: NumberLiteral) -> DataValue { DataValue(.number(value)) }
    public static func array(_ value: [DataValue]) -> DataValue { DataValue(.array(value)) }
    public static func object(_ value: [String: DataValue]) -> DataValue { DataValue(.object(value)) }
    public static func orderedObject(_ value: [ValueMember]) -> DataValue { DataValue(.orderedObject(value)) }
    public static func == (left: DataValue, right: DataValue) -> Bool { left.kind == right.kind }
    static func read(_ json: DataValue) throws -> DataValue { json }
    var json: DataValue { self }
    static func parse(_ data: Data) throws -> DataValue {
        try requireNativePackage()
        var owner: OpaquePointer?
        try data.withUnsafeBytes { raw in
            try sessionCheck(thinkthen_value_parse(raw.bindMemory(to: CChar.self).baseAddress, raw.count, &owner))
        }
        guard let owner else { throw ValueConversionError("Missing native value") }
        defer { thinkthen_value_free(owner) }
        let copied = try nativeValue(owner)
        return DataValue(copied.kind, original: data)
    }
    func data() throws -> Data {
        func text(_ value: DataValue) throws -> String {
            if let original = value.original { return String(decoding: original, as: UTF8.self) }
            switch value.kind {
            case .null: return "null"
            case .bool(let value): return value ? "true" : "false"
            case .string(let value): return String(decoding: try JSONEncoder().encode(value), as: UTF8.self)
            case .number(let value):
                _ = try NumberLiteral(value.rawValue)
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

func nativeString(_ value: thinkthen_complete_utf8_v1) -> String { String(decoding: UnsafeRawBufferPointer(start: value.data, count: value.len), as: UTF8.self) }


extension DataValue {
    static func copyNative(_ pointer: UnsafePointer<thinkthen_complete_json_v1>) throws -> DataValue {
        let value = pointer.pointee
        switch value.kind {
        case UInt32(THINKTHEN_COMPLETE_JSON_NULL_V1): return .null
        case UInt32(THINKTHEN_COMPLETE_JSON_BOOLEAN_V1): return .bool(value.data.boolean != 0)
        case UInt32(THINKTHEN_COMPLETE_JSON_NUMBER_V1): return .number(NumberLiteral(unchecked: nativeString(value.data.number)))
        case UInt32(THINKTHEN_COMPLETE_JSON_STRING_V1): return .string(nativeString(value.data.string))
        case UInt32(THINKTHEN_COMPLETE_JSON_ARRAY_V1):
            let values = value.data.array
            return .array(try (0..<values.len).map {
                guard let child = values.data?[$0] else { throw ValueConversionError("Missing native array value") }
                return try copyNative(child)
            })
        case UInt32(THINKTHEN_COMPLETE_JSON_OBJECT_V1):
            let values = value.data.object
            return .orderedObject(try (0..<values.len).map {
                guard let member = values.data?[$0], let child = member.value else { throw ValueConversionError("Missing native object value") }
                return ValueMember(nativeString(member.name), try copyNative(child))
            })
        default: throw ValueConversionError("Unknown native value kind")
        }
    }
}

public enum Presence<Value: Sendable>: Sendable {
    case absent, null, value(Value)
    public var value: Value? { if case .value(let value) = self { return value }; return nil }
    public var isPresent: Bool { if case .absent = self { return false }; return true }
}

func jsonObject(_ json: DataValue) throws -> [String: DataValue] {
    switch json.kind {
    case .object(let object): return object
    case .orderedObject(let members): return members.reduce(into: [:]) { $0[$1.key] = $1.value }
    default: throw ValueConversionError("Expected JSON object")
    }
}
func jsonRequired(_ object: [String: DataValue], _ key: String) throws -> DataValue {
    guard let value = object[key] else { throw ValueConversionError("Missing JSON member: " + key) }
    return value
}
func readPresence<T: ValueRepresentable>(_ object: [String: DataValue], _ key: String, _ type: T.Type) throws -> Presence<T> {
    guard let value = object[key] else { return .absent }
    if value == .null { return .null }
    return .value(try T.read(value))
}
func writePresence<T: ValueRepresentable>(_ value: Presence<T>, _ key: String, _ object: inout [String: DataValue]) {
    switch value { case .absent: break; case .null: object[key] = .null; case .value(let value): object[key] = value.json }
}

extension String: ValueRepresentable {
    static func read(_ json: DataValue) throws -> String {
        guard case .string(let value) = json.kind else { throw ValueConversionError("Expected JSON string") }; return value
    }
    var json: DataValue { .string(self) }
}
extension Bool: ValueRepresentable {
    static func read(_ json: DataValue) throws -> Bool {
        guard case .bool(let value) = json.kind else { throw ValueConversionError("Expected JSON boolean") }; return value
    }
    var json: DataValue { .bool(self) }
}
extension Int64: ValueRepresentable {
    static func read(_ json: DataValue) throws -> Int64 {
        guard case .number(let value) = json.kind, let parsed = Int64(value.rawValue) else { throw ValueConversionError("Expected signed JSON integer") }; return parsed
    }
    var json: DataValue { .number(NumberLiteral(unchecked: String(self))) }
}
extension UInt64: ValueRepresentable {
    static func read(_ json: DataValue) throws -> UInt64 {
        guard case .number(let value) = json.kind, let parsed = UInt64(value.rawValue) else { throw ValueConversionError("Expected unsigned JSON integer") }; return parsed
    }
    var json: DataValue { .number(NumberLiteral(unchecked: String(self))) }
}
extension Double: ValueRepresentable {
    static func read(_ json: DataValue) throws -> Double {
        guard case .number(let value) = json.kind, let parsed = Double(value.rawValue), parsed.isFinite else { throw ValueConversionError("Expected finite JSON number") }; return parsed
    }
    var json: DataValue { .number(NumberLiteral(unchecked: String(self))) }
}
extension Optional: ValueRepresentable where Wrapped: ValueRepresentable {
    static func read(_ json: DataValue) throws -> Self { json == .null ? nil : try Wrapped.read(json) }
    var json: DataValue { self?.json ?? .null }
}
extension Array: ValueRepresentable where Element: ValueRepresentable {
    static func read(_ json: DataValue) throws -> Self {
        guard case .array(let values) = json.kind else { throw ValueConversionError("Expected JSON array") }; return try values.map(Element.read)
    }
    var json: DataValue { .array(map(\.json)) }
}
extension Dictionary: ValueRepresentable where Key == String, Value: ValueRepresentable {
    static func read(_ json: DataValue) throws -> Self { try jsonObject(json).mapValues(Value.read) }
    var json: DataValue { .object(mapValues(\.json)) }
}
