#include "scalar_settings.hpp"
#include "bridge.hpp"
#include "duckdb/common/file_system.hpp"

#include <limits>

namespace duckdb {

namespace {
int64_t NumericSetting(ClientContext &context, const char *name) {
	Value value;
	return context.TryGetCurrentSetting(name, value) && !value.IsNull() ? value.GetValue<int64_t>()
	                                                                      : std::numeric_limits<int64_t>::min();
}

std::optional<string> TextSetting(ClientContext &context, const char *name) {
	Value value;
	if (context.TryGetCurrentSetting(name, value) && !value.IsNull()) {
		return value.GetValue<string>();
	}
	return std::nullopt;
}

bool FolderAllowed(ClientContext &context, const std::optional<string> &folder) {
	if (!folder || *folder == "off" || folder->empty() || folder->front() != '/' || folder->find("://") != string::npos) {
		return true;
	}
	try {
		FileSystem::GetFileSystem(context).OpenFile(*folder + "/.probe", FileOpenFlags::FILE_FLAGS_READ);
	} catch (const IOException &) {
		// A missing probe file still means the caller could open the folder.
	} catch (const Exception &) {
		return false;
	}
	return true;
}

} // namespace

SessionSettings Settings(ClientContext &context) {
	SessionSettings settings {TextSetting(context, "thinkthen_batch"),
	                          NumericSetting(context, "thinkthen_throttle"),
	                          NumericSetting(context, "thinkthen_max_requests"),
	                          NumericSetting(context, "thinkthen_max_request_bytes"),
	                          NumericSetting(context, "thinkthen_max_requests_total"),
	                          TextSetting(context, "thinkthen_cache"), 1,
	                          TextSetting(context, "thinkthen_model"),
	                          NumericSetting(context, "thinkthen_timeout"),
	                          NumericSetting(context, "thinkthen_max_retries"),
	                          TextSetting(context, "thinkthen_profile"),
	                          TextSetting(context, "thinkthen_record"),
	                          TextSetting(context, "thinkthen_replay")};
	if (settings.max_requests_total != std::numeric_limits<int64_t>::min() &&
	    settings.max_requests_total < 0) {
		throw OrdinaryError("thinkthen usage: a request total is a whole number of 0 or more");
	}
	settings.cache_allowed = FolderAllowed(context, settings.cache) ? 1 : 0;
	settings.record_allowed = FolderAllowed(context, settings.record) ? 1 : 0;
	settings.replay_allowed = FolderAllowed(context, settings.replay) ? 1 : 0;
	return settings;
}

} // namespace duckdb
