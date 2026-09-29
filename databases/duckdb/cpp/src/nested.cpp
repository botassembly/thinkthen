#include "bridge.hpp"
#include "listed_result.hpp"
#include "nested.hpp"
#include "nested_result.hpp"
#include "scalar_owner.hpp"
#include "scalar_settings.hpp"
#include "portable.hpp"
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

	NestedBind(weak_ptr<ClientContext> context, int32_t kind) : context(std::move(context)), kind(kind) {
	}
	unique_ptr<FunctionData> Copy() const override {
		auto copy = make_uniq<NestedBind>(context, kind);
		copy->constant_argument = constant_argument;
		return copy;
	}
	bool Equals(const FunctionData &other) const override {
		auto &held = other.Cast<NestedBind>();
		return context.lock() == held.context.lock() && kind == held.kind &&
		       constant_argument == held.constant_argument;
	}
};

unique_ptr<FunctionData> BindNested(ClientContext &context, ScalarFunction &function,
                                     vector<unique_ptr<Expression>> &arguments) {
	context.registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
	const auto kind = function.name == "thinkthen_native_recognize" ? 8 : 9;
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
				ValidateNested(kind, ResolveQuestion(context, *bound->constant_argument), {});
			}
		}
	}
	return bound;
}

struct Group {
	ResolvedQuestion argument;
	vector<string> members;
	string settings;
	vector<string> texts;
	std::map<string, idx_t> seen;
};

void Nested(DataChunk &args, ExpressionState &state, Vector &result) {
	auto &bound = state.expr.Cast<BoundFunctionExpression>().bind_info->Cast<NestedBind>();
	auto context = bound.context.lock();
	if (!context) {
		throw OrdinaryError("thinkthen defect: the caller session ended");
	}
	auto owner = context->registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
	vector<Group> groups;
	std::map<std::tuple<string, vector<string>, string>, idx_t> known;
	std::set<std::pair<string, vector<string>>> validated;
	std::map<string, ResolvedQuestion> resolved;
	vector<std::optional<std::pair<idx_t, idx_t>>> slots(args.size());
	for (idx_t row = 0; row < args.size(); ++row) {
		auto evidence = args.data[0].GetValue(row);
		auto argument = args.data[1].GetValue(row);
		if (evidence.IsNull() || argument.IsNull()) {
			continue;
		}
		const auto type = args.data[3].GetValue(row).GetValue<string>();
		if (type != "\"NULL\"" && type != "VARCHAR") {
			throw InvalidInputException("thinkthen usage: the deadline and context moved into the settings object; pass '{\"deadline_ms\": …, \"context\": …}'");
		}
		const auto setting = args.data[2].GetValue(row);
		const auto call = setting.IsNull() ? string("{}") : setting.GetValue<string>();
		auto members = bound.kind == 8 ? Members(argument) : std::optional<vector<string>>(vector<string>());
		if (!members) {
			continue;
		}
		const auto raw = bound.kind == 8 ? string() : argument.GetValue<string>();
		if (validated.emplace(raw, *members).second) {
			auto named = bound.kind == 8 ? ResolvedQuestion {"", false} : owner->Resolve(*context, raw);
			ValidateNested(bound.kind, named, *members);
			resolved.emplace(raw, std::move(named));
		}
		auto [place, fresh] = known.emplace(std::make_tuple(raw, *members, call), groups.size());
		if (fresh) {
			Group group {resolved.at(raw), *members, call, {}, {}};
			vector<ThinkThenText> views;
			for (auto &member : group.members) {
				views.push_back({reinterpret_cast<const uint8_t *>(member.data()), member.size()});
			}
			RustReply checked(thinkthen_cpp_validate_portable_nested(
			    reinterpret_cast<const uint8_t *>(group.argument.text.data()), group.argument.text.size(),
			    group.argument.from_file ? 1 : 0, views.data(), views.size(),
			    reinterpret_cast<const uint8_t *>(group.settings.data()), group.settings.size(), bound.kind));
			Checked(checked.value);
			groups.push_back(std::move(group));
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
		vector<ThinkThenText> members, texts;
		for (auto &member : group.members) {
			members.push_back({reinterpret_cast<const uint8_t *>(member.data()), member.size()});
		}
		for (auto &text : group.texts) {
			texts.push_back({reinterpret_cast<const uint8_t *>(text.data()), text.size()});
		}
		RustReply reply(thinkthen_cpp_portable_nested_group(reinterpret_cast<const uint8_t *>(group.argument.text.data()),
		                                           group.argument.text.size(), group.argument.from_file ? 1 : 0,
		                                           members.data(), members.size(), texts.data(), texts.size(),
		                                           reinterpret_cast<const uint8_t *>(group.settings.data()), group.settings.size(),
		                                           bound.kind, budget, settings.Bridge(), StopFor(*context)));
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
	ScalarFunction recognize("thinkthen_native_recognize",
	                         {LogicalType::VARCHAR, LogicalType::LIST(LogicalType::VARCHAR),
	                          LogicalType::VARCHAR, LogicalType::VARCHAR},
	                         NestedType(8), Nested, BindNested);
	recognize.null_handling = FunctionNullHandling::SPECIAL_HANDLING;
	recognize.SetStability(FunctionStability::VOLATILE);
	loader.RegisterFunction(recognize);
	RegisterPortableMacro(loader, "CREATE MACRO thinkthen_recognize(input, kinds, settings := NULL) AS "
	                              "thinkthen_native_recognize(input, kinds, CAST(settings AS VARCHAR), typeof(settings))");
	ScalarFunction relations("thinkthen_native_relations",
	                         {LogicalType::VARCHAR, LogicalType::VARCHAR,
	                          LogicalType::VARCHAR, LogicalType::VARCHAR},
	                         NestedType(9), Nested, BindNested);
	relations.null_handling = FunctionNullHandling::SPECIAL_HANDLING;
	relations.SetStability(FunctionStability::VOLATILE);
	loader.RegisterFunction(relations);
	RegisterPortableMacro(loader, "CREATE MACRO thinkthen_relations(input, spec, settings := NULL) AS "
	                              "thinkthen_native_relations(input, spec, CAST(settings AS VARCHAR), typeof(settings))");
}

} // namespace duckdb
