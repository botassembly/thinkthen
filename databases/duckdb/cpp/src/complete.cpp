#include "descriptions.hpp"
#include "bridge.hpp"
#include "json_result.hpp"
#include "images.hpp"
#include "relate.hpp"
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
#include <map>
#include <tuple>

extern "C" {
ThinkThenReply thinkthen_cpp_complete_failure_envelope(ThinkThenText);
ThinkThenReply thinkthen_cpp_complete_question_resolve(ThinkThenText, void **);
void thinkthen_cpp_complete_question_free(void *);
ThinkThenReply thinkthen_cpp_native_source_input(ThinkThenText, bool, bool);
ThinkThenReply thinkthen_cpp_details_keyed_inputs(ThinkThenText);
ThinkThenReply thinkthen_cpp_native_records(ThinkThenText, ThinkThenText, ThinkThenText, ThinkThenText,
    ThinkThenText, const void *, bool, bool, bool, int64_t, ThinkThenSettings, ThinkThenStop);
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
    const bool collection = (input.id() == LogicalTypeId::LIST &&
        (ListType::GetChildType(input).id() == LogicalTypeId::STRUCT || ListType::GetChildType(input).id() == LogicalTypeId::SQLNULL)) || verb == "rank" || verb == "relate";
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
Value AdmissionFailure(const Exception &error, ClientContext &context, const string &verb, bool output_list) {
    if (ErrorData(error).Type() == ExceptionType::INTERRUPT || QueryInterrupted(context)) { throw; }
    const auto failure = CompleteAdmissionError(error);
    auto doc = duckdb_yyjson::yyjson_read(failure.data(), failure.size(), 0);
    if (!doc) { throw; }
    auto kind = ReceiptMember(ReceiptMember(duckdb_yyjson::yyjson_doc_get_root(doc), "error"), "kind");
    const auto hard = kind && duckdb_yyjson::yyjson_is_str(kind) &&
        (string(duckdb_yyjson::yyjson_get_str(kind)) == "cancelled" ||
         string(duckdb_yyjson::yyjson_get_str(kind)) == "deadline" ||
         string(duckdb_yyjson::yyjson_get_str(kind)) == "defect");
    duckdb_yyjson::yyjson_doc_free(doc);
    if (hard) { throw; }
    return DecodeDetails(FailureEnvelope(failure), verb, output_list);
}
void NativeRecords(DataChunk &args, ExpressionState &state, Vector &result) {
    auto &expression = state.expr.Cast<BoundFunctionExpression>();
    auto context = ThinkThenBindData(expression).Cast<Bind>().context.lock();
    if (!context) { throw OrdinaryError("thinkthen defect: the caller session ended"); }
    const auto name = ThinkThenFunctionName(expression);
    const auto verb = name.substr(string("thinkthen_native_").size(), name.size() - string("thinkthen_native_").size() - string("_details").size());
    const bool output_list = result.GetType().id() == LogicalTypeId::LIST;
    const auto input_type = args.data[1].GetType();
    const bool collection = input_type.id() == LogicalTypeId::LIST &&
        (ListType::GetChildType(input_type).id() == LogicalTypeId::STRUCT || ListType::GetChildType(input_type).id() == LogicalTypeId::SQLNULL);
    auto owner = context->registered_state->GetOrCreate<StatementOwner>(OWNER_KEY);
    std::optional<SessionSettings> session;
    struct Pending {
        string content, inputs, options, controls;
        unique_ptr<QuestionSelection> selection;
        bool source, incremental, cancelled;
        vector<idx_t> slots;
        Value rules;
    };
    vector<Pending> calls;
    std::map<std::tuple<string, string, string, bool, bool>, idx_t> groups;
    for (idx_t row = 0; row < args.size(); ++row) {
        auto question = args.data[0].GetValue(row), records = args.data[1].GetValue(row);
        if (question.IsNull() || records.IsNull()) { result.SetValue(row, Value(result.GetType())); continue; }
        try {
            auto settings = args.data[3].GetValue(row);
            auto controls = settings.IsNull() ? string("{}") : settings.type().id() == LogicalTypeId::VARCHAR ? settings.GetValue<string>() : NativeInputJSON(settings);
            const auto reading = args.data[2].GetValue(row);
            if (!collection && !reading.IsNull()) {
                if ((verb == "choose" || verb == "tag" || verb == "score") && reading.type().id() == LogicalTypeId::LIST) {
                    auto last = controls.find_last_not_of(" \t\r\n");
                    if (last == string::npos || controls.at(last) != '}') {
                        throw OrdinaryError("thinkthen usage: settings must be an object");
                    }
                    const auto first = controls.find_first_not_of(" \t\r\n", controls.find('{') + 1);
                    const auto key = verb == "choose" ? "options" : verb == "tag" ? "labels" : "levels";
                    controls = controls.substr(0, last) + (first == last ? "" : ",") + "\"" + key + "\":" + NativeInputJSON(reading) + "}";
                } else if (settings.IsNull()) {
                    controls = reading.type().id() == LogicalTypeId::VARCHAR ? reading.GetValue<string>() : NativeInputJSON(reading);
                } else { throw OrdinaryError("thinkthen usage: pass one settings object"); }
            }
            const auto threshold = args.data[6].GetValue(row);
            if (!threshold.IsNull()) {
                const auto end = controls.find_last_not_of(" \t\r\n");
                if (end == string::npos || controls[end] != '}') {
                    throw OrdinaryError("thinkthen usage: settings must be an object");
                }
                const auto first = controls.find_first_not_of(" \t\r\n", controls.find('{') + 1);
                controls = controls.substr(0, end) + (first == end ? "" : ",") + "\"threshold\":" + NativeInputJSON(threshold) + "}";
            }
            RustReply validated(thinkthen_cpp_portable_settings(View(controls)));
            Checked(validated.value);
            for (idx_t argument : {idx_t(4), idx_t(5)}) {
                auto flag = args.data[argument].GetValue(row);
                if (!flag.IsNull() && flag.type().id() != LogicalTypeId::BOOLEAN) {
                    throw OrdinaryError("thinkthen usage: incremental and cancelled require native booleans");
                }
            }
            if (!collection && verb == "relate") {
                auto cancelled = args.data[5].GetValue(row);
                if (!cancelled.IsNull() && cancelled.GetValue<bool>()) { throw OrdinaryError("thinkthen cancelled: the call was cancelled"); }
                const auto query = question.GetValue<string>();
                if (query.find_first_not_of(" \t\r\n") == string::npos) { throw OrdinaryError("thinkthen usage: the relate query is NULL or blank"); }
                calls.push_back({query, "", "", controls, make_uniq<QuestionSelection>(), false, false, false, {row}, records});
                continue;
            }
            auto authored = question.type().id() == LogicalTypeId::VARCHAR ? question.GetValue<string>() : NativeInputJSON(question);
            if (!collection && verb == "recognize") {
                authored = records.type().id() == LogicalTypeId::VARCHAR ? records.GetValue<string>() : NativeInputJSON(records);
            }
            auto selection = make_uniq<QuestionSelection>();
            string content = authored;
            if (!authored.empty() && authored[0] == '@') {
                RustReply resolved(thinkthen_cpp_complete_question_resolve(View(authored), &selection->value));
                if (resolved.value.status != 0) { result.SetValue(row, DecodeDetails(FailureEnvelope(ReplyText(resolved.value)), verb, output_list)); continue; }
                const auto path = ReplyText(resolved.value);
                AuthorizeLocalSource(FileSystem::GetFileSystem(*context), path);
                content = ReadQuestion(*context, path, "question", true);
            }
            const auto options = !collection || reading.IsNull() ? string("{}") : NativeInputJSON(reading);
            const Value *source = nullptr;
            if (records.type().id() == LogicalTypeId::LIST) {
                auto &members = ListValue::GetChildren(records);
                if (members.size() == 1 && NativeSourceSelector(members[0])) { source = &members[0]; }
            }
            vector<Value> native_records;
            const auto text_type = LogicalType::STRUCT({{"text", LogicalType::VARCHAR}});
            if (!collection && verb == "find" && records.type().id() == LogicalTypeId::LIST) {
                for (const auto &unit : ListValue::GetChildren(records)) { native_records.push_back(Value::STRUCT({{"text", unit}})); }
            } else if (!collection && verb != "rank") { native_records.push_back(Value::STRUCT({{"text", verb == "recognize" ? question : records}})); }
            const auto input = collection ? records : Value::LIST(text_type, std::move(native_records));
            auto inputs = NativeInputJSON(source ? *source : input, !source);
            if (!collection && verb == "rank") {
                const auto raw = records.type().id() == LogicalTypeId::VARCHAR ? records.GetValue<string>() : NativeInputJSON(records);
                RustReply normalized(thinkthen_cpp_details_keyed_inputs(View(raw))); Checked(normalized.value);
                inputs = ReplyText(normalized.value);
            }
            const auto incremental = args.data[4].GetValue(row), cancelled = args.data[5].GetValue(row);
            const auto eager = !incremental.IsNull() && incremental.GetValue<bool>();
            const auto stopped = !cancelled.IsNull() && cancelled.GetValue<bool>();
            const auto key = std::make_tuple(authored, options, controls, eager, stopped);
            auto place = collection || verb == "rank" || verb == "find" || verb == "relate" ? groups.end() : groups.find(key);
            if (place != groups.end()) {
                auto &call = calls.at(place->second);
                call.inputs.pop_back();
                call.inputs += "," + inputs.substr(1);
                call.slots.push_back(row);
            } else {
                if (!collection && verb != "rank" && verb != "find" && verb != "relate") { groups.emplace(key, calls.size()); }
                calls.push_back({content, inputs, options, controls, std::move(selection), source != nullptr,
                    eager, stopped, {row}, Value()});
            }
        } catch (const Exception &error) {
            result.SetValue(row, AdmissionFailure(error, *context, verb, output_list));
        }
    }
    if (!calls.empty()) { session.emplace(Settings(*context)); }
    for (auto &call : calls) {
        try {
        string answer;
        if (!collection && verb == "relate") {
            answer = RelateDetailsCall(*context, call.content, call.rules, call.controls,
                owner->Remaining(*context), session->Bridge());
        } else if (call.source) {
            RustReply descriptor(thinkthen_cpp_native_source_input(View(call.inputs), call.incremental, call.cancelled));
            Checked(descriptor.value);
            CompleteFileCall(*context, verb, call.content, ReplyText(descriptor.value), call.controls,
                call.selection->value, owner->Remaining(*context), session->Bridge(), answer, call.options);
        } else {
            RustReply reply(thinkthen_cpp_native_records(View(verb), View(call.content), View(call.inputs), View(call.options),
                View(call.controls), call.selection->value, call.incremental, call.cancelled, !collection,
                owner->Remaining(*context), session->Bridge(), StopFor(*context)));
            Checked(reply.value);
            answer = ReplyText(reply.value);
        }
        auto decoded = DecodeDetails(answer, verb, true);
        if (output_list) { result.SetValue(call.slots.at(0), decoded); continue; }
        const auto &rows = ListValue::GetChildren(decoded);
        for (idx_t ordinal = 0; ordinal < call.slots.size(); ++ordinal) {
            const Value *found = nullptr;
            for (auto &row : rows) {
                const auto &fields = StructValue::GetChildren(row);
                if (!fields.at(0).IsNull() && fields.at(0).GetValue<uint64_t>() == ordinal) { found = &row; break; }
            }
            if (!found && rows.size() == 1) { found = &rows.at(0); }
            result.SetValue(call.slots.at(ordinal), found ? *found : Value(DetailsType(verb)));
        }
        } catch (const Exception &error) {
            auto failed = AdmissionFailure(error, *context, verb, output_list);
            for (const auto slot : call.slots) { result.SetValue(slot, failed); }
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
            {LogicalType::ANY, LogicalType::ANY, LogicalType::ANY, LogicalType::ANY, LogicalType::ANY, LogicalType::ANY, LogicalType::ANY},
            DetailsType(verb), NativeRecords, BindComplete);
        ThinkThenSpecialNulls(function);
        function.SetStability(FunctionStability::VOLATILE);
        RegisterDescribedScalar(loader, function, purpose);
        const string verb_name = verb;
        const auto threshold = verb_name == "decide" || verb_name == "filter";
        const string input = verb_name == "find" ? "coalesce(units, input)" : verb_name == "rank" ? "coalesce(keyed_json, input)" :
            verb_name == "relate" ? "coalesce(rules, input)" : verb_name == "recognize" ? "coalesce(kinds, input)" : "input";
        const string question = verb_name == "relate" ? "coalesce(query, question)" :
            verb_name == "recognize" ? "CASE WHEN kinds IS NOT NULL THEN " + PortableJSONArgument("input") + " ELSE " + PortableJSONArgument("question") + " END" : "question";
        const string records = "((starts_with(typeof(" + input + "), 'STRUCT(') OR typeof(" + input + ") = '\"NULL\"[]') AND ends_with(typeof(" + input + "), '[]'))";
        const string listed = verb_name == "choose" || verb_name == "tag" || verb_name == "score" ? "true" : "false";
        const auto members = "(" + listed + " AND ends_with(typeof(reading), '[]'))";
        const auto settings = "CASE WHEN " + records + " OR " + members + " THEN " + PortableJSONArgument("settings") +
            " ELSE coalesce(" + PortableJSONArgument("settings") + ", " + PortableJSONArgument("reading") + ") END";
        const auto reading = "CASE WHEN members IS NOT NULL THEN members WHEN " + records + " OR " + members + " THEN reading ELSE NULL END";
        const auto aliases = verb_name == "find" ? ", units := NULL" : verb_name == "rank" ? ", keyed_json := NULL" :
            verb_name == "relate" ? ", query := NULL, rules := NULL" : verb_name == "recognize" ? ", kinds := NULL" : "";
        const string slots = threshold ? ", reading := NULL, threshold := NULL, settings := NULL" : ", reading := NULL, settings := NULL";
        RegisterPortableMacro(loader, string("CREATE MACRO thinkthen_") + verb +
            "_details(question := NULL, input := NULL" + slots + ", incremental := false, cancelled := false, members := NULL" + aliases + PortableOptionParameters() + ") AS " +
            name + "(" + question + ", " + input + ", " + reading + ", " + PortableOptions(settings, question + " IS NULL OR " + input + " IS NULL", false, true) +
            ", incremental, cancelled, " + (threshold ? "threshold" : "NULL") + ")", purpose);
    }
}
}
