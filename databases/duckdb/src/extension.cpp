#include "duckdb.hpp"
#include "duckdb/parser/parsed_data/create_table_function_info.hpp"
#include "duckdb/common/string_util.hpp"
#include "duckdb/common/types/column/column_data_collection.hpp"
#include "duckdb/main/client_context.hpp"
#include "duckdb/main/client_context_state.hpp"
#include "duckdb/main/extension/extension_loader.hpp"
#include "duckdb/planner/operator/logical_extension_operator.hpp"
#include "duckdb/planner/operator/logical_get.hpp"
#include "duckdb/execution/physical_plan_generator.hpp"
#include "duckdb/parallel/meta_pipeline.hpp"
#include "duckdb/parallel/pipeline.hpp"
#include "thinkthen_sql.h"
#include "yyjson.hpp"
#include <cstdlib>
#include <memory>
#include <mutex>
#include <cmath>
#include <map>

namespace {
using namespace duckdb;
using namespace duckdb_yyjson;
using ClientOwner = std::shared_ptr<thinkthen_client>;
using CallOwner = std::unique_ptr<thinkthen_sql_call, decltype(&thinkthen_sql_call_free)>;
struct RowOwner {
    thinkthen_row value{};
    RowOwner() { value.struct_size = sizeof(value); }
    ~RowOwner() { thinkthen_row_free(&value); }
};
struct ErrorOwner {
    thinkthen_error value{};
    ErrorOwner() { value.struct_size = sizeof(value); }
    ~ErrorOwner() { thinkthen_error_free(&value); }
};
using Document = std::unique_ptr<yyjson_doc, decltype(&yyjson_doc_free)>;

const char *Bytes(const string &value) { return value.data(); }
string Text(const thinkthen_buffer &value) { return string(reinterpret_cast<const char *>(value.data), value.len); }
#include "values.hpp"
Value EngineCode(int32_t number) {
    auto code = thinkthen_code_name(number);
    if (!code) throw InvalidInputException("usage.abi: unknown engine error code");
    try { return Value(code).DefaultCastAs(CodeType()); }
    catch (const Exception &) { throw InvalidInputException("usage.abi: unknown engine error code"); }
}
void Checked(int32_t result, ErrorOwner &error) {
    if (result >= 0) return;
    auto code = EngineCode(error.value.code).ToString();
    throw InvalidInputException("%s: %s", code.c_str(), Text(error.value.message).c_str());
}
ClientOwner Client(const string &settings) {
    thinkthen_client *client = nullptr;
    ErrorOwner error;
    Checked(thinkthen_sql_client_open(Bytes(settings), settings.size(), &client, &error.value), error);
    return ClientOwner(client, thinkthen_client_free);
}
struct Session : ClientContextState {
    std::mutex mutex;
    std::map<string, string> settings;
    ClientOwner client;
    ClientOwner Current() {
        std::lock_guard<std::mutex> lock(mutex);
        if (!client) client = Client("{}");
        return client;
    }
    void Configure(const string &key, const Value &value) {
        std::lock_guard<std::mutex> lock(mutex);
        auto replacement = settings;
        if (value.IsNull()) replacement.erase(key);
        else replacement[key] = Encode(value);
        string json = "{";
        string retry = "{";
        for (auto &item : replacement) {
            auto &target = tt_setting(item.first.c_str()) != 4 ? json : retry;
            if (target.size() > 1) target += ",";
            target += Quote(item.first) + ":" + item.second;
        }
        if (retry.size() > 1) { if (json.size() > 1) json += ","; json += "\"retry\":" + retry + "}"; }
        auto admitted = Client(json + "}");
        settings = std::move(replacement); client = std::move(admitted);
    }
};
template <int Option> void Configure(ClientContext &context, SetScope scope, Value &value) {
    if (scope == SetScope::GLOBAL) throw InvalidInputException("ThinkThen settings are session-only");
    context.registered_state->GetOrCreate<Session>("thinkthen")->Configure(SettingKey(Option), value);
}
using QuestionOwner = std::shared_ptr<thinkthen_question>;
struct PipelineCount { idx_t value = 1; };
struct Binding : FunctionData {
    string function, identity, name;
    bool json, has_name = false, failure_rows = false;
    idx_t record;
    std::shared_ptr<PipelineCount> pipelines = std::make_shared<PipelineCount>();
    vector<LogicalType> types;
    QuestionOwner question;
    unique_ptr<FunctionData> Copy() const override { return make_uniq<Binding>(*this); }
    bool Equals(const FunctionData &other) const override {
        auto &value = other.Cast<Binding>();
        return identity == value.identity && json == value.json && record == value.record && types == value.types;
    }
};
struct RetainedChunk {
    DataChunk source;
    unique_ptr<ColumnDataCollection> spilled;
    idx_t remaining = 0;
};
struct LocalInvocation : LocalTableFunctionState {
    bool finalized = false;
};
struct Invocation : GlobalTableFunctionState {
    idx_t remaining_pipelines = 0;
    CallOwner call{nullptr, thinkthen_sql_call_free};
    bool submitted = false, ended = false;
    uint64_t position = 0;
    uint64_t answered = 0, unsure = 0, failed = 0, backend_failed = 0;
    string first_code, first_message;
    std::map<uint64_t, RetainedChunk> retained;
    ColumnDataScanState ranked_scan;
    DataChunk ranked_chunk;
    uint64_t ranked_chunk_end = 0;
    idx_t ranked_chunk_index = DConstants::INVALID_INDEX;
};
unique_ptr<GlobalTableFunctionState> Init(ClientContext &context, TableFunctionInitInput &input) {
    auto &bind = input.bind_data->Cast<Binding>();
    auto client = context.registered_state->GetOrCreate<Session>("thinkthen")->Current();
    auto state = make_uniq<Invocation>();
    state->remaining_pipelines = bind.pipelines->value;
    thinkthen_sql_call *call = nullptr;
    ErrorOwner error;
    thinkthen_text name{Bytes(bind.name), bind.name.size()};
    Checked(thinkthen_sql_call_open_question(client.get(), bind.question.get(), bind.has_name ? &name : nullptr, &call, &error.value), error);
    state->call.reset(call);
    return state;
}
uint8_t Interrupted(void *context) { return static_cast<ClientContext *>(context)->IsInterrupted() ? 1 : 0; }
void CheckInterrupt(ClientContext &context, Invocation &state) {
    if (context.IsInterrupted()) { state.call.reset(); throw InterruptException(); }
}
void Emit(DataChunk &output, idx_t index, const thinkthen_row &row, Invocation &state, const Binding &bind) {
    auto original = state.retained.lower_bound(row.position);
    if (original == state.retained.end()) throw InvalidInputException("usage.abi: unknown row position");
    auto &retained = original->second;
    auto size = retained.spilled ? retained.spilled->Count() : retained.source.size();
    if (row.position <= original->first - size) throw InvalidInputException("usage.abi: unknown row position");
    auto offset = row.position - (original->first - size) - 1;
    auto *source = &retained.source;
    if (retained.spilled) {
        // Each input chunk has its own spill collection, so an arbitrary ranked
        // position never scans the whole table. Keep only the selected chunk's
        // pages pinned; FetchChunk would duplicate its entire string heap.
        auto chunk_index = offset / STANDARD_VECTOR_SIZE;
        if (state.ranked_chunk_end != original->first || state.ranked_chunk_index > chunk_index) {
            state.ranked_scan = ColumnDataScanState{};
            retained.spilled->InitializeScan(state.ranked_scan, ColumnDataScanProperties::ALLOW_ZERO_COPY);
            state.ranked_chunk_end = original->first;
            state.ranked_chunk_index = DConstants::INVALID_INDEX;
        }
        while (state.ranked_chunk_index == DConstants::INVALID_INDEX || state.ranked_chunk_index < chunk_index) {
            if (!retained.spilled->Scan(state.ranked_scan, state.ranked_chunk))
                throw InvalidInputException("usage.abi: unknown row position");
            state.ranked_chunk_index = state.ranked_chunk_index == DConstants::INVALID_INDEX ? 0 : state.ranked_chunk_index + 1;
        }
        source = &state.ranked_chunk;
        offset %= STANDARD_VECTOR_SIZE;
    }
    idx_t col = 0;
    for (; col < source->ColumnCount(); ++col) output.data[col].Append(source->GetValue(col, offset));
    if (--retained.remaining == 0) state.retained.erase(original);
    auto put = [&](Value value) { output.data[col++].Append(value); };
    put(Value::BIGINT(static_cast<int64_t>(row.position)));
    put(Project(LogicalType::JSON(), row.id));
    put(bind.function == "annotate" ? AnnotationAnswer(bind.types[col], row.answer) : Project(bind.types[col], row.answer));
    put(std::isnan(row.probability) ? Value(LogicalTypeId::DOUBLE) : Value::DOUBLE(row.probability));
    const char *status = row.status == THINKTHEN_STATUS_ANSWERED ? "answered" : row.status == THINKTHEN_STATUS_UNSURE ? "unsure" : row.status == THINKTHEN_STATUS_FAILED ? "failed" : nullptr;
    if (!status) throw InvalidInputException("usage.abi: unknown engine status");
    put(Value(status).DefaultCastAs(StatusType()));
    if (row.error_code) put(EngineCode(row.error_code));
    else put(Value(CodeType()));
    auto error = row.error.data ? Parse(Text(row.error)) : Document(nullptr, yyjson_doc_free);
    auto root = error ? yyjson_doc_get_root(error.get()) : nullptr;
    put(Project(LogicalTypeId::VARCHAR, root ? yyjson_obj_get(root, "message") : nullptr));
    put(Project(LogicalTypeId::BIGINT, root ? yyjson_obj_get(root, "http_status") : nullptr));
    put(Project(SourceType(), row.source));
    put(Project(bind.types[col], bind.function == "annotate" ? row.answer : row.detail));
}
bool Select(const thinkthen_row &row, Invocation &state, const Binding &bind) {
    if (row.status == THINKTHEN_STATUS_ANSWERED) ++state.answered;
    else if (row.status == THINKTHEN_STATUS_UNSURE) ++state.unsure;
    else if (row.status == THINKTHEN_STATUS_FAILED) {
        if (++state.failed == 1) {
            state.first_code = EngineCode(row.error_code).ToString();
            auto error = Parse(Text(row.error));
            auto message = yyjson_obj_get(yyjson_doc_get_root(error.get()), "message");
            state.first_message = string(yyjson_get_str(message), yyjson_get_len(message));
        }
        const char *family = thinkthen_code_family(row.error_code);
        if (family && string(family) == "backend") ++state.backend_failed;
    }
    bool selected = bind.function != "filter" ||
        (row.status == THINKTHEN_STATUS_ANSWERED && row.answer_bool) ||
        (row.status == THINKTHEN_STATUS_FAILED && bind.failure_rows);
    if (!selected) {
        auto original = state.retained.lower_bound(row.position);
        if (original == state.retained.end()) throw InvalidInputException("usage.abi: unknown row position");
        if (--original->second.remaining == 0) state.retained.erase(original);
    }
    return selected;
}
void Complete(const Invocation &state, const Binding &bind) {
    if (state.failed && !state.answered && !state.unsure && state.backend_failed == state.failed)
        throw InvalidInputException("%s: %s", state.first_code, state.first_message);
    if (bind.function == "filter" && state.failed && !bind.failure_rows)
        throw InvalidInputException("%s: %llu records failed; %s", state.first_code,
                                    static_cast<unsigned long long>(state.failed), state.first_message);
}
int32_t Poll(ClientContext &context, Invocation &state, RowOwner &row, uint32_t timeout) {
    ErrorOwner error;
    auto result = thinkthen_sql_call_poll(state.call.get(), timeout, Interrupted, &context, &row.value, &error.value);
    CheckInterrupt(context, state);
    Checked(result, error);
    return result;
}
OperatorResultType Run(ExecutionContext &context, TableFunctionInput &data, DataChunk &input, DataChunk &output) {
    auto &state = data.global_state->Cast<Invocation>();
    auto &bind = data.bind_data->Cast<Binding>();
    idx_t count = 0;
    while (count < STANDARD_VECTOR_SIZE) {
        RowOwner row;
        auto result = Poll(context.client, state, row, count ? 0 : 20);
        if (result == THINKTHEN_OK) { if (Select(row.value, state, bind)) Emit(output, count++, row.value, state, bind); }
        else if (result == THINKTHEN_PENDING && count) {
            output.CheckCardinality(count); return OperatorResultType::HAVE_MORE_OUTPUT;
        }
        else if (result == THINKTHEN_NEEDS_INPUT) {
            if (state.submitted) { state.submitted = false; output.CheckCardinality(count); return OperatorResultType::NEED_MORE_INPUT; }
            vector<string> values;
            values.reserve(input.size());
            vector<thinkthen_sql_input> records;
            records.reserve(input.size());
            state.position += input.size();
            auto &retained = state.retained[state.position];
            retained.remaining = input.size();
            if (bind.function == "rank") {
                retained.spilled = make_uniq<ColumnDataCollection>(context.client, input.GetTypes());
                if (!state.ranked_chunk.ColumnCount()) retained.spilled->InitializeScanChunk(state.ranked_chunk);
                retained.spilled->Append(input);
            } else {
                retained.source.Initialize(context.client, input.GetTypes(), input.size());
                input.Copy(retained.source);
            }
            for (idx_t i = 0; i < input.size(); ++i) {
                CheckInterrupt(context.client, state);
                auto value = input.GetValue(bind.record, i);
                if (value.IsNull()) { records.push_back({THINKTHEN_RECORD_MISSING, nullptr, 0}); continue; }
                values.push_back(value.GetValue<string>());
                records.push_back({static_cast<uint32_t>(bind.json ? THINKTHEN_RECORD_JSON : THINKTHEN_RECORD_TEXT),
                                   Bytes(values.back()), values.back().size()});
            }
            ErrorOwner error;
            Checked(thinkthen_sql_call_send_chunk(state.call.get(), records.data(), records.size(), &error.value), error);
            state.submitted = true;
        } else if (result == THINKTHEN_END) {
            if (count) { output.CheckCardinality(count); return OperatorResultType::HAVE_MORE_OUTPUT; }
            Complete(state, bind);
            output.CheckCardinality(count); return OperatorResultType::FINISHED;
        }
    }
    output.CheckCardinality(count);
    return OperatorResultType::HAVE_MORE_OUTPUT;
}
OperatorFinalizeResultType Final(ExecutionContext &context, TableFunctionInput &data, DataChunk &output) {
    auto &state = data.global_state->Cast<Invocation>();
    auto &bind = data.bind_data->Cast<Binding>();
    auto &local = data.local_state->Cast<LocalInvocation>();
    if (!local.finalized) {
        local.finalized = true;
        --state.remaining_pipelines;
    }
    if (state.remaining_pipelines) return OperatorFinalizeResultType::FINISHED;
    if (!state.ended) { thinkthen_sql_call_end_input(state.call.get()); state.ended = true; }
    idx_t count = 0;
    while (count < STANDARD_VECTOR_SIZE) {
        RowOwner row;
        auto result = Poll(context.client, state, row, count ? 0 : 20);
        if (result == THINKTHEN_OK) { if (Select(row.value, state, bind)) Emit(output, count++, row.value, state, bind); }
        else if (result == THINKTHEN_PENDING && count) {
            output.CheckCardinality(count); return OperatorFinalizeResultType::HAVE_MORE_OUTPUT;
        }
        else if (result == THINKTHEN_END) {
            if (count) { output.CheckCardinality(count); return OperatorFinalizeResultType::HAVE_MORE_OUTPUT; }
            Complete(state, bind);
            output.CheckCardinality(count); return OperatorFinalizeResultType::FINISHED;
        }
    }
    output.CheckCardinality(count);
    return OperatorFinalizeResultType::HAVE_MORE_OUTPUT;
}
struct Sequential : PhysicalOperator {
    std::shared_ptr<PipelineCount> pipelines;
    Sequential(PhysicalPlan &plan, vector<LogicalType> types, std::shared_ptr<PipelineCount> pipelines)
        : PhysicalOperator(plan, PhysicalOperatorType::EXTENSION, std::move(types), 1), pipelines(std::move(pipelines)) {}
    void BuildPipelines(Pipeline &current, MetaPipeline &meta) override {
        PhysicalOperator::BuildPipelines(current, meta);
        // UNION and outer joins may create several input pipelines. Count the
        // actual consumers, rather than guessing from the logical query tree.
        vector<shared_ptr<Pipeline>> consumers;
        meta.GetPipelines(consumers, false);
        pipelines->value = 0;
        for (auto &pipeline : consumers) {
            for (auto &op : meta.GetState().GetPipelineOperators(*pipeline)) {
                if (&op.get() == this) ++pipelines->value;
            }
        }
    }
    OperatorResultType Execute(ExecutionContext &, DataChunk &input, DataChunk &output, GlobalOperatorState &, OperatorState &) const override {
        output.Reference(input); return OperatorResultType::NEED_MORE_INPUT;
    }
    bool ParallelOperator() const override { return false; }
    OrderPreservationType OperatorOrder() const override { return OrderPreservationType::FIXED_ORDER; }
};
struct LogicalSequential : LogicalExtensionOperator {
    std::shared_ptr<PipelineCount> pipelines;
    explicit LogicalSequential(unique_ptr<LogicalOperator> child)
        : pipelines(child->Cast<LogicalGet>().bind_data->Cast<Binding>().pipelines) { children.push_back(std::move(child)); }
    vector<ColumnBinding> GetColumnBindings() override { return children[0]->GetColumnBindings(); }
    void ResolveTypes() override { types = children[0]->types; }
    string GetExtensionName() const override { return "thinkthen"; }
    bool SupportSerialization() const override { return false; }
    PhysicalOperator &CreatePlan(ClientContext &, PhysicalPlanGenerator &planner) override {
        auto &child = planner.CreatePlan(*children[0]);
        auto &op = planner.Make<Sequential>(types, pipelines); op.children.push_back(child); return op;
    }
};
#include "binding.hpp"
} // namespace
extern "C" {
DUCKDB_CPP_EXTENSION_ENTRY(thinkthen, loader) { Load(loader); }
}
