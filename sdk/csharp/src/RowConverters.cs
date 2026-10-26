using System.Collections.Immutable;
using System.Text.Json;
using System.Text.Json.Serialization;
namespace ThinkThen;
// The supported closed row families use explicit converters without reflection activation.
public sealed class RowConverters : JsonConverterFactory
{
    public RowConverters() { }
    public override bool CanConvert(Type type) => Create(type) is not null;
    public override JsonConverter CreateConverter(Type type, JsonSerializerOptions options) => Create(type) ?? throw Json.Abi();
    private static JsonConverter? Create(Type type)
    {
        if (type == typeof(Row<bool,NoDetail>) || type == typeof(Answered<bool,NoDetail>) || type == typeof(Unsure<bool,NoDetail>) || type == typeof(Failed<bool,NoDetail>)) return new Converter<bool,NoDetail>();
        if (type == typeof(Row<string,LabelsDetail>) || type == typeof(Answered<string,LabelsDetail>) || type == typeof(Unsure<string,LabelsDetail>) || type == typeof(Failed<string,LabelsDetail>)) return new Converter<string,LabelsDetail>();
        if (type == typeof(Row<ImmutableArray<string>,LabelsDetail>) || type == typeof(Answered<ImmutableArray<string>,LabelsDetail>) || type == typeof(Unsure<ImmutableArray<string>,LabelsDetail>) || type == typeof(Failed<ImmutableArray<string>,LabelsDetail>)) return new Converter<ImmutableArray<string>,LabelsDetail>();
        if (type == typeof(Row<double,LevelsDetail>) || type == typeof(Answered<double,LevelsDetail>) || type == typeof(Unsure<double,LevelsDetail>) || type == typeof(Failed<double,LevelsDetail>)) return new Converter<double,LevelsDetail>();
        if (type == typeof(Row<long,LevelsDetail>) || type == typeof(Answered<long,LevelsDetail>) || type == typeof(Unsure<long,LevelsDetail>) || type == typeof(Failed<long,LevelsDetail>)) return new Converter<long,LevelsDetail>();
        if (type == typeof(Row<TextSpan?,NoDetail>) || type == typeof(Answered<TextSpan?,NoDetail>) || type == typeof(Unsure<TextSpan?,NoDetail>) || type == typeof(Failed<TextSpan?,NoDetail>)) return new Converter<TextSpan?,NoDetail>();
        if (type == typeof(Row<ImmutableArray<Entity>,RecognizeDetail>) || type == typeof(Answered<ImmutableArray<Entity>,RecognizeDetail>) || type == typeof(Unsure<ImmutableArray<Entity>,RecognizeDetail>) || type == typeof(Failed<ImmutableArray<Entity>,RecognizeDetail>)) return new Converter<ImmutableArray<Entity>,RecognizeDetail>();
        if (type == typeof(Row<ImmutableArray<Relation>,RelateDetail>) || type == typeof(Answered<ImmutableArray<Relation>,RelateDetail>) || type == typeof(Unsure<ImmutableArray<Relation>,RelateDetail>) || type == typeof(Failed<ImmutableArray<Relation>,RelateDetail>)) return new Converter<ImmutableArray<Relation>,RelateDetail>();
        if (type == typeof(Row<ImmutableDictionary<string, AnnotationEntry>,NoDetail>) || type == typeof(Answered<ImmutableDictionary<string, AnnotationEntry>,NoDetail>) || type == typeof(Unsure<ImmutableDictionary<string, AnnotationEntry>,NoDetail>) || type == typeof(Failed<ImmutableDictionary<string, AnnotationEntry>,NoDetail>)) return new Converter<ImmutableDictionary<string, AnnotationEntry>,NoDetail>();
        return null;
    }
    private sealed class Converter<A,D> : JsonConverter<Row<A,D>>
    {
        public override bool CanConvert(Type type) => type == typeof(Row<A,D>) || type == typeof(Answered<A,D>) || type == typeof(Unsure<A,D>) || type == typeof(Failed<A,D>);
        public override Row<A,D> Read(ref Utf8JsonReader reader, Type type, JsonSerializerOptions options) => throw new NotSupportedException("Rows are returned by the client.");
        public override void Write(Utf8JsonWriter writer, Row<A,D> value, JsonSerializerOptions options) => value.Canonical.WriteTo(writer);
    }
}
