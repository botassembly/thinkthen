#pragma once

#include "duckdb.hpp"

#include <cstdint>
#include <vector>

namespace duckdb {

vector<Value> DecodeListed(const uint8_t *bytes, size_t length, idx_t count, int32_t kind);

} // namespace duckdb
