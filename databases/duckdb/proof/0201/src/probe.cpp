#define DUCKDB_EXTENSION_MAIN

#include "duckdb.hpp"
#include "duckdb/common/exception.hpp"
#include "duckdb/common/weak_ptr_ipp.hpp"
#include "duckdb/function/aggregate_function.hpp"
#include "duckdb/function/aggregate_state.hpp"
#include "duckdb/main/client_context.hpp"
#include "duckdb/main/config.hpp"
#include "duckdb/main/extension/extension_loader.hpp"

#include <cstdint>
#include <cstring>
#include <memory>
#include <string>
#include <vector>

extern "C" {
struct BridgeReply {
	int32_t status;
	uint8_t *bytes;
	size_t len;
};
BridgeReply thinkthen_probe_row(const uint8_t *bytes, size_t len);
void thinkthen_probe_free(uint8_t *bytes, size_t len);
}

namespace duckdb {
namespace {

struct RustReply {
	explicit RustReply(const string &text)
	    : reply(thinkthen_probe_row(reinterpret_cast<const uint8_t *>(text.data()), text.size())) {
	}
	~RustReply() {
		thinkthen_probe_free(reply.bytes, reply.len);
	}
	RustReply(const RustReply &) = delete;
	RustReply &operator=(const RustReply &) = delete;
	BridgeReply reply;
};

string ReadField(const BridgeReply &reply, size_t &offset) {
	if (offset > reply.len || reply.len - offset < 4) {
		throw InvalidInputException("thinkthen probe: truncated Rust field length");
	}
	uint32_t len = 0;
	for (unsigned i = 0; i < 4; ++i) {
		len |= static_cast<uint32_t>(reply.bytes[offset + i]) << (i * 8);
	}
	offset += 4;
	if (len > reply.len - offset) {
		throw InvalidInputException("thinkthen probe: truncated Rust field");
	}
	string value(reinterpret_cast<const char *>(reply.bytes + offset), len);
	offset += len;
	return value;
}

void NestedScalar(DataChunk &args, ExpressionState &, Vector &result) {
	for (idx_t row = 0; row < args.size(); ++row) {
		auto input = args.data[0].GetValue(row);
		if (input.IsNull()) {
			result.SetValue(row, Value(result.GetType()));
			continue;
		}
		RustReply owned(input.GetValue<string>());
		if (owned.reply.status != 0) {
			throw InvalidInputException("thinkthen probe: tagged Rust error %d", owned.reply.status);
		}
		size_t offset = 0;
		auto text = ReadField(owned.reply, offset);
		auto first_tag = ReadField(owned.reply, offset);
		auto second_tag = ReadField(owned.reply, offset);
		if (offset != owned.reply.len) {
			throw InvalidInputException("thinkthen probe: trailing Rust bytes");
		}
		result.SetValue(row, Value::STRUCT({{"text", Value(text)},
		                                    {"tags", Value::LIST(LogicalType::VARCHAR,
		                                                         {Value(first_tag), Value(second_tag)})}}));
	}
}

struct SettingBindData : FunctionData {
	explicit SettingBindData(weak_ptr<ClientContext> context_p) : context(std::move(context_p)) {
	}
	weak_ptr<ClientContext> context;
	unique_ptr<FunctionData> Copy() const override {
		return make_uniq<SettingBindData>(context);
	}
	bool Equals(const FunctionData &other) const override {
		return context.lock() == other.Cast<SettingBindData>().context.lock();
	}
};

struct SettingState {
	bool seen;
	int32_t value;
};

idx_t SettingSize(const AggregateFunction &) {
	return sizeof(SettingState);
}
void SettingInitialize(const AggregateFunction &, data_ptr_t ptr) {
	new (ptr) SettingState{false, 0};
}
unique_ptr<FunctionData> SettingBind(ClientContext &context, AggregateFunction &,
                                      vector<unique_ptr<Expression>> &) {
	return make_uniq<SettingBindData>(context.shared_from_this());
}
void SettingUpdate(Vector[], AggregateInputData &input, idx_t, Vector &states, idx_t count) {
	auto &bind = input.bind_data->Cast<SettingBindData>();
	auto caller = bind.context.lock();
	if (!caller) {
		throw InvalidInputException("thinkthen probe: caller session ended");
	}
	Value setting;
	if (!caller->TryGetCurrentSetting("thinkthen_probe_limit", setting)) {
		throw InvalidInputException("thinkthen probe: caller setting missing");
	}
	const auto value = setting.GetValue<int32_t>();
	UnifiedVectorFormat state_data;
	states.ToUnifiedFormat(count, state_data);
	auto state_ptrs = UnifiedVectorFormat::GetData<SettingState *>(state_data);
	for (idx_t row = 0; row < count; ++row) {
		auto &state = *state_ptrs[state_data.sel->get_index(row)];
		if (!state.seen) {
			state = {true, value};
		}
	}
}
void SettingCombine(Vector &source, Vector &target, AggregateInputData &, idx_t count) {
	UnifiedVectorFormat source_data;
	source.ToUnifiedFormat(count, source_data);
	auto source_ptrs = UnifiedVectorFormat::GetData<SettingState *>(source_data);
	auto target_ptrs = FlatVector::GetData<SettingState *>(target);
	for (idx_t row = 0; row < count; ++row) {
		auto &from = *source_ptrs[source_data.sel->get_index(row)];
		auto &to = *target_ptrs[row];
		if (from.seen) {
			if (to.seen && to.value != from.value) {
				throw InvalidInputException("thinkthen probe: one statement saw two settings");
			}
			to = from;
		}
	}
}
void SettingFinalize(Vector &states, AggregateInputData &, Vector &result, idx_t count, idx_t offset) {
	UnifiedVectorFormat state_data;
	states.ToUnifiedFormat(count, state_data);
	auto state_ptrs = UnifiedVectorFormat::GetData<SettingState *>(state_data);
	for (idx_t row = 0; row < count; ++row) {
		auto &state = *state_ptrs[state_data.sel->get_index(row)];
		result.SetValue(offset + row, state.seen ? Value::INTEGER(state.value) : Value(LogicalType::INTEGER));
	}
}

} // namespace

void LoadProbe(ExtensionLoader &loader) {
	auto &config = DBConfig::GetConfig(loader.GetDatabaseInstance());
	config.AddExtensionOption("thinkthen_probe_limit", "Caller-session setting proof", LogicalType::INTEGER,
	                          Value::INTEGER(0));
	auto nested_type = LogicalType::STRUCT({{"text", LogicalType::VARCHAR},
	                                        {"tags", LogicalType::LIST(LogicalType::VARCHAR)}});
	loader.RegisterFunction(ScalarFunction("thinkthen_probe_nested", {LogicalType::VARCHAR}, nested_type, NestedScalar));
	auto aggregate = AggregateFunction("thinkthen_probe_setting", vector<LogicalType>{LogicalType::INTEGER},
	                                   LogicalType::INTEGER,
	                                   SettingSize, SettingInitialize, SettingUpdate, SettingCombine, SettingFinalize,
	                                   FunctionNullHandling::DEFAULT_NULL_HANDLING);
	aggregate.bind = SettingBind;
	loader.RegisterFunction(aggregate);
}

} // namespace duckdb

extern "C" {
DUCKDB_CPP_EXTENSION_ENTRY(thinkthen_probe, loader) {
	duckdb::LoadProbe(loader);
}
}
