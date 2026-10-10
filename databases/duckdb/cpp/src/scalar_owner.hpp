#pragma once

#include "duckdb.hpp"
#include "duckdb/common/exception.hpp"
#include "duckdb/main/client_context.hpp"
#include "duckdb/main/client_context_state.hpp"
#include "bridge.hpp"

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

string ReadQuestion(ClientContext &context, const string &path, const char *role = "question", bool regular = false);
ResolvedQuestion ResolveQuestion(ClientContext &context, const string &argument, const char *role = "question");

struct StatementOwner : ClientContextState {
	std::mutex lock;
	uint64_t generation = 0;
	bool active = false;
	bool first_use = false;
	bool budget_ready = false;
	std::chrono::steady_clock::time_point started;
	std::optional<std::chrono::steady_clock::time_point> expiry;
	std::map<string, ResolvedQuestion> files;
	void *signal_scope = nullptr;

	~StatementOwner() override {
		thinkthen_cpp_query_end(signal_scope);
	}

	void QueryBegin(ClientContext &) override {
		std::lock_guard<std::mutex> held(lock);
		Start(false);
	}
	void QueryEnd(ClientContext &, optional_ptr<ErrorData>) override {
		std::lock_guard<std::mutex> held(lock);
		active = false;
		files.clear();
		thinkthen_cpp_query_end(signal_scope);
		signal_scope = nullptr;
	}
	void Start(bool late) {
		started = std::chrono::steady_clock::now();
		budget_ready = false;
		expiry.reset();
		thinkthen_cpp_query_end(signal_scope);
		signal_scope = thinkthen_cpp_query_begin();
		if (!signal_scope) {
			throw OrdinaryError("thinkthen defect: the signal scope could not start");
		}
		active = true;
		first_use = late;
		generation++;
		files.clear();
	}
	void Budget(ClientContext &context) {
		if (budget_ready) { return; }
		Value setting;
		const auto budget = context.TryGetCurrentSetting("thinkthen_query_budget_ms", setting) && !setting.IsNull()
		                        ? setting.GetValue<int64_t>()
		                        : -1;
		if (budget < -1 || budget > 4294967295000LL) {
			throw OrdinaryError("thinkthen usage: the query budget is outside the supported range");
		}
		expiry = budget < 0 ? std::nullopt
		                    : std::optional<std::chrono::steady_clock::time_point>(
		                          started + std::chrono::milliseconds(budget));
		budget_ready = true;
	}
	void AdmitBudget(ClientContext &context) {
		std::lock_guard<std::mutex> held(lock);
		if (!active) { Start(true); }
		Budget(context);
	}
	bool Stopped(ClientContext &context) {
		std::lock_guard<std::mutex> held(lock);
		if (!signal_scope) {
			signal_scope = thinkthen_cpp_query_begin();
		}
		return context.IsInterrupted() || !signal_scope || thinkthen_cpp_query_stopped(signal_scope) != 0;
	}
	int64_t Remaining(ClientContext &context) {
		std::lock_guard<std::mutex> held(lock);
		if (!active) {
			Start(true);
		}
		Budget(context);
		if (!expiry) {
			return -1;
		}
		const auto now = std::chrono::steady_clock::now();
		if (now >= *expiry) {
			throw OrdinaryError("thinkthen deadline: the query has spent its time budget");
		}
		return std::chrono::duration_cast<std::chrono::milliseconds>(*expiry - now).count();
	}
	ResolvedQuestion Resolve(ClientContext &context, const string &argument) {
		{
			std::lock_guard<std::mutex> held(lock);
			if (!active) {
				Start(true);
			}
			Budget(context);
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
