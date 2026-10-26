"""Generated native requirement. Do not edit."""
ABI_VERSION = 131073
ABI_VERSION_EXPORT = "ABI_VERSION"
ABI_CODE = "usage.abi"
ABI_MESSAGE = "the native ABI is incompatible"
ABI_EXPORTS = ["ABI_VERSION","ERROR_CODES","Error","NativeCancel","NativeCancel.cancel","NativeCancel.child","NativeEngine","NativeEngine.settings","NativeEngine.start","NativeEngine.start_finite","NativeQuestion","NativeQuestion.definition","NativeQuestion.function","NativeStream","NativeStream.cancel","NativeStream.close","NativeStream.receive","NativeToolbox","NativeToolbox.definition","NativeToolbox.from_json","NativeToolbox.question","NativeToolbox.to_json","reject_cancelled","reject_closed_client","reject_question_conversion","reject_settings_conversion","seconds_to_millis"]
FUNCTION_NAMES: dict[int, str] = {
    1: "decide",
    2: "choose",
    3: "tag",
    4: "score",
    5: "filter",
    7: "find",
    8: "recognize",
    9: "relate",
    6: "rank",
    10: "annotate",
}
STATUS_NAMES: dict[int, str] = {
    1: "answered",
    2: "unsure",
    3: "failed",
}
BACKEND_NAMES: dict[int, str] = {
    1: "systemone",
    2: "openai",
    3: "stub",
}
ID_NAMES: dict[int, str] = {
    0: "absent",
    1: "string",
    2: "signed",
    3: "unsigned",
}
RECORD_NAMES: dict[int, str] = {
    0: "text",
    1: "json",
    2: "missing",
    3: "invalid",
}
OFFSET_NAMES: dict[int, str] = {
    0: "utf8",
    1: "utf16",
    2: "code_points",
    3: "code_points_inclusive",
}
CODE_NAMES: dict[int, str] = {
    1001: "usage.threshold",
    1002: "usage.question",
    1003: "usage.name",
    1004: "usage.settings",
    1005: "usage.backend",
    1006: "usage.url",
    1007: "usage.key-missing",
    1008: "usage.batch",
    1009: "usage.pointer",
    1010: "usage.sql-policy",
    1011: "usage.sql-selection",
    1012: "usage.abi",
    2001: "record.json",
    2002: "record.missing",
    2003: "record.too-large",
    2004: "record.invalid",
    3001: "backend.transport",
    3002: "backend.status",
    3003: "backend.reply",
    4001: "local.cancelled",
    4002: "local.io",
    4003: "local.toolbox",
    5001: "internal.bug",
}
