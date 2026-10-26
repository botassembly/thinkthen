Value Argument(TableFunctionBindInput &input, const string &name, idx_t position = DConstants::INVALID_INDEX) {
    auto found = input.named_parameters.find(name);
    if (position < input.inputs.size()) {
        if (found != input.named_parameters.end()) throw BinderException("usage.question: duplicate %s", name);
        return input.inputs[position];
    }
    return found == input.named_parameters.end() ? Value() : found->second;
}
string RequiredList(const string &function) {
    if (function == "choose") return "choices";
    if (function == "tag") return "labels";
    if (function == "score" || function == "rank") return "levels";
    if (function == "recognize") return "kinds";
    if (function == "relate") return "rules";
    if (function == "annotate") return "toolbox";
    return "";
}
QuestionOwner Admit(TableFunctionBindInput &input, Binding &bind) {
    ErrorOwner error;
    thinkthen_question *raw = nullptr;
    auto list = RequiredList(bind.function);
    bool wording = bind.function != "recognize" && bind.function != "relate" && bind.function != "annotate";
    auto required = list.empty() ? Value() : Argument(input, list, wording ? 2 : 1);
    if (!list.empty() && required.IsNull()) throw BinderException("usage.question: %s is required", list);
    if (bind.function == "relate" || bind.function == "annotate") {
        string json = "{\"function\":" + Quote(bind.function) + "," + Quote(bind.function == "annotate" ? "questions" : "rules") + ":" + Encode(required) + "}";
        Checked(thinkthen_sql_question_from_json(Bytes(json), json.size(), &raw, &error.value), error);
    } else {
        if (wording && input.inputs[1].IsNull()) throw BinderException("usage.question: question cannot be SQL NULL");
        string text = wording ? input.inputs[1].GetValue<string>() : "";
        uint32_t function = 0;
        for (uint32_t id = 1; id <= 10; ++id) if (bind.function == FunctionName(id)) function = id;
        Checked(thinkthen_question_new(function, Bytes(text), text.size(), &raw, &error.value), error);
    }
    QuestionOwner question(raw, thinkthen_question_free);
    if (!required.IsNull() && bind.function != "relate" && bind.function != "annotate") {
        for (auto &item : ListValue::GetChildren(required)) {
            if (item.IsNull()) throw BinderException("usage.question: list members cannot be SQL NULL");
            auto text = item.GetValue<string>();
            auto add = list == "choices" ? thinkthen_question_add_choice : list == "labels" ? thinkthen_question_add_label : list == "kinds" ? thinkthen_question_add_kind : thinkthen_question_add_level;
            Checked(add(question.get(), Bytes(text), text.size(), &error.value), error);
        }
    }
    double cut = 0, low = 0, high = 0;
    auto c = Argument(input, "threshold"), l = Argument(input, "low"), h = Argument(input, "high");
    if (!c.IsNull()) cut = c.GetValue<double>();
    if (!l.IsNull()) low = l.GetValue<double>();
    if (!h.IsNull()) high = h.GetValue<double>();
    Checked(thinkthen_sql_question_set_threshold(question.get(), c.IsNull() ? nullptr : &cut, l.IsNull() ? nullptr : &low, h.IsNull() ? nullptr : &high, &error.value), error);
    auto evidence = Argument(input, "evidence");
    if (!evidence.IsNull()) {
        auto text = evidence.GetValue<string>(); bind.json = true;
        Checked(thinkthen_question_set_evidence(question.get(), Bytes(text), text.size(), &error.value), error);
    }
    auto top = Argument(input, "top");
    if (!top.IsNull()) Checked(thinkthen_question_set_top(question.get(), top.GetValue<uint64_t>(), &error.value), error);
    return question;
}
unique_ptr<LogicalOperator> Bind(ClientContext &, TableFunctionBindInput &input, idx_t index, vector<string> &names) {
    auto bind = make_uniq<Binding>();
    bind->function = input.table_function.name.substr(10);
    bind->record = DConstants::INVALID_INDEX;
    auto record = Argument(input, "record");
    auto record_name = record.IsNull() ? "record" : record.GetValue<string>();
    for (idx_t i = 0; i < input.input_table_names.size(); ++i) {
        if (StringUtil::CIEquals(input.input_table_names[i], record_name)) bind->record = i;
    }
    if (bind->record == DConstants::INVALID_INDEX || input.input_table_types[bind->record].id() != LogicalTypeId::VARCHAR)
        throw BinderException("usage.sql-selection: record must select a VARCHAR or JSON input column");
    bind->json = input.input_table_types[bind->record] == LogicalType::JSON();
    bind->question = Admit(input, *bind);
    auto name = Argument(input, "name");
    bind->has_name = !name.IsNull();
    if (bind->has_name) bind->name = name.GetValue<string>();
    auto answer = bind->function == "annotate" ? AnnotationType(yyjson_doc_get_root(Parse(Encode(Argument(input, "toolbox", 1))).get())) : AnswerType(bind->function);
    names = {"position", "id", "status", "probability", "error_code", "error_message", "error_http_status", "source", "detail", bind->function == "find" ? "found" : bind->function == "rank" ? "rank" : "answer"};
    vector<LogicalType> types{LogicalTypeId::UBIGINT, LogicalType::JSON(), StatusType(), LogicalTypeId::DOUBLE, CodeType(), LogicalTypeId::VARCHAR, LogicalTypeId::BIGINT, SourceType(), DetailType(bind->function), answer};
    if (bind->function == "filter") { names.push_back("kept"); types.push_back(LogicalTypeId::BOOLEAN); }
    case_insensitive_set_t reserved;
    for (auto &name : names) reserved.insert(name);
    for (auto &name : input.input_table_names) if (!reserved.insert(name).second) throw BinderException("usage.sql-selection: duplicate or reserved input column %s", name);
    names.insert(names.begin(), input.input_table_names.begin(), input.input_table_names.end());
    types.insert(types.begin(), input.input_table_types.begin(), input.input_table_types.end());
    bind->types = types;
    bind->identity = bind->function + bind->name;
    for (idx_t i = 1; i < input.inputs.size(); ++i) bind->identity += Encode(input.inputs[i]);
    for (auto &value : input.named_parameters) bind->identity += value.first + Encode(value.second);
    auto fn = input.table_function; fn.bind_operator = nullptr;
    auto get = make_uniq<LogicalGet>(index, std::move(fn), std::move(bind), types, names);
    get->input_table_types = input.input_table_types; get->input_table_names = input.input_table_names;
    for (idx_t i = 0; i < types.size(); ++i) get->AddColumnId(i);
    return make_uniq<LogicalSequential>(std::move(get));
}
void Load(ExtensionLoader &loader) {
    if (!thinkthen_abi_compatible(THINKTHEN_ABI_VERSION, thinkthen_abi_version()))
        throw InvalidInputException("%s: %s", THINKTHEN_ABI_ERROR_CODE, THINKTHEN_ABI_ERROR_MESSAGE);
    auto &config = DBConfig::GetConfig(loader.GetDatabaseInstance());
    config.AddExtensionOption("thinkthen_backend", "ThinkThen backend", BackendType(), Value("systemone").DefaultCastAs(BackendType()), Configure<0>, SetScope::SESSION);
    config.AddExtensionOption("thinkthen_model", "ThinkThen model", LogicalTypeId::VARCHAR, Value(LogicalTypeId::VARCHAR), Configure<1>, SetScope::SESSION);
    config.AddExtensionOption("thinkthen_batch", "ThinkThen batch", LogicalTypeId::UBIGINT, Value(LogicalTypeId::UBIGINT), Configure<2>, SetScope::SESSION);
    config.AddExtensionOption("thinkthen_max_retries", "ThinkThen retry count", LogicalTypeId::UBIGINT, Value(LogicalTypeId::UBIGINT), Configure<3>, SetScope::SESSION);
    config.AddExtensionOption("thinkthen_initial_backoff_ms", "ThinkThen retry backoff", LogicalTypeId::UBIGINT, Value(LogicalTypeId::UBIGINT), Configure<4>, SetScope::SESSION);
    config.AddExtensionOption("thinkthen_attempt_timeout_ms", "ThinkThen attempt timeout", LogicalTypeId::UBIGINT, Value(LogicalTypeId::UBIGINT), Configure<5>, SetScope::SESSION);
    auto rule = LogicalType::STRUCT({{"name", LogicalTypeId::VARCHAR}, {"source", LogicalTypeId::VARCHAR}, {"target", LogicalTypeId::VARCHAR}, {"reads", LogicalTypeId::VARCHAR}, {"either", LogicalTypeId::BOOLEAN}});
    for (uint32_t id = 1; id <= 10; ++id) {
        string function = FunctionName(id);
        bool wording = function != "recognize" && function != "relate" && function != "annotate";
        auto list = RequiredList(function);
        auto list_type = function == "relate" ? LogicalType::LIST(rule) : function == "annotate" ? LogicalType::JSON() : LogicalType::LIST(LogicalTypeId::VARCHAR);
        {
            vector<LogicalType> args{LogicalTypeId::TABLE}; if (wording) args.push_back(LogicalTypeId::VARCHAR);
            TableFunction fn("thinkthen_" + function, args, nullptr);
            if (!list.empty()) fn.named_parameters[list] = list_type;
            fn.named_parameters["name"] = LogicalTypeId::VARCHAR;
            fn.named_parameters["record"] = LogicalTypeId::VARCHAR;
            fn.named_parameters["evidence"] = LogicalTypeId::VARCHAR;
            if (function != "score" && function != "rank" && function != "annotate") fn.named_parameters["threshold"] = LogicalTypeId::DOUBLE;
            if (function == "decide" || function == "choose" || function == "filter") { fn.named_parameters["low"] = LogicalTypeId::DOUBLE; fn.named_parameters["high"] = LogicalTypeId::DOUBLE; }
            if (function == "rank") fn.named_parameters["top"] = LogicalTypeId::UBIGINT;
            fn.bind_operator = Bind; fn.init_local = Init; fn.in_out_function = Run; fn.in_out_function_final = Final;
            loader.RegisterFunction(std::move(fn));
        }
    }
}
