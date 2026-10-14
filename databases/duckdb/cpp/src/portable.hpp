#pragma once

#include "duckdb/main/extension/extension_loader.hpp"

namespace duckdb {
void RegisterPortableDecide(ExtensionLoader &loader);
void RegisterRecordOverloads(ExtensionLoader &loader, const string &sql, const string &purpose, bool list_rules = false);
void RegisterPortableMacro(ExtensionLoader &loader, const string &sql, const string &purpose);
string PortableOptionParameters();
string PortableJSONArgument(const string &argument);
bool PortableSettingsType(const string &type);
string PortableOptions(const string &settings = "settings", const string &nulls = "question IS NULL OR input IS NULL", bool legacy_type = true, bool recoverable = false);
string NativeRecordTableSQL(const string &verb);
void RegisterPlan(ExtensionLoader &loader);
} // namespace duckdb
