// Generated native requirement. Do not edit.
#nullable enable
namespace ThinkThen;
internal static class Contract
{
    internal const uint AbiVersion = 131076;
    internal const string AbiCode = "usage.abi";
    internal const string AbiMessage = "the native ABI is incompatible";
    internal const string LibraryVariable = "THINKTHEN_LIBRARY";
    internal const string LibraryPathMessage = "THINKTHEN_LIBRARY must be an absolute path";
    internal static string? FunctionName(long value) => value switch
    {
        1 => "decide",
        2 => "choose",
        3 => "tag",
        4 => "score",
        5 => "filter",
        7 => "find",
        8 => "recognize",
        9 => "relate",
        6 => "rank",
        10 => "annotate",
        _ => null
    };
    internal static string? StatusName(long value) => value switch
    {
        1 => "answered",
        2 => "unsure",
        3 => "failed",
        _ => null
    };
    internal static string? BackendName(long value) => value switch
    {
        1 => "systemone",
        2 => "openai",
        3 => "stub",
        _ => null
    };
    internal static string? IdName(long value) => value switch
    {
        0 => "absent",
        1 => "string",
        2 => "signed",
        3 => "unsigned",
        _ => null
    };
    internal static string? RecordName(long value) => value switch
    {
        0 => "text",
        1 => "json",
        2 => "missing",
        3 => "invalid",
        _ => null
    };
    internal static string? OffsetName(long value) => value switch
    {
        0 => "utf8",
        1 => "utf16",
        2 => "code_points",
        3 => "code_points_inclusive",
        _ => null
    };
    internal static string? CodeName(long value) => value switch
    {
        1001 => "usage.threshold",
        1002 => "usage.question",
        1003 => "usage.name",
        1004 => "usage.settings",
        1005 => "usage.backend",
        1006 => "usage.url",
        1007 => "usage.key-missing",
        1008 => "usage.batch",
        1009 => "usage.pointer",
        1010 => "usage.sql-policy",
        1011 => "usage.sql-selection",
        1012 => "usage.abi",
        1013 => "usage.closed",
        1014 => "usage.function",
        1015 => "usage.record-kind",
        1016 => "usage.offsets",
        1017 => "usage.argument",
        1018 => "usage.option",
        1019 => "usage.column",
        1020 => "usage.sheet",
        1021 => "usage.output",
        1022 => "usage.table",
        2001 => "record.json",
        2002 => "record.missing",
        2003 => "record.too-large",
        2004 => "record.invalid",
        2005 => "record.empty",
        3001 => "backend.transport",
        3002 => "backend.status",
        3003 => "backend.reply",
        4001 => "local.cancelled",
        4002 => "local.io",
        4003 => "local.toolbox",
        4004 => "local.accounting",
        4005 => "local.request-limit",
        5001 => "internal.bug",
        _ => null
    };
    internal static readonly string[] Exports = ["thinkthen_call_events","thinkthen_events_version","thinkthen_events_wait","thinkthen_events_wake","thinkthen_events_free","thinkthen_settings_conversion_error","thinkthen_question_conversion_error","thinkthen_client_open_json","thinkthen_client_free","thinkthen_client_usage","thinkthen_client_usage_settled","thinkthen_call_open_question","thinkthen_call_send_utf16","thinkthen_call_send_json","thinkthen_call_end_input","thinkthen_call_next","thinkthen_call_cancel","thinkthen_call_free","thinkthen_error_free","thinkthen_code_family","thinkthen_code_name","thinkthen_call_send_missing","thinkthen_row_free","thinkthen_row_original","thinkthen_question_from_json","thinkthen_question_free","thinkthen_toolbox_from_json","thinkthen_toolbox_from_definitions_json","thinkthen_toolbox_to_json","thinkthen_toolbox_definitions_json","thinkthen_buffer_free","thinkthen_toolbox_free","thinkthen_call_send_invalid","thinkthen_toolbox_select_as","thinkthen_settings_from_json","thinkthen_settings_free"];
}
/// <summary>Stable error codes from the shared engine registry.</summary>
[System.Text.Json.Serialization.JsonConverter(typeof(ErrorCodeConverter))]
public enum ErrorCode
{
    /// <summary>The shared `usage.threshold` error.</summary>
    UsageThreshold = 1001,
    /// <summary>The shared `usage.question` error.</summary>
    UsageQuestion = 1002,
    /// <summary>The shared `usage.name` error.</summary>
    UsageName = 1003,
    /// <summary>The shared `usage.settings` error.</summary>
    UsageSettings = 1004,
    /// <summary>The shared `usage.backend` error.</summary>
    UsageBackend = 1005,
    /// <summary>The shared `usage.url` error.</summary>
    UsageUrl = 1006,
    /// <summary>The shared `usage.key-missing` error.</summary>
    UsageKeyMissing = 1007,
    /// <summary>The shared `usage.batch` error.</summary>
    UsageBatch = 1008,
    /// <summary>The shared `usage.pointer` error.</summary>
    UsagePointer = 1009,
    /// <summary>The shared `usage.sql-policy` error.</summary>
    UsageSqlPolicy = 1010,
    /// <summary>The shared `usage.sql-selection` error.</summary>
    UsageSqlSelection = 1011,
    /// <summary>The shared `usage.abi` error.</summary>
    UsageAbi = 1012,
    /// <summary>The shared `usage.closed` error.</summary>
    UsageClosed = 1013,
    /// <summary>The shared `usage.function` error.</summary>
    UsageFunction = 1014,
    /// <summary>The shared `usage.record-kind` error.</summary>
    UsageRecordKind = 1015,
    /// <summary>The shared `usage.offsets` error.</summary>
    UsageOffsets = 1016,
    /// <summary>The shared `usage.argument` error.</summary>
    UsageArgument = 1017,
    /// <summary>The shared `usage.option` error.</summary>
    UsageOption = 1018,
    /// <summary>The shared `usage.column` error.</summary>
    UsageColumn = 1019,
    /// <summary>The shared `usage.sheet` error.</summary>
    UsageSheet = 1020,
    /// <summary>The shared `usage.output` error.</summary>
    UsageOutput = 1021,
    /// <summary>The shared `usage.table` error.</summary>
    UsageTable = 1022,
    /// <summary>The shared `record.json` error.</summary>
    RecordJson = 2001,
    /// <summary>The shared `record.missing` error.</summary>
    RecordMissing = 2002,
    /// <summary>The shared `record.too-large` error.</summary>
    RecordTooLarge = 2003,
    /// <summary>The shared `record.invalid` error.</summary>
    RecordInvalid = 2004,
    /// <summary>The shared `record.empty` error.</summary>
    RecordEmpty = 2005,
    /// <summary>The shared `backend.transport` error.</summary>
    BackendTransport = 3001,
    /// <summary>The shared `backend.status` error.</summary>
    BackendStatus = 3002,
    /// <summary>The shared `backend.reply` error.</summary>
    BackendReply = 3003,
    /// <summary>The shared `local.cancelled` error.</summary>
    LocalCancelled = 4001,
    /// <summary>The shared `local.io` error.</summary>
    LocalIo = 4002,
    /// <summary>The shared `local.toolbox` error.</summary>
    LocalToolbox = 4003,
    /// <summary>The shared `local.accounting` error.</summary>
    LocalAccounting = 4004,
    /// <summary>The shared `local.request-limit` error.</summary>
    LocalRequestLimit = 4005,
    /// <summary>The shared `internal.bug` error.</summary>
    InternalBug = 5001,
}
