#pragma once
#include "duckdb.hpp"
#include "bridge.hpp"
extern "C" {
struct ThinkThenImage { ThinkThenText media; ThinkThenText data; };
}
namespace duckdb {
LogicalType ImageType();
vector<std::pair<string, string>> ImageMembers(const Value &images);
vector<ThinkThenImage> ImageViews(const vector<std::pair<string, string>> &images);
void RegisterImages(ExtensionLoader &loader);
}
