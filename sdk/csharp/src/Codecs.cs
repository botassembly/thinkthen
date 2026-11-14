using System.Runtime.InteropServices;
using System.Text;
using System.Text.Json;
using System.Text.Json.Nodes;
using System.Text.Json.Serialization;
namespace ThinkThen;

internal static class Json
{
    internal static ThinkThenException Abi(Exception? cause = null) => new UsageException(new(ErrorCode.UsageAbi,Contract.AbiMessage),cause);
    internal static void Check(int status, NativeError error)
    {
        try
        {
            if (status == 0) return;
            string? name = Contract.CodeName(error.Code);
            if (name is null || Marshal.PtrToStringUTF8(Native.CodeName(error.Code)) != name) throw Abi();
            throw Failure(new((ErrorCode)error.Code, Encoding.UTF8.GetString(Native.Copy(error.Message)),
                error.HttpStatus == 0 ? null : error.HttpStatus));
        }
        finally { Native.ErrorFree(ref error); }
    }
    internal static ThinkThenException Failure(RowError error,Exception? cause=null) => error.ErrorCode.Family() switch
    {
        ErrorFamily.Usage => error.ErrorCode is ErrorCode.UsageKeyMissing ? new SetupException(error,cause) : new UsageException(error,cause),
        _ => new InternalException(error,cause)
    };
    internal static ThinkThenException Conversion(Exception? cause, bool settings = false)
    {
        NativeError error = NativeError.Empty;
        int result = settings ? Native.SettingsConversionError(ref error) : Native.QuestionConversionError(ref error);
        try { Check(result, error); }
        catch (ThinkThenException failure) { return Failure(new(failure.ErrorCode, failure.Message, failure.HttpStatus),cause); }
        throw Abi();
    }
    internal static string HostText(string value, bool settings = false)
    {
        ArgumentNullException.ThrowIfNull(value);
        try { _ = new UTF8Encoding(false, true).GetByteCount(value); return value; }
        catch (Exception cause) when (cause is EncoderFallbackException or ArgumentNullException) { throw Conversion(cause, settings); }
    }
    internal static ErrorCode Code(string name)
    {
        foreach (ErrorCode code in Enum.GetValues<ErrorCode>()) if (Contract.CodeName((int)code) == name) return code;
        throw Abi();
    }
    internal static Backend Backend(string name)
    {
        foreach (Backend backend in Enum.GetValues<Backend>()) if (Contract.BackendName((int)backend) == name) return backend;
        throw Abi();
    }
    internal static Function Function(string name)
    {
        foreach (Function function in Enum.GetValues<Function>()) if (Contract.FunctionName((int)function) == name) return function;
        throw Abi();
    }
}
/// <summary>Looks up error families from the shared engine registry.</summary>
public static class ErrorCodes
{
    /// <summary>Gets this code's family from the shared engine.</summary>
    public static ErrorFamily Family(this ErrorCode code) => (Marshal.PtrToStringUTF8(Native.CodeFamily((int)code))) switch
    { "usage" => ErrorFamily.Usage, "record" => ErrorFamily.Record, "backend" => ErrorFamily.Backend,
      "local" => ErrorFamily.Local, "internal" => ErrorFamily.Internal, _ => throw Json.Abi() };
}
/// <summary>Serializes shared error codes by their stable wire name.</summary>
public sealed class ErrorCodeConverter : JsonConverter<ErrorCode>
{
    /// <summary>Serializes shared error codes by their stable wire name.</summary>
    public ErrorCodeConverter() { }
    /// <inheritdoc/>
    public override ErrorCode Read(ref Utf8JsonReader reader, Type type, JsonSerializerOptions options) => Json.Code(reader.GetString()!);
    /// <inheritdoc/>
    public override void Write(Utf8JsonWriter writer, ErrorCode value, JsonSerializerOptions options) =>
        writer.WriteStringValue(Contract.CodeName((int)value) ?? throw Json.Abi());
}
/// <summary>Serializes backend names using the engine registry.</summary>
public sealed class BackendConverter : JsonConverter<Backend>
{
    /// <summary>Serializes backend names using the engine registry.</summary>
    public BackendConverter() { }
    /// <inheritdoc/>
    public override Backend Read(ref Utf8JsonReader reader, Type type, JsonSerializerOptions options) => Json.Backend(reader.GetString()!);
    /// <inheritdoc/>
    public override void Write(Utf8JsonWriter writer, Backend value, JsonSerializerOptions options) =>
        writer.WriteStringValue(Contract.BackendName((int)value) ?? throw Json.Abi());
}
[JsonSourceGenerationOptions(PropertyNamingPolicy = JsonKnownNamingPolicy.SnakeCaseLower)]
[JsonSerializable(typeof(JsonElement))]
[JsonSerializable(typeof(Usage))]
internal partial class ContractJson : JsonSerializerContext { }

[JsonSourceGenerationOptions(PropertyNamingPolicy = JsonKnownNamingPolicy.SnakeCaseLower,
    UnmappedMemberHandling = JsonUnmappedMemberHandling.Disallow,
    RespectRequiredConstructorParameters = true, RespectNullableAnnotations = true)]
[JsonSerializable(typeof(Source))]
internal partial class SourceJson : JsonSerializerContext { }

internal static class Codecs
{
    internal static Row<A,T> Row<A,D,T>(byte[] host,byte[] saved,Question<A,D> question,T item)
    {
        try
        {
            using var document=JsonDocument.Parse(host);
            using var canonical=JsonDocument.Parse(saved);
            var value=document.RootElement;
            if (value.GetProperty("function").GetString()!=Contract.FunctionName((uint)question.Function)) throw Json.Abi();
            var route=value.GetProperty("source");
            var source=route.Deserialize(SourceJson.Default.Source) ?? throw Json.Abi();
            if(route.TryGetProperty("name",out var name)&&name.ValueKind==JsonValueKind.Null) throw Json.Abi();
            var id=value.TryGetProperty("id",out var identity) ? Id(identity) : null;
            return Decode(value,canonical.RootElement,question.Definition,source,value.GetProperty("position").GetUInt64(),id,item,question.ConvertAnswer,identity);
        }
        catch (Exception cause) when (cause is JsonException or InvalidOperationException or KeyNotFoundException or FormatException or OverflowException or InvalidCastException)
        { throw Json.Abi(cause); }
    }
    private static Row<A,T> Decode<A,T>(JsonElement value,JsonElement saved,JsonObject definition,Source source,ulong position,RecordId? id,T item,Func<object?,A> convert,JsonElement rawId=default)
    {
        ValidateOutcome(value);
        var function=Json.Function(value.GetProperty("function").GetString()!);
        var status=value.GetProperty("status").GetString();
        JsonElement canonical=saved;
        if (!saved.TryGetProperty("position",out _))
        {
            var detached=JsonNode.Parse(saved.GetRawText())!.AsObject();
            detached["position"]=position;
            if(rawId.ValueKind!=JsonValueKind.Undefined) detached["id"]=JsonNode.Parse(rawId.GetRawText());
            detached["source"]=new JsonObject { ["question_identity"]=source.QuestionIdentity,["backend"]=Contract.BackendName((int)source.Backend),["model"]=source.Model };
            if (source.Name is not null) detached["source"]!["name"]=source.Name;
            using var document=JsonDocument.Parse(detached.ToJsonString()); canonical=document.RootElement.Clone();
        }
        if (status=="failed") return new Failed<A,T>(function,position,id,source,item,Error(value.GetProperty("error"))) { Canonical=canonical.Clone() };
        Detail? detail=value.TryGetProperty("detail",out var rawDetail) ? Detail(function,rawDetail,saved.GetProperty("detail"),definition) : null;
        double? probability=value.TryGetProperty("probability",out var number) ? number.GetDouble() : null;
        if (status=="unsure") return new Unsure<A,T>(function,position,id,source,item,probability ?? throw Json.Abi(),detail) { Canonical=canonical.Clone() };
        object? answer=function==Function.Annotate ? Annotations(value.GetProperty("answer"),saved.GetProperty("answer"),definition["questions"]!.AsObject(),source,position,id,item,rawId) : Answer(function,value.GetProperty("answer"),saved.GetProperty("answer"));
        return new Answered<A,T>(function,position,id,source,item,convert(answer),probability,detail) { Canonical=canonical.Clone() };
    }
    private static RecordId Id(JsonElement value) => value.ValueKind switch
    {
        JsonValueKind.String => new StringId(value.GetString()!),
        JsonValueKind.Number when value.TryGetInt64(out long signed) => new SignedId(signed),
        JsonValueKind.Number when value.TryGetUInt64(out ulong unsigned) => new UnsignedId(unsigned),
        _ => throw Json.Abi()
    };
    private static RowError Error(JsonElement value) => new(Json.Code(value.GetProperty("code").GetString()!),value.GetProperty("message").GetString()!,value.TryGetProperty("http_status",out var status) ? status.GetInt32() : null);
    private static TextSpan Span(JsonElement value,JsonElement saved) => new(value.GetProperty("text").GetString()!,value.GetProperty("start").GetInt32(),value.GetProperty("end").GetInt32()) { SavedStart=saved.GetProperty("start").GetInt32(), SavedEnd=saved.GetProperty("end").GetInt32() };
    private static Mention Mention(JsonElement value,JsonElement saved) => new(Span(value,saved),value.GetProperty("kind").GetString()!);
    private static Entity Entity(JsonElement value,JsonElement saved) => new(Mention(value,saved),value.GetProperty("probability").GetDouble());
    private static FoundRelation Relation(JsonElement value,JsonElement saved) => new(Mention(value.GetProperty("source"),saved.GetProperty("source")),Mention(value.GetProperty("target"),saved.GetProperty("target")),value.GetProperty("relation").GetString()!,value.GetProperty("probability").GetDouble(),value.TryGetProperty("either",out var either)&&either.GetBoolean());
    private static ValueList<T> Paired<T>(JsonElement host,JsonElement saved,Func<JsonElement,JsonElement,T> read) => host.EnumerateArray().Zip(saved.EnumerateArray(),read).ToValueList();
    private static object? Answer(Function function,JsonElement value,JsonElement saved) => function switch
    {
        Function.Decide or Function.Filter => value.GetBoolean(),
        Function.Choose => value.GetString(),
        Function.Tag => value.EnumerateArray().Select(item=>item.GetString()!).ToValueList(),
        Function.Score => value.GetDouble(),
        Function.Rank => value.GetInt64(),
        Function.Find => value.ValueKind==JsonValueKind.Null ? null : Span(value,saved),
        Function.Recognize => Paired(value,saved,Entity),
        Function.Relate => Paired(value,saved,Relation),
        _ => throw Json.Abi()
    };
    private static ValueMap<string,double> Map(JsonElement value) => value.EnumerateObject().ToValueMap(pair=>pair.Name,pair=>pair.Value.GetDouble());
    private static BoundaryTag Boundary(string name) => Enum.Parse<BoundaryTag>(name,true);
    private static Detail Detail(Function function,JsonElement value,JsonElement saved,JsonObject definition) => function switch
    {
        Function.Choose or Function.Tag => new LabelsDetail(QuestionWords(definition,function==Function.Choose ? "choices" : "labels").ToValueMap(word=>word,word=>value.GetProperty("labels").GetProperty(word).GetDouble())),
        Function.Score or Function.Rank => new LevelsDetail(value.GetProperty("levels").EnumerateArray().Select(item=>item.GetDouble()).ToValueList()),
        Function.Recognize => new RecognizeDetail(Paired(value.GetProperty("pieces"),saved.GetProperty("pieces"),(piece,canonical)=>new PieceDetail(Span(piece,canonical),piece.GetProperty("tags").EnumerateObject().ToValueMap(pair=>Boundary(pair.Name),pair=>pair.Value.GetDouble()))),Paired(value.GetProperty("names"),saved.GetProperty("names"),(name,canonical)=>new KindDetail(name.GetProperty("text").GetString()!,name.GetProperty("start").GetInt32(),name.GetProperty("end").GetInt32(),Map(name.GetProperty("kinds"))))),
        Function.Relate => new RelationDetail(Paired(value.GetProperty("names"),saved.GetProperty("names"),Entity),Paired(value.GetProperty("pairs"),saved.GetProperty("pairs"),Relation)),
        _ => throw Json.Abi()
    };
    internal static IReadOnlyList<string> QuestionWords(JsonObject definition,string key) => definition[key]!.AsArray().Select(value=>value is JsonObject option ? option["name"]!.GetValue<string>() : value!.GetValue<string>()).ToArray();
    private static Annotations<T> Annotations<T>(JsonElement values,JsonElement saved,JsonObject definitions,Source source,ulong position,RecordId? id,T item,JsonElement rawId)
    {
        var entries=new List<KeyValuePair<string,IRow>>();
        foreach (var entry in values.EnumerateObject())
        {
            var definition=definitions[entry.Name]!.AsObject();
            var canonical=saved.GetProperty(entry.Name);
            IRow row=entry.Value.GetProperty("function").GetString() switch
            {
                "decide" => Decode(entry.Value,canonical,definition,source,position,id,item,answer=>(bool)answer!,rawId),
                "choose" => Decode(entry.Value,canonical,definition,source,position,id,item,answer=>(string)answer!,rawId),
                "tag" => Decode(entry.Value,canonical,definition,source,position,id,item,answer=>(ValueList<string>)answer!,rawId),
                "score" => Decode(entry.Value,canonical,definition,source,position,id,item,answer=>(double)answer!,rawId),
                "find" => Decode(entry.Value,canonical,definition,source,position,id,item,answer=>(TextSpan?)answer,rawId),
                "recognize" => Decode(entry.Value,canonical,definition,source,position,id,item,answer=>(ValueList<Entity>)answer!,rawId),
                "relate" => Decode(entry.Value,canonical,definition,source,position,id,item,answer=>(ValueList<FoundRelation>)answer!,rawId),
                _ => throw Json.Abi()
            };
            entries.Add(KeyValuePair.Create(entry.Name,row));
        }
        var domains=definitions.Where(entry=>entry.Value!["function"]!.GetValue<string>() is "choose" or "tag").ToDictionary(entry=>entry.Key,entry=>QuestionWords(entry.Value!.AsObject(),entry.Value!["function"]!.GetValue<string>()=="choose" ? "choices" : "labels"));
        return new(entries,domains);
    }
    private static void ValidateOutcome(JsonElement value)
    {
        bool Has(string key)=>value.TryGetProperty(key,out _);
        bool valid=value.GetProperty("status").GetString() switch
        { "answered" => Has("answer")&&!Has("error"), "unsure" => !Has("answer")&&!Has("error")&&Has("probability"), "failed" => !Has("answer")&&!Has("probability")&&!Has("detail")&&Has("error"), _=>false };
        if (!valid) throw Json.Abi();
    }
}
