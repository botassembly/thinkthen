#include "portable.hpp"
#include "bridge.hpp"
#include "scalar_owner.hpp"
#include "scalar_settings.hpp"
#include "duckdb/common/weak_ptr_ipp.hpp"
#include "duckdb/planner/expression/bound_function_expression.hpp"

#include <array>
#include <cstring>
#include <limits>
#include <optional>
#include <vector>

namespace duckdb {
namespace {

LogicalType PlanType() {
	return LogicalType::STRUCT({{"records", LogicalType::BIGINT}, {"requests", LogicalType::BIGINT},
	                            {"estimated_bytes", LogicalType::BIGINT},
	                            {"estimated_input_tokens", LogicalType::STRUCT({{"lower", LogicalType::BIGINT},
	                                                                             {"upper", LogicalType::BIGINT}})},
	                            {"upper_bound", LogicalType::BOOLEAN}, {"first_body", LogicalType::VARCHAR}});
}

struct PlanBind : FunctionData {
	weak_ptr<ClientContext> context;
	explicit PlanBind(weak_ptr<ClientContext> value) : context(std::move(value)) {
	}
	unique_ptr<FunctionData> Copy() const override { return make_uniq<PlanBind>(context); }
	bool Equals(const FunctionData &other) const override {
		return context.lock() == other.Cast<PlanBind>().context.lock();
	}
};

unique_ptr<FunctionData> Bind(ClientContext &context, ScalarFunction &, vector<unique_ptr<Expression>> &) {
	context.registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
	return make_uniq<PlanBind>(context.shared_from_this());
}

struct Input {
	ResolvedQuestion question;
	string keyed;
	string settings;
};

const uint8_t *Bytes(const string &value) { return reinterpret_cast<const uint8_t *>(value.data()); }

Value Decode(const ThinkThenReply &reply) {
	constexpr size_t header = 6 * sizeof(uint64_t) + 2;
	if (!reply.bytes || reply.len < header) {
		throw OrdinaryError("thinkthen defect: the bridge returned no plan values");
	}
	std::array<int64_t, 6> counts;
	for (size_t index = 0; index < counts.size(); ++index) {
		uint64_t value;
		std::memcpy(&value, reply.bytes + index * sizeof(value), sizeof(value));
		if (value > uint64_t(std::numeric_limits<int64_t>::max())) {
			throw OrdinaryError("thinkthen defect: a plan count exceeds the SQL range");
		}
		counts[index] = int64_t(value);
	}
	const auto upper = reply.bytes[6 * sizeof(uint64_t)];
	const auto present = reply.bytes[6 * sizeof(uint64_t) + 1];
	if (upper > 1 || present > 1 || (!present && counts[5] != 0) ||
	    size_t(counts[5]) != reply.len - header) {
		throw OrdinaryError("thinkthen defect: the bridge returned an invalid plan shape");
	}
	const auto body = present ? Value(string(reinterpret_cast<const char *>(reply.bytes + header), size_t(counts[5])))
	                          : Value(LogicalType::VARCHAR);
	return Value::STRUCT({{"records", Value::BIGINT(counts[0])}, {"requests", Value::BIGINT(counts[1])},
	                      {"estimated_bytes", Value::BIGINT(counts[2])},
	                      {"estimated_input_tokens", Value::STRUCT({{"lower", Value::BIGINT(counts[3])},
	                                                               {"upper", Value::BIGINT(counts[4])}})},
	                      {"upper_bound", Value::BOOLEAN(upper != 0)}, {"first_body", body}});
}

void Plan(DataChunk &args, ExpressionState &state, Vector &result) {
	auto context = state.expr.Cast<BoundFunctionExpression>().bind_info->Cast<PlanBind>().context.lock();
	if (!context) { throw OrdinaryError("thinkthen defect: the caller session ended"); }
	auto owner = context->registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
	vector<std::optional<Input>> calls(args.size());
	for (idx_t row = 0; row < args.size(); ++row) {
		auto question = args.data[0].GetValue(row);
		auto keyed = args.data[1].GetValue(row);
		if (question.IsNull() || keyed.IsNull()) { continue; }
		const auto type = args.data[3].GetValue(row).GetValue<string>();
		if (type != "\"NULL\"" && type != "VARCHAR") {
			throw OrdinaryError("thinkthen usage: plan settings are one JSON text object");
		}
		auto setting = args.data[2].GetValue(row);
		Input call {owner->Resolve(*context, question.GetValue<string>()), keyed.GetValue<string>(),
		            setting.IsNull() ? string("{}") : setting.GetValue<string>()};
		RustReply checked(thinkthen_cpp_validate_plan(Bytes(call.question.text), call.question.text.size(),
		                                            call.question.from_file ? 1 : 0, Bytes(call.keyed), call.keyed.size(),
		                                            Bytes(call.settings), call.settings.size()));
		Checked(checked.value);
		calls[row] = std::move(call);
	}
	const auto session = Settings(*context);
	for (idx_t row = 0; row < args.size(); ++row) {
		if (!calls[row]) { result.SetValue(row, Value(PlanType())); continue; }
		const auto &call = *calls[row];
		RustReply reply(thinkthen_cpp_plan(Bytes(call.question.text), call.question.text.size(),
		                                call.question.from_file ? 1 : 0, Bytes(call.keyed), call.keyed.size(),
		                                Bytes(call.settings), call.settings.size(), session.Bridge()));
		Checked(reply.value);
		result.SetValue(row, Decode(reply.value));
	}
}

} // namespace

void RegisterPlan(ExtensionLoader &loader) {
	ScalarFunction function("thinkthen_native_plan", {LogicalType::VARCHAR, LogicalType::VARCHAR,
	                                                LogicalType::VARCHAR, LogicalType::VARCHAR},
	                        PlanType(), Plan, Bind);
	function.null_handling = FunctionNullHandling::SPECIAL_HANDLING;
	function.SetStability(FunctionStability::VOLATILE);
	loader.RegisterFunction(function);
	RegisterPortableMacro(loader, "CREATE MACRO thinkthen_plan(question, keyed_json, settings := NULL) AS "
	                              "thinkthen_native_plan(question, keyed_json, CAST(settings AS VARCHAR), typeof(settings))");
}

} // namespace duckdb
