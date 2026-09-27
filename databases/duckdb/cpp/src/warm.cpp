#include "warm.hpp"
#include "bridge.hpp"
#include "scalar_owner.hpp"
#include "duckdb/common/weak_ptr_ipp.hpp"
#include "duckdb/function/aggregate_function.hpp"

#include <cstring>
#include <limits>
#include <new>
#include <optional>
#include <set>

namespace duckdb {
namespace {

struct WarmData {
	std::optional<string> question;
	std::set<string> texts;

	void Bind(const string &value) {
		if (question && *question != value) {
			throw InvalidInputException("thinkthen usage: thinkthen_warm judges one question per group, and this group carries more than one");
		}
		question = value;
	}
};

struct WarmState {
	WarmData *data;
};

struct WarmBind : FunctionData {
	weak_ptr<ClientContext> context;
	explicit WarmBind(weak_ptr<ClientContext> context) : context(std::move(context)) {
	}
	unique_ptr<FunctionData> Copy() const override {
		return make_uniq<WarmBind>(context);
	}
	bool Equals(const FunctionData &other) const override {
		return context.lock() == other.Cast<WarmBind>().context.lock();
	}
};

unique_ptr<FunctionData> BindWarm(ClientContext &context, AggregateFunction &,
                                   vector<unique_ptr<Expression>> &) {
	context.registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
	return make_uniq<WarmBind>(context.shared_from_this());
}

idx_t WarmSize(const AggregateFunction &) {
	return sizeof(WarmState);
}

void WarmInit(const AggregateFunction &, data_ptr_t state) {
	new (state) WarmState {new WarmData};
}

template <class Action> void Each(Vector &states, idx_t count, Action action) {
	UnifiedVectorFormat format;
	states.ToUnifiedFormat(count, format);
	auto pointers = UnifiedVectorFormat::GetData<WarmState *>(format);
	for (idx_t row = 0; row < count; ++row) {
		action(*pointers[format.sel->get_index(row)], row);
	}
}

void WarmUpdate(Vector inputs[], AggregateInputData &, idx_t input_count, Vector &states, idx_t count) {
	if (input_count != 2) {
		throw InvalidInputException("thinkthen defect: warm received another input shape");
	}
	Each(states, count, [&](WarmState &state, idx_t row) {
		auto question = inputs[0].GetValue(row);
		auto evidence = inputs[1].GetValue(row);
		if (!question.IsNull() && !evidence.IsNull()) {
			state.data->Bind(question.GetValue<string>());
			state.data->texts.insert(evidence.GetValue<string>());
		}
	});
}

void WarmCombine(Vector &sources, Vector &targets, AggregateInputData &, idx_t count) {
	UnifiedVectorFormat source_format, target_format;
	sources.ToUnifiedFormat(count, source_format);
	targets.ToUnifiedFormat(count, target_format);
	auto from = UnifiedVectorFormat::GetData<WarmState *>(source_format);
	auto to = UnifiedVectorFormat::GetData<WarmState *>(target_format);
	for (idx_t row = 0; row < count; ++row) {
		auto &source = *from[source_format.sel->get_index(row)]->data;
		auto &target = *to[target_format.sel->get_index(row)]->data;
		if (source.question) {
			target.Bind(*source.question);
		}
		target.texts.insert(source.texts.begin(), source.texts.end());
	}
}

int64_t Finish(WarmData &data, ClientContext &context) {
	if (!data.question || data.texts.empty()) {
		return 0;
	}
	if (data.question->rfind("@~", 0) == 0) {
		throw InvalidInputException("thinkthen usage: thinkthen_warm cannot read an '@~' path, because home_directory is a session setting it cannot see; write the full path");
	}
	const auto resolved = ResolveQuestion(context, *data.question);
	vector<string> texts(data.texts.begin(), data.texts.end());
	vector<ThinkThenText> copied;
	for (auto &text : texts) {
		copied.push_back({reinterpret_cast<const uint8_t *>(text.data()), text.size()});
	}
	RustReply reply(thinkthen_cpp_warm(reinterpret_cast<const uint8_t *>(resolved.text.data()), resolved.text.size(),
	                                  copied.data(), copied.size(), resolved.from_file ? 1 : 0));
	Checked(reply.value);
	if (!reply.value.bytes || reply.value.len != sizeof(int64_t)) {
		throw InvalidInputException("thinkthen defect: the bridge returned an invalid warm count");
	}
	int64_t count;
	std::memcpy(&count, reply.value.bytes, sizeof(count));
	return count;
}

void WarmFinalize(Vector &states, AggregateInputData &input, Vector &result, idx_t count, idx_t offset) {
	auto context = input.bind_data->Cast<WarmBind>().context.lock();
	if (!context) {
		throw InvalidInputException("thinkthen defect: the warm caller session ended");
	}
	Each(states, count, [&](WarmState &state, idx_t row) {
		result.SetValue(offset + row, Value::BIGINT(Finish(*state.data, *context)));
	});
}

void WarmDestroy(Vector &states, AggregateInputData &, idx_t count) {
	Each(states, count, [](WarmState &state, idx_t) {
		delete state.data;
		state.data = nullptr;
	});
}

} // namespace

void RegisterWarm(ExtensionLoader &loader) {
	AggregateFunction function("thinkthen_warm", {LogicalType::VARCHAR, LogicalType::VARCHAR},
	                           LogicalType::BIGINT, WarmSize, WarmInit, WarmUpdate, WarmCombine,
	                           WarmFinalize, FunctionNullHandling::SPECIAL_HANDLING, nullptr,
	                           BindWarm, WarmDestroy);
	loader.RegisterFunction(function);
}

} // namespace duckdb
