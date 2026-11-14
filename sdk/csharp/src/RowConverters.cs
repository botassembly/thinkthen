using System.Text.Json;
using System.Text.Json.Serialization;
namespace ThinkThen;
/// <summary>Writes result rows without reflection or access to their original record.</summary>
public sealed class RowConverters : JsonConverterFactory
{
    /// <inheritdoc/>
    public override bool CanConvert(Type type) => Supports(type);
    /// <inheritdoc/>
    public override JsonConverter CreateConverter(Type type,JsonSerializerOptions options) => Supports(type) ? new Converter() : throw new NotSupportedException("Only ThinkThen result rows are supported.");
    private static bool Supports(Type type)=>type==typeof(IRow)||type==typeof(IFailed)||typeof(IRowValue).IsAssignableFrom(type);
    private sealed class Converter : JsonConverter<IRow>
    {
        /// <inheritdoc/>
        public override bool CanConvert(Type type) => Supports(type);
        /// <inheritdoc/>
        public override IRow Read(ref Utf8JsonReader reader,Type type,JsonSerializerOptions options)=>throw new NotSupportedException("Rows are returned by the client.");
        /// <inheritdoc/>
        public override void Write(Utf8JsonWriter writer,IRow value,JsonSerializerOptions options)
        {
            if(value is not IRowValue row) throw new NotSupportedException("Only rows returned by ThinkThen are supported.");
            if(row.Canonical.ValueKind!=JsonValueKind.Undefined) row.Canonical.WriteTo(writer);
            else RowJson.Write(writer,row);
        }
    }
}
