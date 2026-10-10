#include "descriptions.hpp"
#include "usage.hpp"
#include "json_result.hpp"
#include "bridge.hpp"
#include "duckdb/function/table_function.hpp"

#include <array>
#include <cstring>

namespace duckdb {
namespace {

struct UsageState : GlobalTableFunctionState {
	bool done = false;
};

void UsageStatus(DataChunk &input, ExpressionState &, Vector &output) {
	for (idx_t row = 0; row < input.size(); ++row) {
		RustReply reply(thinkthen_cpp_usage_status());
		Checked(reply.value);
		output.SetValue(row, ThinkThenJSON(string(reinterpret_cast<const char *>(reply.value.bytes), reply.value.len)));
	}
}

unique_ptr<FunctionData> BindUsage(ClientContext &, TableFunctionBindInput &,
                                   vector<LogicalType> &types, vector<string> &names) {
	types = {LogicalType::VARCHAR, LogicalType::BIGINT};
	names = {"metric", "value"};
	return nullptr;
}

unique_ptr<GlobalTableFunctionState> InitUsage(ClientContext &, TableFunctionInitInput &) {
	return make_uniq<UsageState>();
}

void ScanUsage(ClientContext &, TableFunctionInput &input, DataChunk &output) {
	auto &state = input.global_state->Cast<UsageState>();
	if (state.done) {
		return;
	}
	state.done = true;
	RustReply reply(thinkthen_cpp_usage());
	Checked(reply.value);
	constexpr std::array<const char *, 4> names = {"requests_sent", "cache_answers", "input_tokens", "output_tokens"};
	if (!reply.value.bytes || reply.value.len != names.size() * sizeof(int64_t)) {
		throw OrdinaryError("thinkthen defect: the bridge returned invalid usage counters");
	}
	for (idx_t row = 0; row < names.size(); ++row) {
		int64_t value;
		std::memcpy(&value, reply.value.bytes + row * sizeof(value), sizeof(value));
		output.SetValue(0, row, Value(names[row]));
		output.SetValue(1, row, Value::BIGINT(value));
	}
	output.SetCardinality(names.size());
}

} // namespace

void RegisterUsage(ExtensionLoader &loader) {
	TableFunction function("thinkthen_usage", {}, ScanUsage, BindUsage, InitUsage);
	RegisterDescribedTable(loader, function, "Read count-only usage totals.");
	ScalarFunction status("thinkthen_usage_status", {}, LogicalType::JSON(), UsageStatus);
	status.SetStability(FunctionStability::VOLATILE);
	RegisterDescribedScalar(loader, status, "Observe usage persistence without creating an engine or waiting for writes.");
}

} // namespace duckdb
