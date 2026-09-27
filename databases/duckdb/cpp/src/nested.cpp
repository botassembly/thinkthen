#include "bridge.hpp"
#include "listed_result.hpp"
#include "nested.hpp"
#include "nested_result.hpp"
#include "scalar_owner.hpp"
#include "scalar_settings.hpp"
#include "duckdb/common/weak_ptr_ipp.hpp"
#include "duckdb/execution/expression_executor.hpp"
#include "duckdb/main/extension/extension_loader.hpp"
#include "duckdb/planner/expression/bound_function_expression.hpp"

#include <algorithm>
#include <map>
#include <optional>
#include <set>
#include <tuple>

namespace duckdb {
namespace {

void ValidateNested(int32_t kind, const ResolvedQuestion &argument, const vector<string> &members) {
	vector<ThinkThenText> copied;
	for (auto &member : members) {
		copied.push_back({reinterpret_cast<const uint8_t *>(member.data()), member.size()});
	}
	RustReply reply(thinkthen_cpp_validate_nested(reinterpret_cast<const uint8_t *>(argument.text.data()),
	                                            argument.text.size(), copied.data(), copied.size(), kind,
	                                            argument.from_file ? 1 : 0));
	Checked(reply.value);
}

struct NestedBind : FunctionData {
	weak_ptr<ClientContext> context;
	int32_t kind;
	std::optional<string> constant_argument;
	std::optional<ResolvedQuestion> resolved_argument;

	NestedBind(weak_ptr<ClientContext> context, int32_t kind) : context(std::move(context)), kind(kind) {
	}
	unique_ptr<FunctionData> Copy() const override {
		auto copy = make_uniq<NestedBind>(context, kind);
		copy->constant_argument = constant_argument;
		copy->resolved_argument = resolved_argument;
		return copy;
	}
	bool Equals(const FunctionData &other) const override {
		auto &held = other.Cast<NestedBind>();
		return context.lock() == held.context.lock() && kind == held.kind &&
		       constant_argument == held.constant_argument && resolved_argument == held.resolved_argument;
	}
};

unique_ptr<FunctionData> BindNested(ClientContext &context, ScalarFunction &function,
                                     vector<unique_ptr<Expression>> &arguments) {
	context.registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
	const auto kind = function.name == "thinkthen_recognize" ? 8 : 9;
	auto bound = make_uniq<NestedBind>(context.shared_from_this(), kind);
	if (arguments[1]->IsFoldable()) {
		auto value = ExpressionExecutor::EvaluateScalar(context, *arguments[1]);
		if (!value.IsNull()) {
			if (kind == 8) {
				if (auto members = Members(value)) {
					ValidateNested(kind, {"", false}, *members);
				}
			} else {
				bound->constant_argument = value.GetValue<string>();
				bound->resolved_argument = ResolveQuestion(context, *bound->constant_argument);
				ValidateNested(kind, *bound->resolved_argument, {});
			}
		}
	}
	if (kind == 9 && arguments.size() == 3 && arguments[2]->IsFoldable()) {
		auto value = ExpressionExecutor::EvaluateScalar(context, *arguments[2]);
		if (!value.IsNull()) {
			const auto due = value.GetValue<int64_t>();
			if (due < -1 || due > 4294967295000LL) {
				throw InvalidInputException("thinkthen usage: the deadline is outside the supported range");
			}
		}
	}
	return bound;
}

struct Group {
	ResolvedQuestion argument;
	vector<string> members;
	int64_t deadline;
	vector<string> texts;
	std::map<string, idx_t> seen;
};

void Nested(DataChunk &args, ExpressionState &state, Vector &result) {
	auto &bound = state.expr.Cast<BoundFunctionExpression>().bind_info->Cast<NestedBind>();
	auto context = bound.context.lock();
	if (!context) {
		throw InvalidInputException("thinkthen defect: the caller session ended");
	}
	auto owner = context->registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
	vector<Group> groups;
	std::map<std::tuple<string, vector<string>, int64_t>, idx_t> known;
	std::set<std::pair<string, vector<string>>> validated;
	std::map<string, ResolvedQuestion> resolved;
	vector<std::optional<std::pair<idx_t, idx_t>>> slots(args.size());
	for (idx_t row = 0; row < args.size(); ++row) {
		auto evidence = args.data[0].GetValue(row);
		auto argument = args.data[1].GetValue(row);
		if (evidence.IsNull() || argument.IsNull() ||
		    (args.ColumnCount() == 3 && args.data[2].GetValue(row).IsNull())) {
			continue;
		}
		auto members = bound.kind == 8 ? Members(argument) : std::optional<vector<string>>(vector<string>());
		if (!members) {
			continue;
		}
		const auto raw = bound.kind == 8 ? string() : argument.GetValue<string>();
		if (validated.emplace(raw, *members).second) {
			auto named = bound.constant_argument && *bound.constant_argument == raw && bound.resolved_argument
			                 ? *bound.resolved_argument : bound.kind == 8 ? ResolvedQuestion {"", false}
			                                                              : owner->Resolve(*context, raw);
			ValidateNested(bound.kind, named, *members);
			resolved.emplace(raw, std::move(named));
		}
		const auto due = args.ColumnCount() == 3 ? args.data[2].GetValue(row).GetValue<int64_t>() : -1;
		if (due < -1 || due > 4294967295000LL) {
			throw InvalidInputException("thinkthen usage: the deadline is outside the supported range");
		}
		auto [place, fresh] = known.emplace(std::make_tuple(raw, *members, due), groups.size());
		if (fresh) {
			groups.push_back({resolved.at(raw), *members, due, {}, {}});
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
		RustReply reply(thinkthen_cpp_nested_group(reinterpret_cast<const uint8_t *>(group.argument.text.data()),
		                                           group.argument.text.size(), members.data(), members.size(), texts.data(),
		                                           texts.size(), due, bound.kind, settings.Bridge(),
		                                           group.argument.from_file ? 1 : 0, StopFor(*context)));
		Checked(reply.value);
		answered.push_back(DecodeNested(reply.value.bytes, reply.value.len, texts.size(), bound.kind));
	}
	for (idx_t row = 0; row < args.size(); ++row) {
		if (slots[row]) {
			auto [group, text] = *slots[row];
			result.SetValue(row, answered[group][text]);
		} else {
			result.SetValue(row, Value(NestedType(bound.kind)));
		}
	}
}

} // namespace

void RegisterNested(ExtensionLoader &loader) {
	ScalarFunction recognize("thinkthen_recognize",
	                         {LogicalType::VARCHAR, LogicalType::LIST(LogicalType::VARCHAR)},
	                         NestedType(8), Nested, BindNested);
	recognize.SetStability(FunctionStability::VOLATILE);
	loader.RegisterFunction(recognize);
	for (auto parameters : {vector<LogicalType>{LogicalType::VARCHAR, LogicalType::VARCHAR},
	                        vector<LogicalType>{LogicalType::VARCHAR, LogicalType::VARCHAR, LogicalType::BIGINT}}) {
		ScalarFunction function("thinkthen_relations", parameters, NestedType(9), Nested, BindNested);
		function.SetStability(FunctionStability::VOLATILE);
		loader.RegisterFunction(function);
	}
}

} // namespace duckdb
