#define DUCKDB_EXTENSION_MAIN

#include "duckdb.hpp"
#include "bridge.hpp"
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

void LoadThinkThen(ExtensionLoader &loader) {
	if (thinkthen_cpp_init() != 0) {
		throw OrdinaryError("thinkthen defect: the Rust bridge did not initialize");
	}
	auto &config = DBConfig::GetConfig(loader.GetDatabaseInstance());
	config.AddExtensionOption("thinkthen_query_budget_ms", "Whole-statement ThinkThen time budget in milliseconds",
	                          LogicalType::BIGINT, Value::BIGINT(-1));
	config.AddExtensionOption("thinkthen_batch", "Maximum records in one ThinkThen request, or max", LogicalType::VARCHAR);
	config.AddExtensionOption("thinkthen_throttle", "Maximum concurrent ThinkThen requests", LogicalType::BIGINT);
	config.AddExtensionOption("thinkthen_max_requests", "Maximum ThinkThen requests in one call", LogicalType::BIGINT);
	config.AddExtensionOption("thinkthen_max_request_bytes", "Positive ThinkThen request-byte ceiling", LogicalType::BIGINT);
	config.AddExtensionOption("thinkthen_max_requests_total", "Maximum ThinkThen requests in this process",
	                          LogicalType::BIGINT);
	config.AddExtensionOption("thinkthen_cache", "Local ThinkThen cache folder", LogicalType::VARCHAR);
	config.AddExtensionOption("thinkthen_backend", "ThinkThen named backend", LogicalType::VARCHAR);
	config.AddExtensionOption("thinkthen_model", "ThinkThen model", LogicalType::VARCHAR);
	config.AddExtensionOption("thinkthen_timeout", "Live attempt timeout in seconds", LogicalType::BIGINT);
	config.AddExtensionOption("thinkthen_max_retries", "Maximum live retries", LogicalType::BIGINT);
	config.AddExtensionOption("thinkthen_profile", "Inline backend limits profile JSON", LogicalType::VARCHAR);
	config.AddExtensionOption("thinkthen_record", "Local recording folder", LogicalType::VARCHAR);
	config.AddExtensionOption("thinkthen_replay", "Local strict replay folder", LogicalType::VARCHAR);
	RegisterPortableDecide(loader);
	RegisterPlan(loader);
	RegisterNested(loader);
	RegisterFind(loader);
	RegisterFiles(loader);
	RegisterImages(loader);
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
