#pragma once

#include "duckdb/common/types/value.hpp"

namespace duckdb {
// Bridge serializers own JSON validity; retain their bytes without a second parser.
inline Value ThinkThenJSON(const string &text) {
    Value value(text);
    value.GetTypeMutable() = LogicalType::JSON();
    return value;
}
} // namespace duckdb
