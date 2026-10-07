#pragma once

#include "bridge.hpp"
#include "duckdb/main/client_context.hpp"

#include <optional>
#include <string>

namespace duckdb {

struct SessionSettings {
	std::optional<string> batch;
	int64_t throttle;
	int64_t max_requests;
	int64_t max_request_bytes;
	int64_t max_requests_total;
	std::optional<string> backend;
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
    std::optional<string> base_url;
    std::optional<string> input_price;
    std::optional<string> output_price;
    int32_t refresh_cache = 0;

	ThinkThenSettings Bridge() const {
		const auto bytes = [](const std::optional<string> &value) {
			return value ? reinterpret_cast<const uint8_t *>(value->data()) : nullptr;
		};
		const auto length = [](const std::optional<string> &value) { return value ? value->size() : 0; };
		return {bytes(batch), length(batch), throttle, max_requests, max_request_bytes, max_requests_total,
		        bytes(backend), length(backend),
		        cache ? reinterpret_cast<const uint8_t *>(cache->data()) : nullptr,
		        cache ? cache->size() : 0, cache_allowed,
		        bytes(model), length(model), timeout, max_retries, bytes(profile), length(profile),
		        bytes(record), length(record), bytes(replay), length(replay), record_allowed, replay_allowed,bytes(base_url),length(base_url),bytes(input_price),length(input_price),bytes(output_price),length(output_price),refresh_cache};
	}
};

SessionSettings Settings(ClientContext &context);

} // namespace duckdb
