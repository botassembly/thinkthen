// Generated native requirement. Do not edit.
#nullable enable
namespace ThinkThen;
internal static class Contract
{
    internal const uint AbiVersion = 131073;
    internal const string AbiCode = "usage.abi";
    internal const string AbiMessage = "the native ABI is incompatible";
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
        2001 => "record.json",
        2002 => "record.missing",
        2003 => "record.too-large",
        2004 => "record.invalid",
        3001 => "backend.transport",
        3002 => "backend.status",
        3003 => "backend.reply",
        4001 => "local.cancelled",
        4002 => "local.io",
        4003 => "local.toolbox",
        5001 => "internal.bug",
        _ => null
    };
    internal static readonly string[] Exports = ["thinkthen_settings_conversion_error","thinkthen_question_conversion_error","thinkthen_client_open_json","thinkthen_client_free","thinkthen_call_open_question","thinkthen_call_send_utf16","thinkthen_call_send_json","thinkthen_call_end_input","thinkthen_call_next","thinkthen_call_cancel","thinkthen_call_free","thinkthen_error_free","thinkthen_code_name","thinkthen_call_send_missing","thinkthen_row_free","thinkthen_question_from_json","thinkthen_question_free","thinkthen_toolbox_from_json","thinkthen_toolbox_from_definitions_json","thinkthen_toolbox_to_json","thinkthen_toolbox_definitions_json","thinkthen_buffer_free","thinkthen_toolbox_free","thinkthen_call_send_invalid","thinkthen_question_new","thinkthen_question_set_cut","thinkthen_question_set_band","thinkthen_question_set_evidence","thinkthen_toolbox_select_as","thinkthen_settings_from_json","thinkthen_settings_free"];
}
[System.Text.Json.Serialization.JsonConverter(typeof(ErrorCodeConverter))]
public enum ErrorCode
{
    UsageThreshold = 1001,
    UsageQuestion = 1002,
    UsageName = 1003,
    UsageSettings = 1004,
    UsageBackend = 1005,
    UsageUrl = 1006,
    UsageKeyMissing = 1007,
    UsageBatch = 1008,
    UsagePointer = 1009,
    UsageSqlPolicy = 1010,
    UsageSqlSelection = 1011,
    UsageAbi = 1012,
    RecordJson = 2001,
    RecordMissing = 2002,
    RecordTooLarge = 2003,
    RecordInvalid = 2004,
    BackendTransport = 3001,
    BackendStatus = 3002,
    BackendReply = 3003,
    LocalCancelled = 4001,
    LocalIo = 4002,
    LocalToolbox = 4003,
    InternalBug = 5001,
}
