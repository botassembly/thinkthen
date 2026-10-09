// Generated from the Rust-derived Request schema; do not edit.
#nullable enable
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.Json;
namespace ThinkThen.Inputs;
public readonly struct InputPresence<T> {
    public bool IsPresent { get; }
    public T Value { get; }
    private InputPresence(T value) { Value = value; IsPresent = true; }
    public static implicit operator InputPresence<T>(T value) => new(value);
}
public abstract class InputDocument {
    private static readonly System.Text.Encoding StrictUtf8 = new System.Text.UTF8Encoding(false, true);
    protected static void WriteText(Utf8JsonWriter writer, string? value) { if (value is not null) _ = StrictUtf8.GetByteCount(value); writer.WriteStringValue(value); }
    protected static void WriteName(Utf8JsonWriter writer, string value) { _ = StrictUtf8.GetByteCount(value); writer.WritePropertyName(value); }
    protected JsonElement? ParsedDocument { get; init; }
    public abstract void Write(Utf8JsonWriter writer);
    internal byte[] ToBytes() { using var stream = new MemoryStream(); using (var writer = new Utf8JsonWriter(stream)) Write(writer); return stream.ToArray(); }
}
public sealed class InputAuthoredChoose : InputDocument {
public InputPresence<InputAuthoredChooseBatch> Batch { get; init; }
public required InputAuthoredQuestionText Choose { get; init; }
public InputPresence<InputAuthoredInputDeclaration> ContextSchema { get; init; }
public InputPresence<InputAuthoredInputDeclaration> ItemSchema { get; init; }
public InputPresence<InputAuthoredName> Model { get; init; }
public InputPresence<string> Name { get; init; }
public InputPresence<InputAuthoredPointers> On { get; init; }
public InputPresence<InputAuthoredOptions> Options { get; init; }
public InputPresence<InputAuthoredProfile> Profile { get; init; }
public InputPresence<InputAuthoredCut> Threshold { get; init; }
public InputPresence<long> WordingVersion { get; init; }
internal static InputAuthoredChoose Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Batch = value.TryGetProperty("batch", out _) ? (InputPresence<InputAuthoredChooseBatch>)(InputAuthoredChooseBatch.Read(value.GetProperty("batch"))) : default, Choose = InputAuthoredQuestionText.Read(value.GetProperty("choose")), ContextSchema = value.TryGetProperty("context_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("context_schema"))) : default, ItemSchema = value.TryGetProperty("item_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("item_schema"))) : default, Model = value.TryGetProperty("model", out _) ? (InputPresence<InputAuthoredName>)(InputAuthoredName.Read(value.GetProperty("model"))) : default, Name = value.TryGetProperty("name", out _) ? (InputPresence<string>)(value.GetProperty("name").GetString()!) : default, On = value.TryGetProperty("on", out _) ? (InputPresence<InputAuthoredPointers>)(InputAuthoredPointers.Read(value.GetProperty("on"))) : default, Options = value.TryGetProperty("options", out _) ? (InputPresence<InputAuthoredOptions>)(InputAuthoredOptions.Read(value.GetProperty("options"))) : default, Profile = value.TryGetProperty("profile", out _) ? (InputPresence<InputAuthoredProfile>)(InputAuthoredProfile.Read(value.GetProperty("profile"))) : default, Threshold = value.TryGetProperty("threshold", out _) ? (InputPresence<InputAuthoredCut>)(InputAuthoredCut.Read(value.GetProperty("threshold"))) : default, WordingVersion = value.TryGetProperty("wording_version", out _) ? (InputPresence<long>)(value.GetProperty("wording_version").GetInt64()) : default };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); if (Batch.IsPresent) { writer.WritePropertyName("batch"); if (Batch.Value is null) writer.WriteNullValue(); else Batch.Value.Write(writer); } writer.WritePropertyName("choose"); if (Choose is null) writer.WriteNullValue(); else Choose.Write(writer); if (ContextSchema.IsPresent) { writer.WritePropertyName("context_schema"); if (ContextSchema.Value is null) writer.WriteNullValue(); else ContextSchema.Value.Write(writer); } if (ItemSchema.IsPresent) { writer.WritePropertyName("item_schema"); if (ItemSchema.Value is null) writer.WriteNullValue(); else ItemSchema.Value.Write(writer); } if (Model.IsPresent) { writer.WritePropertyName("model"); if (Model.Value is null) writer.WriteNullValue(); else Model.Value.Write(writer); } if (Name.IsPresent) { writer.WritePropertyName("name"); WriteText(writer, Name.Value); } if (On.IsPresent) { writer.WritePropertyName("on"); if (On.Value is null) writer.WriteNullValue(); else On.Value.Write(writer); } if (Options.IsPresent) { writer.WritePropertyName("options"); if (Options.Value is null) writer.WriteNullValue(); else Options.Value.Write(writer); } if (Profile.IsPresent) { writer.WritePropertyName("profile"); if (Profile.Value is null) writer.WriteNullValue(); else Profile.Value.Write(writer); } if (Threshold.IsPresent) { writer.WritePropertyName("threshold"); if (Threshold.Value is null) writer.WriteNullValue(); else Threshold.Value.Write(writer); } if (WordingVersion.IsPresent) { writer.WritePropertyName("wording_version"); writer.WriteNumberValue(WordingVersion.Value); } writer.WriteEndObject(); }
}

public abstract class InputAuthoredChooseBatch : InputDocument { private protected InputAuthoredChooseBatch() {} internal static InputAuthoredChooseBatch Read(JsonElement value) { if ((value.ValueKind == JsonValueKind.String && value.GetString() == "max")) return InputAuthoredChooseBatchAlternative0.Read(value); if ((value.ValueKind == JsonValueKind.Number)) return InputAuthoredChooseBatchAlternative1.Read(value); throw new JsonException("Authored data does not match the native Request schema."); } }

public sealed class InputAuthoredChooseBatchAlternative0 : InputAuthoredChooseBatch {
internal new static InputAuthoredChooseBatchAlternative0 Read(JsonElement value) => new() { ParsedDocument = value.Clone() };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } WriteText(writer, "max"); }
}

public sealed class InputAuthoredChooseBatchAlternative1 : InputAuthoredChooseBatch {
public required long Value { get; init; }
internal new static InputAuthoredChooseBatchAlternative1 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = value.GetInt64() };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteNumberValue(Value); }
}

public abstract class InputAuthoredCriterion : InputDocument { private protected InputAuthoredCriterion() {} internal static InputAuthoredCriterion Read(JsonElement value) { if ((value.ValueKind == JsonValueKind.String)) return InputAuthoredCriterionAlternative0.Read(value); if ((value.ValueKind == JsonValueKind.Object)) return InputAuthoredCriterionAlternative1.Read(value); if ((value.ValueKind == JsonValueKind.Array)) return InputAuthoredCriterionAlternative2.Read(value); if ((value.ValueKind == JsonValueKind.Null)) return InputAuthoredCriterionAlternative3.Read(value); throw new JsonException("Authored data does not match the native Request schema."); } }

public sealed class InputAuthoredCriterionAlternative0 : InputAuthoredCriterion {
public required string Value { get; init; }
internal new static InputAuthoredCriterionAlternative0 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = value.GetString()! };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } WriteText(writer, Value); }
}

public sealed class InputAuthoredCriterionAlternative1 : InputAuthoredCriterion {
public required JsonElement Value { get; init; }
internal new static InputAuthoredCriterionAlternative1 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = value.Clone() };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } Value.WriteTo(writer); }
}

public sealed class InputAuthoredCriterionAlternative2 : InputAuthoredCriterion {
public required IReadOnlyList<JsonElement> Value { get; init; }
internal new static InputAuthoredCriterionAlternative2 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = Array.AsReadOnly(value.EnumerateArray().Select(item0 => item0.Clone()).ToArray()) };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartArray(); foreach (var item in Value) { item.WriteTo(writer); } writer.WriteEndArray(); }
}

public sealed class InputAuthoredCriterionAlternative3 : InputAuthoredCriterion {
internal new static InputAuthoredCriterionAlternative3 Read(JsonElement value) => new() { ParsedDocument = value.Clone() };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteNullValue(); }
}

public abstract class InputAuthoredCut : InputDocument { private protected InputAuthoredCut() {} internal static InputAuthoredCut Read(JsonElement value) { if ((value.ValueKind == JsonValueKind.Number)) return InputAuthoredCutAlternative0.Read(value); if ((value.ValueKind == JsonValueKind.String)) return InputAuthoredCutAlternative1.Read(value); throw new JsonException("Authored data does not match the native Request schema."); } }

public sealed class InputAuthoredCutAlternative0 : InputAuthoredCut {
public required double Value { get; init; }
internal new static InputAuthoredCutAlternative0 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = value.GetDouble() };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteNumberValue(Value); }
}

public sealed class InputAuthoredCutAlternative1 : InputAuthoredCut {
public required string Value { get; init; }
internal new static InputAuthoredCutAlternative1 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = value.GetString()! };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } WriteText(writer, Value); }
}

public sealed class InputAuthoredDecide : InputDocument {
public InputPresence<InputAuthoredDecideBatch> Batch { get; init; }
public InputPresence<InputAuthoredInputDeclaration> ContextSchema { get; init; }
public required InputAuthoredQuestionText Decide { get; init; }
public InputPresence<InputAuthoredCriterion> False { get; init; }
public InputPresence<InputAuthoredInputDeclaration> ItemSchema { get; init; }
public InputPresence<InputAuthoredName> Model { get; init; }
public InputPresence<string> Name { get; init; }
public InputPresence<InputAuthoredPointers> On { get; init; }
public InputPresence<InputAuthoredProfile> Profile { get; init; }
public InputPresence<InputAuthoredThreshold> Threshold { get; init; }
public InputPresence<InputAuthoredCriterion> True { get; init; }
public InputPresence<long> WordingVersion { get; init; }
internal static InputAuthoredDecide Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Batch = value.TryGetProperty("batch", out _) ? (InputPresence<InputAuthoredDecideBatch>)(InputAuthoredDecideBatch.Read(value.GetProperty("batch"))) : default, ContextSchema = value.TryGetProperty("context_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("context_schema"))) : default, Decide = InputAuthoredQuestionText.Read(value.GetProperty("decide")), False = value.TryGetProperty("false", out _) ? (InputPresence<InputAuthoredCriterion>)(InputAuthoredCriterion.Read(value.GetProperty("false"))) : default, ItemSchema = value.TryGetProperty("item_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("item_schema"))) : default, Model = value.TryGetProperty("model", out _) ? (InputPresence<InputAuthoredName>)(InputAuthoredName.Read(value.GetProperty("model"))) : default, Name = value.TryGetProperty("name", out _) ? (InputPresence<string>)(value.GetProperty("name").GetString()!) : default, On = value.TryGetProperty("on", out _) ? (InputPresence<InputAuthoredPointers>)(InputAuthoredPointers.Read(value.GetProperty("on"))) : default, Profile = value.TryGetProperty("profile", out _) ? (InputPresence<InputAuthoredProfile>)(InputAuthoredProfile.Read(value.GetProperty("profile"))) : default, Threshold = value.TryGetProperty("threshold", out _) ? (InputPresence<InputAuthoredThreshold>)(InputAuthoredThreshold.Read(value.GetProperty("threshold"))) : default, True = value.TryGetProperty("true", out _) ? (InputPresence<InputAuthoredCriterion>)(InputAuthoredCriterion.Read(value.GetProperty("true"))) : default, WordingVersion = value.TryGetProperty("wording_version", out _) ? (InputPresence<long>)(value.GetProperty("wording_version").GetInt64()) : default };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); if (Batch.IsPresent) { writer.WritePropertyName("batch"); if (Batch.Value is null) writer.WriteNullValue(); else Batch.Value.Write(writer); } if (ContextSchema.IsPresent) { writer.WritePropertyName("context_schema"); if (ContextSchema.Value is null) writer.WriteNullValue(); else ContextSchema.Value.Write(writer); } writer.WritePropertyName("decide"); if (Decide is null) writer.WriteNullValue(); else Decide.Write(writer); if (False.IsPresent) { writer.WritePropertyName("false"); if (False.Value is null) writer.WriteNullValue(); else False.Value.Write(writer); } if (ItemSchema.IsPresent) { writer.WritePropertyName("item_schema"); if (ItemSchema.Value is null) writer.WriteNullValue(); else ItemSchema.Value.Write(writer); } if (Model.IsPresent) { writer.WritePropertyName("model"); if (Model.Value is null) writer.WriteNullValue(); else Model.Value.Write(writer); } if (Name.IsPresent) { writer.WritePropertyName("name"); WriteText(writer, Name.Value); } if (On.IsPresent) { writer.WritePropertyName("on"); if (On.Value is null) writer.WriteNullValue(); else On.Value.Write(writer); } if (Profile.IsPresent) { writer.WritePropertyName("profile"); if (Profile.Value is null) writer.WriteNullValue(); else Profile.Value.Write(writer); } if (Threshold.IsPresent) { writer.WritePropertyName("threshold"); if (Threshold.Value is null) writer.WriteNullValue(); else Threshold.Value.Write(writer); } if (True.IsPresent) { writer.WritePropertyName("true"); if (True.Value is null) writer.WriteNullValue(); else True.Value.Write(writer); } if (WordingVersion.IsPresent) { writer.WritePropertyName("wording_version"); writer.WriteNumberValue(WordingVersion.Value); } writer.WriteEndObject(); }
}

public abstract class InputAuthoredDecideBatch : InputDocument { private protected InputAuthoredDecideBatch() {} internal static InputAuthoredDecideBatch Read(JsonElement value) { if ((value.ValueKind == JsonValueKind.String && value.GetString() == "max")) return InputAuthoredDecideBatchAlternative0.Read(value); if ((value.ValueKind == JsonValueKind.Number)) return InputAuthoredDecideBatchAlternative1.Read(value); throw new JsonException("Authored data does not match the native Request schema."); } }

public sealed class InputAuthoredDecideBatchAlternative0 : InputAuthoredDecideBatch {
internal new static InputAuthoredDecideBatchAlternative0 Read(JsonElement value) => new() { ParsedDocument = value.Clone() };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } WriteText(writer, "max"); }
}

public sealed class InputAuthoredDecideBatchAlternative1 : InputAuthoredDecideBatch {
public required long Value { get; init; }
internal new static InputAuthoredDecideBatchAlternative1 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = value.GetInt64() };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteNumberValue(Value); }
}

public sealed class InputAuthoredDescription : InputDocument {
public required InputAuthoredDescriptionValue Value { get; init; }
internal static InputAuthoredDescription Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = InputAuthoredDescriptionValue.Read(value) };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } if (Value is null) writer.WriteNullValue(); else Value.Write(writer); }
}

public abstract class InputAuthoredDescriptionValue : InputDocument { private protected InputAuthoredDescriptionValue() {} internal static InputAuthoredDescriptionValue Read(JsonElement value) { if ((value.ValueKind == JsonValueKind.String)) return InputAuthoredDescriptionValueAlternative0.Read(value); if ((value.ValueKind == JsonValueKind.Object)) return InputAuthoredDescriptionValueAlternative1.Read(value); if ((value.ValueKind == JsonValueKind.Array)) return InputAuthoredDescriptionValueAlternative2.Read(value); if ((value.ValueKind == JsonValueKind.Null)) return InputAuthoredDescriptionValueAlternative3.Read(value); throw new JsonException("Authored data does not match the native Request schema."); } }

public sealed class InputAuthoredDescriptionValueAlternative0 : InputAuthoredDescriptionValue {
public required string Value { get; init; }
internal new static InputAuthoredDescriptionValueAlternative0 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = value.GetString()! };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } WriteText(writer, Value); }
}

public sealed class InputAuthoredDescriptionValueAlternative1 : InputAuthoredDescriptionValue {
public required JsonElement Value { get; init; }
internal new static InputAuthoredDescriptionValueAlternative1 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = value.Clone() };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } Value.WriteTo(writer); }
}

public sealed class InputAuthoredDescriptionValueAlternative2 : InputAuthoredDescriptionValue {
public required IReadOnlyList<JsonElement> Value { get; init; }
internal new static InputAuthoredDescriptionValueAlternative2 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = Array.AsReadOnly(value.EnumerateArray().Select(item0 => item0.Clone()).ToArray()) };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartArray(); foreach (var item in Value) { item.WriteTo(writer); } writer.WriteEndArray(); }
}

public sealed class InputAuthoredDescriptionValueAlternative3 : InputAuthoredDescriptionValue {
internal new static InputAuthoredDescriptionValueAlternative3 Read(JsonElement value) => new() { ParsedDocument = value.Clone() };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteNullValue(); }
}

public sealed class InputAuthoredFind : InputDocument {
public InputPresence<InputAuthoredInputDeclaration> ContextSchema { get; init; }
public required InputAuthoredQuestionText Find { get; init; }
public InputPresence<InputAuthoredInputDeclaration> ItemSchema { get; init; }
public InputPresence<InputAuthoredName> Model { get; init; }
public InputPresence<string> Name { get; init; }
public InputPresence<InputAuthoredPointers> On { get; init; }
public InputPresence<InputAuthoredProfile> Profile { get; init; }
public InputPresence<long> WordingVersion { get; init; }
internal static InputAuthoredFind Read(JsonElement value) => new() { ParsedDocument = value.Clone(), ContextSchema = value.TryGetProperty("context_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("context_schema"))) : default, Find = InputAuthoredQuestionText.Read(value.GetProperty("find")), ItemSchema = value.TryGetProperty("item_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("item_schema"))) : default, Model = value.TryGetProperty("model", out _) ? (InputPresence<InputAuthoredName>)(InputAuthoredName.Read(value.GetProperty("model"))) : default, Name = value.TryGetProperty("name", out _) ? (InputPresence<string>)(value.GetProperty("name").GetString()!) : default, On = value.TryGetProperty("on", out _) ? (InputPresence<InputAuthoredPointers>)(InputAuthoredPointers.Read(value.GetProperty("on"))) : default, Profile = value.TryGetProperty("profile", out _) ? (InputPresence<InputAuthoredProfile>)(InputAuthoredProfile.Read(value.GetProperty("profile"))) : default, WordingVersion = value.TryGetProperty("wording_version", out _) ? (InputPresence<long>)(value.GetProperty("wording_version").GetInt64()) : default };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); if (ContextSchema.IsPresent) { writer.WritePropertyName("context_schema"); if (ContextSchema.Value is null) writer.WriteNullValue(); else ContextSchema.Value.Write(writer); } writer.WritePropertyName("find"); if (Find is null) writer.WriteNullValue(); else Find.Write(writer); if (ItemSchema.IsPresent) { writer.WritePropertyName("item_schema"); if (ItemSchema.Value is null) writer.WriteNullValue(); else ItemSchema.Value.Write(writer); } if (Model.IsPresent) { writer.WritePropertyName("model"); if (Model.Value is null) writer.WriteNullValue(); else Model.Value.Write(writer); } if (Name.IsPresent) { writer.WritePropertyName("name"); WriteText(writer, Name.Value); } if (On.IsPresent) { writer.WritePropertyName("on"); if (On.Value is null) writer.WriteNullValue(); else On.Value.Write(writer); } if (Profile.IsPresent) { writer.WritePropertyName("profile"); if (Profile.Value is null) writer.WriteNullValue(); else Profile.Value.Write(writer); } if (WordingVersion.IsPresent) { writer.WritePropertyName("wording_version"); writer.WriteNumberValue(WordingVersion.Value); } writer.WriteEndObject(); }
}

public abstract class InputAuthoredInputDeclaration : InputDocument { private protected InputAuthoredInputDeclaration() {} internal static InputAuthoredInputDeclaration Read(JsonElement value) { if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("type", out _) && (value.GetProperty("type").ValueKind == JsonValueKind.String && value.GetProperty("type").GetString() == "string")) return InputAuthoredInputDeclarationString.Read(value); if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("type", out _) && value.TryGetProperty("properties", out _) && (value.GetProperty("type").ValueKind == JsonValueKind.String && value.GetProperty("type").GetString() == "object")) return InputAuthoredInputDeclarationObject.Read(value); throw new JsonException("Authored data does not match the native Request schema."); } }

public sealed class InputAuthoredInputDeclarationObject : InputAuthoredInputDeclaration {
public required IReadOnlyDictionary<string, InputAuthoredInputProperty> Properties { get; init; }
public InputPresence<IReadOnlyList<string>> Required { get; init; }
internal new static InputAuthoredInputDeclarationObject Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Properties = new System.Collections.ObjectModel.ReadOnlyDictionary<string, InputAuthoredInputProperty>(value.GetProperty("properties").EnumerateObject().ToDictionary(entry0 => entry0.Name, entry0 => InputAuthoredInputProperty.Read(entry0.Value))), Required = value.TryGetProperty("required", out _) ? (InputPresence<IReadOnlyList<string>>)(Array.AsReadOnly(value.GetProperty("required").EnumerateArray().Select(item0 => item0.GetString()!).ToArray())) : default };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("properties"); writer.WriteStartObject(); foreach (var entry in Properties) { WriteName(writer, entry.Key); if (entry.Value is null) writer.WriteNullValue(); else entry.Value.Write(writer); } writer.WriteEndObject(); if (Required.IsPresent) { writer.WritePropertyName("required"); writer.WriteStartArray(); foreach (var item in Required.Value) { WriteText(writer, item); } writer.WriteEndArray(); } writer.WritePropertyName("type"); WriteText(writer, "object"); writer.WriteEndObject(); }
}

public sealed class InputAuthoredInputDeclarationString : InputAuthoredInputDeclaration {
internal new static InputAuthoredInputDeclarationString Read(JsonElement value) => new() { ParsedDocument = value.Clone() };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("type"); WriteText(writer, "string"); writer.WriteEndObject(); }
}

public abstract class InputAuthoredInputProperty : InputDocument { private protected InputAuthoredInputProperty() {} internal static InputAuthoredInputProperty Read(JsonElement value) { if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("type", out _) && (value.GetProperty("type").ValueKind == JsonValueKind.String && value.GetProperty("type").GetString() == "string")) return InputAuthoredInputPropertyString.Read(value); if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("type", out _) && (value.GetProperty("type").ValueKind == JsonValueKind.String && value.GetProperty("type").GetString() == "number")) return InputAuthoredInputPropertyNumber.Read(value); if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("type", out _) && (value.GetProperty("type").ValueKind == JsonValueKind.String && value.GetProperty("type").GetString() == "boolean")) return InputAuthoredInputPropertyBoolean.Read(value); if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("type", out _) && value.TryGetProperty("items", out _) && (value.GetProperty("type").ValueKind == JsonValueKind.String && value.GetProperty("type").GetString() == "array")) return InputAuthoredInputPropertyArray.Read(value); throw new JsonException("Authored data does not match the native Request schema."); } }

public sealed class InputAuthoredInputPropertyArray : InputAuthoredInputProperty {
public required InputAuthoredInputPropertyArrayItems Items { get; init; }
internal new static InputAuthoredInputPropertyArray Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Items = InputAuthoredInputPropertyArrayItems.Read(value.GetProperty("items")) };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("items"); if (Items is null) writer.WriteNullValue(); else Items.Write(writer); writer.WritePropertyName("type"); WriteText(writer, "array"); writer.WriteEndObject(); }
}

public sealed class InputAuthoredInputPropertyArrayItems : InputDocument {
internal static InputAuthoredInputPropertyArrayItems Read(JsonElement value) => new() { ParsedDocument = value.Clone() };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("type"); WriteText(writer, "string"); writer.WriteEndObject(); }
}

public sealed class InputAuthoredInputPropertyBoolean : InputAuthoredInputProperty {
internal new static InputAuthoredInputPropertyBoolean Read(JsonElement value) => new() { ParsedDocument = value.Clone() };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("type"); WriteText(writer, "boolean"); writer.WriteEndObject(); }
}

public sealed class InputAuthoredInputPropertyNumber : InputAuthoredInputProperty {
internal new static InputAuthoredInputPropertyNumber Read(JsonElement value) => new() { ParsedDocument = value.Clone() };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("type"); WriteText(writer, "number"); writer.WriteEndObject(); }
}

public sealed class InputAuthoredInputPropertyString : InputAuthoredInputProperty {
internal new static InputAuthoredInputPropertyString Read(JsonElement value) => new() { ParsedDocument = value.Clone() };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("type"); WriteText(writer, "string"); writer.WriteEndObject(); }
}

public abstract class InputAuthoredLabels : InputDocument { private protected InputAuthoredLabels() {} internal static InputAuthoredLabels Read(JsonElement value) { if ((value.ValueKind == JsonValueKind.Array)) return InputAuthoredLabelsAlternative0.Read(value); if ((value.ValueKind == JsonValueKind.Object)) return InputAuthoredLabelsAlternative1.Read(value); throw new JsonException("Authored data does not match the native Request schema."); } }

public sealed class InputAuthoredLabelsAlternative0 : InputAuthoredLabels {
public required IReadOnlyList<InputAuthoredName> Value { get; init; }
internal new static InputAuthoredLabelsAlternative0 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = Array.AsReadOnly(value.EnumerateArray().Select(item0 => InputAuthoredName.Read(item0)).ToArray()) };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartArray(); foreach (var item in Value) { if (item is null) writer.WriteNullValue(); else item.Write(writer); } writer.WriteEndArray(); }
}

public sealed class InputAuthoredLabelsAlternative1 : InputAuthoredLabels {
public required IReadOnlyDictionary<string, InputAuthoredDescription> Value { get; init; }
internal new static InputAuthoredLabelsAlternative1 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = new System.Collections.ObjectModel.ReadOnlyDictionary<string, InputAuthoredDescription>(value.EnumerateObject().ToDictionary(entry0 => entry0.Name, entry0 => InputAuthoredDescription.Read(entry0.Value))) };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); foreach (var entry in Value) { WriteName(writer, entry.Key); if (entry.Value is null) writer.WriteNullValue(); else entry.Value.Write(writer); } writer.WriteEndObject(); }
}

public abstract class InputAuthoredLevels : InputDocument { private protected InputAuthoredLevels() {} internal static InputAuthoredLevels Read(JsonElement value) { if ((value.ValueKind == JsonValueKind.Array)) return InputAuthoredLevelsAlternative0.Read(value); if ((value.ValueKind == JsonValueKind.Object)) return InputAuthoredLevelsAlternative1.Read(value); throw new JsonException("Authored data does not match the native Request schema."); } }

public sealed class InputAuthoredLevelsAlternative0 : InputAuthoredLevels {
public required IReadOnlyList<InputAuthoredName> Value { get; init; }
internal new static InputAuthoredLevelsAlternative0 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = Array.AsReadOnly(value.EnumerateArray().Select(item0 => InputAuthoredName.Read(item0)).ToArray()) };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartArray(); foreach (var item in Value) { if (item is null) writer.WriteNullValue(); else item.Write(writer); } writer.WriteEndArray(); }
}

public sealed class InputAuthoredLevelsAlternative1 : InputAuthoredLevels {
public required IReadOnlyDictionary<string, InputAuthoredCriterion> Value { get; init; }
internal new static InputAuthoredLevelsAlternative1 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = new System.Collections.ObjectModel.ReadOnlyDictionary<string, InputAuthoredCriterion>(value.EnumerateObject().ToDictionary(entry0 => entry0.Name, entry0 => InputAuthoredCriterion.Read(entry0.Value))) };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); foreach (var entry in Value) { WriteName(writer, entry.Key); if (entry.Value is null) writer.WriteNullValue(); else entry.Value.Write(writer); } writer.WriteEndObject(); }
}

public sealed class InputAuthoredName : InputDocument {
public required string Value { get; init; }
internal static InputAuthoredName Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = value.GetString()! };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } WriteText(writer, Value); }
}

public abstract class InputAuthoredOptions : InputDocument { private protected InputAuthoredOptions() {} internal static InputAuthoredOptions Read(JsonElement value) { if ((value.ValueKind == JsonValueKind.Array)) return InputAuthoredOptionsAlternative0.Read(value); if ((value.ValueKind == JsonValueKind.Object)) return InputAuthoredOptionsAlternative1.Read(value); throw new JsonException("Authored data does not match the native Request schema."); } }

public sealed class InputAuthoredOptionsAlternative0 : InputAuthoredOptions {
public required IReadOnlyList<InputAuthoredName> Value { get; init; }
internal new static InputAuthoredOptionsAlternative0 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = Array.AsReadOnly(value.EnumerateArray().Select(item0 => InputAuthoredName.Read(item0)).ToArray()) };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartArray(); foreach (var item in Value) { if (item is null) writer.WriteNullValue(); else item.Write(writer); } writer.WriteEndArray(); }
}

public sealed class InputAuthoredOptionsAlternative1 : InputAuthoredOptions {
public required IReadOnlyDictionary<string, InputAuthoredDescription> Value { get; init; }
internal new static InputAuthoredOptionsAlternative1 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = new System.Collections.ObjectModel.ReadOnlyDictionary<string, InputAuthoredDescription>(value.EnumerateObject().ToDictionary(entry0 => entry0.Name, entry0 => InputAuthoredDescription.Read(entry0.Value))) };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); foreach (var entry in Value) { WriteName(writer, entry.Key); if (entry.Value is null) writer.WriteNullValue(); else entry.Value.Write(writer); } writer.WriteEndObject(); }
}

public abstract class InputAuthoredPointers : InputDocument { private protected InputAuthoredPointers() {} internal static InputAuthoredPointers Read(JsonElement value) { if ((value.ValueKind == JsonValueKind.String)) return InputAuthoredPointersAlternative0.Read(value); if ((value.ValueKind == JsonValueKind.Array)) return InputAuthoredPointersAlternative1.Read(value); throw new JsonException("Authored data does not match the native Request schema."); } }

public sealed class InputAuthoredPointersAlternative0 : InputAuthoredPointers {
public required string Value { get; init; }
internal new static InputAuthoredPointersAlternative0 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = value.GetString()! };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } WriteText(writer, Value); }
}

public sealed class InputAuthoredPointersAlternative1 : InputAuthoredPointers {
public required IReadOnlyList<string> Value { get; init; }
internal new static InputAuthoredPointersAlternative1 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = Array.AsReadOnly(value.EnumerateArray().Select(item0 => item0.GetString()!).ToArray()) };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartArray(); foreach (var item in Value) { WriteText(writer, item); } writer.WriteEndArray(); }
}

public sealed class InputAuthoredProfile : InputDocument {
public required string Value { get; init; }
internal static InputAuthoredProfile Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = value.GetString()! };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } WriteText(writer, Value); }
}

public abstract class InputAuthoredQuestionText : InputDocument { private protected InputAuthoredQuestionText() {} internal static InputAuthoredQuestionText Read(JsonElement value) { if ((value.ValueKind == JsonValueKind.String)) return InputAuthoredQuestionTextAlternative0.Read(value); if ((value.ValueKind == JsonValueKind.Object)) return InputAuthoredQuestionTextAlternative1.Read(value); if ((value.ValueKind == JsonValueKind.Array)) return InputAuthoredQuestionTextAlternative2.Read(value); throw new JsonException("Authored data does not match the native Request schema."); } }

public sealed class InputAuthoredQuestionTextAlternative0 : InputAuthoredQuestionText {
public required string Value { get; init; }
internal new static InputAuthoredQuestionTextAlternative0 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = value.GetString()! };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } WriteText(writer, Value); }
}

public sealed class InputAuthoredQuestionTextAlternative1 : InputAuthoredQuestionText {
public required JsonElement Value { get; init; }
internal new static InputAuthoredQuestionTextAlternative1 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = value.Clone() };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } Value.WriteTo(writer); }
}

public sealed class InputAuthoredQuestionTextAlternative2 : InputAuthoredQuestionText {
public required IReadOnlyList<JsonElement> Value { get; init; }
internal new static InputAuthoredQuestionTextAlternative2 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = Array.AsReadOnly(value.EnumerateArray().Select(item0 => item0.Clone()).ToArray()) };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartArray(); foreach (var item in Value) { item.WriteTo(writer); } writer.WriteEndArray(); }
}

public sealed class InputAuthoredRelate : InputDocument {
public InputPresence<InputAuthoredInputDeclaration> ContextSchema { get; init; }
public InputPresence<InputAuthoredInputDeclaration> ItemSchema { get; init; }
public InputPresence<InputAuthoredName> Model { get; init; }
public InputPresence<string> Name { get; init; }
public InputPresence<InputAuthoredProfile> Profile { get; init; }
public required InputAuthoredRelateRelate Relate { get; init; }
public InputPresence<InputAuthoredCut> Threshold { get; init; }
public InputPresence<long> WordingVersion { get; init; }
internal static InputAuthoredRelate Read(JsonElement value) => new() { ParsedDocument = value.Clone(), ContextSchema = value.TryGetProperty("context_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("context_schema"))) : default, ItemSchema = value.TryGetProperty("item_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("item_schema"))) : default, Model = value.TryGetProperty("model", out _) ? (InputPresence<InputAuthoredName>)(InputAuthoredName.Read(value.GetProperty("model"))) : default, Name = value.TryGetProperty("name", out _) ? (InputPresence<string>)(value.GetProperty("name").GetString()!) : default, Profile = value.TryGetProperty("profile", out _) ? (InputPresence<InputAuthoredProfile>)(InputAuthoredProfile.Read(value.GetProperty("profile"))) : default, Relate = InputAuthoredRelateRelate.Read(value.GetProperty("relate")), Threshold = value.TryGetProperty("threshold", out _) ? (InputPresence<InputAuthoredCut>)(InputAuthoredCut.Read(value.GetProperty("threshold"))) : default, WordingVersion = value.TryGetProperty("wording_version", out _) ? (InputPresence<long>)(value.GetProperty("wording_version").GetInt64()) : default };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); if (ContextSchema.IsPresent) { writer.WritePropertyName("context_schema"); if (ContextSchema.Value is null) writer.WriteNullValue(); else ContextSchema.Value.Write(writer); } if (ItemSchema.IsPresent) { writer.WritePropertyName("item_schema"); if (ItemSchema.Value is null) writer.WriteNullValue(); else ItemSchema.Value.Write(writer); } if (Model.IsPresent) { writer.WritePropertyName("model"); if (Model.Value is null) writer.WriteNullValue(); else Model.Value.Write(writer); } if (Name.IsPresent) { writer.WritePropertyName("name"); WriteText(writer, Name.Value); } if (Profile.IsPresent) { writer.WritePropertyName("profile"); if (Profile.Value is null) writer.WriteNullValue(); else Profile.Value.Write(writer); } writer.WritePropertyName("relate"); if (Relate is null) writer.WriteNullValue(); else Relate.Write(writer); if (Threshold.IsPresent) { writer.WritePropertyName("threshold"); if (Threshold.Value is null) writer.WriteNullValue(); else Threshold.Value.Write(writer); } writer.WritePropertyName("version"); writer.WriteNumberValue(1); if (WordingVersion.IsPresent) { writer.WritePropertyName("wording_version"); writer.WriteNumberValue(WordingVersion.Value); } writer.WriteEndObject(); }
}

public sealed class InputAuthoredRelateRelate : InputDocument {
public InputPresence<InputAuthoredRelateRelateFields> Fields { get; init; }
public required IReadOnlyList<InputAuthoredRelation> Relations { get; init; }
internal static InputAuthoredRelateRelate Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Fields = value.TryGetProperty("fields", out _) ? (InputPresence<InputAuthoredRelateRelateFields>)(InputAuthoredRelateRelateFields.Read(value.GetProperty("fields"))) : default, Relations = Array.AsReadOnly(value.GetProperty("relations").EnumerateArray().Select(item0 => InputAuthoredRelation.Read(item0)).ToArray()) };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); if (Fields.IsPresent) { writer.WritePropertyName("fields"); if (Fields.Value is null) writer.WriteNullValue(); else Fields.Value.Write(writer); } writer.WritePropertyName("relations"); writer.WriteStartArray(); foreach (var item in Relations) { if (item is null) writer.WriteNullValue(); else item.Write(writer); } writer.WriteEndArray(); writer.WriteEndObject(); }
}

public sealed class InputAuthoredRelateRelateFields : InputDocument {
public required string Kind { get; init; }
public required string Name { get; init; }
internal static InputAuthoredRelateRelateFields Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Kind = value.GetProperty("kind").GetString()!, Name = value.GetProperty("name").GetString()! };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("kind"); WriteText(writer, Kind); writer.WritePropertyName("name"); WriteText(writer, Name); writer.WriteEndObject(); }
}

public sealed class InputAuthoredRelation : InputDocument {
public InputPresence<bool> Either { get; init; }
public required InputAuthoredName Name { get; init; }
public InputPresence<InputAuthoredName> Reads { get; init; }
public InputPresence<bool> Single { get; init; }
public InputPresence<InputAuthoredName> Source { get; init; }
public InputPresence<InputAuthoredName> Target { get; init; }
internal static InputAuthoredRelation Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Either = value.TryGetProperty("either", out _) ? (InputPresence<bool>)(value.GetProperty("either").GetBoolean()) : default, Name = InputAuthoredName.Read(value.GetProperty("name")), Reads = value.TryGetProperty("reads", out _) ? (InputPresence<InputAuthoredName>)(InputAuthoredName.Read(value.GetProperty("reads"))) : default, Single = value.TryGetProperty("single", out _) ? (InputPresence<bool>)(value.GetProperty("single").GetBoolean()) : default, Source = value.TryGetProperty("source", out _) ? (InputPresence<InputAuthoredName>)(InputAuthoredName.Read(value.GetProperty("source"))) : default, Target = value.TryGetProperty("target", out _) ? (InputPresence<InputAuthoredName>)(InputAuthoredName.Read(value.GetProperty("target"))) : default };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); if (Either.IsPresent) { writer.WritePropertyName("either"); writer.WriteBooleanValue(Either.Value); } writer.WritePropertyName("name"); if (Name is null) writer.WriteNullValue(); else Name.Write(writer); if (Reads.IsPresent) { writer.WritePropertyName("reads"); if (Reads.Value is null) writer.WriteNullValue(); else Reads.Value.Write(writer); } if (Single.IsPresent) { writer.WritePropertyName("single"); writer.WriteBooleanValue(Single.Value); } if (Source.IsPresent) { writer.WritePropertyName("source"); if (Source.Value is null) writer.WriteNullValue(); else Source.Value.Write(writer); } if (Target.IsPresent) { writer.WritePropertyName("target"); if (Target.Value is null) writer.WriteNullValue(); else Target.Value.Write(writer); } writer.WriteEndObject(); }
}

public sealed class InputAuthoredScore : InputDocument {
public InputPresence<InputAuthoredScoreBatch> Batch { get; init; }
public InputPresence<InputAuthoredInputDeclaration> ContextSchema { get; init; }
public InputPresence<InputAuthoredInputDeclaration> ItemSchema { get; init; }
public InputPresence<InputAuthoredLevels> Levels { get; init; }
public InputPresence<InputAuthoredName> Model { get; init; }
public InputPresence<string> Name { get; init; }
public InputPresence<InputAuthoredPointers> On { get; init; }
public InputPresence<InputAuthoredProfile> Profile { get; init; }
public required InputAuthoredQuestionText Score { get; init; }
public InputPresence<long> WordingVersion { get; init; }
internal static InputAuthoredScore Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Batch = value.TryGetProperty("batch", out _) ? (InputPresence<InputAuthoredScoreBatch>)(InputAuthoredScoreBatch.Read(value.GetProperty("batch"))) : default, ContextSchema = value.TryGetProperty("context_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("context_schema"))) : default, ItemSchema = value.TryGetProperty("item_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("item_schema"))) : default, Levels = value.TryGetProperty("levels", out _) ? (InputPresence<InputAuthoredLevels>)(InputAuthoredLevels.Read(value.GetProperty("levels"))) : default, Model = value.TryGetProperty("model", out _) ? (InputPresence<InputAuthoredName>)(InputAuthoredName.Read(value.GetProperty("model"))) : default, Name = value.TryGetProperty("name", out _) ? (InputPresence<string>)(value.GetProperty("name").GetString()!) : default, On = value.TryGetProperty("on", out _) ? (InputPresence<InputAuthoredPointers>)(InputAuthoredPointers.Read(value.GetProperty("on"))) : default, Profile = value.TryGetProperty("profile", out _) ? (InputPresence<InputAuthoredProfile>)(InputAuthoredProfile.Read(value.GetProperty("profile"))) : default, Score = InputAuthoredQuestionText.Read(value.GetProperty("score")), WordingVersion = value.TryGetProperty("wording_version", out _) ? (InputPresence<long>)(value.GetProperty("wording_version").GetInt64()) : default };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); if (Batch.IsPresent) { writer.WritePropertyName("batch"); if (Batch.Value is null) writer.WriteNullValue(); else Batch.Value.Write(writer); } if (ContextSchema.IsPresent) { writer.WritePropertyName("context_schema"); if (ContextSchema.Value is null) writer.WriteNullValue(); else ContextSchema.Value.Write(writer); } if (ItemSchema.IsPresent) { writer.WritePropertyName("item_schema"); if (ItemSchema.Value is null) writer.WriteNullValue(); else ItemSchema.Value.Write(writer); } if (Levels.IsPresent) { writer.WritePropertyName("levels"); if (Levels.Value is null) writer.WriteNullValue(); else Levels.Value.Write(writer); } if (Model.IsPresent) { writer.WritePropertyName("model"); if (Model.Value is null) writer.WriteNullValue(); else Model.Value.Write(writer); } if (Name.IsPresent) { writer.WritePropertyName("name"); WriteText(writer, Name.Value); } if (On.IsPresent) { writer.WritePropertyName("on"); if (On.Value is null) writer.WriteNullValue(); else On.Value.Write(writer); } if (Profile.IsPresent) { writer.WritePropertyName("profile"); if (Profile.Value is null) writer.WriteNullValue(); else Profile.Value.Write(writer); } writer.WritePropertyName("score"); if (Score is null) writer.WriteNullValue(); else Score.Write(writer); if (WordingVersion.IsPresent) { writer.WritePropertyName("wording_version"); writer.WriteNumberValue(WordingVersion.Value); } writer.WriteEndObject(); }
}

public abstract class InputAuthoredScoreBatch : InputDocument { private protected InputAuthoredScoreBatch() {} internal static InputAuthoredScoreBatch Read(JsonElement value) { if ((value.ValueKind == JsonValueKind.String && value.GetString() == "max")) return InputAuthoredScoreBatchAlternative0.Read(value); if ((value.ValueKind == JsonValueKind.Number)) return InputAuthoredScoreBatchAlternative1.Read(value); throw new JsonException("Authored data does not match the native Request schema."); } }

public sealed class InputAuthoredScoreBatchAlternative0 : InputAuthoredScoreBatch {
internal new static InputAuthoredScoreBatchAlternative0 Read(JsonElement value) => new() { ParsedDocument = value.Clone() };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } WriteText(writer, "max"); }
}

public sealed class InputAuthoredScoreBatchAlternative1 : InputAuthoredScoreBatch {
public required long Value { get; init; }
internal new static InputAuthoredScoreBatchAlternative1 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = value.GetInt64() };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteNumberValue(Value); }
}

public sealed class InputAuthoredTag : InputDocument {
public InputPresence<InputAuthoredTagBatch> Batch { get; init; }
public InputPresence<InputAuthoredInputDeclaration> ContextSchema { get; init; }
public InputPresence<InputAuthoredInputDeclaration> ItemSchema { get; init; }
public InputPresence<InputAuthoredLabels> Labels { get; init; }
public InputPresence<InputAuthoredName> Model { get; init; }
public InputPresence<string> Name { get; init; }
public InputPresence<InputAuthoredPointers> On { get; init; }
public InputPresence<InputAuthoredProfile> Profile { get; init; }
public required InputAuthoredQuestionText Tag { get; init; }
public InputPresence<InputAuthoredCut> Threshold { get; init; }
public InputPresence<long> WordingVersion { get; init; }
internal static InputAuthoredTag Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Batch = value.TryGetProperty("batch", out _) ? (InputPresence<InputAuthoredTagBatch>)(InputAuthoredTagBatch.Read(value.GetProperty("batch"))) : default, ContextSchema = value.TryGetProperty("context_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("context_schema"))) : default, ItemSchema = value.TryGetProperty("item_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("item_schema"))) : default, Labels = value.TryGetProperty("labels", out _) ? (InputPresence<InputAuthoredLabels>)(InputAuthoredLabels.Read(value.GetProperty("labels"))) : default, Model = value.TryGetProperty("model", out _) ? (InputPresence<InputAuthoredName>)(InputAuthoredName.Read(value.GetProperty("model"))) : default, Name = value.TryGetProperty("name", out _) ? (InputPresence<string>)(value.GetProperty("name").GetString()!) : default, On = value.TryGetProperty("on", out _) ? (InputPresence<InputAuthoredPointers>)(InputAuthoredPointers.Read(value.GetProperty("on"))) : default, Profile = value.TryGetProperty("profile", out _) ? (InputPresence<InputAuthoredProfile>)(InputAuthoredProfile.Read(value.GetProperty("profile"))) : default, Tag = InputAuthoredQuestionText.Read(value.GetProperty("tag")), Threshold = value.TryGetProperty("threshold", out _) ? (InputPresence<InputAuthoredCut>)(InputAuthoredCut.Read(value.GetProperty("threshold"))) : default, WordingVersion = value.TryGetProperty("wording_version", out _) ? (InputPresence<long>)(value.GetProperty("wording_version").GetInt64()) : default };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); if (Batch.IsPresent) { writer.WritePropertyName("batch"); if (Batch.Value is null) writer.WriteNullValue(); else Batch.Value.Write(writer); } if (ContextSchema.IsPresent) { writer.WritePropertyName("context_schema"); if (ContextSchema.Value is null) writer.WriteNullValue(); else ContextSchema.Value.Write(writer); } if (ItemSchema.IsPresent) { writer.WritePropertyName("item_schema"); if (ItemSchema.Value is null) writer.WriteNullValue(); else ItemSchema.Value.Write(writer); } if (Labels.IsPresent) { writer.WritePropertyName("labels"); if (Labels.Value is null) writer.WriteNullValue(); else Labels.Value.Write(writer); } if (Model.IsPresent) { writer.WritePropertyName("model"); if (Model.Value is null) writer.WriteNullValue(); else Model.Value.Write(writer); } if (Name.IsPresent) { writer.WritePropertyName("name"); WriteText(writer, Name.Value); } if (On.IsPresent) { writer.WritePropertyName("on"); if (On.Value is null) writer.WriteNullValue(); else On.Value.Write(writer); } if (Profile.IsPresent) { writer.WritePropertyName("profile"); if (Profile.Value is null) writer.WriteNullValue(); else Profile.Value.Write(writer); } writer.WritePropertyName("tag"); if (Tag is null) writer.WriteNullValue(); else Tag.Write(writer); if (Threshold.IsPresent) { writer.WritePropertyName("threshold"); if (Threshold.Value is null) writer.WriteNullValue(); else Threshold.Value.Write(writer); } if (WordingVersion.IsPresent) { writer.WritePropertyName("wording_version"); writer.WriteNumberValue(WordingVersion.Value); } writer.WriteEndObject(); }
}

public abstract class InputAuthoredTagBatch : InputDocument { private protected InputAuthoredTagBatch() {} internal static InputAuthoredTagBatch Read(JsonElement value) { if ((value.ValueKind == JsonValueKind.String && value.GetString() == "max")) return InputAuthoredTagBatchAlternative0.Read(value); if ((value.ValueKind == JsonValueKind.Number)) return InputAuthoredTagBatchAlternative1.Read(value); throw new JsonException("Authored data does not match the native Request schema."); } }

public sealed class InputAuthoredTagBatchAlternative0 : InputAuthoredTagBatch {
internal new static InputAuthoredTagBatchAlternative0 Read(JsonElement value) => new() { ParsedDocument = value.Clone() };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } WriteText(writer, "max"); }
}

public sealed class InputAuthoredTagBatchAlternative1 : InputAuthoredTagBatch {
public required long Value { get; init; }
internal new static InputAuthoredTagBatchAlternative1 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = value.GetInt64() };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteNumberValue(Value); }
}

public abstract class InputAuthoredThreshold : InputDocument { private protected InputAuthoredThreshold() {} internal static InputAuthoredThreshold Read(JsonElement value) { if ((value.ValueKind == JsonValueKind.Number)) return InputAuthoredThresholdAlternative0.Read(value); if ((value.ValueKind == JsonValueKind.String)) return InputAuthoredThresholdAlternative1.Read(value); throw new JsonException("Authored data does not match the native Request schema."); } }

public sealed class InputAuthoredThresholdAlternative0 : InputAuthoredThreshold {
public required double Value { get; init; }
internal new static InputAuthoredThresholdAlternative0 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = value.GetDouble() };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteNumberValue(Value); }
}

public sealed class InputAuthoredThresholdAlternative1 : InputAuthoredThreshold {
public required string Value { get; init; }
internal new static InputAuthoredThresholdAlternative1 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = value.GetString()! };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } WriteText(writer, Value); }
}

public abstract class InputCacheDocument : InputDocument { private protected InputCacheDocument() {} }

public sealed class InputCacheDocumentAlternative0 : InputCacheDocument {
public required string Value { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } WriteText(writer, Value); }
}

public sealed class InputCacheDocumentAlternative1 : InputCacheDocument {
public required InputDisabledCache Value { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } if (Value is null) writer.WriteNullValue(); else Value.Write(writer); }
}

public abstract class InputContextSchema : InputDocument { private protected InputContextSchema() {} }

public sealed class InputContextSchemaAlternative0 : InputContextSchema {
public required string Value { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } WriteText(writer, Value); }
}

public sealed class InputContextSchemaAlternative1 : InputContextSchema {
public required JsonElement Value { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } Value.WriteTo(writer); }
}

public sealed class InputDisabledCache : InputDocument {
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteBooleanValue(false); }
}

public sealed class InputEngineSettings : InputDocument {
public InputPresence<string> Backend { get; init; }
public InputPresence<string> BaseUrl { get; init; }
public InputPresence<InputRequestBatch> Batch { get; init; }
public InputPresence<InputCacheDocument> Cache { get; init; }
public InputPresence<InputEngineSettingsMaxEstimatedInputTokensTotal> MaxEstimatedInputTokensTotal { get; init; }
public InputPresence<ulong> MaxRequestBytes { get; init; }
public InputPresence<InputEngineSettingsMaxRequests> MaxRequests { get; init; }
public InputPresence<InputEngineSettingsMaxRequestsTotal> MaxRequestsTotal { get; init; }
public InputPresence<uint> MaxRetries { get; init; }
public InputPresence<string> Model { get; init; }
public InputPresence<string> Profile { get; init; }
public InputPresence<JsonElement> Proxy { get; init; }
public InputPresence<string> Record { get; init; }
public InputPresence<bool> RefreshCache { get; init; }
public InputPresence<string> Replay { get; init; }
public InputPresence<long> Throttle { get; init; }
public InputPresence<ulong> Timeout { get; init; }
public InputPresence<string> UsdPerMillionInput { get; init; }
public InputPresence<string> UsdPerMillionOutput { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); if (Backend.IsPresent) { writer.WritePropertyName("backend"); WriteText(writer, Backend.Value); } if (BaseUrl.IsPresent) { writer.WritePropertyName("base_url"); WriteText(writer, BaseUrl.Value); } if (Batch.IsPresent) { writer.WritePropertyName("batch"); if (Batch.Value is null) writer.WriteNullValue(); else Batch.Value.Write(writer); } if (Cache.IsPresent) { writer.WritePropertyName("cache"); if (Cache.Value is null) writer.WriteNullValue(); else Cache.Value.Write(writer); } if (MaxEstimatedInputTokensTotal.IsPresent) { writer.WritePropertyName("max_estimated_input_tokens_total"); if (MaxEstimatedInputTokensTotal.Value is null) writer.WriteNullValue(); else MaxEstimatedInputTokensTotal.Value.Write(writer); } if (MaxRequestBytes.IsPresent) { writer.WritePropertyName("max_request_bytes"); writer.WriteNumberValue(MaxRequestBytes.Value); } if (MaxRequests.IsPresent) { writer.WritePropertyName("max_requests"); if (MaxRequests.Value is null) writer.WriteNullValue(); else MaxRequests.Value.Write(writer); } if (MaxRequestsTotal.IsPresent) { writer.WritePropertyName("max_requests_total"); if (MaxRequestsTotal.Value is null) writer.WriteNullValue(); else MaxRequestsTotal.Value.Write(writer); } if (MaxRetries.IsPresent) { writer.WritePropertyName("max_retries"); writer.WriteNumberValue(MaxRetries.Value); } if (Model.IsPresent) { writer.WritePropertyName("model"); WriteText(writer, Model.Value); } if (Profile.IsPresent) { writer.WritePropertyName("profile"); WriteText(writer, Profile.Value); } if (Proxy.IsPresent) { writer.WritePropertyName("proxy"); Proxy.Value.WriteTo(writer); } if (Record.IsPresent) { writer.WritePropertyName("record"); WriteText(writer, Record.Value); } if (RefreshCache.IsPresent) { writer.WritePropertyName("refresh_cache"); writer.WriteBooleanValue(RefreshCache.Value); } if (Replay.IsPresent) { writer.WritePropertyName("replay"); WriteText(writer, Replay.Value); } if (Throttle.IsPresent) { writer.WritePropertyName("throttle"); writer.WriteNumberValue(Throttle.Value); } if (Timeout.IsPresent) { writer.WritePropertyName("timeout"); writer.WriteNumberValue(Timeout.Value); } if (UsdPerMillionInput.IsPresent) { writer.WritePropertyName("usd_per_million_input"); WriteText(writer, UsdPerMillionInput.Value); } if (UsdPerMillionOutput.IsPresent) { writer.WritePropertyName("usd_per_million_output"); WriteText(writer, UsdPerMillionOutput.Value); } writer.WriteEndObject(); }
}

public abstract class InputEngineSettingsMaxEstimatedInputTokensTotal : InputDocument { private protected InputEngineSettingsMaxEstimatedInputTokensTotal() {} }

public sealed class InputEngineSettingsMaxEstimatedInputTokensTotalAlternative0 : InputEngineSettingsMaxEstimatedInputTokensTotal {
public required ulong Value { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteNumberValue(Value); }
}

public sealed class InputEngineSettingsMaxEstimatedInputTokensTotalAlternative1 : InputEngineSettingsMaxEstimatedInputTokensTotal {
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteNullValue(); }
}

public abstract class InputEngineSettingsMaxRequests : InputDocument { private protected InputEngineSettingsMaxRequests() {} }

public sealed class InputEngineSettingsMaxRequestsAlternative0 : InputEngineSettingsMaxRequests {
public required ulong Value { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteNumberValue(Value); }
}

public sealed class InputEngineSettingsMaxRequestsAlternative1 : InputEngineSettingsMaxRequests {
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteNullValue(); }
}

public abstract class InputEngineSettingsMaxRequestsTotal : InputDocument { private protected InputEngineSettingsMaxRequestsTotal() {} }

public sealed class InputEngineSettingsMaxRequestsTotalAlternative0 : InputEngineSettingsMaxRequestsTotal {
public required ulong Value { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteNumberValue(Value); }
}

public sealed class InputEngineSettingsMaxRequestsTotalAlternative1 : InputEngineSettingsMaxRequestsTotal {
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteNullValue(); }
}

public abstract class InputImageMedia : InputDocument { private protected InputImageMedia() {} }

public sealed class InputImageMediaAlternative0 : InputImageMedia {
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } WriteText(writer, "image/jpeg"); }
}

public sealed class InputImageMediaAlternative1 : InputImageMedia {
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } WriteText(writer, "image/png"); }
}

public sealed class InputOptionSchema : InputDocument {
public InputPresence<JsonElement> Description { get; init; }
public required string Name { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); if (Description.IsPresent) { writer.WritePropertyName("description"); Description.Value.WriteTo(writer); } writer.WritePropertyName("name"); WriteText(writer, Name); writer.WriteEndObject(); }
}

public abstract class InputReaderMedia : InputDocument { private protected InputReaderMedia() {} }

public sealed class InputReaderMediaAlternative0 : InputReaderMedia {
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } WriteText(writer, "text"); }
}

public sealed class InputReaderMediaAlternative1 : InputReaderMedia {
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } WriteText(writer, "image"); }
}

public abstract class InputRecognitionExample : InputDocument { private protected InputRecognitionExample() {} }

public sealed class InputRecognitionExampleEntity : InputDocument {
public required ulong End { get; init; }
public required string Kind { get; init; }
public required ulong Start { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("end"); writer.WriteNumberValue(End); writer.WritePropertyName("kind"); WriteText(writer, Kind); writer.WritePropertyName("start"); writer.WriteNumberValue(Start); writer.WriteEndObject(); }
}

public sealed class InputRecognitionExampleText : InputDocument {
public required IReadOnlyList<InputRecognitionExampleEntity> Entities { get; init; }
public InputPresence<IReadOnlyList<string>> Kinds { get; init; }
public required string Text { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("entities"); writer.WriteStartArray(); foreach (var item in Entities) { if (item is null) writer.WriteNullValue(); else item.Write(writer); } writer.WriteEndArray(); if (Kinds.IsPresent) { writer.WritePropertyName("kinds"); writer.WriteStartArray(); foreach (var item in Kinds.Value) { WriteText(writer, item); } writer.WriteEndArray(); } writer.WritePropertyName("text"); WriteText(writer, Text); writer.WriteEndObject(); }
}

public sealed class InputRecognitionExampleAlternative0 : InputRecognitionExample {
public required string Value { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } WriteText(writer, Value); }
}

public sealed class InputRecognitionExampleAlternative1 : InputRecognitionExample {
public required InputRecognitionExampleText Value { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } if (Value is null) writer.WriteNullValue(); else Value.Write(writer); }
}

public abstract class InputRecognitionMode : InputDocument { private protected InputRecognitionMode() {} internal static InputRecognitionMode Read(JsonElement value) { if ((value.ValueKind == JsonValueKind.String) && (value.ValueKind == JsonValueKind.String && value.GetString() == "whole")) return InputRecognitionModeAlternative0.Read(value); if ((value.ValueKind == JsonValueKind.String) && (value.ValueKind == JsonValueKind.String && value.GetString() == "boundary_only")) return InputRecognitionModeAlternative1.Read(value); throw new JsonException("Authored data does not match the native Request schema."); } }

public sealed class InputRecognitionModeAlternative0 : InputRecognitionMode {
internal new static InputRecognitionModeAlternative0 Read(JsonElement value) => new() { ParsedDocument = value.Clone() };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } WriteText(writer, "whole"); }
}

public sealed class InputRecognitionModeAlternative1 : InputRecognitionMode {
internal new static InputRecognitionModeAlternative1 Read(JsonElement value) => new() { ParsedDocument = value.Clone() };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } WriteText(writer, "boundary_only"); }
}

public sealed class InputRecognitionSeedSpan : InputDocument {
public required ulong End { get; init; }
public InputPresence<string> Kind { get; init; }
public required ulong Start { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("end"); writer.WriteNumberValue(End); if (Kind.IsPresent) { writer.WritePropertyName("kind"); WriteText(writer, Kind.Value); } writer.WritePropertyName("start"); writer.WriteNumberValue(Start); writer.WriteEndObject(); }
}

public sealed class InputRecognitionStageContext : InputDocument {
public InputPresence<string> Boundary { get; init; }
public InputPresence<string> KindEdge { get; init; }
public InputPresence<string> Relation { get; init; }
internal static InputRecognitionStageContext Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Boundary = value.TryGetProperty("boundary", out _) ? (InputPresence<string>)(value.GetProperty("boundary").GetString()!) : default, KindEdge = value.TryGetProperty("kind_edge", out _) ? (InputPresence<string>)(value.GetProperty("kind_edge").GetString()!) : default, Relation = value.TryGetProperty("relation", out _) ? (InputPresence<string>)(value.GetProperty("relation").GetString()!) : default };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); if (Boundary.IsPresent) { writer.WritePropertyName("boundary"); WriteText(writer, Boundary.Value); } if (KindEdge.IsPresent) { writer.WritePropertyName("kind_edge"); WriteText(writer, KindEdge.Value); } if (Relation.IsPresent) { writer.WritePropertyName("relation"); WriteText(writer, Relation.Value); } writer.WriteEndObject(); }
}

public sealed class InputRequest : InputDocument {
public required InputRequestCall Call { get; init; }
public required InputRequestVersion Schema { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("call"); if (Call is null) writer.WriteNullValue(); else Call.Write(writer); writer.WritePropertyName("schema"); if (Schema is null) writer.WriteNullValue(); else Schema.Write(writer); writer.WriteEndObject(); }
}

public abstract class InputRequestBatch : InputDocument { private protected InputRequestBatch() {} }

public sealed class InputRequestBatchAlternative0 : InputRequestBatch {
public required ulong Value { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteNumberValue(Value); }
}

public sealed class InputRequestBatchAlternative1 : InputRequestBatch {
public required string Value { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } WriteText(writer, Value); }
}

public abstract class InputRequestCall : InputDocument { private protected InputRequestCall() {} }

public sealed class InputRequestCallAnnotate : InputRequestCall {
public required InputRequestInput Input { get; init; }
public InputPresence<InputRequestOptions> Options { get; init; }
public required InputRequestQuestion Question { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("function"); WriteText(writer, "annotate"); writer.WritePropertyName("input"); if (Input is null) writer.WriteNullValue(); else Input.Write(writer); if (Options.IsPresent) { writer.WritePropertyName("options"); if (Options.Value is null) writer.WriteNullValue(); else Options.Value.Write(writer); } writer.WritePropertyName("question"); if (Question is null) writer.WriteNullValue(); else Question.Write(writer); writer.WriteEndObject(); }
}

public sealed class InputRequestCallChoose : InputRequestCall {
public required InputRequestInput Input { get; init; }
public InputPresence<InputRequestOptions> Options { get; init; }
public required InputRequestQuestion Question { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("function"); WriteText(writer, "choose"); writer.WritePropertyName("input"); if (Input is null) writer.WriteNullValue(); else Input.Write(writer); if (Options.IsPresent) { writer.WritePropertyName("options"); if (Options.Value is null) writer.WriteNullValue(); else Options.Value.Write(writer); } writer.WritePropertyName("question"); if (Question is null) writer.WriteNullValue(); else Question.Write(writer); writer.WriteEndObject(); }
}

public sealed class InputRequestCallDecide : InputRequestCall {
public required InputRequestInput Input { get; init; }
public InputPresence<InputRequestOptions> Options { get; init; }
public required InputRequestQuestion Question { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("function"); WriteText(writer, "decide"); writer.WritePropertyName("input"); if (Input is null) writer.WriteNullValue(); else Input.Write(writer); if (Options.IsPresent) { writer.WritePropertyName("options"); if (Options.Value is null) writer.WriteNullValue(); else Options.Value.Write(writer); } writer.WritePropertyName("question"); if (Question is null) writer.WriteNullValue(); else Question.Write(writer); writer.WriteEndObject(); }
}

public sealed class InputRequestCallFilter : InputRequestCall {
public required InputRequestInput Input { get; init; }
public InputPresence<InputRequestOptions> Options { get; init; }
public required InputRequestQuestion Question { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("function"); WriteText(writer, "filter"); writer.WritePropertyName("input"); if (Input is null) writer.WriteNullValue(); else Input.Write(writer); if (Options.IsPresent) { writer.WritePropertyName("options"); if (Options.Value is null) writer.WriteNullValue(); else Options.Value.Write(writer); } writer.WritePropertyName("question"); if (Question is null) writer.WriteNullValue(); else Question.Write(writer); writer.WriteEndObject(); }
}

public sealed class InputRequestCallFind : InputRequestCall {
public required InputRequestInput Input { get; init; }
public InputPresence<InputRequestOptions> Options { get; init; }
public required InputRequestQuestion Question { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("function"); WriteText(writer, "find"); writer.WritePropertyName("input"); if (Input is null) writer.WriteNullValue(); else Input.Write(writer); if (Options.IsPresent) { writer.WritePropertyName("options"); if (Options.Value is null) writer.WriteNullValue(); else Options.Value.Write(writer); } writer.WritePropertyName("question"); if (Question is null) writer.WriteNullValue(); else Question.Write(writer); writer.WriteEndObject(); }
}

public sealed class InputRequestCallRank : InputRequestCall {
public required InputRequestInput Input { get; init; }
public InputPresence<InputRequestOptions> Options { get; init; }
public required InputRequestQuestion Question { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("function"); WriteText(writer, "rank"); writer.WritePropertyName("input"); if (Input is null) writer.WriteNullValue(); else Input.Write(writer); if (Options.IsPresent) { writer.WritePropertyName("options"); if (Options.Value is null) writer.WriteNullValue(); else Options.Value.Write(writer); } writer.WritePropertyName("question"); if (Question is null) writer.WriteNullValue(); else Question.Write(writer); writer.WriteEndObject(); }
}

public sealed class InputRequestCallRecognize : InputRequestCall {
public required InputRequestInput Input { get; init; }
public InputPresence<InputRequestOptions> Options { get; init; }
public required InputRequestQuestion Question { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("function"); WriteText(writer, "recognize"); writer.WritePropertyName("input"); if (Input is null) writer.WriteNullValue(); else Input.Write(writer); if (Options.IsPresent) { writer.WritePropertyName("options"); if (Options.Value is null) writer.WriteNullValue(); else Options.Value.Write(writer); } writer.WritePropertyName("question"); if (Question is null) writer.WriteNullValue(); else Question.Write(writer); writer.WriteEndObject(); }
}

public sealed class InputRequestCallRelate : InputRequestCall {
public required InputRequestInput Input { get; init; }
public InputPresence<InputRequestOptions> Options { get; init; }
public required InputRequestQuestion Question { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("function"); WriteText(writer, "relate"); writer.WritePropertyName("input"); if (Input is null) writer.WriteNullValue(); else Input.Write(writer); if (Options.IsPresent) { writer.WritePropertyName("options"); if (Options.Value is null) writer.WriteNullValue(); else Options.Value.Write(writer); } writer.WritePropertyName("question"); if (Question is null) writer.WriteNullValue(); else Question.Write(writer); writer.WriteEndObject(); }
}

public sealed class InputRequestCallScore : InputRequestCall {
public required InputRequestInput Input { get; init; }
public InputPresence<InputRequestOptions> Options { get; init; }
public required InputRequestQuestion Question { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("function"); WriteText(writer, "score"); writer.WritePropertyName("input"); if (Input is null) writer.WriteNullValue(); else Input.Write(writer); if (Options.IsPresent) { writer.WritePropertyName("options"); if (Options.Value is null) writer.WriteNullValue(); else Options.Value.Write(writer); } writer.WritePropertyName("question"); if (Question is null) writer.WriteNullValue(); else Question.Write(writer); writer.WriteEndObject(); }
}

public sealed class InputRequestCallTag : InputRequestCall {
public required InputRequestInput Input { get; init; }
public InputPresence<InputRequestOptions> Options { get; init; }
public required InputRequestQuestion Question { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("function"); WriteText(writer, "tag"); writer.WritePropertyName("input"); if (Input is null) writer.WriteNullValue(); else Input.Write(writer); if (Options.IsPresent) { writer.WritePropertyName("options"); if (Options.Value is null) writer.WriteNullValue(); else Options.Value.Write(writer); } writer.WritePropertyName("question"); if (Question is null) writer.WriteNullValue(); else Question.Write(writer); writer.WriteEndObject(); }
}

public abstract class InputRequestDefinition : InputDocument { private protected InputRequestDefinition() {} internal static InputRequestDefinition Read(JsonElement value) { if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("decide", out _)) return InputRequestDefinitionAlternative0.Read(value); if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("choose", out _)) return InputRequestDefinitionAlternative1.Read(value); if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("tag", out _)) return InputRequestDefinitionAlternative2.Read(value); if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("score", out _)) return InputRequestDefinitionAlternative3.Read(value); if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("version", out _) && value.TryGetProperty("relate", out _) && (value.GetProperty("version").ValueKind == JsonValueKind.Number && value.GetProperty("version").GetInt64() == 1)) return InputRequestDefinitionAlternative4.Read(value); if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("find", out _)) return InputRequestDefinitionAlternative5.Read(value); if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("version", out _) && value.TryGetProperty("recognize", out _) && (value.GetProperty("version").ValueKind == JsonValueKind.Number && value.GetProperty("version").GetInt64() == 1)) return InputRequestDefinitionAlternative6.Read(value); if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("version", out _) && value.TryGetProperty("questions", out _) && (value.GetProperty("version").ValueKind == JsonValueKind.Number && value.GetProperty("version").GetInt64() == 1)) return InputRequestDefinitionAlternative7.Read(value); throw new JsonException("Authored data does not match the native Request schema."); } }

public sealed class InputRequestDefinitionAlternative0 : InputRequestDefinition {
public required InputAuthoredDecide Value { get; init; }
internal new static InputRequestDefinitionAlternative0 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = InputAuthoredDecide.Read(value) };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } if (Value is null) writer.WriteNullValue(); else Value.Write(writer); }
}

public sealed class InputRequestDefinitionAlternative1 : InputRequestDefinition {
public required InputAuthoredChoose Value { get; init; }
internal new static InputRequestDefinitionAlternative1 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = InputAuthoredChoose.Read(value) };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } if (Value is null) writer.WriteNullValue(); else Value.Write(writer); }
}

public sealed class InputRequestDefinitionAlternative2 : InputRequestDefinition {
public required InputAuthoredTag Value { get; init; }
internal new static InputRequestDefinitionAlternative2 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = InputAuthoredTag.Read(value) };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } if (Value is null) writer.WriteNullValue(); else Value.Write(writer); }
}

public sealed class InputRequestDefinitionAlternative3 : InputRequestDefinition {
public required InputAuthoredScore Value { get; init; }
internal new static InputRequestDefinitionAlternative3 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = InputAuthoredScore.Read(value) };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } if (Value is null) writer.WriteNullValue(); else Value.Write(writer); }
}

public sealed class InputRequestDefinitionAlternative4 : InputRequestDefinition {
public required InputAuthoredRelate Value { get; init; }
internal new static InputRequestDefinitionAlternative4 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = InputAuthoredRelate.Read(value) };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } if (Value is null) writer.WriteNullValue(); else Value.Write(writer); }
}

public sealed class InputRequestDefinitionAlternative5 : InputRequestDefinition {
public required InputAuthoredFind Value { get; init; }
internal new static InputRequestDefinitionAlternative5 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Value = InputAuthoredFind.Read(value) };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } if (Value is null) writer.WriteNullValue(); else Value.Write(writer); }
}

public sealed class InputRequestDefinitionAlternative6 : InputRequestDefinition {
public InputPresence<InputAuthoredInputDeclaration> ContextSchema { get; init; }
public InputPresence<InputAuthoredInputDeclaration> ItemSchema { get; init; }
public InputPresence<InputAuthoredName> Model { get; init; }
public InputPresence<string> Name { get; init; }
public InputPresence<InputAuthoredPointers> On { get; init; }
public InputPresence<InputAuthoredProfile> Profile { get; init; }
public required InputRequestDefinitionAlternative6Recognize Recognize { get; init; }
public InputPresence<InputAuthoredCut> RelationThreshold { get; init; }
public InputPresence<InputAuthoredCut> Threshold { get; init; }
public InputPresence<long> WordingVersion { get; init; }
internal new static InputRequestDefinitionAlternative6 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), ContextSchema = value.TryGetProperty("context_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("context_schema"))) : default, ItemSchema = value.TryGetProperty("item_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("item_schema"))) : default, Model = value.TryGetProperty("model", out _) ? (InputPresence<InputAuthoredName>)(InputAuthoredName.Read(value.GetProperty("model"))) : default, Name = value.TryGetProperty("name", out _) ? (InputPresence<string>)(value.GetProperty("name").GetString()!) : default, On = value.TryGetProperty("on", out _) ? (InputPresence<InputAuthoredPointers>)(InputAuthoredPointers.Read(value.GetProperty("on"))) : default, Profile = value.TryGetProperty("profile", out _) ? (InputPresence<InputAuthoredProfile>)(InputAuthoredProfile.Read(value.GetProperty("profile"))) : default, Recognize = InputRequestDefinitionAlternative6Recognize.Read(value.GetProperty("recognize")), RelationThreshold = value.TryGetProperty("relation_threshold", out _) ? (InputPresence<InputAuthoredCut>)(InputAuthoredCut.Read(value.GetProperty("relation_threshold"))) : default, Threshold = value.TryGetProperty("threshold", out _) ? (InputPresence<InputAuthoredCut>)(InputAuthoredCut.Read(value.GetProperty("threshold"))) : default, WordingVersion = value.TryGetProperty("wording_version", out _) ? (InputPresence<long>)(value.GetProperty("wording_version").GetInt64()) : default };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); if (ContextSchema.IsPresent) { writer.WritePropertyName("context_schema"); if (ContextSchema.Value is null) writer.WriteNullValue(); else ContextSchema.Value.Write(writer); } if (ItemSchema.IsPresent) { writer.WritePropertyName("item_schema"); if (ItemSchema.Value is null) writer.WriteNullValue(); else ItemSchema.Value.Write(writer); } if (Model.IsPresent) { writer.WritePropertyName("model"); if (Model.Value is null) writer.WriteNullValue(); else Model.Value.Write(writer); } if (Name.IsPresent) { writer.WritePropertyName("name"); WriteText(writer, Name.Value); } if (On.IsPresent) { writer.WritePropertyName("on"); if (On.Value is null) writer.WriteNullValue(); else On.Value.Write(writer); } if (Profile.IsPresent) { writer.WritePropertyName("profile"); if (Profile.Value is null) writer.WriteNullValue(); else Profile.Value.Write(writer); } writer.WritePropertyName("recognize"); if (Recognize is null) writer.WriteNullValue(); else Recognize.Write(writer); if (RelationThreshold.IsPresent) { writer.WritePropertyName("relation_threshold"); if (RelationThreshold.Value is null) writer.WriteNullValue(); else RelationThreshold.Value.Write(writer); } if (Threshold.IsPresent) { writer.WritePropertyName("threshold"); if (Threshold.Value is null) writer.WriteNullValue(); else Threshold.Value.Write(writer); } writer.WritePropertyName("version"); writer.WriteNumberValue(1); if (WordingVersion.IsPresent) { writer.WritePropertyName("wording_version"); writer.WriteNumberValue(WordingVersion.Value); } writer.WriteEndObject(); }
}

public sealed class InputRequestDefinitionAlternative6Recognize : InputDocument {
public InputPresence<InputAuthoredQuestionText> EntityDefinition { get; init; }
public InputPresence<InputAuthoredQuestionText> Instructions { get; init; }
public InputPresence<IReadOnlyDictionary<string, InputAuthoredDescription>> Kinds { get; init; }
public InputPresence<InputRecognitionMode> Mode { get; init; }
public InputPresence<IReadOnlyList<InputAuthoredRelation>> Relations { get; init; }
public InputPresence<uint> SnippetPieces { get; init; }
public InputPresence<InputRecognitionStageContext> StageContext { get; init; }
internal static InputRequestDefinitionAlternative6Recognize Read(JsonElement value) => new() { ParsedDocument = value.Clone(), EntityDefinition = value.TryGetProperty("entity_definition", out _) ? (InputPresence<InputAuthoredQuestionText>)(InputAuthoredQuestionText.Read(value.GetProperty("entity_definition"))) : default, Instructions = value.TryGetProperty("instructions", out _) ? (InputPresence<InputAuthoredQuestionText>)(InputAuthoredQuestionText.Read(value.GetProperty("instructions"))) : default, Kinds = value.TryGetProperty("kinds", out _) ? (InputPresence<IReadOnlyDictionary<string, InputAuthoredDescription>>)(new System.Collections.ObjectModel.ReadOnlyDictionary<string, InputAuthoredDescription>(value.GetProperty("kinds").EnumerateObject().ToDictionary(entry0 => entry0.Name, entry0 => InputAuthoredDescription.Read(entry0.Value)))) : default, Mode = value.TryGetProperty("mode", out _) ? (InputPresence<InputRecognitionMode>)(InputRecognitionMode.Read(value.GetProperty("mode"))) : default, Relations = value.TryGetProperty("relations", out _) ? (InputPresence<IReadOnlyList<InputAuthoredRelation>>)(Array.AsReadOnly(value.GetProperty("relations").EnumerateArray().Select(item0 => InputAuthoredRelation.Read(item0)).ToArray())) : default, SnippetPieces = value.TryGetProperty("snippet_pieces", out _) ? (InputPresence<uint>)(value.GetProperty("snippet_pieces").GetUInt32()) : default, StageContext = value.TryGetProperty("stage_context", out _) ? (InputPresence<InputRecognitionStageContext>)(InputRecognitionStageContext.Read(value.GetProperty("stage_context"))) : default };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); if (EntityDefinition.IsPresent) { writer.WritePropertyName("entity_definition"); if (EntityDefinition.Value is null) writer.WriteNullValue(); else EntityDefinition.Value.Write(writer); } if (Instructions.IsPresent) { writer.WritePropertyName("instructions"); if (Instructions.Value is null) writer.WriteNullValue(); else Instructions.Value.Write(writer); } if (Kinds.IsPresent) { writer.WritePropertyName("kinds"); writer.WriteStartObject(); foreach (var entry in Kinds.Value) { WriteName(writer, entry.Key); if (entry.Value is null) writer.WriteNullValue(); else entry.Value.Write(writer); } writer.WriteEndObject(); } if (Mode.IsPresent) { writer.WritePropertyName("mode"); if (Mode.Value is null) writer.WriteNullValue(); else Mode.Value.Write(writer); } if (Relations.IsPresent) { writer.WritePropertyName("relations"); writer.WriteStartArray(); foreach (var item in Relations.Value) { if (item is null) writer.WriteNullValue(); else item.Write(writer); } writer.WriteEndArray(); } if (SnippetPieces.IsPresent) { writer.WritePropertyName("snippet_pieces"); writer.WriteNumberValue(SnippetPieces.Value); } if (StageContext.IsPresent) { writer.WritePropertyName("stage_context"); if (StageContext.Value is null) writer.WriteNullValue(); else StageContext.Value.Write(writer); } writer.WriteEndObject(); }
}

public sealed class InputRequestDefinitionAlternative7 : InputRequestDefinition {
public InputPresence<JsonElement> Batch { get; init; }
public InputPresence<InputAuthoredProfile> Profile { get; init; }
public required IReadOnlyDictionary<string, InputRequestDefinitionAlternative7QuestionsEntry> Questions { get; init; }
public InputPresence<InputAuthoredThreshold> Threshold { get; init; }
internal new static InputRequestDefinitionAlternative7 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Batch = value.TryGetProperty("batch", out _) ? (InputPresence<JsonElement>)(value.GetProperty("batch").Clone()) : default, Profile = value.TryGetProperty("profile", out _) ? (InputPresence<InputAuthoredProfile>)(InputAuthoredProfile.Read(value.GetProperty("profile"))) : default, Questions = new System.Collections.ObjectModel.ReadOnlyDictionary<string, InputRequestDefinitionAlternative7QuestionsEntry>(value.GetProperty("questions").EnumerateObject().ToDictionary(entry0 => entry0.Name, entry0 => InputRequestDefinitionAlternative7QuestionsEntry.Read(entry0.Value))), Threshold = value.TryGetProperty("threshold", out _) ? (InputPresence<InputAuthoredThreshold>)(InputAuthoredThreshold.Read(value.GetProperty("threshold"))) : default };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); if (Batch.IsPresent) { writer.WritePropertyName("batch"); Batch.Value.WriteTo(writer); } if (Profile.IsPresent) { writer.WritePropertyName("profile"); if (Profile.Value is null) writer.WriteNullValue(); else Profile.Value.Write(writer); } writer.WritePropertyName("questions"); writer.WriteStartObject(); foreach (var entry in Questions) { WriteName(writer, entry.Key); if (entry.Value is null) writer.WriteNullValue(); else entry.Value.Write(writer); } writer.WriteEndObject(); if (Threshold.IsPresent) { writer.WritePropertyName("threshold"); if (Threshold.Value is null) writer.WriteNullValue(); else Threshold.Value.Write(writer); } writer.WritePropertyName("version"); writer.WriteNumberValue(1); writer.WriteEndObject(); }
}

public abstract class InputRequestDefinitionAlternative7QuestionsEntry : InputDocument { private protected InputRequestDefinitionAlternative7QuestionsEntry() {} internal static InputRequestDefinitionAlternative7QuestionsEntry Read(JsonElement value) { if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("decide", out _)) return InputRequestDefinitionAlternative7QuestionsEntryAlternative0.Read(value); if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("choose", out _)) return InputRequestDefinitionAlternative7QuestionsEntryAlternative1.Read(value); if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("tag", out _)) return InputRequestDefinitionAlternative7QuestionsEntryAlternative2.Read(value); if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("score", out _)) return InputRequestDefinitionAlternative7QuestionsEntryAlternative3.Read(value); throw new JsonException("Authored data does not match the native Request schema."); } }

public sealed class InputRequestDefinitionAlternative7QuestionsEntryAlternative0 : InputRequestDefinitionAlternative7QuestionsEntry {
public InputPresence<InputAuthoredInputDeclaration> ContextSchema { get; init; }
public required InputAuthoredQuestionText Decide { get; init; }
public InputPresence<InputAuthoredCriterion> False { get; init; }
public InputPresence<InputAuthoredInputDeclaration> ItemSchema { get; init; }
public InputPresence<string> Name { get; init; }
public InputPresence<InputAuthoredPointers> On { get; init; }
public InputPresence<InputAuthoredThreshold> Threshold { get; init; }
public InputPresence<InputAuthoredCriterion> True { get; init; }
public InputPresence<long> WordingVersion { get; init; }
internal new static InputRequestDefinitionAlternative7QuestionsEntryAlternative0 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), ContextSchema = value.TryGetProperty("context_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("context_schema"))) : default, Decide = InputAuthoredQuestionText.Read(value.GetProperty("decide")), False = value.TryGetProperty("false", out _) ? (InputPresence<InputAuthoredCriterion>)(InputAuthoredCriterion.Read(value.GetProperty("false"))) : default, ItemSchema = value.TryGetProperty("item_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("item_schema"))) : default, Name = value.TryGetProperty("name", out _) ? (InputPresence<string>)(value.GetProperty("name").GetString()!) : default, On = value.TryGetProperty("on", out _) ? (InputPresence<InputAuthoredPointers>)(InputAuthoredPointers.Read(value.GetProperty("on"))) : default, Threshold = value.TryGetProperty("threshold", out _) ? (InputPresence<InputAuthoredThreshold>)(InputAuthoredThreshold.Read(value.GetProperty("threshold"))) : default, True = value.TryGetProperty("true", out _) ? (InputPresence<InputAuthoredCriterion>)(InputAuthoredCriterion.Read(value.GetProperty("true"))) : default, WordingVersion = value.TryGetProperty("wording_version", out _) ? (InputPresence<long>)(value.GetProperty("wording_version").GetInt64()) : default };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); if (ContextSchema.IsPresent) { writer.WritePropertyName("context_schema"); if (ContextSchema.Value is null) writer.WriteNullValue(); else ContextSchema.Value.Write(writer); } writer.WritePropertyName("decide"); if (Decide is null) writer.WriteNullValue(); else Decide.Write(writer); if (False.IsPresent) { writer.WritePropertyName("false"); if (False.Value is null) writer.WriteNullValue(); else False.Value.Write(writer); } if (ItemSchema.IsPresent) { writer.WritePropertyName("item_schema"); if (ItemSchema.Value is null) writer.WriteNullValue(); else ItemSchema.Value.Write(writer); } if (Name.IsPresent) { writer.WritePropertyName("name"); WriteText(writer, Name.Value); } if (On.IsPresent) { writer.WritePropertyName("on"); if (On.Value is null) writer.WriteNullValue(); else On.Value.Write(writer); } if (Threshold.IsPresent) { writer.WritePropertyName("threshold"); if (Threshold.Value is null) writer.WriteNullValue(); else Threshold.Value.Write(writer); } if (True.IsPresent) { writer.WritePropertyName("true"); if (True.Value is null) writer.WriteNullValue(); else True.Value.Write(writer); } if (WordingVersion.IsPresent) { writer.WritePropertyName("wording_version"); writer.WriteNumberValue(WordingVersion.Value); } writer.WriteEndObject(); }
}

public sealed class InputRequestDefinitionAlternative7QuestionsEntryAlternative1 : InputRequestDefinitionAlternative7QuestionsEntry {
public required InputAuthoredQuestionText Choose { get; init; }
public InputPresence<InputAuthoredInputDeclaration> ContextSchema { get; init; }
public InputPresence<InputAuthoredInputDeclaration> ItemSchema { get; init; }
public InputPresence<string> Name { get; init; }
public InputPresence<InputAuthoredPointers> On { get; init; }
public InputPresence<InputAuthoredOptions> Options { get; init; }
public InputPresence<InputAuthoredCut> Threshold { get; init; }
public InputPresence<long> WordingVersion { get; init; }
internal new static InputRequestDefinitionAlternative7QuestionsEntryAlternative1 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), Choose = InputAuthoredQuestionText.Read(value.GetProperty("choose")), ContextSchema = value.TryGetProperty("context_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("context_schema"))) : default, ItemSchema = value.TryGetProperty("item_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("item_schema"))) : default, Name = value.TryGetProperty("name", out _) ? (InputPresence<string>)(value.GetProperty("name").GetString()!) : default, On = value.TryGetProperty("on", out _) ? (InputPresence<InputAuthoredPointers>)(InputAuthoredPointers.Read(value.GetProperty("on"))) : default, Options = value.TryGetProperty("options", out _) ? (InputPresence<InputAuthoredOptions>)(InputAuthoredOptions.Read(value.GetProperty("options"))) : default, Threshold = value.TryGetProperty("threshold", out _) ? (InputPresence<InputAuthoredCut>)(InputAuthoredCut.Read(value.GetProperty("threshold"))) : default, WordingVersion = value.TryGetProperty("wording_version", out _) ? (InputPresence<long>)(value.GetProperty("wording_version").GetInt64()) : default };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("choose"); if (Choose is null) writer.WriteNullValue(); else Choose.Write(writer); if (ContextSchema.IsPresent) { writer.WritePropertyName("context_schema"); if (ContextSchema.Value is null) writer.WriteNullValue(); else ContextSchema.Value.Write(writer); } if (ItemSchema.IsPresent) { writer.WritePropertyName("item_schema"); if (ItemSchema.Value is null) writer.WriteNullValue(); else ItemSchema.Value.Write(writer); } if (Name.IsPresent) { writer.WritePropertyName("name"); WriteText(writer, Name.Value); } if (On.IsPresent) { writer.WritePropertyName("on"); if (On.Value is null) writer.WriteNullValue(); else On.Value.Write(writer); } if (Options.IsPresent) { writer.WritePropertyName("options"); if (Options.Value is null) writer.WriteNullValue(); else Options.Value.Write(writer); } if (Threshold.IsPresent) { writer.WritePropertyName("threshold"); if (Threshold.Value is null) writer.WriteNullValue(); else Threshold.Value.Write(writer); } if (WordingVersion.IsPresent) { writer.WritePropertyName("wording_version"); writer.WriteNumberValue(WordingVersion.Value); } writer.WriteEndObject(); }
}

public sealed class InputRequestDefinitionAlternative7QuestionsEntryAlternative2 : InputRequestDefinitionAlternative7QuestionsEntry {
public InputPresence<InputAuthoredInputDeclaration> ContextSchema { get; init; }
public InputPresence<InputAuthoredInputDeclaration> ItemSchema { get; init; }
public InputPresence<InputAuthoredLabels> Labels { get; init; }
public InputPresence<string> Name { get; init; }
public InputPresence<InputAuthoredPointers> On { get; init; }
public required InputAuthoredQuestionText Tag { get; init; }
public InputPresence<InputAuthoredCut> Threshold { get; init; }
public InputPresence<long> WordingVersion { get; init; }
internal new static InputRequestDefinitionAlternative7QuestionsEntryAlternative2 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), ContextSchema = value.TryGetProperty("context_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("context_schema"))) : default, ItemSchema = value.TryGetProperty("item_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("item_schema"))) : default, Labels = value.TryGetProperty("labels", out _) ? (InputPresence<InputAuthoredLabels>)(InputAuthoredLabels.Read(value.GetProperty("labels"))) : default, Name = value.TryGetProperty("name", out _) ? (InputPresence<string>)(value.GetProperty("name").GetString()!) : default, On = value.TryGetProperty("on", out _) ? (InputPresence<InputAuthoredPointers>)(InputAuthoredPointers.Read(value.GetProperty("on"))) : default, Tag = InputAuthoredQuestionText.Read(value.GetProperty("tag")), Threshold = value.TryGetProperty("threshold", out _) ? (InputPresence<InputAuthoredCut>)(InputAuthoredCut.Read(value.GetProperty("threshold"))) : default, WordingVersion = value.TryGetProperty("wording_version", out _) ? (InputPresence<long>)(value.GetProperty("wording_version").GetInt64()) : default };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); if (ContextSchema.IsPresent) { writer.WritePropertyName("context_schema"); if (ContextSchema.Value is null) writer.WriteNullValue(); else ContextSchema.Value.Write(writer); } if (ItemSchema.IsPresent) { writer.WritePropertyName("item_schema"); if (ItemSchema.Value is null) writer.WriteNullValue(); else ItemSchema.Value.Write(writer); } if (Labels.IsPresent) { writer.WritePropertyName("labels"); if (Labels.Value is null) writer.WriteNullValue(); else Labels.Value.Write(writer); } if (Name.IsPresent) { writer.WritePropertyName("name"); WriteText(writer, Name.Value); } if (On.IsPresent) { writer.WritePropertyName("on"); if (On.Value is null) writer.WriteNullValue(); else On.Value.Write(writer); } writer.WritePropertyName("tag"); if (Tag is null) writer.WriteNullValue(); else Tag.Write(writer); if (Threshold.IsPresent) { writer.WritePropertyName("threshold"); if (Threshold.Value is null) writer.WriteNullValue(); else Threshold.Value.Write(writer); } if (WordingVersion.IsPresent) { writer.WritePropertyName("wording_version"); writer.WriteNumberValue(WordingVersion.Value); } writer.WriteEndObject(); }
}

public sealed class InputRequestDefinitionAlternative7QuestionsEntryAlternative3 : InputRequestDefinitionAlternative7QuestionsEntry {
public InputPresence<InputAuthoredInputDeclaration> ContextSchema { get; init; }
public InputPresence<InputAuthoredInputDeclaration> ItemSchema { get; init; }
public InputPresence<InputAuthoredLevels> Levels { get; init; }
public InputPresence<string> Name { get; init; }
public InputPresence<InputAuthoredPointers> On { get; init; }
public required InputAuthoredQuestionText Score { get; init; }
public InputPresence<long> WordingVersion { get; init; }
internal new static InputRequestDefinitionAlternative7QuestionsEntryAlternative3 Read(JsonElement value) => new() { ParsedDocument = value.Clone(), ContextSchema = value.TryGetProperty("context_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("context_schema"))) : default, ItemSchema = value.TryGetProperty("item_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("item_schema"))) : default, Levels = value.TryGetProperty("levels", out _) ? (InputPresence<InputAuthoredLevels>)(InputAuthoredLevels.Read(value.GetProperty("levels"))) : default, Name = value.TryGetProperty("name", out _) ? (InputPresence<string>)(value.GetProperty("name").GetString()!) : default, On = value.TryGetProperty("on", out _) ? (InputPresence<InputAuthoredPointers>)(InputAuthoredPointers.Read(value.GetProperty("on"))) : default, Score = InputAuthoredQuestionText.Read(value.GetProperty("score")), WordingVersion = value.TryGetProperty("wording_version", out _) ? (InputPresence<long>)(value.GetProperty("wording_version").GetInt64()) : default };
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); if (ContextSchema.IsPresent) { writer.WritePropertyName("context_schema"); if (ContextSchema.Value is null) writer.WriteNullValue(); else ContextSchema.Value.Write(writer); } if (ItemSchema.IsPresent) { writer.WritePropertyName("item_schema"); if (ItemSchema.Value is null) writer.WriteNullValue(); else ItemSchema.Value.Write(writer); } if (Levels.IsPresent) { writer.WritePropertyName("levels"); if (Levels.Value is null) writer.WriteNullValue(); else Levels.Value.Write(writer); } if (Name.IsPresent) { writer.WritePropertyName("name"); WriteText(writer, Name.Value); } if (On.IsPresent) { writer.WritePropertyName("on"); if (On.Value is null) writer.WriteNullValue(); else On.Value.Write(writer); } writer.WritePropertyName("score"); if (Score is null) writer.WriteNullValue(); else Score.Write(writer); if (WordingVersion.IsPresent) { writer.WritePropertyName("wording_version"); writer.WriteNumberValue(WordingVersion.Value); } writer.WriteEndObject(); }
}

public abstract class InputRequestFraming : InputDocument { private protected InputRequestFraming() {} }

public sealed class InputRequestFramingAlternative0 : InputRequestFraming {
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } WriteText(writer, "document"); }
}

public sealed class InputRequestFramingAlternative1 : InputRequestFraming {
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } WriteText(writer, "lines"); }
}

public sealed class InputRequestFramingAlternative2 : InputRequestFraming {
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } WriteText(writer, "jsonl"); }
}

public sealed class InputRequestFramingAlternative3 : InputRequestFraming {
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } WriteText(writer, "csv"); }
}

public sealed class InputRequestFramingAlternative4 : InputRequestFraming {
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } WriteText(writer, "tsv"); }
}

public abstract class InputRequestImage : InputDocument { private protected InputRequestImage() {} }

public sealed class InputRequestImageBytes : InputRequestImage {
public required string Bytes { get; init; }
public required InputImageMedia Media { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("bytes"); WriteText(writer, Bytes); writer.WritePropertyName("kind"); WriteText(writer, "bytes"); writer.WritePropertyName("media"); if (Media is null) writer.WriteNullValue(); else Media.Write(writer); writer.WriteEndObject(); }
}

public sealed class InputRequestImageFile : InputRequestImage {
public InputPresence<InputImageMedia> Media { get; init; }
public required string Path { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("kind"); WriteText(writer, "file"); if (Media.IsPresent) { writer.WritePropertyName("media"); if (Media.Value is null) writer.WriteNullValue(); else Media.Value.Write(writer); } writer.WritePropertyName("path"); WriteText(writer, Path); writer.WriteEndObject(); }
}

public abstract class InputRequestInput : InputDocument { private protected InputRequestInput() {} }

public sealed class InputRequestInputEntities : InputRequestInput {
public required IReadOnlyList<InputRequestItem> Items { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("items"); writer.WriteStartArray(); foreach (var item in Items) { if (item is null) writer.WriteNullValue(); else item.Write(writer); } writer.WriteEndArray(); writer.WritePropertyName("kind"); WriteText(writer, "entities"); writer.WriteEndObject(); }
}

public sealed class InputRequestInputFeed : InputRequestInput {
public InputPresence<InputRequestFraming> Framing { get; init; }
public InputPresence<IReadOnlyList<InputRequestImage>> Images { get; init; }
public required string Name { get; init; }
public InputPresence<InputRequestReader> Reading { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); if (Framing.IsPresent) { writer.WritePropertyName("framing"); if (Framing.Value is null) writer.WriteNullValue(); else Framing.Value.Write(writer); } if (Images.IsPresent) { writer.WritePropertyName("images"); writer.WriteStartArray(); foreach (var item in Images.Value) { if (item is null) writer.WriteNullValue(); else item.Write(writer); } writer.WriteEndArray(); } writer.WritePropertyName("kind"); WriteText(writer, "feed"); writer.WritePropertyName("name"); WriteText(writer, Name); if (Reading.IsPresent) { writer.WritePropertyName("reading"); if (Reading.Value is null) writer.WriteNullValue(); else Reading.Value.Write(writer); } writer.WriteEndObject(); }
}

public sealed class InputRequestInputJson : InputRequestInput {
public InputPresence<IReadOnlyList<InputRequestImage>> Images { get; init; }
public required JsonElement Value { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); if (Images.IsPresent) { writer.WritePropertyName("images"); writer.WriteStartArray(); foreach (var item in Images.Value) { if (item is null) writer.WriteNullValue(); else item.Write(writer); } writer.WriteEndArray(); } writer.WritePropertyName("kind"); WriteText(writer, "json"); writer.WritePropertyName("value"); Value.WriteTo(writer); writer.WriteEndObject(); }
}

public sealed class InputRequestInputRecords : InputRequestInput {
public required IReadOnlyList<InputRequestItem> Items { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("items"); writer.WriteStartArray(); foreach (var item in Items) { if (item is null) writer.WriteNullValue(); else item.Write(writer); } writer.WriteEndArray(); writer.WritePropertyName("kind"); WriteText(writer, "records"); writer.WriteEndObject(); }
}

public sealed class InputRequestInputSource : InputRequestInput {
public required InputRequestSource Source { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("kind"); WriteText(writer, "source"); writer.WritePropertyName("source"); if (Source is null) writer.WriteNullValue(); else Source.Write(writer); writer.WriteEndObject(); }
}

public sealed class InputRequestInputText : InputRequestInput {
public InputPresence<IReadOnlyList<InputRequestImage>> Images { get; init; }
public required string Text { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); if (Images.IsPresent) { writer.WritePropertyName("images"); writer.WriteStartArray(); foreach (var item in Images.Value) { if (item is null) writer.WriteNullValue(); else item.Write(writer); } writer.WriteEndArray(); } writer.WritePropertyName("kind"); WriteText(writer, "text"); writer.WritePropertyName("text"); WriteText(writer, Text); writer.WriteEndObject(); }
}

public sealed class InputRequestInputUnits : InputRequestInput {
public required IReadOnlyList<InputRequestItem> Items { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("items"); writer.WriteStartArray(); foreach (var item in Items) { if (item is null) writer.WriteNullValue(); else item.Write(writer); } writer.WriteEndArray(); writer.WritePropertyName("kind"); WriteText(writer, "units"); writer.WriteEndObject(); }
}

public sealed class InputRequestItem : InputDocument {
public InputPresence<InputContextSchema> Context { get; init; }
public InputPresence<IReadOnlyList<InputRecognitionExample>> Examples { get; init; }
public InputPresence<IReadOnlyList<InputRequestImage>> Images { get; init; }
public InputPresence<IReadOnlyList<InputOptionSchema>> Options { get; init; }
public InputPresence<InputRequestOriginal> Original { get; init; }
public InputPresence<IReadOnlyList<InputRecognitionSeedSpan>> SeedSpans { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); if (Context.IsPresent) { writer.WritePropertyName("context"); if (Context.Value is null) writer.WriteNullValue(); else Context.Value.Write(writer); } if (Examples.IsPresent) { writer.WritePropertyName("examples"); writer.WriteStartArray(); foreach (var item in Examples.Value) { if (item is null) writer.WriteNullValue(); else item.Write(writer); } writer.WriteEndArray(); } if (Images.IsPresent) { writer.WritePropertyName("images"); writer.WriteStartArray(); foreach (var item in Images.Value) { if (item is null) writer.WriteNullValue(); else item.Write(writer); } writer.WriteEndArray(); } if (Options.IsPresent) { writer.WritePropertyName("options"); writer.WriteStartArray(); foreach (var item in Options.Value) { if (item is null) writer.WriteNullValue(); else item.Write(writer); } writer.WriteEndArray(); } if (Original.IsPresent) { writer.WritePropertyName("original"); if (Original.Value is null) writer.WriteNullValue(); else Original.Value.Write(writer); } if (SeedSpans.IsPresent) { writer.WritePropertyName("seed_spans"); writer.WriteStartArray(); foreach (var item in SeedSpans.Value) { if (item is null) writer.WriteNullValue(); else item.Write(writer); } writer.WriteEndArray(); } writer.WriteEndObject(); }
}

public sealed class InputRequestOptions : InputDocument {
public InputPresence<bool> Attempts { get; init; }
public InputPresence<InputRequestBatch> Batch { get; init; }
public InputPresence<string> Context { get; init; }
public InputPresence<string> ContextField { get; init; }
public InputPresence<long> DeadlineMs { get; init; }
public InputPresence<bool> Details { get; init; }
public InputPresence<IReadOnlyList<InputRecognitionExample>> Examples { get; init; }
public InputPresence<string> ExamplesField { get; init; }
public InputPresence<IReadOnlyList<string>> Field { get; init; }
public InputPresence<bool> FilesOnly { get; init; }
public InputPresence<ulong> MaxRequestsTotal { get; init; }
public InputPresence<InputRecognitionMode> Mode { get; init; }
public InputPresence<string> Model { get; init; }
public InputPresence<bool> None { get; init; }
public InputPresence<string> OptionsField { get; init; }
public InputPresence<InputRequestThreshold> RelationThreshold { get; init; }
public InputPresence<IReadOnlyList<InputRecognitionSeedSpan>> SeedSpans { get; init; }
public InputPresence<string> SeedSpansField { get; init; }
public InputPresence<uint> SnippetPieces { get; init; }
public InputPresence<InputRecognitionStageContext> StageContext { get; init; }
public InputPresence<InputRequestThreshold> Threshold { get; init; }
public InputPresence<ulong> Top { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); if (Attempts.IsPresent) { writer.WritePropertyName("attempts"); writer.WriteBooleanValue(Attempts.Value); } if (Batch.IsPresent) { writer.WritePropertyName("batch"); if (Batch.Value is null) writer.WriteNullValue(); else Batch.Value.Write(writer); } if (Context.IsPresent) { writer.WritePropertyName("context"); WriteText(writer, Context.Value); } if (ContextField.IsPresent) { writer.WritePropertyName("context_field"); WriteText(writer, ContextField.Value); } if (DeadlineMs.IsPresent) { writer.WritePropertyName("deadline_ms"); writer.WriteNumberValue(DeadlineMs.Value); } if (Details.IsPresent) { writer.WritePropertyName("details"); writer.WriteBooleanValue(Details.Value); } if (Examples.IsPresent) { writer.WritePropertyName("examples"); writer.WriteStartArray(); foreach (var item in Examples.Value) { if (item is null) writer.WriteNullValue(); else item.Write(writer); } writer.WriteEndArray(); } if (ExamplesField.IsPresent) { writer.WritePropertyName("examples_field"); WriteText(writer, ExamplesField.Value); } if (Field.IsPresent) { writer.WritePropertyName("field"); writer.WriteStartArray(); foreach (var item in Field.Value) { WriteText(writer, item); } writer.WriteEndArray(); } if (FilesOnly.IsPresent) { writer.WritePropertyName("files_only"); writer.WriteBooleanValue(FilesOnly.Value); } if (MaxRequestsTotal.IsPresent) { writer.WritePropertyName("max_requests_total"); writer.WriteNumberValue(MaxRequestsTotal.Value); } if (Mode.IsPresent) { writer.WritePropertyName("mode"); if (Mode.Value is null) writer.WriteNullValue(); else Mode.Value.Write(writer); } if (Model.IsPresent) { writer.WritePropertyName("model"); WriteText(writer, Model.Value); } if (None.IsPresent) { writer.WritePropertyName("none"); writer.WriteBooleanValue(None.Value); } if (OptionsField.IsPresent) { writer.WritePropertyName("options_field"); WriteText(writer, OptionsField.Value); } if (RelationThreshold.IsPresent) { writer.WritePropertyName("relation_threshold"); if (RelationThreshold.Value is null) writer.WriteNullValue(); else RelationThreshold.Value.Write(writer); } if (SeedSpans.IsPresent) { writer.WritePropertyName("seed_spans"); writer.WriteStartArray(); foreach (var item in SeedSpans.Value) { if (item is null) writer.WriteNullValue(); else item.Write(writer); } writer.WriteEndArray(); } if (SeedSpansField.IsPresent) { writer.WritePropertyName("seed_spans_field"); WriteText(writer, SeedSpansField.Value); } if (SnippetPieces.IsPresent) { writer.WritePropertyName("snippet_pieces"); writer.WriteNumberValue(SnippetPieces.Value); } if (StageContext.IsPresent) { writer.WritePropertyName("stage_context"); if (StageContext.Value is null) writer.WriteNullValue(); else StageContext.Value.Write(writer); } if (Threshold.IsPresent) { writer.WritePropertyName("threshold"); if (Threshold.Value is null) writer.WriteNullValue(); else Threshold.Value.Write(writer); } if (Top.IsPresent) { writer.WritePropertyName("top"); writer.WriteNumberValue(Top.Value); } writer.WriteEndObject(); }
}

public abstract class InputRequestOriginal : InputDocument { private protected InputRequestOriginal() {} }

public sealed class InputRequestOriginalJson : InputRequestOriginal {
public required JsonElement Value { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("kind"); WriteText(writer, "json"); writer.WritePropertyName("value"); Value.WriteTo(writer); writer.WriteEndObject(); }
}

public sealed class InputRequestOriginalText : InputRequestOriginal {
public required string Text { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("kind"); WriteText(writer, "text"); writer.WritePropertyName("text"); WriteText(writer, Text); writer.WriteEndObject(); }
}

public abstract class InputRequestQuestion : InputDocument { private protected InputRequestQuestion() {} }

public sealed class InputRequestQuestionDefinition : InputRequestQuestion {
public required InputRequestDefinition Value { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("kind"); WriteText(writer, "definition"); writer.WritePropertyName("value"); if (Value is null) writer.WriteNullValue(); else Value.Write(writer); writer.WriteEndObject(); }
}

public sealed class InputRequestQuestionFile : InputRequestQuestion {
public required string Path { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("kind"); WriteText(writer, "file"); writer.WritePropertyName("path"); WriteText(writer, Path); writer.WriteEndObject(); }
}

public sealed class InputRequestQuestionName : InputRequestQuestion {
public required string Name { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("kind"); WriteText(writer, "name"); writer.WritePropertyName("name"); WriteText(writer, Name); writer.WriteEndObject(); }
}

public sealed class InputRequestQuestionReference : InputRequestQuestion {
public required string Reference { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("kind"); WriteText(writer, "reference"); writer.WritePropertyName("reference"); WriteText(writer, Reference); writer.WriteEndObject(); }
}

public sealed class InputRequestQuestionText : InputRequestQuestion {
public required string Text { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("kind"); WriteText(writer, "text"); writer.WritePropertyName("text"); WriteText(writer, Text); writer.WriteEndObject(); }
}

public sealed class InputRequestReader : InputDocument {
public InputPresence<InputSourceUnit> Unit { get; init; }
public InputPresence<ulong> Window { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); if (Unit.IsPresent) { writer.WritePropertyName("unit"); if (Unit.Value is null) writer.WriteNullValue(); else Unit.Value.Write(writer); } if (Window.IsPresent) { writer.WritePropertyName("window"); writer.WriteNumberValue(Window.Value); } writer.WriteEndObject(); }
}

public abstract class InputRequestReaderFailure : InputDocument { private protected InputRequestReaderFailure() {} }

public sealed class InputRequestReaderFailureInvalidInput : InputRequestReaderFailure {
public InputPresence<InputSessionSourceLocation> Location { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("kind"); WriteText(writer, "invalid_input"); if (Location.IsPresent) { writer.WritePropertyName("location"); if (Location.Value is null) writer.WriteNullValue(); else Location.Value.Write(writer); } writer.WriteEndObject(); }
}

public sealed class InputRequestReaderFailureIo : InputRequestReaderFailure {
public InputPresence<InputSessionSourceLocation> Location { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("kind"); WriteText(writer, "io"); if (Location.IsPresent) { writer.WritePropertyName("location"); if (Location.Value is null) writer.WriteNullValue(); else Location.Value.Write(writer); } writer.WriteEndObject(); }
}

public sealed class InputRequestReaderFailureUtf8 : InputRequestReaderFailure {
public InputPresence<InputSessionSourceLocation> Location { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("kind"); WriteText(writer, "utf8"); if (Location.IsPresent) { writer.WritePropertyName("location"); if (Location.Value is null) writer.WriteNullValue(); else Location.Value.Write(writer); } writer.WriteEndObject(); }
}

public sealed class InputRequestSessionDescriptor : InputDocument {
public required InputRequestItem Item { get; init; }
public InputPresence<InputSessionSourceLocation> Location { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("item"); if (Item is null) writer.WriteNullValue(); else Item.Write(writer); if (Location.IsPresent) { writer.WritePropertyName("location"); if (Location.Value is null) writer.WriteNullValue(); else Location.Value.Write(writer); } writer.WriteEndObject(); }
}

public sealed class InputRequestSource : InputDocument {
public InputPresence<InputReaderMedia> Media { get; init; }
public required IReadOnlyList<string> Paths { get; init; }
public InputPresence<InputRequestReader> Reading { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); if (Media.IsPresent) { writer.WritePropertyName("media"); if (Media.Value is null) writer.WriteNullValue(); else Media.Value.Write(writer); } writer.WritePropertyName("paths"); writer.WriteStartArray(); foreach (var item in Paths) { WriteText(writer, item); } writer.WriteEndArray(); if (Reading.IsPresent) { writer.WritePropertyName("reading"); if (Reading.Value is null) writer.WriteNullValue(); else Reading.Value.Write(writer); } writer.WriteEndObject(); }
}

public abstract class InputRequestThreshold : InputDocument { private protected InputRequestThreshold() {} }

public sealed class InputRequestThresholdAlternative0 : InputRequestThreshold {
public required double Value { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteNumberValue(Value); }
}

public sealed class InputRequestThresholdAlternative1 : InputRequestThreshold {
public required string Value { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } WriteText(writer, Value); }
}

public abstract class InputRequestVersion : InputDocument { private protected InputRequestVersion() {} }

public sealed class InputRequestVersionAlternative0 : InputRequestVersion {
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } WriteText(writer, "thinkthen.request/1"); }
}

public sealed class InputSessionSourceLocation : InputDocument {
public required string File { get; init; }
public InputPresence<ulong> FirstLine { get; init; }
public InputPresence<ulong> LastLine { get; init; }
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } writer.WriteStartObject(); writer.WritePropertyName("file"); WriteText(writer, File); if (FirstLine.IsPresent) { writer.WritePropertyName("first_line"); writer.WriteNumberValue(FirstLine.Value); } if (LastLine.IsPresent) { writer.WritePropertyName("last_line"); writer.WriteNumberValue(LastLine.Value); } writer.WriteEndObject(); }
}

public abstract class InputSourceUnit : InputDocument { private protected InputSourceUnit() {} }

public sealed class InputSourceUnitAlternative0 : InputSourceUnit {
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } WriteText(writer, "line"); }
}

public sealed class InputSourceUnitAlternative1 : InputSourceUnit {
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } WriteText(writer, "window"); }
}

public sealed class InputSourceUnitAlternative2 : InputSourceUnit {
public override void Write(Utf8JsonWriter writer) { if (ParsedDocument is {} parsed) { writer.WriteRawValue(parsed.GetRawText()); return; } WriteText(writer, "file"); }
}
