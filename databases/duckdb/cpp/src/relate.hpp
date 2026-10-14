#pragma once

#include "duckdb/main/extension/extension_loader.hpp"
#include "bridge.hpp"

namespace duckdb {
void RegisterRelate(ExtensionLoader &loader);
string RelateDetailsCall(ClientContext &context, const string &query, const Value &rules,
    const string &controls, int64_t deadline, ThinkThenSettings settings);
} // namespace duckdb
