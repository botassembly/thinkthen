#include "descriptions.hpp"
#include "portable.hpp"
#include "json_result.hpp"
#include "bridge.hpp"
#include "listed_result.hpp"
#include "scalar_owner.hpp"
#include "scalar_settings.hpp"
#include "duckdb/common/weak_ptr_ipp.hpp"
#include "duckdb/execution/expression_executor.hpp"
#include "duckdb/parser/parser.hpp"
#include "duckdb/parser/statement/create_statement.hpp"
#include "duckdb/parser/parsed_data/create_macro_info.hpp"
#include "duckdb/planner/expression/bound_function_expression.hpp"

#include <map>
#include <cstring>
#include <optional>
#include <tuple>
#include <vector>

namespace duckdb {
namespace {

// The keyed kind `thinkthen_rank` sends; the bridge reads the same number.
constexpr int32_t RANK_KIND = 8;

struct PortableBind : FunctionData {
	weak_ptr<ClientContext> context;
	explicit PortableBind(weak_ptr<ClientContext> value) : context(std::move(value)) {
	}
	unique_ptr<FunctionData> Copy() const override {
		return make_uniq<PortableBind>(context);
	}
	bool Equals(const FunctionData &other) const override {
		return context.lock() == other.Cast<PortableBind>().context.lock();
	}
};

unique_ptr<FunctionData> Bind(ClientContext &context, const string &,
                              vector<unique_ptr<Expression>> &) {
	context.registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
	return make_uniq<PortableBind>(context.shared_from_this());
}

struct Group {
	ResolvedQuestion question;
	string settings;
	std::optional<string> threshold;
	vector<string> texts;
	std::map<string, idx_t> seen;
};

void Validate(const Group &group) {
	const auto *threshold = group.threshold ? reinterpret_cast<const uint8_t *>(group.threshold->data()) : nullptr;
	RustReply reply(thinkthen_cpp_validate_portable_decide(
	    reinterpret_cast<const uint8_t *>(group.question.text.data()), group.question.text.size(),
	    group.question.from_file ? 1 : 0,
	    reinterpret_cast<const uint8_t *>(group.settings.data()), group.settings.size(),
	    threshold, group.threshold ? group.threshold->size() : 0));
	Checked(reply.value);
}

void Decide(DataChunk &args, ExpressionState &state, Vector &result) {
	auto context = ThinkThenBindData(state.expr.Cast<BoundFunctionExpression>()).Cast<PortableBind>().context.lock();
	if (!context) {
		throw OrdinaryError("thinkthen defect: the caller session ended");
	}
	ValidateRequestTotal(*context);
	std::optional<SessionSettings> session;
	auto owner = context->registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
	vector<Group> groups;
	std::map<std::tuple<string, string, std::optional<string>>, idx_t> known;
	vector<std::optional<std::pair<idx_t, idx_t>>> slots(args.size());
	// Parse every local question and settings object before the first send.
	for (idx_t row = 0; row < args.size(); ++row) {
		auto question = args.data[0].GetValue(row);
		auto evidence = args.data[1].GetValue(row);
		if (question.IsNull() || evidence.IsNull()) {
			continue;
		}
		if (!session) { session.emplace(Settings(*context)); }
		const auto settings_type = args.data[4].GetValue(row).GetValue<string>();
		if (!PortableSettingsType(settings_type)) {
			throw InvalidInputException("thinkthen usage: the deadline and context moved into the settings object; pass '{\"deadline_ms\": …, \"context\": …}'");
		}
		const auto raw = question.GetValue<string>();
		const auto settings_value = args.data[2].GetValue(row);
		const auto settings = settings_value.IsNull() ? string("{}") : settings_value.GetValue<string>();
		const auto threshold_value = args.data[3].GetValue(row);
		const auto threshold = threshold_value.IsNull() ? std::nullopt
		                    : std::optional<string>(threshold_value.GetValue<string>());
		auto [place, fresh] = known.emplace(std::make_tuple(raw, settings, threshold), groups.size());
		if (fresh) {
			groups.push_back({owner->Resolve(*context, raw), settings, threshold, {}, {}});
			Validate(groups.back());
		}
		auto &group = groups[place->second];
		auto [position, first] = group.seen.emplace(evidence.GetValue<string>(), group.texts.size());
		if (first) {
			group.texts.push_back(position->first);
		}
		slots[row] = std::make_pair(place->second, position->second);
	}
	vector<vector<uint8_t>> answers;
	for (auto &group : groups) {
		vector<ThinkThenText> texts;
		texts.reserve(group.texts.size());
		for (auto &value : group.texts) {
			texts.push_back({reinterpret_cast<const uint8_t *>(value.data()), value.size()});
		}
		const auto *threshold = group.threshold ? reinterpret_cast<const uint8_t *>(group.threshold->data()) : nullptr;
		RustReply reply(thinkthen_cpp_portable_decide_group(
		    reinterpret_cast<const uint8_t *>(group.question.text.data()), group.question.text.size(),
		    group.question.from_file ? 1 : 0, texts.data(), texts.size(),
		    reinterpret_cast<const uint8_t *>(group.settings.data()), group.settings.size(),
		    threshold, group.threshold ? group.threshold->size() : 0,
		    owner->Remaining(*context), session->Bridge(), StopFor(*context)));
		Checked(reply.value);
		if (!reply.value.bytes || reply.value.len != group.texts.size()) {
			throw OrdinaryError("thinkthen defect: the bridge returned an invalid decision group");
		}
		answers.emplace_back(reply.value.bytes, reply.value.bytes + reply.value.len);
	}
	for (idx_t row = 0; row < args.size(); ++row) {
		if (!slots[row]) {
			result.SetValue(row, Value(LogicalType::BOOLEAN));
			continue;
		}
		auto [group, text] = *slots[row];
		switch (answers.at(group).at(text)) {
		case 0: result.SetValue(row, Value::BOOLEAN(false)); break;
		case 1: result.SetValue(row, Value::BOOLEAN(true)); break;
		case 2: result.SetValue(row, Value(LogicalType::BOOLEAN)); break;
		default: throw OrdinaryError("thinkthen defect: the bridge returned an invalid decision");
		}
	}
}

void Many(DataChunk &args, ExpressionState &state, Vector &result) {
	auto context = ThinkThenBindData(state.expr.Cast<BoundFunctionExpression>()).Cast<PortableBind>().context.lock();
	if (!context) {
		throw OrdinaryError("thinkthen defect: the caller session ended");
	}
	std::optional<SessionSettings> session;
	auto owner = context->registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
	struct Row { ResolvedQuestion question; string keyed; string settings; int32_t kind; };
	vector<std::optional<Row>> calls(args.size());
	for (idx_t row = 0; row < args.size(); ++row) {
		auto question = args.data[0].GetValue(row);
		auto keyed = args.data[1].GetValue(row);
		if (question.IsNull() || keyed.IsNull()) { continue; }
		if (!session) { session.emplace(Settings(*context)); }
		auto settings = args.data[2].GetValue(row);
		auto kind = args.data[3].GetValue(row).GetValue<int32_t>();
		// A rank question is literal text, as find's is, and never opens a file.
		auto asked = kind == RANK_KIND ? ResolvedQuestion {question.GetValue<string>(), false}
		                               : owner->Resolve(*context, question.GetValue<string>());
		Row call {std::move(asked), keyed.GetValue<string>(),
		          settings.IsNull() ? string("{}") : settings.GetValue<string>(), kind};
		RustReply checked(thinkthen_cpp_validate_portable_many(
		    reinterpret_cast<const uint8_t *>(call.question.text.data()), call.question.text.size(),
		    call.question.from_file ? 1 : 0,
		    reinterpret_cast<const uint8_t *>(call.keyed.data()), call.keyed.size(),
		    reinterpret_cast<const uint8_t *>(call.settings.data()), call.settings.size(), kind));
		Checked(checked.value);
		calls[row] = std::move(call);
	}
	for (idx_t row = 0; row < args.size(); ++row) {
		if (!calls[row]) {
			result.SetValue(row, Value(LogicalType::JSON()));
			continue;
		}
		auto &call = *calls[row];
		RustReply reply(thinkthen_cpp_portable_many(
		    reinterpret_cast<const uint8_t *>(call.question.text.data()), call.question.text.size(),
		    call.question.from_file ? 1 : 0,
		    reinterpret_cast<const uint8_t *>(call.keyed.data()), call.keyed.size(),
		    reinterpret_cast<const uint8_t *>(call.settings.data()), call.settings.size(), call.kind,
		    owner->Remaining(*context), session->Bridge(), StopFor(*context)));
		Checked(reply.value);
		result.SetValue(row, ThinkThenJSON(ReplyText(reply.value)));
	}
}

void Listed(DataChunk &args, ExpressionState &state, Vector &result) {
	if (args.size() == 0) { return; }
	auto context = ThinkThenBindData(state.expr.Cast<BoundFunctionExpression>()).Cast<PortableBind>().context.lock();
	if (!context) { throw OrdinaryError("thinkthen defect: the caller session ended"); }
	std::optional<SessionSettings> session;
	auto owner = context->registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
	struct ListedGroup {
		ResolvedQuestion question;
		std::optional<string> members;
		string settings;
		int32_t kind;
		vector<string> texts;
		std::map<string, idx_t> seen;
	};
	vector<ListedGroup> groups;
	std::map<std::tuple<string, std::optional<string>, string, int32_t>, idx_t> known;
	vector<std::optional<std::pair<idx_t, idx_t>>> slots(args.size());
	for (idx_t row = 0; row < args.size(); ++row) {
		auto question = args.data[0].GetValue(row);
		auto input = args.data[1].GetValue(row);
		if (question.IsNull() || input.IsNull()) { continue; }
		if (!session) { session.emplace(Settings(*context)); }
		const auto members_type = args.data[5].GetValue(row).GetValue<string>();
		const auto settings_type = args.data[6].GetValue(row).GetValue<string>();
		if ((!PortableSettingsType(members_type) && members_type != "VARCHAR[]") ||
		    (!PortableSettingsType(settings_type))) {
			throw InvalidInputException("thinkthen usage: the deadline and context moved into the settings object; pass '{\"deadline_ms\": …, \"context\": …}'");
		}
		auto member = args.data[2].GetValue(row);
		auto setting = args.data[3].GetValue(row);
		if (members_type == "VARCHAR" && settings_type != "\"NULL\"" && !setting.IsNull()) {
			throw OrdinaryError("thinkthen usage: a settings object follows members, not another settings object");
		}
		const auto members = member.IsNull() ? std::nullopt : std::optional<string>(member.GetValue<string>());
		const auto settings = setting.IsNull() ? string("{}") : setting.GetValue<string>();
		const auto kind = args.data[4].GetValue(row).GetValue<int32_t>();
		const auto raw = question.GetValue<string>();
		auto [place, fresh] = known.emplace(std::make_tuple(raw, members, settings, kind), groups.size());
		if (fresh) {
			ListedGroup group {owner->Resolve(*context, raw), members, settings, kind, {}, {}};
			const auto *list = group.members ? reinterpret_cast<const uint8_t *>(group.members->data()) : nullptr;
			RustReply checked(thinkthen_cpp_validate_portable_listed(
			    reinterpret_cast<const uint8_t *>(group.question.text.data()), group.question.text.size(),
			    group.question.from_file ? 1 : 0, list, group.members ? group.members->size() : 0,
			    reinterpret_cast<const uint8_t *>(group.settings.data()), group.settings.size(), kind));
			Checked(checked.value);
			groups.push_back(std::move(group));
		}
		auto &group = groups[place->second];
		auto [position, first] = group.seen.emplace(input.GetValue<string>(), group.texts.size());
		if (first) { group.texts.push_back(position->first); }
		slots[row] = std::make_pair(place->second, position->second);
	}
	vector<vector<Value>> answers;
	for (auto &group : groups) {
		vector<ThinkThenText> texts;
		for (auto &value : group.texts) {
			texts.push_back({reinterpret_cast<const uint8_t *>(value.data()), value.size()});
		}
		const auto *members = group.members ? reinterpret_cast<const uint8_t *>(group.members->data()) : nullptr;
		RustReply reply(thinkthen_cpp_portable_listed_group(
		    reinterpret_cast<const uint8_t *>(group.question.text.data()), group.question.text.size(),
		    group.question.from_file ? 1 : 0, texts.data(), texts.size(),
		    members, group.members ? group.members->size() : 0,
		    reinterpret_cast<const uint8_t *>(group.settings.data()), group.settings.size(), group.kind,
		    owner->Remaining(*context), session->Bridge(), StopFor(*context)));
		Checked(reply.value);
		answers.push_back(DecodeListed(reply.value.bytes, reply.value.len, texts.size(), group.kind));
	}
	const auto kind = args.data[4].GetValue(0).GetValue<int32_t>();
	const auto null_type = kind == 5 ? LogicalType::DOUBLE : kind == 6 ? LogicalType::LIST(LogicalType::VARCHAR)
	                                                                  : LogicalType::VARCHAR;
	for (idx_t row = 0; row < args.size(); ++row) {
		if (slots[row]) {
			auto [group, text] = *slots[row];
			result.SetValue(row, answers.at(group).at(text));
		} else {
			result.SetValue(row, Value(null_type));
		}
	}
}

void Scalar(DataChunk &args, ExpressionState &state, Vector &result) {
	if (args.size() == 0) { return; }
	auto context = ThinkThenBindData(state.expr.Cast<BoundFunctionExpression>()).Cast<PortableBind>().context.lock();
	if (!context) { throw OrdinaryError("thinkthen defect: the caller session ended"); }
	std::optional<SessionSettings> session;
	auto owner = context->registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
	struct ScalarGroup {
		ResolvedQuestion question;
		string settings;
		int32_t kind;
		vector<string> texts;
		std::map<string, idx_t> seen;
	};
	vector<ScalarGroup> groups;
	std::map<std::tuple<string, string, int32_t>, idx_t> known;
	vector<std::optional<std::pair<idx_t, idx_t>>> slots(args.size());
	for (idx_t row = 0; row < args.size(); ++row) {
		auto question = args.data[0].GetValue(row);
		auto input = args.data[1].GetValue(row);
		if (question.IsNull() || input.IsNull()) { continue; }
		if (!session) { session.emplace(Settings(*context)); }
		const auto type = args.data[4].GetValue(row).GetValue<string>();
		if (!PortableSettingsType(type)) {
			throw InvalidInputException("thinkthen usage: the deadline and context moved into the settings object; pass '{\"deadline_ms\": …, \"context\": …}'");
		}
		const auto raw = question.GetValue<string>();
		auto setting = args.data[2].GetValue(row);
		const auto settings = setting.IsNull() ? string("{}") : setting.GetValue<string>();
		const auto kind = args.data[3].GetValue(row).GetValue<int32_t>();
		auto [place, fresh] = known.emplace(std::make_tuple(raw, settings, kind), groups.size());
		if (fresh) {
			ResolvedQuestion resolved;
			try {
				resolved = owner->Resolve(*context, raw);
			} catch (const InvalidInputException &) {
				if (kind != 3) { throw; }
				resolved = {raw, false};
			}
			ScalarGroup group {std::move(resolved), settings, kind, {}, {}};
			if (kind != 3) {
			RustReply checked(kind == 7 ? thinkthen_cpp_validate_portable_annotate(
			    reinterpret_cast<const uint8_t *>(group.question.text.data()), group.question.text.size(),
			    group.question.from_file ? 1 : 0,
			    reinterpret_cast<const uint8_t *>(group.settings.data()), group.settings.size())
			    : thinkthen_cpp_validate_portable_scalar(
			    reinterpret_cast<const uint8_t *>(group.question.text.data()), group.question.text.size(),
			    group.question.from_file ? 1 : 0,
			    reinterpret_cast<const uint8_t *>(group.settings.data()), group.settings.size(), kind));
			Checked(checked.value);
			}
			groups.push_back(std::move(group));
		}
		auto &group = groups[place->second];
		auto [position, first] = group.seen.emplace(input.GetValue<string>(), group.texts.size());
		if (first) { group.texts.push_back(position->first); }
		slots[row] = std::make_pair(place->second, position->second);
	}
	vector<vector<Value>> answers;
	for (auto &group : groups) {
		vector<ThinkThenText> texts;
		for (auto &value : group.texts) {
			texts.push_back({reinterpret_cast<const uint8_t *>(value.data()), value.size()});
		}
		RustReply reply(group.kind == 3 ? thinkthen_cpp_portable_try_details_group(
		    reinterpret_cast<const uint8_t *>(group.question.text.data()), group.question.text.size(),
		    group.question.from_file ? 1 : 0, texts.data(), texts.size(),
		    reinterpret_cast<const uint8_t *>(group.settings.data()), group.settings.size(),
		    owner->Remaining(*context), session->Bridge(), StopFor(*context))
		    : group.kind == 7 ? thinkthen_cpp_portable_annotate_group(
		    reinterpret_cast<const uint8_t *>(group.question.text.data()), group.question.text.size(),
		    group.question.from_file ? 1 : 0, texts.data(), texts.size(),
		    reinterpret_cast<const uint8_t *>(group.settings.data()), group.settings.size(),
		    owner->Remaining(*context), session->Bridge(), StopFor(*context))
		    : thinkthen_cpp_portable_scalar_group(
		    reinterpret_cast<const uint8_t *>(group.question.text.data()), group.question.text.size(),
		    group.question.from_file ? 1 : 0, texts.data(), texts.size(),
		    reinterpret_cast<const uint8_t *>(group.settings.data()), group.settings.size(), group.kind,
		    owner->Remaining(*context), session->Bridge(), StopFor(*context)));
		Checked(reply.value);
		vector<Value> values;
		if (!reply.value.bytes) { throw OrdinaryError("thinkthen defect: the bridge returned no scalar values"); }
		size_t at = 0;
		for (idx_t index = 0; index < texts.size(); ++index) {
			if (reply.value.len - at < sizeof(uint32_t)) { throw OrdinaryError("thinkthen defect: truncated details length"); }
			uint32_t length;
			std::memcpy(&length, reply.value.bytes + at, sizeof(length));
			at += sizeof(length);
			if (reply.value.len - at < length) { throw OrdinaryError("thinkthen defect: truncated details"); }
			values.push_back(ThinkThenJSON(string(reinterpret_cast<const char *>(reply.value.bytes + at), length)));
			at += length;
		}
		if (at != reply.value.len) { throw OrdinaryError("thinkthen defect: extra scalar bytes"); }
		answers.push_back(std::move(values));
	}
	for (idx_t row = 0; row < args.size(); ++row) {
		if (slots[row]) {
			auto [group, text] = *slots[row];
			result.SetValue(row, answers.at(group).at(text));
		} else {
			result.SetValue(row, Value(LogicalType::JSON()));
		}
	}
}

void OriginalSettings(DataChunk &args, ExpressionState &, Vector &result) {
	for (idx_t row = 0; row < args.size(); ++row) {
		auto value = args.data[0].GetValue(row);
		if (value.IsNull()) { result.SetValue(row, Value(LogicalType::VARCHAR)); continue; }
		const auto raw = value.GetValue<string>();
		RustReply reply(thinkthen_cpp_portable_settings({reinterpret_cast<const uint8_t *>(raw.data()), raw.size()}));
		Checked(reply.value);
		result.SetValue(row, value);
	}
}

void RegisterMacro(ExtensionLoader &loader, const string &sql, const string &purpose) {
	auto parser = ThinkThenMacroParser();
	parser.ParseQuery(sql);
	auto &statement = parser.statements.at(0)->Cast<CreateStatement>();
	ThinkThenMacroSchema(*statement.info);
	statement.info->internal = true;
	auto &info = statement.info->Cast<CreateMacroInfo>();
	FunctionDescription description;
	description.description = purpose;
	info.descriptions.push_back(std::move(description));
	loader.RegisterFunction(info);
}

} // namespace

void RegisterPortableMacro(ExtensionLoader &loader, const string &sql, const string &purpose) {
	RegisterMacro(loader, sql, purpose);
}

string PortableOptionParameters() {
	return ", context := NULL, model := NULL, batch := NULL, deadline_ms := NULL, allow_none := NULL, "
	       "true_reading := NULL, false_reading := NULL";
}

bool PortableSettingsType(const string &type) {
	return type == "\"NULL\"" || type == "VARCHAR" || type == "JSON" ||
	       type.rfind("STRUCT(", 0) == 0 || type.rfind("MAP(", 0) == 0;
}

string PortableJSONArgument(const string &argument) {
	return "CASE WHEN typeof(" + argument + ") IN ('VARCHAR', 'JSON') THEN CAST(" + argument +
	       " AS VARCHAR) ELSE CAST(to_json(" + argument + ") AS VARCHAR) END";
}

string PortableOptions(const string &settings, const string &nulls, bool legacy_type) {
	const auto guard = nulls + (legacy_type ? " OR (typeof(settings) NOT IN ('VARCHAR', 'JSON', '\"NULL\"') AND "
	    "NOT starts_with(typeof(settings), 'STRUCT(') AND NOT starts_with(typeof(settings), 'MAP('))" : "");
	string call = "thinkthen_native_options(" + PortableJSONArgument(settings);
	// Mask operands before their casts: the preview optimizer folds inner casts
	// even when an enclosing constant CASE already selects its NULL branch.
	for (auto parameter : {"context", "model", "batch", "deadline_ms", "allow_none", "true_reading", "false_reading"}) {
		call += ", CASE WHEN " + nulls + " THEN NULL ELSE " + parameter + " END";
	}
	return "CASE WHEN " + guard + " THEN NULL ELSE " + call + ") END";
}

void RegisterPortableDecide(ExtensionLoader &loader) {
	ScalarFunction checked("thinkthen_native_settings", {LogicalType::VARCHAR}, LogicalType::VARCHAR, OriginalSettings);
	ThinkThenSpecialNulls(checked);
	checked.SetStability(FunctionStability::VOLATILE);
	RegisterDescribedScalar(loader, checked, "Validate original settings through the shared native parser.");
	// Keep option serialization at the SQL edge. Null named values mean omission;
	// false, zero and ordered SQL collections retain their native values. Concatenate
	// the two objects so the shared parser still refuses duplicate settings keys.
	string encoded = "concat('{', concat_ws(','";
	for (auto option : {"context", "model", "batch", "deadline_ms", "allow_none", "true_reading", "false_reading"}) {
		const string parameter = option;
		const auto key = parameter == "allow_none" ? "none" : parameter == "true_reading" ? "true" : parameter == "false_reading" ? "false" : parameter;
		const auto value = parameter == "context" || parameter == "model" ? "CAST(" + parameter + " AS VARCHAR)"
		                 : parameter == "deadline_ms" ? "CAST(deadline_ms AS BIGINT)"
		                 : parameter == "allow_none" ? "CAST(allow_none AS BOOLEAN)" : parameter;
		encoded += ", CASE WHEN " + parameter + " IS NULL THEN NULL ELSE concat('\"" + key + "\":', to_json(" + value + ")) END";
	}
	encoded += "), '}')";
	RegisterMacro(loader, "CREATE MACRO thinkthen_native_options(settings, context, model, batch, deadline_ms, "
	                      "allow_none, true_reading, false_reading) AS CASE WHEN " + encoded + " = '{}' THEN thinkthen_native_settings(settings) ELSE "
	                      "concat(substr(trim(coalesce(thinkthen_native_settings(settings), '{}')), 1, length(trim(coalesce(thinkthen_native_settings(settings), '{}'))) - 1), "
	                      "CASE WHEN regexp_full_match(coalesce(thinkthen_native_settings(settings), '{}'), '[[:space:]]*[{][[:space:]]*[}][[:space:]]*') THEN '' ELSE ',' END, "
	                      "substr(" + encoded + ", 2)) END", "Encode named SQL controls for native admission.");
	const string purpose = "Judge whether text answers a yes-or-no question.";
	ScalarFunction native("thinkthen_native_decide",
	                      {LogicalType::VARCHAR, LogicalType::VARCHAR, LogicalType::VARCHAR, LogicalType::VARCHAR,
	                       LogicalType::VARCHAR},
	                      LogicalType::BOOLEAN, Decide, ThinkThenBindScalar<Bind>);
	ThinkThenSpecialNulls(native);
	native.SetStability(FunctionStability::VOLATILE);
	RegisterDescribedScalar(loader, native, purpose);
	// A scalar := argument is positional in DuckDB 1.5.5. This catalog macro
	// binds names before calling the native vector function.
	RegisterMacro(loader, "CREATE MACRO thinkthen_decide(question, input, settings := NULL, threshold := NULL" + PortableOptionParameters() + ") "
	                      "AS thinkthen_native_decide(" + PortableJSONArgument("question") + ", input, " + PortableOptions() + ", "
	                      "CAST(threshold AS VARCHAR), typeof(settings))", purpose);
	ScalarFunction many("thinkthen_native_many",
	                    {LogicalType::VARCHAR, LogicalType::VARCHAR, LogicalType::VARCHAR, LogicalType::INTEGER},
	                    LogicalType::JSON(), Many, ThinkThenBindScalar<Bind>);
	ThinkThenSpecialNulls(many);
	many.SetStability(FunctionStability::VOLATILE);
	RegisterDescribedScalar(loader, many, "Judge or rank keyed text records and return their values as JSON.");
	for (auto kind : {0, 4, 5, 6}) {
		string name = kind == 0 ? "decide" : kind == 4 ? "choose" : kind == 5 ? "score" : "tag";
		string conversion = kind == 0 ? "CAST(json_extract(item.value, '$.value') AS BOOLEAN)"
		                  : kind == 4 ? "json_extract_string(item.value, '$.value')"
		                  : kind == 5 ? "CAST(json_extract(item.value, '$.value') AS DOUBLE)"
		                              : "CAST(json_extract(item.value, '$.value') AS VARCHAR[])";
		string projection = "json_extract_string(item.value, '$.key') AS key, " + conversion + " AS value";
		if (kind == 0 || kind == 4) {
			projection += ", CAST(json_extract(item.value, '$.probability') AS DOUBLE) AS probability";
		}
		RegisterMacro(loader, "CREATE MACRO thinkthen_" + name + "_many(question, keyed_json, settings := NULL" + PortableOptionParameters() + ") "
		                      "AS TABLE SELECT " + projection + " FROM json_each(thinkthen_native_many(" + PortableJSONArgument("question") + ", " + PortableJSONArgument("keyed_json") + ", " + PortableOptions("settings", "question IS NULL OR keyed_json IS NULL", false) + ", "
		                      + std::to_string(kind) + ")) item", "Judge keyed text records with " + name + " and return one row per record.");
	}
	for (auto kind : {4, 5, 6}) {
		const string name = kind == 4 ? "choose" : kind == 5 ? "score" : "tag";
		const string purpose = kind == 4 ? "Choose an option for text." : kind == 5 ? "Score text against ordered levels." : "Tag text with matching labels.";
		const auto type = kind == 5 ? LogicalType::DOUBLE : kind == 6 ? LogicalType::LIST(LogicalType::VARCHAR)
		                                                : LogicalType::VARCHAR;
		ScalarFunction native(ThinkThenIdentifier("thinkthen_native_" + name),
		                      {LogicalType::VARCHAR, LogicalType::VARCHAR, LogicalType::VARCHAR,
		                       LogicalType::VARCHAR, LogicalType::INTEGER, LogicalType::VARCHAR,
		                       LogicalType::VARCHAR}, type, Listed, ThinkThenBindScalar<Bind>);
		ThinkThenSpecialNulls(native);
		native.SetStability(FunctionStability::VOLATILE);
		RegisterDescribedScalar(loader, native, purpose);
		RegisterMacro(loader, "CREATE MACRO thinkthen_" + name + "(question, input, members := NULL, settings := NULL" + PortableOptionParameters() + ") AS "
		                      "thinkthen_native_" + name + "(" + PortableJSONArgument("question") + ", input, "
		                      "CASE WHEN typeof(members) != 'VARCHAR' THEN CAST(to_json(members) AS VARCHAR) ELSE NULL END, "
		                      + PortableOptions("CASE WHEN members IS NOT NULL AND typeof(members) = 'VARCHAR' THEN CAST(members AS VARCHAR) ELSE " + PortableJSONArgument("settings") + " END") + ", "
		                      + std::to_string(kind) + ", CASE WHEN members IS NULL THEN '\"NULL\"' ELSE typeof(members) END, typeof(settings))", purpose);
	}
	RegisterMacro(loader, "CREATE MACRO thinkthen_rank(question, keyed_json, settings := NULL" + PortableOptionParameters() + ") "
	                      "AS TABLE SELECT json_extract_string(item.value, '$.key') AS key, "
	                      "CAST(json_extract(item.value, '$.rank') AS BIGINT) AS rank, "
	                      "CAST(json_extract(item.value, '$.probability') AS DOUBLE) AS probability "
	                      "FROM json_each(thinkthen_native_many(" + PortableJSONArgument("question") + ", " + PortableJSONArgument("keyed_json") + ", " + PortableOptions("settings", "question IS NULL OR keyed_json IS NULL", false) + ", "
	                      + std::to_string(RANK_KIND) + ")) item ORDER BY rank", "Rank keyed text records by their probability of yes.");
	RegisterMacro(loader, "CREATE MACRO thinkthen_rank_set(questions, keyed_json, settings := NULL" + PortableOptionParameters() + ") "
	                      "AS TABLE SELECT json_extract_string(item.value, '$.key') AS key, "
	                      "CAST(json_extract(item.value, '$.rank') AS BIGINT) AS rank, "
	                      "CAST(json_extract(item.value, '$.probability') AS DOUBLE) AS probability, "
	                      "json_extract_string(item.value, '$.question_name') AS question_name, "
	                      "json_extract(item.value, '$.facts') AS facts "
	                      "FROM json_each(thinkthen_native_many(" + PortableJSONArgument("questions") + ", " + PortableJSONArgument("keyed_json") + ", " + PortableOptions("settings", "questions IS NULL OR keyed_json IS NULL", false) + ", 9)) item ORDER BY rank", "Rank keyed text records for each named question and include request facts.");
	for (auto kind : {2, 3, 7}) {
		const string name = kind == 2 ? "details" : kind == 3 ? "try_details" : "annotate";
		const string purpose = kind == 2 ? "Return judgment details as JSON." : kind == 3 ? "Return judgment details or a safe recoverable failure as JSON." : "Annotate text with a question set and return its values as JSON.";
		ScalarFunction native(ThinkThenIdentifier("thinkthen_native_" + name),
		                      {LogicalType::VARCHAR, LogicalType::VARCHAR, LogicalType::VARCHAR,
		                       LogicalType::INTEGER, LogicalType::VARCHAR},
		                      LogicalType::JSON(), Scalar, ThinkThenBindScalar<Bind>);
		ThinkThenSpecialNulls(native);
		native.SetStability(FunctionStability::VOLATILE);
		RegisterDescribedScalar(loader, native, purpose);
		RegisterMacro(loader, "CREATE MACRO thinkthen_" + name + "(question, input, settings := NULL" + PortableOptionParameters() + ") AS "
		                      "thinkthen_native_" + name + "(" + PortableJSONArgument("question") + ", input, " + PortableOptions() + ", "
		                      + std::to_string(kind) + ", typeof(settings))", purpose);
	}
}

} // namespace duckdb
