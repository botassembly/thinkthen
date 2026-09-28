#define DUCKDB_EXTENSION_MAIN

#include "duckdb.hpp"
#include "bridge.hpp"
#include "find.hpp"
#include "listed_result.hpp"
#include "nested.hpp"
#include "scalar_owner.hpp"
#include "scalar_settings.hpp"
#include "usage.hpp"
#include "warm.hpp"
#include "relate.hpp"
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
#include <tuple>
#include <vector>

namespace duckdb {
namespace {

void ValidateQuestion(const ResolvedQuestion &resolved, bool set = false) {
	const auto validate = set ? thinkthen_cpp_validate_set : thinkthen_cpp_validate_question;
	RustReply checked(validate(reinterpret_cast<const uint8_t *>(resolved.text.data()), resolved.text.size(),
	                           resolved.from_file ? 1 : 0));
	Checked(checked.value);
}

void ValidateListed(const string &question, const vector<string> &members, int32_t kind) {
	vector<ThinkThenText> copied;
	for (auto &member : members) {
		copied.push_back({reinterpret_cast<const uint8_t *>(member.data()), member.size()});
	}
	RustReply checked(thinkthen_cpp_validate_listed(reinterpret_cast<const uint8_t *>(question.data()), question.size(),
	                                               copied.data(), copied.size(), kind));
	Checked(checked.value);
}

struct ScalarBind : FunctionData {
	weak_ptr<ClientContext> context;
	std::optional<string> constant_question;
	std::optional<int64_t> constant_deadline;
	int32_t kind = 0;

	explicit ScalarBind(weak_ptr<ClientContext> context_p) : context(std::move(context_p)) {
	}
	unique_ptr<FunctionData> Copy() const override {
		auto copy = make_uniq<ScalarBind>(context);
		copy->constant_question = constant_question;
		copy->constant_deadline = constant_deadline;
		copy->kind = kind;
		return copy;
	}
	bool Equals(const FunctionData &other) const override {
		auto &held = other.Cast<ScalarBind>();
		return context.lock() == held.context.lock() && constant_question == held.constant_question &&
		       constant_deadline == held.constant_deadline && kind == held.kind;
	}
};

bool IsListed(int32_t kind) {
	return kind == 4 || kind == 5 || kind == 6;
}

unique_ptr<FunctionData> BindDecide(ClientContext &context, ScalarFunction &function,
                                     vector<unique_ptr<Expression>> &arguments) {
	context.registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
	auto bound = make_uniq<ScalarBind>(context.shared_from_this());
	bound->kind = function.name == "thinkthen_probability" ? 1
	            : function.name == "thinkthen_details" ? 2
	            : function.name == "thinkthen_try_details" ? 3
	            : function.name == "thinkthen_choose" ? 4
	            : function.name == "thinkthen_score" ? 5
	            : function.name == "thinkthen_tag" ? 6
	            : function.name == "thinkthen_annotate" ? 7 : 0;
	if (arguments[0]->IsFoldable()) {
		auto value = ExpressionExecutor::EvaluateScalar(context, *arguments[0]);
		if (!value.IsNull()) {
			bound->constant_question = value.GetValue<string>();
			auto &text = *bound->constant_question;
			if (bound->kind == 0 || bound->kind == 1 || bound->kind == 2 || bound->kind == 7) {
				ValidateQuestion(ResolveQuestion(context, text), bound->kind == 7);
			}
		}
	}
	if (IsListed(bound->kind) && bound->constant_question && arguments[2]->IsFoldable()) {
		auto value = ExpressionExecutor::EvaluateScalar(context, *arguments[2]);
		if (auto members = Members(value)) {
			ValidateListed(*bound->constant_question, *members, bound->kind);
		}
	}
	const auto deadline_index = IsListed(bound->kind) ? 3 : 2;
	if (arguments.size() > deadline_index && arguments[deadline_index]->IsFoldable()) {
		auto value = ExpressionExecutor::EvaluateScalar(context, *arguments[deadline_index]);
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
	std::optional<string> context;
	vector<string> texts;
	std::map<string, idx_t> seen;
};

std::optional<string> LiteralContext(DataChunk &args, idx_t row, idx_t column, bool recoverable = false) {
	if (args.ColumnCount() <= column) {
		return std::nullopt;
	}
	auto value = args.data[column].GetValue(row);
	if (value.IsNull()) {
		return std::nullopt;
	}
	auto text = value.GetValue<string>();
	if (!recoverable && text.find_first_not_of(" \t\r\n\f\v") == string::npos) {
		throw InvalidInputException("thinkthen usage: context is text, not white space");
	}
	return text;
}

void Decide(DataChunk &args, ExpressionState &state, Vector &result) {
	auto &bound = Bound(state);
	auto context = bound.context.lock();
	if (!context) {
		throw InvalidInputException("thinkthen defect: the caller session ended");
	}
	auto owner = context->registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
	vector<DecisionGroup> groups;
	std::map<std::tuple<string, int64_t, std::optional<string>>, idx_t> known_groups;
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
		if (bound.kind != 3 && args.ColumnCount() >= 3 && args.data[2].GetValue(row).IsNull()) {
			continue;
		}
		auto question_text = question.GetValue<string>();
		if (checked_questions.insert(question_text).second) {
			ResolvedQuestion resolved;
			try {
				resolved = owner->Resolve(*context, question_text);
			} catch (const InvalidInputException &) {
				if (bound.kind != 3) { throw; }
				resolved = {question_text, false};
			}
			if (bound.kind != 3) { ValidateQuestion(resolved, bound.kind == 7); }
			resolved_questions.emplace(question_text, std::move(resolved));
		}
		auto due = args.ColumnCount() >= 3 && !args.data[2].GetValue(row).IsNull()
		               ? args.data[2].GetValue(row).GetValue<int64_t>() : -1;
		if (bound.kind != 3 && (due < -1 || due > 4294967295000LL)) {
			throw InvalidInputException("thinkthen usage: the deadline is outside the supported range");
		}
		auto literal_context = LiteralContext(args, row, 3, bound.kind == 3);
		auto key = std::make_tuple(question_text, due, literal_context);
		auto [place, new_group] = known_groups.emplace(key, groups.size());
		if (new_group) {
			auto &resolved = resolved_questions.at(question_text);
			groups.push_back({resolved.text, due, resolved.from_file, literal_context, {}, {}});
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
		RustReply answered(bound.kind == 3 ? thinkthen_cpp_try_details_group(reinterpret_cast<const uint8_t *>(group.question.data()),
		                                             group.question.size(), texts.data(), texts.size(), due,
		                                             settings.Bridge(), group.from_file ? 1 : 0,
		                                             group.context ? reinterpret_cast<const uint8_t *>(group.context->data()) : nullptr,
		                                             group.context ? group.context->size() : 0, StopFor(*context))
		                              : thinkthen_cpp_scalar_group(reinterpret_cast<const uint8_t *>(group.question.data()),
		                                             group.question.size(), texts.data(), texts.size(), due, bound.kind,
		                                             settings.Bridge(), group.from_file ? 1 : 0,
		                                             group.context ? reinterpret_cast<const uint8_t *>(group.context->data()) : nullptr,
		                                             group.context ? group.context->size() : 0, StopFor(*context)));
		Checked(answered.value);
		if (!answered.value.bytes) {
			throw InvalidInputException("thinkthen defect: the bridge returned an invalid decision group");
		}
		if (bound.kind == 2 || bound.kind == 3 || bound.kind == 7) {
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
			result.SetValue(row, Value(bound.kind == 2 || bound.kind == 3 || bound.kind == 7 ? LogicalType::VARCHAR
			                                      : bound.kind == 1 ? LogicalType::DOUBLE : LogicalType::BOOLEAN));
			continue;
		}
		auto [group, text] = *slots[row];
		if (bound.kind == 2 || bound.kind == 3 || bound.kind == 7) {
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

void Listed(DataChunk &args, ExpressionState &state, Vector &result) {
	auto &bound = Bound(state);
	auto context = bound.context.lock();
	if (!context) {
		throw InvalidInputException("thinkthen defect: the caller session ended");
	}
	auto owner = context->registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
	struct Group {
		string question;
		vector<string> members;
		int64_t deadline;
		std::optional<string> context;
		vector<string> texts;
		std::map<string, idx_t> seen;
	};
	vector<Group> groups;
	std::map<std::tuple<string, vector<string>, int64_t, std::optional<string>>, idx_t> known;
	std::set<std::pair<string, vector<string>>> validated;
	vector<std::optional<std::pair<idx_t, idx_t>>> slots(args.size());
	for (idx_t row = 0; row < args.size(); ++row) {
		auto question = args.data[0].GetValue(row);
		auto evidence = args.data[1].GetValue(row);
		auto members = Members(args.data[2].GetValue(row));
		if (question.IsNull() || evidence.IsNull() || !members ||
		    (args.ColumnCount() >= 4 && args.data[3].GetValue(row).IsNull())) {
			continue;
		}
		auto question_text = question.GetValue<string>();
		if (validated.emplace(question_text, *members).second) {
			ValidateListed(question_text, *members, bound.kind);
		}
		auto due = args.ColumnCount() >= 4 ? args.data[3].GetValue(row).GetValue<int64_t>() : -1;
		if (due < -1 || due > 4294967295000LL) {
			throw InvalidInputException("thinkthen usage: the deadline is outside the supported range");
		}
		auto literal_context = LiteralContext(args, row, 4);
		auto [place, fresh] = known.emplace(std::make_tuple(question_text, *members, due, literal_context), groups.size());
		if (fresh) {
			groups.push_back({question_text, *members, due, literal_context, {}, {}});
		}
		auto &group = groups[place->second];
		auto [position, first] = group.seen.emplace(evidence.GetValue<string>(), group.texts.size());
		if (first) {
			group.texts.push_back(position->first);
		}
		slots[row] = std::make_pair(place->second, position->second);
	}
	vector<vector<Value>> answered;
	const auto settings = Settings(*context);
	for (auto &group : groups) {
		const auto budget = owner->Remaining(*context);
		const auto due = budget < 0 ? group.deadline : group.deadline < 0 ? budget : std::min(group.deadline, budget);
		vector<ThinkThenText> members, texts;
		for (auto &member : group.members) {
			members.push_back({reinterpret_cast<const uint8_t *>(member.data()), member.size()});
		}
		for (auto &text : group.texts) {
			texts.push_back({reinterpret_cast<const uint8_t *>(text.data()), text.size()});
		}
		RustReply reply(thinkthen_cpp_listed_group(reinterpret_cast<const uint8_t *>(group.question.data()),
		                                           group.question.size(), members.data(), members.size(), texts.data(),
		                                           texts.size(), due, bound.kind, settings.Bridge(),
		                                           group.context ? reinterpret_cast<const uint8_t *>(group.context->data()) : nullptr,
		                                           group.context ? group.context->size() : 0, StopFor(*context)));
		Checked(reply.value);
		answered.push_back(DecodeListed(reply.value.bytes, reply.value.len, texts.size(), bound.kind));
	}
	const auto type = bound.kind == 5 ? LogicalType::DOUBLE : bound.kind == 6 ? LogicalType::LIST(LogicalType::VARCHAR)
	                                                                       : LogicalType::VARCHAR;
	for (idx_t row = 0; row < args.size(); ++row) {
		if (slots[row]) {
			auto [group, text] = *slots[row];
			result.SetValue(row, answered[group][text]);
		} else {
			result.SetValue(row, Value(type));
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
	config.AddExtensionOption("thinkthen_batch", "Maximum records in one ThinkThen request, or max", LogicalType::VARCHAR);
	config.AddExtensionOption("thinkthen_throttle", "Maximum concurrent ThinkThen requests", LogicalType::BIGINT);
	config.AddExtensionOption("thinkthen_max_requests", "Maximum ThinkThen requests in one call", LogicalType::BIGINT);
	config.AddExtensionOption("thinkthen_max_request_bytes", "Positive ThinkThen request-byte ceiling", LogicalType::BIGINT);
	config.AddExtensionOption("thinkthen_max_requests_total", "Maximum ThinkThen requests in this process",
	                          LogicalType::BIGINT);
	config.AddExtensionOption("thinkthen_cache", "Local ThinkThen cache folder", LogicalType::VARCHAR);
	config.AddExtensionOption("thinkthen_model", "ThinkThen model", LogicalType::VARCHAR);
	config.AddExtensionOption("thinkthen_timeout", "Live attempt timeout in seconds", LogicalType::BIGINT);
	config.AddExtensionOption("thinkthen_max_retries", "Maximum live retries", LogicalType::BIGINT);
	config.AddExtensionOption("thinkthen_profile", "Inline backend limits profile JSON", LogicalType::VARCHAR);
	config.AddExtensionOption("thinkthen_record", "Local recording folder", LogicalType::VARCHAR);
	config.AddExtensionOption("thinkthen_replay", "Local strict replay folder", LogicalType::VARCHAR);
	for (auto name : {"thinkthen_decide", "thinkthen_probability", "thinkthen_details", "thinkthen_try_details", "thinkthen_annotate"}) {
		const auto result = string(name) == "thinkthen_details" || string(name) == "thinkthen_try_details" || string(name) == "thinkthen_annotate" ? LogicalType::VARCHAR
		                    : string(name) == "thinkthen_probability" ? LogicalType::DOUBLE : LogicalType::BOOLEAN;
		for (auto parameters : {vector<LogicalType>{LogicalType::VARCHAR, LogicalType::VARCHAR},
		                        vector<LogicalType>{LogicalType::VARCHAR, LogicalType::VARCHAR, LogicalType::BIGINT},
		                        vector<LogicalType>{LogicalType::VARCHAR, LogicalType::VARCHAR, LogicalType::BIGINT, LogicalType::VARCHAR}}) {
			if (parameters.size() == 4 && string(name) == "thinkthen_annotate") { continue; }
			ScalarFunction function(name, parameters, result, Decide, BindDecide);
			if (string(name) == "thinkthen_try_details") {
				function.null_handling = FunctionNullHandling::SPECIAL_HANDLING;
			}
			function.SetStability(FunctionStability::VOLATILE);
			loader.RegisterFunction(function);
		}
	}
	for (auto name : {"thinkthen_choose", "thinkthen_score", "thinkthen_tag"}) {
		const auto result = string(name) == "thinkthen_score" ? LogicalType::DOUBLE
		                    : string(name) == "thinkthen_tag" ? LogicalType::LIST(LogicalType::VARCHAR)
		                                                       : LogicalType::VARCHAR;
		for (auto parameters : {vector<LogicalType>{LogicalType::VARCHAR, LogicalType::VARCHAR,
		                                                 LogicalType::LIST(LogicalType::VARCHAR)},
		                        vector<LogicalType>{LogicalType::VARCHAR, LogicalType::VARCHAR,
		                                                 LogicalType::LIST(LogicalType::VARCHAR), LogicalType::BIGINT},
		                        vector<LogicalType>{LogicalType::VARCHAR, LogicalType::VARCHAR,
		                                                 LogicalType::LIST(LogicalType::VARCHAR), LogicalType::BIGINT, LogicalType::VARCHAR}}) {
			ScalarFunction function(name, parameters, result, Listed, BindDecide);
			function.SetStability(FunctionStability::VOLATILE);
			loader.RegisterFunction(function);
		}
	}
	RegisterNested(loader);
	RegisterFind(loader);
	RegisterUsage(loader);
	RegisterWarm(loader);
	RegisterRelate(loader);
}

} // namespace duckdb

extern "C" {
DUCKDB_CPP_EXTENSION_ENTRY(thinkthen, loader) {
	duckdb::LoadThinkThen(loader);
}
}
