Value Argument(TableFunctionBindInput &input, const string &name, idx_t position = DConstants::INVALID_INDEX) {
    auto found = input.named_parameters.find(Identifier(name));
    if (position < input.inputs.size()) {
        if (found != input.named_parameters.end()) throw InvalidInputException("usage.question: duplicate %s", name);
        return input.inputs[position];
    }
    return found == input.named_parameters.end() ? Value() : found->second;
}
string RequiredList(const string &function) {
    for (auto name : {"choices", "labels", "levels", "kinds", "rules", "toolbox"})
        if (HasArgument(function, name)) return name;
    return "";
}
void TextArgument(const string &name, const Value &value) {
    if (!value.IsNull() && value.type().id() != LogicalTypeId::VARCHAR)
        throw InvalidInputException("usage.option: %s expects VARCHAR, received %s", name, value.type().ToString());
}
string Described(const Value &required, const Value &descriptions) {
    if (descriptions.type().id() != LogicalTypeId::MAP ||
        MapType::KeyType(descriptions.type()).id() != LogicalTypeId::VARCHAR ||
        MapType::ValueType(descriptions.type()).id() != LogicalTypeId::VARCHAR)
        throw InvalidInputException("usage.option: descriptions expects MAP(VARCHAR, VARCHAR)");
    if (required.type().id() != LogicalTypeId::LIST)
        return Encode(required); // Let shared admission diagnose the domain type.
    auto mapping = Parse(Encode(descriptions));
    auto root = yyjson_doc_get_root(mapping.get());
    string result = "[";
    // Description names are case sensitive, just like the engine's labels.
    std::map<string, bool> used;
    size_t i, n; yyjson_val *key, *description;
    yyjson_obj_foreach(root, i, n, key, description) used[yyjson_get_str(key)] = false;
    for (auto &item : ListValue::GetChildren(required)) {
        if (result.size() > 1) result += ",";
        if (item.IsNull() || item.type().id() != LogicalTypeId::VARCHAR) { result += Encode(item); continue; }
        auto name = item.GetValue<string>();
        auto found = used.find(name);
        if (found == used.end()) result += Encode(item);
        else {
            found->second = true;
            auto value = yyjson_obj_get(root, name.c_str());
            if (!yyjson_is_str(value)) throw InvalidInputException("usage.option: description for %s expects VARCHAR", name);
            result += "{\"name\":" + Quote(name) + ",\"description\":" + Quote(string(yyjson_get_str(value), yyjson_get_len(value))) + "}";
        }
    }
    for (auto &entry : used) if (!entry.second)
        throw InvalidInputException("usage.option: descriptions names unknown label or level %s", entry.first);
    return result + "]";
}
QuestionOwner Admit(TableFunctionBindInput &input, Binding &bind) {
    ErrorOwner error;
    thinkthen_question *raw = nullptr;
    auto list = RequiredList(bind.function);
    bool wording = HasArgument(bind.function, "question");
    auto required = list.empty() ? Value() : Argument(input, list, wording ? 2 : 1);
    if (!list.empty() && required.IsNull()) throw InvalidInputException("usage.question: %s is required", list);
    string json = "{\"function\":" + Quote(bind.function);
    if (wording) {
        if (input.inputs[1].IsNull()) throw InvalidInputException("usage.question: question cannot be SQL NULL");
        json += ",\"question\":" + Encode(input.inputs[1]);
    }
    if (!list.empty()) {
        auto descriptions = Argument(input, "descriptions");
        json += "," + Quote(bind.function == "annotate" ? "questions" : list) + ":" +
            (descriptions.IsNull() ? Encode(required) : Described(required, descriptions));
    }
    for (auto &key : {"yes_means", "no_means", "top", "field"}) {
        auto value = Argument(input, key);
        if (value.IsNull()) continue;
        if (string(key) == "field") bind.json = true;
        json += "," + Quote(string(key) == "yes_means" ? "true" : string(key) == "no_means" ? "false" : key) + ":" + Encode(value);
    }
    json += "}";
    Checked(thinkthen_sql_question_from_json(Bytes(json), json.size(), &raw, &error.value), error);
    QuestionOwner question(raw, thinkthen_question_free);
    double cut = 0, low = 0, high = 0;
    auto c = Argument(input, "threshold"), l = Argument(input, "low"), h = Argument(input, "high");
    auto number = [](const Value &value) {
        if (!value.type().IsNumeric()) throw InvalidInputException("usage.threshold: expected a number, received %s", value.type().ToString());
        return value.GetValue<double>();
    };
    if (!c.IsNull()) cut = number(c);
    if (!l.IsNull()) low = number(l);
    if (!h.IsNull()) high = number(h);
    Checked(thinkthen_sql_question_set_threshold(question.get(), c.IsNull() ? nullptr : &cut, l.IsNull() ? nullptr : &low, h.IsNull() ? nullptr : &high, &error.value), error);
    return question;
}
unique_ptr<LogicalOperator> Bind(ClientContext &, TableFunctionBindInput &input, TableIndex index, vector<Identifier> &names) {
    auto bind = make_uniq<Binding>();
    bind->function = input.table_function.GetName().GetIdentifierName().substr(10);
    bind->record = DConstants::INVALID_INDEX;
    auto record = Argument(input, "record");
    TextArgument("record", record);
    auto record_name = record.IsNull() ? "record" : record.GetValue<string>();
    for (idx_t i = 0; i < input.input_table_names.size(); ++i) {
        if (input.input_table_names[i] == Identifier(record_name)) bind->record = i;
    }
    if (bind->record == DConstants::INVALID_INDEX)
        throw InvalidInputException("usage.sql-selection: record column %s was not found; select a VARCHAR or JSON column with record := 'column_name'", record_name);
    if (input.input_table_types[bind->record].id() != LogicalTypeId::VARCHAR)
        throw InvalidInputException("usage.sql-selection: record column %s expects VARCHAR or JSON, received %s", record_name, input.input_table_types[bind->record].ToString());
    bind->json = input.input_table_types[bind->record] == LogicalType::JSON();
    bind->question = Admit(input, *bind);
    auto name = Argument(input, "name");
    TextArgument("name", name);
    bind->has_name = !name.IsNull();
    if (bind->has_name) bind->name = name.GetValue<string>();
    auto answer = bind->function == "annotate" ? AnnotationType(yyjson_doc_get_root(Parse(Encode(Argument(input, "toolbox", 1))).get()), true) : AnswerType(bind->function);
    auto detail = bind->function == "annotate" ? AnnotationType(yyjson_doc_get_root(Parse(Encode(Argument(input, "toolbox", 1))).get()), false) : DetailType(bind->function);
    names = {"position", "id", "answer", "probability", "status", "error_code", "error_message", "error_http_status", "source", "detail"};
    vector<LogicalType> types{LogicalTypeId::BIGINT, LogicalType::JSON(), answer, LogicalTypeId::DOUBLE, StatusType(), CodeType(), LogicalTypeId::VARCHAR, LogicalTypeId::BIGINT, SourceType(), detail};
    auto failures = Argument(input, "failures");
    if (!failures.IsNull()) {
        TextArgument("failures", failures);
        auto value = failures.GetValue<string>();
        if (value != "row") throw InvalidInputException("usage.option: failures expects 'row', received %s", value);
        bind->failure_rows = true;
    }
    case_insensitive_set_t reserved;
    for (auto &name : names) reserved.insert(name.GetIdentifierName());
    for (auto &name : input.input_table_names) if (!reserved.insert(name.GetIdentifierName()).second) throw InvalidInputException("usage.sql-selection: duplicate or reserved input column %s; alias it, for example id AS doc_id", name);
    names.insert(names.begin(), input.input_table_names.begin(), input.input_table_names.end());
    types.insert(types.begin(), input.input_table_types.begin(), input.input_table_types.end());
    bind->types = types;
    bind->identity = bind->function + bind->name;
    for (idx_t i = 1; i < input.inputs.size(); ++i) bind->identity += Encode(input.inputs[i]);
    for (auto &value : input.named_parameters) bind->identity += value.first.GetIdentifierName() + Encode(value.second);
    auto fn = input.table_function; fn.bind_operator = nullptr;
    auto get = make_uniq<LogicalGet>(index, std::move(fn), std::move(bind), types, names);
    get->input_table_types = input.input_table_types; get->input_table_names = input.input_table_names;
    for (idx_t i = 0; i < types.size(); ++i) get->AddColumnId(i);
    return make_uniq<LogicalSequential>(std::move(get));
}
void Load(ExtensionLoader &loader) {
    if (!thinkthen_abi_compatible(THINKTHEN_ABI_VERSION, thinkthen_abi_version()))
        throw InvalidInputException("%s: %s", THINKTHEN_ABI_ERROR_CODE, THINKTHEN_ABI_ERROR_MESSAGE);
    loader.RegisterType("thinkthen_status", StatusType());
    loader.RegisterType("thinkthen_error_code", CodeType());
    auto &config = DBConfig::GetConfig(loader.GetDatabaseInstance());
    TT_REGISTER_SETTINGS
    for (uint32_t id = 1; id <= 10; ++id) {
        string function = FunctionName(id);
        bool wording = HasArgument(function, "question");
        auto list = RequiredList(function);
        // The engine admits rule fields, including omitted reads/either. A
        // fixed struct signature would require those optional fields in DuckDB 2.
        {
            vector<LogicalType> args{LogicalTypeId::TABLE}; if (wording) args.push_back(LogicalType::ANY);
            TableFunction fn(Identifier("thinkthen_" + function), args, nullptr);
            if (!list.empty()) fn.GetSignature().AddKeywordOnly(Identifier(list), LogicalType::ANY, Value());
            fn.GetSignature().AddKeywordOnly(Identifier("name"), LogicalType::ANY, Value());
            fn.GetSignature().AddKeywordOnly(Identifier("record"), LogicalType::ANY, Value());
            fn.GetSignature().AddKeywordOnly(Identifier("field"), LogicalType::ANY, Value());
            if (HasArgument(function, "threshold")) fn.GetSignature().AddKeywordOnly(Identifier("threshold"), LogicalType::ANY, Value());
            if (HasArgument(function, "low")) { fn.GetSignature().AddKeywordOnly(Identifier("low"), LogicalType::ANY, Value()); fn.GetSignature().AddKeywordOnly(Identifier("high"), LogicalType::ANY, Value()); }
            if (HasArgument(function, "top")) fn.GetSignature().AddKeywordOnly(Identifier("top"), LogicalType::ANY, Value());
            if (function == "decide" || function == "filter") {
                fn.GetSignature().AddKeywordOnly(Identifier("yes_means"), LogicalType::ANY, Value());
                fn.GetSignature().AddKeywordOnly(Identifier("no_means"), LogicalType::ANY, Value());
            }
            if (function == "choose" || function == "tag" || function == "score" || function == "rank")
                fn.GetSignature().AddKeywordOnly(Identifier("descriptions"), LogicalType::ANY, Value());
            if (function == "filter") fn.GetSignature().AddKeywordOnly(Identifier("failures"), LogicalType::ANY, Value());
            fn.bind_operator = Bind; fn.init_global = Init;
            fn.init_local = [](ExecutionContext &, TableFunctionInitInput &, GlobalTableFunctionState *) -> unique_ptr<LocalTableFunctionState> {
                return make_uniq<LocalInvocation>();
            }; fn.in_out_function = Run; fn.in_out_function_final = Final;
            CreateTableFunctionInfo info(std::move(fn));
            FunctionDescription description;
            description.description = FunctionHelp(function);
            description.examples.push_back(FunctionExample(function));
            info.descriptions.push_back(std::move(description));
            loader.RegisterFunction(std::move(info));
        }
    }
}
