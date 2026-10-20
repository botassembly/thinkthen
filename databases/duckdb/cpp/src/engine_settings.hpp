#pragma once
#include "duckdb.hpp"
namespace duckdb {
struct EngineSetting {
    const char *name;
    const char *description;
    LogicalTypeId type;
};
inline const EngineSetting ENGINE_SETTINGS[] = {
    {"batch", "Maximum records in one ThinkThen request, or max", LogicalTypeId::VARCHAR},
    {"throttle", "Maximum concurrent ThinkThen requests", LogicalTypeId::BIGINT},
    {"max_requests", "Maximum ThinkThen requests in one call", LogicalTypeId::BIGINT},
    {"max_request_bytes", "Positive ThinkThen request-byte ceiling", LogicalTypeId::BIGINT},
    {"max_requests_total", "Maximum ThinkThen requests in this process", LogicalTypeId::BIGINT},
    {"cache", "Local ThinkThen cache folder", LogicalTypeId::VARCHAR},
    {"backend", "ThinkThen named backend", LogicalTypeId::VARCHAR},
    {"base_url", "Explicit ThinkThen backend address", LogicalTypeId::VARCHAR},
    {"usd_per_million_input", "Caller input price per million tokens", LogicalTypeId::VARCHAR},
    {"usd_per_million_output", "Caller output price per million tokens", LogicalTypeId::VARCHAR},
    {"refresh_cache", "Refresh cached native answers: 0 or 1", LogicalTypeId::BIGINT},
    {"model", "ThinkThen model", LogicalTypeId::VARCHAR},
    {"timeout", "Live attempt timeout in seconds", LogicalTypeId::BIGINT},
    {"max_retries", "Maximum live retries", LogicalTypeId::BIGINT},
    {"profile", "Inline backend limits profile JSON", LogicalTypeId::VARCHAR},
    {"record", "Local recording folder", LogicalTypeId::VARCHAR},
    {"replay", "Local strict replay folder", LogicalTypeId::VARCHAR},
};
} // namespace duckdb
