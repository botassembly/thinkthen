#include "warm.hpp"
#include "bridge.hpp"
#include "relate_query.hpp"
#include "scalar_owner.hpp"
#include "scalar_settings.hpp"
#include "duckdb/common/weak_ptr_ipp.hpp"
#include "duckdb/function/aggregate_function.hpp"
#include "duckdb/function/function_set.hpp"

#include <cstring>
#include <limits>
#include <new>
#include <optional>
#include <map>
#include <set>

namespace duckdb {
namespace {

struct WarmData {
	std::optional<string> question;
	struct Group {
		std::optional<string> context;
		vector<string> texts;
		std::set<string> seen;
	};
	vector<Group> groups;
	std::map<std::optional<string>, idx_t> known;

	void Bind(const string &value) {
		if (question && *question != value) {
			throw InvalidInputException("thinkthen usage: thinkthen_warm judges one question per group, and this group carries more than one");
		}
		question = value;
	}
	void Add(const std::optional<string> &context, const string &text) {
		auto [place, fresh] = known.emplace(context, groups.size());
		if (fresh) {
			groups.push_back({context, {}, {}});
		}
		auto &group = groups[place->second];
		if (group.seen.insert(text).second) {
			group.texts.push_back(text);
		}
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
	if (input_count != 2 && input_count != 3) {
		throw InvalidInputException("thinkthen defect: warm received another input shape");
	}
	Each(states, count, [&](WarmState &state, idx_t row) {
		auto question = inputs[0].GetValue(row);
		auto evidence = inputs[1].GetValue(row);
		if (!question.IsNull() && !evidence.IsNull()) {
			state.data->Bind(question.GetValue<string>());
			std::optional<string> context;
			if (input_count == 3) {
				auto value = inputs[2].GetValue(row);
				if (!value.IsNull()) {
					context = value.GetValue<string>();
					if (context->find_first_not_of(" \t\r\n\f\v") == string::npos) {
						throw InvalidInputException("thinkthen usage: context is text, not white space");
					}
				}
			}
			state.data->Add(context, evidence.GetValue<string>());
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
		for (auto &group : source.groups) {
			for (auto &text : group.texts) {
				target.Add(group.context, text);
			}
		}
	}
}

int64_t Finish(WarmData &data, ClientContext &context) {
	if (!data.question || data.groups.empty()) {
		return 0;
	}
	if (data.question->rfind("@~", 0) == 0) {
		throw InvalidInputException("thinkthen usage: thinkthen_warm cannot read an '@~' path; write the full path");
	}
	if (!data.question->empty() && data.question->front() == '@' && RelateBusyFor(context)) {
		throw InvalidInputException("thinkthen usage: thinkthen_warm cannot read '@file' while a relate query runs on this database; run it before or after the relate, or pass the file's JSON text");
	}
	const auto resolved = ResolveQuestion(context, *data.question);
	auto settings = Settings(context);
	auto owner = context.registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
	int64_t count = 0;
	for (auto &group : data.groups) {
		vector<ThinkThenText> copied;
		for (auto &text : group.texts) {
			copied.push_back({reinterpret_cast<const uint8_t *>(text.data()), text.size()});
		}
		const auto due = owner->Remaining(context);
		RustReply reply(thinkthen_cpp_warm(reinterpret_cast<const uint8_t *>(resolved.text.data()), resolved.text.size(),
		                                  copied.data(), copied.size(), resolved.from_file ? 1 : 0, due,
		                                  group.context ? reinterpret_cast<const uint8_t *>(group.context->data()) : nullptr,
		                                  group.context ? group.context->size() : 0,
		                                  settings.Bridge(), StopFor(context)));
		Checked(reply.value);
		if (!reply.value.bytes || reply.value.len != sizeof(int64_t)) {
			throw InvalidInputException("thinkthen defect: the bridge returned an invalid warm count");
		}
		int64_t part;
		std::memcpy(&part, reply.value.bytes, sizeof(part));
		if (part > std::numeric_limits<int64_t>::max() - count) {
			throw InvalidInputException("thinkthen defect: the warm count overflowed");
		}
		count += part;
	}
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
	AggregateFunctionSet overloads("thinkthen_warm");
	for (auto parameters : {vector<LogicalType>{LogicalType::VARCHAR, LogicalType::VARCHAR},
	                        vector<LogicalType>{LogicalType::VARCHAR, LogicalType::VARCHAR, LogicalType::VARCHAR}}) {
		AggregateFunction function("thinkthen_warm", parameters, LogicalType::BIGINT, WarmSize, WarmInit,
		                           WarmUpdate, WarmCombine, WarmFinalize, FunctionNullHandling::SPECIAL_HANDLING,
		                           nullptr, BindWarm, WarmDestroy);
		overloads.AddFunction(function);
	}
	loader.RegisterFunction(overloads);
}

} // namespace duckdb
