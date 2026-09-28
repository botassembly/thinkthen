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
	std::optional<string> model;
	int64_t timeout;
	int64_t max_retries;
	std::optional<string> profile;
	std::optional<string> record;
	std::optional<string> replay;
	int32_t record_allowed = 1;
	int32_t replay_allowed = 1;

	ThinkThenSettings Bridge() const {
		const auto bytes = [](const std::optional<string> &value) {
			return value ? reinterpret_cast<const uint8_t *>(value->data()) : nullptr;
		};
		const auto length = [](const std::optional<string> &value) { return value ? value->size() : 0; };
		return {throttle, max_requests, max_requests_total,
		        cache ? reinterpret_cast<const uint8_t *>(cache->data()) : nullptr,
		        cache ? cache->size() : 0, cache_allowed,
		        bytes(model), length(model), timeout, max_retries, bytes(profile), length(profile),
		        bytes(record), length(record), bytes(replay), length(replay), record_allowed, replay_allowed};
	}
};

SessionSettings Settings(ClientContext &context);

} // namespace duckdb
