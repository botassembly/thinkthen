using System.Collections;
using System.Text.Json;

namespace ThinkThen;

// Constructed model rows have no cached wire representation. Write their public
// values using the same closed, trimming-safe families accepted by RowConverters.
internal static class RowJson
{
    internal static void Write<TAnswer, TDetail>(Utf8JsonWriter writer, Row<TAnswer, TDetail> row)
    {
        writer.WriteStartObject();
        writer.WriteString("function", Contract.FunctionName((int)row.Function));
        writer.WriteNumber("position", row.Position);
        if (row.Id is not null) Property(writer, "id", row.Id);
        writer.WriteString("status", Contract.StatusName((int)row.Status));
        Property(writer, "source", row.Source);
        if (row.IsAnswered) Property(writer, "answer", row.Answer);
        if (row.Probability is double probability) writer.WriteNumber("probability", probability);
        object? detail = row switch
        {
            Answered<TAnswer, TDetail> answered => answered.Detail,
            Unsure<TAnswer, TDetail> unsure => unsure.Detail,
            _ => null
        };
        if (detail is not null) Property(writer, "detail", detail);
        if (row is Failed<TAnswer, TDetail> failed) Property(writer, "error", failed.Error);
        writer.WriteEndObject();
    }
    private static void Property(Utf8JsonWriter writer, string name, object? value)
    { writer.WritePropertyName(name); Value(writer, value); }
    private static void Span(Utf8JsonWriter writer, TextSpan span)
    {
        writer.WriteString("text", span.Text);
        writer.WriteNumber("start", span.Start);
        writer.WriteNumber("end", span.End);
    }
    private static void Mention(Utf8JsonWriter writer, Mention mention)
    { Span(writer, mention.Span); writer.WriteString("kind", mention.Kind); }
    private static void Value(Utf8JsonWriter writer, object? value)
    {
        switch (value)
        {
            case null: writer.WriteNullValue(); return;
            case string text: writer.WriteStringValue(text); return;
            case bool boolean: writer.WriteBooleanValue(boolean); return;
            case int number: writer.WriteNumberValue(number); return;
            case long number: writer.WriteNumberValue(number); return;
            case ulong number: writer.WriteNumberValue(number); return;
            case double number: writer.WriteNumberValue(number); return;
            case StringId id: writer.WriteStringValue(id.Value); return;
            case SignedId id: writer.WriteNumberValue(id.Value); return;
            case UnsignedId id: writer.WriteNumberValue(id.Value); return;
            case IReadOnlyDictionary<string, double> map:
                writer.WriteStartObject();
                foreach (var pair in map) writer.WriteNumber(pair.Key, pair.Value);
                writer.WriteEndObject(); return;
            case IReadOnlyDictionary<BoundaryTag, double> map:
                writer.WriteStartObject();
                foreach (var pair in map) writer.WriteNumber(pair.Key.ToString().ToUpperInvariant(), pair.Value);
                writer.WriteEndObject(); return;
            case IReadOnlyDictionary<string, AnnotationEntry> map:
                writer.WriteStartObject();
                foreach (var pair in map) Property(writer, pair.Key, pair.Value);
                writer.WriteEndObject(); return;
            case IEnumerable sequence:
                writer.WriteStartArray();
                foreach (object? item in sequence) Value(writer, item);
                writer.WriteEndArray(); return;
        }
        writer.WriteStartObject();
        switch (value)
        {
            case Source source:
                writer.WriteString("question_identity", source.QuestionIdentity);
                writer.WriteString("backend", Contract.BackendName((int)source.Backend));
                writer.WriteString("model", source.Model);
                if (source.Name is not null) writer.WriteString("name", source.Name);
                break;
            case RowError error:
                writer.WriteString("code", error.Code);
                writer.WriteString("message", error.Message);
                if (error.HttpStatus is int status) writer.WriteNumber("http_status", status);
                break;
            case TextSpan span: Span(writer, span); break;
            case Entity entity: Mention(writer, entity.Mention); writer.WriteNumber("probability", entity.Probability); break;
            case Mention mention: Mention(writer, mention); break;
            case Relation relation:
                Property(writer, "source", relation.Source); Property(writer, "target", relation.Target);
                writer.WriteString("relation", relation.Name); writer.WriteNumber("probability", relation.Probability);
                if (relation.Either) writer.WriteBoolean("either", true);
                break;
            case LabelsDetail detail: Property(writer, "labels", detail.Labels); break;
            case LevelsDetail detail: Property(writer, "levels", detail.Levels); break;
            case PieceDetail detail: Span(writer, detail.Span); Property(writer, "tags", detail.Tags); break;
            case KindDetail detail:
                writer.WriteString("text", detail.Text);
                writer.WriteNumber("start", detail.Start); writer.WriteNumber("end", detail.End);
                Property(writer, "kinds", detail.Kinds); break;
            case RecognizeDetail detail: Property(writer, "pieces", detail.Pieces); Property(writer, "names", detail.Names); break;
            case RelateDetail detail: Property(writer, "names", detail.Names); Property(writer, "pairs", detail.Pairs); break;
            case AnnotationEntry entry:
                writer.WriteString("function", Contract.FunctionName((int)entry.Function));
                writer.WriteString("status", Contract.StatusName((int)entry.Status));
                object?[] parts = entry.Value.Values;
                if (entry.IsAnswered) Property(writer, "answer", entry.Answer);
                if (entry.Probability is double probability) writer.WriteNumber("probability", probability);
                if (entry.IsFailed) Property(writer, "error", parts[1]);
                else if (parts[^1] is object detail) Property(writer, "detail", detail);
                break;
            default: throw new NotSupportedException($"Unsupported answer value {value.GetType().Name}.");
        }
        writer.WriteEndObject();
    }
}
