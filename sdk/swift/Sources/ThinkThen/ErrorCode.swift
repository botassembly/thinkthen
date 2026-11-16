// Generated from the engine registry during packaging.
/// A stable engine error name, including names from newer engines.
public struct ErrorCode: RawRepresentable, Sendable, Codable, Hashable, CaseIterable, CustomStringConvertible {
  /// The stable engine diagnostic name.
  public let rawValue: String
  /// Preserve an engine name, including names added by a newer engine.
  public init(rawValue: String) { self.rawValue = rawValue }
  public var description: String { rawValue }
  /// Engine code `usage.threshold`.
  public static let usageThreshold = Self(rawValue: "usage.threshold")
  /// Engine code `usage.question`.
  public static let usageQuestion = Self(rawValue: "usage.question")
  /// Engine code `usage.name`.
  public static let usageName = Self(rawValue: "usage.name")
  /// Engine code `usage.settings`.
  public static let usageSettings = Self(rawValue: "usage.settings")
  /// Engine code `usage.backend`.
  public static let usageBackend = Self(rawValue: "usage.backend")
  /// Engine code `usage.url`.
  public static let usageURL = Self(rawValue: "usage.url")
  /// Engine code `usage.key-missing`.
  public static let usageKeyMissing = Self(rawValue: "usage.key-missing")
  /// Engine code `usage.batch`.
  public static let usageBatch = Self(rawValue: "usage.batch")
  /// Engine code `usage.pointer`.
  public static let usagePointer = Self(rawValue: "usage.pointer")
  /// Engine code `usage.sql-policy`.
  public static let usageSQLPolicy = Self(rawValue: "usage.sql-policy")
  /// Engine code `usage.sql-selection`.
  public static let usageSQLSelection = Self(rawValue: "usage.sql-selection")
  /// Engine code `usage.abi`.
  public static let usageABI = Self(rawValue: "usage.abi")
  /// Engine code `usage.closed`.
  public static let usageClosed = Self(rawValue: "usage.closed")
  /// Engine code `usage.function`.
  public static let usageFunction = Self(rawValue: "usage.function")
  /// Engine code `usage.record-kind`.
  public static let usageRecordKind = Self(rawValue: "usage.record-kind")
  /// Engine code `usage.offsets`.
  public static let usageOffsets = Self(rawValue: "usage.offsets")
  /// Engine code `usage.argument`.
  public static let usageArgument = Self(rawValue: "usage.argument")
  /// Engine code `usage.option`.
  public static let usageOption = Self(rawValue: "usage.option")
  /// Engine code `usage.column`.
  public static let usageColumn = Self(rawValue: "usage.column")
  /// Engine code `usage.sheet`.
  public static let usageSheet = Self(rawValue: "usage.sheet")
  /// Engine code `usage.output`.
  public static let usageOutput = Self(rawValue: "usage.output")
  /// Engine code `usage.table`.
  public static let usageTable = Self(rawValue: "usage.table")
  /// Engine code `record.json`.
  public static let recordJSON = Self(rawValue: "record.json")
  /// Engine code `record.missing`.
  public static let recordMissing = Self(rawValue: "record.missing")
  /// Engine code `record.too-large`.
  public static let recordTooLarge = Self(rawValue: "record.too-large")
  /// Engine code `record.invalid`.
  public static let recordInvalid = Self(rawValue: "record.invalid")
  /// Engine code `record.empty`.
  public static let recordEmpty = Self(rawValue: "record.empty")
  /// Engine code `backend.transport`.
  public static let backendTransport = Self(rawValue: "backend.transport")
  /// Engine code `backend.status`.
  public static let backendStatus = Self(rawValue: "backend.status")
  /// Engine code `backend.reply`.
  public static let backendReply = Self(rawValue: "backend.reply")
  /// Engine code `local.cancelled`.
  public static let localCancelled = Self(rawValue: "local.cancelled")
  /// Engine code `local.io`.
  public static let localIO = Self(rawValue: "local.io")
  /// Engine code `local.toolbox`.
  public static let localToolbox = Self(rawValue: "local.toolbox")
  /// Engine code `local.accounting`.
  public static let localAccounting = Self(rawValue: "local.accounting")
  /// Engine code `local.request-limit`.
  public static let localRequestLimit = Self(rawValue: "local.request-limit")
  /// Engine code `internal.bug`.
  public static let internalBug = Self(rawValue: "internal.bug")
  /// The codes known by this package; newer names remain representable.
  public static let allCases: [Self] = [.usageThreshold, .usageQuestion, .usageName, .usageSettings, .usageBackend, .usageURL, .usageKeyMissing, .usageBatch, .usagePointer, .usageSQLPolicy, .usageSQLSelection, .usageABI, .usageClosed, .usageFunction, .usageRecordKind, .usageOffsets, .usageArgument, .usageOption, .usageColumn, .usageSheet, .usageOutput, .usageTable, .recordJSON, .recordMissing, .recordTooLarge, .recordInvalid, .recordEmpty, .backendTransport, .backendStatus, .backendReply, .localCancelled, .localIO, .localToolbox, .localAccounting, .localRequestLimit, .internalBug]
  /// The integer engine identity, when this package knows the code.
  public var nativeValue: Int32? {
    switch rawValue {
    case "usage.threshold": 1001
    case "usage.question": 1002
    case "usage.name": 1003
    case "usage.settings": 1004
    case "usage.backend": 1005
    case "usage.url": 1006
    case "usage.key-missing": 1007
    case "usage.batch": 1008
    case "usage.pointer": 1009
    case "usage.sql-policy": 1010
    case "usage.sql-selection": 1011
    case "usage.abi": 1012
    case "usage.closed": 1013
    case "usage.function": 1014
    case "usage.record-kind": 1015
    case "usage.offsets": 1016
    case "usage.argument": 1017
    case "usage.option": 1018
    case "usage.column": 1019
    case "usage.sheet": 1020
    case "usage.output": 1021
    case "usage.table": 1022
    case "record.json": 2001
    case "record.missing": 2002
    case "record.too-large": 2003
    case "record.invalid": 2004
    case "record.empty": 2005
    case "backend.transport": 3001
    case "backend.status": 3002
    case "backend.reply": 3003
    case "local.cancelled": 4001
    case "local.io": 4002
    case "local.toolbox": 4003
    case "local.accounting": 4004
    case "local.request-limit": 4005
    case "internal.bug": 5001
    default: nil
    }
  }
  public var family: String? {
    switch rawValue {
    case "usage.threshold": "usage"
    case "usage.question": "usage"
    case "usage.name": "usage"
    case "usage.settings": "usage"
    case "usage.backend": "usage"
    case "usage.url": "usage"
    case "usage.key-missing": "usage"
    case "usage.batch": "usage"
    case "usage.pointer": "usage"
    case "usage.sql-policy": "usage"
    case "usage.sql-selection": "usage"
    case "usage.abi": "usage"
    case "usage.closed": "usage"
    case "usage.function": "usage"
    case "usage.record-kind": "usage"
    case "usage.offsets": "usage"
    case "usage.argument": "usage"
    case "usage.option": "usage"
    case "usage.column": "usage"
    case "usage.sheet": "usage"
    case "usage.output": "usage"
    case "usage.table": "usage"
    case "record.json": "record"
    case "record.missing": "record"
    case "record.too-large": "record"
    case "record.invalid": "record"
    case "record.empty": "record"
    case "backend.transport": "backend"
    case "backend.status": "backend"
    case "backend.reply": "backend"
    case "local.cancelled": "local"
    case "local.io": "local"
    case "local.toolbox": "local"
    case "local.accounting": "local"
    case "local.request-limit": "local"
    case "internal.bug": "internal"
    default: nil
    }
  }
  /// Decode a stable diagnostic name.
  public init(from decoder: any Decoder) throws { self.init(rawValue: try decoder.singleValueContainer().decode(String.self)) }
  /// Encode the stable diagnostic name.
  public func encode(to encoder: any Encoder) throws { var c = encoder.singleValueContainer(); try c.encode(rawValue) }
}
