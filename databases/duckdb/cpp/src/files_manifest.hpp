#pragma once
#include "duckdb/common/file_system.hpp"
namespace duckdb {
vector<string> FileManifest(FileSystem &files, const vector<string> &paths);
void AuthorizeLocalSource(FileSystem &files, const string &path);
} // namespace duckdb
