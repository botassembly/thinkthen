using System.Text;
using System.Text.Json;
using System.Text.Json.Nodes;
namespace ThinkThen;
/// <summary>An immutable, admitted question with a typed answer.</summary>
public abstract class Question<TAnswer,TDetail>
{
    internal Question(JsonObject definition, QuestionHandle? handle = null)
    { Definition = (JsonObject)definition.DeepClone(); using var admitted = handle ?? Admission.Admit(Definition); }
    internal virtual TAnswer ConvertAnswer(object? value) => (TAnswer)value!;
    internal JsonObject Definition { get; }
    /// <summary>Identifies one of the ten question functions.</summary>
    public Function Function => Json.Function(Definition["function"]!.GetValue<string>());
    private protected abstract void Closed();
}
internal static class Admission
{
    internal static T Required<T>(T? value) where T : class =>
        value ?? throw new ArgumentNullException(nameof(value));
    internal static JsonObject Named<T>(IReadOnlyDictionary<string, T> questions) where T : ToolboxQuestion =>
        new(Required(questions).Select(pair => KeyValuePair.Create<string, JsonNode?>(
            Json.HostText(pair.Key), Required(pair.Value).Definition.DeepClone())));
    internal static byte[] Bytes(JsonNode value) => Encoding.UTF8.GetBytes(value.ToJsonString());
    internal static QuestionHandle Admit(JsonObject value)
    {
        byte[] bytes = Bytes(value);
        NativeError error = NativeError.Empty;
        int result = Native.QuestionFromJson(bytes, (nuint)bytes.Length, out nint pointer, ref error);
        try { Json.Check(result, error); return new(pointer); }
        catch { if (pointer != 0) Native.QuestionFree(pointer); throw; }
    }
    internal static JsonObject Build(Function function, string? text, JsonNode? domain, string? domainKey,
        Pointer? field = null, Threshold? threshold = null, int? top = null, string? yesMeans = null, string? noMeans = null, IReadOnlyDictionary<string, string>? descriptions = null)
    {
        JsonObject value = new() { ["function"] = Contract.FunctionName((uint)function) };
        if (text is not null) value["question"] = Json.HostText(text);
        if (yesMeans is not null) value["yes_means"] = Json.HostText(yesMeans);
        if (noMeans is not null) value["no_means"] = Json.HostText(noMeans);
        if (domain is JsonArray options && descriptions is not null)
        {
            var remaining = new JsonObject(descriptions.Select(pair => KeyValuePair.Create<string, JsonNode?>(
                Json.HostText(pair.Key), JsonValue.Create(Json.HostText(pair.Value)))));
            for (int index = 0; index < options.Count; index++)
            {
                string name = options[index]!.GetValue<string>();
                if (!remaining.TryGetPropertyValue(name, out JsonNode? description)) continue;
                options[index] = new JsonObject { ["name"] = name, ["description"] = description?.DeepClone() };
                remaining.Remove(name);
            }
            // Preserve entries with no canonical option for shared unknown-field admission.
            if (remaining.Count > 0) value["descriptions"] = remaining;
        }
        if (domainKey is not null) value[domainKey] = domain;
        if (field is not null) value["field"] = field.Value;
        if (threshold is not null)
        {
            using var stream = new MemoryStream();
            using (var writer = new Utf8JsonWriter(stream)) threshold.Write(writer);
            value["threshold"] = JsonNode.Parse(stream.ToArray());
        }
        if (top is not null) value["top"] = top.Value;
        return value;
    }
    internal static JsonArray Strings(IReadOnlyList<string> values) =>
        new(Required(values).Select(value => (JsonNode?)JsonValue.Create(Json.HostText(value))).ToArray());
    internal static JsonArray Rules(IReadOnlyList<RelationRule> values) => new(Required(values).Select(rule => Rule(Required(rule))).ToArray());
    private static JsonNode Rule(RelationRule rule) => new JsonObject
    { ["source"] = Json.HostText(rule.Source), ["target"] = Json.HostText(rule.Target), ["relation"] = Json.HostText(rule.Relation),
      ["reads"] = rule.Reads is null ? null : Json.HostText(rule.Reads), ["either"] = rule.Either };
}
/// <summary>An immutable, admitted decide question.</summary>
public sealed class DecideQuestion : Question<bool,NoDetail>
{
    private DecideQuestion(JsonObject definition, QuestionHandle? handle = null) : base(definition, handle) { }
    private protected override void Closed() { }
    internal static DecideQuestion Admit(JsonObject value, QuestionHandle handle) => new(value, handle);
    /// <summary>Admits a question definition through the shared engine.</summary>
    public static DecideQuestion Create(string question, DecideOptions? options = null, CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        ArgumentNullException.ThrowIfNull(question);
        return new(Admission.Build(Function.Decide, question, null, null, options?.Field, options?.Threshold, null, options?.YesMeans, options?.NoMeans));
    }
    /// <summary>Gets the question text.</summary>
    public string Text => Definition["question"]!.GetValue<string>();
}
/// <summary>An immutable, admitted choose question.</summary>
public sealed class ChooseQuestion : Question<string,LabelsDetail>
{
    /// <summary>Provides Create.</summary>
    public static ChooseQuestion<T> Create<T>(string question,IReadOnlyList<T> choices,ChooseOptions<T>? options=null,CancellationToken cancellationToken=default) where T:struct,Enum
    {
        cancellationToken.ThrowIfCancellationRequested();
        var words=choices.Select(Domain.Member).ToArray();
        return new(Admission.Build(Function.Choose,question,Admission.Strings(words),"choices",options?.Field,options?.Threshold,null,descriptions:Domain.Descriptions(options?.Descriptions)));
    }

    private ChooseQuestion(JsonObject definition, QuestionHandle? handle = null) : base(definition, handle) { }
    private protected override void Closed() { }
    internal static ChooseQuestion Admit(JsonObject value, QuestionHandle handle) => new(value, handle);
    /// <summary>Admits a question definition through the shared engine.</summary>
    public static ChooseQuestion Create(string question, IReadOnlyList<string> choices, ChooseOptions? options = null, CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        ArgumentNullException.ThrowIfNull(question);
        return new(Admission.Build(Function.Choose, question, Admission.Strings(choices), "choices", options?.Field, options?.Threshold, null, descriptions: options?.Descriptions));
    }
    /// <summary>Gets the question text.</summary>
    public string Text => Definition["question"]!.GetValue<string>();
    /// <summary>Gets choice names in the question's order.</summary>
    public ValueList<string> Choices => Definition["choices"]!.AsArray().Select(value => value is JsonObject option ? option["name"]!.GetValue<string>() : value!.GetValue<string>()).ToValueList();
}
/// <summary>An immutable, admitted tag question.</summary>
public sealed class TagQuestion : Question<ValueList<string>,LabelsDetail>
{
    /// <summary>Provides Create.</summary>
    public static TagQuestion<T> Create<T>(string question,IReadOnlyList<T> labels,TagOptions<T>? options=null,CancellationToken cancellationToken=default) where T:struct,Enum
    {
        cancellationToken.ThrowIfCancellationRequested();
        var words=labels.Select(Domain.Member).ToArray();
        return new(Admission.Build(Function.Tag,question,Admission.Strings(words),"labels",options?.Field,options?.Threshold,null,descriptions:Domain.Descriptions(options?.Descriptions)));
    }

    private TagQuestion(JsonObject definition, QuestionHandle? handle = null) : base(definition, handle) { }
    private protected override void Closed() { }
    internal static TagQuestion Admit(JsonObject value, QuestionHandle handle) => new(value, handle);
    /// <summary>Admits a question definition through the shared engine.</summary>
    public static TagQuestion Create(string question, IReadOnlyList<string> labels, TagOptions? options = null, CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        ArgumentNullException.ThrowIfNull(question);
        return new(Admission.Build(Function.Tag, question, Admission.Strings(labels), "labels", options?.Field, options?.Threshold, null, descriptions: options?.Descriptions));
    }
    /// <summary>Gets the question text.</summary>
    public string Text => Definition["question"]!.GetValue<string>();
    /// <summary>Gets label names in the question's order.</summary>
    public ValueList<string> Labels => Definition["labels"]!.AsArray().Select(value => value is JsonObject option ? option["name"]!.GetValue<string>() : value!.GetValue<string>()).ToValueList();
}
/// <summary>An immutable, admitted score question.</summary>
public sealed class ScoreQuestion : Question<double,LevelsDetail>
{
    /// <summary>Provides Create.</summary>
    public static ScoreQuestion Create<T>(string question,IReadOnlyList<T> levels,ScoreOptions<T>? options=null,CancellationToken cancellationToken=default) where T:struct,Enum
    {
        cancellationToken.ThrowIfCancellationRequested();
        var words=levels.Select(Domain.Member).ToArray();
        return new(Admission.Build(Function.Score,question,Admission.Strings(words),"levels",options?.Field,null,null,descriptions:Domain.Descriptions(options?.Descriptions)));
    }

    private ScoreQuestion(JsonObject definition, QuestionHandle? handle = null) : base(definition, handle) { }
    private protected override void Closed() { }
    internal static ScoreQuestion Admit(JsonObject value, QuestionHandle handle) => new(value, handle);
    /// <summary>Admits a question definition through the shared engine.</summary>
    public static ScoreQuestion Create(string question, IReadOnlyList<string> levels, ScoreOptions? options = null, CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        ArgumentNullException.ThrowIfNull(question);
        return new(Admission.Build(Function.Score, question, Admission.Strings(levels), "levels", options?.Field, null, null, descriptions: options?.Descriptions));
    }
    /// <summary>Gets the question text.</summary>
    public string Text => Definition["question"]!.GetValue<string>();
    /// <summary>Gets level names in the question's order.</summary>
    public ValueList<string> Levels => Definition["levels"]!.AsArray().Select(value => value is JsonObject option ? option["name"]!.GetValue<string>() : value!.GetValue<string>()).ToValueList();
}
/// <summary>An immutable, admitted filter question.</summary>
public sealed class FilterQuestion : Question<bool,NoDetail>
{
    private FilterQuestion(JsonObject definition, QuestionHandle? handle = null) : base(definition, handle) { }
    private protected override void Closed() { }
    internal static FilterQuestion Admit(JsonObject value, QuestionHandle handle) => new(value, handle);
    /// <summary>Admits a question definition through the shared engine.</summary>
    public static FilterQuestion Create(string question, FilterOptions? options = null, CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        ArgumentNullException.ThrowIfNull(question);
        return new(Admission.Build(Function.Filter, question, null, null, options?.Field, options?.Threshold, null, options?.YesMeans, options?.NoMeans));
    }
    /// <summary>Gets the question text.</summary>
    public string Text => Definition["question"]!.GetValue<string>();
}
/// <summary>An immutable, admitted rank question.</summary>
public sealed class RankQuestion : Question<long,LevelsDetail>
{
    /// <summary>Provides Create.</summary>
    public static RankQuestion Create<T>(string question,IReadOnlyList<T> levels,RankOptions<T>? options=null,CancellationToken cancellationToken=default) where T:struct,Enum
    {
        cancellationToken.ThrowIfCancellationRequested();
        var words=levels.Select(Domain.Member).ToArray();
        return new(Admission.Build(Function.Rank,question,Admission.Strings(words),"levels",options?.Field,null,options?.Top,descriptions:Domain.Descriptions(options?.Descriptions)));
    }

    private RankQuestion(JsonObject definition, QuestionHandle? handle = null) : base(definition, handle) { }
    private protected override void Closed() { }
    internal static RankQuestion Admit(JsonObject value, QuestionHandle handle) => new(value, handle);
    /// <summary>Admits a question definition through the shared engine.</summary>
    public static RankQuestion Create(string question, IReadOnlyList<string> levels, RankOptions? options = null, CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        ArgumentNullException.ThrowIfNull(question);
        return new(Admission.Build(Function.Rank, question, Admission.Strings(levels), "levels", options?.Field, null, options?.Top, descriptions: options?.Descriptions));
    }
    /// <summary>Gets the question text.</summary>
    public string Text => Definition["question"]!.GetValue<string>();
    /// <summary>Gets level names in the question's order.</summary>
    public ValueList<string> Levels => Definition["levels"]!.AsArray().Select(value => value is JsonObject option ? option["name"]!.GetValue<string>() : value!.GetValue<string>()).ToValueList();
}
/// <summary>An immutable, admitted find question.</summary>
public sealed class FindQuestion : Question<TextSpan?,NoDetail>
{
    private FindQuestion(JsonObject definition, QuestionHandle? handle = null) : base(definition, handle) { }
    private protected override void Closed() { }
    internal static FindQuestion Admit(JsonObject value, QuestionHandle handle) => new(value, handle);
    /// <summary>Admits a question definition through the shared engine.</summary>
    public static FindQuestion Create(string question, FindOptions? options = null, CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        ArgumentNullException.ThrowIfNull(question);
        return new(Admission.Build(Function.Find, question, null, null, options?.Field, options?.Threshold, null));
    }
    /// <summary>Gets the question text.</summary>
    public string Text => Definition["question"]!.GetValue<string>();
}
/// <summary>An immutable, admitted recognize question.</summary>
public sealed class RecognizeQuestion : Question<ValueList<Entity>,RecognizeDetail>
{
    private RecognizeQuestion(JsonObject definition, QuestionHandle? handle = null) : base(definition, handle) { }
    private protected override void Closed() { }
    internal static RecognizeQuestion Admit(JsonObject value, QuestionHandle handle) => new(value, handle);
    /// <summary>Admits a question definition through the shared engine.</summary>
    public static RecognizeQuestion Create(IReadOnlyList<string> kinds, RecognizeOptions? options = null, CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        return new(Admission.Build(Function.Recognize, null, Admission.Strings(kinds), "kinds", options?.Field, options?.Threshold, null));
    }
    /// <summary>Gets the requested recognized kinds in question order.</summary>
    public ValueList<string> Kinds => Definition["kinds"]!.AsArray().Select(value => value is JsonObject option ? option["name"]!.GetValue<string>() : value!.GetValue<string>()).ToValueList();
}
/// <summary>An immutable, admitted relate question.</summary>
public sealed class RelateQuestion : Question<ValueList<FoundRelation>,RelationDetail>
{
    private RelateQuestion(JsonObject definition, QuestionHandle? handle = null) : base(definition, handle) { }
    private protected override void Closed() { }
    internal static RelateQuestion Admit(JsonObject value, QuestionHandle handle) => new(value, handle);
    /// <summary>Admits a question definition through the shared engine.</summary>
    public static RelateQuestion Create(IReadOnlyList<RelationRule> rules, RelateOptions? options = null, CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        return new(Admission.Build(Function.Relate, null, Admission.Rules(rules), "rules", options?.Field, options?.Threshold, null));
    }
}
/// <summary>An immutable, admitted annotate question.</summary>
public sealed class AnnotateQuestion : Question<object,NoDetail>
{
    private AnnotateQuestion(JsonObject definition, QuestionHandle? handle = null) : base(definition, handle) { }
    private protected override void Closed() { }
    internal static AnnotateQuestion Admit(JsonObject value, QuestionHandle? handle = null) => new(value, handle);
    /// <summary>Admits a question definition through the shared engine.</summary>
    public static AnnotateQuestion Create(AnnotationQuestions questions, AnnotateOptions? options = null, CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        return new(Admission.Build(Function.Annotate, null, Admission.Required(questions).Definition.DeepClone(), "questions", options?.Field, null, null));
    }
}
/// <summary>An immutable collection of named, admitted questions.</summary>
public sealed class Toolbox
{
    internal JsonObject Definition { get; }
    private readonly string json;
    private Toolbox(JsonObject definition, string json) { Definition = definition; this.json=json; }
    /// <summary>Admits bare named question definitions.</summary>
    public static Toolbox Parse(string json, CancellationToken cancellationToken = default) => Admit(json, false, cancellationToken);
    /// <summary>Loads a saved toolbox JSON value.</summary>
    public static Toolbox FromJson(string json, CancellationToken cancellationToken = default) => Admit(json, true, cancellationToken);
    private static Toolbox Admit(string json, bool saved, CancellationToken cancellationToken)
    {
        cancellationToken.ThrowIfCancellationRequested();
        byte[] bytes = Encoding.UTF8.GetBytes(Json.HostText(json));
        NativeError error = NativeError.Empty;
        nint pointer;
        int result = saved
            ? Native.ToolboxFromJson(bytes, (nuint)bytes.Length, out pointer, ref error)
            : Native.ToolboxFromDefinitionsJson(bytes, (nuint)bytes.Length, out pointer, ref error);
        using ToolboxHandle owner = new(pointer);
        try {
            Json.Check(result, error);
            error = NativeError.Empty;
            result = Native.ToolboxDefinitionsJson(owner, out Buffer output, ref error);
            try {
                Json.Check(result,error);
                var definition=JsonNode.Parse(Native.Copy(output))!.AsObject();
                error=NativeError.Empty;
                int savedResult=Native.ToolboxToJson(owner,out Buffer savedOutput,ref error);
                try { Json.Check(savedResult,error); return new(definition,Encoding.UTF8.GetString(Native.Copy(savedOutput))); }
                finally { Native.BufferFree(ref savedOutput); }
            }
            finally { Native.BufferFree(ref output); }
        }
        catch(UsageException failure) when(failure.ErrorCode==ErrorCode.UsageThreshold && FilterBandContext(json,saved))
        { throw new UsageException(new(failure.ErrorCode,failure.Message+"; in C# use DecideStreamAsync for a band",failure.HttpStatus),failure); }
    }
    private static bool FilterBandContext(string json,bool saved)
    {
        try
        {
            var definitions=JsonNode.Parse(json)!;
            if(saved) definitions=definitions["entries"]!;
            foreach(var entry in definitions.AsObject())
            {
                var question=saved ? entry.Value?["question"] : entry.Value;
                if(question?["function"]?.GetValue<string>()=="filter" && question["threshold"] is JsonObject threshold && threshold.ContainsKey("low") && threshold.ContainsKey("high")) return true;
            }
        }
        catch(Exception cause) when(cause is JsonException or InvalidOperationException) { /* The engine already owns malformed input diagnostics. */ }
        return false;
    }
    /// <summary>Loads a saved toolbox from a UTF-8 file.</summary>
    public static Toolbox Load(string path) => FromJson(File.ReadAllText(path));
    /// <summary>Returns the saved toolbox JSON value.</summary>
    public string ToJson() => json;
    private ToolboxHandle OpenHandle()
    {
        byte[] bytes=Encoding.UTF8.GetBytes(json); var error=NativeError.Empty;
        int result=Native.ToolboxFromJson(bytes,(nuint)bytes.Length,out nint pointer,ref error);
        try { Json.Check(result,error); return new(pointer); }
        catch { if(pointer!=0) Native.ToolboxFree(pointer); throw; }
    }
    internal (JsonObject Definition, QuestionHandle Handle) Select(string name, Function function)
    {
        using var handle=OpenHandle();
        byte[] key = Encoding.UTF8.GetBytes(Json.HostText(name));
        NativeError error = NativeError.Empty;
        int result = Native.ToolboxSelectAs(handle, key, (nuint)key.Length, (uint)function, out nint pointer, ref error);
        var question = new QuestionHandle(pointer);
        try { Json.Check(result, error); return ((JsonObject)Definition[name]!.DeepClone(), question); }
        catch { question.Dispose(); throw; }
    }
    /// <summary>Selects the named decide question.</summary>
    public DecideQuestion Decide(string name) { var selected = Select(name, Function.Decide); return DecideQuestion.Admit(selected.Definition, selected.Handle); }
    /// <summary>Selects the named choose question.</summary>
    public ChooseQuestion Choose(string name) { var selected = Select(name, Function.Choose); return ChooseQuestion.Admit(selected.Definition, selected.Handle); }
    /// <summary>Selects the named tag question.</summary>
    public TagQuestion Tag(string name) { var selected = Select(name, Function.Tag); return TagQuestion.Admit(selected.Definition, selected.Handle); }
    /// <summary>Selects the named score question.</summary>
    public ScoreQuestion Score(string name) { var selected = Select(name, Function.Score); return ScoreQuestion.Admit(selected.Definition, selected.Handle); }
    /// <summary>Selects the named filter question.</summary>
    public FilterQuestion Filter(string name) { var selected = Select(name, Function.Filter); return FilterQuestion.Admit(selected.Definition, selected.Handle); }
    /// <summary>Selects the named rank question.</summary>
    public RankQuestion Rank(string name) { var selected = Select(name, Function.Rank); return RankQuestion.Admit(selected.Definition, selected.Handle); }
    /// <summary>Selects the named find question.</summary>
    public FindQuestion Find(string name) { var selected = Select(name, Function.Find); return FindQuestion.Admit(selected.Definition, selected.Handle); }
    /// <summary>Selects the named recognize question.</summary>
    public RecognizeQuestion Recognize(string name) { var selected = Select(name, Function.Recognize); return RecognizeQuestion.Admit(selected.Definition, selected.Handle); }
    /// <summary>Selects the named relate question.</summary>
    public RelateQuestion Relate(string name) { var selected = Select(name, Function.Relate); return RelateQuestion.Admit(selected.Definition, selected.Handle); }
    /// <summary>Provides Choose.</summary>
    public ChooseQuestion<T> Choose<T>(string name) where T:struct,Enum { var selected=Select(name,Function.Choose); using(selected.Handle) return new(selected.Definition); }
    /// <summary>Provides Tag.</summary>
    public TagQuestion<T> Tag<T>(string name) where T:struct,Enum { var selected=Select(name,Function.Tag); using(selected.Handle) return new(selected.Definition); }
    /// <summary>Provides Score.</summary>
    public ScoreQuestion Score<T>(string name) where T:struct,Enum { var question=Score(name); foreach(var word in question.Levels) _=Domain<T>.Read(word); return question; }
    /// <summary>Provides Rank.</summary>
    public RankQuestion Rank<T>(string name) where T:struct,Enum { var question=Rank(name); foreach(var word in question.Levels) _=Domain<T>.Read(word); return question; }
    /// <summary>Admits a question definition through the shared engine.</summary>
    public static Toolbox Create(IReadOnlyDictionary<string, ToolboxQuestion> questions) =>
        Parse(Admission.Named(questions).ToJsonString());
}
/// <summary>A question definition stored in a toolbox.</summary>
public abstract record ToolboxQuestion
{
    internal ToolboxQuestion(JsonObject definition) => Definition = definition;
    internal JsonObject Definition { get; }
    private protected abstract void Closed();
    /// <summary>A saved filter definition, which cannot be included in a form.</summary>
    public sealed record Filter : ToolboxQuestion
    {
        /// <summary>Snapshots an admitted filter question.</summary>
        public Filter(FilterQuestion question):base(Admission.Required(question).Definition){}
        private protected override void Closed(){}
    }
    /// <summary>A typed rank question definition.</summary>
    public sealed record Rank : ToolboxQuestion
    {
        /// <summary>Creates an annotation question definition from the supplied admitted question.</summary>
        public Rank(RankQuestion question) : base(Admission.Required(question).Definition) { }
        private protected override void Closed() { }
    }
}
/// <summary>A question definition included in an annotation form.</summary>
public abstract record AnnotationQuestion : ToolboxQuestion
{
    internal AnnotationQuestion(JsonObject definition) : base(definition) { }
    /// <summary>A typed decide question definition.</summary>
    public sealed record Decide : AnnotationQuestion
    {
        /// <summary>Include the supplied decide question in an annotation form.</summary>
        public Decide(DecideQuestion question) : base(Admission.Required(question).Definition) { }
        private protected override void Closed() { }
    }
    /// <summary>A typed choose question definition.</summary>
    public sealed record Choose : AnnotationQuestion
    {
        /// <summary>Include the supplied choose question in an annotation form.</summary>
        public Choose(ChooseQuestion question) : base(Admission.Required(question).Definition) { }
        private protected override void Closed() { }
    }
    /// <summary>A typed tag question definition.</summary>
    public sealed record Tag : AnnotationQuestion
    {
        /// <summary>Include the supplied tag question in an annotation form.</summary>
        public Tag(TagQuestion question) : base(Admission.Required(question).Definition) { }
        private protected override void Closed() { }
    }
    /// <summary>A typed score question definition.</summary>
    public sealed record Score : AnnotationQuestion
    {
        /// <summary>Include the supplied score question in an annotation form.</summary>
        public Score(ScoreQuestion question) : base(Admission.Required(question).Definition) { }
        private protected override void Closed() { }
    }
    /// <summary>A typed find question definition.</summary>
    public sealed record Find : AnnotationQuestion
    {
        /// <summary>Include the supplied find question in an annotation form.</summary>
        public Find(FindQuestion question) : base(Admission.Required(question).Definition) { }
        private protected override void Closed() { }
    }
    /// <summary>A typed recognize question definition.</summary>
    public sealed record Recognize : AnnotationQuestion
    {
        /// <summary>Include the supplied recognize question in an annotation form.</summary>
        public Recognize(RecognizeQuestion question) : base(Admission.Required(question).Definition) { }
        private protected override void Closed() { }
    }
    /// <summary>A typed relate question definition.</summary>
    public sealed record Relate : AnnotationQuestion
    {
        /// <summary>Include the supplied relate question in an annotation form.</summary>
        public Relate(RelateQuestion question) : base(Admission.Required(question).Definition) { }
        private protected override void Closed() { }
    }
}
/// <summary>A form of named annotation questions. Finish adding entries before passing the form to a client.</summary>
public sealed class AnnotationQuestions : IEnumerable<string>
{
    private readonly JsonObject definition=new();
    private bool frozen;
    internal JsonObject Definition { get { frozen=true; return (JsonObject)definition.DeepClone(); } }
    /// <summary>Provides AnnotationQuestions.</summary>
    public AnnotationQuestions() {}
    private AnnotationQuestions(JsonObject value)
    {
        definition=(JsonObject)value.DeepClone();
        using var admitted=Admission.Admit(Admission.Build(Function.Annotate,null,definition.DeepClone(),"questions"));
    }
    /// <summary>Provides Count.</summary>
    public int Count => definition.Count;
    private AnnotationQuestions AddDefinition(string name,JsonObject question)
    {
        if(frozen) throw new InvalidOperationException("Add questions before using the form.");
        definition.Add(Json.HostText(name),question.DeepClone()); return this;
    }
    /// <summary>Provides Add.</summary>
    public AnnotationQuestions Add(string name,AnnotationQuestion question) => AddDefinition(name,question.Definition);
    /// <summary>Provides Add.</summary>
    public AnnotationQuestions Add(string name,DecideQuestion question) => AddDefinition(name,question.Definition);
    /// <summary>Provides Add.</summary>
    public AnnotationQuestions Add(string name,ChooseQuestion question) => AddDefinition(name,question.Definition);
    /// <summary>Provides Add.</summary>
    public AnnotationQuestions Add<T>(string name,ChooseQuestion<T> question) where T:struct,Enum => AddDefinition(name,question.Definition);
    /// <summary>Provides Add.</summary>
    public AnnotationQuestions Add(string name,TagQuestion question) => AddDefinition(name,question.Definition);
    /// <summary>Provides Add.</summary>
    public AnnotationQuestions Add<T>(string name,TagQuestion<T> question) where T:struct,Enum => AddDefinition(name,question.Definition);
    /// <summary>Provides Add.</summary>
    public AnnotationQuestions Add(string name,ScoreQuestion question) => AddDefinition(name,question.Definition);
    /// <summary>Provides Add.</summary>
    public AnnotationQuestions Add(string name,FindQuestion question) => AddDefinition(name,question.Definition);
    /// <summary>Provides Add.</summary>
    public AnnotationQuestions Add(string name,RecognizeQuestion question) => AddDefinition(name,question.Definition);
    /// <summary>Provides Add.</summary>
    public AnnotationQuestions Add(string name,RelateQuestion question) => AddDefinition(name,question.Definition);
    /// <summary>Provides GetEnumerator.</summary>
    public IEnumerator<string> GetEnumerator() => definition.Select(pair=>pair.Key).GetEnumerator();
    System.Collections.IEnumerator System.Collections.IEnumerable.GetEnumerator()=>GetEnumerator();
    /// <summary>Provides Create.</summary>
    public static AnnotationQuestions Create(IReadOnlyDictionary<string,AnnotationQuestion> questions)=>new(Admission.Named(questions));
    /// <summary>Provides Parse.</summary>
    public static AnnotationQuestions Parse(string json,CancellationToken cancellationToken=default)
    { var toolbox=Toolbox.Parse(json,cancellationToken); return new(toolbox.Definition); }
}

/// <summary>A choose question whose answer is an enum member.</summary>
public sealed class ChooseQuestion<T> : Question<T,LabelsDetail> where T:struct,Enum
{
    internal ChooseQuestion(JsonObject definition):base(definition) { foreach(var word in Codecs.QuestionWords(definition,"choices")) _=Domain<T>.Read(word); }
    private protected override void Closed(){}
    internal override T ConvertAnswer(object? value)=>Domain<T>.Read((string)value!);
}
/// <summary>A tag question whose answers are enum members.</summary>
public sealed class TagQuestion<T> : Question<ValueList<T>,LabelsDetail> where T:struct,Enum
{
    internal TagQuestion(JsonObject definition):base(definition) { foreach(var word in Codecs.QuestionWords(definition,"labels")) _=Domain<T>.Read(word); }
    private protected override void Closed(){}
    internal override ValueList<T> ConvertAnswer(object? value)=>new(((ValueList<string>)value!).Select(Domain<T>.Read));
}
internal sealed class TypedAnnotateQuestion<T> : Question<Annotations<T>,NoDetail>
{
    internal TypedAnnotateQuestion(JsonObject definition):base(definition){}
    private protected override void Closed(){}
}
