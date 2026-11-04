using System.Text.Json;
using System.Text.Json.Serialization;
namespace ThinkThen;
// The supported closed row families use explicit converters without reflection activation.
/// <summary>Provides trimming-safe serialization for supported typed result rows.</summary>
public sealed class RowConverters : JsonConverterFactory
{
    /// <summary>Provides trimming-safe serialization for supported typed result rows.</summary>
    public RowConverters() { }
    /// <inheritdoc/>
    public override bool CanConvert(Type type) => Create(type) is not null;
    /// <inheritdoc/>
    public override JsonConverter CreateConverter(Type type, JsonSerializerOptions options) => Create(type) ?? throw Json.Abi();
    private static JsonConverter? Create(Type type)
    {
        if (type == typeof(Row<bool,NoDetail>) || type == typeof(Answered<bool,NoDetail>) || type == typeof(Unsure<bool,NoDetail>) || type == typeof(Failed<bool,NoDetail>)) return new Converter<bool,NoDetail>();
        if (type == typeof(Row<string,LabelsDetail>) || type == typeof(Answered<string,LabelsDetail>) || type == typeof(Unsure<string,LabelsDetail>) || type == typeof(Failed<string,LabelsDetail>)) return new Converter<string,LabelsDetail>();
        if (type == typeof(Row<ValueList<string>,LabelsDetail>) || type == typeof(Answered<ValueList<string>,LabelsDetail>) || type == typeof(Unsure<ValueList<string>,LabelsDetail>) || type == typeof(Failed<ValueList<string>,LabelsDetail>)) return new Converter<ValueList<string>,LabelsDetail>();
        if (type == typeof(Row<double,LevelsDetail>) || type == typeof(Answered<double,LevelsDetail>) || type == typeof(Unsure<double,LevelsDetail>) || type == typeof(Failed<double,LevelsDetail>)) return new Converter<double,LevelsDetail>();
        if (type == typeof(Row<long,LevelsDetail>) || type == typeof(Answered<long,LevelsDetail>) || type == typeof(Unsure<long,LevelsDetail>) || type == typeof(Failed<long,LevelsDetail>)) return new Converter<long,LevelsDetail>();
        if (type == typeof(Row<TextSpan?,NoDetail>) || type == typeof(Answered<TextSpan?,NoDetail>) || type == typeof(Unsure<TextSpan?,NoDetail>) || type == typeof(Failed<TextSpan?,NoDetail>)) return new Converter<TextSpan?,NoDetail>();
        if (type == typeof(Row<ValueList<Entity>,RecognizeDetail>) || type == typeof(Answered<ValueList<Entity>,RecognizeDetail>) || type == typeof(Unsure<ValueList<Entity>,RecognizeDetail>) || type == typeof(Failed<ValueList<Entity>,RecognizeDetail>)) return new Converter<ValueList<Entity>,RecognizeDetail>();
        if (type == typeof(Row<ValueList<Relation>,RelateDetail>) || type == typeof(Answered<ValueList<Relation>,RelateDetail>) || type == typeof(Unsure<ValueList<Relation>,RelateDetail>) || type == typeof(Failed<ValueList<Relation>,RelateDetail>)) return new Converter<ValueList<Relation>,RelateDetail>();
        if (type == typeof(Row<ValueMap<string, AnnotationEntry>,NoDetail>) || type == typeof(Answered<ValueMap<string, AnnotationEntry>,NoDetail>) || type == typeof(Unsure<ValueMap<string, AnnotationEntry>,NoDetail>) || type == typeof(Failed<ValueMap<string, AnnotationEntry>,NoDetail>)) return new Converter<ValueMap<string, AnnotationEntry>,NoDetail>();
        return null;
    }
    private sealed class Converter<TAnswer,TDetail> : JsonConverter<Row<TAnswer,TDetail>>
    {
        /// <inheritdoc/>
        public override bool CanConvert(Type type) => type == typeof(Row<TAnswer,TDetail>) || type == typeof(Answered<TAnswer,TDetail>) || type == typeof(Unsure<TAnswer,TDetail>) || type == typeof(Failed<TAnswer,TDetail>);
        /// <inheritdoc/>
        public override Row<TAnswer,TDetail> Read(ref Utf8JsonReader reader, Type type, JsonSerializerOptions options) => throw new NotSupportedException("Rows are returned by the client.");
        /// <inheritdoc/>
        public override void Write(Utf8JsonWriter writer, Row<TAnswer,TDetail> value, JsonSerializerOptions options)
        {
            if (value.Canonical.ValueKind != JsonValueKind.Undefined) value.Canonical.WriteTo(writer);
            else RowJson.Write(writer, value);
        }
    }
}
