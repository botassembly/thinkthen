#include "duckdb.hpp"
#include "duckdb/execution/expression_executor.hpp"
#include "duckdb/main/client_context.hpp"
#include "duckdb/main/client_context_state.hpp"
#include "duckdb/main/extension/extension_loader.hpp"
#include "duckdb/planner/expression/bound_function_expression.hpp"

#include <atomic>
#include <mutex>
#include <string>

namespace duckdb {
namespace {

constexpr const char *OWNER_KEY = "thinkthen_probe_owner";
std::atomic<uint64_t> probe_sends {0};
std::atomic<uint64_t> signal_sequence {0};

struct ProbeOwner : ClientContextState {
	std::mutex lock;
	uint64_t generation = 0;
	uint64_t calls = 0;
	uint64_t begun_at = 0;
	uint64_t consumed = 0;
	uint64_t ends = 0;
	uint64_t failures = 0;
	bool active = false;
	bool first_use = false;

	void QueryBegin(ClientContext &) override {
		std::lock_guard<std::mutex> held(lock);
		Start(false);
	}
	void QueryEnd(ClientContext &, optional_ptr<ErrorData> error) override {
		std::lock_guard<std::mutex> held(lock);
		if (active) {
			ends++;
			if (error && error->HasError()) {
				failures++;
			}
			if (signal_sequence.load() != begun_at) {
				consumed = signal_sequence.load();
			}
			active = false;
		}
	}
	void Start(bool late) {
		active = true;
		first_use = late;
		generation++;
		calls = 0;
		begun_at = signal_sequence.load();
	}
	string Observe() {
		std::lock_guard<std::mutex> held(lock);
		if (!active) {
			Start(true); // This state was registered after QueryBegin.
		}
		calls++;
		return string(first_use ? "first" : "begin") + ":" + std::to_string(generation) + ":" +
		       std::to_string(calls) + ":" + std::to_string(ends) + ":" +
		       std::to_string(failures) + ":" + std::to_string(consumed);
	}
};

struct ProbeBindData : FunctionData {
	weak_ptr<ClientContext> context;
	bool bad_constant = false;
	bool try_value = false;
	ProbeBindData(weak_ptr<ClientContext> context_p, bool bad_p, bool try_p)
	    : context(std::move(context_p)), bad_constant(bad_p), try_value(try_p) {
	}
	unique_ptr<FunctionData> Copy() const override {
		return make_uniq<ProbeBindData>(context, bad_constant, try_value);
	}
	bool Equals(const FunctionData &other) const override {
		auto &held = other.Cast<ProbeBindData>();
		return context.lock() == held.context.lock() && bad_constant == held.bad_constant &&
		       try_value == held.try_value;
	}
};

bool BadQuestion(const Value &value) {
	return !value.IsNull() && (value.GetValue<string>().empty() || value.GetValue<string>() == "bad");
}

unique_ptr<FunctionData> BindProbe(ClientContext &context, ScalarFunction &function,
	                               vector<unique_ptr<Expression>> &arguments) {
	context.registered_state->GetOrCreate<ProbeOwner>(OWNER_KEY);
	bool bad = false;
	if (!arguments.empty() && arguments[0]->IsFoldable()) {
		bad = BadQuestion(ExpressionExecutor::EvaluateScalar(context, *arguments[0]));
	}
	const bool try_value = function.name == "thinkthen_probe_try";
	if (bad && !try_value) {
		throw InvalidInputException("thinkthen usage: bad constant question");
	}
	return make_uniq<ProbeBindData>(context.shared_from_this(), bad, try_value);
}

ProbeBindData &Bound(ExpressionState &state) {
	return state.expr.Cast<BoundFunctionExpression>().bind_info->Cast<ProbeBindData>();
}

shared_ptr<ProbeOwner> Owner(ProbeBindData &bind) {
	auto context = bind.context.lock();
	if (!context) {
		throw InvalidInputException("thinkthen probe: caller session ended");
	}
	return context->registered_state->GetOrCreate<ProbeOwner>(OWNER_KEY);
}

void CallProbe(DataChunk &args, ExpressionState &state, Vector &result) {
	auto &bind = Bound(state);
	auto owner = Owner(bind);
	for (idx_t row = 0; row < args.size(); ++row) {
		auto question = args.data[0].GetValue(row);
		auto text = args.data[1].GetValue(row);
		if (question.IsNull() || text.IsNull()) {
			result.SetValue(row, Value(LogicalType::VARCHAR));
			continue;
		}
		if (bind.bad_constant || BadQuestion(question)) {
			if (!bind.try_value) {
				throw InvalidInputException("thinkthen usage: bad question");
			}
			result.SetValue(row, Value("failed:usage"));
			continue;
		}
		auto observed = owner->Observe();
		probe_sends++;
		result.SetValue(row, Value("answered:" + observed + ":" + text.GetValue<string>()));
	}
}

void OwnerProbe(DataChunk &args, ExpressionState &state, Vector &result) {
	auto owner = Owner(Bound(state));
	for (idx_t row = 0; row < args.size(); ++row) {
		result.SetValue(row, Value(owner->Observe()));
	}
}

void CountProbe(DataChunk &args, ExpressionState &, Vector &result) {
	for (idx_t row = 0; row < args.size(); ++row) {
		result.SetValue(row, Value::BIGINT(static_cast<int64_t>(probe_sends.load())));
	}
}

void SignalProbe(DataChunk &args, ExpressionState &, Vector &result) {
	for (idx_t row = 0; row < args.size(); ++row) {
		signal_sequence++;
		result.SetValue(row, args.data[0].GetValue(row));
	}
}

} // namespace

void LoadBindOwner(ExtensionLoader &loader) {
	for (auto name : {"thinkthen_probe_call", "thinkthen_probe_try"}) {
		ScalarFunction function(name, {LogicalType::VARCHAR, LogicalType::VARCHAR}, LogicalType::VARCHAR,
		                        CallProbe, BindProbe);
		function.SetStability(FunctionStability::VOLATILE);
		loader.RegisterFunction(function);
	}
	ScalarFunction owner("thinkthen_probe_owner", {LogicalType::VARCHAR}, LogicalType::VARCHAR,
	                     OwnerProbe, BindProbe);
	owner.SetStability(FunctionStability::VOLATILE);
	loader.RegisterFunction(owner);
	ScalarFunction count("thinkthen_probe_count", {}, LogicalType::BIGINT, CountProbe);
	count.SetStability(FunctionStability::VOLATILE);
	loader.RegisterFunction(count);
	ScalarFunction signal("thinkthen_probe_signal", {LogicalType::VARCHAR}, LogicalType::VARCHAR,
	                      SignalProbe);
	signal.SetStability(FunctionStability::VOLATILE);
	loader.RegisterFunction(signal);
}

} // namespace duckdb
