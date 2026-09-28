#pragma once

#include "duckdb/main/extension/extension_loader.hpp"

namespace duckdb {

void RegisterWarm(ExtensionLoader &loader);

} // namespace duckdb
