#include "scalar_owner.hpp"
#include "bridge.hpp"
#include "duckdb/common/exception.hpp"
#include "duckdb/common/file_system.hpp"
#include <filesystem>

namespace duckdb {

bool QueryInterrupted(ClientContext &context) noexcept {
	try {
		auto owner = context.registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
		return owner->Stopped(context);
	} catch (...) {
		return true;
	}
}

ResolvedQuestion ResolveQuestion(ClientContext &context, const string &argument, const char *role) {
	if (argument.empty() || argument[0] != '@') { return {argument, false}; }
	return {ReadQuestion(context, argument.substr(1), role), true};
}

string ReadQuestion(ClientContext &context, const string &path, const char *role, bool regular) {
	auto &files = FileSystem::GetFileSystem(context);
	if (regular) {
		std::error_code error;
		if (!std::filesystem::is_regular_file(std::filesystem::status(files.ExpandPath(path), error))) {
			throw OrdinaryError("thinkthen local: the question file must be regular");
		}
	}
	string content;
	bool too_large = false;
	try {
		auto handle = files.OpenFile(path, FileOpenFlags::FILE_FLAGS_READ);
		if (regular && files.GetFileType(*handle) != FileType::FILE_TYPE_REGULAR) {
			throw OrdinaryError("thinkthen local: the question file must be regular");
		}
		vector<char> buffer(64 * 1024);
		for (;;) {
			const auto read = files.Read(*handle, buffer.data(), static_cast<int64_t>(buffer.size()));
			if (read <= 0) {
				break;
			}
			content.append(buffer.data(), static_cast<size_t>(read));
			if (content.size() > 1024 * 1024) {
				too_large = true;
				break;
			}
		}
	} catch (const PermissionException &) {
		throw OrdinaryError("thinkthen local: the %s file %s was not read: this database's file settings refuse it", role, path.c_str());
	} catch (const IOException &) {
		throw OrdinaryError("thinkthen local: the %s file %s was not read: it does not exist or could not be opened", role, path.c_str());
	} catch (const Exception &) {
		throw OrdinaryError("thinkthen local: the %s file %s was not read: this database's file settings refuse it", role, path.c_str());
	}
	if (too_large) {
		throw OrdinaryError("thinkthen local: the %s file %s was not read: it holds more than 1 MiB", role, path.c_str());
	}
	if (!Value::StringIsValid(content)) {
		throw OrdinaryError("thinkthen local: the %s file %s was not read: it is not UTF-8 text", role, path.c_str());
	}
	return content;
}

} // namespace duckdb
