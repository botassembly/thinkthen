// Generated from the engine code registry. Do not edit.
#ifndef THINKTHEN_CPP_CODES_HPP
#define THINKTHEN_CPP_CODES_HPP
#include <thinkthen.h>
#include <cstdint>
namespace thinkthen { inline namespace v0_2 {
/// Stable error codes from the engine registry.
enum class errc : std::int32_t {
    /** usage.threshold: A cut or band is out of range or not allowed for this function. */
    usage_threshold = THINKTHEN_CODE_USAGE_THRESHOLD,
    /** usage.question: The question is not valid: blank or missing a part it needs. */
    usage_question = THINKTHEN_CODE_USAGE_QUESTION,
    /** usage.name: A name is blank, repeated, or not the name of an existing question. */
    usage_name = THINKTHEN_CODE_USAGE_NAME,
    /** usage.settings: The settings are not valid. */
    usage_settings = THINKTHEN_CODE_USAGE_SETTINGS,
    /** usage.backend: The backend is unknown, or not allowed here. */
    usage_backend = THINKTHEN_CODE_USAGE_BACKEND,
    /** usage.url: The URL is not valid. */
    usage_url = THINKTHEN_CODE_USAGE_URL,
    /** usage.key-missing: The backend needs an API key and none was given or found. */
    usage_key_missing = THINKTHEN_CODE_USAGE_KEY_MISSING,
    /** usage.batch: The batch size is not a positive count. */
    usage_batch = THINKTHEN_CODE_USAGE_BATCH,
    /** usage.pointer: A pointer, such as a field pointer, is not a valid JSON Pointer. */
    usage_pointer = THINKTHEN_CODE_USAGE_POINTER,
    /** usage.sql-policy: The database administrator's SQL policy is not valid. */
    usage_sql_policy = THINKTHEN_CODE_USAGE_SQL_POLICY,
    /** usage.sql-selection: The database administrator does not allow this backend and model. */
    usage_sql_selection = THINKTHEN_CODE_USAGE_SQL_SELECTION,
    /** usage.abi: A struct, handle or slot is not one this library made or initialized, or the library's binary interface is incompatible. */
    usage_abi = THINKTHEN_CODE_USAGE_ABI,
    /** usage.closed: The client or call was used after it was closed. */
    usage_closed = THINKTHEN_CODE_USAGE_CLOSED,
    /** usage.function: This function does not fit the question's or value's function. */
    usage_function = THINKTHEN_CODE_USAGE_FUNCTION,
    /** usage.record-kind: The record kind is unknown. */
    usage_record_kind = THINKTHEN_CODE_USAGE_RECORD_KIND,
    /** usage.offsets: The offset unit is unknown. */
    usage_offsets = THINKTHEN_CODE_USAGE_OFFSETS,
    /** usage.argument: An argument is NULL, out of range, or not allowed here. */
    usage_argument = THINKTHEN_CODE_USAGE_ARGUMENT,
    /** usage.option: An option is not valid for this function. */
    usage_option = THINKTHEN_CODE_USAGE_OPTION,
    /** usage.column: A table column, answer column or field pointer is not valid. */
    usage_column = THINKTHEN_CODE_USAGE_COLUMN,
    /** usage.sheet: The table sheet selection is not valid. */
    usage_sheet = THINKTHEN_CODE_USAGE_SHEET,
    /** usage.output: The table answers cannot go to the output asked for. */
    usage_output = THINKTHEN_CODE_USAGE_OUTPUT,
    /** usage.table: The table input file cannot be used as it is. */
    usage_table = THINKTHEN_CODE_USAGE_TABLE,
    /** record.json: The record is not valid JSON. A record error: that record's row fails and the call goes on. */
    record_json = THINKTHEN_CODE_RECORD_JSON,
    /** record.missing: The record is a missing value. A record error: that record's row fails and the call goes on. */
    record_missing = THINKTHEN_CODE_RECORD_MISSING,
    /** record.too-large: The record is larger than the backend accepts. A record error: that record's row fails and the call goes on. */
    record_too_large = THINKTHEN_CODE_RECORD_TOO_LARGE,
    /** record.invalid: The record could not be read, for example text that is not UTF-8. A record error: that record's row fails and the call goes on. */
    record_invalid = THINKTHEN_CODE_RECORD_INVALID,
    /** record.empty: The record's text is blank, so it is never sent. A record error: that record's row fails and the call goes on. */
    record_empty = THINKTHEN_CODE_RECORD_EMPTY,
    /** backend.transport: The backend could not be reached. A record error: that record's row fails and the call goes on. */
    backend_transport = THINKTHEN_CODE_BACKEND_TRANSPORT,
    /** backend.status: The backend refused the request. A record error: that record's row fails and the call goes on. */
    backend_status = THINKTHEN_CODE_BACKEND_STATUS,
    /** backend.reply: The backend's reply could not be read. A record error: that record's row fails and the call goes on. */
    backend_reply = THINKTHEN_CODE_BACKEND_REPLY,
    /** local.cancelled: The call was cancelled. */
    local_cancelled = THINKTHEN_CODE_LOCAL_CANCELLED,
    /** local.io: A local file could not be read or written. */
    local_io = THINKTHEN_CODE_LOCAL_IO,
    /** local.toolbox: A toolbox could not be read. */
    local_toolbox = THINKTHEN_CODE_LOCAL_TOOLBOX,
    /** local.accounting: Client accounting exceeded its supported range before another request could be sent. */
    local_accounting = THINKTHEN_CODE_LOCAL_ACCOUNTING,
    /** local.request-limit: The client request allowance is exhausted; no further request was sent. */
    local_request_limit = THINKTHEN_CODE_LOCAL_REQUEST_LIMIT,
    /** local.response-store:  */
    local_response_store = THINKTHEN_CODE_LOCAL_RESPONSE_STORE,
    /** local.store-corrupt:  */
    local_store_corrupt = THINKTHEN_CODE_LOCAL_STORE_CORRUPT,
    /** local.store-format:  */
    local_store_format = THINKTHEN_CODE_LOCAL_STORE_FORMAT,
    /** local.store-answer:  */
    local_store_answer = THINKTHEN_CODE_LOCAL_STORE_ANSWER,
    /** local.replay-miss:  */
    local_replay_miss = THINKTHEN_CODE_LOCAL_REPLAY_MISS,
    /** local.store-quota:  */
    local_store_quota = THINKTHEN_CODE_LOCAL_STORE_QUOTA,
    /** internal.bug: A bug in this library; please report it. */
    internal_bug = THINKTHEN_CODE_INTERNAL_BUG,
};
namespace detail {
inline bool known_code(std::int32_t value) noexcept {
    switch (value) {
        case THINKTHEN_CODE_USAGE_THRESHOLD: return true;
        case THINKTHEN_CODE_USAGE_QUESTION: return true;
        case THINKTHEN_CODE_USAGE_NAME: return true;
        case THINKTHEN_CODE_USAGE_SETTINGS: return true;
        case THINKTHEN_CODE_USAGE_BACKEND: return true;
        case THINKTHEN_CODE_USAGE_URL: return true;
        case THINKTHEN_CODE_USAGE_KEY_MISSING: return true;
        case THINKTHEN_CODE_USAGE_BATCH: return true;
        case THINKTHEN_CODE_USAGE_POINTER: return true;
        case THINKTHEN_CODE_USAGE_SQL_POLICY: return true;
        case THINKTHEN_CODE_USAGE_SQL_SELECTION: return true;
        case THINKTHEN_CODE_USAGE_ABI: return true;
        case THINKTHEN_CODE_USAGE_CLOSED: return true;
        case THINKTHEN_CODE_USAGE_FUNCTION: return true;
        case THINKTHEN_CODE_USAGE_RECORD_KIND: return true;
        case THINKTHEN_CODE_USAGE_OFFSETS: return true;
        case THINKTHEN_CODE_USAGE_ARGUMENT: return true;
        case THINKTHEN_CODE_USAGE_OPTION: return true;
        case THINKTHEN_CODE_USAGE_COLUMN: return true;
        case THINKTHEN_CODE_USAGE_SHEET: return true;
        case THINKTHEN_CODE_USAGE_OUTPUT: return true;
        case THINKTHEN_CODE_USAGE_TABLE: return true;
        case THINKTHEN_CODE_RECORD_JSON: return true;
        case THINKTHEN_CODE_RECORD_MISSING: return true;
        case THINKTHEN_CODE_RECORD_TOO_LARGE: return true;
        case THINKTHEN_CODE_RECORD_INVALID: return true;
        case THINKTHEN_CODE_RECORD_EMPTY: return true;
        case THINKTHEN_CODE_BACKEND_TRANSPORT: return true;
        case THINKTHEN_CODE_BACKEND_STATUS: return true;
        case THINKTHEN_CODE_BACKEND_REPLY: return true;
        case THINKTHEN_CODE_LOCAL_CANCELLED: return true;
        case THINKTHEN_CODE_LOCAL_IO: return true;
        case THINKTHEN_CODE_LOCAL_TOOLBOX: return true;
        case THINKTHEN_CODE_LOCAL_ACCOUNTING: return true;
        case THINKTHEN_CODE_LOCAL_REQUEST_LIMIT: return true;
        case THINKTHEN_CODE_LOCAL_RESPONSE_STORE: return true;
        case THINKTHEN_CODE_LOCAL_STORE_CORRUPT: return true;
        case THINKTHEN_CODE_LOCAL_STORE_FORMAT: return true;
        case THINKTHEN_CODE_LOCAL_STORE_ANSWER: return true;
        case THINKTHEN_CODE_LOCAL_REPLAY_MISS: return true;
        case THINKTHEN_CODE_LOCAL_STORE_QUOTA: return true;
        case THINKTHEN_CODE_INTERNAL_BUG: return true;
        default: return false;
    }
}
}
}}
#endif
