// Generated from the Rust-derived Request schema; do not edit.
#nullable enable
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.Json;
namespace ThinkThen.Inputs;
public readonly struct AuthoredPresence<T> {
    public bool IsPresent {
        get;
    }
    public T Value {
        get;
    }
    private AuthoredPresence(T value) {
        Value = value;
        IsPresent = true;
    }
    public static implicit operator AuthoredPresence<T>(T value) => new(value);
}
public abstract class AuthoredDocument {
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
    internal abstract void Write(Utf8JsonWriter writer);
    internal byte[] ToBytes() {
        using var stream = new MemoryStream();
        using (var writer = new Utf8JsonWriter(stream, new JsonWriterOptions {
            MaxDepth = int.MaxValue
        })) Write(writer);
        return stream.ToArray();
    }
}
public sealed class AnnotateDefinition : AuthoredDocument {
    public AuthoredPresence<JsonElement> Batch {
        get;
        init;
    }
    public AuthoredPresence<AuthoredProfile> Profile {
        get;
        init;
    }
    public required IReadOnlyList<KeyValuePair<string, AnnotateQuestion>> Questions {
        get;
        init;
    }
    public AuthoredPresence<AuthoredThreshold> Threshold {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public abstract class AnnotateQuestion : AuthoredDocument {
    private protected AnnotateQuestion() {
    } internal static AnnotateQuestion Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("decide",
        out _)) return AnnotateQuestionDecide.Read(value);
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("choose",
        out _)) return AnnotateQuestionChoose.Read(value);
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("tag",
        out _)) return AnnotateQuestionTag.Read(value);
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("score",
        out _)) return AnnotateQuestionScore.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class AnnotateQuestionChoose : AnnotateQuestion {
    public required AuthoredQuestionText Choose {
        get;
        init;
    }
    public AuthoredPresence<AuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public AuthoredPresence<AuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public AuthoredPresence<string> Name {
        get;
        init;
    }
    public AuthoredPresence<AuthoredPointers> On {
        get;
        init;
    }
    public AuthoredPresence<AuthoredOptions> Options {
        get;
        init;
    }
    public AuthoredPresence<AuthoredCut> Threshold {
        get;
        init;
    }
    public AuthoredPresence<long> WordingVersion {
        get;
        init;
    }
    internal new static AnnotateQuestionChoose Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Choose = AuthoredQuestionText.Read(value.GetProperty("choose")),
        ContextSchema = value.TryGetProperty("context_schema", out _) ? (AuthoredPresence<AuthoredInputDeclaration>)(AuthoredInputDeclaration.Read(value.GetProperty("context_schema"))) : default,
        ItemSchema = value.TryGetProperty("item_schema", out _) ? (AuthoredPresence<AuthoredInputDeclaration>)(AuthoredInputDeclaration.Read(value.GetProperty("item_schema"))) : default,
        Name = value.TryGetProperty("name", out _) ? (AuthoredPresence<string>)(value.GetProperty("name").GetString()!) : default,
        On = value.TryGetProperty("on", out _) ? (AuthoredPresence<AuthoredPointers>)(AuthoredPointers.Read(value.GetProperty("on"))) : default,
        Options = value.TryGetProperty("options", out _) ? (AuthoredPresence<AuthoredOptions>)(AuthoredOptions.Read(value.GetProperty("options"))) : default,
        Threshold = value.TryGetProperty("threshold", out _) ? (AuthoredPresence<AuthoredCut>)(AuthoredCut.Read(value.GetProperty("threshold"))) : default,
        WordingVersion = value.TryGetProperty("wording_version", out _) ? (AuthoredPresence<long>)(value.GetProperty("wording_version").GetInt64()) : default
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class AnnotateQuestionDecide : AnnotateQuestion {
    public AuthoredPresence<AuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public required AuthoredQuestionText Decide {
        get;
        init;
    }
    public AuthoredPresence<AuthoredCriterion> False {
        get;
        init;
    }
    public AuthoredPresence<AuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public AuthoredPresence<string> Name {
        get;
        init;
    }
    public AuthoredPresence<AuthoredPointers> On {
        get;
        init;
    }
    public AuthoredPresence<AuthoredThreshold> Threshold {
        get;
        init;
    }
    public AuthoredPresence<AuthoredCriterion> True {
        get;
        init;
    }
    public AuthoredPresence<long> WordingVersion {
        get;
        init;
    }
    internal new static AnnotateQuestionDecide Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), ContextSchema = value.TryGetProperty("context_schema", out _) ? (AuthoredPresence<AuthoredInputDeclaration>)(AuthoredInputDeclaration.Read(value.GetProperty("context_schema"))) : default,
        Decide = AuthoredQuestionText.Read(value.GetProperty("decide")), False = value.TryGetProperty("false",
        out _) ? (AuthoredPresence<AuthoredCriterion>)(AuthoredCriterion.Read(value.GetProperty("false"))) : default,
        ItemSchema = value.TryGetProperty("item_schema", out _) ? (AuthoredPresence<AuthoredInputDeclaration>)(AuthoredInputDeclaration.Read(value.GetProperty("item_schema"))) : default,
        Name = value.TryGetProperty("name", out _) ? (AuthoredPresence<string>)(value.GetProperty("name").GetString()!) : default,
        On = value.TryGetProperty("on", out _) ? (AuthoredPresence<AuthoredPointers>)(AuthoredPointers.Read(value.GetProperty("on"))) : default,
        Threshold = value.TryGetProperty("threshold", out _) ? (AuthoredPresence<AuthoredThreshold>)(AuthoredThreshold.Read(value.GetProperty("threshold"))) : default,
        True = value.TryGetProperty("true", out _) ? (AuthoredPresence<AuthoredCriterion>)(AuthoredCriterion.Read(value.GetProperty("true"))) : default,
        WordingVersion = value.TryGetProperty("wording_version", out _) ? (AuthoredPresence<long>)(value.GetProperty("wording_version").GetInt64()) : default
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class AnnotateQuestionScore : AnnotateQuestion {
    public AuthoredPresence<AuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public AuthoredPresence<AuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public AuthoredPresence<AuthoredLevels> Levels {
        get;
        init;
    }
    public AuthoredPresence<string> Name {
        get;
        init;
    }
    public AuthoredPresence<AuthoredPointers> On {
        get;
        init;
    }
    public required AuthoredQuestionText Score {
        get;
        init;
    }
    public AuthoredPresence<long> WordingVersion {
        get;
        init;
    }
    internal new static AnnotateQuestionScore Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), ContextSchema = value.TryGetProperty("context_schema", out _) ? (AuthoredPresence<AuthoredInputDeclaration>)(AuthoredInputDeclaration.Read(value.GetProperty("context_schema"))) : default,
        ItemSchema = value.TryGetProperty("item_schema", out _) ? (AuthoredPresence<AuthoredInputDeclaration>)(AuthoredInputDeclaration.Read(value.GetProperty("item_schema"))) : default,
        Levels = value.TryGetProperty("levels", out _) ? (AuthoredPresence<AuthoredLevels>)(AuthoredLevels.Read(value.GetProperty("levels"))) : default,
        Name = value.TryGetProperty("name", out _) ? (AuthoredPresence<string>)(value.GetProperty("name").GetString()!) : default,
        On = value.TryGetProperty("on", out _) ? (AuthoredPresence<AuthoredPointers>)(AuthoredPointers.Read(value.GetProperty("on"))) : default,
        Score = AuthoredQuestionText.Read(value.GetProperty("score")), WordingVersion = value.TryGetProperty("wording_version",
        out _) ? (AuthoredPresence<long>)(value.GetProperty("wording_version").GetInt64()) : default
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class AnnotateQuestionTag : AnnotateQuestion {
    public AuthoredPresence<AuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public AuthoredPresence<AuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public AuthoredPresence<AuthoredLabels> Labels {
        get;
        init;
    }
    public AuthoredPresence<string> Name {
        get;
        init;
    }
    public AuthoredPresence<AuthoredPointers> On {
        get;
        init;
    }
    public required AuthoredQuestionText Tag {
        get;
        init;
    }
    public AuthoredPresence<AuthoredCut> Threshold {
        get;
        init;
    }
    public AuthoredPresence<long> WordingVersion {
        get;
        init;
    }
    internal new static AnnotateQuestionTag Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), ContextSchema = value.TryGetProperty("context_schema", out _) ? (AuthoredPresence<AuthoredInputDeclaration>)(AuthoredInputDeclaration.Read(value.GetProperty("context_schema"))) : default,
        ItemSchema = value.TryGetProperty("item_schema", out _) ? (AuthoredPresence<AuthoredInputDeclaration>)(AuthoredInputDeclaration.Read(value.GetProperty("item_schema"))) : default,
        Labels = value.TryGetProperty("labels", out _) ? (AuthoredPresence<AuthoredLabels>)(AuthoredLabels.Read(value.GetProperty("labels"))) : default,
        Name = value.TryGetProperty("name", out _) ? (AuthoredPresence<string>)(value.GetProperty("name").GetString()!) : default,
        On = value.TryGetProperty("on", out _) ? (AuthoredPresence<AuthoredPointers>)(AuthoredPointers.Read(value.GetProperty("on"))) : default,
        Tag = AuthoredQuestionText.Read(value.GetProperty("tag")), Threshold = value.TryGetProperty("threshold",
        out _) ? (AuthoredPresence<AuthoredCut>)(AuthoredCut.Read(value.GetProperty("threshold"))) : default,
        WordingVersion = value.TryGetProperty("wording_version", out _) ? (AuthoredPresence<long>)(value.GetProperty("wording_version").GetInt64()) : default
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class AuthoredChoose : AuthoredDocument {
    public AuthoredPresence<AuthoredChooseBatch> Batch {
        get;
        init;
    }
    public required AuthoredQuestionText Choose {
        get;
        init;
    }
    public AuthoredPresence<AuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public AuthoredPresence<AuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public AuthoredPresence<AuthoredName> Model {
        get;
        init;
    }
    public AuthoredPresence<string> Name {
        get;
        init;
    }
    public AuthoredPresence<AuthoredPointers> On {
        get;
        init;
    }
    public AuthoredPresence<AuthoredOptions> Options {
        get;
        init;
    }
    public AuthoredPresence<AuthoredProfile> Profile {
        get;
        init;
    }
    public AuthoredPresence<AuthoredCut> Threshold {
        get;
        init;
    }
    public AuthoredPresence<long> WordingVersion {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public abstract class AuthoredChooseBatch : AuthoredDocument {
    private protected AuthoredChooseBatch() {
    }
}
public sealed class AuthoredChooseBatchInteger : AuthoredChooseBatch {
    public required long Value {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNumberValue(Value);
    }
}
public sealed class AuthoredChooseBatchMax : AuthoredChooseBatch {
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "max");
    }
}
public abstract class AuthoredCriterion : AuthoredDocument {
    private protected AuthoredCriterion() {
    } internal static AuthoredCriterion Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.String)) return AuthoredCriterionString.Read(value);
        if ((value.ValueKind == JsonValueKind.Object)) return AuthoredCriterionObject.Read(value);
        if ((value.ValueKind == JsonValueKind.Array)) return AuthoredCriterionList.Read(value);
        if ((value.ValueKind == JsonValueKind.Null)) return AuthoredCriterionNull.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class AuthoredCriterionList : AuthoredCriterion {
    public required IReadOnlyList<JsonElement> Value {
        get;
        init;
    }
    internal new static AuthoredCriterionList Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = Array.AsReadOnly(value.EnumerateArray().Select(item0 => item0.Clone()).ToArray())
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class AuthoredCriterionNull : AuthoredCriterion {
    internal new static AuthoredCriterionNull Read(JsonElement value) => new() {
        ParsedDocument = value.Clone()
    };
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNullValue();
    }
}
public sealed class AuthoredCriterionObject : AuthoredCriterion {
    public required JsonElement Value {
        get;
        init;
    }
    internal new static AuthoredCriterionObject Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.Clone()
    };
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } Value.WriteTo(writer);
    }
}
public sealed class AuthoredCriterionString : AuthoredCriterion {
    public required string Value {
        get;
        init;
    }
    internal new static AuthoredCriterionString Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.GetString()!
    };
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, Value);
    }
}
public abstract class AuthoredCut : AuthoredDocument {
    private protected AuthoredCut() {
    } internal static AuthoredCut Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.Number)) return AuthoredCutNumber.Read(value);
        if ((value.ValueKind == JsonValueKind.String)) return AuthoredCutString.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class AuthoredCutNumber : AuthoredCut {
    public required double Value {
        get;
        init;
    }
    internal new static AuthoredCutNumber Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.GetDouble()
    };
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNumberValue(Value);
    }
}
public sealed class AuthoredCutString : AuthoredCut {
    public required string Value {
        get;
        init;
    }
    internal new static AuthoredCutString Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.GetString()!
    };
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, Value);
    }
}
public sealed class AuthoredDecide : AuthoredDocument {
    public AuthoredPresence<AuthoredDecideBatch> Batch {
        get;
        init;
    }
    public AuthoredPresence<AuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public required AuthoredQuestionText Decide {
        get;
        init;
    }
    public AuthoredPresence<AuthoredCriterion> False {
        get;
        init;
    }
    public AuthoredPresence<AuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public AuthoredPresence<AuthoredName> Model {
        get;
        init;
    }
    public AuthoredPresence<string> Name {
        get;
        init;
    }
    public AuthoredPresence<AuthoredPointers> On {
        get;
        init;
    }
    public AuthoredPresence<AuthoredProfile> Profile {
        get;
        init;
    }
    public AuthoredPresence<AuthoredThreshold> Threshold {
        get;
        init;
    }
    public AuthoredPresence<AuthoredCriterion> True {
        get;
        init;
    }
    public AuthoredPresence<long> WordingVersion {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public abstract class AuthoredDecideBatch : AuthoredDocument {
    private protected AuthoredDecideBatch() {
    }
}
public sealed class AuthoredDecideBatchInteger : AuthoredDecideBatch {
    public required long Value {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNumberValue(Value);
    }
}
public sealed class AuthoredDecideBatchMax : AuthoredDecideBatch {
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "max");
    }
}
public sealed class AuthoredDescription : AuthoredDocument {
    public required AuthoredDescriptionValue Value {
        get;
        init;
    }
    internal static AuthoredDescription Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = AuthoredDescriptionValue.Read(value)
    };
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } if (Value is null) writer.WriteNullValue();
        else Value.Write(writer);
    }
}
public abstract class AuthoredDescriptionValue : AuthoredDocument {
    private protected AuthoredDescriptionValue() {
    } internal static AuthoredDescriptionValue Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.String)) return AuthoredDescriptionValueString.Read(value);
        if ((value.ValueKind == JsonValueKind.Object)) return AuthoredDescriptionValueObject.Read(value);
        if ((value.ValueKind == JsonValueKind.Array)) return AuthoredDescriptionValueList.Read(value);
        if ((value.ValueKind == JsonValueKind.Null)) return AuthoredDescriptionValueNull.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class AuthoredDescriptionValueList : AuthoredDescriptionValue {
    public required IReadOnlyList<JsonElement> Value {
        get;
        init;
    }
    internal new static AuthoredDescriptionValueList Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = Array.AsReadOnly(value.EnumerateArray().Select(item0 => item0.Clone()).ToArray())
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class AuthoredDescriptionValueNull : AuthoredDescriptionValue {
    internal new static AuthoredDescriptionValueNull Read(JsonElement value) => new() {
        ParsedDocument = value.Clone()
    };
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNullValue();
    }
}
public sealed class AuthoredDescriptionValueObject : AuthoredDescriptionValue {
    public required JsonElement Value {
        get;
        init;
    }
    internal new static AuthoredDescriptionValueObject Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.Clone()
    };
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } Value.WriteTo(writer);
    }
}
public sealed class AuthoredDescriptionValueString : AuthoredDescriptionValue {
    public required string Value {
        get;
        init;
    }
    internal new static AuthoredDescriptionValueString Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.GetString()!
    };
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, Value);
    }
}
public sealed class AuthoredFind : AuthoredDocument {
    public AuthoredPresence<AuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public required AuthoredQuestionText Find {
        get;
        init;
    }
    public AuthoredPresence<AuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public AuthoredPresence<AuthoredName> Model {
        get;
        init;
    }
    public AuthoredPresence<string> Name {
        get;
        init;
    }
    public AuthoredPresence<AuthoredPointers> On {
        get;
        init;
    }
    public AuthoredPresence<AuthoredProfile> Profile {
        get;
        init;
    }
    public AuthoredPresence<long> WordingVersion {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public abstract class AuthoredInputDeclaration : AuthoredDocument {
    private protected AuthoredInputDeclaration() {
    } internal static AuthoredInputDeclaration Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("type",
        out _) && (value.GetProperty("type").ValueKind == JsonValueKind.String && value.GetProperty("type").GetString() == "string")) return AuthoredInputDeclarationString.Read(value);
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("type",
        out _) && value.TryGetProperty("properties", out _) && (value.GetProperty("type").ValueKind == JsonValueKind.String && value.GetProperty("type").GetString() == "object")) return AuthoredInputDeclarationObject.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class AuthoredInputDeclarationObject : AuthoredInputDeclaration {
    public required IReadOnlyList<KeyValuePair<string, AuthoredInputProperty>> Properties {
        get;
        init;
    }
    public AuthoredPresence<IReadOnlyList<string>> Required {
        get;
        init;
    }
    internal new static AuthoredInputDeclarationObject Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Properties = Array.AsReadOnly(value.GetProperty("properties").EnumerateObject().Select(entry0 => new KeyValuePair<string,
        AuthoredInputProperty>(entry0.Name, AuthoredInputProperty.Read(entry0.Value))).ToArray()),
        Required = value.TryGetProperty("required", out _) ? (AuthoredPresence<IReadOnlyList<string>>)(Array.AsReadOnly(value.GetProperty("required").EnumerateArray().Select(item0 => item0.GetString()!).ToArray())) : default
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class AuthoredInputDeclarationString : AuthoredInputDeclaration {
    internal new static AuthoredInputDeclarationString Read(JsonElement value) => new() {
        ParsedDocument = value.Clone()
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public abstract class AuthoredInputProperty : AuthoredDocument {
    private protected AuthoredInputProperty() {
    } internal static AuthoredInputProperty Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("type",
        out _) && (value.GetProperty("type").ValueKind == JsonValueKind.String && value.GetProperty("type").GetString() == "string")) return AuthoredInputPropertyString.Read(value);
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("type",
        out _) && (value.GetProperty("type").ValueKind == JsonValueKind.String && value.GetProperty("type").GetString() == "number")) return AuthoredInputPropertyNumber.Read(value);
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("type",
        out _) && (value.GetProperty("type").ValueKind == JsonValueKind.String && value.GetProperty("type").GetString() == "boolean")) return AuthoredInputPropertyBoolean.Read(value);
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("type",
        out _) && value.TryGetProperty("items", out _) && (value.GetProperty("type").ValueKind == JsonValueKind.String && value.GetProperty("type").GetString() == "array")) return AuthoredInputPropertyArray.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class AuthoredInputPropertyArray : AuthoredInputProperty {
    public required AuthoredInputPropertyArrayItems Items {
        get;
        init;
    }
    internal new static AuthoredInputPropertyArray Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Items = AuthoredInputPropertyArrayItems.Read(value.GetProperty("items"))
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class AuthoredInputPropertyArrayItems : AuthoredDocument {
    internal static AuthoredInputPropertyArrayItems Read(JsonElement value) => new() {
        ParsedDocument = value.Clone()
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class AuthoredInputPropertyBoolean : AuthoredInputProperty {
    internal new static AuthoredInputPropertyBoolean Read(JsonElement value) => new() {
        ParsedDocument = value.Clone()
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class AuthoredInputPropertyNumber : AuthoredInputProperty {
    internal new static AuthoredInputPropertyNumber Read(JsonElement value) => new() {
        ParsedDocument = value.Clone()
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class AuthoredInputPropertyString : AuthoredInputProperty {
    internal new static AuthoredInputPropertyString Read(JsonElement value) => new() {
        ParsedDocument = value.Clone()
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public abstract class AuthoredLabels : AuthoredDocument {
    private protected AuthoredLabels() {
    } internal static AuthoredLabels Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.Array)) return AuthoredLabelsList.Read(value);
        if ((value.ValueKind == JsonValueKind.Object)) return AuthoredLabelsObject.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class AuthoredLabelsList : AuthoredLabels {
    public required IReadOnlyList<AuthoredName> Value {
        get;
        init;
    }
    internal new static AuthoredLabelsList Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = Array.AsReadOnly(value.EnumerateArray().Select(item0 => AuthoredName.Read(item0)).ToArray())
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class AuthoredLabelsObject : AuthoredLabels {
    public required IReadOnlyList<KeyValuePair<string, AuthoredDescription>> Value {
        get;
        init;
    }
    internal new static AuthoredLabelsObject Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = Array.AsReadOnly(value.EnumerateObject().Select(entry0 => new KeyValuePair<string,
        AuthoredDescription>(entry0.Name, AuthoredDescription.Read(entry0.Value))).ToArray())
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public abstract class AuthoredLevels : AuthoredDocument {
    private protected AuthoredLevels() {
    } internal static AuthoredLevels Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.Array)) return AuthoredLevelsList.Read(value);
        if ((value.ValueKind == JsonValueKind.Object)) return AuthoredLevelsObject.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class AuthoredLevelsList : AuthoredLevels {
    public required IReadOnlyList<AuthoredName> Value {
        get;
        init;
    }
    internal new static AuthoredLevelsList Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = Array.AsReadOnly(value.EnumerateArray().Select(item0 => AuthoredName.Read(item0)).ToArray())
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class AuthoredLevelsObject : AuthoredLevels {
    public required IReadOnlyList<KeyValuePair<string, AuthoredCriterion>> Value {
        get;
        init;
    }
    internal new static AuthoredLevelsObject Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = Array.AsReadOnly(value.EnumerateObject().Select(entry0 => new KeyValuePair<string,
        AuthoredCriterion>(entry0.Name, AuthoredCriterion.Read(entry0.Value))).ToArray())
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class AuthoredName : AuthoredDocument {
    public required string Value {
        get;
        init;
    }
    internal static AuthoredName Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.GetString()!
    };
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, Value);
    }
}
public abstract class AuthoredOptions : AuthoredDocument {
    private protected AuthoredOptions() {
    } internal static AuthoredOptions Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.Array)) return AuthoredOptionsList.Read(value);
        if ((value.ValueKind == JsonValueKind.Object)) return AuthoredOptionsObject.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class AuthoredOptionsList : AuthoredOptions {
    public required IReadOnlyList<AuthoredName> Value {
        get;
        init;
    }
    internal new static AuthoredOptionsList Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = Array.AsReadOnly(value.EnumerateArray().Select(item0 => AuthoredName.Read(item0)).ToArray())
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class AuthoredOptionsObject : AuthoredOptions {
    public required IReadOnlyList<KeyValuePair<string, AuthoredDescription>> Value {
        get;
        init;
    }
    internal new static AuthoredOptionsObject Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = Array.AsReadOnly(value.EnumerateObject().Select(entry0 => new KeyValuePair<string,
        AuthoredDescription>(entry0.Name, AuthoredDescription.Read(entry0.Value))).ToArray())
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public abstract class AuthoredPointers : AuthoredDocument {
    private protected AuthoredPointers() {
    } internal static AuthoredPointers Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.String)) return AuthoredPointersString.Read(value);
        if ((value.ValueKind == JsonValueKind.Array)) return AuthoredPointersList.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class AuthoredPointersList : AuthoredPointers {
    public required IReadOnlyList<string> Value {
        get;
        init;
    }
    internal new static AuthoredPointersList Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = Array.AsReadOnly(value.EnumerateArray().Select(item0 => item0.GetString()!).ToArray())
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class AuthoredPointersString : AuthoredPointers {
    public required string Value {
        get;
        init;
    }
    internal new static AuthoredPointersString Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.GetString()!
    };
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, Value);
    }
}
public sealed class AuthoredProfile : AuthoredDocument {
    public required string Value {
        get;
        init;
    }
    internal static AuthoredProfile Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.GetString()!
    };
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, Value);
    }
}
public abstract class AuthoredQuestionText : AuthoredDocument {
    private protected AuthoredQuestionText() {
    } internal static AuthoredQuestionText Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.String)) return AuthoredQuestionTextString.Read(value);
        if ((value.ValueKind == JsonValueKind.Object)) return AuthoredQuestionTextObject.Read(value);
        if ((value.ValueKind == JsonValueKind.Array)) return AuthoredQuestionTextList.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class AuthoredQuestionTextList : AuthoredQuestionText {
    public required IReadOnlyList<JsonElement> Value {
        get;
        init;
    }
    internal new static AuthoredQuestionTextList Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = Array.AsReadOnly(value.EnumerateArray().Select(item0 => item0.Clone()).ToArray())
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class AuthoredQuestionTextObject : AuthoredQuestionText {
    public required JsonElement Value {
        get;
        init;
    }
    internal new static AuthoredQuestionTextObject Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.Clone()
    };
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } Value.WriteTo(writer);
    }
}
public sealed class AuthoredQuestionTextString : AuthoredQuestionText {
    public required string Value {
        get;
        init;
    }
    internal new static AuthoredQuestionTextString Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.GetString()!
    };
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, Value);
    }
}
public sealed class AuthoredRelate : AuthoredDocument {
    public AuthoredPresence<AuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public AuthoredPresence<AuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public AuthoredPresence<AuthoredName> Model {
        get;
        init;
    }
    public AuthoredPresence<string> Name {
        get;
        init;
    }
    public AuthoredPresence<AuthoredProfile> Profile {
        get;
        init;
    }
    public required RelateQuestion Relate {
        get;
        init;
    }
    public AuthoredPresence<AuthoredCut> Threshold {
        get;
        init;
    }
    public AuthoredPresence<long> WordingVersion {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class AuthoredRelation : AuthoredDocument {
    public AuthoredPresence<bool> Either {
        get;
        init;
    }
    public required AuthoredName Name {
        get;
        init;
    }
    public AuthoredPresence<AuthoredName> Reads {
        get;
        init;
    }
    public AuthoredPresence<bool> Single {
        get;
        init;
    }
    public AuthoredPresence<AuthoredName> Source {
        get;
        init;
    }
    public AuthoredPresence<AuthoredName> Target {
        get;
        init;
    }
    internal static AuthoredRelation Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Either = value.TryGetProperty("either", out _) ? (AuthoredPresence<bool>)(value.GetProperty("either").GetBoolean()) : default,
        Name = AuthoredName.Read(value.GetProperty("name")), Reads = value.TryGetProperty("reads",
        out _) ? (AuthoredPresence<AuthoredName>)(AuthoredName.Read(value.GetProperty("reads"))) : default,
        Single = value.TryGetProperty("single", out _) ? (AuthoredPresence<bool>)(value.GetProperty("single").GetBoolean()) : default,
        Source = value.TryGetProperty("source", out _) ? (AuthoredPresence<AuthoredName>)(AuthoredName.Read(value.GetProperty("source"))) : default,
        Target = value.TryGetProperty("target", out _) ? (AuthoredPresence<AuthoredName>)(AuthoredName.Read(value.GetProperty("target"))) : default
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class AuthoredScore : AuthoredDocument {
    public AuthoredPresence<AuthoredScoreBatch> Batch {
        get;
        init;
    }
    public AuthoredPresence<AuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public AuthoredPresence<AuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public AuthoredPresence<AuthoredLevels> Levels {
        get;
        init;
    }
    public AuthoredPresence<AuthoredName> Model {
        get;
        init;
    }
    public AuthoredPresence<string> Name {
        get;
        init;
    }
    public AuthoredPresence<AuthoredPointers> On {
        get;
        init;
    }
    public AuthoredPresence<AuthoredProfile> Profile {
        get;
        init;
    }
    public required AuthoredQuestionText Score {
        get;
        init;
    }
    public AuthoredPresence<long> WordingVersion {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public abstract class AuthoredScoreBatch : AuthoredDocument {
    private protected AuthoredScoreBatch() {
    }
}
public sealed class AuthoredScoreBatchInteger : AuthoredScoreBatch {
    public required long Value {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNumberValue(Value);
    }
}
public sealed class AuthoredScoreBatchMax : AuthoredScoreBatch {
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "max");
    }
}
public sealed class AuthoredTag : AuthoredDocument {
    public AuthoredPresence<AuthoredTagBatch> Batch {
        get;
        init;
    }
    public AuthoredPresence<AuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public AuthoredPresence<AuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public AuthoredPresence<AuthoredLabels> Labels {
        get;
        init;
    }
    public AuthoredPresence<AuthoredName> Model {
        get;
        init;
    }
    public AuthoredPresence<string> Name {
        get;
        init;
    }
    public AuthoredPresence<AuthoredPointers> On {
        get;
        init;
    }
    public AuthoredPresence<AuthoredProfile> Profile {
        get;
        init;
    }
    public required AuthoredQuestionText Tag {
        get;
        init;
    }
    public AuthoredPresence<AuthoredCut> Threshold {
        get;
        init;
    }
    public AuthoredPresence<long> WordingVersion {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public abstract class AuthoredTagBatch : AuthoredDocument {
    private protected AuthoredTagBatch() {
    }
}
public sealed class AuthoredTagBatchInteger : AuthoredTagBatch {
    public required long Value {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNumberValue(Value);
    }
}
public sealed class AuthoredTagBatchMax : AuthoredTagBatch {
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "max");
    }
}
public abstract class AuthoredThreshold : AuthoredDocument {
    private protected AuthoredThreshold() {
    } internal static AuthoredThreshold Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.Number)) return AuthoredThresholdNumber.Read(value);
        if ((value.ValueKind == JsonValueKind.String)) return AuthoredThresholdString.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class AuthoredThresholdNumber : AuthoredThreshold {
    public required double Value {
        get;
        init;
    }
    internal new static AuthoredThresholdNumber Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.GetDouble()
    };
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNumberValue(Value);
    }
}
public sealed class AuthoredThresholdString : AuthoredThreshold {
    public required string Value {
        get;
        init;
    }
    internal new static AuthoredThresholdString Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.GetString()!
    };
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, Value);
    }
}
public abstract class CacheDocument : AuthoredDocument {
    private protected CacheDocument() {
    }
}
public sealed class CacheDocumentDisabledCache : CacheDocument {
    public required DisabledCache Value {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } if (Value is null) writer.WriteNullValue();
        else Value.Write(writer);
    }
}
public sealed class CacheDocumentString : CacheDocument {
    public required string Value {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, Value);
    }
}
public abstract class ContextSchema : AuthoredDocument {
    private protected ContextSchema() {
    }
}
public sealed class ContextSchemaObject : ContextSchema {
    public required JsonElement Value {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } Value.WriteTo(writer);
    }
}
public sealed class ContextSchemaString : ContextSchema {
    public required string Value {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, Value);
    }
}
public sealed class DisabledCache : AuthoredDocument {
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteBooleanValue(false);
    }
}
public sealed class EngineSettings : AuthoredDocument {
    public AuthoredPresence<string> Backend {
        get;
        init;
    }
    public AuthoredPresence<string> BaseUrl {
        get;
        init;
    }
    public AuthoredPresence<RequestBatch> Batch {
        get;
        init;
    }
    public AuthoredPresence<CacheDocument> Cache {
        get;
        init;
    }
    public AuthoredPresence<EngineSettingsMaxEstimatedInputTokensTotal> MaxEstimatedInputTokensTotal {
        get;
        init;
    }
    public AuthoredPresence<ulong> MaxRequestBytes {
        get;
        init;
    }
    public AuthoredPresence<EngineSettingsMaxRequests> MaxRequests {
        get;
        init;
    }
    public AuthoredPresence<EngineSettingsMaxRequestsTotal> MaxRequestsTotal {
        get;
        init;
    }
    public AuthoredPresence<uint> MaxRetries {
        get;
        init;
    }
    public AuthoredPresence<string> Model {
        get;
        init;
    }
    public AuthoredPresence<string> Profile {
        get;
        init;
    }
    public AuthoredPresence<JsonElement> Proxy {
        get;
        init;
    }
    public AuthoredPresence<string> Record {
        get;
        init;
    }
    public AuthoredPresence<bool> RefreshCache {
        get;
        init;
    }
    public AuthoredPresence<string> Replay {
        get;
        init;
    }
    public AuthoredPresence<long> Throttle {
        get;
        init;
    }
    public AuthoredPresence<ulong> Timeout {
        get;
        init;
    }
    public AuthoredPresence<string> UsdPerMillionInput {
        get;
        init;
    }
    public AuthoredPresence<string> UsdPerMillionOutput {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public abstract class EngineSettingsMaxEstimatedInputTokensTotal : AuthoredDocument {
    private protected EngineSettingsMaxEstimatedInputTokensTotal() {
    }
}
public sealed class EngineSettingsMaxEstimatedInputTokensTotalInteger : EngineSettingsMaxEstimatedInputTokensTotal {
    public required ulong Value {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNumberValue(Value);
    }
}
public sealed class EngineSettingsMaxEstimatedInputTokensTotalNull : EngineSettingsMaxEstimatedInputTokensTotal {
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNullValue();
    }
}
public abstract class EngineSettingsMaxRequests : AuthoredDocument {
    private protected EngineSettingsMaxRequests() {
    }
}
public sealed class EngineSettingsMaxRequestsInteger : EngineSettingsMaxRequests {
    public required ulong Value {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNumberValue(Value);
    }
}
public sealed class EngineSettingsMaxRequestsNull : EngineSettingsMaxRequests {
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNullValue();
    }
}
public abstract class EngineSettingsMaxRequestsTotal : AuthoredDocument {
    private protected EngineSettingsMaxRequestsTotal() {
    }
}
public sealed class EngineSettingsMaxRequestsTotalInteger : EngineSettingsMaxRequestsTotal {
    public required ulong Value {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNumberValue(Value);
    }
}
public sealed class EngineSettingsMaxRequestsTotalNull : EngineSettingsMaxRequestsTotal {
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNullValue();
    }
}
public abstract class ImageMedia : AuthoredDocument {
    private protected ImageMedia() {
    }
}
public sealed class ImageMediaImageJpeg : ImageMedia {
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "image/jpeg");
    }
}
public sealed class ImageMediaImagePng : ImageMedia {
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "image/png");
    }
}
public sealed class OptionSchema : AuthoredDocument {
    public AuthoredPresence<JsonElement> Description {
        get;
        init;
    }
    public required string Name {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public abstract class ReaderMedia : AuthoredDocument {
    private protected ReaderMedia() {
    }
}
public sealed class ReaderMediaImage : ReaderMedia {
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "image");
    }
}
public sealed class ReaderMediaText : ReaderMedia {
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "text");
    }
}
public abstract class RecognitionExample : AuthoredDocument {
    private protected RecognitionExample() {
    }
}
public sealed class RecognitionExampleEntity : AuthoredDocument {
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
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class RecognitionExampleText : AuthoredDocument {
    public required IReadOnlyList<RecognitionExampleEntity> Entities {
        get;
        init;
    }
    public AuthoredPresence<IReadOnlyList<string>> Kinds {
        get;
        init;
    }
    public required string Text {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class RecognitionExampleRecognitionExampleText : RecognitionExample {
    public required RecognitionExampleText Value {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } if (Value is null) writer.WriteNullValue();
        else Value.Write(writer);
    }
}
public sealed class RecognitionExampleString : RecognitionExample {
    public required string Value {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, Value);
    }
}
public abstract class RecognitionMode : AuthoredDocument {
    private protected RecognitionMode() {
    } internal static RecognitionMode Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.String) && (value.ValueKind == JsonValueKind.String && value.GetString() == "whole")) return RecognitionModeWhole.Read(value);
        if ((value.ValueKind == JsonValueKind.String) && (value.ValueKind == JsonValueKind.String && value.GetString() == "boundary_only")) return RecognitionModeBoundaryOnly.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class RecognitionModeBoundaryOnly : RecognitionMode {
    internal new static RecognitionModeBoundaryOnly Read(JsonElement value) => new() {
        ParsedDocument = value.Clone()
    };
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "boundary_only");
    }
}
public sealed class RecognitionModeWhole : RecognitionMode {
    internal new static RecognitionModeWhole Read(JsonElement value) => new() {
        ParsedDocument = value.Clone()
    };
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "whole");
    }
}
public sealed class RecognitionSeedSpan : AuthoredDocument {
    public required ulong End {
        get;
        init;
    }
    public AuthoredPresence<string> Kind {
        get;
        init;
    }
    public required ulong Start {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class RecognitionStageContext : AuthoredDocument {
    public AuthoredPresence<string> Boundary {
        get;
        init;
    }
    public AuthoredPresence<string> KindEdge {
        get;
        init;
    }
    public AuthoredPresence<string> Relation {
        get;
        init;
    }
    internal static RecognitionStageContext Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Boundary = value.TryGetProperty("boundary", out _) ? (AuthoredPresence<string>)(value.GetProperty("boundary").GetString()!) : default,
        KindEdge = value.TryGetProperty("kind_edge", out _) ? (AuthoredPresence<string>)(value.GetProperty("kind_edge").GetString()!) : default,
        Relation = value.TryGetProperty("relation", out _) ? (AuthoredPresence<string>)(value.GetProperty("relation").GetString()!) : default
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class RecognizeDefinition : AuthoredDocument {
    public AuthoredPresence<AuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public AuthoredPresence<AuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public AuthoredPresence<AuthoredName> Model {
        get;
        init;
    }
    public AuthoredPresence<string> Name {
        get;
        init;
    }
    public AuthoredPresence<AuthoredPointers> On {
        get;
        init;
    }
    public AuthoredPresence<AuthoredProfile> Profile {
        get;
        init;
    }
    public required RecognizeQuestion Recognize {
        get;
        init;
    }
    public AuthoredPresence<AuthoredCut> RelationThreshold {
        get;
        init;
    }
    public AuthoredPresence<AuthoredCut> Threshold {
        get;
        init;
    }
    public AuthoredPresence<long> WordingVersion {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class RecognizeQuestion : AuthoredDocument {
    public AuthoredPresence<AuthoredQuestionText> EntityDefinition {
        get;
        init;
    }
    public AuthoredPresence<AuthoredQuestionText> Instructions {
        get;
        init;
    }
    public AuthoredPresence<IReadOnlyList<KeyValuePair<string, AuthoredDescription>>> Kinds {
        get;
        init;
    }
    public AuthoredPresence<RecognitionMode> Mode {
        get;
        init;
    }
    public AuthoredPresence<IReadOnlyList<AuthoredRelation>> Relations {
        get;
        init;
    }
    public AuthoredPresence<uint> SnippetPieces {
        get;
        init;
    }
    public AuthoredPresence<RecognitionStageContext> StageContext {
        get;
        init;
    }
    internal static RecognizeQuestion Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), EntityDefinition = value.TryGetProperty("entity_definition",
        out _) ? (AuthoredPresence<AuthoredQuestionText>)(AuthoredQuestionText.Read(value.GetProperty("entity_definition"))) : default,
        Instructions = value.TryGetProperty("instructions", out _) ? (AuthoredPresence<AuthoredQuestionText>)(AuthoredQuestionText.Read(value.GetProperty("instructions"))) : default,
        Kinds = value.TryGetProperty("kinds", out _) ? (AuthoredPresence<IReadOnlyList<KeyValuePair<string,
        AuthoredDescription>>>)(Array.AsReadOnly(value.GetProperty("kinds").EnumerateObject().Select(entry0 => new KeyValuePair<string,
        AuthoredDescription>(entry0.Name, AuthoredDescription.Read(entry0.Value))).ToArray())) : default,
        Mode = value.TryGetProperty("mode", out _) ? (AuthoredPresence<RecognitionMode>)(RecognitionMode.Read(value.GetProperty("mode"))) : default,
        Relations = value.TryGetProperty("relations", out _) ? (AuthoredPresence<IReadOnlyList<AuthoredRelation>>)(Array.AsReadOnly(value.GetProperty("relations").EnumerateArray().Select(item0 => AuthoredRelation.Read(item0)).ToArray())) : default,
        SnippetPieces = value.TryGetProperty("snippet_pieces", out _) ? (AuthoredPresence<uint>)(value.GetProperty("snippet_pieces").GetUInt32()) : default,
        StageContext = value.TryGetProperty("stage_context", out _) ? (AuthoredPresence<RecognitionStageContext>)(RecognitionStageContext.Read(value.GetProperty("stage_context"))) : default
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class RelateFields : AuthoredDocument {
    public required string Kind {
        get;
        init;
    }
    public required string Name {
        get;
        init;
    }
    internal static RelateFields Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Kind = value.GetProperty("kind").GetString()!, Name = value.GetProperty("name").GetString()!
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class RelateQuestion : AuthoredDocument {
    public AuthoredPresence<RelateFields> Fields {
        get;
        init;
    }
    public required IReadOnlyList<AuthoredRelation> Relations {
        get;
        init;
    }
    internal static RelateQuestion Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Fields = value.TryGetProperty("fields", out _) ? (AuthoredPresence<RelateFields>)(RelateFields.Read(value.GetProperty("fields"))) : default,
        Relations = Array.AsReadOnly(value.GetProperty("relations").EnumerateArray().Select(item0 => AuthoredRelation.Read(item0)).ToArray())
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class Request : AuthoredDocument {
    public required RequestCall Call {
        get;
        init;
    }
    public required RequestVersion Schema {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public abstract class RequestBatch : AuthoredDocument {
    private protected RequestBatch() {
    }
}
public sealed class RequestBatchInteger : RequestBatch {
    public required ulong Value {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNumberValue(Value);
    }
}
public sealed class RequestBatchString : RequestBatch {
    public required string Value {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, Value);
    }
}
public sealed class RequestCall : AuthoredDocument {
    public required RequestFunction Function {
        get;
        init;
    }
    public required RequestInput Input {
        get;
        init;
    }
    public AuthoredPresence<RequestOptions> Options {
        get;
        init;
    }
    public required RequestQuestion Question {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public abstract class RequestDefinition : AuthoredDocument {
    private protected RequestDefinition() {
    } internal static RequestDefinition Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("decide",
        out _)) return RequestDefinitionAuthoredDecide.Read(value);
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("choose",
        out _)) return RequestDefinitionAuthoredChoose.Read(value);
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("tag",
        out _)) return RequestDefinitionAuthoredTag.Read(value);
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("score",
        out _)) return RequestDefinitionAuthoredScore.Read(value);
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("version",
        out _) && value.TryGetProperty("relate", out _) && (value.GetProperty("version").ValueKind == JsonValueKind.Number && value.GetProperty("version").GetInt64() == 1)) return RequestDefinitionAuthoredRelate.Read(value);
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("find",
        out _)) return RequestDefinitionAuthoredFind.Read(value);
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("version",
        out _) && value.TryGetProperty("recognize", out _) && (value.GetProperty("version").ValueKind == JsonValueKind.Number && value.GetProperty("version").GetInt64() == 1)) return RequestDefinitionRecognizeDefinition.Read(value);
        if ((value.ValueKind == JsonValueKind.Object) && value.ValueKind == JsonValueKind.Object && value.TryGetProperty("version",
        out _) && value.TryGetProperty("questions", out _) && (value.GetProperty("version").ValueKind == JsonValueKind.Number && value.GetProperty("version").GetInt64() == 1)) return RequestDefinitionAnnotateDefinition.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class RequestDefinitionAnnotateDefinition : RequestDefinition {
    public AuthoredPresence<JsonElement> Batch {
        get;
        init;
    }
    public AuthoredPresence<AuthoredProfile> Profile {
        get;
        init;
    }
    public required IReadOnlyList<KeyValuePair<string, AnnotateQuestion>> Questions {
        get;
        init;
    }
    public AuthoredPresence<AuthoredThreshold> Threshold {
        get;
        init;
    }
    internal new static RequestDefinitionAnnotateDefinition Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Batch = value.TryGetProperty("batch", out _) ? (AuthoredPresence<JsonElement>)(value.GetProperty("batch").Clone()) : default,
        Profile = value.TryGetProperty("profile", out _) ? (AuthoredPresence<AuthoredProfile>)(AuthoredProfile.Read(value.GetProperty("profile"))) : default,
        Questions = Array.AsReadOnly(value.GetProperty("questions").EnumerateObject().Select(entry0 => new KeyValuePair<string,
        AnnotateQuestion>(entry0.Name, AnnotateQuestion.Read(entry0.Value))).ToArray()), Threshold = value.TryGetProperty("threshold",
        out _) ? (AuthoredPresence<AuthoredThreshold>)(AuthoredThreshold.Read(value.GetProperty("threshold"))) : default
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class RequestDefinitionAuthoredChoose : RequestDefinition {
    public AuthoredPresence<RequestDefinitionAuthoredChooseBatch> Batch {
        get;
        init;
    }
    public required AuthoredQuestionText Choose {
        get;
        init;
    }
    public AuthoredPresence<AuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public AuthoredPresence<AuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public AuthoredPresence<AuthoredName> Model {
        get;
        init;
    }
    public AuthoredPresence<string> Name {
        get;
        init;
    }
    public AuthoredPresence<AuthoredPointers> On {
        get;
        init;
    }
    public AuthoredPresence<AuthoredOptions> Options {
        get;
        init;
    }
    public AuthoredPresence<AuthoredProfile> Profile {
        get;
        init;
    }
    public AuthoredPresence<AuthoredCut> Threshold {
        get;
        init;
    }
    public AuthoredPresence<long> WordingVersion {
        get;
        init;
    }
    internal new static RequestDefinitionAuthoredChoose Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Batch = value.TryGetProperty("batch", out _) ? (AuthoredPresence<RequestDefinitionAuthoredChooseBatch>)(RequestDefinitionAuthoredChooseBatch.Read(value.GetProperty("batch"))) : default,
        Choose = AuthoredQuestionText.Read(value.GetProperty("choose")), ContextSchema = value.TryGetProperty("context_schema",
        out _) ? (AuthoredPresence<AuthoredInputDeclaration>)(AuthoredInputDeclaration.Read(value.GetProperty("context_schema"))) : default,
        ItemSchema = value.TryGetProperty("item_schema", out _) ? (AuthoredPresence<AuthoredInputDeclaration>)(AuthoredInputDeclaration.Read(value.GetProperty("item_schema"))) : default,
        Model = value.TryGetProperty("model", out _) ? (AuthoredPresence<AuthoredName>)(AuthoredName.Read(value.GetProperty("model"))) : default,
        Name = value.TryGetProperty("name", out _) ? (AuthoredPresence<string>)(value.GetProperty("name").GetString()!) : default,
        On = value.TryGetProperty("on", out _) ? (AuthoredPresence<AuthoredPointers>)(AuthoredPointers.Read(value.GetProperty("on"))) : default,
        Options = value.TryGetProperty("options", out _) ? (AuthoredPresence<AuthoredOptions>)(AuthoredOptions.Read(value.GetProperty("options"))) : default,
        Profile = value.TryGetProperty("profile", out _) ? (AuthoredPresence<AuthoredProfile>)(AuthoredProfile.Read(value.GetProperty("profile"))) : default,
        Threshold = value.TryGetProperty("threshold", out _) ? (AuthoredPresence<AuthoredCut>)(AuthoredCut.Read(value.GetProperty("threshold"))) : default,
        WordingVersion = value.TryGetProperty("wording_version", out _) ? (AuthoredPresence<long>)(value.GetProperty("wording_version").GetInt64()) : default
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public abstract class RequestDefinitionAuthoredChooseBatch : AuthoredDocument {
    private protected RequestDefinitionAuthoredChooseBatch() {
    } internal static RequestDefinitionAuthoredChooseBatch Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.String && value.GetString() == "max")) return RequestDefinitionAuthoredChooseBatchMax.Read(value);
        if ((value.ValueKind == JsonValueKind.Number)) return RequestDefinitionAuthoredChooseBatchInteger.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class RequestDefinitionAuthoredChooseBatchInteger : RequestDefinitionAuthoredChooseBatch {
    public required long Value {
        get;
        init;
    }
    internal new static RequestDefinitionAuthoredChooseBatchInteger Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.GetInt64()
    };
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNumberValue(Value);
    }
}
public sealed class RequestDefinitionAuthoredChooseBatchMax : RequestDefinitionAuthoredChooseBatch {
    internal new static RequestDefinitionAuthoredChooseBatchMax Read(JsonElement value) => new() {
        ParsedDocument = value.Clone()
    };
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "max");
    }
}
public sealed class RequestDefinitionAuthoredDecide : RequestDefinition {
    public AuthoredPresence<RequestDefinitionAuthoredDecideBatch> Batch {
        get;
        init;
    }
    public AuthoredPresence<AuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public required AuthoredQuestionText Decide {
        get;
        init;
    }
    public AuthoredPresence<AuthoredCriterion> False {
        get;
        init;
    }
    public AuthoredPresence<AuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public AuthoredPresence<AuthoredName> Model {
        get;
        init;
    }
    public AuthoredPresence<string> Name {
        get;
        init;
    }
    public AuthoredPresence<AuthoredPointers> On {
        get;
        init;
    }
    public AuthoredPresence<AuthoredProfile> Profile {
        get;
        init;
    }
    public AuthoredPresence<AuthoredThreshold> Threshold {
        get;
        init;
    }
    public AuthoredPresence<AuthoredCriterion> True {
        get;
        init;
    }
    public AuthoredPresence<long> WordingVersion {
        get;
        init;
    }
    internal new static RequestDefinitionAuthoredDecide Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Batch = value.TryGetProperty("batch", out _) ? (AuthoredPresence<RequestDefinitionAuthoredDecideBatch>)(RequestDefinitionAuthoredDecideBatch.Read(value.GetProperty("batch"))) : default,
        ContextSchema = value.TryGetProperty("context_schema", out _) ? (AuthoredPresence<AuthoredInputDeclaration>)(AuthoredInputDeclaration.Read(value.GetProperty("context_schema"))) : default,
        Decide = AuthoredQuestionText.Read(value.GetProperty("decide")), False = value.TryGetProperty("false",
        out _) ? (AuthoredPresence<AuthoredCriterion>)(AuthoredCriterion.Read(value.GetProperty("false"))) : default,
        ItemSchema = value.TryGetProperty("item_schema", out _) ? (AuthoredPresence<AuthoredInputDeclaration>)(AuthoredInputDeclaration.Read(value.GetProperty("item_schema"))) : default,
        Model = value.TryGetProperty("model", out _) ? (AuthoredPresence<AuthoredName>)(AuthoredName.Read(value.GetProperty("model"))) : default,
        Name = value.TryGetProperty("name", out _) ? (AuthoredPresence<string>)(value.GetProperty("name").GetString()!) : default,
        On = value.TryGetProperty("on", out _) ? (AuthoredPresence<AuthoredPointers>)(AuthoredPointers.Read(value.GetProperty("on"))) : default,
        Profile = value.TryGetProperty("profile", out _) ? (AuthoredPresence<AuthoredProfile>)(AuthoredProfile.Read(value.GetProperty("profile"))) : default,
        Threshold = value.TryGetProperty("threshold", out _) ? (AuthoredPresence<AuthoredThreshold>)(AuthoredThreshold.Read(value.GetProperty("threshold"))) : default,
        True = value.TryGetProperty("true", out _) ? (AuthoredPresence<AuthoredCriterion>)(AuthoredCriterion.Read(value.GetProperty("true"))) : default,
        WordingVersion = value.TryGetProperty("wording_version", out _) ? (AuthoredPresence<long>)(value.GetProperty("wording_version").GetInt64()) : default
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public abstract class RequestDefinitionAuthoredDecideBatch : AuthoredDocument {
    private protected RequestDefinitionAuthoredDecideBatch() {
    } internal static RequestDefinitionAuthoredDecideBatch Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.String && value.GetString() == "max")) return RequestDefinitionAuthoredDecideBatchMax.Read(value);
        if ((value.ValueKind == JsonValueKind.Number)) return RequestDefinitionAuthoredDecideBatchInteger.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class RequestDefinitionAuthoredDecideBatchInteger : RequestDefinitionAuthoredDecideBatch {
    public required long Value {
        get;
        init;
    }
    internal new static RequestDefinitionAuthoredDecideBatchInteger Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.GetInt64()
    };
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNumberValue(Value);
    }
}
public sealed class RequestDefinitionAuthoredDecideBatchMax : RequestDefinitionAuthoredDecideBatch {
    internal new static RequestDefinitionAuthoredDecideBatchMax Read(JsonElement value) => new() {
        ParsedDocument = value.Clone()
    };
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "max");
    }
}
public sealed class RequestDefinitionAuthoredFind : RequestDefinition {
    public AuthoredPresence<AuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public required AuthoredQuestionText Find {
        get;
        init;
    }
    public AuthoredPresence<AuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public AuthoredPresence<AuthoredName> Model {
        get;
        init;
    }
    public AuthoredPresence<string> Name {
        get;
        init;
    }
    public AuthoredPresence<AuthoredPointers> On {
        get;
        init;
    }
    public AuthoredPresence<AuthoredProfile> Profile {
        get;
        init;
    }
    public AuthoredPresence<long> WordingVersion {
        get;
        init;
    }
    internal new static RequestDefinitionAuthoredFind Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), ContextSchema = value.TryGetProperty("context_schema", out _) ? (AuthoredPresence<AuthoredInputDeclaration>)(AuthoredInputDeclaration.Read(value.GetProperty("context_schema"))) : default,
        Find = AuthoredQuestionText.Read(value.GetProperty("find")), ItemSchema = value.TryGetProperty("item_schema",
        out _) ? (AuthoredPresence<AuthoredInputDeclaration>)(AuthoredInputDeclaration.Read(value.GetProperty("item_schema"))) : default,
        Model = value.TryGetProperty("model", out _) ? (AuthoredPresence<AuthoredName>)(AuthoredName.Read(value.GetProperty("model"))) : default,
        Name = value.TryGetProperty("name", out _) ? (AuthoredPresence<string>)(value.GetProperty("name").GetString()!) : default,
        On = value.TryGetProperty("on", out _) ? (AuthoredPresence<AuthoredPointers>)(AuthoredPointers.Read(value.GetProperty("on"))) : default,
        Profile = value.TryGetProperty("profile", out _) ? (AuthoredPresence<AuthoredProfile>)(AuthoredProfile.Read(value.GetProperty("profile"))) : default,
        WordingVersion = value.TryGetProperty("wording_version", out _) ? (AuthoredPresence<long>)(value.GetProperty("wording_version").GetInt64()) : default
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class RequestDefinitionAuthoredRelate : RequestDefinition {
    public AuthoredPresence<AuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public AuthoredPresence<AuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public AuthoredPresence<AuthoredName> Model {
        get;
        init;
    }
    public AuthoredPresence<string> Name {
        get;
        init;
    }
    public AuthoredPresence<AuthoredProfile> Profile {
        get;
        init;
    }
    public required RelateQuestion Relate {
        get;
        init;
    }
    public AuthoredPresence<AuthoredCut> Threshold {
        get;
        init;
    }
    public AuthoredPresence<long> WordingVersion {
        get;
        init;
    }
    internal new static RequestDefinitionAuthoredRelate Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), ContextSchema = value.TryGetProperty("context_schema", out _) ? (AuthoredPresence<AuthoredInputDeclaration>)(AuthoredInputDeclaration.Read(value.GetProperty("context_schema"))) : default,
        ItemSchema = value.TryGetProperty("item_schema", out _) ? (AuthoredPresence<AuthoredInputDeclaration>)(AuthoredInputDeclaration.Read(value.GetProperty("item_schema"))) : default,
        Model = value.TryGetProperty("model", out _) ? (AuthoredPresence<AuthoredName>)(AuthoredName.Read(value.GetProperty("model"))) : default,
        Name = value.TryGetProperty("name", out _) ? (AuthoredPresence<string>)(value.GetProperty("name").GetString()!) : default,
        Profile = value.TryGetProperty("profile", out _) ? (AuthoredPresence<AuthoredProfile>)(AuthoredProfile.Read(value.GetProperty("profile"))) : default,
        Relate = RelateQuestion.Read(value.GetProperty("relate")), Threshold = value.TryGetProperty("threshold",
        out _) ? (AuthoredPresence<AuthoredCut>)(AuthoredCut.Read(value.GetProperty("threshold"))) : default,
        WordingVersion = value.TryGetProperty("wording_version", out _) ? (AuthoredPresence<long>)(value.GetProperty("wording_version").GetInt64()) : default
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class RequestDefinitionAuthoredScore : RequestDefinition {
    public AuthoredPresence<RequestDefinitionAuthoredScoreBatch> Batch {
        get;
        init;
    }
    public AuthoredPresence<AuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public AuthoredPresence<AuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public AuthoredPresence<AuthoredLevels> Levels {
        get;
        init;
    }
    public AuthoredPresence<AuthoredName> Model {
        get;
        init;
    }
    public AuthoredPresence<string> Name {
        get;
        init;
    }
    public AuthoredPresence<AuthoredPointers> On {
        get;
        init;
    }
    public AuthoredPresence<AuthoredProfile> Profile {
        get;
        init;
    }
    public required AuthoredQuestionText Score {
        get;
        init;
    }
    public AuthoredPresence<long> WordingVersion {
        get;
        init;
    }
    internal new static RequestDefinitionAuthoredScore Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Batch = value.TryGetProperty("batch", out _) ? (AuthoredPresence<RequestDefinitionAuthoredScoreBatch>)(RequestDefinitionAuthoredScoreBatch.Read(value.GetProperty("batch"))) : default,
        ContextSchema = value.TryGetProperty("context_schema", out _) ? (AuthoredPresence<AuthoredInputDeclaration>)(AuthoredInputDeclaration.Read(value.GetProperty("context_schema"))) : default,
        ItemSchema = value.TryGetProperty("item_schema", out _) ? (AuthoredPresence<AuthoredInputDeclaration>)(AuthoredInputDeclaration.Read(value.GetProperty("item_schema"))) : default,
        Levels = value.TryGetProperty("levels", out _) ? (AuthoredPresence<AuthoredLevels>)(AuthoredLevels.Read(value.GetProperty("levels"))) : default,
        Model = value.TryGetProperty("model", out _) ? (AuthoredPresence<AuthoredName>)(AuthoredName.Read(value.GetProperty("model"))) : default,
        Name = value.TryGetProperty("name", out _) ? (AuthoredPresence<string>)(value.GetProperty("name").GetString()!) : default,
        On = value.TryGetProperty("on", out _) ? (AuthoredPresence<AuthoredPointers>)(AuthoredPointers.Read(value.GetProperty("on"))) : default,
        Profile = value.TryGetProperty("profile", out _) ? (AuthoredPresence<AuthoredProfile>)(AuthoredProfile.Read(value.GetProperty("profile"))) : default,
        Score = AuthoredQuestionText.Read(value.GetProperty("score")), WordingVersion = value.TryGetProperty("wording_version",
        out _) ? (AuthoredPresence<long>)(value.GetProperty("wording_version").GetInt64()) : default
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public abstract class RequestDefinitionAuthoredScoreBatch : AuthoredDocument {
    private protected RequestDefinitionAuthoredScoreBatch() {
    } internal static RequestDefinitionAuthoredScoreBatch Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.String && value.GetString() == "max")) return RequestDefinitionAuthoredScoreBatchMax.Read(value);
        if ((value.ValueKind == JsonValueKind.Number)) return RequestDefinitionAuthoredScoreBatchInteger.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class RequestDefinitionAuthoredScoreBatchInteger : RequestDefinitionAuthoredScoreBatch {
    public required long Value {
        get;
        init;
    }
    internal new static RequestDefinitionAuthoredScoreBatchInteger Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.GetInt64()
    };
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNumberValue(Value);
    }
}
public sealed class RequestDefinitionAuthoredScoreBatchMax : RequestDefinitionAuthoredScoreBatch {
    internal new static RequestDefinitionAuthoredScoreBatchMax Read(JsonElement value) => new() {
        ParsedDocument = value.Clone()
    };
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "max");
    }
}
public sealed class RequestDefinitionAuthoredTag : RequestDefinition {
    public AuthoredPresence<RequestDefinitionAuthoredTagBatch> Batch {
        get;
        init;
    }
    public AuthoredPresence<AuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public AuthoredPresence<AuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public AuthoredPresence<AuthoredLabels> Labels {
        get;
        init;
    }
    public AuthoredPresence<AuthoredName> Model {
        get;
        init;
    }
    public AuthoredPresence<string> Name {
        get;
        init;
    }
    public AuthoredPresence<AuthoredPointers> On {
        get;
        init;
    }
    public AuthoredPresence<AuthoredProfile> Profile {
        get;
        init;
    }
    public required AuthoredQuestionText Tag {
        get;
        init;
    }
    public AuthoredPresence<AuthoredCut> Threshold {
        get;
        init;
    }
    public AuthoredPresence<long> WordingVersion {
        get;
        init;
    }
    internal new static RequestDefinitionAuthoredTag Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Batch = value.TryGetProperty("batch", out _) ? (AuthoredPresence<RequestDefinitionAuthoredTagBatch>)(RequestDefinitionAuthoredTagBatch.Read(value.GetProperty("batch"))) : default,
        ContextSchema = value.TryGetProperty("context_schema", out _) ? (AuthoredPresence<AuthoredInputDeclaration>)(AuthoredInputDeclaration.Read(value.GetProperty("context_schema"))) : default,
        ItemSchema = value.TryGetProperty("item_schema", out _) ? (AuthoredPresence<AuthoredInputDeclaration>)(AuthoredInputDeclaration.Read(value.GetProperty("item_schema"))) : default,
        Labels = value.TryGetProperty("labels", out _) ? (AuthoredPresence<AuthoredLabels>)(AuthoredLabels.Read(value.GetProperty("labels"))) : default,
        Model = value.TryGetProperty("model", out _) ? (AuthoredPresence<AuthoredName>)(AuthoredName.Read(value.GetProperty("model"))) : default,
        Name = value.TryGetProperty("name", out _) ? (AuthoredPresence<string>)(value.GetProperty("name").GetString()!) : default,
        On = value.TryGetProperty("on", out _) ? (AuthoredPresence<AuthoredPointers>)(AuthoredPointers.Read(value.GetProperty("on"))) : default,
        Profile = value.TryGetProperty("profile", out _) ? (AuthoredPresence<AuthoredProfile>)(AuthoredProfile.Read(value.GetProperty("profile"))) : default,
        Tag = AuthoredQuestionText.Read(value.GetProperty("tag")), Threshold = value.TryGetProperty("threshold",
        out _) ? (AuthoredPresence<AuthoredCut>)(AuthoredCut.Read(value.GetProperty("threshold"))) : default,
        WordingVersion = value.TryGetProperty("wording_version", out _) ? (AuthoredPresence<long>)(value.GetProperty("wording_version").GetInt64()) : default
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public abstract class RequestDefinitionAuthoredTagBatch : AuthoredDocument {
    private protected RequestDefinitionAuthoredTagBatch() {
    } internal static RequestDefinitionAuthoredTagBatch Read(JsonElement value) {
        if ((value.ValueKind == JsonValueKind.String && value.GetString() == "max")) return RequestDefinitionAuthoredTagBatchMax.Read(value);
        if ((value.ValueKind == JsonValueKind.Number)) return RequestDefinitionAuthoredTagBatchInteger.Read(value);
        throw new JsonException("Authored data does not match the native Request schema.");
    }
}
public sealed class RequestDefinitionAuthoredTagBatchInteger : RequestDefinitionAuthoredTagBatch {
    public required long Value {
        get;
        init;
    }
    internal new static RequestDefinitionAuthoredTagBatchInteger Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), Value = value.GetInt64()
    };
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNumberValue(Value);
    }
}
public sealed class RequestDefinitionAuthoredTagBatchMax : RequestDefinitionAuthoredTagBatch {
    internal new static RequestDefinitionAuthoredTagBatchMax Read(JsonElement value) => new() {
        ParsedDocument = value.Clone()
    };
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "max");
    }
}
public sealed class RequestDefinitionRecognizeDefinition : RequestDefinition {
    public AuthoredPresence<AuthoredInputDeclaration> ContextSchema {
        get;
        init;
    }
    public AuthoredPresence<AuthoredInputDeclaration> ItemSchema {
        get;
        init;
    }
    public AuthoredPresence<AuthoredName> Model {
        get;
        init;
    }
    public AuthoredPresence<string> Name {
        get;
        init;
    }
    public AuthoredPresence<AuthoredPointers> On {
        get;
        init;
    }
    public AuthoredPresence<AuthoredProfile> Profile {
        get;
        init;
    }
    public required RecognizeQuestion Recognize {
        get;
        init;
    }
    public AuthoredPresence<AuthoredCut> RelationThreshold {
        get;
        init;
    }
    public AuthoredPresence<AuthoredCut> Threshold {
        get;
        init;
    }
    public AuthoredPresence<long> WordingVersion {
        get;
        init;
    }
    internal new static RequestDefinitionRecognizeDefinition Read(JsonElement value) => new() {
        ParsedDocument = value.Clone(), ContextSchema = value.TryGetProperty("context_schema", out _) ? (AuthoredPresence<AuthoredInputDeclaration>)(AuthoredInputDeclaration.Read(value.GetProperty("context_schema"))) : default,
        ItemSchema = value.TryGetProperty("item_schema", out _) ? (AuthoredPresence<AuthoredInputDeclaration>)(AuthoredInputDeclaration.Read(value.GetProperty("item_schema"))) : default,
        Model = value.TryGetProperty("model", out _) ? (AuthoredPresence<AuthoredName>)(AuthoredName.Read(value.GetProperty("model"))) : default,
        Name = value.TryGetProperty("name", out _) ? (AuthoredPresence<string>)(value.GetProperty("name").GetString()!) : default,
        On = value.TryGetProperty("on", out _) ? (AuthoredPresence<AuthoredPointers>)(AuthoredPointers.Read(value.GetProperty("on"))) : default,
        Profile = value.TryGetProperty("profile", out _) ? (AuthoredPresence<AuthoredProfile>)(AuthoredProfile.Read(value.GetProperty("profile"))) : default,
        Recognize = RecognizeQuestion.Read(value.GetProperty("recognize")), RelationThreshold = value.TryGetProperty("relation_threshold",
        out _) ? (AuthoredPresence<AuthoredCut>)(AuthoredCut.Read(value.GetProperty("relation_threshold"))) : default,
        Threshold = value.TryGetProperty("threshold", out _) ? (AuthoredPresence<AuthoredCut>)(AuthoredCut.Read(value.GetProperty("threshold"))) : default,
        WordingVersion = value.TryGetProperty("wording_version", out _) ? (AuthoredPresence<long>)(value.GetProperty("wording_version").GetInt64()) : default
    };
    internal override void Write(Utf8JsonWriter writer) {
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
public abstract class RequestFraming : AuthoredDocument {
    private protected RequestFraming() {
    }
}
public sealed class RequestFramingCsv : RequestFraming {
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "csv");
    }
}
public sealed class RequestFramingDocument : RequestFraming {
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "document");
    }
}
public sealed class RequestFramingJsonl : RequestFraming {
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "jsonl");
    }
}
public sealed class RequestFramingLines : RequestFraming {
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "lines");
    }
}
public sealed class RequestFramingTsv : RequestFraming {
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "tsv");
    }
}
public abstract class RequestFunction : AuthoredDocument {
    private protected RequestFunction() {
    }
}
public sealed class RequestFunctionAnnotate : RequestFunction {
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "annotate");
    }
}
public sealed class RequestFunctionChoose : RequestFunction {
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "choose");
    }
}
public sealed class RequestFunctionDecide : RequestFunction {
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "decide");
    }
}
public sealed class RequestFunctionFilter : RequestFunction {
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "filter");
    }
}
public sealed class RequestFunctionFind : RequestFunction {
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "find");
    }
}
public sealed class RequestFunctionRank : RequestFunction {
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "rank");
    }
}
public sealed class RequestFunctionRecognize : RequestFunction {
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "recognize");
    }
}
public sealed class RequestFunctionRelate : RequestFunction {
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "relate");
    }
}
public sealed class RequestFunctionScore : RequestFunction {
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "score");
    }
}
public sealed class RequestFunctionTag : RequestFunction {
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "tag");
    }
}
public abstract class RequestImage : AuthoredDocument {
    private protected RequestImage() {
    }
}
public sealed class RequestImageBytes : RequestImage {
    public required string Bytes {
        get;
        init;
    }
    public required ImageMedia Media {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class RequestImageFile : RequestImage {
    public AuthoredPresence<ImageMedia> Media {
        get;
        init;
    }
    public required string Path {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public abstract class RequestInput : AuthoredDocument {
    private protected RequestInput() {
    }
}
public sealed class RequestInputEntities : RequestInput {
    public required IReadOnlyList<RequestItem> Items {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class RequestInputFeed : RequestInput {
    public AuthoredPresence<RequestFraming> Framing {
        get;
        init;
    }
    public AuthoredPresence<IReadOnlyList<RequestImage>> Images {
        get;
        init;
    }
    public required string Name {
        get;
        init;
    }
    public AuthoredPresence<RequestReader> Reading {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class RequestInputJson : RequestInput {
    public AuthoredPresence<IReadOnlyList<RequestImage>> Images {
        get;
        init;
    }
    public required JsonElement Value {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class RequestInputRecords : RequestInput {
    public required IReadOnlyList<RequestItem> Items {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class RequestInputSource : RequestInput {
    public required RequestSource Source {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class RequestInputText : RequestInput {
    public AuthoredPresence<IReadOnlyList<RequestImage>> Images {
        get;
        init;
    }
    public required string Text {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class RequestInputUnits : RequestInput {
    public required IReadOnlyList<RequestItem> Items {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class RequestItem : AuthoredDocument {
    public AuthoredPresence<ContextSchema> Context {
        get;
        init;
    }
    public AuthoredPresence<IReadOnlyList<RecognitionExample>> Examples {
        get;
        init;
    }
    public AuthoredPresence<IReadOnlyList<RequestImage>> Images {
        get;
        init;
    }
    public AuthoredPresence<IReadOnlyList<OptionSchema>> Options {
        get;
        init;
    }
    public AuthoredPresence<RequestOriginal> Original {
        get;
        init;
    }
    public AuthoredPresence<IReadOnlyList<RecognitionSeedSpan>> SeedSpans {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class RequestOptions : AuthoredDocument {
    public AuthoredPresence<bool> Attempts {
        get;
        init;
    }
    public AuthoredPresence<RequestBatch> Batch {
        get;
        init;
    }
    public AuthoredPresence<string> Context {
        get;
        init;
    }
    public AuthoredPresence<string> ContextField {
        get;
        init;
    }
    public AuthoredPresence<long> DeadlineMs {
        get;
        init;
    }
    public AuthoredPresence<bool> Details {
        get;
        init;
    }
    public AuthoredPresence<IReadOnlyList<RecognitionExample>> Examples {
        get;
        init;
    }
    public AuthoredPresence<string> ExamplesField {
        get;
        init;
    }
    public AuthoredPresence<IReadOnlyList<string>> Field {
        get;
        init;
    }
    public AuthoredPresence<bool> FilesOnly {
        get;
        init;
    }
    public AuthoredPresence<ulong> MaxRequestsTotal {
        get;
        init;
    }
    public AuthoredPresence<RecognitionMode> Mode {
        get;
        init;
    }
    public AuthoredPresence<string> Model {
        get;
        init;
    }
    public AuthoredPresence<bool> None {
        get;
        init;
    }
    public AuthoredPresence<string> OptionsField {
        get;
        init;
    }
    public AuthoredPresence<RequestThreshold> RelationThreshold {
        get;
        init;
    }
    public AuthoredPresence<IReadOnlyList<RecognitionSeedSpan>> SeedSpans {
        get;
        init;
    }
    public AuthoredPresence<string> SeedSpansField {
        get;
        init;
    }
    public AuthoredPresence<uint> SnippetPieces {
        get;
        init;
    }
    public AuthoredPresence<RecognitionStageContext> StageContext {
        get;
        init;
    }
    public AuthoredPresence<RequestThreshold> Threshold {
        get;
        init;
    }
    public AuthoredPresence<ulong> Top {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public abstract class RequestOriginal : AuthoredDocument {
    private protected RequestOriginal() {
    }
}
public sealed class RequestOriginalJson : RequestOriginal {
    public required JsonElement Value {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class RequestOriginalText : RequestOriginal {
    public required string Text {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public abstract class RequestQuestion : AuthoredDocument {
    private protected RequestQuestion() {
    }
}
public sealed class RequestQuestionDefinition : RequestQuestion {
    public required RequestDefinition Value {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class RequestQuestionFile : RequestQuestion {
    public required string Path {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class RequestQuestionName : RequestQuestion {
    public required string Name {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class RequestQuestionReference : RequestQuestion {
    public required string Reference {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class RequestQuestionText : RequestQuestion {
    public required string Text {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class RequestReader : AuthoredDocument {
    public AuthoredPresence<SourceUnit> Unit {
        get;
        init;
    }
    public AuthoredPresence<ulong> Window {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public abstract class RequestReaderFailure : AuthoredDocument {
    private protected RequestReaderFailure() {
    }
}
public sealed class RequestReaderFailureInvalidInput : RequestReaderFailure {
    public AuthoredPresence<SessionSourceLocation> Location {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class RequestReaderFailureIo : RequestReaderFailure {
    public AuthoredPresence<SessionSourceLocation> Location {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class RequestReaderFailureProducer : RequestReaderFailure {
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class RequestReaderFailureUtf8 : RequestReaderFailure {
    public AuthoredPresence<SessionSourceLocation> Location {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class RequestSessionDescriptor : AuthoredDocument {
    public required RequestItem Item {
        get;
        init;
    }
    public AuthoredPresence<SessionSourceLocation> Location {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public sealed class RequestSource : AuthoredDocument {
    public AuthoredPresence<RequestFraming> Framing {
        get;
        init;
    }
    public AuthoredPresence<ReaderMedia> Media {
        get;
        init;
    }
    public required IReadOnlyList<string> Paths {
        get;
        init;
    }
    public AuthoredPresence<RequestReader> Reading {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public abstract class RequestThreshold : AuthoredDocument {
    private protected RequestThreshold() {
    }
}
public sealed class RequestThresholdNumber : RequestThreshold {
    public required double Value {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } writer.WriteNumberValue(Value);
    }
}
public sealed class RequestThresholdString : RequestThreshold {
    public required string Value {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, Value);
    }
}
public abstract class RequestVersion : AuthoredDocument {
    private protected RequestVersion() {
    }
}
public sealed class RequestVersionThinkthenRequest1 : RequestVersion {
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "thinkthen.request/1");
    }
}
public sealed class SessionSourceLocation : AuthoredDocument {
    public required string File {
        get;
        init;
    }
    public AuthoredPresence<ulong> FirstLine {
        get;
        init;
    }
    public AuthoredPresence<ulong> LastLine {
        get;
        init;
    }
    internal override void Write(Utf8JsonWriter writer) {
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
public abstract class SourceUnit : AuthoredDocument {
    private protected SourceUnit() {
    }
}
public sealed class SourceUnitFile : SourceUnit {
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "file");
    }
}
public sealed class SourceUnitLine : SourceUnit {
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "line");
    }
}
public sealed class SourceUnitWindow : SourceUnit {
    internal override void Write(Utf8JsonWriter writer) {
        if (ParsedDocument is {
        } parsed) {
            writer.WriteRawValue(parsed.GetRawText(), skipInputValidation: true);
            return;
        } WriteText(writer, "window");
    }
}
public static class QuestionBuilders {
    public static RequestQuestionDefinition Relate(RelateQuestion question) {
        ArgumentNullException.ThrowIfNull(question);
        return new RequestQuestionDefinition {
            Value = new RequestDefinitionAuthoredRelate {
                Relate = question
            }
        };
    }
    public static RequestQuestionDefinition Recognize(RecognizeQuestion question) {
        ArgumentNullException.ThrowIfNull(question);
        return new RequestQuestionDefinition {
            Value = new RequestDefinitionRecognizeDefinition {
                Recognize = question
            }
        };
    }
}
