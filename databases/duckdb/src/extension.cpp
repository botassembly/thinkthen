#include "duckdb.hpp"
#include "duckdb/common/string_util.hpp"
#include "duckdb/main/client_context.hpp"
#include "duckdb/main/client_context_state.hpp"
#include "duckdb/main/extension/extension_loader.hpp"
#include "duckdb/planner/operator/logical_extension_operator.hpp"
#include "duckdb/planner/operator/logical_get.hpp"
#include "duckdb/execution/physical_plan_generator.hpp"
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

const uint8_t *Bytes(const string &value) { return reinterpret_cast<const uint8_t *>(value.data()); }
string Text(const thinkthen_buffer &value) { return string(reinterpret_cast<const char *>(value.data), value.len); }
#include "values.hpp"
void Checked(int32_t result, ErrorOwner &error) {
    if (result != THINKTHEN_ERROR) return;
    auto code = thinkthen_code_name(error.value.code);
    if (!code) throw InvalidInputException("usage.abi: unknown engine error code");
    try { Value(code).DefaultCastAs(CodeType()); }
    catch (const Exception &) { throw InvalidInputException("usage.abi: unknown engine error code"); }
    throw InvalidInputException("%s: %s", code, Text(error.value.message).c_str());
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
struct Binding : FunctionData {
    string function, identity, name;
    bool json, has_name = false;
    idx_t record;
    vector<LogicalType> types;
    QuestionOwner question;
    unique_ptr<FunctionData> Copy() const override { return make_uniq<Binding>(*this); }
    bool Equals(const FunctionData &other) const override {
        auto &value = other.Cast<Binding>();
        return identity == value.identity && json == value.json && record == value.record && types == value.types;
    }
};
struct Invocation : LocalTableFunctionState {
    CallOwner call{nullptr, thinkthen_sql_call_free};
    bool submitted = false, ended = false;
    uint64_t position = 0;
    std::map<uint64_t, vector<Value>> retained;
};
unique_ptr<LocalTableFunctionState> Init(ExecutionContext &context, TableFunctionInitInput &input,
                                       GlobalTableFunctionState *) {
    auto &bind = input.bind_data->Cast<Binding>();
    auto client = context.client.registered_state->GetOrCreate<Session>("thinkthen")->Current();
    auto state = make_uniq<Invocation>();
    thinkthen_sql_call *call = nullptr;
    ErrorOwner error;
    thinkthen_text name{Bytes(bind.name), bind.name.size()};
    Checked(thinkthen_sql_call_open_question(client.get(), bind.question.get(), bind.has_name ? &name : nullptr, &call, &error.value), error);
    state->call.reset(call);
    return state;
}
uint8_t Interrupted(void *context) { return static_cast<ClientContext *>(context)->interrupted ? 1 : 0; }
void CheckInterrupt(ClientContext &context, Invocation &state) {
    if (context.interrupted) { state.call.reset(); throw InterruptException(); }
}
void Emit(DataChunk &output, idx_t index, const thinkthen_row &row, Invocation &state, const Binding &bind) {
    auto original = state.retained.find(row.position);
    if (original == state.retained.end()) throw InvalidInputException("usage.abi: unknown row position");
    idx_t col = 0;
    for (auto &value : original->second) output.SetValue(col++, index, value);
    state.retained.erase(original);
    auto put = [&](Value value) { output.SetValue(col++, index, value); };
    put(Value::UBIGINT(row.position));
    put(Project(LogicalType::JSON(), row.id));
    const char *status = row.status == THINKTHEN_STATUS_ANSWERED ? "answered" : row.status == THINKTHEN_STATUS_UNSURE ? "unsure" : row.status == THINKTHEN_STATUS_FAILED ? "failed" : nullptr;
    if (!status) throw InvalidInputException("usage.abi: unknown engine status");
    put(Value(status).DefaultCastAs(StatusType()));
    put(std::isnan(row.probability) ? Value(LogicalTypeId::DOUBLE) : Value::DOUBLE(row.probability));
    if (row.error_code) {
        auto code = thinkthen_code_name(row.error_code);
        try { put(Value(code).DefaultCastAs(CodeType())); }
        catch (const Exception &) { throw InvalidInputException("usage.abi: unknown engine error code"); }
    } else put(Value(CodeType()));
    auto error = row.error.data ? Parse(Text(row.error)) : Document(nullptr, yyjson_doc_free);
    auto root = error ? yyjson_doc_get_root(error.get()) : nullptr;
    put(Project(LogicalTypeId::VARCHAR, root ? yyjson_obj_get(root, "message") : nullptr));
    put(Project(LogicalTypeId::BIGINT, root ? yyjson_obj_get(root, "http_status") : nullptr));
    put(Project(SourceType(), row.source));
    put(Project(bind.types[col], row.detail));
    put(Project(bind.types[col], row.answer));
    if (bind.function == "filter") put(Value::BOOLEAN(row.status == THINKTHEN_STATUS_ANSWERED && row.answer_bool != 0));
}
int32_t Poll(ClientContext &context, Invocation &state, RowOwner &row) {
    ErrorOwner error;
    auto result = thinkthen_sql_call_poll(state.call.get(), 20, Interrupted, &context, &row.value, &error.value);
    CheckInterrupt(context, state);
    Checked(result, error);
    return result;
}
OperatorResultType Run(ExecutionContext &context, TableFunctionInput &data, DataChunk &input, DataChunk &output) {
    auto &state = data.local_state->Cast<Invocation>();
    auto &bind = data.bind_data->Cast<Binding>();
    idx_t count = 0;
    while (count < STANDARD_VECTOR_SIZE) {
        RowOwner row;
        auto result = Poll(context.client, state, row);
        if (result == THINKTHEN_OK) Emit(output, count++, row.value, state, bind);
        else if (result == THINKTHEN_NEEDS_INPUT) {
            if (state.submitted) { state.submitted = false; output.SetCardinality(count); return OperatorResultType::NEED_MORE_INPUT; }
            vector<string> values;
            values.reserve(input.size());
            vector<thinkthen_sql_input> records;
            records.reserve(input.size());
            for (idx_t i = 0; i < input.size(); ++i) {
                CheckInterrupt(context.client, state);
                vector<Value> retained;
                for (idx_t column = 0; column < input.ColumnCount(); ++column) retained.push_back(input.GetValue(column, i));
                auto value = retained[bind.record];
                state.retained.emplace(++state.position, std::move(retained));
                if (value.IsNull()) { records.push_back({THINKTHEN_RECORD_MISSING, nullptr, 0}); continue; }
                values.push_back(value.GetValue<string>());
                records.push_back({bind.json ? THINKTHEN_RECORD_JSON : THINKTHEN_RECORD_TEXT,
                                   Bytes(values.back()), values.back().size()});
            }
            ErrorOwner error;
            Checked(thinkthen_sql_call_send_chunk(state.call.get(), records.data(), records.size(), &error.value), error);
            state.submitted = true;
        } else if (result == THINKTHEN_END) {
            output.SetCardinality(count); return OperatorResultType::FINISHED;
        }
    }
    output.SetCardinality(count);
    return OperatorResultType::HAVE_MORE_OUTPUT;
}
OperatorFinalizeResultType Final(ExecutionContext &context, TableFunctionInput &data, DataChunk &output) {
    auto &state = data.local_state->Cast<Invocation>();
    auto &bind = data.bind_data->Cast<Binding>();
    if (!state.ended) { thinkthen_sql_call_end_input(state.call.get()); state.ended = true; }
    idx_t count = 0;
    while (count < STANDARD_VECTOR_SIZE) {
        RowOwner row;
        auto result = Poll(context.client, state, row);
        if (result == THINKTHEN_OK) Emit(output, count++, row.value, state, bind);
        else if (result == THINKTHEN_END) {
            output.SetCardinality(count); return OperatorFinalizeResultType::FINISHED;
        }
    }
    output.SetCardinality(count);
    return OperatorFinalizeResultType::HAVE_MORE_OUTPUT;
}
struct Sequential : PhysicalOperator {
    Sequential(PhysicalPlan &plan, vector<LogicalType> types) : PhysicalOperator(plan, PhysicalOperatorType::EXTENSION, std::move(types), 1) {}
    OperatorResultType Execute(ExecutionContext &, DataChunk &input, DataChunk &output, GlobalOperatorState &, OperatorState &) const override {
        output.Reference(input); return OperatorResultType::NEED_MORE_INPUT;
    }
    bool ParallelOperator() const override { return false; }
    OrderPreservationType OperatorOrder() const override { return OrderPreservationType::FIXED_ORDER; }
};
struct LogicalSequential : LogicalExtensionOperator {
    explicit LogicalSequential(unique_ptr<LogicalOperator> child) { children.push_back(std::move(child)); }
    vector<ColumnBinding> GetColumnBindings() override { return children[0]->GetColumnBindings(); }
    void ResolveTypes() override { types = children[0]->types; }
    string GetExtensionName() const override { return "thinkthen"; }
    bool SupportSerialization() const override { return false; }
    PhysicalOperator &CreatePlan(ClientContext &, PhysicalPlanGenerator &planner) override {
        auto &child = planner.CreatePlan(*children[0]);
        auto &op = planner.Make<Sequential>(types); op.children.push_back(child); return op;
    }
};
#include "binding.hpp"
} // namespace
extern "C" {
DUCKDB_CPP_EXTENSION_ENTRY(thinkthen, loader) { Load(loader); }
}
