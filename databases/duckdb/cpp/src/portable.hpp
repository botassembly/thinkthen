#pragma once

#include "duckdb/main/extension/extension_loader.hpp"

namespace duckdb {
void RegisterPortableDecide(ExtensionLoader &loader);
void RegisterPortableMacro(ExtensionLoader &loader, const string &sql, const string &purpose);
string PortableOptionParameters();
string PortableJSONArgument(const string &argument);
bool PortableSettingsType(const string &type);
string PortableOptions(const string &settings = "settings", const string &nulls = "question IS NULL OR input IS NULL", bool legacy_type = true, bool recoverable = false);
void RegisterPlan(ExtensionLoader &loader);
} // namespace duckdb
