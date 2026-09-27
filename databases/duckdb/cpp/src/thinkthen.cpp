#define DUCKDB_EXTENSION_MAIN

#include "duckdb.hpp"
#include "scalar_owner.hpp"
#include "duckdb/common/exception.hpp"
#include "duckdb/common/file_system.hpp"
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
	const uint8_t *cache_bytes;
	size_t cache_len;
	int32_t cache_allowed;
};
int32_t thinkthen_cpp_init();
ThinkThenReply thinkthen_cpp_validate_question(const uint8_t *bytes, size_t len, int32_t from_file);
ThinkThenReply thinkthen_cpp_scalar_group(const uint8_t *question, size_t question_len, const ThinkThenText *texts,
                                          size_t count, int64_t deadline_ms, int32_t kind,
                                          ThinkThenSettings settings, int32_t from_file);
ThinkThenReply thinkthen_cpp_try_details_row(const uint8_t *question, size_t question_len,
                                             const uint8_t *evidence, size_t evidence_len,
                                             int64_t deadline_ms, ThinkThenSettings settings, int32_t from_file);
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

void ValidateQuestion(const ResolvedQuestion &resolved) {
	RustReply checked(thinkthen_cpp_validate_question(reinterpret_cast<const uint8_t *>(resolved.text.data()),
	                                               resolved.text.size(), resolved.from_file ? 1 : 0));
	Checked(checked.value);
}

struct ScalarBind : FunctionData {
	weak_ptr<ClientContext> context;
	std::optional<string> constant_question;
	std::optional<ResolvedQuestion> resolved_question;
	std::optional<int64_t> constant_deadline;
	int32_t kind = 0;

	explicit ScalarBind(weak_ptr<ClientContext> context_p) : context(std::move(context_p)) {
	}
	unique_ptr<FunctionData> Copy() const override {
		auto copy = make_uniq<ScalarBind>(context);
		copy->constant_question = constant_question;
		copy->resolved_question = resolved_question;
		copy->constant_deadline = constant_deadline;
		copy->kind = kind;
		return copy;
	}
	bool Equals(const FunctionData &other) const override {
		auto &held = other.Cast<ScalarBind>();
		return context.lock() == held.context.lock() && constant_question == held.constant_question &&
		       resolved_question == held.resolved_question &&
		       constant_deadline == held.constant_deadline && kind == held.kind;
	}
};

unique_ptr<FunctionData> BindDecide(ClientContext &context, ScalarFunction &function,
                                     vector<unique_ptr<Expression>> &arguments) {
	context.registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
	auto bound = make_uniq<ScalarBind>(context.shared_from_this());
	bound->kind = function.name == "thinkthen_probability" ? 1
	            : function.name == "thinkthen_details" ? 2
	            : function.name == "thinkthen_try_details" ? 3 : 0;
	if (arguments[0]->IsFoldable()) {
		auto value = ExpressionExecutor::EvaluateScalar(context, *arguments[0]);
		if (!value.IsNull()) {
			bound->constant_question = value.GetValue<string>();
			auto &text = *bound->constant_question;
			if (bound->kind != 3) {
				bound->resolved_question = ResolveQuestion(context, text);
				ValidateQuestion(*bound->resolved_question);
			}
		}
	}
	if (arguments.size() == 3 && arguments[2]->IsFoldable()) {
		auto value = ExpressionExecutor::EvaluateScalar(context, *arguments[2]);
		if (!value.IsNull()) {
			bound->constant_deadline = value.GetValue<int64_t>();
			if (bound->kind != 3 && (*bound->constant_deadline < -1 || *bound->constant_deadline > 4294967295000LL)) {
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
	bool from_file;
	vector<string> texts;
	std::map<string, idx_t> seen;
};

int64_t NumericSetting(ClientContext &context, const char *name) {
	Value value;
	return context.TryGetCurrentSetting(name, value) && !value.IsNull() ? value.GetValue<int64_t>()
	                                                                      : std::numeric_limits<int64_t>::min();
}

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

SessionSettings Settings(ClientContext &context) {
	SessionSettings settings {NumericSetting(context, "thinkthen_throttle"),
	                          NumericSetting(context, "thinkthen_max_requests"),
	                          NumericSetting(context, "thinkthen_max_requests_total")};
	Value value;
	if (context.TryGetCurrentSetting("thinkthen_cache", value) && !value.IsNull()) {
		settings.cache = value.GetValue<string>();
		const auto &folder = *settings.cache;
		if (!folder.empty() && folder[0] == '/' && folder.find("://") == string::npos) {
			try {
				FileSystem::GetFileSystem(context).OpenFile(folder + "/.probe", FileOpenFlags::FILE_FLAGS_READ);
			} catch (const IOException &) {
				// A missing probe file still means the caller could open the folder.
			} catch (const Exception &) {
				settings.cache_allowed = 0;
			}
		}
	}
	return settings;
}

void Decide(DataChunk &args, ExpressionState &state, Vector &result) {
	auto &bound = Bound(state);
	auto context = bound.context.lock();
	if (!context) {
		throw InvalidInputException("thinkthen defect: the caller session ended");
	}
	auto owner = context->registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
	if (bound.kind == 3) {
		const auto settings = Settings(*context);
		for (idx_t row = 0; row < args.size(); ++row) {
			auto question = args.data[0].GetValue(row);
			auto evidence = args.data[1].GetValue(row);
			if (question.IsNull() || evidence.IsNull() ||
			    (args.ColumnCount() == 3 && args.data[2].GetValue(row).IsNull())) {
				result.SetValue(row, Value(LogicalType::VARCHAR));
				continue;
			}
			const auto budget = owner->Remaining(*context);
			const auto trailing = args.ColumnCount() == 3 ? args.data[2].GetValue(row).GetValue<int64_t>() : -1;
			const auto due = trailing < -1 || budget < 0 ? trailing
			                 : trailing < 0 ? budget : std::min(trailing, budget);
			auto question_text = question.GetValue<string>();
			auto evidence_text = evidence.GetValue<string>();
			ResolvedQuestion resolved;
			try {
				resolved = owner->Resolve(*context, question_text);
			} catch (const InvalidInputException &) {
				resolved = {question_text, false}; // Rust turns an unreadable @file into a safe local value.
			}
			RustReply answered(thinkthen_cpp_try_details_row(reinterpret_cast<const uint8_t *>(resolved.text.data()),
			                                                 resolved.text.size(),
			                                                 reinterpret_cast<const uint8_t *>(evidence_text.data()),
			                                                 evidence_text.size(), due, settings.Bridge(),
			                                                 resolved.from_file ? 1 : 0));
			Checked(answered.value);
			if (!answered.value.bytes) {
				throw InvalidInputException("thinkthen defect: the bridge returned no try-details value");
			}
			result.SetValue(row, Value(ReplyText(answered.value)));
		}
		return;
	}
	vector<DecisionGroup> groups;
	std::map<std::pair<string, int64_t>, idx_t> known_groups;
	std::set<string> checked_questions;
	std::map<string, ResolvedQuestion> resolved_questions;
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
			auto resolved = bound.constant_question && *bound.constant_question == question_text && bound.resolved_question
				                    ? *bound.resolved_question : owner->Resolve(*context, question_text);
			ValidateQuestion(resolved);
			resolved_questions.emplace(question_text, std::move(resolved));
		}
		auto due = args.ColumnCount() == 3 ? args.data[2].GetValue(row).GetValue<int64_t>() : -1;
		if (due < -1 || due > 4294967295000LL) {
			throw InvalidInputException("thinkthen usage: the deadline is outside the supported range");
		}
		auto key = std::make_pair(question_text, due);
		auto [place, new_group] = known_groups.emplace(key, groups.size());
		if (new_group) {
			auto &resolved = resolved_questions.at(question_text);
			groups.push_back({resolved.text, due, resolved.from_file, {}, {}});
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
	vector<vector<string>> details;
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
		RustReply answered(thinkthen_cpp_scalar_group(reinterpret_cast<const uint8_t *>(group.question.data()),
		                                             group.question.size(), texts.data(), texts.size(), due, bound.kind,
		                                             settings.Bridge(), group.from_file ? 1 : 0));
		Checked(answered.value);
		if (!answered.value.bytes) {
			throw InvalidInputException("thinkthen defect: the bridge returned an invalid decision group");
		}
		if (bound.kind == 2) {
			vector<string> values;
			size_t at = 0;
			for (idx_t index = 0; index < texts.size(); ++index) {
				if (answered.value.len - at < sizeof(uint32_t)) {
					throw InvalidInputException("thinkthen defect: the bridge returned truncated details");
				}
				uint32_t len;
				std::memcpy(&len, answered.value.bytes + at, sizeof(len));
				at += sizeof(len);
				if (len > answered.value.len - at) {
					throw InvalidInputException("thinkthen defect: the bridge returned truncated details");
				}
				values.emplace_back(reinterpret_cast<const char *>(answered.value.bytes + at), len);
				at += len;
			}
			if (at != answered.value.len) {
				throw InvalidInputException("thinkthen defect: the bridge returned extra details bytes");
			}
			details.push_back(std::move(values));
		} else {
			const auto width = bound.kind == 1 ? sizeof(double) : sizeof(uint8_t);
			if (answered.value.len != texts.size() * width) {
				throw InvalidInputException("thinkthen defect: the bridge returned an invalid decision group");
			}
			outcomes.emplace_back(answered.value.bytes, answered.value.bytes + answered.value.len);
		}
	}
	for (idx_t row = 0; row < args.size(); ++row) {
		if (!slots[row]) {
			result.SetValue(row, Value(bound.kind == 2 ? LogicalType::VARCHAR
			                                      : bound.kind == 1 ? LogicalType::DOUBLE : LogicalType::BOOLEAN));
			continue;
		}
		auto [group, text] = *slots[row];
		if (bound.kind == 2) {
			if (group >= details.size() || text >= details[group].size()) {
				throw InvalidInputException("thinkthen defect: a details row lost its answer");
			}
			result.SetValue(row, Value(details[group][text]));
			continue;
		}
		const auto width = bound.kind == 1 ? sizeof(double) : sizeof(uint8_t);
		if (group >= outcomes.size() || (text + 1) * width > outcomes[group].size()) {
			throw InvalidInputException("thinkthen defect: a decision row lost its answer");
		}
		if (bound.kind == 1) {
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
	config.AddExtensionOption("thinkthen_cache", "Local ThinkThen cache folder", LogicalType::VARCHAR);
	for (auto name : {"thinkthen_decide", "thinkthen_probability", "thinkthen_details", "thinkthen_try_details"}) {
		const auto result = string(name) == "thinkthen_details" || string(name) == "thinkthen_try_details" ? LogicalType::VARCHAR
		                    : string(name) == "thinkthen_probability" ? LogicalType::DOUBLE : LogicalType::BOOLEAN;
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
