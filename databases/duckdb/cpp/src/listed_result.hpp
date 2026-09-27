#pragma once

#include "duckdb.hpp"

#include <cstdint>
#include <optional>
#include <vector>

namespace duckdb {

std::optional<vector<string>> Members(const Value &value);
vector<Value> DecodeListed(const uint8_t *bytes, size_t length, idx_t count, int32_t kind);

} // namespace duckdb
