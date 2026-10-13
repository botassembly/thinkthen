// Generated from the Rust-derived Request schema; do not edit.
#nullable enable
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.Json;
namespace ThinkThen.Inputs;
public readonly struct InputPresence<T> {
    public bool IsPresent {
        get;
    }
    public T Value {
        get;
    }
    private InputPresence(T value) {
        Value = value;
        IsPresent = true;
    }
    public static implicit operator InputPresence<T>(T value) => new(value);
}
public abstract class InputDocument {
    private static readonly System.Text.Encoding StrictUtf8 = new System.Text.UTF8Encoding(false,
    true);
    protected static void WriteText(Utf8JsonWriter writer, string? value) {
        if (value is not null) _ = StrictUtf8.GetByteCount(value);
        writer.WriteStringValue(value);
    }
    protected static void WriteName(Utf8JsonWriter writer, string value) {
        _ = StrictUtf8.GetByteCount(value);
        writer.WritePropertyName(value);
    }
    // Only internal readers retain native-admitted JSON; raw writes bypass host depth validation.
    private protected JsonElement? ParsedDocument {
        get;
        init;
    }
    public abstract void Write(Utf8JsonWriter writer);
    internal byte[] ToBytes() {
        using var stream = new MemoryStream();
        using (var writer = new Utf8JsonWriter(stream, new JsonWriterOptions {
            MaxDepth = int.MaxValue
        })) Write(writer);
        return stream.ToArray();
    }
}
public sealed class InputAnnotateDefinition : InputDocument {
    public InputPresence<JsonElement> Batch {
        get;
        init;
    }
    public InputPresence<InputAuthoredProfile> Profile {
        get;
        init;
    }
    public required IReadOnlyDictionary<string, InputAnnotateQuestion> Questions {
        get;
        init;
    }
    public InputPresence<InputAuthoredThreshold> Threshold {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        if (Batch.IsPresent) {
            writer.WritePropertyName("batch");
            Batch.Value.WriteTo(writer);
        } if (Profile.IsPresent) {
            writer.WritePropertyName("profile");
            if (Profile.Value is null) writer.WriteNullValue();
            else Profile.Value.Write(writer);
        } writer.WritePropertyName("questions");
        writer.WriteStartObject();
        foreach (var entry in Questions) {
            WriteName(writer, entry.Key);
            if (entry.Value is null) writer.WriteNullValue();
            else entry.Value.Write(writer);
        } writer.WriteEndObject();
        if (Threshold.IsPresent) {
            writer.WritePropertyName("threshold");
            if (Threshold.Value is null) writer.WriteNullValue();
            else Threshold.Value.Write(writer);
        } writer.WritePropertyName("version");
        writer.WriteNumberValue(1);
        writer.WriteEndObject();
    }
}
public abstract class InputAnnotateQuestion : InputDocument {
    private protected InputAnnotateQuestion() {
    } internal static InputAnnotateQuestion Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("decide",
        out _)) return InputAnnotateQuestionDecide.Read(value);
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("choose",
        out _)) return InputAnnotateQuestionChoose.Read(value);
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("tag",
        out _)) return InputAnnotateQuestionTag.Read(value);
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("score",
        out _)) return InputAnnotateQuestionScore.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class InputAnnotateQuestionChoose : InputAnnotateQuestion {
    public required InputAuthoredQuestionText Choose {
        get;
        init;
    }
    public InputPresence<InputAuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public InputPresence<InputAuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public InputPresence<string> Name {
        get;
        init;
    }
    public InputPresence<InputAuthoredPointers> On {
        get;
        init;
    }
    public InputPresence<InputAuthoredOptions> Options {
        get;
        init;
    }
    public InputPresence<InputAuthoredCut> Threshold {
        get;
        init;
    }
    public InputPresence<long> WordingVersion {
        get;
        init;
    }
    internal new static InputAnnotateQuestionChoose Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Choose = InputAuthoredQuestionText.Read(value.GetProperty("choose")),
        ContextSchema = value.TryGetProperty("context_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("context_schema"))) : default,
        ItemSchema = value.TryGetProperty("item_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("item_schema"))) : default,
        Name = value.TryGetProperty("name", out _) ? (InputPresence<string>)(value.GetProperty("name").GetString()!) : default,
        On = value.TryGetProperty("on", out _) ? (InputPresence<InputAuthoredPointers>)(InputAuthoredPointers.Read(value.GetProperty("on"))) : default,
        Options = value.TryGetProperty("options", out _) ? (InputPresence<InputAuthoredOptions>)(InputAuthoredOptions.Read(value.GetProperty("options"))) : default,
        Threshold = value.TryGetProperty("threshold", out _) ? (InputPresence<InputAuthoredCut>)(InputAuthoredCut.Read(value.GetProperty("threshold"))) : default,
        WordingVersion = value.TryGetProperty("wording_version", out _) ? (InputPresence<long>)(value.GetProperty("wording_version").GetInt64()) : default
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        writer.WritePropertyName("choose");
        if (Choose is null) writer.WriteNullValue();
        else Choose.Write(writer);
        if (ContextSchema.IsPresent) {
            writer.WritePropertyName("context_schema");
            if (ContextSchema.Value is null) writer.WriteNullValue();
            else ContextSchema.Value.Write(writer);
        } if (ItemSchema.IsPresent) {
            writer.WritePropertyName("item_schema");
            if (ItemSchema.Value is null) writer.WriteNullValue();
            else ItemSchema.Value.Write(writer);
        } if (Name.IsPresent) {
            writer.WritePropertyName("name");
            WriteText(writer, Name.Value);
        } if (On.IsPresent) {
            writer.WritePropertyName("on");
            if (On.Value is null) writer.WriteNullValue();
            else On.Value.Write(writer);
        } if (Options.IsPresent) {
            writer.WritePropertyName("options");
            if (Options.Value is null) writer.WriteNullValue();
            else Options.Value.Write(writer);
        } if (Threshold.IsPresent) {
            writer.WritePropertyName("threshold");
            if (Threshold.Value is null) writer.WriteNullValue();
            else Threshold.Value.Write(writer);
        } if (WordingVersion.IsPresent) {
            writer.WritePropertyName("wording_version");
            writer.WriteNumberValue(WordingVersion.Value);
        } writer.WriteEndObject();
    }
}
public sealed class InputAnnotateQuestionDecide : InputAnnotateQuestion {
    public InputPresence<InputAuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public required InputAuthoredQuestionText Decide {
        get;
        init;
    }
    public InputPresence<InputAuthoredCriterion> False {
        get;
        init;
    }
    public InputPresence<InputAuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public InputPresence<string> Name {
        get;
        init;
    }
    public InputPresence<InputAuthoredPointers> On {
        get;
        init;
    }
    public InputPresence<InputAuthoredThreshold> Threshold {
        get;
        init;
    }
    public InputPresence<InputAuthoredCriterion> True {
        get;
        init;
    }
    public InputPresence<long> WordingVersion {
        get;
        init;
    }
    internal new static InputAnnotateQuestionDecide Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), ContextSchema = value.TryGetProperty("context_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("context_schema"))) : default,
        Decide = InputAuthoredQuestionText.Read(value.GetProperty("decide")), False = value.TryGetProperty("false",
        out _) ? (InputPresence<InputAuthoredCriterion>)(InputAuthoredCriterion.Read(value.GetProperty("false"))) : default,
        ItemSchema = value.TryGetProperty("item_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("item_schema"))) : default,
        Name = value.TryGetProperty("name", out _) ? (InputPresence<string>)(value.GetProperty("name").GetString()!) : default,
        On = value.TryGetProperty("on", out _) ? (InputPresence<InputAuthoredPointers>)(InputAuthoredPointers.Read(value.GetProperty("on"))) : default,
        Threshold = value.TryGetProperty("threshold", out _) ? (InputPresence<InputAuthoredThreshold>)(InputAuthoredThreshold.Read(value.GetProperty("threshold"))) : default,
        True = value.TryGetProperty("true", out _) ? (InputPresence<InputAuthoredCriterion>)(InputAuthoredCriterion.Read(value.GetProperty("true"))) : default,
        WordingVersion = value.TryGetProperty("wording_version", out _) ? (InputPresence<long>)(value.GetProperty("wording_version").GetInt64()) : default
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        if (ContextSchema.IsPresent) {
            writer.WritePropertyName("context_schema");
            if (ContextSchema.Value is null) writer.WriteNullValue();
            else ContextSchema.Value.Write(writer);
        } writer.WritePropertyName("decide");
        if (Decide is null) writer.WriteNullValue();
        else Decide.Write(writer);
        if (False.IsPresent) {
            writer.WritePropertyName("false");
            if (False.Value is null) writer.WriteNullValue();
            else False.Value.Write(writer);
        } if (ItemSchema.IsPresent) {
            writer.WritePropertyName("item_schema");
            if (ItemSchema.Value is null) writer.WriteNullValue();
            else ItemSchema.Value.Write(writer);
        } if (Name.IsPresent) {
            writer.WritePropertyName("name");
            WriteText(writer, Name.Value);
        } if (On.IsPresent) {
            writer.WritePropertyName("on");
            if (On.Value is null) writer.WriteNullValue();
            else On.Value.Write(writer);
        } if (Threshold.IsPresent) {
            writer.WritePropertyName("threshold");
            if (Threshold.Value is null) writer.WriteNullValue();
            else Threshold.Value.Write(writer);
        } if (True.IsPresent) {
            writer.WritePropertyName("true");
            if (True.Value is null) writer.WriteNullValue();
            else True.Value.Write(writer);
        } if (WordingVersion.IsPresent) {
            writer.WritePropertyName("wording_version");
            writer.WriteNumberValue(WordingVersion.Value);
        } writer.WriteEndObject();
    }
}
public sealed class InputAnnotateQuestionScore : InputAnnotateQuestion {
    public InputPresence<InputAuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public InputPresence<InputAuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public InputPresence<InputAuthoredLevels> Levels {
        get;
        init;
    }
    public InputPresence<string> Name {
        get;
        init;
    }
    public InputPresence<InputAuthoredPointers> On {
        get;
        init;
    }
    public required InputAuthoredQuestionText Score {
        get;
        init;
    }
    public InputPresence<long> WordingVersion {
        get;
        init;
    }
    internal new static InputAnnotateQuestionScore Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), ContextSchema = value.TryGetProperty("context_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("context_schema"))) : default,
        ItemSchema = value.TryGetProperty("item_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("item_schema"))) : default,
        Levels = value.TryGetProperty("levels", out _) ? (InputPresence<InputAuthoredLevels>)(InputAuthoredLevels.Read(value.GetProperty("levels"))) : default,
        Name = value.TryGetProperty("name", out _) ? (InputPresence<string>)(value.GetProperty("name").GetString()!) : default,
        On = value.TryGetProperty("on", out _) ? (InputPresence<InputAuthoredPointers>)(InputAuthoredPointers.Read(value.GetProperty("on"))) : default,
        Score = InputAuthoredQuestionText.Read(value.GetProperty("score")), WordingVersion = value.TryGetProperty("wording_version",
        out _) ? (InputPresence<long>)(value.GetProperty("wording_version").GetInt64()) : default
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        if (ContextSchema.IsPresent) {
            writer.WritePropertyName("context_schema");
            if (ContextSchema.Value is null) writer.WriteNullValue();
            else ContextSchema.Value.Write(writer);
        } if (ItemSchema.IsPresent) {
            writer.WritePropertyName("item_schema");
            if (ItemSchema.Value is null) writer.WriteNullValue();
            else ItemSchema.Value.Write(writer);
        } if (Levels.IsPresent) {
            writer.WritePropertyName("levels");
            if (Levels.Value is null) writer.WriteNullValue();
            else Levels.Value.Write(writer);
        } if (Name.IsPresent) {
            writer.WritePropertyName("name");
            WriteText(writer, Name.Value);
        } if (On.IsPresent) {
            writer.WritePropertyName("on");
            if (On.Value is null) writer.WriteNullValue();
            else On.Value.Write(writer);
        } writer.WritePropertyName("score");
        if (Score is null) writer.WriteNullValue();
        else Score.Write(writer);
        if (WordingVersion.IsPresent) {
            writer.WritePropertyName("wording_version");
            writer.WriteNumberValue(WordingVersion.Value);
        } writer.WriteEndObject();
    }
}
public sealed class InputAnnotateQuestionTag : InputAnnotateQuestion {
    public InputPresence<InputAuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public InputPresence<InputAuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public InputPresence<InputAuthoredLabels> Labels {
        get;
        init;
    }
    public InputPresence<string> Name {
        get;
        init;
    }
    public InputPresence<InputAuthoredPointers> On {
        get;
        init;
    }
    public required InputAuthoredQuestionText Tag {
        get;
        init;
    }
    public InputPresence<InputAuthoredCut> Threshold {
        get;
        init;
    }
    public InputPresence<long> WordingVersion {
        get;
        init;
    }
    internal new static InputAnnotateQuestionTag Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), ContextSchema = value.TryGetProperty("context_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("context_schema"))) : default,
        ItemSchema = value.TryGetProperty("item_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("item_schema"))) : default,
        Labels = value.TryGetProperty("labels", out _) ? (InputPresence<InputAuthoredLabels>)(InputAuthoredLabels.Read(value.GetProperty("labels"))) : default,
        Name = value.TryGetProperty("name", out _) ? (InputPresence<string>)(value.GetProperty("name").GetString()!) : default,
        On = value.TryGetProperty("on", out _) ? (InputPresence<InputAuthoredPointers>)(InputAuthoredPointers.Read(value.GetProperty("on"))) : default,
        Tag = InputAuthoredQuestionText.Read(value.GetProperty("tag")), Threshold = value.TryGetProperty("threshold",
        out _) ? (InputPresence<InputAuthoredCut>)(InputAuthoredCut.Read(value.GetProperty("threshold"))) : default,
        WordingVersion = value.TryGetProperty("wording_version", out _) ? (InputPresence<long>)(value.GetProperty("wording_version").GetInt64()) : default
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        if (ContextSchema.IsPresent) {
            writer.WritePropertyName("context_schema");
            if (ContextSchema.Value is null) writer.WriteNullValue();
            else ContextSchema.Value.Write(writer);
        } if (ItemSchema.IsPresent) {
            writer.WritePropertyName("item_schema");
            if (ItemSchema.Value is null) writer.WriteNullValue();
            else ItemSchema.Value.Write(writer);
        } if (Labels.IsPresent) {
            writer.WritePropertyName("labels");
            if (Labels.Value is null) writer.WriteNullValue();
            else Labels.Value.Write(writer);
        } if (Name.IsPresent) {
            writer.WritePropertyName("name");
            WriteText(writer, Name.Value);
        } if (On.IsPresent) {
            writer.WritePropertyName("on");
            if (On.Value is null) writer.WriteNullValue();
            else On.Value.Write(writer);
        } writer.WritePropertyName("tag");
        if (Tag is null) writer.WriteNullValue();
        else Tag.Write(writer);
        if (Threshold.IsPresent) {
            writer.WritePropertyName("threshold");
            if (Threshold.Value is null) writer.WriteNullValue();
            else Threshold.Value.Write(writer);
        } if (WordingVersion.IsPresent) {
            writer.WritePropertyName("wording_version");
            writer.WriteNumberValue(WordingVersion.Value);
        } writer.WriteEndObject();
    }
}
public sealed class InputAuthoredChoose : InputDocument {
    public InputPresence<InputAuthoredChooseBatch> Batch {
        get;
        init;
    }
    public required InputAuthoredQuestionText Choose {
        get;
        init;
    }
    public InputPresence<InputAuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public InputPresence<InputAuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public InputPresence<InputAuthoredName> Model {
        get;
        init;
    }
    public InputPresence<string> Name {
        get;
        init;
    }
    public InputPresence<InputAuthoredPointers> On {
        get;
        init;
    }
    public InputPresence<InputAuthoredOptions> Options {
        get;
        init;
    }
    public InputPresence<InputAuthoredProfile> Profile {
        get;
        init;
    }
    public InputPresence<InputAuthoredCut> Threshold {
        get;
        init;
    }
    public InputPresence<long> WordingVersion {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        if (Batch.IsPresent) {
            writer.WritePropertyName("batch");
            if (Batch.Value is null) writer.WriteNullValue();
            else Batch.Value.Write(writer);
        } writer.WritePropertyName("choose");
        if (Choose is null) writer.WriteNullValue();
        else Choose.Write(writer);
        if (ContextSchema.IsPresent) {
            writer.WritePropertyName("context_schema");
            if (ContextSchema.Value is null) writer.WriteNullValue();
            else ContextSchema.Value.Write(writer);
        } if (ItemSchema.IsPresent) {
            writer.WritePropertyName("item_schema");
            if (ItemSchema.Value is null) writer.WriteNullValue();
            else ItemSchema.Value.Write(writer);
        } if (Model.IsPresent) {
            writer.WritePropertyName("model");
            if (Model.Value is null) writer.WriteNullValue();
            else Model.Value.Write(writer);
        } if (Name.IsPresent) {
            writer.WritePropertyName("name");
            WriteText(writer, Name.Value);
        } if (On.IsPresent) {
            writer.WritePropertyName("on");
            if (On.Value is null) writer.WriteNullValue();
            else On.Value.Write(writer);
        } if (Options.IsPresent) {
            writer.WritePropertyName("options");
            if (Options.Value is null) writer.WriteNullValue();
            else Options.Value.Write(writer);
        } if (Profile.IsPresent) {
            writer.WritePropertyName("profile");
            if (Profile.Value is null) writer.WriteNullValue();
            else Profile.Value.Write(writer);
        } if (Threshold.IsPresent) {
            writer.WritePropertyName("threshold");
            if (Threshold.Value is null) writer.WriteNullValue();
            else Threshold.Value.Write(writer);
        } if (WordingVersion.IsPresent) {
            writer.WritePropertyName("wording_version");
            writer.WriteNumberValue(WordingVersion.Value);
        } writer.WriteEndObject();
    }
}
public abstract class InputAuthoredChooseBatch : InputDocument {
    private protected InputAuthoredChooseBatch() {
    }
}
public sealed class InputAuthoredChooseBatchInteger : InputAuthoredChooseBatch {
    public required long Value {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNumberValue(Value);
    }
}
public sealed class InputAuthoredChooseBatchMax : InputAuthoredChooseBatch {
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "max");
    }
}
public abstract class InputAuthoredCriterion : InputDocument {
    private protected InputAuthoredCriterion() {
    } internal static InputAuthoredCriterion Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.String)) return InputAuthoredCriterionString.Read(value);
        if ((value.ValueKind == JsonValueKind.Object)) return InputAuthoredCriterionObject.Read(value);
        if ((value.ValueKind == JsonValueKind.Array)) return InputAuthoredCriterionList.Read(value);
        if ((value.ValueKind == JsonValueKind.Null)) return InputAuthoredCriterionNull.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class InputAuthoredCriterionList : InputAuthoredCriterion {
    public required IReadOnlyList<JsonElement> Value {
        get;
        init;
    }
    internal new static InputAuthoredCriterionList Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = Array.AsReadOnly(value.EnumerateArray().Select(item0 => item0.Clone()).ToArray())
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartArray();
        foreach (var item in Value) {
            item.WriteTo(writer);
        } writer.WriteEndArray();
    }
}
public sealed class InputAuthoredCriterionNull : InputAuthoredCriterion {
    internal new static InputAuthoredCriterionNull Read(JsonElement value) => new() {
        ParsedDocument = value.Clone()
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNullValue();
    }
}
public sealed class InputAuthoredCriterionObject : InputAuthoredCriterion {
    public required JsonElement Value {
        get;
        init;
    }
    internal new static InputAuthoredCriterionObject Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.Clone()
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } Value.WriteTo(writer);
    }
}
public sealed class InputAuthoredCriterionString : InputAuthoredCriterion {
    public required string Value {
        get;
        init;
    }
    internal new static InputAuthoredCriterionString Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.GetString()!
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, Value);
    }
}
public abstract class InputAuthoredCut : InputDocument {
    private protected InputAuthoredCut() {
    } internal static InputAuthoredCut Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.Number)) return InputAuthoredCutNumber.Read(value);
        if ((value.ValueKind == JsonValueKind.String)) return InputAuthoredCutString.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class InputAuthoredCutNumber : InputAuthoredCut {
    public required double Value {
        get;
        init;
    }
    internal new static InputAuthoredCutNumber Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.GetDouble()
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNumberValue(Value);
    }
}
public sealed class InputAuthoredCutString : InputAuthoredCut {
    public required string Value {
        get;
        init;
    }
    internal new static InputAuthoredCutString Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.GetString()!
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, Value);
    }
}
public sealed class InputAuthoredDecide : InputDocument {
    public InputPresence<InputAuthoredDecideBatch> Batch {
        get;
        init;
    }
    public InputPresence<InputAuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public required InputAuthoredQuestionText Decide {
        get;
        init;
    }
    public InputPresence<InputAuthoredCriterion> False {
        get;
        init;
    }
    public InputPresence<InputAuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public InputPresence<InputAuthoredName> Model {
        get;
        init;
    }
    public InputPresence<string> Name {
        get;
        init;
    }
    public InputPresence<InputAuthoredPointers> On {
        get;
        init;
    }
    public InputPresence<InputAuthoredProfile> Profile {
        get;
        init;
    }
    public InputPresence<InputAuthoredThreshold> Threshold {
        get;
        init;
    }
    public InputPresence<InputAuthoredCriterion> True {
        get;
        init;
    }
    public InputPresence<long> WordingVersion {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        if (Batch.IsPresent) {
            writer.WritePropertyName("batch");
            if (Batch.Value is null) writer.WriteNullValue();
            else Batch.Value.Write(writer);
        } if (ContextSchema.IsPresent) {
            writer.WritePropertyName("context_schema");
            if (ContextSchema.Value is null) writer.WriteNullValue();
            else ContextSchema.Value.Write(writer);
        } writer.WritePropertyName("decide");
        if (Decide is null) writer.WriteNullValue();
        else Decide.Write(writer);
        if (False.IsPresent) {
            writer.WritePropertyName("false");
            if (False.Value is null) writer.WriteNullValue();
            else False.Value.Write(writer);
        } if (ItemSchema.IsPresent) {
            writer.WritePropertyName("item_schema");
            if (ItemSchema.Value is null) writer.WriteNullValue();
            else ItemSchema.Value.Write(writer);
        } if (Model.IsPresent) {
            writer.WritePropertyName("model");
            if (Model.Value is null) writer.WriteNullValue();
            else Model.Value.Write(writer);
        } if (Name.IsPresent) {
            writer.WritePropertyName("name");
            WriteText(writer, Name.Value);
        } if (On.IsPresent) {
            writer.WritePropertyName("on");
            if (On.Value is null) writer.WriteNullValue();
            else On.Value.Write(writer);
        } if (Profile.IsPresent) {
            writer.WritePropertyName("profile");
            if (Profile.Value is null) writer.WriteNullValue();
            else Profile.Value.Write(writer);
        } if (Threshold.IsPresent) {
            writer.WritePropertyName("threshold");
            if (Threshold.Value is null) writer.WriteNullValue();
            else Threshold.Value.Write(writer);
        } if (True.IsPresent) {
            writer.WritePropertyName("true");
            if (True.Value is null) writer.WriteNullValue();
            else True.Value.Write(writer);
        } if (WordingVersion.IsPresent) {
            writer.WritePropertyName("wording_version");
            writer.WriteNumberValue(WordingVersion.Value);
        } writer.WriteEndObject();
    }
}
public abstract class InputAuthoredDecideBatch : InputDocument {
    private protected InputAuthoredDecideBatch() {
    }
}
public sealed class InputAuthoredDecideBatchInteger : InputAuthoredDecideBatch {
    public required long Value {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNumberValue(Value);
    }
}
public sealed class InputAuthoredDecideBatchMax : InputAuthoredDecideBatch {
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "max");
    }
}
public sealed class InputAuthoredDescription : InputDocument {
    public required InputAuthoredDescriptionValue Value {
        get;
        init;
    }
    internal static InputAuthoredDescription Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = InputAuthoredDescriptionValue.Read(value)
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } if (Value is null) writer.WriteNullValue();
        else Value.Write(writer);
    }
}
public abstract class InputAuthoredDescriptionValue : InputDocument {
    private protected InputAuthoredDescriptionValue() {
    } internal static InputAuthoredDescriptionValue Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.String)) return InputAuthoredDescriptionValueString.Read(value);
        if ((value.ValueKind == JsonValueKind.Object)) return InputAuthoredDescriptionValueObject.Read(value);
        if ((value.ValueKind == JsonValueKind.Array)) return InputAuthoredDescriptionValueList.Read(value);
        if ((value.ValueKind == JsonValueKind.Null)) return InputAuthoredDescriptionValueNull.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class InputAuthoredDescriptionValueList : InputAuthoredDescriptionValue {
    public required IReadOnlyList<JsonElement> Value {
        get;
        init;
    }
    internal new static InputAuthoredDescriptionValueList Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = Array.AsReadOnly(value.EnumerateArray().Select(item0 => item0.Clone()).ToArray())
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartArray();
        foreach (var item in Value) {
            item.WriteTo(writer);
        } writer.WriteEndArray();
    }
}
public sealed class InputAuthoredDescriptionValueNull : InputAuthoredDescriptionValue {
    internal new static InputAuthoredDescriptionValueNull Read(JsonElement value) => new() {
        ParsedDocument = value.Clone()
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNullValue();
    }
}
public sealed class InputAuthoredDescriptionValueObject : InputAuthoredDescriptionValue {
    public required JsonElement Value {
        get;
        init;
    }
    internal new static InputAuthoredDescriptionValueObject Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.Clone()
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } Value.WriteTo(writer);
    }
}
public sealed class InputAuthoredDescriptionValueString : InputAuthoredDescriptionValue {
    public required string Value {
        get;
        init;
    }
    internal new static InputAuthoredDescriptionValueString Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.GetString()!
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, Value);
    }
}
public sealed class InputAuthoredFind : InputDocument {
    public InputPresence<InputAuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public required InputAuthoredQuestionText Find {
        get;
        init;
    }
    public InputPresence<InputAuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public InputPresence<InputAuthoredName> Model {
        get;
        init;
    }
    public InputPresence<string> Name {
        get;
        init;
    }
    public InputPresence<InputAuthoredPointers> On {
        get;
        init;
    }
    public InputPresence<InputAuthoredProfile> Profile {
        get;
        init;
    }
    public InputPresence<long> WordingVersion {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        if (ContextSchema.IsPresent) {
            writer.WritePropertyName("context_schema");
            if (ContextSchema.Value is null) writer.WriteNullValue();
            else ContextSchema.Value.Write(writer);
        } writer.WritePropertyName("find");
        if (Find is null) writer.WriteNullValue();
        else Find.Write(writer);
        if (ItemSchema.IsPresent) {
            writer.WritePropertyName("item_schema");
            if (ItemSchema.Value is null) writer.WriteNullValue();
            else ItemSchema.Value.Write(writer);
        } if (Model.IsPresent) {
            writer.WritePropertyName("model");
            if (Model.Value is null) writer.WriteNullValue();
            else Model.Value.Write(writer);
        } if (Name.IsPresent) {
            writer.WritePropertyName("name");
            WriteText(writer, Name.Value);
        } if (On.IsPresent) {
            writer.WritePropertyName("on");
            if (On.Value is null) writer.WriteNullValue();
            else On.Value.Write(writer);
        } if (Profile.IsPresent) {
            writer.WritePropertyName("profile");
            if (Profile.Value is null) writer.WriteNullValue();
            else Profile.Value.Write(writer);
        } if (WordingVersion.IsPresent) {
            writer.WritePropertyName("wording_version");
            writer.WriteNumberValue(WordingVersion.Value);
        } writer.WriteEndObject();
    }
}
public abstract class InputAuthoredInputDeclaration : InputDocument {
    private protected InputAuthoredInputDeclaration() {
    } internal static InputAuthoredInputDeclaration Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("type",
        out _) && (value.GetProperty("type").ValueKind == JsonValueKind.String && value.GetProperty("type").GetString() == "string")) return InputAuthoredInputDeclarationString.Read(value);
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("type",
        out _) && value.TryGetProperty("properties", out _) && (value.GetProperty("type").ValueKind == JsonValueKind.String && value.GetProperty("type").GetString() == "object")) return InputAuthoredInputDeclarationObject.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class InputAuthoredInputDeclarationObject : InputAuthoredInputDeclaration {
    public required IReadOnlyDictionary<string, InputAuthoredInputProperty> Properties {
        get;
        init;
    }
    public InputPresence<IReadOnlyList<string>> Required {
        get;
        init;
    }
    internal new static InputAuthoredInputDeclarationObject Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Properties = new System.Collections.ObjectModel.ReadOnlyDictionary<string,
        InputAuthoredInputProperty>(value.GetProperty("properties").EnumerateObject().ToDictionary(entry0 => entry0.Name,
        entry0 => InputAuthoredInputProperty.Read(entry0.Value))), Required = value.TryGetProperty("required",
        out _) ? (InputPresence<IReadOnlyList<string>>)(Array.AsReadOnly(value.GetProperty("required").EnumerateArray().Select(item0 => item0.GetString()!).ToArray())) : default
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        writer.WritePropertyName("properties");
        writer.WriteStartObject();
        foreach (var entry in Properties) {
            WriteName(writer, entry.Key);
            if (entry.Value is null) writer.WriteNullValue();
            else entry.Value.Write(writer);
        } writer.WriteEndObject();
        if (Required.IsPresent) {
            writer.WritePropertyName("required");
            writer.WriteStartArray();
            foreach (var item in Required.Value) {
                WriteText(writer, item);
            } writer.WriteEndArray();
        } writer.WritePropertyName("type");
        WriteText(writer, "object");
        writer.WriteEndObject();
    }
}
public sealed class InputAuthoredInputDeclarationString : InputAuthoredInputDeclaration {
    internal new static InputAuthoredInputDeclarationString Read(JsonElement value) => new() {
        ParsedDocument = value.Clone()
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        writer.WritePropertyName("type");
        WriteText(writer, "string");
        writer.WriteEndObject();
    }
}
public abstract class InputAuthoredInputProperty : InputDocument {
    private protected InputAuthoredInputProperty() {
    } internal static InputAuthoredInputProperty Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("type",
        out _) && (value.GetProperty("type").ValueKind == JsonValueKind.String && value.GetProperty("type").GetString() == "string")) return InputAuthoredInputPropertyString.Read(value);
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("type",
        out _) && (value.GetProperty("type").ValueKind == JsonValueKind.String && value.GetProperty("type").GetString() == "number")) return InputAuthoredInputPropertyNumber.Read(value);
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("type",
        out _) && (value.GetProperty("type").ValueKind == JsonValueKind.String && value.GetProperty("type").GetString() == "boolean")) return InputAuthoredInputPropertyBoolean.Read(value);
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("type",
        out _) && value.TryGetProperty("items", out _) && (value.GetProperty("type").ValueKind == JsonValueKind.String && value.GetProperty("type").GetString() == "array")) return InputAuthoredInputPropertyArray.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class InputAuthoredInputPropertyArray : InputAuthoredInputProperty {
    public required InputAuthoredInputPropertyArrayItems Items {
        get;
        init;
    }
    internal new static InputAuthoredInputPropertyArray Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Items = InputAuthoredInputPropertyArrayItems.Read(value.GetProperty("items"))
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        writer.WritePropertyName("items");
        if (Items is null) writer.WriteNullValue();
        else Items.Write(writer);
        writer.WritePropertyName("type");
        WriteText(writer, "array");
        writer.WriteEndObject();
    }
}
public sealed class InputAuthoredInputPropertyArrayItems : InputDocument {
    internal static InputAuthoredInputPropertyArrayItems Read(JsonElement value) => new() {
        ParsedDocument = value.Clone()
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        writer.WritePropertyName("type");
        WriteText(writer, "string");
        writer.WriteEndObject();
    }
}
public sealed class InputAuthoredInputPropertyBoolean : InputAuthoredInputProperty {
    internal new static InputAuthoredInputPropertyBoolean Read(JsonElement value) => new() {
        ParsedDocument = value.Clone()
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        writer.WritePropertyName("type");
        WriteText(writer, "boolean");
        writer.WriteEndObject();
    }
}
public sealed class InputAuthoredInputPropertyNumber : InputAuthoredInputProperty {
    internal new static InputAuthoredInputPropertyNumber Read(JsonElement value) => new() {
        ParsedDocument = value.Clone()
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        writer.WritePropertyName("type");
        WriteText(writer, "number");
        writer.WriteEndObject();
    }
}
public sealed class InputAuthoredInputPropertyString : InputAuthoredInputProperty {
    internal new static InputAuthoredInputPropertyString Read(JsonElement value) => new() {
        ParsedDocument = value.Clone()
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        writer.WritePropertyName("type");
        WriteText(writer, "string");
        writer.WriteEndObject();
    }
}
public abstract class InputAuthoredLabels : InputDocument {
    private protected InputAuthoredLabels() {
    } internal static InputAuthoredLabels Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.Array)) return InputAuthoredLabelsList.Read(value);
        if ((value.ValueKind == JsonValueKind.Object)) return InputAuthoredLabelsObject.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class InputAuthoredLabelsList : InputAuthoredLabels {
    public required IReadOnlyList<InputAuthoredName> Value {
        get;
        init;
    }
    internal new static InputAuthoredLabelsList Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = Array.AsReadOnly(value.EnumerateArray().Select(item0 => InputAuthoredName.Read(item0)).ToArray())
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartArray();
        foreach (var item in Value) {
            if (item is null) writer.WriteNullValue();
            else item.Write(writer);
        } writer.WriteEndArray();
    }
}
public sealed class InputAuthoredLabelsObject : InputAuthoredLabels {
    public required IReadOnlyDictionary<string, InputAuthoredDescription> Value {
        get;
        init;
    }
    internal new static InputAuthoredLabelsObject Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = new System.Collections.ObjectModel.ReadOnlyDictionary<string,
        InputAuthoredDescription>(value.EnumerateObject().ToDictionary(entry0 => entry0.Name, entry0 => InputAuthoredDescription.Read(entry0.Value)))
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        foreach (var entry in Value) {
            WriteName(writer, entry.Key);
            if (entry.Value is null) writer.WriteNullValue();
            else entry.Value.Write(writer);
        } writer.WriteEndObject();
    }
}
public abstract class InputAuthoredLevels : InputDocument {
    private protected InputAuthoredLevels() {
    } internal static InputAuthoredLevels Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.Array)) return InputAuthoredLevelsList.Read(value);
        if ((value.ValueKind == JsonValueKind.Object)) return InputAuthoredLevelsObject.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class InputAuthoredLevelsList : InputAuthoredLevels {
    public required IReadOnlyList<InputAuthoredName> Value {
        get;
        init;
    }
    internal new static InputAuthoredLevelsList Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = Array.AsReadOnly(value.EnumerateArray().Select(item0 => InputAuthoredName.Read(item0)).ToArray())
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartArray();
        foreach (var item in Value) {
            if (item is null) writer.WriteNullValue();
            else item.Write(writer);
        } writer.WriteEndArray();
    }
}
public sealed class InputAuthoredLevelsObject : InputAuthoredLevels {
    public required IReadOnlyDictionary<string, InputAuthoredCriterion> Value {
        get;
        init;
    }
    internal new static InputAuthoredLevelsObject Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = new System.Collections.ObjectModel.ReadOnlyDictionary<string,
        InputAuthoredCriterion>(value.EnumerateObject().ToDictionary(entry0 => entry0.Name, entry0 => InputAuthoredCriterion.Read(entry0.Value)))
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        foreach (var entry in Value) {
            WriteName(writer, entry.Key);
            if (entry.Value is null) writer.WriteNullValue();
            else entry.Value.Write(writer);
        } writer.WriteEndObject();
    }
}
public sealed class InputAuthoredName : InputDocument {
    public required string Value {
        get;
        init;
    }
    internal static InputAuthoredName Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.GetString()!
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, Value);
    }
}
public abstract class InputAuthoredOptions : InputDocument {
    private protected InputAuthoredOptions() {
    } internal static InputAuthoredOptions Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.Array)) return InputAuthoredOptionsList.Read(value);
        if ((value.ValueKind == JsonValueKind.Object)) return InputAuthoredOptionsObject.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class InputAuthoredOptionsList : InputAuthoredOptions {
    public required IReadOnlyList<InputAuthoredName> Value {
        get;
        init;
    }
    internal new static InputAuthoredOptionsList Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = Array.AsReadOnly(value.EnumerateArray().Select(item0 => InputAuthoredName.Read(item0)).ToArray())
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartArray();
        foreach (var item in Value) {
            if (item is null) writer.WriteNullValue();
            else item.Write(writer);
        } writer.WriteEndArray();
    }
}
public sealed class InputAuthoredOptionsObject : InputAuthoredOptions {
    public required IReadOnlyDictionary<string, InputAuthoredDescription> Value {
        get;
        init;
    }
    internal new static InputAuthoredOptionsObject Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = new System.Collections.ObjectModel.ReadOnlyDictionary<string,
        InputAuthoredDescription>(value.EnumerateObject().ToDictionary(entry0 => entry0.Name, entry0 => InputAuthoredDescription.Read(entry0.Value)))
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        foreach (var entry in Value) {
            WriteName(writer, entry.Key);
            if (entry.Value is null) writer.WriteNullValue();
            else entry.Value.Write(writer);
        } writer.WriteEndObject();
    }
}
public abstract class InputAuthoredPointers : InputDocument {
    private protected InputAuthoredPointers() {
    } internal static InputAuthoredPointers Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.String)) return InputAuthoredPointersString.Read(value);
        if ((value.ValueKind == JsonValueKind.Array)) return InputAuthoredPointersList.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class InputAuthoredPointersList : InputAuthoredPointers {
    public required IReadOnlyList<string> Value {
        get;
        init;
    }
    internal new static InputAuthoredPointersList Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = Array.AsReadOnly(value.EnumerateArray().Select(item0 => item0.GetString()!).ToArray())
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartArray();
        foreach (var item in Value) {
            WriteText(writer, item);
        } writer.WriteEndArray();
    }
}
public sealed class InputAuthoredPointersString : InputAuthoredPointers {
    public required string Value {
        get;
        init;
    }
    internal new static InputAuthoredPointersString Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.GetString()!
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, Value);
    }
}
public sealed class InputAuthoredProfile : InputDocument {
    public required string Value {
        get;
        init;
    }
    internal static InputAuthoredProfile Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.GetString()!
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, Value);
    }
}
public abstract class InputAuthoredQuestionText : InputDocument {
    private protected InputAuthoredQuestionText() {
    } internal static InputAuthoredQuestionText Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.String)) return InputAuthoredQuestionTextString.Read(value);
        if ((value.ValueKind == JsonValueKind.Object)) return InputAuthoredQuestionTextObject.Read(value);
        if ((value.ValueKind == JsonValueKind.Array)) return InputAuthoredQuestionTextList.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class InputAuthoredQuestionTextList : InputAuthoredQuestionText {
    public required IReadOnlyList<JsonElement> Value {
        get;
        init;
    }
    internal new static InputAuthoredQuestionTextList Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = Array.AsReadOnly(value.EnumerateArray().Select(item0 => item0.Clone()).ToArray())
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartArray();
        foreach (var item in Value) {
            item.WriteTo(writer);
        } writer.WriteEndArray();
    }
}
public sealed class InputAuthoredQuestionTextObject : InputAuthoredQuestionText {
    public required JsonElement Value {
        get;
        init;
    }
    internal new static InputAuthoredQuestionTextObject Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.Clone()
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } Value.WriteTo(writer);
    }
}
public sealed class InputAuthoredQuestionTextString : InputAuthoredQuestionText {
    public required string Value {
        get;
        init;
    }
    internal new static InputAuthoredQuestionTextString Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.GetString()!
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, Value);
    }
}
public sealed class InputAuthoredRelate : InputDocument {
    public InputPresence<InputAuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public InputPresence<InputAuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public InputPresence<InputAuthoredName> Model {
        get;
        init;
    }
    public InputPresence<string> Name {
        get;
        init;
    }
    public InputPresence<InputAuthoredProfile> Profile {
        get;
        init;
    }
    public required InputRelateQuestion Relate {
        get;
        init;
    }
    public InputPresence<InputAuthoredCut> Threshold {
        get;
        init;
    }
    public InputPresence<long> WordingVersion {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        if (ContextSchema.IsPresent) {
            writer.WritePropertyName("context_schema");
            if (ContextSchema.Value is null) writer.WriteNullValue();
            else ContextSchema.Value.Write(writer);
        } if (ItemSchema.IsPresent) {
            writer.WritePropertyName("item_schema");
            if (ItemSchema.Value is null) writer.WriteNullValue();
            else ItemSchema.Value.Write(writer);
        } if (Model.IsPresent) {
            writer.WritePropertyName("model");
            if (Model.Value is null) writer.WriteNullValue();
            else Model.Value.Write(writer);
        } if (Name.IsPresent) {
            writer.WritePropertyName("name");
            WriteText(writer, Name.Value);
        } if (Profile.IsPresent) {
            writer.WritePropertyName("profile");
            if (Profile.Value is null) writer.WriteNullValue();
            else Profile.Value.Write(writer);
        } writer.WritePropertyName("relate");
        if (Relate is null) writer.WriteNullValue();
        else Relate.Write(writer);
        if (Threshold.IsPresent) {
            writer.WritePropertyName("threshold");
            if (Threshold.Value is null) writer.WriteNullValue();
            else Threshold.Value.Write(writer);
        } writer.WritePropertyName("version");
        writer.WriteNumberValue(1);
        if (WordingVersion.IsPresent) {
            writer.WritePropertyName("wording_version");
            writer.WriteNumberValue(WordingVersion.Value);
        } writer.WriteEndObject();
    }
}
public sealed class InputAuthoredRelation : InputDocument {
    public InputPresence<bool> Either {
        get;
        init;
    }
    public required InputAuthoredName Name {
        get;
        init;
    }
    public InputPresence<InputAuthoredName> Reads {
        get;
        init;
    }
    public InputPresence<bool> Single {
        get;
        init;
    }
    public InputPresence<InputAuthoredName> Source {
        get;
        init;
    }
    public InputPresence<InputAuthoredName> Target {
        get;
        init;
    }
    internal static InputAuthoredRelation Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Either = value.TryGetProperty("either", out _) ? (InputPresence<bool>)(value.GetProperty("either").GetBoolean()) : default,
        Name = InputAuthoredName.Read(value.GetProperty("name")), Reads = value.TryGetProperty("reads",
        out _) ? (InputPresence<InputAuthoredName>)(InputAuthoredName.Read(value.GetProperty("reads"))) : default,
        Single = value.TryGetProperty("single", out _) ? (InputPresence<bool>)(value.GetProperty("single").GetBoolean()) : default,
        Source = value.TryGetProperty("source", out _) ? (InputPresence<InputAuthoredName>)(InputAuthoredName.Read(value.GetProperty("source"))) : default,
        Target = value.TryGetProperty("target", out _) ? (InputPresence<InputAuthoredName>)(InputAuthoredName.Read(value.GetProperty("target"))) : default
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        if (Either.IsPresent) {
            writer.WritePropertyName("either");
            writer.WriteBooleanValue(Either.Value);
        } writer.WritePropertyName("name");
        if (Name is null) writer.WriteNullValue();
        else Name.Write(writer);
        if (Reads.IsPresent) {
            writer.WritePropertyName("reads");
            if (Reads.Value is null) writer.WriteNullValue();
            else Reads.Value.Write(writer);
        } if (Single.IsPresent) {
            writer.WritePropertyName("single");
            writer.WriteBooleanValue(Single.Value);
        } if (Source.IsPresent) {
            writer.WritePropertyName("source");
            if (Source.Value is null) writer.WriteNullValue();
            else Source.Value.Write(writer);
        } if (Target.IsPresent) {
            writer.WritePropertyName("target");
            if (Target.Value is null) writer.WriteNullValue();
            else Target.Value.Write(writer);
        } writer.WriteEndObject();
    }
}
public sealed class InputAuthoredScore : InputDocument {
    public InputPresence<InputAuthoredScoreBatch> Batch {
        get;
        init;
    }
    public InputPresence<InputAuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public InputPresence<InputAuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public InputPresence<InputAuthoredLevels> Levels {
        get;
        init;
    }
    public InputPresence<InputAuthoredName> Model {
        get;
        init;
    }
    public InputPresence<string> Name {
        get;
        init;
    }
    public InputPresence<InputAuthoredPointers> On {
        get;
        init;
    }
    public InputPresence<InputAuthoredProfile> Profile {
        get;
        init;
    }
    public required InputAuthoredQuestionText Score {
        get;
        init;
    }
    public InputPresence<long> WordingVersion {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        if (Batch.IsPresent) {
            writer.WritePropertyName("batch");
            if (Batch.Value is null) writer.WriteNullValue();
            else Batch.Value.Write(writer);
        } if (ContextSchema.IsPresent) {
            writer.WritePropertyName("context_schema");
            if (ContextSchema.Value is null) writer.WriteNullValue();
            else ContextSchema.Value.Write(writer);
        } if (ItemSchema.IsPresent) {
            writer.WritePropertyName("item_schema");
            if (ItemSchema.Value is null) writer.WriteNullValue();
            else ItemSchema.Value.Write(writer);
        } if (Levels.IsPresent) {
            writer.WritePropertyName("levels");
            if (Levels.Value is null) writer.WriteNullValue();
            else Levels.Value.Write(writer);
        } if (Model.IsPresent) {
            writer.WritePropertyName("model");
            if (Model.Value is null) writer.WriteNullValue();
            else Model.Value.Write(writer);
        } if (Name.IsPresent) {
            writer.WritePropertyName("name");
            WriteText(writer, Name.Value);
        } if (On.IsPresent) {
            writer.WritePropertyName("on");
            if (On.Value is null) writer.WriteNullValue();
            else On.Value.Write(writer);
        } if (Profile.IsPresent) {
            writer.WritePropertyName("profile");
            if (Profile.Value is null) writer.WriteNullValue();
            else Profile.Value.Write(writer);
        } writer.WritePropertyName("score");
        if (Score is null) writer.WriteNullValue();
        else Score.Write(writer);
        if (WordingVersion.IsPresent) {
            writer.WritePropertyName("wording_version");
            writer.WriteNumberValue(WordingVersion.Value);
        } writer.WriteEndObject();
    }
}
public abstract class InputAuthoredScoreBatch : InputDocument {
    private protected InputAuthoredScoreBatch() {
    }
}
public sealed class InputAuthoredScoreBatchInteger : InputAuthoredScoreBatch {
    public required long Value {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNumberValue(Value);
    }
}
public sealed class InputAuthoredScoreBatchMax : InputAuthoredScoreBatch {
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "max");
    }
}
public sealed class InputAuthoredTag : InputDocument {
    public InputPresence<InputAuthoredTagBatch> Batch {
        get;
        init;
    }
    public InputPresence<InputAuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public InputPresence<InputAuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public InputPresence<InputAuthoredLabels> Labels {
        get;
        init;
    }
    public InputPresence<InputAuthoredName> Model {
        get;
        init;
    }
    public InputPresence<string> Name {
        get;
        init;
    }
    public InputPresence<InputAuthoredPointers> On {
        get;
        init;
    }
    public InputPresence<InputAuthoredProfile> Profile {
        get;
        init;
    }
    public required InputAuthoredQuestionText Tag {
        get;
        init;
    }
    public InputPresence<InputAuthoredCut> Threshold {
        get;
        init;
    }
    public InputPresence<long> WordingVersion {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        if (Batch.IsPresent) {
            writer.WritePropertyName("batch");
            if (Batch.Value is null) writer.WriteNullValue();
            else Batch.Value.Write(writer);
        } if (ContextSchema.IsPresent) {
            writer.WritePropertyName("context_schema");
            if (ContextSchema.Value is null) writer.WriteNullValue();
            else ContextSchema.Value.Write(writer);
        } if (ItemSchema.IsPresent) {
            writer.WritePropertyName("item_schema");
            if (ItemSchema.Value is null) writer.WriteNullValue();
            else ItemSchema.Value.Write(writer);
        } if (Labels.IsPresent) {
            writer.WritePropertyName("labels");
            if (Labels.Value is null) writer.WriteNullValue();
            else Labels.Value.Write(writer);
        } if (Model.IsPresent) {
            writer.WritePropertyName("model");
            if (Model.Value is null) writer.WriteNullValue();
            else Model.Value.Write(writer);
        } if (Name.IsPresent) {
            writer.WritePropertyName("name");
            WriteText(writer, Name.Value);
        } if (On.IsPresent) {
            writer.WritePropertyName("on");
            if (On.Value is null) writer.WriteNullValue();
            else On.Value.Write(writer);
        } if (Profile.IsPresent) {
            writer.WritePropertyName("profile");
            if (Profile.Value is null) writer.WriteNullValue();
            else Profile.Value.Write(writer);
        } writer.WritePropertyName("tag");
        if (Tag is null) writer.WriteNullValue();
        else Tag.Write(writer);
        if (Threshold.IsPresent) {
            writer.WritePropertyName("threshold");
            if (Threshold.Value is null) writer.WriteNullValue();
            else Threshold.Value.Write(writer);
        } if (WordingVersion.IsPresent) {
            writer.WritePropertyName("wording_version");
            writer.WriteNumberValue(WordingVersion.Value);
        } writer.WriteEndObject();
    }
}
public abstract class InputAuthoredTagBatch : InputDocument {
    private protected InputAuthoredTagBatch() {
    }
}
public sealed class InputAuthoredTagBatchInteger : InputAuthoredTagBatch {
    public required long Value {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNumberValue(Value);
    }
}
public sealed class InputAuthoredTagBatchMax : InputAuthoredTagBatch {
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "max");
    }
}
public abstract class InputAuthoredThreshold : InputDocument {
    private protected InputAuthoredThreshold() {
    } internal static InputAuthoredThreshold Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.Number)) return InputAuthoredThresholdNumber.Read(value);
        if ((value.ValueKind == JsonValueKind.String)) return InputAuthoredThresholdString.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class InputAuthoredThresholdNumber : InputAuthoredThreshold {
    public required double Value {
        get;
        init;
    }
    internal new static InputAuthoredThresholdNumber Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.GetDouble()
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNumberValue(Value);
    }
}
public sealed class InputAuthoredThresholdString : InputAuthoredThreshold {
    public required string Value {
        get;
        init;
    }
    internal new static InputAuthoredThresholdString Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.GetString()!
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, Value);
    }
}
public abstract class InputCacheDocument : InputDocument {
    private protected InputCacheDocument() {
    }
}
public sealed class InputCacheDocumentDisabledCache : InputCacheDocument {
    public required InputDisabledCache Value {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } if (Value is null) writer.WriteNullValue();
        else Value.Write(writer);
    }
}
public sealed class InputCacheDocumentString : InputCacheDocument {
    public required string Value {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, Value);
    }
}
public abstract class InputContextSchema : InputDocument {
    private protected InputContextSchema() {
    }
}
public sealed class InputContextSchemaObject : InputContextSchema {
    public required JsonElement Value {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } Value.WriteTo(writer);
    }
}
public sealed class InputContextSchemaString : InputContextSchema {
    public required string Value {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, Value);
    }
}
public sealed class InputDisabledCache : InputDocument {
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteBooleanValue(false);
    }
}
public sealed class InputEngineSettings : InputDocument {
    public InputPresence<string> Backend {
        get;
        init;
    }
    public InputPresence<string> BaseUrl {
        get;
        init;
    }
    public InputPresence<InputRequestBatch> Batch {
        get;
        init;
    }
    public InputPresence<InputCacheDocument> Cache {
        get;
        init;
    }
    public InputPresence<InputEngineSettingsMaxEstimatedInputTokensTotal> MaxEstimatedInputTokensTotal {
        get;
        init;
    }
    public InputPresence<ulong> MaxRequestBytes {
        get;
        init;
    }
    public InputPresence<InputEngineSettingsMaxRequests> MaxRequests {
        get;
        init;
    }
    public InputPresence<InputEngineSettingsMaxRequestsTotal> MaxRequestsTotal {
        get;
        init;
    }
    public InputPresence<uint> MaxRetries {
        get;
        init;
    }
    public InputPresence<string> Model {
        get;
        init;
    }
    public InputPresence<string> Profile {
        get;
        init;
    }
    public InputPresence<JsonElement> Proxy {
        get;
        init;
    }
    public InputPresence<string> Record {
        get;
        init;
    }
    public InputPresence<bool> RefreshCache {
        get;
        init;
    }
    public InputPresence<string> Replay {
        get;
        init;
    }
    public InputPresence<long> Throttle {
        get;
        init;
    }
    public InputPresence<ulong> Timeout {
        get;
        init;
    }
    public InputPresence<string> UsdPerMillionInput {
        get;
        init;
    }
    public InputPresence<string> UsdPerMillionOutput {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        if (Backend.IsPresent) {
            writer.WritePropertyName("backend");
            WriteText(writer, Backend.Value);
        } if (BaseUrl.IsPresent) {
            writer.WritePropertyName("base_url");
            WriteText(writer, BaseUrl.Value);
        } if (Batch.IsPresent) {
            writer.WritePropertyName("batch");
            if (Batch.Value is null) writer.WriteNullValue();
            else Batch.Value.Write(writer);
        } if (Cache.IsPresent) {
            writer.WritePropertyName("cache");
            if (Cache.Value is null) writer.WriteNullValue();
            else Cache.Value.Write(writer);
        } if (MaxEstimatedInputTokensTotal.IsPresent) {
            writer.WritePropertyName("max_estimated_input_tokens_total");
            if (MaxEstimatedInputTokensTotal.Value is null) writer.WriteNullValue();
            else MaxEstimatedInputTokensTotal.Value.Write(writer);
        } if (MaxRequestBytes.IsPresent) {
            writer.WritePropertyName("max_request_bytes");
            writer.WriteNumberValue(MaxRequestBytes.Value);
        } if (MaxRequests.IsPresent) {
            writer.WritePropertyName("max_requests");
            if (MaxRequests.Value is null) writer.WriteNullValue();
            else MaxRequests.Value.Write(writer);
        } if (MaxRequestsTotal.IsPresent) {
            writer.WritePropertyName("max_requests_total");
            if (MaxRequestsTotal.Value is null) writer.WriteNullValue();
            else MaxRequestsTotal.Value.Write(writer);
        } if (MaxRetries.IsPresent) {
            writer.WritePropertyName("max_retries");
            writer.WriteNumberValue(MaxRetries.Value);
        } if (Model.IsPresent) {
            writer.WritePropertyName("model");
            WriteText(writer, Model.Value);
        } if (Profile.IsPresent) {
            writer.WritePropertyName("profile");
            WriteText(writer, Profile.Value);
        } if (Proxy.IsPresent) {
            writer.WritePropertyName("proxy");
            Proxy.Value.WriteTo(writer);
        } if (Record.IsPresent) {
            writer.WritePropertyName("record");
            WriteText(writer, Record.Value);
        } if (RefreshCache.IsPresent) {
            writer.WritePropertyName("refresh_cache");
            writer.WriteBooleanValue(RefreshCache.Value);
        } if (Replay.IsPresent) {
            writer.WritePropertyName("replay");
            WriteText(writer, Replay.Value);
        } if (Throttle.IsPresent) {
            writer.WritePropertyName("throttle");
            writer.WriteNumberValue(Throttle.Value);
        } if (Timeout.IsPresent) {
            writer.WritePropertyName("timeout");
            writer.WriteNumberValue(Timeout.Value);
        } if (UsdPerMillionInput.IsPresent) {
            writer.WritePropertyName("usd_per_million_input");
            WriteText(writer, UsdPerMillionInput.Value);
        } if (UsdPerMillionOutput.IsPresent) {
            writer.WritePropertyName("usd_per_million_output");
            WriteText(writer, UsdPerMillionOutput.Value);
        } writer.WriteEndObject();
    }
}
public abstract class InputEngineSettingsMaxEstimatedInputTokensTotal : InputDocument {
    private protected InputEngineSettingsMaxEstimatedInputTokensTotal() {
    }
}
public sealed class InputEngineSettingsMaxEstimatedInputTokensTotalInteger : InputEngineSettingsMaxEstimatedInputTokensTotal {
    public required ulong Value {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNumberValue(Value);
    }
}
public sealed class InputEngineSettingsMaxEstimatedInputTokensTotalNull : InputEngineSettingsMaxEstimatedInputTokensTotal {
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNullValue();
    }
}
public abstract class InputEngineSettingsMaxRequests : InputDocument {
    private protected InputEngineSettingsMaxRequests() {
    }
}
public sealed class InputEngineSettingsMaxRequestsInteger : InputEngineSettingsMaxRequests {
    public required ulong Value {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNumberValue(Value);
    }
}
public sealed class InputEngineSettingsMaxRequestsNull : InputEngineSettingsMaxRequests {
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNullValue();
    }
}
public abstract class InputEngineSettingsMaxRequestsTotal : InputDocument {
    private protected InputEngineSettingsMaxRequestsTotal() {
    }
}
public sealed class InputEngineSettingsMaxRequestsTotalInteger : InputEngineSettingsMaxRequestsTotal {
    public required ulong Value {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNumberValue(Value);
    }
}
public sealed class InputEngineSettingsMaxRequestsTotalNull : InputEngineSettingsMaxRequestsTotal {
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNullValue();
    }
}
public abstract class InputImageMedia : InputDocument {
    private protected InputImageMedia() {
    }
}
public sealed class InputImageMediaImageJpeg : InputImageMedia {
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "image/jpeg");
    }
}
public sealed class InputImageMediaImagePng : InputImageMedia {
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "image/png");
    }
}
public sealed class InputOptionSchema : InputDocument {
    public InputPresence<JsonElement> Description {
        get;
        init;
    }
    public required string Name {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        if (Description.IsPresent) {
            writer.WritePropertyName("description");
            Description.Value.WriteTo(writer);
        } writer.WritePropertyName("name");
        WriteText(writer, Name);
        writer.WriteEndObject();
    }
}
public abstract class InputReaderMedia : InputDocument {
    private protected InputReaderMedia() {
    }
}
public sealed class InputReaderMediaImage : InputReaderMedia {
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "image");
    }
}
public sealed class InputReaderMediaText : InputReaderMedia {
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "text");
    }
}
public abstract class InputRecognitionExample : InputDocument {
    private protected InputRecognitionExample() {
    }
}
public sealed class InputRecognitionExampleEntity : InputDocument {
    public required ulong End {
        get;
        init;
    }
    public required string Kind {
        get;
        init;
    }
    public required ulong Start {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        writer.WritePropertyName("end");
        writer.WriteNumberValue(End);
        writer.WritePropertyName("kind");
        WriteText(writer, Kind);
        writer.WritePropertyName("start");
        writer.WriteNumberValue(Start);
        writer.WriteEndObject();
    }
}
public sealed class InputRecognitionExampleText : InputDocument {
    public required IReadOnlyList<InputRecognitionExampleEntity> Entities {
        get;
        init;
    }
    public InputPresence<IReadOnlyList<string>> Kinds {
        get;
        init;
    }
    public required string Text {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        writer.WritePropertyName("entities");
        writer.WriteStartArray();
        foreach (var item in Entities) {
            if (item is null) writer.WriteNullValue();
            else item.Write(writer);
        } writer.WriteEndArray();
        if (Kinds.IsPresent) {
            writer.WritePropertyName("kinds");
            writer.WriteStartArray();
            foreach (var item in Kinds.Value) {
                WriteText(writer, item);
            } writer.WriteEndArray();
        } writer.WritePropertyName("text");
        WriteText(writer, Text);
        writer.WriteEndObject();
    }
}
public sealed class InputRecognitionExampleRecognitionExampleText : InputRecognitionExample {
    public required InputRecognitionExampleText Value {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } if (Value is null) writer.WriteNullValue();
        else Value.Write(writer);
    }
}
public sealed class InputRecognitionExampleString : InputRecognitionExample {
    public required string Value {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, Value);
    }
}
public abstract class InputRecognitionMode : InputDocument {
    private protected InputRecognitionMode() {
    } internal static InputRecognitionMode Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.String) && (value.ValueKind == JsonValueKind.String && value.GetString() == "whole")) return InputRecognitionModeWhole.Read(value);
        if ((value.ValueKind == JsonValueKind.String) && (value.ValueKind == JsonValueKind.String && value.GetString() == "boundary_only")) return InputRecognitionModeBoundaryOnly.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class InputRecognitionModeBoundaryOnly : InputRecognitionMode {
    internal new static InputRecognitionModeBoundaryOnly Read(JsonElement value) => new() {
        ParsedDocument = value.Clone()
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "boundary_only");
    }
}
public sealed class InputRecognitionModeWhole : InputRecognitionMode {
    internal new static InputRecognitionModeWhole Read(JsonElement value) => new() {
        ParsedDocument = value.Clone()
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "whole");
    }
}
public sealed class InputRecognitionSeedSpan : InputDocument {
    public required ulong End {
        get;
        init;
    }
    public InputPresence<string> Kind {
        get;
        init;
    }
    public required ulong Start {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        writer.WritePropertyName("end");
        writer.WriteNumberValue(End);
        if (Kind.IsPresent) {
            writer.WritePropertyName("kind");
            WriteText(writer, Kind.Value);
        } writer.WritePropertyName("start");
        writer.WriteNumberValue(Start);
        writer.WriteEndObject();
    }
}
public sealed class InputRecognitionStageContext : InputDocument {
    public InputPresence<string> Boundary {
        get;
        init;
    }
    public InputPresence<string> KindEdge {
        get;
        init;
    }
    public InputPresence<string> Relation {
        get;
        init;
    }
    internal static InputRecognitionStageContext Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Boundary = value.TryGetProperty("boundary", out _) ? (InputPresence<string>)(value.GetProperty("boundary").GetString()!) : default,
        KindEdge = value.TryGetProperty("kind_edge", out _) ? (InputPresence<string>)(value.GetProperty("kind_edge").GetString()!) : default,
        Relation = value.TryGetProperty("relation", out _) ? (InputPresence<string>)(value.GetProperty("relation").GetString()!) : default
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        if (Boundary.IsPresent) {
            writer.WritePropertyName("boundary");
            WriteText(writer, Boundary.Value);
        } if (KindEdge.IsPresent) {
            writer.WritePropertyName("kind_edge");
            WriteText(writer, KindEdge.Value);
        } if (Relation.IsPresent) {
            writer.WritePropertyName("relation");
            WriteText(writer, Relation.Value);
        } writer.WriteEndObject();
    }
}
public sealed class InputRecognizeDefinition : InputDocument {
    public InputPresence<InputAuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public InputPresence<InputAuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public InputPresence<InputAuthoredName> Model {
        get;
        init;
    }
    public InputPresence<string> Name {
        get;
        init;
    }
    public InputPresence<InputAuthoredPointers> On {
        get;
        init;
    }
    public InputPresence<InputAuthoredProfile> Profile {
        get;
        init;
    }
    public required InputRecognizeQuestion Recognize {
        get;
        init;
    }
    public InputPresence<InputAuthoredCut> RelationThreshold {
        get;
        init;
    }
    public InputPresence<InputAuthoredCut> Threshold {
        get;
        init;
    }
    public InputPresence<long> WordingVersion {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        if (ContextSchema.IsPresent) {
            writer.WritePropertyName("context_schema");
            if (ContextSchema.Value is null) writer.WriteNullValue();
            else ContextSchema.Value.Write(writer);
        } if (ItemSchema.IsPresent) {
            writer.WritePropertyName("item_schema");
            if (ItemSchema.Value is null) writer.WriteNullValue();
            else ItemSchema.Value.Write(writer);
        } if (Model.IsPresent) {
            writer.WritePropertyName("model");
            if (Model.Value is null) writer.WriteNullValue();
            else Model.Value.Write(writer);
        } if (Name.IsPresent) {
            writer.WritePropertyName("name");
            WriteText(writer, Name.Value);
        } if (On.IsPresent) {
            writer.WritePropertyName("on");
            if (On.Value is null) writer.WriteNullValue();
            else On.Value.Write(writer);
        } if (Profile.IsPresent) {
            writer.WritePropertyName("profile");
            if (Profile.Value is null) writer.WriteNullValue();
            else Profile.Value.Write(writer);
        } writer.WritePropertyName("recognize");
        if (Recognize is null) writer.WriteNullValue();
        else Recognize.Write(writer);
        if (RelationThreshold.IsPresent) {
            writer.WritePropertyName("relation_threshold");
            if (RelationThreshold.Value is null) writer.WriteNullValue();
            else RelationThreshold.Value.Write(writer);
        } if (Threshold.IsPresent) {
            writer.WritePropertyName("threshold");
            if (Threshold.Value is null) writer.WriteNullValue();
            else Threshold.Value.Write(writer);
        } writer.WritePropertyName("version");
        writer.WriteNumberValue(1);
        if (WordingVersion.IsPresent) {
            writer.WritePropertyName("wording_version");
            writer.WriteNumberValue(WordingVersion.Value);
        } writer.WriteEndObject();
    }
}
public sealed class InputRecognizeQuestion : InputDocument {
    public InputPresence<InputAuthoredQuestionText> EntityDefinition {
        get;
        init;
    }
    public InputPresence<InputAuthoredQuestionText> Instructions {
        get;
        init;
    }
    public InputPresence<IReadOnlyDictionary<string, InputAuthoredDescription>> Kinds {
        get;
        init;
    }
    public InputPresence<InputRecognitionMode> Mode {
        get;
        init;
    }
    public InputPresence<IReadOnlyList<InputAuthoredRelation>> Relations {
        get;
        init;
    }
    public InputPresence<uint> SnippetPieces {
        get;
        init;
    }
    public InputPresence<InputRecognitionStageContext> StageContext {
        get;
        init;
    }
    internal static InputRecognizeQuestion Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), EntityDefinition = value.TryGetProperty("entity_definition",
        out _) ? (InputPresence<InputAuthoredQuestionText>)(InputAuthoredQuestionText.Read(value.GetProperty("entity_definition"))) : default,
        Instructions = value.TryGetProperty("instructions", out _) ? (InputPresence<InputAuthoredQuestionText>)(InputAuthoredQuestionText.Read(value.GetProperty("instructions"))) : default,
        Kinds = value.TryGetProperty("kinds", out _) ? (InputPresence<IReadOnlyDictionary<string, InputAuthoredDescription>>)(new System.Collections.ObjectModel.ReadOnlyDictionary<string,
        InputAuthoredDescription>(value.GetProperty("kinds").EnumerateObject().ToDictionary(entry0 => entry0.Name,
        entry0 => InputAuthoredDescription.Read(entry0.Value)))) : default, Mode = value.TryGetProperty("mode",
        out _) ? (InputPresence<InputRecognitionMode>)(InputRecognitionMode.Read(value.GetProperty("mode"))) : default,
        Relations = value.TryGetProperty("relations", out _) ? (InputPresence<IReadOnlyList<InputAuthoredRelation>>)(Array.AsReadOnly(value.GetProperty("relations").EnumerateArray().Select(item0 => InputAuthoredRelation.Read(item0)).ToArray())) : default,
        SnippetPieces = value.TryGetProperty("snippet_pieces", out _) ? (InputPresence<uint>)(value.GetProperty("snippet_pieces").GetUInt32()) : default,
        StageContext = value.TryGetProperty("stage_context", out _) ? (InputPresence<InputRecognitionStageContext>)(InputRecognitionStageContext.Read(value.GetProperty("stage_context"))) : default
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        if (EntityDefinition.IsPresent) {
            writer.WritePropertyName("entity_definition");
            if (EntityDefinition.Value is null) writer.WriteNullValue();
            else EntityDefinition.Value.Write(writer);
        } if (Instructions.IsPresent) {
            writer.WritePropertyName("instructions");
            if (Instructions.Value is null) writer.WriteNullValue();
            else Instructions.Value.Write(writer);
        } if (Kinds.IsPresent) {
            writer.WritePropertyName("kinds");
            writer.WriteStartObject();
            foreach (var entry in Kinds.Value) {
                WriteName(writer, entry.Key);
                if (entry.Value is null) writer.WriteNullValue();
                else entry.Value.Write(writer);
            } writer.WriteEndObject();
        } if (Mode.IsPresent) {
            writer.WritePropertyName("mode");
            if (Mode.Value is null) writer.WriteNullValue();
            else Mode.Value.Write(writer);
        } if (Relations.IsPresent) {
            writer.WritePropertyName("relations");
            writer.WriteStartArray();
            foreach (var item in Relations.Value) {
                if (item is null) writer.WriteNullValue();
                else item.Write(writer);
            } writer.WriteEndArray();
        } if (SnippetPieces.IsPresent) {
            writer.WritePropertyName("snippet_pieces");
            writer.WriteNumberValue(SnippetPieces.Value);
        } if (StageContext.IsPresent) {
            writer.WritePropertyName("stage_context");
            if (StageContext.Value is null) writer.WriteNullValue();
            else StageContext.Value.Write(writer);
        } writer.WriteEndObject();
    }
}
public sealed class InputRelateFields : InputDocument {
    public required string Kind {
        get;
        init;
    }
    public required string Name {
        get;
        init;
    }
    internal static InputRelateFields Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Kind = value.GetProperty("kind").GetString()!, Name = value.GetProperty("name").GetString()!
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        writer.WritePropertyName("kind");
        WriteText(writer, Kind);
        writer.WritePropertyName("name");
        WriteText(writer, Name);
        writer.WriteEndObject();
    }
}
public sealed class InputRelateQuestion : InputDocument {
    public InputPresence<InputRelateFields> Fields {
        get;
        init;
    }
    public required IReadOnlyList<InputAuthoredRelation> Relations {
        get;
        init;
    }
    internal static InputRelateQuestion Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Fields = value.TryGetProperty("fields", out _) ? (InputPresence<InputRelateFields>)(InputRelateFields.Read(value.GetProperty("fields"))) : default,
        Relations = Array.AsReadOnly(value.GetProperty("relations").EnumerateArray().Select(item0 => InputAuthoredRelation.Read(item0)).ToArray())
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        if (Fields.IsPresent) {
            writer.WritePropertyName("fields");
            if (Fields.Value is null) writer.WriteNullValue();
            else Fields.Value.Write(writer);
        } writer.WritePropertyName("relations");
        writer.WriteStartArray();
        foreach (var item in Relations) {
            if (item is null) writer.WriteNullValue();
            else item.Write(writer);
        } writer.WriteEndArray();
        writer.WriteEndObject();
    }
}
public sealed class InputRequest : InputDocument {
    public required InputRequestCall Call {
        get;
        init;
    }
    public required InputRequestVersion Schema {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        writer.WritePropertyName("call");
        if (Call is null) writer.WriteNullValue();
        else Call.Write(writer);
        writer.WritePropertyName("schema");
        if (Schema is null) writer.WriteNullValue();
        else Schema.Write(writer);
        writer.WriteEndObject();
    }
}
public abstract class InputRequestBatch : InputDocument {
    private protected InputRequestBatch() {
    }
}
public sealed class InputRequestBatchInteger : InputRequestBatch {
    public required ulong Value {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNumberValue(Value);
    }
}
public sealed class InputRequestBatchString : InputRequestBatch {
    public required string Value {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, Value);
    }
}
public sealed class InputRequestCall : InputDocument {
    public required InputRequestFunction Function {
        get;
        init;
    }
    public required InputRequestInput Input {
        get;
        init;
    }
    public InputPresence<InputRequestOptions> Options {
        get;
        init;
    }
    public required InputRequestQuestion Question {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        writer.WritePropertyName("function");
        if (Function is null) writer.WriteNullValue();
        else Function.Write(writer);
        writer.WritePropertyName("input");
        if (Input is null) writer.WriteNullValue();
        else Input.Write(writer);
        if (Options.IsPresent) {
            writer.WritePropertyName("options");
            if (Options.Value is null) writer.WriteNullValue();
            else Options.Value.Write(writer);
        } writer.WritePropertyName("question");
        if (Question is null) writer.WriteNullValue();
        else Question.Write(writer);
        writer.WriteEndObject();
    }
}
public abstract class InputRequestDefinition : InputDocument {
    private protected InputRequestDefinition() {
    } internal static InputRequestDefinition Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("decide",
        out _)) return InputRequestDefinitionAuthoredDecide.Read(value);
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("choose",
        out _)) return InputRequestDefinitionAuthoredChoose.Read(value);
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("tag",
        out _)) return InputRequestDefinitionAuthoredTag.Read(value);
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("score",
        out _)) return InputRequestDefinitionAuthoredScore.Read(value);
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("version",
        out _) && value.TryGetProperty("relate", out _) && (value.GetProperty("version").ValueKind == JsonValueKind.Number && value.GetProperty("version").GetInt64() == 1)) return InputRequestDefinitionAuthoredRelate.Read(value);
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("find",
        out _)) return InputRequestDefinitionAuthoredFind.Read(value);
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("version",
        out _) && value.TryGetProperty("recognize", out _) && (value.GetProperty("version").ValueKind == JsonValueKind.Number && value.GetProperty("version").GetInt64() == 1)) return InputRequestDefinitionRecognizeDefinition.Read(value);
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("version",
        out _) && value.TryGetProperty("questions", out _) && (value.GetProperty("version").ValueKind == JsonValueKind.Number && value.GetProperty("version").GetInt64() == 1)) return InputRequestDefinitionAnnotateDefinition.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class InputRequestDefinitionAnnotateDefinition : InputRequestDefinition {
    public InputPresence<JsonElement> Batch {
        get;
        init;
    }
    public InputPresence<InputAuthoredProfile> Profile {
        get;
        init;
    }
    public required IReadOnlyDictionary<string, InputAnnotateQuestion> Questions {
        get;
        init;
    }
    public InputPresence<InputAuthoredThreshold> Threshold {
        get;
        init;
    }
    internal new static InputRequestDefinitionAnnotateDefinition Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Batch = value.TryGetProperty("batch", out _) ? (InputPresence<JsonElement>)(value.GetProperty("batch").Clone()) : default,
        Profile = value.TryGetProperty("profile", out _) ? (InputPresence<InputAuthoredProfile>)(InputAuthoredProfile.Read(value.GetProperty("profile"))) : default,
        Questions = new System.Collections.ObjectModel.ReadOnlyDictionary<string, InputAnnotateQuestion>(value.GetProperty("questions").EnumerateObject().ToDictionary(entry0 => entry0.Name,
        entry0 => InputAnnotateQuestion.Read(entry0.Value))), Threshold = value.TryGetProperty("threshold",
        out _) ? (InputPresence<InputAuthoredThreshold>)(InputAuthoredThreshold.Read(value.GetProperty("threshold"))) : default
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        if (Batch.IsPresent) {
            writer.WritePropertyName("batch");
            Batch.Value.WriteTo(writer);
        } if (Profile.IsPresent) {
            writer.WritePropertyName("profile");
            if (Profile.Value is null) writer.WriteNullValue();
            else Profile.Value.Write(writer);
        } writer.WritePropertyName("questions");
        writer.WriteStartObject();
        foreach (var entry in Questions) {
            WriteName(writer, entry.Key);
            if (entry.Value is null) writer.WriteNullValue();
            else entry.Value.Write(writer);
        } writer.WriteEndObject();
        if (Threshold.IsPresent) {
            writer.WritePropertyName("threshold");
            if (Threshold.Value is null) writer.WriteNullValue();
            else Threshold.Value.Write(writer);
        } writer.WritePropertyName("version");
        writer.WriteNumberValue(1);
        writer.WriteEndObject();
    }
}
public sealed class InputRequestDefinitionAuthoredChoose : InputRequestDefinition {
    public InputPresence<InputRequestDefinitionAuthoredChooseBatch> Batch {
        get;
        init;
    }
    public required InputAuthoredQuestionText Choose {
        get;
        init;
    }
    public InputPresence<InputAuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public InputPresence<InputAuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public InputPresence<InputAuthoredName> Model {
        get;
        init;
    }
    public InputPresence<string> Name {
        get;
        init;
    }
    public InputPresence<InputAuthoredPointers> On {
        get;
        init;
    }
    public InputPresence<InputAuthoredOptions> Options {
        get;
        init;
    }
    public InputPresence<InputAuthoredProfile> Profile {
        get;
        init;
    }
    public InputPresence<InputAuthoredCut> Threshold {
        get;
        init;
    }
    public InputPresence<long> WordingVersion {
        get;
        init;
    }
    internal new static InputRequestDefinitionAuthoredChoose Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Batch = value.TryGetProperty("batch", out _) ? (InputPresence<InputRequestDefinitionAuthoredChooseBatch>)(InputRequestDefinitionAuthoredChooseBatch.Read(value.GetProperty("batch"))) : default,
        Choose = InputAuthoredQuestionText.Read(value.GetProperty("choose")), ContextSchema = value.TryGetProperty("context_schema",
        out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("context_schema"))) : default,
        ItemSchema = value.TryGetProperty("item_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("item_schema"))) : default,
        Model = value.TryGetProperty("model", out _) ? (InputPresence<InputAuthoredName>)(InputAuthoredName.Read(value.GetProperty("model"))) : default,
        Name = value.TryGetProperty("name", out _) ? (InputPresence<string>)(value.GetProperty("name").GetString()!) : default,
        On = value.TryGetProperty("on", out _) ? (InputPresence<InputAuthoredPointers>)(InputAuthoredPointers.Read(value.GetProperty("on"))) : default,
        Options = value.TryGetProperty("options", out _) ? (InputPresence<InputAuthoredOptions>)(InputAuthoredOptions.Read(value.GetProperty("options"))) : default,
        Profile = value.TryGetProperty("profile", out _) ? (InputPresence<InputAuthoredProfile>)(InputAuthoredProfile.Read(value.GetProperty("profile"))) : default,
        Threshold = value.TryGetProperty("threshold", out _) ? (InputPresence<InputAuthoredCut>)(InputAuthoredCut.Read(value.GetProperty("threshold"))) : default,
        WordingVersion = value.TryGetProperty("wording_version", out _) ? (InputPresence<long>)(value.GetProperty("wording_version").GetInt64()) : default
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        if (Batch.IsPresent) {
            writer.WritePropertyName("batch");
            if (Batch.Value is null) writer.WriteNullValue();
            else Batch.Value.Write(writer);
        } writer.WritePropertyName("choose");
        if (Choose is null) writer.WriteNullValue();
        else Choose.Write(writer);
        if (ContextSchema.IsPresent) {
            writer.WritePropertyName("context_schema");
            if (ContextSchema.Value is null) writer.WriteNullValue();
            else ContextSchema.Value.Write(writer);
        } if (ItemSchema.IsPresent) {
            writer.WritePropertyName("item_schema");
            if (ItemSchema.Value is null) writer.WriteNullValue();
            else ItemSchema.Value.Write(writer);
        } if (Model.IsPresent) {
            writer.WritePropertyName("model");
            if (Model.Value is null) writer.WriteNullValue();
            else Model.Value.Write(writer);
        } if (Name.IsPresent) {
            writer.WritePropertyName("name");
            WriteText(writer, Name.Value);
        } if (On.IsPresent) {
            writer.WritePropertyName("on");
            if (On.Value is null) writer.WriteNullValue();
            else On.Value.Write(writer);
        } if (Options.IsPresent) {
            writer.WritePropertyName("options");
            if (Options.Value is null) writer.WriteNullValue();
            else Options.Value.Write(writer);
        } if (Profile.IsPresent) {
            writer.WritePropertyName("profile");
            if (Profile.Value is null) writer.WriteNullValue();
            else Profile.Value.Write(writer);
        } if (Threshold.IsPresent) {
            writer.WritePropertyName("threshold");
            if (Threshold.Value is null) writer.WriteNullValue();
            else Threshold.Value.Write(writer);
        } if (WordingVersion.IsPresent) {
            writer.WritePropertyName("wording_version");
            writer.WriteNumberValue(WordingVersion.Value);
        } writer.WriteEndObject();
    }
}
public abstract class InputRequestDefinitionAuthoredChooseBatch : InputDocument {
    private protected InputRequestDefinitionAuthoredChooseBatch() {
    } internal static InputRequestDefinitionAuthoredChooseBatch Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.String && value.GetString() == "max")) return InputRequestDefinitionAuthoredChooseBatchMax.Read(value);
        if ((value.ValueKind == JsonValueKind.Number)) return InputRequestDefinitionAuthoredChooseBatchInteger.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class InputRequestDefinitionAuthoredChooseBatchInteger : InputRequestDefinitionAuthoredChooseBatch {
    public required long Value {
        get;
        init;
    }
    internal new static InputRequestDefinitionAuthoredChooseBatchInteger Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.GetInt64()
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNumberValue(Value);
    }
}
public sealed class InputRequestDefinitionAuthoredChooseBatchMax : InputRequestDefinitionAuthoredChooseBatch {
    internal new static InputRequestDefinitionAuthoredChooseBatchMax Read(JsonElement value) => new() {
        ParsedDocument = value.Clone()
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "max");
    }
}
public sealed class InputRequestDefinitionAuthoredDecide : InputRequestDefinition {
    public InputPresence<InputRequestDefinitionAuthoredDecideBatch> Batch {
        get;
        init;
    }
    public InputPresence<InputAuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public required InputAuthoredQuestionText Decide {
        get;
        init;
    }
    public InputPresence<InputAuthoredCriterion> False {
        get;
        init;
    }
    public InputPresence<InputAuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public InputPresence<InputAuthoredName> Model {
        get;
        init;
    }
    public InputPresence<string> Name {
        get;
        init;
    }
    public InputPresence<InputAuthoredPointers> On {
        get;
        init;
    }
    public InputPresence<InputAuthoredProfile> Profile {
        get;
        init;
    }
    public InputPresence<InputAuthoredThreshold> Threshold {
        get;
        init;
    }
    public InputPresence<InputAuthoredCriterion> True {
        get;
        init;
    }
    public InputPresence<long> WordingVersion {
        get;
        init;
    }
    internal new static InputRequestDefinitionAuthoredDecide Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Batch = value.TryGetProperty("batch", out _) ? (InputPresence<InputRequestDefinitionAuthoredDecideBatch>)(InputRequestDefinitionAuthoredDecideBatch.Read(value.GetProperty("batch"))) : default,
        ContextSchema = value.TryGetProperty("context_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("context_schema"))) : default,
        Decide = InputAuthoredQuestionText.Read(value.GetProperty("decide")), False = value.TryGetProperty("false",
        out _) ? (InputPresence<InputAuthoredCriterion>)(InputAuthoredCriterion.Read(value.GetProperty("false"))) : default,
        ItemSchema = value.TryGetProperty("item_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("item_schema"))) : default,
        Model = value.TryGetProperty("model", out _) ? (InputPresence<InputAuthoredName>)(InputAuthoredName.Read(value.GetProperty("model"))) : default,
        Name = value.TryGetProperty("name", out _) ? (InputPresence<string>)(value.GetProperty("name").GetString()!) : default,
        On = value.TryGetProperty("on", out _) ? (InputPresence<InputAuthoredPointers>)(InputAuthoredPointers.Read(value.GetProperty("on"))) : default,
        Profile = value.TryGetProperty("profile", out _) ? (InputPresence<InputAuthoredProfile>)(InputAuthoredProfile.Read(value.GetProperty("profile"))) : default,
        Threshold = value.TryGetProperty("threshold", out _) ? (InputPresence<InputAuthoredThreshold>)(InputAuthoredThreshold.Read(value.GetProperty("threshold"))) : default,
        True = value.TryGetProperty("true", out _) ? (InputPresence<InputAuthoredCriterion>)(InputAuthoredCriterion.Read(value.GetProperty("true"))) : default,
        WordingVersion = value.TryGetProperty("wording_version", out _) ? (InputPresence<long>)(value.GetProperty("wording_version").GetInt64()) : default
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        if (Batch.IsPresent) {
            writer.WritePropertyName("batch");
            if (Batch.Value is null) writer.WriteNullValue();
            else Batch.Value.Write(writer);
        } if (ContextSchema.IsPresent) {
            writer.WritePropertyName("context_schema");
            if (ContextSchema.Value is null) writer.WriteNullValue();
            else ContextSchema.Value.Write(writer);
        } writer.WritePropertyName("decide");
        if (Decide is null) writer.WriteNullValue();
        else Decide.Write(writer);
        if (False.IsPresent) {
            writer.WritePropertyName("false");
            if (False.Value is null) writer.WriteNullValue();
            else False.Value.Write(writer);
        } if (ItemSchema.IsPresent) {
            writer.WritePropertyName("item_schema");
            if (ItemSchema.Value is null) writer.WriteNullValue();
            else ItemSchema.Value.Write(writer);
        } if (Model.IsPresent) {
            writer.WritePropertyName("model");
            if (Model.Value is null) writer.WriteNullValue();
            else Model.Value.Write(writer);
        } if (Name.IsPresent) {
            writer.WritePropertyName("name");
            WriteText(writer, Name.Value);
        } if (On.IsPresent) {
            writer.WritePropertyName("on");
            if (On.Value is null) writer.WriteNullValue();
            else On.Value.Write(writer);
        } if (Profile.IsPresent) {
            writer.WritePropertyName("profile");
            if (Profile.Value is null) writer.WriteNullValue();
            else Profile.Value.Write(writer);
        } if (Threshold.IsPresent) {
            writer.WritePropertyName("threshold");
            if (Threshold.Value is null) writer.WriteNullValue();
            else Threshold.Value.Write(writer);
        } if (True.IsPresent) {
            writer.WritePropertyName("true");
            if (True.Value is null) writer.WriteNullValue();
            else True.Value.Write(writer);
        } if (WordingVersion.IsPresent) {
            writer.WritePropertyName("wording_version");
            writer.WriteNumberValue(WordingVersion.Value);
        } writer.WriteEndObject();
    }
}
public abstract class InputRequestDefinitionAuthoredDecideBatch : InputDocument {
    private protected InputRequestDefinitionAuthoredDecideBatch() {
    } internal static InputRequestDefinitionAuthoredDecideBatch Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.String && value.GetString() == "max")) return InputRequestDefinitionAuthoredDecideBatchMax.Read(value);
        if ((value.ValueKind == JsonValueKind.Number)) return InputRequestDefinitionAuthoredDecideBatchInteger.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class InputRequestDefinitionAuthoredDecideBatchInteger : InputRequestDefinitionAuthoredDecideBatch {
    public required long Value {
        get;
        init;
    }
    internal new static InputRequestDefinitionAuthoredDecideBatchInteger Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.GetInt64()
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNumberValue(Value);
    }
}
public sealed class InputRequestDefinitionAuthoredDecideBatchMax : InputRequestDefinitionAuthoredDecideBatch {
    internal new static InputRequestDefinitionAuthoredDecideBatchMax Read(JsonElement value) => new() {
        ParsedDocument = value.Clone()
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "max");
    }
}
public sealed class InputRequestDefinitionAuthoredFind : InputRequestDefinition {
    public InputPresence<InputAuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public required InputAuthoredQuestionText Find {
        get;
        init;
    }
    public InputPresence<InputAuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public InputPresence<InputAuthoredName> Model {
        get;
        init;
    }
    public InputPresence<string> Name {
        get;
        init;
    }
    public InputPresence<InputAuthoredPointers> On {
        get;
        init;
    }
    public InputPresence<InputAuthoredProfile> Profile {
        get;
        init;
    }
    public InputPresence<long> WordingVersion {
        get;
        init;
    }
    internal new static InputRequestDefinitionAuthoredFind Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), ContextSchema = value.TryGetProperty("context_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("context_schema"))) : default,
        Find = InputAuthoredQuestionText.Read(value.GetProperty("find")), ItemSchema = value.TryGetProperty("item_schema",
        out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("item_schema"))) : default,
        Model = value.TryGetProperty("model", out _) ? (InputPresence<InputAuthoredName>)(InputAuthoredName.Read(value.GetProperty("model"))) : default,
        Name = value.TryGetProperty("name", out _) ? (InputPresence<string>)(value.GetProperty("name").GetString()!) : default,
        On = value.TryGetProperty("on", out _) ? (InputPresence<InputAuthoredPointers>)(InputAuthoredPointers.Read(value.GetProperty("on"))) : default,
        Profile = value.TryGetProperty("profile", out _) ? (InputPresence<InputAuthoredProfile>)(InputAuthoredProfile.Read(value.GetProperty("profile"))) : default,
        WordingVersion = value.TryGetProperty("wording_version", out _) ? (InputPresence<long>)(value.GetProperty("wording_version").GetInt64()) : default
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        if (ContextSchema.IsPresent) {
            writer.WritePropertyName("context_schema");
            if (ContextSchema.Value is null) writer.WriteNullValue();
            else ContextSchema.Value.Write(writer);
        } writer.WritePropertyName("find");
        if (Find is null) writer.WriteNullValue();
        else Find.Write(writer);
        if (ItemSchema.IsPresent) {
            writer.WritePropertyName("item_schema");
            if (ItemSchema.Value is null) writer.WriteNullValue();
            else ItemSchema.Value.Write(writer);
        } if (Model.IsPresent) {
            writer.WritePropertyName("model");
            if (Model.Value is null) writer.WriteNullValue();
            else Model.Value.Write(writer);
        } if (Name.IsPresent) {
            writer.WritePropertyName("name");
            WriteText(writer, Name.Value);
        } if (On.IsPresent) {
            writer.WritePropertyName("on");
            if (On.Value is null) writer.WriteNullValue();
            else On.Value.Write(writer);
        } if (Profile.IsPresent) {
            writer.WritePropertyName("profile");
            if (Profile.Value is null) writer.WriteNullValue();
            else Profile.Value.Write(writer);
        } if (WordingVersion.IsPresent) {
            writer.WritePropertyName("wording_version");
            writer.WriteNumberValue(WordingVersion.Value);
        } writer.WriteEndObject();
    }
}
public sealed class InputRequestDefinitionAuthoredRelate : InputRequestDefinition {
    public InputPresence<InputAuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public InputPresence<InputAuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public InputPresence<InputAuthoredName> Model {
        get;
        init;
    }
    public InputPresence<string> Name {
        get;
        init;
    }
    public InputPresence<InputAuthoredProfile> Profile {
        get;
        init;
    }
    public required InputRelateQuestion Relate {
        get;
        init;
    }
    public InputPresence<InputAuthoredCut> Threshold {
        get;
        init;
    }
    public InputPresence<long> WordingVersion {
        get;
        init;
    }
    internal new static InputRequestDefinitionAuthoredRelate Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), ContextSchema = value.TryGetProperty("context_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("context_schema"))) : default,
        ItemSchema = value.TryGetProperty("item_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("item_schema"))) : default,
        Model = value.TryGetProperty("model", out _) ? (InputPresence<InputAuthoredName>)(InputAuthoredName.Read(value.GetProperty("model"))) : default,
        Name = value.TryGetProperty("name", out _) ? (InputPresence<string>)(value.GetProperty("name").GetString()!) : default,
        Profile = value.TryGetProperty("profile", out _) ? (InputPresence<InputAuthoredProfile>)(InputAuthoredProfile.Read(value.GetProperty("profile"))) : default,
        Relate = InputRelateQuestion.Read(value.GetProperty("relate")), Threshold = value.TryGetProperty("threshold",
        out _) ? (InputPresence<InputAuthoredCut>)(InputAuthoredCut.Read(value.GetProperty("threshold"))) : default,
        WordingVersion = value.TryGetProperty("wording_version", out _) ? (InputPresence<long>)(value.GetProperty("wording_version").GetInt64()) : default
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        if (ContextSchema.IsPresent) {
            writer.WritePropertyName("context_schema");
            if (ContextSchema.Value is null) writer.WriteNullValue();
            else ContextSchema.Value.Write(writer);
        } if (ItemSchema.IsPresent) {
            writer.WritePropertyName("item_schema");
            if (ItemSchema.Value is null) writer.WriteNullValue();
            else ItemSchema.Value.Write(writer);
        } if (Model.IsPresent) {
            writer.WritePropertyName("model");
            if (Model.Value is null) writer.WriteNullValue();
            else Model.Value.Write(writer);
        } if (Name.IsPresent) {
            writer.WritePropertyName("name");
            WriteText(writer, Name.Value);
        } if (Profile.IsPresent) {
            writer.WritePropertyName("profile");
            if (Profile.Value is null) writer.WriteNullValue();
            else Profile.Value.Write(writer);
        } writer.WritePropertyName("relate");
        if (Relate is null) writer.WriteNullValue();
        else Relate.Write(writer);
        if (Threshold.IsPresent) {
            writer.WritePropertyName("threshold");
            if (Threshold.Value is null) writer.WriteNullValue();
            else Threshold.Value.Write(writer);
        } writer.WritePropertyName("version");
        writer.WriteNumberValue(1);
        if (WordingVersion.IsPresent) {
            writer.WritePropertyName("wording_version");
            writer.WriteNumberValue(WordingVersion.Value);
        } writer.WriteEndObject();
    }
}
public sealed class InputRequestDefinitionAuthoredScore : InputRequestDefinition {
    public InputPresence<InputRequestDefinitionAuthoredScoreBatch> Batch {
        get;
        init;
    }
    public InputPresence<InputAuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public InputPresence<InputAuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public InputPresence<InputAuthoredLevels> Levels {
        get;
        init;
    }
    public InputPresence<InputAuthoredName> Model {
        get;
        init;
    }
    public InputPresence<string> Name {
        get;
        init;
    }
    public InputPresence<InputAuthoredPointers> On {
        get;
        init;
    }
    public InputPresence<InputAuthoredProfile> Profile {
        get;
        init;
    }
    public required InputAuthoredQuestionText Score {
        get;
        init;
    }
    public InputPresence<long> WordingVersion {
        get;
        init;
    }
    internal new static InputRequestDefinitionAuthoredScore Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Batch = value.TryGetProperty("batch", out _) ? (InputPresence<InputRequestDefinitionAuthoredScoreBatch>)(InputRequestDefinitionAuthoredScoreBatch.Read(value.GetProperty("batch"))) : default,
        ContextSchema = value.TryGetProperty("context_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("context_schema"))) : default,
        ItemSchema = value.TryGetProperty("item_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("item_schema"))) : default,
        Levels = value.TryGetProperty("levels", out _) ? (InputPresence<InputAuthoredLevels>)(InputAuthoredLevels.Read(value.GetProperty("levels"))) : default,
        Model = value.TryGetProperty("model", out _) ? (InputPresence<InputAuthoredName>)(InputAuthoredName.Read(value.GetProperty("model"))) : default,
        Name = value.TryGetProperty("name", out _) ? (InputPresence<string>)(value.GetProperty("name").GetString()!) : default,
        On = value.TryGetProperty("on", out _) ? (InputPresence<InputAuthoredPointers>)(InputAuthoredPointers.Read(value.GetProperty("on"))) : default,
        Profile = value.TryGetProperty("profile", out _) ? (InputPresence<InputAuthoredProfile>)(InputAuthoredProfile.Read(value.GetProperty("profile"))) : default,
        Score = InputAuthoredQuestionText.Read(value.GetProperty("score")), WordingVersion = value.TryGetProperty("wording_version",
        out _) ? (InputPresence<long>)(value.GetProperty("wording_version").GetInt64()) : default
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        if (Batch.IsPresent) {
            writer.WritePropertyName("batch");
            if (Batch.Value is null) writer.WriteNullValue();
            else Batch.Value.Write(writer);
        } if (ContextSchema.IsPresent) {
            writer.WritePropertyName("context_schema");
            if (ContextSchema.Value is null) writer.WriteNullValue();
            else ContextSchema.Value.Write(writer);
        } if (ItemSchema.IsPresent) {
            writer.WritePropertyName("item_schema");
            if (ItemSchema.Value is null) writer.WriteNullValue();
            else ItemSchema.Value.Write(writer);
        } if (Levels.IsPresent) {
            writer.WritePropertyName("levels");
            if (Levels.Value is null) writer.WriteNullValue();
            else Levels.Value.Write(writer);
        } if (Model.IsPresent) {
            writer.WritePropertyName("model");
            if (Model.Value is null) writer.WriteNullValue();
            else Model.Value.Write(writer);
        } if (Name.IsPresent) {
            writer.WritePropertyName("name");
            WriteText(writer, Name.Value);
        } if (On.IsPresent) {
            writer.WritePropertyName("on");
            if (On.Value is null) writer.WriteNullValue();
            else On.Value.Write(writer);
        } if (Profile.IsPresent) {
            writer.WritePropertyName("profile");
            if (Profile.Value is null) writer.WriteNullValue();
            else Profile.Value.Write(writer);
        } writer.WritePropertyName("score");
        if (Score is null) writer.WriteNullValue();
        else Score.Write(writer);
        if (WordingVersion.IsPresent) {
            writer.WritePropertyName("wording_version");
            writer.WriteNumberValue(WordingVersion.Value);
        } writer.WriteEndObject();
    }
}
public abstract class InputRequestDefinitionAuthoredScoreBatch : InputDocument {
    private protected InputRequestDefinitionAuthoredScoreBatch() {
    } internal static InputRequestDefinitionAuthoredScoreBatch Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.String && value.GetString() == "max")) return InputRequestDefinitionAuthoredScoreBatchMax.Read(value);
        if ((value.ValueKind == JsonValueKind.Number)) return InputRequestDefinitionAuthoredScoreBatchInteger.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class InputRequestDefinitionAuthoredScoreBatchInteger : InputRequestDefinitionAuthoredScoreBatch {
    public required long Value {
        get;
        init;
    }
    internal new static InputRequestDefinitionAuthoredScoreBatchInteger Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.GetInt64()
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNumberValue(Value);
    }
}
public sealed class InputRequestDefinitionAuthoredScoreBatchMax : InputRequestDefinitionAuthoredScoreBatch {
    internal new static InputRequestDefinitionAuthoredScoreBatchMax Read(JsonElement value) => new() {
        ParsedDocument = value.Clone()
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "max");
    }
}
public sealed class InputRequestDefinitionAuthoredTag : InputRequestDefinition {
    public InputPresence<InputRequestDefinitionAuthoredTagBatch> Batch {
        get;
        init;
    }
    public InputPresence<InputAuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public InputPresence<InputAuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public InputPresence<InputAuthoredLabels> Labels {
        get;
        init;
    }
    public InputPresence<InputAuthoredName> Model {
        get;
        init;
    }
    public InputPresence<string> Name {
        get;
        init;
    }
    public InputPresence<InputAuthoredPointers> On {
        get;
        init;
    }
    public InputPresence<InputAuthoredProfile> Profile {
        get;
        init;
    }
    public required InputAuthoredQuestionText Tag {
        get;
        init;
    }
    public InputPresence<InputAuthoredCut> Threshold {
        get;
        init;
    }
    public InputPresence<long> WordingVersion {
        get;
        init;
    }
    internal new static InputRequestDefinitionAuthoredTag Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Batch = value.TryGetProperty("batch", out _) ? (InputPresence<InputRequestDefinitionAuthoredTagBatch>)(InputRequestDefinitionAuthoredTagBatch.Read(value.GetProperty("batch"))) : default,
        ContextSchema = value.TryGetProperty("context_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("context_schema"))) : default,
        ItemSchema = value.TryGetProperty("item_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("item_schema"))) : default,
        Labels = value.TryGetProperty("labels", out _) ? (InputPresence<InputAuthoredLabels>)(InputAuthoredLabels.Read(value.GetProperty("labels"))) : default,
        Model = value.TryGetProperty("model", out _) ? (InputPresence<InputAuthoredName>)(InputAuthoredName.Read(value.GetProperty("model"))) : default,
        Name = value.TryGetProperty("name", out _) ? (InputPresence<string>)(value.GetProperty("name").GetString()!) : default,
        On = value.TryGetProperty("on", out _) ? (InputPresence<InputAuthoredPointers>)(InputAuthoredPointers.Read(value.GetProperty("on"))) : default,
        Profile = value.TryGetProperty("profile", out _) ? (InputPresence<InputAuthoredProfile>)(InputAuthoredProfile.Read(value.GetProperty("profile"))) : default,
        Tag = InputAuthoredQuestionText.Read(value.GetProperty("tag")), Threshold = value.TryGetProperty("threshold",
        out _) ? (InputPresence<InputAuthoredCut>)(InputAuthoredCut.Read(value.GetProperty("threshold"))) : default,
        WordingVersion = value.TryGetProperty("wording_version", out _) ? (InputPresence<long>)(value.GetProperty("wording_version").GetInt64()) : default
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        if (Batch.IsPresent) {
            writer.WritePropertyName("batch");
            if (Batch.Value is null) writer.WriteNullValue();
            else Batch.Value.Write(writer);
        } if (ContextSchema.IsPresent) {
            writer.WritePropertyName("context_schema");
            if (ContextSchema.Value is null) writer.WriteNullValue();
            else ContextSchema.Value.Write(writer);
        } if (ItemSchema.IsPresent) {
            writer.WritePropertyName("item_schema");
            if (ItemSchema.Value is null) writer.WriteNullValue();
            else ItemSchema.Value.Write(writer);
        } if (Labels.IsPresent) {
            writer.WritePropertyName("labels");
            if (Labels.Value is null) writer.WriteNullValue();
            else Labels.Value.Write(writer);
        } if (Model.IsPresent) {
            writer.WritePropertyName("model");
            if (Model.Value is null) writer.WriteNullValue();
            else Model.Value.Write(writer);
        } if (Name.IsPresent) {
            writer.WritePropertyName("name");
            WriteText(writer, Name.Value);
        } if (On.IsPresent) {
            writer.WritePropertyName("on");
            if (On.Value is null) writer.WriteNullValue();
            else On.Value.Write(writer);
        } if (Profile.IsPresent) {
            writer.WritePropertyName("profile");
            if (Profile.Value is null) writer.WriteNullValue();
            else Profile.Value.Write(writer);
        } writer.WritePropertyName("tag");
        if (Tag is null) writer.WriteNullValue();
        else Tag.Write(writer);
        if (Threshold.IsPresent) {
            writer.WritePropertyName("threshold");
            if (Threshold.Value is null) writer.WriteNullValue();
            else Threshold.Value.Write(writer);
        } if (WordingVersion.IsPresent) {
            writer.WritePropertyName("wording_version");
            writer.WriteNumberValue(WordingVersion.Value);
        } writer.WriteEndObject();
    }
}
public abstract class InputRequestDefinitionAuthoredTagBatch : InputDocument {
    private protected InputRequestDefinitionAuthoredTagBatch() {
    } internal static InputRequestDefinitionAuthoredTagBatch Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.String && value.GetString() == "max")) return InputRequestDefinitionAuthoredTagBatchMax.Read(value);
        if ((value.ValueKind == JsonValueKind.Number)) return InputRequestDefinitionAuthoredTagBatchInteger.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class InputRequestDefinitionAuthoredTagBatchInteger : InputRequestDefinitionAuthoredTagBatch {
    public required long Value {
        get;
        init;
    }
    internal new static InputRequestDefinitionAuthoredTagBatchInteger Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.GetInt64()
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNumberValue(Value);
    }
}
public sealed class InputRequestDefinitionAuthoredTagBatchMax : InputRequestDefinitionAuthoredTagBatch {
    internal new static InputRequestDefinitionAuthoredTagBatchMax Read(JsonElement value) => new() {
        ParsedDocument = value.Clone()
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "max");
    }
}
public sealed class InputRequestDefinitionRecognizeDefinition : InputRequestDefinition {
    public InputPresence<InputAuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public InputPresence<InputAuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public InputPresence<InputAuthoredName> Model {
        get;
        init;
    }
    public InputPresence<string> Name {
        get;
        init;
    }
    public InputPresence<InputAuthoredPointers> On {
        get;
        init;
    }
    public InputPresence<InputAuthoredProfile> Profile {
        get;
        init;
    }
    public required InputRecognizeQuestion Recognize {
        get;
        init;
    }
    public InputPresence<InputAuthoredCut> RelationThreshold {
        get;
        init;
    }
    public InputPresence<InputAuthoredCut> Threshold {
        get;
        init;
    }
    public InputPresence<long> WordingVersion {
        get;
        init;
    }
    internal new static InputRequestDefinitionRecognizeDefinition Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), ContextSchema = value.TryGetProperty("context_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("context_schema"))) : default,
        ItemSchema = value.TryGetProperty("item_schema", out _) ? (InputPresence<InputAuthoredInputDeclaration>)(InputAuthoredInputDeclaration.Read(value.GetProperty("item_schema"))) : default,
        Model = value.TryGetProperty("model", out _) ? (InputPresence<InputAuthoredName>)(InputAuthoredName.Read(value.GetProperty("model"))) : default,
        Name = value.TryGetProperty("name", out _) ? (InputPresence<string>)(value.GetProperty("name").GetString()!) : default,
        On = value.TryGetProperty("on", out _) ? (InputPresence<InputAuthoredPointers>)(InputAuthoredPointers.Read(value.GetProperty("on"))) : default,
        Profile = value.TryGetProperty("profile", out _) ? (InputPresence<InputAuthoredProfile>)(InputAuthoredProfile.Read(value.GetProperty("profile"))) : default,
        Recognize = InputRecognizeQuestion.Read(value.GetProperty("recognize")), RelationThreshold = value.TryGetProperty("relation_threshold",
        out _) ? (InputPresence<InputAuthoredCut>)(InputAuthoredCut.Read(value.GetProperty("relation_threshold"))) : default,
        Threshold = value.TryGetProperty("threshold", out _) ? (InputPresence<InputAuthoredCut>)(InputAuthoredCut.Read(value.GetProperty("threshold"))) : default,
        WordingVersion = value.TryGetProperty("wording_version", out _) ? (InputPresence<long>)(value.GetProperty("wording_version").GetInt64()) : default
    };
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        if (ContextSchema.IsPresent) {
            writer.WritePropertyName("context_schema");
            if (ContextSchema.Value is null) writer.WriteNullValue();
            else ContextSchema.Value.Write(writer);
        } if (ItemSchema.IsPresent) {
            writer.WritePropertyName("item_schema");
            if (ItemSchema.Value is null) writer.WriteNullValue();
            else ItemSchema.Value.Write(writer);
        } if (Model.IsPresent) {
            writer.WritePropertyName("model");
            if (Model.Value is null) writer.WriteNullValue();
            else Model.Value.Write(writer);
        } if (Name.IsPresent) {
            writer.WritePropertyName("name");
            WriteText(writer, Name.Value);
        } if (On.IsPresent) {
            writer.WritePropertyName("on");
            if (On.Value is null) writer.WriteNullValue();
            else On.Value.Write(writer);
        } if (Profile.IsPresent) {
            writer.WritePropertyName("profile");
            if (Profile.Value is null) writer.WriteNullValue();
            else Profile.Value.Write(writer);
        } writer.WritePropertyName("recognize");
        if (Recognize is null) writer.WriteNullValue();
        else Recognize.Write(writer);
        if (RelationThreshold.IsPresent) {
            writer.WritePropertyName("relation_threshold");
            if (RelationThreshold.Value is null) writer.WriteNullValue();
            else RelationThreshold.Value.Write(writer);
        } if (Threshold.IsPresent) {
            writer.WritePropertyName("threshold");
            if (Threshold.Value is null) writer.WriteNullValue();
            else Threshold.Value.Write(writer);
        } writer.WritePropertyName("version");
        writer.WriteNumberValue(1);
        if (WordingVersion.IsPresent) {
            writer.WritePropertyName("wording_version");
            writer.WriteNumberValue(WordingVersion.Value);
        } writer.WriteEndObject();
    }
}
public abstract class InputRequestFraming : InputDocument {
    private protected InputRequestFraming() {
    }
}
public sealed class InputRequestFramingCsv : InputRequestFraming {
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "csv");
    }
}
public sealed class InputRequestFramingDocument : InputRequestFraming {
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "document");
    }
}
public sealed class InputRequestFramingJsonl : InputRequestFraming {
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "jsonl");
    }
}
public sealed class InputRequestFramingLines : InputRequestFraming {
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "lines");
    }
}
public sealed class InputRequestFramingTsv : InputRequestFraming {
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "tsv");
    }
}
public abstract class InputRequestFunction : InputDocument {
    private protected InputRequestFunction() {
    }
}
public sealed class InputRequestFunctionAnnotate : InputRequestFunction {
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "annotate");
    }
}
public sealed class InputRequestFunctionChoose : InputRequestFunction {
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "choose");
    }
}
public sealed class InputRequestFunctionDecide : InputRequestFunction {
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "decide");
    }
}
public sealed class InputRequestFunctionFilter : InputRequestFunction {
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "filter");
    }
}
public sealed class InputRequestFunctionFind : InputRequestFunction {
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "find");
    }
}
public sealed class InputRequestFunctionRank : InputRequestFunction {
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "rank");
    }
}
public sealed class InputRequestFunctionRecognize : InputRequestFunction {
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "recognize");
    }
}
public sealed class InputRequestFunctionRelate : InputRequestFunction {
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "relate");
    }
}
public sealed class InputRequestFunctionScore : InputRequestFunction {
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "score");
    }
}
public sealed class InputRequestFunctionTag : InputRequestFunction {
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "tag");
    }
}
public abstract class InputRequestImage : InputDocument {
    private protected InputRequestImage() {
    }
}
public sealed class InputRequestImageBytes : InputRequestImage {
    public required string Bytes {
        get;
        init;
    }
    public required InputImageMedia Media {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        writer.WritePropertyName("bytes");
        WriteText(writer, Bytes);
        writer.WritePropertyName("kind");
        WriteText(writer, "bytes");
        writer.WritePropertyName("media");
        if (Media is null) writer.WriteNullValue();
        else Media.Write(writer);
        writer.WriteEndObject();
    }
}
public sealed class InputRequestImageFile : InputRequestImage {
    public InputPresence<InputImageMedia> Media {
        get;
        init;
    }
    public required string Path {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        writer.WritePropertyName("kind");
        WriteText(writer, "file");
        if (Media.IsPresent) {
            writer.WritePropertyName("media");
            if (Media.Value is null) writer.WriteNullValue();
            else Media.Value.Write(writer);
        } writer.WritePropertyName("path");
        WriteText(writer, Path);
        writer.WriteEndObject();
    }
}
public abstract class InputRequestInput : InputDocument {
    private protected InputRequestInput() {
    }
}
public sealed class InputRequestInputEntities : InputRequestInput {
    public required IReadOnlyList<InputRequestItem> Items {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        writer.WritePropertyName("items");
        writer.WriteStartArray();
        foreach (var item in Items) {
            if (item is null) writer.WriteNullValue();
            else item.Write(writer);
        } writer.WriteEndArray();
        writer.WritePropertyName("kind");
        WriteText(writer, "entities");
        writer.WriteEndObject();
    }
}
public sealed class InputRequestInputFeed : InputRequestInput {
    public InputPresence<InputRequestFraming> Framing {
        get;
        init;
    }
    public InputPresence<IReadOnlyList<InputRequestImage>> Images {
        get;
        init;
    }
    public required string Name {
        get;
        init;
    }
    public InputPresence<InputRequestReader> Reading {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        if (Framing.IsPresent) {
            writer.WritePropertyName("framing");
            if (Framing.Value is null) writer.WriteNullValue();
            else Framing.Value.Write(writer);
        } if (Images.IsPresent) {
            writer.WritePropertyName("images");
            writer.WriteStartArray();
            foreach (var item in Images.Value) {
                if (item is null) writer.WriteNullValue();
                else item.Write(writer);
            } writer.WriteEndArray();
        } writer.WritePropertyName("kind");
        WriteText(writer, "feed");
        writer.WritePropertyName("name");
        WriteText(writer, Name);
        if (Reading.IsPresent) {
            writer.WritePropertyName("reading");
            if (Reading.Value is null) writer.WriteNullValue();
            else Reading.Value.Write(writer);
        } writer.WriteEndObject();
    }
}
public sealed class InputRequestInputJson : InputRequestInput {
    public InputPresence<IReadOnlyList<InputRequestImage>> Images {
        get;
        init;
    }
    public required JsonElement Value {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        if (Images.IsPresent) {
            writer.WritePropertyName("images");
            writer.WriteStartArray();
            foreach (var item in Images.Value) {
                if (item is null) writer.WriteNullValue();
                else item.Write(writer);
            } writer.WriteEndArray();
        } writer.WritePropertyName("kind");
        WriteText(writer, "json");
        writer.WritePropertyName("value");
        Value.WriteTo(writer);
        writer.WriteEndObject();
    }
}
public sealed class InputRequestInputRecords : InputRequestInput {
    public required IReadOnlyList<InputRequestItem> Items {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        writer.WritePropertyName("items");
        writer.WriteStartArray();
        foreach (var item in Items) {
            if (item is null) writer.WriteNullValue();
            else item.Write(writer);
        } writer.WriteEndArray();
        writer.WritePropertyName("kind");
        WriteText(writer, "records");
        writer.WriteEndObject();
    }
}
public sealed class InputRequestInputSource : InputRequestInput {
    public required InputRequestSource Source {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        writer.WritePropertyName("kind");
        WriteText(writer, "source");
        writer.WritePropertyName("source");
        if (Source is null) writer.WriteNullValue();
        else Source.Write(writer);
        writer.WriteEndObject();
    }
}
public sealed class InputRequestInputText : InputRequestInput {
    public InputPresence<IReadOnlyList<InputRequestImage>> Images {
        get;
        init;
    }
    public required string Text {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        if (Images.IsPresent) {
            writer.WritePropertyName("images");
            writer.WriteStartArray();
            foreach (var item in Images.Value) {
                if (item is null) writer.WriteNullValue();
                else item.Write(writer);
            } writer.WriteEndArray();
        } writer.WritePropertyName("kind");
        WriteText(writer, "text");
        writer.WritePropertyName("text");
        WriteText(writer, Text);
        writer.WriteEndObject();
    }
}
public sealed class InputRequestInputUnits : InputRequestInput {
    public required IReadOnlyList<InputRequestItem> Items {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        writer.WritePropertyName("items");
        writer.WriteStartArray();
        foreach (var item in Items) {
            if (item is null) writer.WriteNullValue();
            else item.Write(writer);
        } writer.WriteEndArray();
        writer.WritePropertyName("kind");
        WriteText(writer, "units");
        writer.WriteEndObject();
    }
}
public sealed class InputRequestItem : InputDocument {
    public InputPresence<InputContextSchema> Context {
        get;
        init;
    }
    public InputPresence<IReadOnlyList<InputRecognitionExample>> Examples {
        get;
        init;
    }
    public InputPresence<IReadOnlyList<InputRequestImage>> Images {
        get;
        init;
    }
    public InputPresence<IReadOnlyList<InputOptionSchema>> Options {
        get;
        init;
    }
    public InputPresence<InputRequestOriginal> Original {
        get;
        init;
    }
    public InputPresence<IReadOnlyList<InputRecognitionSeedSpan>> SeedSpans {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        if (Context.IsPresent) {
            writer.WritePropertyName("context");
            if (Context.Value is null) writer.WriteNullValue();
            else Context.Value.Write(writer);
        } if (Examples.IsPresent) {
            writer.WritePropertyName("examples");
            writer.WriteStartArray();
            foreach (var item in Examples.Value) {
                if (item is null) writer.WriteNullValue();
                else item.Write(writer);
            } writer.WriteEndArray();
        } if (Images.IsPresent) {
            writer.WritePropertyName("images");
            writer.WriteStartArray();
            foreach (var item in Images.Value) {
                if (item is null) writer.WriteNullValue();
                else item.Write(writer);
            } writer.WriteEndArray();
        } if (Options.IsPresent) {
            writer.WritePropertyName("options");
            writer.WriteStartArray();
            foreach (var item in Options.Value) {
                if (item is null) writer.WriteNullValue();
                else item.Write(writer);
            } writer.WriteEndArray();
        } if (Original.IsPresent) {
            writer.WritePropertyName("original");
            if (Original.Value is null) writer.WriteNullValue();
            else Original.Value.Write(writer);
        } if (SeedSpans.IsPresent) {
            writer.WritePropertyName("seed_spans");
            writer.WriteStartArray();
            foreach (var item in SeedSpans.Value) {
                if (item is null) writer.WriteNullValue();
                else item.Write(writer);
            } writer.WriteEndArray();
        } writer.WriteEndObject();
    }
}
public sealed class InputRequestOptions : InputDocument {
    public InputPresence<bool> Attempts {
        get;
        init;
    }
    public InputPresence<InputRequestBatch> Batch {
        get;
        init;
    }
    public InputPresence<string> Context {
        get;
        init;
    }
    public InputPresence<string> ContextField {
        get;
        init;
    }
    public InputPresence<long> DeadlineMs {
        get;
        init;
    }
    public InputPresence<bool> Details {
        get;
        init;
    }
    public InputPresence<IReadOnlyList<InputRecognitionExample>> Examples {
        get;
        init;
    }
    public InputPresence<string> ExamplesField {
        get;
        init;
    }
    public InputPresence<IReadOnlyList<string>> Field {
        get;
        init;
    }
    public InputPresence<bool> FilesOnly {
        get;
        init;
    }
    public InputPresence<ulong> MaxRequestsTotal {
        get;
        init;
    }
    public InputPresence<InputRecognitionMode> Mode {
        get;
        init;
    }
    public InputPresence<string> Model {
        get;
        init;
    }
    public InputPresence<bool> None {
        get;
        init;
    }
    public InputPresence<string> OptionsField {
        get;
        init;
    }
    public InputPresence<InputRequestThreshold> RelationThreshold {
        get;
        init;
    }
    public InputPresence<IReadOnlyList<InputRecognitionSeedSpan>> SeedSpans {
        get;
        init;
    }
    public InputPresence<string> SeedSpansField {
        get;
        init;
    }
    public InputPresence<uint> SnippetPieces {
        get;
        init;
    }
    public InputPresence<InputRecognitionStageContext> StageContext {
        get;
        init;
    }
    public InputPresence<InputRequestThreshold> Threshold {
        get;
        init;
    }
    public InputPresence<ulong> Top {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        if (Attempts.IsPresent) {
            writer.WritePropertyName("attempts");
            writer.WriteBooleanValue(Attempts.Value);
        } if (Batch.IsPresent) {
            writer.WritePropertyName("batch");
            if (Batch.Value is null) writer.WriteNullValue();
            else Batch.Value.Write(writer);
        } if (Context.IsPresent) {
            writer.WritePropertyName("context");
            WriteText(writer, Context.Value);
        } if (ContextField.IsPresent) {
            writer.WritePropertyName("context_field");
            WriteText(writer, ContextField.Value);
        } if (DeadlineMs.IsPresent) {
            writer.WritePropertyName("deadline_ms");
            writer.WriteNumberValue(DeadlineMs.Value);
        } if (Details.IsPresent) {
            writer.WritePropertyName("details");
            writer.WriteBooleanValue(Details.Value);
        } if (Examples.IsPresent) {
            writer.WritePropertyName("examples");
            writer.WriteStartArray();
            foreach (var item in Examples.Value) {
                if (item is null) writer.WriteNullValue();
                else item.Write(writer);
            } writer.WriteEndArray();
        } if (ExamplesField.IsPresent) {
            writer.WritePropertyName("examples_field");
            WriteText(writer, ExamplesField.Value);
        } if (Field.IsPresent) {
            writer.WritePropertyName("field");
            writer.WriteStartArray();
            foreach (var item in Field.Value) {
                WriteText(writer, item);
            } writer.WriteEndArray();
        } if (FilesOnly.IsPresent) {
            writer.WritePropertyName("files_only");
            writer.WriteBooleanValue(FilesOnly.Value);
        } if (MaxRequestsTotal.IsPresent) {
            writer.WritePropertyName("max_requests_total");
            writer.WriteNumberValue(MaxRequestsTotal.Value);
        } if (Mode.IsPresent) {
            writer.WritePropertyName("mode");
            if (Mode.Value is null) writer.WriteNullValue();
            else Mode.Value.Write(writer);
        } if (Model.IsPresent) {
            writer.WritePropertyName("model");
            WriteText(writer, Model.Value);
        } if (None.IsPresent) {
            writer.WritePropertyName("none");
            writer.WriteBooleanValue(None.Value);
        } if (OptionsField.IsPresent) {
            writer.WritePropertyName("options_field");
            WriteText(writer, OptionsField.Value);
        } if (RelationThreshold.IsPresent) {
            writer.WritePropertyName("relation_threshold");
            if (RelationThreshold.Value is null) writer.WriteNullValue();
            else RelationThreshold.Value.Write(writer);
        } if (SeedSpans.IsPresent) {
            writer.WritePropertyName("seed_spans");
            writer.WriteStartArray();
            foreach (var item in SeedSpans.Value) {
                if (item is null) writer.WriteNullValue();
                else item.Write(writer);
            } writer.WriteEndArray();
        } if (SeedSpansField.IsPresent) {
            writer.WritePropertyName("seed_spans_field");
            WriteText(writer, SeedSpansField.Value);
        } if (SnippetPieces.IsPresent) {
            writer.WritePropertyName("snippet_pieces");
            writer.WriteNumberValue(SnippetPieces.Value);
        } if (StageContext.IsPresent) {
            writer.WritePropertyName("stage_context");
            if (StageContext.Value is null) writer.WriteNullValue();
            else StageContext.Value.Write(writer);
        } if (Threshold.IsPresent) {
            writer.WritePropertyName("threshold");
            if (Threshold.Value is null) writer.WriteNullValue();
            else Threshold.Value.Write(writer);
        } if (Top.IsPresent) {
            writer.WritePropertyName("top");
            writer.WriteNumberValue(Top.Value);
        } writer.WriteEndObject();
    }
}
public abstract class InputRequestOriginal : InputDocument {
    private protected InputRequestOriginal() {
    }
}
public sealed class InputRequestOriginalJson : InputRequestOriginal {
    public required JsonElement Value {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        writer.WritePropertyName("kind");
        WriteText(writer, "json");
        writer.WritePropertyName("value");
        Value.WriteTo(writer);
        writer.WriteEndObject();
    }
}
public sealed class InputRequestOriginalText : InputRequestOriginal {
    public required string Text {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        writer.WritePropertyName("kind");
        WriteText(writer, "text");
        writer.WritePropertyName("text");
        WriteText(writer, Text);
        writer.WriteEndObject();
    }
}
public abstract class InputRequestQuestion : InputDocument {
    private protected InputRequestQuestion() {
    }
}
public sealed class InputRequestQuestionDefinition : InputRequestQuestion {
    public required InputRequestDefinition Value {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        writer.WritePropertyName("kind");
        WriteText(writer, "definition");
        writer.WritePropertyName("value");
        if (Value is null) writer.WriteNullValue();
        else Value.Write(writer);
        writer.WriteEndObject();
    }
}
public sealed class InputRequestQuestionFile : InputRequestQuestion {
    public required string Path {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        writer.WritePropertyName("kind");
        WriteText(writer, "file");
        writer.WritePropertyName("path");
        WriteText(writer, Path);
        writer.WriteEndObject();
    }
}
public sealed class InputRequestQuestionName : InputRequestQuestion {
    public required string Name {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        writer.WritePropertyName("kind");
        WriteText(writer, "name");
        writer.WritePropertyName("name");
        WriteText(writer, Name);
        writer.WriteEndObject();
    }
}
public sealed class InputRequestQuestionReference : InputRequestQuestion {
    public required string Reference {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        writer.WritePropertyName("kind");
        WriteText(writer, "reference");
        writer.WritePropertyName("reference");
        WriteText(writer, Reference);
        writer.WriteEndObject();
    }
}
public sealed class InputRequestQuestionText : InputRequestQuestion {
    public required string Text {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        writer.WritePropertyName("kind");
        WriteText(writer, "text");
        writer.WritePropertyName("text");
        WriteText(writer, Text);
        writer.WriteEndObject();
    }
}
public sealed class InputRequestReader : InputDocument {
    public InputPresence<InputSourceUnit> Unit {
        get;
        init;
    }
    public InputPresence<ulong> Window {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        if (Unit.IsPresent) {
            writer.WritePropertyName("unit");
            if (Unit.Value is null) writer.WriteNullValue();
            else Unit.Value.Write(writer);
        } if (Window.IsPresent) {
            writer.WritePropertyName("window");
            writer.WriteNumberValue(Window.Value);
        } writer.WriteEndObject();
    }
}
public abstract class InputRequestReaderFailure : InputDocument {
    private protected InputRequestReaderFailure() {
    }
}
public sealed class InputRequestReaderFailureInvalidInput : InputRequestReaderFailure {
    public InputPresence<InputSessionSourceLocation> Location {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        writer.WritePropertyName("kind");
        WriteText(writer, "invalid_input");
        if (Location.IsPresent) {
            writer.WritePropertyName("location");
            if (Location.Value is null) writer.WriteNullValue();
            else Location.Value.Write(writer);
        } writer.WriteEndObject();
    }
}
public sealed class InputRequestReaderFailureIo : InputRequestReaderFailure {
    public InputPresence<InputSessionSourceLocation> Location {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        writer.WritePropertyName("kind");
        WriteText(writer, "io");
        if (Location.IsPresent) {
            writer.WritePropertyName("location");
            if (Location.Value is null) writer.WriteNullValue();
            else Location.Value.Write(writer);
        } writer.WriteEndObject();
    }
}
public sealed class InputRequestReaderFailureProducer : InputRequestReaderFailure {
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        writer.WritePropertyName("kind");
        WriteText(writer, "producer");
        writer.WriteEndObject();
    }
}
public sealed class InputRequestReaderFailureUtf8 : InputRequestReaderFailure {
    public InputPresence<InputSessionSourceLocation> Location {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        writer.WritePropertyName("kind");
        WriteText(writer, "utf8");
        if (Location.IsPresent) {
            writer.WritePropertyName("location");
            if (Location.Value is null) writer.WriteNullValue();
            else Location.Value.Write(writer);
        } writer.WriteEndObject();
    }
}
public sealed class InputRequestSessionDescriptor : InputDocument {
    public required InputRequestItem Item {
        get;
        init;
    }
    public InputPresence<InputSessionSourceLocation> Location {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        writer.WritePropertyName("item");
        if (Item is null) writer.WriteNullValue();
        else Item.Write(writer);
        if (Location.IsPresent) {
            writer.WritePropertyName("location");
            if (Location.Value is null) writer.WriteNullValue();
            else Location.Value.Write(writer);
        } writer.WriteEndObject();
    }
}
public sealed class InputRequestSource : InputDocument {
    public InputPresence<InputRequestFraming> Framing {
        get;
        init;
    }
    public InputPresence<InputReaderMedia> Media {
        get;
        init;
    }
    public required IReadOnlyList<string> Paths {
        get;
        init;
    }
    public InputPresence<InputRequestReader> Reading {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        if (Framing.IsPresent) {
            writer.WritePropertyName("framing");
            if (Framing.Value is null) writer.WriteNullValue();
            else Framing.Value.Write(writer);
        } if (Media.IsPresent) {
            writer.WritePropertyName("media");
            if (Media.Value is null) writer.WriteNullValue();
            else Media.Value.Write(writer);
        } writer.WritePropertyName("paths");
        writer.WriteStartArray();
        foreach (var item in Paths) {
            WriteText(writer, item);
        } writer.WriteEndArray();
        if (Reading.IsPresent) {
            writer.WritePropertyName("reading");
            if (Reading.Value is null) writer.WriteNullValue();
            else Reading.Value.Write(writer);
        } writer.WriteEndObject();
    }
}
public abstract class InputRequestThreshold : InputDocument {
    private protected InputRequestThreshold() {
    }
}
public sealed class InputRequestThresholdNumber : InputRequestThreshold {
    public required double Value {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNumberValue(Value);
    }
}
public sealed class InputRequestThresholdString : InputRequestThreshold {
    public required string Value {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, Value);
    }
}
public abstract class InputRequestVersion : InputDocument {
    private protected InputRequestVersion() {
    }
}
public sealed class InputRequestVersionThinkthenRequest1 : InputRequestVersion {
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "thinkthen.request/1");
    }
}
public sealed class InputSessionSourceLocation : InputDocument {
    public required string File {
        get;
        init;
    }
    public InputPresence<ulong> FirstLine {
        get;
        init;
    }
    public InputPresence<ulong> LastLine {
        get;
        init;
    }
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteStartObject();
        writer.WritePropertyName("file");
        WriteText(writer, File);
        if (FirstLine.IsPresent) {
            writer.WritePropertyName("first_line");
            writer.WriteNumberValue(FirstLine.Value);
        } if (LastLine.IsPresent) {
            writer.WritePropertyName("last_line");
            writer.WriteNumberValue(LastLine.Value);
        } writer.WriteEndObject();
    }
}
public abstract class InputSourceUnit : InputDocument {
    private protected InputSourceUnit() {
    }
}
public sealed class InputSourceUnitFile : InputSourceUnit {
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "file");
    }
}
public sealed class InputSourceUnitLine : InputSourceUnit {
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "line");
    }
}
public sealed class InputSourceUnitWindow : InputSourceUnit {
    public override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "window");
    }
}
public static class QuestionBuilders {
    public static InputRequestQuestionDefinition Relate(InputRelateQuestion question) {
        ArgumentNullException.ThrowIfNull(question);
        return new InputRequestQuestionDefinition {
            Value = new InputRequestDefinitionAuthoredRelate {
                Relate = question
            }
        };
    }
    public static InputRequestQuestionDefinition Recognize(InputRecognizeQuestion question) {
        ArgumentNullException.ThrowIfNull(question);
        return new InputRequestQuestionDefinition {
            Value = new InputRequestDefinitionRecognizeDefinition {
                Recognize = question
            }
        };
    }
}
