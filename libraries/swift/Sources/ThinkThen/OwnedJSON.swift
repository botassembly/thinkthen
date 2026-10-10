import Foundation

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
        var parser = JSONParser(bytes: Array(data))
        let value = try parser.value()
        parser.space()
        guard parser.index == parser.bytes.count else { throw JSONConversionError("Trailing JSON content") }
        return JSONValue(value.kind, original: data)
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
                return "{" + (try values.keys.sorted().map { key in
                    try text(.string(key)) + ":" + text(values[key]!)
                }).joined(separator: ",") + "}"
            case .orderedObject(let values):
                return "{" + (try values.map { try text(.string($0.key)) + ":" + text($0.value) }).joined(separator: ",") + "}"
            }
        }
        return Data(try text(self).utf8)
    }
}

private struct JSONParser {
    let bytes: [UInt8]
    var index = 0
    mutating func space() { while index < bytes.count && [9, 10, 13, 32].contains(bytes[index]) { index += 1 } }
    mutating func consume(_ byte: UInt8) -> Bool {
        space()
        if index < bytes.count && bytes[index] == byte { index += 1; return true }
        return false
    }
    mutating func string() throws -> String {
        space()
        let start = index
        guard index < bytes.count && bytes[index] == 34 else { throw JSONConversionError("Expected JSON string") }
        index += 1
        while index < bytes.count {
            let byte = bytes[index]
            index += 1
            if byte == 34 { return try JSONDecoder().decode(String.self, from: Data(bytes[start..<index])) }
            if byte == 92 { index += 1 }
        }
        throw JSONConversionError("Unterminated JSON string")
    }
    mutating func value() throws -> JSONValue {
        space()
        guard index < bytes.count else { throw JSONConversionError("Missing JSON value") }
        switch bytes[index] {
        case 34: return .string(try string())
        case 91:
            index += 1
            var values: [JSONValue] = []
            if consume(93) { return .array(values) }
            repeat { values.append(try value()) } while consume(44)
            guard consume(93) else { throw JSONConversionError("Expected JSON array end") }
            return .array(values)
        case 123:
            index += 1
            var values: [JSONMember] = []
            if consume(125) { return .orderedObject(values) }
            repeat {
                let key = try string()
                guard consume(58) else { throw JSONConversionError("Expected JSON colon") }
                values.append(JSONMember(key, try value()))
            } while consume(44)
            guard consume(125) else { throw JSONConversionError("Expected JSON object end") }
            return .orderedObject(values)
        case 110, 116, 102:
            for (literal, value) in [("null", JSONValue.null), ("true", .bool(true)), ("false", .bool(false))] {
                let token = Array(literal.utf8)
                if index + token.count <= bytes.count && Array(bytes[index..<index + token.count]) == token {
                    index += token.count
                    return value
                }
            }
            throw JSONConversionError("Invalid JSON literal")
        default:
            let start = index
            if bytes[index] == 45 { index += 1 }
            guard index < bytes.count else { throw JSONConversionError("Invalid JSON number") }
            if bytes[index] == 48 { index += 1 }
            else {
                guard (49...57).contains(bytes[index]) else { throw JSONConversionError("Invalid JSON number") }
                while index < bytes.count && (48...57).contains(bytes[index]) { index += 1 }
            }
            if index < bytes.count && bytes[index] == 46 {
                index += 1
                let fraction = index
                while index < bytes.count && (48...57).contains(bytes[index]) { index += 1 }
                guard index > fraction else { throw JSONConversionError("Invalid JSON fraction") }
            }
            if index < bytes.count && [69, 101].contains(bytes[index]) {
                index += 1
                if index < bytes.count && [43, 45].contains(bytes[index]) { index += 1 }
                let exponent = index
                while index < bytes.count && (48...57).contains(bytes[index]) { index += 1 }
                guard index > exponent else { throw JSONConversionError("Invalid JSON exponent") }
            }
            return .number(JSONNumber(unchecked: String(decoding: bytes[start..<index], as: UTF8.self)))
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
