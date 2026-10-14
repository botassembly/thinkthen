#include "descriptions.hpp"
#include "bridge.hpp"
#include "json_result.hpp"
#include "images.hpp"
#include "complete_files.hpp"
#include "files_manifest.hpp"
#include "duckdb/common/file_system.hpp"
#include "duckdb/common/error_data.hpp"
#include "portable.hpp"
#include "details_result.hpp"
#include "native_input.hpp"
#include "request_helpers_generated.hpp"
#include "scalar_owner.hpp"
#include "scalar_settings.hpp"
#include "duckdb/common/weak_ptr_ipp.hpp"
#include "duckdb/main/extension/extension_loader.hpp"
#include "duckdb/planner/expression/bound_function_expression.hpp"

extern "C" {
ThinkThenReply thinkthen_cpp_complete_failure_envelope(ThinkThenText);
ThinkThenReply thinkthen_cpp_complete_question_resolve(ThinkThenText, void **);
void thinkthen_cpp_complete_question_free(void *);
ThinkThenReply thinkthen_cpp_native_source_input(ThinkThenText, bool, bool);
ThinkThenReply thinkthen_cpp_native_records(ThinkThenText, ThinkThenText, ThinkThenText, ThinkThenText,
    ThinkThenText, const void *, bool, bool, int64_t, ThinkThenSettings, ThinkThenStop);
}
namespace duckdb {
namespace {
ThinkThenText View(const string &text) { return {reinterpret_cast<const uint8_t *>(text.data()), text.size()}; }
string FailureEnvelope(const string &error) {
    RustReply reply(thinkthen_cpp_complete_failure_envelope(View(error)));
    Checked(reply.value);
    return ReplyText(reply.value);
}
struct QuestionSelection {
    void *value = nullptr;
    ~QuestionSelection() { thinkthen_cpp_complete_question_free(value); }
};
struct Bind : FunctionData {
    weak_ptr<ClientContext> context;
    explicit Bind(weak_ptr<ClientContext> context) : context(std::move(context)) {}
    unique_ptr<FunctionData> Copy() const override { return make_uniq<Bind>(context); }
    bool Equals(const FunctionData &other) const override { return context.lock() == other.Cast<Bind>().context.lock(); }
};
unique_ptr<FunctionData> BindCompleteImpl(ClientContext &context, const string &name,
    vector<unique_ptr<Expression>> &arguments, LogicalType &result) {
    context.registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
    const auto verb = name.substr(string("thinkthen_native_").size(),
        name.size() - string("thinkthen_native_").size() - string("_details").size());
#if defined(THINKTHEN_DUCKDB_V2) && THINKTHEN_DUCKDB_V2
    const auto &input = arguments.at(1)->GetReturnType();
#else
    const auto &input = arguments.at(1)->return_type;
#endif
    const bool collection = input.id() == LogicalTypeId::LIST &&
        ListType::GetChildType(input).id() == LogicalTypeId::STRUCT;
    result = collection ? LogicalType::LIST(DetailsType(verb)) : DetailsType(verb);
    return make_uniq<Bind>(context.shared_from_this());
}
#if defined(THINKTHEN_DUCKDB_V2) && THINKTHEN_DUCKDB_V2
unique_ptr<FunctionData> BindComplete(BindScalarFunctionInput &input) {
    LogicalType result;
    auto data = BindCompleteImpl(input.GetClientContext(), input.GetBoundFunction().GetName().GetIdentifierName(), input.GetArguments(), result);
    input.GetBoundFunction().SetReturnType(std::move(result));
    return data;
}
#else
unique_ptr<FunctionData> BindComplete(ClientContext &context, ScalarFunction &function,
    vector<unique_ptr<Expression>> &arguments) {
    return BindCompleteImpl(context, function.name, arguments, function.return_type);
}
#endif
bool NativeSourceSelector(const Value &value) {
    if (value.IsNull() || value.type().id() != LogicalTypeId::STRUCT) { return false; }
    auto &fields = StructValue::GetChildren(value);
    for (idx_t index = 0; index < fields.size(); ++index) {
        if (ThinkThenStructName(value.type(), index) == "source" && !fields[index].IsNull() &&
            fields[index].type().id() == LogicalTypeId::BOOLEAN && fields[index].GetValue<bool>()) { return true; }
    }
    return false;
}
void NativeRecords(DataChunk &args, ExpressionState &state, Vector &result) {
    auto &expression = state.expr.Cast<BoundFunctionExpression>();
    auto context = ThinkThenBindData(expression).Cast<Bind>().context.lock();
    if (!context) { throw OrdinaryError("thinkthen defect: the caller session ended"); }
    const auto name = ThinkThenFunctionName(expression);
    const auto verb = name.substr(string("thinkthen_native_").size(), name.size() - string("thinkthen_native_").size() - string("_details").size());
    const bool collection = result.GetType().id() == LogicalTypeId::LIST;
    auto owner = context->registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
    std::optional<SessionSettings> session;
    for (idx_t row = 0; row < args.size(); ++row) {
        auto question = args.data[0].GetValue(row), records = args.data[1].GetValue(row);
        if (question.IsNull() || records.IsNull()) { result.SetValue(row, Value(result.GetType())); continue; }
        try {
            auto settings = args.data[3].GetValue(row);
            const auto controls = settings.IsNull() ? string("{}") : settings.type().id() == LogicalTypeId::VARCHAR ? settings.GetValue<string>() : NativeInputJSON(settings);
            RustReply validated(thinkthen_cpp_portable_settings(View(controls)));
            Checked(validated.value);
            for (idx_t argument : {idx_t(4), idx_t(5)}) {
                auto flag = args.data[argument].GetValue(row);
                if (!flag.IsNull() && flag.type().id() != LogicalTypeId::BOOLEAN) {
                    throw OrdinaryError("thinkthen usage: incremental and cancelled require native booleans");
                }
            }
            const auto authored = question.type().id() == LogicalTypeId::VARCHAR ? question.GetValue<string>() : NativeInputJSON(question);
            QuestionSelection selection;
            string content = authored;
            if (!authored.empty() && authored[0] == '@') {
                RustReply resolved(thinkthen_cpp_complete_question_resolve(View(authored), &selection.value));
                if (resolved.value.status != 0) { result.SetValue(row, DecodeDetails(FailureEnvelope(ReplyText(resolved.value)), verb, collection)); continue; }
                const auto path = ReplyText(resolved.value);
                AuthorizeLocalSource(FileSystem::GetFileSystem(*context), path);
                content = ReadQuestion(*context, path, "question", true);
            }
            auto reading = args.data[2].GetValue(row);
            const auto options = reading.IsNull() ? string("{}") : NativeInputJSON(reading);
            const Value *source = nullptr;
            if (records.type().id() == LogicalTypeId::LIST) {
                auto &members = ListValue::GetChildren(records);
                if (members.size() == 1 && NativeSourceSelector(members[0])) { source = &members[0]; }
            }
            const auto input = collection ? records : Value::LIST(LogicalType::STRUCT({{"text", LogicalType::VARCHAR}}), {Value::STRUCT({{"text", records}})});
            const auto inputs = NativeInputJSON(source ? *source : input, !source);
            const auto incremental = args.data[4].GetValue(row), cancelled = args.data[5].GetValue(row);
            if (!session) { session.emplace(Settings(*context)); }
            if (source) {
                RustReply descriptor(thinkthen_cpp_native_source_input(View(inputs), !incremental.IsNull() && incremental.GetValue<bool>(), !cancelled.IsNull() && cancelled.GetValue<bool>()));
                if (descriptor.value.status != 0) {
                    result.SetValue(row, DecodeDetails(FailureEnvelope(ReplyText(descriptor.value)), verb, collection)); continue;
                }
                string source_result;
                CompleteFileCall(*context, verb, content, ReplyText(descriptor.value), controls, selection.value,
                    owner->Remaining(*context), session->Bridge(), source_result, options);
                result.SetValue(row, DecodeDetails(source_result, verb, collection)); continue;
            }
            RustReply reply(thinkthen_cpp_native_records(View(verb), View(content), View(inputs), View(options), View(controls),
                selection.value, !incremental.IsNull() && incremental.GetValue<bool>(), !cancelled.IsNull() && cancelled.GetValue<bool>(), owner->Remaining(*context), session->Bridge(), StopFor(*context)));
            Checked(reply.value);
            result.SetValue(row, DecodeDetails(ReplyText(reply.value), verb, collection));
        } catch (const Exception &error) {
            if (ErrorData(error).Type() == ExceptionType::INTERRUPT || QueryInterrupted(*context)) { throw; }
            result.SetValue(row, DecodeDetails(FailureEnvelope(CompleteAdmissionError(error)), verb, collection));
        }
    }
}
}
void RegisterComplete(ExtensionLoader &loader) {
    RegisterRequestHelpers(loader);
    for (auto verb : {"decide", "choose", "tag", "score", "filter", "rank", "find", "annotate", "recognize", "relate"}) {
        const auto name = string("thinkthen_native_") + verb + "_details";
        const string purpose = "Return typed values, probabilities, results, failures, facts and observations.";
        ScalarFunction function(ThinkThenIdentifier(name),
            {LogicalType::ANY, LogicalType::ANY, LogicalType::ANY, LogicalType::ANY, LogicalType::ANY, LogicalType::ANY},
            DetailsType(verb), NativeRecords, BindComplete);
        ThinkThenSpecialNulls(function);
        function.SetStability(FunctionStability::VOLATILE);
        RegisterDescribedScalar(loader, function, purpose);
        RegisterPortableMacro(loader, string("CREATE MACRO thinkthen_") + verb +
            "_details(question, input, reading := NULL, settings := NULL, incremental := false, cancelled := false) AS " +
            name + "(question, input, reading, settings, incremental, cancelled)", purpose);
    }
}
}
