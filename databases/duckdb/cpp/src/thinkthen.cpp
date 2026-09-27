#define DUCKDB_EXTENSION_MAIN

#include "duckdb.hpp"
#include "duckdb/common/exception.hpp"
#include "duckdb/common/weak_ptr_ipp.hpp"
#include "duckdb/execution/expression_executor.hpp"
#include "duckdb/main/client_context.hpp"
#include "duckdb/main/client_context_state.hpp"
#include "duckdb/main/extension/extension_loader.hpp"
#include "duckdb/planner/expression/bound_function_expression.hpp"

#include <cstdint>
#include <cstring>
#include <memory>
#include <mutex>
#include <optional>
#include <string>

extern "C" {
struct ThinkThenReply {
	int32_t status;
	uint8_t *bytes;
	size_t len;
};
int32_t thinkthen_cpp_init();
ThinkThenReply thinkthen_cpp_validate_question(const uint8_t *bytes, size_t len);
ThinkThenReply thinkthen_cpp_decide(const uint8_t *question, size_t question_len, const uint8_t *evidence,
                                    size_t evidence_len, int64_t deadline_ms, void *caller);
void thinkthen_cpp_free(uint8_t *bytes, size_t len);
}

namespace duckdb {
namespace {

constexpr const char *OWNER_KEY = "thinkthen_statement_owner";

struct RustReply {
	explicit RustReply(ThinkThenReply value) : value(value) {
	}
	~RustReply() {
		thinkthen_cpp_free(value.bytes, value.len);
	}
	RustReply(const RustReply &) = delete;
	RustReply &operator=(const RustReply &) = delete;
	ThinkThenReply value;
};

string ReplyText(const ThinkThenReply &reply) {
	if (!reply.bytes) {
		return "thinkthen defect: the bridge returned no error buffer";
	}
	return string(reinterpret_cast<const char *>(reply.bytes), reply.len);
}

void Checked(const ThinkThenReply &reply) {
	if (reply.status != 0) {
		throw InvalidInputException("%s", ReplyText(reply).c_str());
	}
}

struct StatementOwner : ClientContextState {
	std::mutex lock;
	uint64_t generation = 0;
	bool active = false;
	bool first_use = false;

	void QueryBegin(ClientContext &) override {
		std::lock_guard<std::mutex> held(lock);
		Start(false);
	}
	void QueryEnd(ClientContext &, optional_ptr<ErrorData>) override {
		std::lock_guard<std::mutex> held(lock);
		active = false;
	}
	void Start(bool late) {
		active = true;
		first_use = late;
		generation++;
	}
	void Observe() {
		std::lock_guard<std::mutex> held(lock);
		if (!active) {
			Start(true);
		}
	}
};

struct ScalarBind : FunctionData {
	weak_ptr<ClientContext> context;
	std::optional<string> constant_question;
	std::optional<int64_t> constant_deadline;

	explicit ScalarBind(weak_ptr<ClientContext> context_p) : context(std::move(context_p)) {
	}
	unique_ptr<FunctionData> Copy() const override {
		auto copy = make_uniq<ScalarBind>(context);
		copy->constant_question = constant_question;
		copy->constant_deadline = constant_deadline;
		return copy;
	}
	bool Equals(const FunctionData &other) const override {
		auto &held = other.Cast<ScalarBind>();
		return context.lock() == held.context.lock() && constant_question == held.constant_question &&
		       constant_deadline == held.constant_deadline;
	}
};

unique_ptr<FunctionData> BindDecide(ClientContext &context, ScalarFunction &,
                                     vector<unique_ptr<Expression>> &arguments) {
	context.registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
	auto bound = make_uniq<ScalarBind>(context.shared_from_this());
	if (arguments[0]->IsFoldable()) {
		auto value = ExpressionExecutor::EvaluateScalar(context, *arguments[0]);
		if (!value.IsNull()) {
			bound->constant_question = value.GetValue<string>();
			auto &text = *bound->constant_question;
			RustReply checked(thinkthen_cpp_validate_question(reinterpret_cast<const uint8_t *>(text.data()), text.size()));
			Checked(checked.value);
		}
	}
	if (arguments.size() == 3 && arguments[2]->IsFoldable()) {
		auto value = ExpressionExecutor::EvaluateScalar(context, *arguments[2]);
		if (!value.IsNull()) {
			bound->constant_deadline = value.GetValue<int64_t>();
			if (*bound->constant_deadline < -1 || *bound->constant_deadline > 4294967295000LL) {
				throw InvalidInputException("thinkthen usage: the deadline is outside the supported range");
			}
		}
	}
	return bound;
}

ScalarBind &Bound(ExpressionState &state) {
	return state.expr.Cast<BoundFunctionExpression>().bind_info->Cast<ScalarBind>();
}

void Decide(DataChunk &args, ExpressionState &state, Vector &result) {
	auto &bound = Bound(state);
	auto context = bound.context.lock();
	if (!context) {
		throw InvalidInputException("thinkthen defect: the caller session ended");
	}
	context->registered_state->GetOrCreate<StatementOwner>(OWNER_KEY)->Observe();
	// Validate the entire chunk before its first real engine call.
	for (idx_t row = 0; row < args.size(); ++row) {
		auto question = args.data[0].GetValue(row);
		auto evidence = args.data[1].GetValue(row);
		if (question.IsNull() || evidence.IsNull()) {
			continue;
		}
		if (args.ColumnCount() == 3 && args.data[2].GetValue(row).IsNull()) {
			continue;
		}
		auto text = question.GetValue<string>();
		RustReply checked(thinkthen_cpp_validate_question(reinterpret_cast<const uint8_t *>(text.data()), text.size()));
		Checked(checked.value);
		if (args.ColumnCount() == 3) {
			auto due = args.data[2].GetValue(row).GetValue<int64_t>();
			if (due < -1 || due > 4294967295000LL) {
				throw InvalidInputException("thinkthen usage: the deadline is outside the supported range");
			}
		}
	}
	for (idx_t row = 0; row < args.size(); ++row) {
		auto question = args.data[0].GetValue(row);
		auto evidence = args.data[1].GetValue(row);
		if (question.IsNull() || evidence.IsNull() ||
		    (args.ColumnCount() == 3 && args.data[2].GetValue(row).IsNull())) {
			result.SetValue(row, Value(LogicalType::BOOLEAN));
			continue;
		}
		auto question_text = question.GetValue<string>();
		auto evidence_text = evidence.GetValue<string>();
		auto due = args.ColumnCount() == 3 ? args.data[2].GetValue(row).GetValue<int64_t>() : -1;
		RustReply answered(thinkthen_cpp_decide(reinterpret_cast<const uint8_t *>(question_text.data()),
		                                       question_text.size(), reinterpret_cast<const uint8_t *>(evidence_text.data()),
		                                       evidence_text.size(), due, context.get()));
		Checked(answered.value);
		if (answered.value.len != 1 || !answered.value.bytes || answered.value.bytes[0] > 2) {
			throw InvalidInputException("thinkthen defect: the bridge returned an invalid decision");
		}
		switch (answered.value.bytes[0]) {
		case 0: result.SetValue(row, Value::BOOLEAN(false)); break;
		case 1: result.SetValue(row, Value::BOOLEAN(true)); break;
		default: result.SetValue(row, Value(LogicalType::BOOLEAN)); break;
		}
	}
}

} // namespace

void LoadThinkThen(ExtensionLoader &loader) {
	if (thinkthen_cpp_init() != 0) {
		throw InvalidInputException("thinkthen defect: the Rust bridge did not initialize");
	}
	for (auto parameters : {vector<LogicalType>{LogicalType::VARCHAR, LogicalType::VARCHAR},
	                        vector<LogicalType>{LogicalType::VARCHAR, LogicalType::VARCHAR, LogicalType::BIGINT}}) {
		ScalarFunction decide("thinkthen_decide", parameters, LogicalType::BOOLEAN, Decide, BindDecide);
		decide.SetStability(FunctionStability::VOLATILE);
		loader.RegisterFunction(decide);
	}
}

} // namespace duckdb

extern "C" {
DUCKDB_CPP_EXTENSION_ENTRY(thinkthen, loader) {
	duckdb::LoadThinkThen(loader);
}
}
