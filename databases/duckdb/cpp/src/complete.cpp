#include "bridge.hpp"
#include "complete_files.hpp"
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
                                     int32_t,int64_t, ThinkThenSettings, ThinkThenStop);
}
namespace duckdb {
namespace {
ThinkThenText View(const string &text) { return {reinterpret_cast<const uint8_t *>(text.data()), text.size()}; }
string FailureEnvelope(const string &error) {
    RustReply reply(thinkthen_cpp_complete_failure_envelope(View(error)));
    Checked(reply.value);
    return ReplyText(reply.value);
}
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
    const auto session = Settings(*context);
    for (idx_t row = 0; row < args.size(); ++row) {
        auto question = args.data[0].GetValue(row), inputs = args.data[1].GetValue(row), settings = args.data[2].GetValue(row);
        if (question.IsNull() || inputs.IsNull()) { result.SetValue(row, Value(LogicalType::VARCHAR)); continue; }
        string failure;
        const auto source = question.GetValue<string>(), input = CompleteFileInputs(*context,inputs.GetValue<string>(),failure), controls = settings.IsNull() ? "{}" : settings.GetValue<string>();
        if (!failure.empty()) { result.SetValue(row,Value(FailureEnvelope(failure))); continue; }
        try {
        if (source.rfind("@@",0)==0) { throw OrdinaryError("thinkthen usage: named questions require the pending authorized native resolver"); }
        const auto resolved = owner->Resolve(*context,source);
        RustReply reply(thinkthen_cpp_complete(View(verb), View(resolved.text), View(input), View(controls), resolved.from_file ? 1 : 0, owner->Remaining(*context), session.Bridge(), StopFor(*context)));
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
        RegisterPortableMacro(loader,string("CREATE MACRO thinkthen_")+verb+"_complete(question, inputs, settings := NULL) AS "+name+"(question, inputs, settings)");
    }
}
}
