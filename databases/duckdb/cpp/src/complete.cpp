#include "bridge.hpp"
#include "images.hpp"
#include "complete_files.hpp"
#include "files_manifest.hpp"
#include "duckdb/common/file_system.hpp"
#include "duckdb/common/error_data.hpp"
#include "portable.hpp"
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
unique_ptr<FunctionData> BindComplete(ClientContext &context, ScalarFunction &, vector<unique_ptr<Expression>> &) {
    context.registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
    return make_uniq<Bind>(context.shared_from_this());
}
void Complete(DataChunk &args, ExpressionState &state, Vector &result) {
    auto &expr = state.expr.Cast<BoundFunctionExpression>();
    auto context = expr.bind_info->Cast<Bind>().context.lock();
    if (!context) { throw OrdinaryError("thinkthen defect: the caller session ended"); }
    auto owner = context->registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
    auto name = expr.function.name;
    const string prefix = "thinkthen_native_", suffix = "_complete";
    const auto verb = name.substr(prefix.size(), name.size() - prefix.size() - suffix.size());
    std::optional<SessionSettings> session;
    for (idx_t row = 0; row < args.size(); ++row) {
        auto question = args.data[0].GetValue(row);
        if (question.IsNull()) { result.SetValue(row, Value(LogicalType::VARCHAR)); continue; }
        auto inputs = args.data[1].GetValue(row);
        if (inputs.IsNull()) { result.SetValue(row, Value(LogicalType::VARCHAR)); continue; }
        if (!session) { session.emplace(Settings(*context)); }
        auto settings = args.data[2].GetValue(row);
        string failure;
        const bool binary = inputs.type().id() == LogicalTypeId::LIST;
        const auto source = question.GetValue<string>(), input = binary ? string() : CompleteFileInputs(*context,inputs.GetValue<string>(),failure), controls = settings.IsNull() ? "{}" : settings.GetValue<string>();
        if (!failure.empty()) { result.SetValue(row,Value(FailureEnvelope(failure))); continue; }
        try {
        QuestionSelection selection;
        string content = source;
        if (!source.empty() && source[0]=='@') {
            RustReply resolved(thinkthen_cpp_complete_question_resolve(View(source), &selection.value));
            if (resolved.value.status!=0) { result.SetValue(row, Value(FailureEnvelope(ReplyText(resolved.value)))); continue; }
            const auto path = ReplyText(resolved.value);
            AuthorizeLocalSource(FileSystem::GetFileSystem(*context), path);
            content = ReadQuestion(*context, path, "question", true);
        }
        auto images = binary ? ImageMembers(inputs) : vector<std::pair<string, string>>();
        auto views = ImageViews(images);
        RustReply reply(binary
            ? thinkthen_cpp_complete_images(View(content), views.data(), views.size(), View(controls), selection.value, owner->Remaining(*context), session->Bridge(), StopFor(*context))
            : thinkthen_cpp_complete(View(verb), View(content), View(input), View(controls), selection.value, owner->Remaining(*context), session->Bridge(), StopFor(*context)));
        Checked(reply.value);
        result.SetValue(row, Value(string(reinterpret_cast<const char *>(reply.value.bytes), reply.value.len)));
        } catch (const Exception &error) {
            if (ErrorData(error).Type()==ExceptionType::INTERRUPT) { throw; }
            result.SetValue(row,Value(FailureEnvelope(CompleteAdmissionError(error))));
        }
    }
}
}
void RegisterComplete(ExtensionLoader &loader) {
    for (auto verb : {"decide","choose","tag","score","filter","rank","find","annotate","recognize","relate"}) {
        const auto name = string("thinkthen_native_") + verb + "_complete";
        ScalarFunction function(name,{LogicalType::VARCHAR,LogicalType::VARCHAR,LogicalType::VARCHAR},LogicalType::VARCHAR,Complete,BindComplete);
        function.null_handling = FunctionNullHandling::SPECIAL_HANDLING;
        function.SetStability(FunctionStability::VOLATILE);
        loader.RegisterFunction(function);
        if (string(verb) == "decide") {
            function.arguments[1] = LogicalType::LIST(ImageType());
            loader.RegisterFunction(function);
        }
        RegisterPortableMacro(loader,string("CREATE MACRO thinkthen_")+verb+"_complete(question, inputs, settings := NULL) AS "+name+"(question, inputs, settings)");
    }
}
}
