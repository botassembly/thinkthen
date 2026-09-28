#pragma once

#include "duckdb.hpp"

#include <cstdint>

namespace duckdb {

LogicalType NestedType(int32_t kind);
vector<Value> DecodeNested(const uint8_t *bytes, size_t length, idx_t count, int32_t kind);

} // namespace duckdb
