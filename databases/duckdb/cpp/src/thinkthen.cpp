#define DUCKDB_EXTENSION_MAIN

#include "duckdb.hpp"
#include "duckdb/common/exception.hpp"
#include "duckdb/common/weak_ptr_ipp.hpp"
#include "duckdb/execution/expression_executor.hpp"
#include "duckdb/main/client_context.hpp"
#include "duckdb/main/client_context_state.hpp"
#include "duckdb/main/extension/extension_loader.hpp"
#include "duckdb/planner/expression/bound_function_expression.hpp"

#include <algorithm>
#include <cstdint>
#include <chrono>
#include <cstring>
#include <memory>
#include <map>
#include <limits>
#include <mutex>
#include <optional>
#include <set>
#include <string>
#include <vector>

static_assert(sizeof(double) == 8, "the Rust bridge returns eight-byte probabilities");

extern "C" {
struct ThinkThenReply {
	int32_t status;
	uint8_t *bytes;
	size_t len;
};
struct ThinkThenText {
	const uint8_t *bytes;
	size_t len;
};
struct ThinkThenSettings {
	int64_t throttle;
	int64_t max_requests;
	int64_t max_requests_total;
};
int32_t thinkthen_cpp_init();
ThinkThenReply thinkthen_cpp_validate_question(const uint8_t *bytes, size_t len);
ThinkThenReply thinkthen_cpp_decision_group(const uint8_t *question, size_t question_len, const ThinkThenText *texts,
                                            size_t count, int64_t deadline_ms, int32_t probability,
                                            ThinkThenSettings settings);
void thinkthen_cpp_free(uint8_t *bytes, size_t len);
}

namespace duckdb {
namespace {

constexpr const char *OWNER_KEY = "thinkthen_statement_owner";

struct RustReply {
	explicit RustReply(ThinkThenReply value) : value(value) {
	}
	~RustReply() {
		thinkthen_cpp_free(value.bytes, value.len);
	}
	RustReply(const RustReply &) = delete;
	RustReply &operator=(const RustReply &) = delete;
	ThinkThenReply value;
};

string ReplyText(const ThinkThenReply &reply) {
	if (!reply.bytes) {
		return "thinkthen defect: the bridge returned no error buffer";
	}
	return string(reinterpret_cast<const char *>(reply.bytes), reply.len);
}

void Checked(const ThinkThenReply &reply) {
	if (reply.status != 0) {
		throw InvalidInputException("%s", ReplyText(reply).c_str());
	}
}

struct StatementOwner : ClientContextState {
	std::mutex lock;
	uint64_t generation = 0;
	bool active = false;
	bool first_use = false;
	std::optional<std::chrono::steady_clock::time_point> expiry;

	void QueryBegin(ClientContext &context) override {
		std::lock_guard<std::mutex> held(lock);
		Start(context, false);
	}
	void QueryEnd(ClientContext &, optional_ptr<ErrorData>) override {
		std::lock_guard<std::mutex> held(lock);
		active = false;
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
};

struct ScalarBind : FunctionData {
	weak_ptr<ClientContext> context;
	std::optional<string> constant_question;
	std::optional<int64_t> constant_deadline;
	bool probability = false;

	explicit ScalarBind(weak_ptr<ClientContext> context_p) : context(std::move(context_p)) {
	}
	unique_ptr<FunctionData> Copy() const override {
		auto copy = make_uniq<ScalarBind>(context);
		copy->constant_question = constant_question;
		copy->constant_deadline = constant_deadline;
		copy->probability = probability;
		return copy;
	}
	bool Equals(const FunctionData &other) const override {
		auto &held = other.Cast<ScalarBind>();
		return context.lock() == held.context.lock() && constant_question == held.constant_question &&
		       constant_deadline == held.constant_deadline && probability == held.probability;
	}
};

unique_ptr<FunctionData> BindDecide(ClientContext &context, ScalarFunction &function,
                                     vector<unique_ptr<Expression>> &arguments) {
	context.registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
	auto bound = make_uniq<ScalarBind>(context.shared_from_this());
	bound->probability = function.name == "thinkthen_probability";
	if (arguments[0]->IsFoldable()) {
		auto value = ExpressionExecutor::EvaluateScalar(context, *arguments[0]);
		if (!value.IsNull()) {
			bound->constant_question = value.GetValue<string>();
			auto &text = *bound->constant_question;
			RustReply checked(thinkthen_cpp_validate_question(reinterpret_cast<const uint8_t *>(text.data()), text.size()));
			Checked(checked.value);
		}
	}
	if (arguments.size() == 3 && arguments[2]->IsFoldable()) {
		auto value = ExpressionExecutor::EvaluateScalar(context, *arguments[2]);
		if (!value.IsNull()) {
			bound->constant_deadline = value.GetValue<int64_t>();
			if (*bound->constant_deadline < -1 || *bound->constant_deadline > 4294967295000LL) {
				throw InvalidInputException("thinkthen usage: the deadline is outside the supported range");
			}
		}
	}
	return bound;
}

ScalarBind &Bound(ExpressionState &state) {
	return state.expr.Cast<BoundFunctionExpression>().bind_info->Cast<ScalarBind>();
}

struct DecisionGroup {
	string question;
	int64_t deadline;
	vector<string> texts;
	std::map<string, idx_t> seen;
};

int64_t NumericSetting(ClientContext &context, const char *name) {
	Value value;
	return context.TryGetCurrentSetting(name, value) && !value.IsNull() ? value.GetValue<int64_t>()
	                                                                      : std::numeric_limits<int64_t>::min();
}

ThinkThenSettings Settings(ClientContext &context) {
	return {NumericSetting(context, "thinkthen_throttle"), NumericSetting(context, "thinkthen_max_requests"),
	        NumericSetting(context, "thinkthen_max_requests_total")};
}

void Decide(DataChunk &args, ExpressionState &state, Vector &result) {
	auto &bound = Bound(state);
	auto context = bound.context.lock();
	if (!context) {
		throw InvalidInputException("thinkthen defect: the caller session ended");
	}
	auto owner = context->registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
	vector<DecisionGroup> groups;
	std::map<std::pair<string, int64_t>, idx_t> known_groups;
	std::set<string> checked_questions;
	vector<std::optional<std::pair<idx_t, idx_t>>> slots(args.size());
	// Validate the entire chunk before its first real engine call.
	for (idx_t row = 0; row < args.size(); ++row) {
		auto question = args.data[0].GetValue(row);
		auto evidence = args.data[1].GetValue(row);
		if (question.IsNull() || evidence.IsNull()) {
			continue;
		}
		if (args.ColumnCount() == 3 && args.data[2].GetValue(row).IsNull()) {
			continue;
		}
		auto question_text = question.GetValue<string>();
		if (checked_questions.insert(question_text).second) {
			RustReply checked(thinkthen_cpp_validate_question(reinterpret_cast<const uint8_t *>(question_text.data()),
			                                              question_text.size()));
			Checked(checked.value);
		}
		auto due = args.ColumnCount() == 3 ? args.data[2].GetValue(row).GetValue<int64_t>() : -1;
		if (due < -1 || due > 4294967295000LL) {
			throw InvalidInputException("thinkthen usage: the deadline is outside the supported range");
		}
		auto key = std::make_pair(question_text, due);
		auto [place, new_group] = known_groups.emplace(key, groups.size());
		if (new_group) {
			groups.push_back({question_text, due, {}, {}});
		}
		auto &group = groups[place->second];
		auto evidence_text = evidence.GetValue<string>();
		auto [position, new_text] = group.seen.emplace(evidence_text, group.texts.size());
		if (new_text) {
			group.texts.push_back(evidence_text);
		}
		slots[row] = std::make_pair(place->second, position->second);
	}
	vector<vector<uint8_t>> outcomes;
	const auto settings = Settings(*context);
	for (auto &group : groups) {
		const auto budget = owner->Remaining(*context);
		const auto due = budget < 0 ? group.deadline
		                           : group.deadline < 0 ? budget : std::min(group.deadline, budget);
		vector<ThinkThenText> texts;
		texts.reserve(group.texts.size());
		for (auto &text : group.texts) {
			texts.push_back({reinterpret_cast<const uint8_t *>(text.data()), text.size()});
		}
		RustReply answered(thinkthen_cpp_decision_group(reinterpret_cast<const uint8_t *>(group.question.data()),
		                                               group.question.size(), texts.data(), texts.size(), due,
		                                               bound.probability ? 1 : 0, settings));
		Checked(answered.value);
		const auto width = bound.probability ? sizeof(double) : sizeof(uint8_t);
		if (answered.value.len != texts.size() * width || !answered.value.bytes) {
			throw InvalidInputException("thinkthen defect: the bridge returned an invalid decision group");
		}
		outcomes.emplace_back(answered.value.bytes, answered.value.bytes + answered.value.len);
	}
	for (idx_t row = 0; row < args.size(); ++row) {
		if (!slots[row]) {
			result.SetValue(row, Value(bound.probability ? LogicalType::DOUBLE : LogicalType::BOOLEAN));
			continue;
		}
		auto [group, text] = *slots[row];
		const auto width = bound.probability ? sizeof(double) : sizeof(uint8_t);
		if (group >= outcomes.size() || (text + 1) * width > outcomes[group].size()) {
			throw InvalidInputException("thinkthen defect: a decision row lost its answer");
		}
		if (bound.probability) {
			double value;
			std::memcpy(&value, outcomes[group].data() + text * width, sizeof(value));
			result.SetValue(row, Value::DOUBLE(value));
			continue;
		}
		switch (outcomes[group][text]) {
		case 0: result.SetValue(row, Value::BOOLEAN(false)); break;
		case 1: result.SetValue(row, Value::BOOLEAN(true)); break;
		case 2: result.SetValue(row, Value(LogicalType::BOOLEAN)); break;
		default: throw InvalidInputException("thinkthen defect: the bridge returned an invalid decision");
		}
	}
}

} // namespace

void LoadThinkThen(ExtensionLoader &loader) {
	if (thinkthen_cpp_init() != 0) {
		throw InvalidInputException("thinkthen defect: the Rust bridge did not initialize");
	}
	auto &config = DBConfig::GetConfig(loader.GetDatabaseInstance());
	config.AddExtensionOption("thinkthen_query_budget_ms", "Whole-statement ThinkThen time budget in milliseconds",
	                          LogicalType::BIGINT, Value::BIGINT(-1));
	config.AddExtensionOption("thinkthen_throttle", "Maximum concurrent ThinkThen requests", LogicalType::BIGINT);
	config.AddExtensionOption("thinkthen_max_requests", "Maximum ThinkThen requests in one call", LogicalType::BIGINT);
	config.AddExtensionOption("thinkthen_max_requests_total", "Maximum ThinkThen requests in this process",
	                          LogicalType::BIGINT);
	for (auto name : {"thinkthen_decide", "thinkthen_probability"}) {
		const auto result = string(name) == "thinkthen_probability" ? LogicalType::DOUBLE : LogicalType::BOOLEAN;
		for (auto parameters : {vector<LogicalType>{LogicalType::VARCHAR, LogicalType::VARCHAR},
		                        vector<LogicalType>{LogicalType::VARCHAR, LogicalType::VARCHAR, LogicalType::BIGINT}}) {
			ScalarFunction function(name, parameters, result, Decide, BindDecide);
			function.SetStability(FunctionStability::VOLATILE);
			loader.RegisterFunction(function);
		}
	}
}

} // namespace duckdb

extern "C" {
DUCKDB_CPP_EXTENSION_ENTRY(thinkthen, loader) {
	duckdb::LoadThinkThen(loader);
}
}
