// Generated from the engine code registry. Do not edit.
#ifndef THINKTHEN_CPP_CODES_HPP
#define THINKTHEN_CPP_CODES_HPP
#include <thinkthen.h>
#include <cstdint>
namespace thinkthen { inline namespace v0_2 {
enum class errc : std::int32_t {
    usage_threshold = THINKTHEN_CODE_USAGE_THRESHOLD,
    usage_question = THINKTHEN_CODE_USAGE_QUESTION,
    usage_name = THINKTHEN_CODE_USAGE_NAME,
    usage_settings = THINKTHEN_CODE_USAGE_SETTINGS,
    usage_backend = THINKTHEN_CODE_USAGE_BACKEND,
    usage_url = THINKTHEN_CODE_USAGE_URL,
    usage_key_missing = THINKTHEN_CODE_USAGE_KEY_MISSING,
    usage_batch = THINKTHEN_CODE_USAGE_BATCH,
    usage_pointer = THINKTHEN_CODE_USAGE_POINTER,
    usage_sql_policy = THINKTHEN_CODE_USAGE_SQL_POLICY,
    usage_sql_selection = THINKTHEN_CODE_USAGE_SQL_SELECTION,
    usage_abi = THINKTHEN_CODE_USAGE_ABI,
    record_json = THINKTHEN_CODE_RECORD_JSON,
    record_missing = THINKTHEN_CODE_RECORD_MISSING,
    record_too_large = THINKTHEN_CODE_RECORD_TOO_LARGE,
    record_invalid = THINKTHEN_CODE_RECORD_INVALID,
    backend_transport = THINKTHEN_CODE_BACKEND_TRANSPORT,
    backend_status = THINKTHEN_CODE_BACKEND_STATUS,
    backend_reply = THINKTHEN_CODE_BACKEND_REPLY,
    local_cancelled = THINKTHEN_CODE_LOCAL_CANCELLED,
    local_io = THINKTHEN_CODE_LOCAL_IO,
    local_toolbox = THINKTHEN_CODE_LOCAL_TOOLBOX,
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
        case THINKTHEN_CODE_RECORD_JSON: return true;
        case THINKTHEN_CODE_RECORD_MISSING: return true;
        case THINKTHEN_CODE_RECORD_TOO_LARGE: return true;
        case THINKTHEN_CODE_RECORD_INVALID: return true;
        case THINKTHEN_CODE_BACKEND_TRANSPORT: return true;
        case THINKTHEN_CODE_BACKEND_STATUS: return true;
        case THINKTHEN_CODE_BACKEND_REPLY: return true;
        case THINKTHEN_CODE_LOCAL_CANCELLED: return true;
        case THINKTHEN_CODE_LOCAL_IO: return true;
        case THINKTHEN_CODE_LOCAL_TOOLBOX: return true;
        case THINKTHEN_CODE_INTERNAL_BUG: return true;
        default: return false;
    }
}
}
}}
#endif
