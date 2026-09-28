#include "scalar_settings.hpp"
#include "duckdb/common/file_system.hpp"

#include <limits>

namespace duckdb {

namespace {
int64_t NumericSetting(ClientContext &context, const char *name) {
	Value value;
	return context.TryGetCurrentSetting(name, value) && !value.IsNull() ? value.GetValue<int64_t>()
	                                                                      : std::numeric_limits<int64_t>::min();
}

} // namespace

SessionSettings Settings(ClientContext &context) {
	SessionSettings settings {NumericSetting(context, "thinkthen_throttle"),
	                          NumericSetting(context, "thinkthen_max_requests"),
	                          NumericSetting(context, "thinkthen_max_requests_total")};
	Value value;
	if (context.TryGetCurrentSetting("thinkthen_cache", value) && !value.IsNull()) {
		settings.cache = value.GetValue<string>();
		const auto &folder = *settings.cache;
		if (!folder.empty() && folder[0] == '/' && folder.find("://") == string::npos) {
			try {
				FileSystem::GetFileSystem(context).OpenFile(folder + "/.probe", FileOpenFlags::FILE_FLAGS_READ);
			} catch (const IOException &) {
				// A missing probe file still means the caller could open the folder.
			} catch (const Exception &) {
				settings.cache_allowed = 0;
			}
		}
	}
	return settings;
}

} // namespace duckdb
