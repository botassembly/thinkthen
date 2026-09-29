#pragma once

#include "duckdb/main/extension/extension_loader.hpp"

namespace duckdb {
void RegisterPortableDecide(ExtensionLoader &loader);
void RegisterPortableMacro(ExtensionLoader &loader, const string &sql);
} // namespace duckdb
