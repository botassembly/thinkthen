#include "descriptions.hpp"
#include "relate.hpp"
#include "relate_query.hpp"
#include "bridge.hpp"
#include "scalar_owner.hpp"
#include "scalar_settings.hpp"
#include "duckdb/function/table_function.hpp"

#include <cstring>
#include <limits>
#include <optional>

namespace duckdb {
namespace {

struct Rules {
	string text;
	vector<string> members;
	bool list = false;
	bool from_file = false;
};

vector<ThinkThenText> Texts(const vector<string> &values) {
	vector<ThinkThenText> copied;
	for (auto &value : values) {
		copied.push_back({reinterpret_cast<const uint8_t *>(value.data()), value.size()});
	}
	return copied;
}

Rules ReadRules(ClientContext &context, const Value &value) {
	if (value.IsNull()) {
		throw OrdinaryError("thinkthen usage: the relate rules are NULL or hold a NULL rule");
	}
	Rules rules;
	string call_settings;
	if (value.type().id() == LogicalTypeId::LIST) {
		rules.list = true;
		for (auto &member : ListValue::GetChildren(value)) {
			if (member.IsNull()) {
				throw OrdinaryError("thinkthen usage: the relate rules are NULL or hold a NULL rule");
			}
			rules.members.push_back(member.GetValue<string>());
		}
	} else {
		const auto resolved = ResolveQuestion(context, value.GetValue<string>(), "rules");
		rules.text = resolved.text;
		rules.from_file = resolved.from_file;
	}
	return rules;
}

uint64_t CountSetting(ClientContext &context, const char *name, uint64_t fallback, const char *refusal) {
	Value value;
	if (!context.TryGetCurrentSetting(name, value) || value.IsNull()) { return fallback; }
	const auto number = value.GetValue<int64_t>();
	if (number < 0) { throw OrdinaryError("thinkthen usage: %s", refusal); }
	return static_cast<uint64_t>(number);
}

struct RelateBind : FunctionData {
	weak_ptr<ClientContext> context;
	std::shared_ptr<RelateDatabase> held;
	string query;
	Value rule_input;
	Rules rules;
	string call_settings;
	SessionSettings settings;
	std::optional<string> search_path;
	uint64_t seconds = 60;
	uint64_t holding = 1000000;
	bool wildcard = false;

	unique_ptr<FunctionData> Copy() const override { return make_uniq<RelateBind>(*this); }
	bool Equals(const FunctionData &other) const override {
		auto &value = other.Cast<RelateBind>();
		return context.lock() == value.context.lock() && query == value.query && rules.text == value.rules.text &&
		       rules.members == value.rules.members && call_settings == value.call_settings &&
		       seconds == value.seconds && holding == value.holding;
	}
};

unique_ptr<FunctionData> BindRelate(ClientContext &context, TableFunctionBindInput &input,
                                     vector<LogicalType> &types, vector<string> &names) {
	types = {LogicalType::VARCHAR, LogicalType::VARCHAR, LogicalType::VARCHAR, LogicalType::DOUBLE, LogicalType::BOOLEAN};
	names = {"relation", "source", "target", "probability", "either"};
	auto bound = make_uniq<RelateBind>();
	context.registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
	bound->context = context.shared_from_this();
	if ((input.inputs.size() != 2 && input.inputs.size() != 3) || input.inputs[0].IsNull()) {
		throw OrdinaryError("thinkthen usage: the relate query is NULL or blank");
	}
	bound->query = input.inputs[0].GetValue<string>();
	if (bound->query.find_first_not_of(" \t\r\n") == string::npos) {
		throw OrdinaryError("thinkthen usage: the relate query is NULL or blank");
	}
	bound->rule_input = input.inputs[1];
	bound->rules = ReadRules(context, bound->rule_input);
	bound->call_settings = input.inputs.size() == 3 && !input.inputs[2].IsNull()
	                           ? input.inputs[2].GetValue<string>() : string("{}");
	bound->seconds = CountSetting(context, "thinkthen_relate_seconds", 60,
	                              "a relate time limit is a whole number of seconds, 0 for none");
	bound->holding = CountSetting(context, "thinkthen_relate_holding_rows", 1000000,
	                              "a relate holding limit is a whole number of rows");
	Value path;
	if (context.TryGetCurrentSetting("search_path", path) && !path.IsNull()) {
		auto text = path.GetValue<string>();
		if (text.find_first_not_of(" \t\r\n") != string::npos) { bound->search_path = std::move(text); }
	}
	bound->settings = Settings(context);
	bound->held = HoldRelateDatabase(context);
	auto members = Texts(bound->rules.members);
	RustReply validated(thinkthen_cpp_relate_validate(reinterpret_cast<const uint8_t *>(bound->rules.text.data()),
	                                                  bound->rules.text.size(), members.data(), members.size(),
	                                                  bound->rules.list ? 1 : 0, bound->rules.from_file ? 1 : 0,
	                                                  reinterpret_cast<const uint8_t *>(bound->call_settings.data()),
	                                                  bound->call_settings.size(),
	                                                  bound->settings.Bridge()));
	Checked(validated.value);
	if (!validated.value.bytes || validated.value.len != 1) {
		throw OrdinaryError("thinkthen defect: the bridge returned no relate rule kind");
	}
	bound->wildcard = validated.value.bytes[0] != 0;
	return bound;
}

struct RelateRow {
	string relation;
	string source;
	string target;
	double probability;
	bool either;
};

struct RelateState : GlobalTableFunctionState {
	bool ready = false;
	vector<RelateRow> rows;
	idx_t at = 0;
};

unique_ptr<GlobalTableFunctionState> InitRelate(ClientContext &, TableFunctionInitInput &) {
	return make_uniq<RelateState>();
}

class Reader {
public:
	Reader(const uint8_t *bytes, size_t len) : bytes(bytes), len(len) {}
	uint32_t Count() {
		if (len - at < sizeof(uint32_t)) { throw OrdinaryError("thinkthen defect: short relate reply"); }
		uint32_t value;
		std::memcpy(&value, bytes + at, sizeof(value)); at += sizeof(value); return value;
	}
	string Text() {
		const auto size = Count();
		if (size > len - at) { throw OrdinaryError("thinkthen defect: short relate text"); }
		string value(reinterpret_cast<const char *>(bytes + at), size); at += size; return value;
	}
	double Probability() {
		if (len - at < sizeof(double)) { throw OrdinaryError("thinkthen defect: short relate probability"); }
		double value;
		std::memcpy(&value, bytes + at, sizeof(value)); at += sizeof(value); return value;
	}
	bool Flag() {
		if (len - at < 1 || bytes[at] > 1) { throw OrdinaryError("thinkthen defect: bad relate either flag"); }
		return bytes[at++] == 1;
	}
	bool Done() const { return at == len; }
private:
	const uint8_t *bytes;
	size_t len;
	size_t at = 0;
};

vector<RelateRow> Answer(const RelateBind &bound, ClientContext &context, RelateFound found) {
	if (found.rows.size() > 255) {
		throw OrdinaryError("thinkthen usage: the relate query returned more than 255 rows, and relate reads at most 255; add a WHERE or a LIMIT");
	}
	if (found.columns != 2 && found.columns != 3) {
		throw OrdinaryError("thinkthen usage: the relate query returns id, name, and kind, or id and name");
	}
	if (found.columns == 2 && !bound.wildcard) {
		throw OrdinaryError("thinkthen usage: a relate query of id and name reads every kind as *, so every rule is bare or *:*");
	}
	vector<string> ids, names, kinds;
	for (idx_t row = 0; row < found.rows.size(); ++row) {
		auto &values = found.rows[row];
		if (!values[0] || !values[1]) {
			throw OrdinaryError("thinkthen usage: relate row %llu holds a NULL id or name", static_cast<unsigned long long>(row + 1));
		}
		if (found.columns == 3 && !values[2]) {
			throw OrdinaryError("thinkthen usage: relate row %llu holds a NULL kind", static_cast<unsigned long long>(row + 1));
		}
		ids.push_back(*values[0]); names.push_back(*values[1]);
		kinds.push_back(found.columns == 3 ? *values[2] : "*");
	}
	if (ids.empty()) { return {}; }
	auto id_bytes = Texts(ids), name_bytes = Texts(names), kind_bytes = Texts(kinds), members = Texts(bound.rules.members);
	RustReply reply(thinkthen_cpp_relate_rows(reinterpret_cast<const uint8_t *>(bound.rules.text.data()), bound.rules.text.size(),
	                                       members.data(), members.size(), bound.rules.list ? 1 : 0,
	                                       bound.rules.from_file ? 1 : 0, id_bytes.data(), name_bytes.data(),
	                                       kind_bytes.data(), ids.size(), found.remaining_ms,
	                                       reinterpret_cast<const uint8_t *>(bound.call_settings.data()),
	                                       bound.call_settings.size(), bound.settings.Bridge(), StopFor(context)));
	Checked(reply.value);
	if (!reply.value.bytes) { throw OrdinaryError("thinkthen defect: empty relate reply"); }
	Reader read(reply.value.bytes, reply.value.len);
	vector<RelateRow> edges;
	const auto count = read.Count();
	for (uint32_t row = 0; row < count; ++row) {
		edges.push_back({read.Text(), read.Text(), read.Text(), read.Probability(), read.Flag()});
	}
	if (!read.Done()) { throw OrdinaryError("thinkthen defect: trailing relate reply bytes"); }
	return edges;
}

void ScanRelate(ClientContext &context, TableFunctionInput &input, DataChunk &output) {
	auto &bound = input.bind_data->Cast<RelateBind>();
	auto &state = input.global_state->Cast<RelateState>();
	if (!state.ready) {
		state.ready = true;
		// Prepared plans retain bind data when the caller changes file access.
		// Resolve and validate rules against the executing session before any send.
		auto current = bound;
		current.rules = ReadRules(context, bound.rule_input);
		current.settings = Settings(context);
		auto members = Texts(current.rules.members);
		RustReply validated(thinkthen_cpp_relate_validate(reinterpret_cast<const uint8_t *>(current.rules.text.data()),
		                                                  current.rules.text.size(), members.data(), members.size(),
		                                                  current.rules.list ? 1 : 0, current.rules.from_file ? 1 : 0,
		                                                  reinterpret_cast<const uint8_t *>(current.call_settings.data()),
		                                                  current.call_settings.size(),
		                                                  current.settings.Bridge()));
		Checked(validated.value);
		if (!validated.value.bytes || validated.value.len != 1) {
			throw OrdinaryError("thinkthen defect: the bridge returned no relate rule kind");
		}
		current.wildcard = validated.value.bytes[0] != 0;
		current.seconds = CountSetting(context, "thinkthen_relate_seconds", 60,
		                              "a relate time limit is a whole number of seconds, 0 for none");
		current.holding = CountSetting(context, "thinkthen_relate_holding_rows", 1000000,
		                              "a relate holding limit is a whole number of rows");
		Value path;
		current.search_path.reset();
		if (context.TryGetCurrentSetting("search_path", path) && !path.IsNull()) {
			auto text = path.GetValue<string>();
			if (text.find_first_not_of(" \t\r\n") != string::npos) { current.search_path = std::move(text); }
		}
		auto found = ReadRelateRows(*current.held, context, current.query, current.seconds, current.holding, current.search_path);
		state.rows = Answer(current, context, std::move(found));
	}
	const auto count = std::min<idx_t>(STANDARD_VECTOR_SIZE, state.rows.size() - state.at);
	for (idx_t row = 0; row < count; ++row) {
		auto &edge = state.rows[state.at + row];
		output.SetValue(0, row, Value(edge.relation));
		output.SetValue(1, row, Value(edge.source));
		output.SetValue(2, row, Value(edge.target));
		output.SetValue(3, row, Value::DOUBLE(edge.probability));
		output.SetValue(4, row, Value::BOOLEAN(edge.either));
	}
	state.at += count;
	output.SetCardinality(count);
}

} // namespace

void RegisterRelate(ExtensionLoader &loader) {
	auto &config = DBConfig::GetConfig(loader.GetDatabaseInstance());
	config.AddExtensionOption("thinkthen_relate_seconds", "Relate query time limit in seconds", LogicalType::BIGINT);
	config.AddExtensionOption("thinkthen_relate_holding_rows", "Relate plan holding-row limit", LogicalType::BIGINT);
	TableFunction function("thinkthen_relate", {LogicalType::VARCHAR, LogicalType::ANY}, ScanRelate, BindRelate, InitRelate);
	TableFunctionSet overloads("thinkthen_relate");
	overloads.AddFunction(function);
	TableFunction with_settings("thinkthen_relate", {LogicalType::VARCHAR, LogicalType::ANY,
	                                               LogicalType::VARCHAR}, ScanRelate, BindRelate, InitRelate);
	overloads.AddFunction(with_settings);
	RegisterDescribedTable(loader, overloads, "Judge relations between ids in committed query rows using caller-supplied rules.");
}

} // namespace duckdb
