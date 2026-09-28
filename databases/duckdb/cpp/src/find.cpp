#include "find.hpp"
#include "bridge.hpp"
#include "scalar_owner.hpp"
#include "scalar_settings.hpp"
#include "duckdb/common/exception.hpp"
#include "duckdb/common/weak_ptr_ipp.hpp"
#include "duckdb/main/client_context.hpp"
#include "duckdb/main/client_context_state.hpp"
#include "duckdb/planner/expression/bound_function_expression.hpp"

#include <algorithm>
#include <cmath>
#include <cstdint>
#include <cstring>
#include <optional>
#include <string>
#include <vector>

namespace duckdb {
namespace {

LogicalType FindType() {
	auto candidate = LogicalType::STRUCT({{"index", LogicalType::BIGINT}, {"probability", LogicalType::DOUBLE}});
	return LogicalType::STRUCT({{"index", LogicalType::BIGINT}, {"value", LogicalType::VARCHAR},
	                            {"probability", LogicalType::DOUBLE}, {"candidates", LogicalType::LIST(candidate)}});
}

struct FindBind : FunctionData {
	weak_ptr<ClientContext> context;
	explicit FindBind(weak_ptr<ClientContext> context_p) : context(std::move(context_p)) {
	}
	unique_ptr<FunctionData> Copy() const override {
		return make_uniq<FindBind>(context);
	}
	bool Equals(const FunctionData &other) const override {
		return context.lock() == other.Cast<FindBind>().context.lock();
	}
};

unique_ptr<FunctionData> BindFind(ClientContext &context, ScalarFunction &, vector<unique_ptr<Expression>> &) {
	context.registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
	return make_uniq<FindBind>(context.shared_from_this());
}

struct Request {
	string question;
	vector<string> units;
	bool none;
	int64_t deadline;
};

vector<ThinkThenText> Views(const vector<string> &units) {
	vector<ThinkThenText> views;
	views.reserve(units.size());
	for (auto &unit : units) {
		views.push_back({reinterpret_cast<const uint8_t *>(unit.data()), unit.size()});
	}
	return views;
}

Request Read(DataChunk &args, idx_t row) {
	Request request;
	request.question = args.data[0].GetValue(row).GetValue<string>();
	auto list = args.data[1].GetValue(row);
	for (auto &child : ListValue::GetChildren(list)) {
		if (child.IsNull()) {
			throw InvalidInputException("thinkthen usage: each find unit is text, not NULL");
		}
		request.units.push_back(child.GetValue<string>());
	}
	request.none = args.ColumnCount() >= 3 && args.data[2].GetValue(row).GetValue<bool>();
	request.deadline = args.ColumnCount() == 4 ? args.data[3].GetValue(row).GetValue<int64_t>() : -1;
	if (request.deadline < -1 || request.deadline > 4294967295000LL) {
		throw InvalidInputException("thinkthen usage: the deadline is outside the supported range");
	}
	return request;
}

Value Decode(const ThinkThenReply &reply, const vector<string> &units, bool none) {
	const size_t minimum = sizeof(uint32_t) + sizeof(int64_t);
	if (!reply.bytes || reply.len < minimum) {
		throw InvalidInputException("thinkthen defect: the bridge returned no find values");
	}
	uint32_t count;
	int64_t selected;
	std::memcpy(&count, reply.bytes, sizeof(count));
	std::memcpy(&selected, reply.bytes + sizeof(count), sizeof(selected));
	if (count != units.size() + size_t(none) || count > 256 || reply.len != minimum + size_t(count) * 16 ||
	    selected < -1 || (selected == -1 && !none) || (selected >= 0 && uint64_t(selected) >= units.size())) {
		throw InvalidInputException("thinkthen defect: the bridge returned an invalid find shape");
	}
	vector<Value> candidates;
	candidates.reserve(count);
	double winner = 0;
	for (uint32_t at = 0; at < count; ++at) {
		int64_t index;
		double probability;
		const auto offset = minimum + size_t(at) * 16;
		std::memcpy(&index, reply.bytes + offset, sizeof(index));
		std::memcpy(&probability, reply.bytes + offset + sizeof(index), sizeof(probability));
		const auto expected = at == units.size() ? -1 : int64_t(at);
		if (index != expected || !std::isfinite(probability) || probability < 0 || probability > 1) {
			throw InvalidInputException("thinkthen defect: the bridge returned an invalid find candidate");
		}
		if (index == selected) { winner = probability; }
		candidates.push_back(Value::STRUCT({{"index", index < 0 ? Value(LogicalType::BIGINT) : Value::BIGINT(index)},
		                                    {"probability", Value::DOUBLE(probability)}}));
	}
	return Value::STRUCT({{"index", selected < 0 ? Value(LogicalType::BIGINT) : Value::BIGINT(selected)},
	                      {"value", selected < 0 ? Value(LogicalType::VARCHAR) : Value(units[size_t(selected)])},
	                      {"probability", Value::DOUBLE(winner)},
	                      {"candidates", Value::LIST(LogicalType::STRUCT({{"index", LogicalType::BIGINT},
	                                                                       {"probability", LogicalType::DOUBLE}}), candidates)}});
}

void Find(DataChunk &args, ExpressionState &state, Vector &result) {
	auto &bound = state.expr.Cast<BoundFunctionExpression>().bind_info->Cast<FindBind>();
	auto context = bound.context.lock();
	if (!context) { throw InvalidInputException("thinkthen defect: the caller session ended"); }
	auto owner = context->registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
	vector<std::optional<Request>> requests(args.size());
	for (idx_t row = 0; row < args.size(); ++row) {
		bool absent = false;
		for (idx_t column = 0; column < args.ColumnCount(); ++column) {
			absent |= args.data[column].GetValue(row).IsNull();
		}
		if (absent) { continue; }
		auto request = Read(args, row);
		if (request.units.empty()) { continue; }
		auto views = Views(request.units);
		RustReply checked(thinkthen_cpp_validate_find(reinterpret_cast<const uint8_t *>(request.question.data()),
		                                            request.question.size(), views.data(), views.size(), int32_t(request.none)));
		Checked(checked.value);
		requests[row] = std::move(request);
	}
	const auto settings = Settings(*context);
	for (idx_t row = 0; row < args.size(); ++row) {
		if (!requests[row]) {
			result.SetValue(row, Value(FindType()));
			continue;
		}
		const auto &request = *requests[row];
		const auto budget = owner->Remaining(*context);
		const auto due = budget < 0 ? request.deadline : request.deadline < 0 ? budget : std::min(budget, request.deadline);
		auto views = Views(request.units);
		RustReply reply(thinkthen_cpp_find(reinterpret_cast<const uint8_t *>(request.question.data()),
		                                request.question.size(), views.data(), views.size(), int32_t(request.none),
		                                due, settings.Bridge(), StopFor(*context)));
		Checked(reply.value);
		result.SetValue(row, Decode(reply.value, request.units, request.none));
	}
}

} // namespace

void RegisterFind(ExtensionLoader &loader) {
	const auto result = FindType();
	for (auto parameters : {vector<LogicalType>{LogicalType::VARCHAR, LogicalType::LIST(LogicalType::VARCHAR)},
	                        vector<LogicalType>{LogicalType::VARCHAR, LogicalType::LIST(LogicalType::VARCHAR), LogicalType::BOOLEAN},
	                        vector<LogicalType>{LogicalType::VARCHAR, LogicalType::LIST(LogicalType::VARCHAR), LogicalType::BOOLEAN, LogicalType::BIGINT}}) {
		ScalarFunction function("thinkthen_find", parameters, result, Find, BindFind);
		function.SetStability(FunctionStability::VOLATILE);
		loader.RegisterFunction(function);
	}
}

} // namespace duckdb
