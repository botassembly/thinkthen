#include "listed_complete.hpp"
#include "bridge.hpp"
#include "listed_result.hpp"
#include "scalar_owner.hpp"
#include "scalar_settings.hpp"

#include <map>
#include <optional>
#include <string>
#include <utility>
#include <vector>

namespace duckdb {
namespace {

struct Group {
	ResolvedQuestion question;
	vector<string> texts;
	std::map<string, idx_t> seen;
};

LogicalType ResultType(int32_t kind) {
	return kind == 5 ? LogicalType::DOUBLE : kind == 6 ? LogicalType::LIST(LogicalType::VARCHAR)
	                                                       : LogicalType::VARCHAR;
}

} // namespace

void ValidateCompleteListed(const ResolvedQuestion &question, int32_t kind) {
	RustReply checked(thinkthen_cpp_validate_complete_listed(reinterpret_cast<const uint8_t *>(question.text.data()),
	                                                       question.text.size(), kind, question.from_file ? 1 : 0));
	Checked(checked.value);
}

void CompleteListed(DataChunk &args, ClientContext &context, int32_t kind, Vector &result) {
	auto owner = context.registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
	vector<Group> groups;
	std::map<string, idx_t> known;
	vector<std::optional<std::pair<idx_t, idx_t>>> slots(args.size());
	// Complete every local file and grammar check before the first send.
	for (idx_t row = 0; row < args.size(); ++row) {
		auto question = args.data[0].GetValue(row);
		auto evidence = args.data[1].GetValue(row);
		if (question.IsNull() || evidence.IsNull()) {
			continue;
		}
		auto argument = question.GetValue<string>();
		auto [place, fresh] = known.emplace(argument, groups.size());
		if (fresh) {
			auto resolved = owner->Resolve(context, argument);
			ValidateCompleteListed(resolved, kind);
			groups.push_back({std::move(resolved), {}, {}});
		}
		auto &group = groups[place->second];
		auto [position, first] = group.seen.emplace(evidence.GetValue<string>(), group.texts.size());
		if (first) {
			group.texts.push_back(position->first);
		}
		slots[row] = std::make_pair(place->second, position->second);
	}
	const auto settings = Settings(context);
	vector<vector<Value>> answered;
	for (auto &group : groups) {
		vector<ThinkThenText> texts;
		texts.reserve(group.texts.size());
		for (auto &text : group.texts) {
			texts.push_back({reinterpret_cast<const uint8_t *>(text.data()), text.size()});
		}
		const auto budget = owner->Remaining(context);
		RustReply reply(thinkthen_cpp_complete_listed_group(
		    reinterpret_cast<const uint8_t *>(group.question.text.data()), group.question.text.size(), texts.data(),
		    texts.size(), budget, kind, settings.Bridge(), group.question.from_file ? 1 : 0, StopFor(context)));
		Checked(reply.value);
		answered.push_back(DecodeListed(reply.value.bytes, reply.value.len, texts.size(), kind));
	}
	for (idx_t row = 0; row < args.size(); ++row) {
		if (!slots[row]) {
			result.SetValue(row, Value(ResultType(kind)));
			continue;
		}
		auto [group, text] = *slots[row];
		result.SetValue(row, answered.at(group).at(text));
	}
}

} // namespace duckdb
