#include "scalar_settings.hpp"
#include "engine_settings.hpp"
#include "scalar_owner.hpp"
#include "bridge.hpp"
#include "duckdb/common/error_data.hpp"
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
	} catch (const std::exception &error) {
		// Host DuckDB typeinfo can be private; classify missing-probe IO by its serialized type.
		return ErrorData(error).Type() == ExceptionType::IO;
	}
	return true;
}

} // namespace

void ValidateRequestTotal(ClientContext &context) {
    const auto total = NumericSetting(context, "thinkthen_max_requests_total");
    if (total != std::numeric_limits<int64_t>::min() && total < 0) {
        throw OrdinaryError("thinkthen usage: a request total is a whole number of 0 or more");
    }
}

SessionSettings Settings(ClientContext &context) {
	context.registered_state->GetOrCreate<StatementOwner>(OWNER_KEY)->AdmitBudget(context);
    SessionSettings settings;
    for (const auto &setting : ENGINE_SETTINGS) {
        Value value;
        if (!context.TryGetCurrentSetting(string("thinkthen_") + setting.name, value) || value.IsNull()) { continue; }
        const string name(setting.name);
        string text = value.ToString();
        bool literal = setting.type != LogicalTypeId::VARCHAR;
        if (name == "batch") {
            settings.batch = text;
        }
        if (name == "model") { settings.model = text; }
        if (name == "max_requests_total") { settings.total = value.GetValue<int64_t>(); }
        if (name == "cache" && text == "off") { text = "false"; literal = true; }
        if (name == "refresh_cache") {
            const auto refresh = value.GetValue<int64_t>();
            if (refresh != 0 && refresh != 1) { throw OrdinaryError("thinkthen usage: thinkthen_refresh_cache is 0 or 1"); }
            text = refresh == 1 ? "true" : "false";
        }
        settings.values.push_back({name, text, literal});
    }
    ValidateRequestTotal(context);
    settings.cache_allowed = FolderAllowed(context, TextSetting(context, "thinkthen_cache")) ? 1 : 0;
    settings.record_allowed = FolderAllowed(context, TextSetting(context, "thinkthen_record")) ? 1 : 0;
    settings.replay_allowed = FolderAllowed(context, TextSetting(context, "thinkthen_replay")) ? 1 : 0;
	return settings;
}

} // namespace duckdb
