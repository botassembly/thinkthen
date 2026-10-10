#include "descriptions.hpp"
#include "removed.hpp"
#include "portable.hpp"
#include "duckdb/common/exception.hpp"
#include "duckdb/function/function_set.hpp"

namespace duckdb {
namespace {

void RemovedWarm(DataChunk &, ExpressionState &, Vector &) {
	throw InvalidInputException("thinkthen usage: thinkthen_warm was removed; pack records with thinkthen_decide_many");
}

void RemovedProbability(DataChunk &, ExpressionState &, Vector &) {
	throw InvalidInputException("thinkthen usage: thinkthen_probability was removed; order records with thinkthen_rank");
}

} // namespace

void RegisterRemoved(ExtensionLoader &loader) {
	ScalarFunctionSet overloads("thinkthen_warm");
	for (auto parameters : {vector<LogicalType>{LogicalType::VARCHAR, LogicalType::VARCHAR},
	                        vector<LogicalType>{LogicalType::VARCHAR, LogicalType::VARCHAR, LogicalType::VARCHAR}}) {
		ScalarFunction function("thinkthen_warm", parameters, LogicalType::BIGINT, RemovedWarm);
		function.null_handling = FunctionNullHandling::SPECIAL_HANDLING;
		overloads.AddFunction(function);
	}
	RegisterDescribedScalar(loader, overloads, "Refuse the removed warm call and direct callers to thinkthen_decide_many.");
	// A macro binds the old named `settings :=` form too, so every spelling reaches the refusal.
	ScalarFunction probability("thinkthen_removed_probability", {}, LogicalType::DOUBLE, RemovedProbability);
	probability.SetStability(FunctionStability::VOLATILE);
	RegisterDescribedScalar(loader, probability, "Refuse the removed probability call and direct callers to thinkthen_rank.");
	RegisterPortableMacro(loader, "CREATE MACRO thinkthen_probability(question, input, settings := NULL) AS "
	                              "thinkthen_removed_probability()", "Refuse the removed probability call and direct callers to thinkthen_rank.");
}

} // namespace duckdb
