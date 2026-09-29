#include "warm.hpp"
#include "duckdb/common/exception.hpp"
#include "duckdb/function/function_set.hpp"

namespace duckdb {
namespace {

void RemovedWarm(DataChunk &, ExpressionState &, Vector &) {
	throw InvalidInputException("thinkthen usage: thinkthen_warm was removed; pack records with thinkthen_decide_many");
}

} // namespace

void RegisterWarm(ExtensionLoader &loader) {
	ScalarFunctionSet overloads("thinkthen_warm");
	for (auto parameters : {vector<LogicalType>{LogicalType::VARCHAR, LogicalType::VARCHAR},
	                        vector<LogicalType>{LogicalType::VARCHAR, LogicalType::VARCHAR, LogicalType::VARCHAR}}) {
		ScalarFunction function("thinkthen_warm", parameters, LogicalType::BIGINT, RemovedWarm);
		function.null_handling = FunctionNullHandling::SPECIAL_HANDLING;
		overloads.AddFunction(function);
	}
	loader.RegisterFunction(overloads);
}

} // namespace duckdb
