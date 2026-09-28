#pragma once

#include "duckdb.hpp"
#include "duckdb/main/connection.hpp"

#include <atomic>
#include <chrono>
#include <memory>
#include <mutex>
#include <optional>

namespace duckdb {

struct RelateDatabase {
	shared_ptr<DatabaseInstance> database;
	Connection connection;
	std::mutex gate;
	std::atomic<bool> busy {false};

	explicit RelateDatabase(shared_ptr<DatabaseInstance> database)
	    : database(std::move(database)), connection(*this->database) {
	}
};

struct RelateLease {
	RelateDatabase &held;
	std::unique_lock<std::mutex> gate;
	RelateLease(RelateDatabase &held, std::unique_lock<std::mutex> gate)
	    : held(held), gate(std::move(gate)) {
		held.busy.store(true, std::memory_order_release);
	}
	~RelateLease() { held.busy.store(false, std::memory_order_release); }
};

struct RelateFound {
	idx_t columns = 0;
	vector<vector<std::optional<string>>> rows;
	int64_t remaining_ms = -1;
	std::unique_ptr<RelateLease> lease;
};

std::shared_ptr<RelateDatabase> HoldRelateDatabase(ClientContext &context);
RelateFound ReadRelateRows(RelateDatabase &held, ClientContext &caller, const string &sql,
                          uint64_t seconds, uint64_t holding, const std::optional<string> &search_path);
void InterruptBusyRelate() noexcept;
bool RelateBusyFor(ClientContext &context) noexcept;

} // namespace duckdb
