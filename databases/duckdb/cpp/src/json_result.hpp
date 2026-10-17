#pragma once

#include "duckdb_api.hpp"

namespace duckdb {
// Bridge serializers own JSON validity; retain their bytes without a second parser.
inline Value ThinkThenJSON(const string &text) {
    return ThinkThenJSONValue(text);
}
} // namespace duckdb
