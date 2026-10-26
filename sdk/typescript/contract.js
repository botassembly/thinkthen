// Generated native requirement. Do not edit.
export const ABI_VERSION = 131073;
export const ABI_VERSION_EXPORT = "abiVersion";
export const ABI_CODE = "usage.abi";
export const ABI_MESSAGE = "the native ABI is incompatible";
export const ABI_EXPORTS = ["Client","Client.prototype.close","Client.prototype.start","Client.prototype.startFinite","Operation","Operation.prototype.cancel","Operation.prototype.close","Operation.prototype.reference","Operation.prototype.unreference","abiVersion","parseToolbox","rejectCancellation","rejectNonFiniteJson","rejectQuestionConversion","rejectSettingsConversion","serializeToolbox"];
/** @type {Readonly<Record<number, string>>} */
export const FUNCTION_NAMES = Object.freeze({
    "1": "decide",
    "2": "choose",
    "3": "tag",
    "4": "score",
    "5": "filter",
    "7": "find",
    "8": "recognize",
    "9": "relate",
    "6": "rank",
    "10": "annotate",
});
/** @type {Readonly<Record<number, string>>} */
export const STATUS_NAMES = Object.freeze({
    "1": "answered",
    "2": "unsure",
    "3": "failed",
});
/** @type {Readonly<Record<number, string>>} */
export const BACKEND_NAMES = Object.freeze({
    "1": "systemone",
    "2": "openai",
    "3": "stub",
});
/** @type {Readonly<Record<number, string>>} */
export const ID_NAMES = Object.freeze({
    "0": "absent",
    "1": "string",
    "2": "signed",
    "3": "unsigned",
});
/** @type {Readonly<Record<number, string>>} */
export const RECORD_NAMES = Object.freeze({
    "0": "text",
    "1": "json",
    "2": "missing",
    "3": "invalid",
});
/** @type {Readonly<Record<number, string>>} */
export const OFFSET_NAMES = Object.freeze({
    "0": "utf8",
    "1": "utf16",
    "2": "code_points",
    "3": "code_points_inclusive",
});
/** @type {Readonly<Record<number, string>>} */
export const CODE_NAMES = Object.freeze({
    "1001": "usage.threshold",
    "1002": "usage.question",
    "1003": "usage.name",
    "1004": "usage.settings",
    "1005": "usage.backend",
    "1006": "usage.url",
    "1007": "usage.key-missing",
    "1008": "usage.batch",
    "1009": "usage.pointer",
    "1010": "usage.sql-policy",
    "1011": "usage.sql-selection",
    "1012": "usage.abi",
    "2001": "record.json",
    "2002": "record.missing",
    "2003": "record.too-large",
    "2004": "record.invalid",
    "3001": "backend.transport",
    "3002": "backend.status",
    "3003": "backend.reply",
    "4001": "local.cancelled",
    "4002": "local.io",
    "4003": "local.toolbox",
    "5001": "internal.bug",
});
