#include "descriptions.hpp"
#include "bridge.hpp"
#include "json_result.hpp"
#include "images.hpp"
#include "complete_files.hpp"
#include "files_manifest.hpp"
#include "duckdb/common/file_system.hpp"
#include "duckdb/common/error_data.hpp"
#include "portable.hpp"
#include "receipt.hpp"
#include "native_input.hpp"
#include "request_helpers_generated.hpp"
#include "scalar_owner.hpp"
#include "scalar_settings.hpp"
#include "duckdb/common/weak_ptr_ipp.hpp"
#include "duckdb/main/extension/extension_loader.hpp"
#include "duckdb/planner/expression/bound_function_expression.hpp"

extern "C" {
ThinkThenReply thinkthen_cpp_complete_failure_envelope(ThinkThenText);
ThinkThenReply thinkthen_cpp_complete(ThinkThenText, ThinkThenText, ThinkThenText, ThinkThenText,
                                     const void *,int64_t, ThinkThenSettings, ThinkThenStop);
ThinkThenReply thinkthen_cpp_complete_images(ThinkThenText, const ThinkThenImage *, size_t, ThinkThenText,
                                            const void *, int64_t, ThinkThenSettings, ThinkThenStop);
ThinkThenReply thinkthen_cpp_complete_question_resolve(ThinkThenText, void **);
void thinkthen_cpp_complete_question_free(void *);
ThinkThenReply thinkthen_cpp_native_source_input(ThinkThenText, bool);
ThinkThenReply thinkthen_cpp_native_records(ThinkThenText, ThinkThenText, ThinkThenText, ThinkThenText,
    ThinkThenText, const void *, bool, int64_t, ThinkThenSettings, ThinkThenStop);
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
unique_ptr<FunctionData> BindComplete(ClientContext &context, const string &, vector<unique_ptr<Expression>> &) {
    context.registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
    return make_uniq<Bind>(context.shared_from_this());
}
void Complete(DataChunk &args, ExpressionState &state, Vector &result) {
    auto &expr = state.expr.Cast<BoundFunctionExpression>();
    auto context = ThinkThenBindData(expr).Cast<Bind>().context.lock();
    if (!context) { throw OrdinaryError("thinkthen defect: the caller session ended"); }
    auto owner = context->registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
    auto name = ThinkThenFunctionName(expr);
    const string prefix = "thinkthen_native_", suffix = "_complete";
    const auto verb = name.substr(prefix.size(), name.size() - prefix.size() - suffix.size());
    std::optional<SessionSettings> session;
    for (idx_t row = 0; row < args.size(); ++row) {
        auto question = args.data[0].GetValue(row);
        if (question.IsNull()) { result.SetValue(row, Value(ReceiptType(verb))); continue; }
        auto inputs = args.data[1].GetValue(row);
        if (inputs.IsNull()) { result.SetValue(row, Value(ReceiptType(verb))); continue; }
        if (!session) { session.emplace(Settings(*context)); }
        auto settings = args.data[2].GetValue(row);
        const bool binary = inputs.type().id() == LogicalTypeId::LIST;
        const auto source = question.GetValue<string>(), input = binary ? string() : inputs.GetValue<string>(), controls = settings.IsNull() ? "{}" : settings.GetValue<string>();
        try {
        QuestionSelection selection;
        string content = source;
        if (!source.empty() && source[0]=='@') {
            RustReply resolved(thinkthen_cpp_complete_question_resolve(View(source), &selection.value));
            if (resolved.value.status!=0) { result.SetValue(row, DecodeReceipt(FailureEnvelope(ReplyText(resolved.value)), verb)); continue; }
            const auto path = ReplyText(resolved.value);
            AuthorizeLocalSource(FileSystem::GetFileSystem(*context), path);
            content = ReadQuestion(*context, path, "question", true);
        }
        string file_result;
        if (!binary && CompleteFileCall(*context,verb,content,input,controls,selection.value,owner->Remaining(*context),session->Bridge(),file_result)) {
            result.SetValue(row,DecodeReceipt(file_result, verb)); continue;
        }
        auto images = binary ? ImageMembers(inputs) : vector<std::pair<string, string>>();
        auto views = ImageViews(images);
        RustReply reply(binary
            ? thinkthen_cpp_complete_images(View(content), views.data(), views.size(), View(controls), selection.value, owner->Remaining(*context), session->Bridge(), StopFor(*context))
            : thinkthen_cpp_complete(View(verb), View(content), View(input), View(controls), selection.value, owner->Remaining(*context), session->Bridge(), StopFor(*context)));
        Checked(reply.value);
        result.SetValue(row, DecodeReceipt(ReplyText(reply.value), verb));
        } catch (const Exception &error) {
            if (ErrorData(error).Type()==ExceptionType::INTERRUPT || QueryInterrupted(*context)) { throw; }
            result.SetValue(row,DecodeReceipt(FailureEnvelope(CompleteAdmissionError(error)), verb));
        }
    }
}
void NativeRecords(DataChunk &args, ExpressionState &state, Vector &result) {
    auto &expression = state.expr.Cast<BoundFunctionExpression>();
    auto context = ThinkThenBindData(expression).Cast<Bind>().context.lock();
    if (!context) { throw OrdinaryError("thinkthen defect: the caller session ended"); }
    const auto verb = ThinkThenFunctionName(expression).substr(string("thinkthen_native_records_").size());
    auto owner = context->registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
    std::optional<SessionSettings> session;
    for (idx_t row = 0; row < args.size(); ++row) {
        auto question = args.data[0].GetValue(row), records = args.data[1].GetValue(row);
        if (question.IsNull() || records.IsNull()) { result.SetValue(row, Value(ReceiptType(verb))); continue; }
        try {
            const auto authored = question.type().id() == LogicalTypeId::VARCHAR ? question.GetValue<string>() : NativeInputJSON(question);
            QuestionSelection selection;
            string content = authored;
            if (!authored.empty() && authored[0] == '@') {
                RustReply resolved(thinkthen_cpp_complete_question_resolve(View(authored), &selection.value));
                if (resolved.value.status != 0) { result.SetValue(row, DecodeReceipt(FailureEnvelope(ReplyText(resolved.value)), verb)); continue; }
                const auto path = ReplyText(resolved.value);
                AuthorizeLocalSource(FileSystem::GetFileSystem(*context), path);
                content = ReadQuestion(*context, path, "question", true);
            }
            auto reading = args.data[2].GetValue(row), settings = args.data[3].GetValue(row);
            const auto options = reading.IsNull() ? string("{}") : NativeInputJSON(reading);
            const auto controls = settings.IsNull() ? string("{}") : settings.type().id() == LogicalTypeId::VARCHAR ? settings.GetValue<string>() : NativeInputJSON(settings);
            const auto inputs = NativeInputJSON(records, true);
            const auto incremental = args.data[4].GetValue(row);
            if (!session) { session.emplace(Settings(*context)); }
            if (records.type().id() == LogicalTypeId::STRUCT) {
                RustReply descriptor(thinkthen_cpp_native_source_input(View(inputs), !incremental.IsNull() && incremental.GetValue<bool>()));
                if (descriptor.value.status != 0) {
                    result.SetValue(row, DecodeReceipt(FailureEnvelope(ReplyText(descriptor.value)), verb)); continue;
                }
                string source_result;
                CompleteFileCall(*context, verb, content, ReplyText(descriptor.value), controls, selection.value,
                    owner->Remaining(*context), session->Bridge(), source_result, options);
                result.SetValue(row, DecodeReceipt(source_result, verb)); continue;
            }
            RustReply reply(thinkthen_cpp_native_records(View(verb), View(content), View(inputs), View(options), View(controls),
                selection.value, !incremental.IsNull() && incremental.GetValue<bool>(), owner->Remaining(*context), session->Bridge(), StopFor(*context)));
            Checked(reply.value);
            result.SetValue(row, DecodeReceipt(ReplyText(reply.value), verb));
        } catch (const Exception &error) {
            if (ErrorData(error).Type() == ExceptionType::INTERRUPT || QueryInterrupted(*context)) { throw; }
            result.SetValue(row, DecodeReceipt(FailureEnvelope(CompleteAdmissionError(error)), verb));
        }
    }
}
}
void RegisterComplete(ExtensionLoader &loader) {
    for (auto verb : {"decide","choose","tag","score","filter","rank","find","annotate","recognize","relate"}) {
        const string purpose = string("Return complete ") + verb + " readings, final facts and failures as native SQL fields.";
        const auto name = string("thinkthen_native_") + verb + "_complete";
        ScalarFunction function(ThinkThenIdentifier(name),{LogicalType::VARCHAR,LogicalType::VARCHAR,LogicalType::VARCHAR},ReceiptType(verb),Complete,ThinkThenBindScalar<BindComplete>);
        ThinkThenSpecialNulls(function);
        function.SetStability(FunctionStability::VOLATILE);
        RegisterDescribedScalar(loader, function, purpose);
        if (string(verb) == "decide") {
            ThinkThenArgument(function, 1, LogicalType::LIST(ImageType()));
            RegisterDescribedScalar(loader, function, purpose);
        }
        RegisterPortableMacro(loader,string("CREATE MACRO thinkthen_")+verb+"_complete(question, inputs, settings := NULL) AS "+name+"(question, inputs, settings)", purpose);
    }
    RegisterRequestHelpers(loader);
    for (auto verb : {"decide", "choose", "tag", "score", "filter", "find", "annotate", "recognize"}) {
        const auto name = string("thinkthen_native_records_") + verb;
        const string purpose = "Read ordered native SQL records and return typed readings, final facts, failures and observations.";
        ScalarFunction function(ThinkThenIdentifier(name), {LogicalType::ANY, LogicalType::ANY, LogicalType::ANY, LogicalType::ANY, LogicalType::ANY},
            ReceiptType(verb), NativeRecords, ThinkThenBindScalar<BindComplete>);
        ThinkThenSpecialNulls(function);
        function.SetStability(FunctionStability::VOLATILE);
        RegisterDescribedScalar(loader, function, purpose);
        RegisterPortableMacro(loader, string("CREATE MACRO thinkthen_") + verb +
            "(question, records, reading := NULL, settings := NULL, incremental := false) AS TABLE "
            "WITH call AS MATERIALIZED (SELECT " + name + "(question, records, reading, settings, incremental) AS receipt) "
            "SELECT list_extract(receipt.ordinals, position) AS ordinal, " +
            ((string(verb) == "decide" || string(verb) == "choose" || string(verb) == "tag" || string(verb) == "score" || string(verb) == "filter")
                ? "list_extract(receipt.answers, position)" : "result.value") +
            " AS value, result, receipt.facts AS facts, "
            "receipt.failure AS failure, receipt.observations AS observations, receipt.selection AS selection "
            "FROM call, UNNEST(CASE WHEN len(receipt.results) = 0 THEN [NULL] ELSE receipt.results END) WITH ORDINALITY AS rows(result, position) "
            "WHERE receipt IS NOT NULL", purpose);
    }
}
}
