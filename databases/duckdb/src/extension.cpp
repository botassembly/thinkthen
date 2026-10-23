#include "duckdb.hpp"
#include "duckdb/common/string_util.hpp"
#include "duckdb/main/client_context.hpp"
#include "duckdb/main/client_context_state.hpp"
#include "duckdb/main/extension/extension_loader.hpp"
#include "duckdb/planner/operator/logical_extension_operator.hpp"
#include "duckdb/planner/operator/logical_get.hpp"
#include "duckdb/execution/physical_plan_generator.hpp"
#include "thinkthen.h"
#include "yyjson.hpp"
#include <cstdlib>
#include <memory>
#include <mutex>

namespace {
using namespace duckdb;
using namespace duckdb_yyjson;
using ClientOwner = std::shared_ptr<thinkthen_client>;
using CallOwner = std::unique_ptr<thinkthen_sql_call, decltype(&thinkthen_sql_free)>;
struct RowOwner {
    thinkthen_row value{};
    ~RowOwner() { thinkthen_row_free(&value); }
};
struct BufferOwner {
    thinkthen_buffer value{};
    ~BufferOwner() { thinkthen_buffer_free(&value); }
};
using Document = std::unique_ptr<yyjson_doc, decltype(&yyjson_doc_free)>;

const uint8_t *Bytes(const string &value) { return reinterpret_cast<const uint8_t *>(value.data()); }
string Text(const thinkthen_buffer &value) { return string(reinterpret_cast<const char *>(value.data), value.len); }
Document Parse(const thinkthen_buffer &value) {
    Document doc(yyjson_read(reinterpret_cast<const char *>(value.data), value.len, 0), yyjson_doc_free);
    if (!doc) throw InvalidInputException("cannot read native result JSON");
    return doc;
}
void Checked(int32_t result, BufferOwner &error) {
    if (result != THINKTHEN_ERROR) return;
    auto doc = Parse(error.value);
    auto root = yyjson_doc_get_root(doc.get());
    throw InvalidInputException("%s: %s", yyjson_get_str(yyjson_obj_get(root, "code")),
                               yyjson_get_str(yyjson_obj_get(root, "message")));
}
ClientOwner Client(const string &settings) {
    thinkthen_client *client = nullptr;
    BufferOwner error;
    Checked(thinkthen_sql_client_open(Bytes(settings), settings.size(), &client, &error.value), error);
    return ClientOwner(client, thinkthen_client_free);
}
struct Session : ClientContextState {
    std::mutex mutex;
    string settings;
    ClientOwner client;
    ClientOwner Current() {
        std::lock_guard<std::mutex> lock(mutex);
        if (!client) { client = Client("{}"); settings = "{}"; }
        return client;
    }
    void Configure(string value) {
        std::lock_guard<std::mutex> lock(mutex);
        if (client && settings == value) return;
        auto replacement = Client(value);
        settings = std::move(value);
        client = std::move(replacement);
    }
};
void Configure(ClientContext &context, SetScope scope, Value &value) {
    if (scope == SetScope::GLOBAL) throw InvalidInputException("thinkthen_settings is session-only");
    if (value.IsNull()) throw InvalidInputException("thinkthen_settings must be client-settings JSON");
    auto session = context.registered_state->GetOrCreate<Session>("thinkthen");
    session->Configure(value.GetValue<string>());
}
struct Binding : FunctionData {
    string function, question, options;
    bool json;
    Binding(string function_p, string question_p, string options_p, bool json_p)
        : function(std::move(function_p)), question(std::move(question_p)), options(std::move(options_p)), json(json_p) {}
    unique_ptr<FunctionData> Copy() const override { return make_uniq<Binding>(function, question, options, json); }
    bool Equals(const FunctionData &other) const override {
        auto &value = other.Cast<Binding>();
        return function == value.function && question == value.question && options == value.options && json == value.json;
    }
};
struct Invocation : LocalTableFunctionState {
    CallOwner call{nullptr, thinkthen_sql_free};
    bool submitted = false, ended = false;
};
unique_ptr<LocalTableFunctionState> Init(ExecutionContext &context, TableFunctionInitInput &input,
                                       GlobalTableFunctionState *) {
    auto &bind = input.bind_data->Cast<Binding>();
    auto client = context.client.registered_state->GetOrCreate<Session>("thinkthen")->Current();
    auto state = make_uniq<Invocation>();
    thinkthen_sql_call *call = nullptr;
    BufferOwner error;
    Checked(thinkthen_sql_open(client.get(), Bytes(bind.function), bind.function.size(), Bytes(bind.question),
                              bind.question.size(), Bytes(bind.options), bind.options.size(), &call, &error.value), error);
    state->call.reset(call);
    return state;
}
uint8_t Interrupted(void *context) { return static_cast<ClientContext *>(context)->interrupted ? 1 : 0; }
void CheckInterrupt(ClientContext &context, Invocation &state) {
    if (context.interrupted) { state.call.reset(); throw InterruptException(); }
}
Value Json(yyjson_val *value) {
    if (!value) return Value(LogicalType::JSON());
    std::unique_ptr<char, decltype(&std::free)> bytes(yyjson_val_write(value, 0, nullptr), std::free);
    if (!bytes) throw std::bad_alloc();
    return Value(string(bytes.get())).DefaultCastAs(LogicalType::JSON());
}
void Emit(DataChunk &output, idx_t index, const thinkthen_row &row) {
    auto doc = Parse(row.fields);
    auto root = yyjson_doc_get_root(doc.get());
    output.SetValue(0, index, Value::UBIGINT(row.position));
    output.SetValue(1, index, Value(yyjson_get_str(yyjson_obj_get(root, "status"))));
    output.SetValue(2, index, Json(yyjson_obj_get(root, "answer")));
    auto probability = yyjson_obj_get(root, "p");
    output.SetValue(3, index, probability ? Value::DOUBLE(yyjson_get_num(probability)) : Value(LogicalType::DOUBLE));
    output.SetValue(4, index, Json(yyjson_obj_get(root, "error")));
    output.SetValue(5, index, row.detail.data ? Value(Text(row.detail)).DefaultCastAs(LogicalType::JSON()) : Value(LogicalType::JSON()));
    output.SetValue(6, index, Value::BOOLEAN(row.kept != 0));
}
int32_t Poll(ClientContext &context, Invocation &state, RowOwner &row) {
    BufferOwner error;
    auto result = thinkthen_sql_poll(state.call.get(), 20, Interrupted, &context, &row.value, &error.value);
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
        if (result == THINKTHEN_OK) Emit(output, count++, row.value);
        else if (result == THINKTHEN_NEEDS_INPUT) {
            if (state.submitted) { state.submitted = false; output.SetCardinality(count); return OperatorResultType::NEED_MORE_INPUT; }
            vector<string> values;
            values.reserve(input.size());
            vector<thinkthen_sql_record> records;
            records.reserve(input.size());
            for (idx_t i = 0; i < input.size(); ++i) {
                CheckInterrupt(context.client, state);
                auto value = input.GetValue(0, i);
                if (value.IsNull()) throw InvalidInputException("source record cannot be SQL NULL");
                values.push_back(value.GetValue<string>());
                records.push_back({bind.json ? THINKTHEN_SQL_JSON : THINKTHEN_SQL_TEXT,
                                   Bytes(values.back()), values.back().size()});
            }
            BufferOwner error;
            Checked(thinkthen_sql_send_chunk(state.call.get(), records.data(), records.size(), &error.value), error);
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
    if (!state.ended) { thinkthen_sql_end_input(state.call.get()); state.ended = true; }
    idx_t count = 0;
    while (count < STANDARD_VECTOR_SIZE) {
        RowOwner row;
        auto result = Poll(context.client, state, row);
        if (result == THINKTHEN_OK) Emit(output, count++, row.value);
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
unique_ptr<LogicalOperator> Bind(ClientContext &, TableFunctionBindInput &input, idx_t index, vector<string> &names) {
    if (input.input_table_types.size() != 1 || !StringUtil::CIEquals(input.input_table_names[0], "record") ||
        input.input_table_types[0].id() != LogicalTypeId::VARCHAR) {
        throw BinderException("ThinkThen input must have one VARCHAR or JSON column named record");
    }
    if (input.inputs[1].IsNull()) throw BinderException("ThinkThen question cannot be SQL NULL");
    string options = "{}";
    auto option = input.named_parameters.find("options");
    if (option != input.named_parameters.end()) {
        if (option->second.IsNull()) throw BinderException("ThinkThen options cannot be SQL NULL");
        options = option->second.GetValue<string>();
    }
    auto bind = make_uniq<Binding>(input.table_function.name.substr(10), input.inputs[1].GetValue<string>(),
                                 options, input.input_table_types[0] == LogicalType::JSON());
    names = {"position", "status", "answer", "p", "error", "detail", "kept"};
    vector<LogicalType> types{LogicalType::UBIGINT, LogicalType::VARCHAR, LogicalType::JSON(), LogicalType::DOUBLE,
                              LogicalType::JSON(), LogicalType::JSON(), LogicalType::BOOLEAN};
    auto fn = input.table_function; fn.bind_operator = nullptr;
    auto get = make_uniq<LogicalGet>(index, std::move(fn), std::move(bind), types, names);
    get->input_table_types = input.input_table_types; get->input_table_names = input.input_table_names;
    for (idx_t i = 0; i < types.size(); ++i) get->AddColumnId(i);
    return make_uniq<LogicalSequential>(std::move(get));
}
void Load(ExtensionLoader &loader) {
    DBConfig::GetConfig(loader.GetDatabaseInstance()).AddExtensionOption("thinkthen_settings", "ThinkThen client settings",
        LogicalType::VARCHAR, Value("{}"), Configure, SetScope::SESSION);
    for (auto function : {"decide", "choose", "tag", "score", "filter", "rank", "find", "annotate", "recognize", "relate"}) {
        TableFunction fn(string("thinkthen_") + function, {LogicalType::TABLE, LogicalType::VARCHAR}, nullptr);
        fn.named_parameters["options"] = LogicalType::VARCHAR;
        fn.bind_operator = Bind; fn.init_local = Init; fn.in_out_function = Run; fn.in_out_function_final = Final;
        loader.RegisterFunction(std::move(fn));
    }
}
} // namespace
extern "C" {
DUCKDB_CPP_EXTENSION_ENTRY(thinkthen, loader) { Load(loader); }
}
