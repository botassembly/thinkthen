#include "files_manifest.hpp"
#include "bridge.hpp"
#include "duckdb/common/opener_file_system.hpp"
#include "duckdb/common/virtual_file_system.hpp"

#include <algorithm>
#include <filesystem>

namespace duckdb {
namespace {
constexpr size_t BYTE_CAP = 16 * 1024 * 1024;
namespace host = std::filesystem;

void Add(vector<string> &manifest, string path, size_t &bytes) {
	if (path.size() >= BYTE_CAP - bytes) {
		throw OrdinaryError("thinkthen local: source manifest exceeds 16 MiB");
	}
	bytes += path.size() + 1;
	manifest.push_back(std::move(path));
}

void Descend(FileSystem &files, const string &path, vector<string> &manifest, size_t &bytes) {
	AuthorizeLocalSource(files, path);
	// DuckDB's pinned ListFiles silently skips special files, failed stats
	// and unreadable descendants. Its recursive glob skips symlinks but keeps
	// those omissions. Strict no-follow enumeration therefore uses LOCAL
	// metadata only, after caller authorization for every path/directory.
	// No file content is opened here. Rust never receives a path to open.
	for (const auto &entry : host::directory_iterator(files.ExpandPath(path))) {
		auto child = files.JoinPath(path, entry.path().filename().string());
		AuthorizeLocalSource(files, child);
		auto kind = host::symlink_status(files.ExpandPath(child));
		if (host::is_symlink(kind)) { continue; }
		if (host::is_directory(kind)) {
			Descend(files, child, manifest, bytes);
		} else if (host::is_regular_file(kind)) {
			Add(manifest, child, bytes);
		} else {
			throw OrdinaryError("thinkthen local: source folder contains an unsupported file kind");
		}
	}
}
} // namespace

void AuthorizeLocalSource(FileSystem &files, const string &path) {
	if (!Value::StringIsValid(path)) {
		throw OrdinaryError("thinkthen local: source filename must be UTF-8");
	}
	// A host metadata interpretation is valid only for the stock local route.
	// Refuse custom registries rather than guessing whether they own a path.
	auto opener = dynamic_cast<OpenerFileSystem *>(&files);
	auto registry = opener ? dynamic_cast<VirtualFileSystem *>(&opener->GetFileSystem()) : nullptr;
	if (!registry || registry->GetDefaultFileSystem().GetName() != "LocalFileSystem" ||
	    !registry->ListSubSystems().empty() || path.find("://") != string::npos) {
		throw OrdinaryError("thinkthen local: source reader requires the stock local filesystem");
	}
	if (files.IsDisabledForPath(path)) {
		throw PermissionException("thinkthen local: source filesystem is disabled");
	}
	// Existence predicates enter the executing OpenerFileSystem permission
	// checks and disabled-filesystem dispatcher before any LOCAL metadata.
	// A false predicate still authorizes metadata of a missing/special path;
	// a permission exception never does. Directory authorization can differ
	// from exact allowed_paths authorization, so try the latter on refusal.
	try {
		if (files.DirectoryExists(path)) { return; }
	} catch (const PermissionException &) {
		files.FileExists(path);
		return;
	}
	files.FileExists(path);
}

vector<string> FileManifest(FileSystem &files, const vector<string> &paths) {
	vector<string> manifest;
	size_t bytes = 0;
	try {
		for (auto &path : paths) {
			AuthorizeLocalSource(files, path);
			auto kind = host::status(files.ExpandPath(path));
			if (host::is_directory(kind)) {
				auto start = manifest.size();
				Descend(files, path, manifest, bytes);
				std::sort(manifest.begin() + start, manifest.end());
			} else if (host::is_regular_file(kind)) {
				Add(manifest, path, bytes);
			} else {
				throw OrdinaryError("thinkthen local: source operand must be a regular file or folder");
			}
		}
	} catch (const host::filesystem_error &) {
		throw OrdinaryError("thinkthen local: source folder could not be enumerated or inspected");
	}
	return manifest;
}
} // namespace duckdb
