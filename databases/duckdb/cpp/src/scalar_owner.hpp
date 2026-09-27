#pragma once

#include "duckdb.hpp"
#include "duckdb/common/exception.hpp"
#include "duckdb/main/client_context.hpp"
#include "duckdb/main/client_context_state.hpp"

#include <chrono>
#include <map>
#include <mutex>
#include <optional>
#include <string>

namespace duckdb {

inline constexpr const char *OWNER_KEY = "thinkthen_statement_owner";

struct ResolvedQuestion {
	string text;
	bool from_file = false;
	bool operator==(const ResolvedQuestion &other) const {
		return text == other.text && from_file == other.from_file;
	}
};

ResolvedQuestion ResolveQuestion(ClientContext &context, const string &argument);

struct StatementOwner : ClientContextState {
	std::mutex lock;
	uint64_t generation = 0;
	bool active = false;
	bool first_use = false;
	std::optional<std::chrono::steady_clock::time_point> expiry;
	std::map<string, ResolvedQuestion> files;

	void QueryBegin(ClientContext &context) override {
		std::lock_guard<std::mutex> held(lock);
		Start(context, false);
	}
	void QueryEnd(ClientContext &, optional_ptr<ErrorData>) override {
		std::lock_guard<std::mutex> held(lock);
		active = false;
		files.clear();
	}
	void Start(ClientContext &context, bool late) {
		Value setting;
		const auto budget = context.TryGetCurrentSetting("thinkthen_query_budget_ms", setting) && !setting.IsNull()
		                        ? setting.GetValue<int64_t>()
		                        : -1;
		if (budget < -1 || budget > 4294967295000LL) {
			throw InvalidInputException("thinkthen usage: the query budget is outside the supported range");
		}
		active = true;
		first_use = late;
		generation++;
		files.clear();
		expiry = budget < 0 ? std::nullopt
		                    : std::optional<std::chrono::steady_clock::time_point>(
		                          std::chrono::steady_clock::now() + std::chrono::milliseconds(budget));
	}
	int64_t Remaining(ClientContext &context) {
		std::lock_guard<std::mutex> held(lock);
		if (!active) {
			Start(context, true);
		}
		if (!expiry) {
			return -1;
		}
		const auto now = std::chrono::steady_clock::now();
		if (now >= *expiry) {
			throw InvalidInputException("thinkthen deadline: the query has spent its time budget");
		}
		return std::chrono::duration_cast<std::chrono::milliseconds>(*expiry - now).count();
	}
	ResolvedQuestion Resolve(ClientContext &context, const string &argument) {
		{
			std::lock_guard<std::mutex> held(lock);
			if (!active) {
				Start(context, true);
			}
			if (auto found = files.find(argument); found != files.end()) {
				return found->second;
			}
		}
		auto resolved = ResolveQuestion(context, argument);
		if (resolved.from_file) {
			std::lock_guard<std::mutex> held(lock);
			return files.emplace(argument, std::move(resolved)).first->second;
		}
		return resolved;
	}
};

} // namespace duckdb
