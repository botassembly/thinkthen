using System.Text;
using System.Text.Json;
using System.Text.Json.Nodes;
namespace ThinkThen;
/// <summary>An owned, admitted question with typed answers and details.</summary>
public abstract class Question<TAnswer,TDetail> : IDisposable
{
    internal Question(JsonObject definition, QuestionHandle? handle = null)
    { Definition = definition; Handle = handle ?? Admission.Admit(definition); }
    internal QuestionHandle Handle { get; }
    /// <summary>Releases the owned native resource.</summary>
    public void Dispose() => Handle.Dispose();
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
        if (yesMeans is not null) value["true"] = Json.HostText(yesMeans);
        if (noMeans is not null) value["false"] = Json.HostText(noMeans);
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
    { ["source"] = Json.HostText(rule.Source), ["target"] = Json.HostText(rule.Target), ["relation"] = Json.HostText(rule.Name),
      ["reads"] = rule.Reads is null ? null : Json.HostText(rule.Reads), ["either"] = rule.Either };
}
/// <summary>An owned, admitted decide question.</summary>
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
/// <summary>An owned, admitted choose question.</summary>
public sealed class ChooseQuestion : Question<string,LabelsDetail>
{
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
/// <summary>An owned, admitted tag question.</summary>
public sealed class TagQuestion : Question<ValueList<string>,LabelsDetail>
{
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
/// <summary>An owned, admitted score question.</summary>
public sealed class ScoreQuestion : Question<double,LevelsDetail>
{
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
/// <summary>An owned, admitted filter question.</summary>
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
/// <summary>An owned, admitted rank question.</summary>
public sealed class RankQuestion : Question<long,LevelsDetail>
{
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
/// <summary>An owned, admitted find question.</summary>
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
/// <summary>An owned, admitted recognize question.</summary>
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
/// <summary>An owned, admitted relate question.</summary>
public sealed class RelateQuestion : Question<ValueList<Relation>,RelateDetail>
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
/// <summary>An owned, admitted annotate question.</summary>
public sealed class AnnotateQuestion : Question<ValueMap<string, AnnotationEntry>,NoDetail>
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
/// <summary>An owned collection of named, admitted questions.</summary>
public sealed class Toolbox : IDisposable
{
    internal JsonObject Definition { get; }
    private readonly ToolboxHandle handle;
    private Toolbox(JsonObject definition, ToolboxHandle handle) { Definition = definition; this.handle = handle; }
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
        ToolboxHandle owner = new(pointer);
        try {
            Json.Check(result, error);
            error = NativeError.Empty;
            result = Native.ToolboxDefinitionsJson(owner, out Buffer output, ref error);
            try { Json.Check(result, error); return new(JsonNode.Parse(Native.Copy(output))!.AsObject(), owner); }
            finally { Native.BufferFree(ref output); }
        }
        catch { owner.Dispose(); throw; }
    }
    /// <summary>Releases the owned native resource.</summary>
    public void Dispose() => handle.Dispose();
    /// <summary>Returns the saved toolbox JSON value.</summary>
    public string ToJson()
    {
        NativeError error = NativeError.Empty;
        int result = Native.ToolboxToJson(handle, out Buffer output, ref error);
        try { Json.Check(result, error); return Encoding.UTF8.GetString(Native.Copy(output)); }
        finally { Native.BufferFree(ref output); }
    }
    internal (JsonObject Definition, QuestionHandle Handle) Select(string name, Function function)
    {
        byte[] key = Encoding.UTF8.GetBytes(Json.HostText(name));
        NativeError error = NativeError.Empty;
        int result = Native.ToolboxSelectAs(handle, key, (nuint)key.Length, (uint)function, out nint pointer, ref error);
        var question = new QuestionHandle(pointer);
        try { Json.Check(result, error); return ((JsonObject)Definition[name]!.DeepClone(), question); }
        catch { question.Dispose(); throw; }
    }
    /// <summary>Selects the named decide question; dispose the returned question after use.</summary>
    public DecideQuestion Decide(string name) { var selected = Select(name, Function.Decide); return DecideQuestion.Admit(selected.Definition, selected.Handle); }
    /// <summary>Selects the named choose question; dispose the returned question after use.</summary>
    public ChooseQuestion Choose(string name) { var selected = Select(name, Function.Choose); return ChooseQuestion.Admit(selected.Definition, selected.Handle); }
    /// <summary>Selects the named tag question; dispose the returned question after use.</summary>
    public TagQuestion Tag(string name) { var selected = Select(name, Function.Tag); return TagQuestion.Admit(selected.Definition, selected.Handle); }
    /// <summary>Selects the named score question; dispose the returned question after use.</summary>
    public ScoreQuestion Score(string name) { var selected = Select(name, Function.Score); return ScoreQuestion.Admit(selected.Definition, selected.Handle); }
    /// <summary>Selects the named filter question; dispose the returned question after use.</summary>
    public FilterQuestion Filter(string name) { var selected = Select(name, Function.Filter); return FilterQuestion.Admit(selected.Definition, selected.Handle); }
    /// <summary>Selects the named rank question; dispose the returned question after use.</summary>
    public RankQuestion Rank(string name) { var selected = Select(name, Function.Rank); return RankQuestion.Admit(selected.Definition, selected.Handle); }
    /// <summary>Selects the named find question; dispose the returned question after use.</summary>
    public FindQuestion Find(string name) { var selected = Select(name, Function.Find); return FindQuestion.Admit(selected.Definition, selected.Handle); }
    /// <summary>Selects the named recognize question; dispose the returned question after use.</summary>
    public RecognizeQuestion Recognize(string name) { var selected = Select(name, Function.Recognize); return RecognizeQuestion.Admit(selected.Definition, selected.Handle); }
    /// <summary>Selects the named relate question; dispose the returned question after use.</summary>
    public RelateQuestion Relate(string name) { var selected = Select(name, Function.Relate); return RelateQuestion.Admit(selected.Definition, selected.Handle); }
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
    /// <summary>A typed filter question definition.</summary>
    public sealed record Filter : AnnotationQuestion
    {
        /// <summary>Include the supplied filter question in an annotation form.</summary>
        public Filter(FilterQuestion question) : base(Admission.Required(question).Definition) { }
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
public sealed class AnnotationQuestions : IDisposable, IEnumerable<string>
{
    private readonly JsonObject definition;
    private AnnotateQuestion? question;
    private bool frozen;
    private bool disposed;
    internal JsonObject Definition
    {
        get { ObjectDisposedException.ThrowIf(disposed, this); frozen = true; return definition; }
    }
    internal AnnotateQuestion Question
    {
        get
        {
            ObjectDisposedException.ThrowIf(disposed, this);
            frozen = true;
            return question ??= AnnotateQuestion.Admit(Admission.Build(Function.Annotate, null, definition.DeepClone(), "questions"));
        }
    }
    /// <summary>Creates an empty form for typed Add calls or a C# collection initializer; the engine admits it when first used.</summary>
    public AnnotationQuestions() => definition = new();
    private AnnotationQuestions(JsonObject definition)
    {
        this.definition = definition;
        question = Question;
    }
    /// <summary>Gets the number of named questions in the form.</summary>
    public int Count => definition.Count;
    /// <summary>Snapshots a named annotation question. Entries cannot change after the form is first admitted.</summary>
    public AnnotationQuestions Add(string name, AnnotationQuestion question)
    {
        ObjectDisposedException.ThrowIf(disposed, this);
        ArgumentNullException.ThrowIfNull(question);
        if (frozen) throw new InvalidOperationException("Add questions before passing this form to a client.");
        definition.Add(Json.HostText(name), question.Definition.DeepClone());
        return this;
    }
    /// <summary>Snapshots a named decide question; ownership of the supplied question stays with its caller.</summary>
    public AnnotationQuestions Add(string name, DecideQuestion question) => Add(name, new AnnotationQuestion.Decide(question));
    /// <summary>Snapshots a named choose question; ownership of the supplied question stays with its caller.</summary>
    public AnnotationQuestions Add(string name, ChooseQuestion question) => Add(name, new AnnotationQuestion.Choose(question));
    /// <summary>Snapshots a named tag question; ownership of the supplied question stays with its caller.</summary>
    public AnnotationQuestions Add(string name, TagQuestion question) => Add(name, new AnnotationQuestion.Tag(question));
    /// <summary>Snapshots a named score question; ownership of the supplied question stays with its caller.</summary>
    public AnnotationQuestions Add(string name, ScoreQuestion question) => Add(name, new AnnotationQuestion.Score(question));
    /// <summary>Snapshots a named filter question; ownership of the supplied question stays with its caller.</summary>
    public AnnotationQuestions Add(string name, FilterQuestion question) => Add(name, new AnnotationQuestion.Filter(question));
    /// <summary>Snapshots a named find question; ownership of the supplied question stays with its caller.</summary>
    public AnnotationQuestions Add(string name, FindQuestion question) => Add(name, new AnnotationQuestion.Find(question));
    /// <summary>Snapshots a named recognize question; ownership of the supplied question stays with its caller.</summary>
    public AnnotationQuestions Add(string name, RecognizeQuestion question) => Add(name, new AnnotationQuestion.Recognize(question));
    /// <summary>Snapshots a named relate question; ownership of the supplied question stays with its caller.</summary>
    public AnnotationQuestions Add(string name, RelateQuestion question) => Add(name, new AnnotationQuestion.Relate(question));
    /// <summary>Enumerates the form's question names in insertion order.</summary>
    public IEnumerator<string> GetEnumerator()
    {
        ObjectDisposedException.ThrowIf(disposed, this);
        return definition.Select(pair => pair.Key).GetEnumerator();
    }
    System.Collections.IEnumerator System.Collections.IEnumerable.GetEnumerator() => GetEnumerator();
    /// <summary>Releases the admitted form, if it has been used; later admission and Add calls are refused.</summary>
    public void Dispose()
    {
        if (disposed) return;
        disposed = true;
        question?.Dispose();
    }
    /// <summary>Snapshots and admits named annotation question definitions through the engine.</summary>
    public static AnnotationQuestions Create(IReadOnlyDictionary<string, AnnotationQuestion> questions) => new(Admission.Named(questions));
    /// <summary>Parses bare named question definitions and admits the annotation form through the engine.</summary>
    public static AnnotationQuestions Parse(string json, CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        using var toolbox = Toolbox.Parse(json, cancellationToken);
        return new(toolbox.Definition);
    }
}
