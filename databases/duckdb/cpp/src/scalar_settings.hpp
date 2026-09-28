#pragma once

#include "bridge.hpp"
#include "duckdb/main/client_context.hpp"

#include <optional>
#include <string>

namespace duckdb {

struct SessionSettings {
	int64_t throttle;
	int64_t max_requests;
	int64_t max_requests_total;
	std::optional<string> cache;
	int32_t cache_allowed = 1;

	ThinkThenSettings Bridge() const {
		return {throttle, max_requests, max_requests_total,
		        cache ? reinterpret_cast<const uint8_t *>(cache->data()) : nullptr,
		        cache ? cache->size() : 0, cache_allowed};
	}
};

SessionSettings Settings(ClientContext &context);

} // namespace duckdb
