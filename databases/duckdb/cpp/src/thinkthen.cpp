#define DUCKDB_EXTENSION_MAIN

#include "duckdb.hpp"
#include "bridge.hpp"
#include "engine_settings.hpp"
#include "find.hpp"
#include "files.hpp"
#include "images.hpp"
#include "nested.hpp"
#include "portable.hpp"
#include "relate.hpp"
#include "usage.hpp"
#include "removed.hpp"
#include "duckdb/common/exception.hpp"
#include "duckdb/main/extension/extension_loader.hpp"

namespace duckdb {

void RegisterComplete(ExtensionLoader &loader);

void LoadThinkThen(ExtensionLoader &loader) {
	if (thinkthen_cpp_init() != 0) {
		throw OrdinaryError("thinkthen defect: the Rust bridge did not initialize");
	}
	auto &config = DBConfig::GetConfig(loader.GetDatabaseInstance());
	config.AddExtensionOption("thinkthen_query_budget_ms", "Whole-statement ThinkThen time budget in milliseconds",
	                          LogicalType::BIGINT, Value::BIGINT(-1));
    for (const auto &setting : ENGINE_SETTINGS) {
        config.AddExtensionOption(string("thinkthen_") + setting.name, setting.description, LogicalType(setting.type));
    }
	RegisterPortableDecide(loader);
	RegisterPlan(loader);
	RegisterNested(loader);
	RegisterFind(loader);
	RegisterFiles(loader);
	RegisterImages(loader);
	RegisterComplete(loader);
	RegisterUsage(loader);
	RegisterRemoved(loader);
	RegisterRelate(loader);
}

} // namespace duckdb

extern "C" {
DUCKDB_CPP_EXTENSION_ENTRY(thinkthen, loader) {
	duckdb::LoadThinkThen(loader);
}
}
