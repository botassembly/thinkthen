// Generated from the Rust-derived Request schema; do not edit.
#nullable enable
using System;
using System.Collections.Generic;
using System.IO;
using System.Text.Json;
namespace ThinkThen.Inputs;
public readonly struct InputPresence<T> {
    public bool IsPresent { get; }
    public T Value { get; }
    private InputPresence(T value) { Value = value; IsPresent = true; }
    public static implicit operator InputPresence<T>(T value) => new(value);
}
public abstract class InputDocument {
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
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); if (Batch.IsPresent) { writer.WritePropertyName("batch"); Batch.Value.Write(writer); } writer.WritePropertyName("choose"); Choose.Write(writer); if (ContextSchema.IsPresent) { writer.WritePropertyName("context_schema"); ContextSchema.Value.Write(writer); } if (ItemSchema.IsPresent) { writer.WritePropertyName("item_schema"); ItemSchema.Value.Write(writer); } if (Model.IsPresent) { writer.WritePropertyName("model"); Model.Value.Write(writer); } if (Name.IsPresent) { writer.WritePropertyName("name"); writer.WriteStringValue(Name.Value); } if (On.IsPresent) { writer.WritePropertyName("on"); On.Value.Write(writer); } if (Options.IsPresent) { writer.WritePropertyName("options"); Options.Value.Write(writer); } if (Profile.IsPresent) { writer.WritePropertyName("profile"); Profile.Value.Write(writer); } if (Threshold.IsPresent) { writer.WritePropertyName("threshold"); Threshold.Value.Write(writer); } if (WordingVersion.IsPresent) { writer.WritePropertyName("wording_version"); writer.WriteNumberValue(WordingVersion.Value); } writer.WriteEndObject(); }
}

public abstract class InputAuthoredChooseBatch : InputDocument { private protected InputAuthoredChooseBatch() {} }

public sealed class InputAuthoredChooseBatchAlternative0 : InputAuthoredChooseBatch {
public override void Write(Utf8JsonWriter writer) { writer.WriteStringValue("max"); }
}

public sealed class InputAuthoredChooseBatchAlternative1 : InputAuthoredChooseBatch {
public required long Value { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteNumberValue(Value); }
}

public abstract class InputAuthoredCriterion : InputDocument { private protected InputAuthoredCriterion() {} }

public sealed class InputAuthoredCriterionAlternative0 : InputAuthoredCriterion {
public required string Value { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStringValue(Value); }
}

public sealed class InputAuthoredCriterionAlternative1 : InputAuthoredCriterion {
public required JsonElement Value { get; init; }
public override void Write(Utf8JsonWriter writer) { Value.WriteTo(writer); }
}

public sealed class InputAuthoredCriterionAlternative2 : InputAuthoredCriterion {
public required IReadOnlyList<JsonElement> Value { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartArray(); foreach (var item in Value) { item.WriteTo(writer); } writer.WriteEndArray(); }
}

public sealed class InputAuthoredCriterionAlternative3 : InputAuthoredCriterion {
public required JsonElement Value { get; init; }
public override void Write(Utf8JsonWriter writer) { Value.WriteTo(writer); }
}

public abstract class InputAuthoredCut : InputDocument { private protected InputAuthoredCut() {} }

public sealed class InputAuthoredCutAlternative0 : InputAuthoredCut {
public required double Value { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteNumberValue(Value); }
}

public sealed class InputAuthoredCutAlternative1 : InputAuthoredCut {
public required string Value { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStringValue(Value); }
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
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); if (Batch.IsPresent) { writer.WritePropertyName("batch"); Batch.Value.Write(writer); } if (ContextSchema.IsPresent) { writer.WritePropertyName("context_schema"); ContextSchema.Value.Write(writer); } writer.WritePropertyName("decide"); Decide.Write(writer); if (False.IsPresent) { writer.WritePropertyName("false"); False.Value.Write(writer); } if (ItemSchema.IsPresent) { writer.WritePropertyName("item_schema"); ItemSchema.Value.Write(writer); } if (Model.IsPresent) { writer.WritePropertyName("model"); Model.Value.Write(writer); } if (Name.IsPresent) { writer.WritePropertyName("name"); writer.WriteStringValue(Name.Value); } if (On.IsPresent) { writer.WritePropertyName("on"); On.Value.Write(writer); } if (Profile.IsPresent) { writer.WritePropertyName("profile"); Profile.Value.Write(writer); } if (Threshold.IsPresent) { writer.WritePropertyName("threshold"); Threshold.Value.Write(writer); } if (True.IsPresent) { writer.WritePropertyName("true"); True.Value.Write(writer); } if (WordingVersion.IsPresent) { writer.WritePropertyName("wording_version"); writer.WriteNumberValue(WordingVersion.Value); } writer.WriteEndObject(); }
}

public abstract class InputAuthoredDecideBatch : InputDocument { private protected InputAuthoredDecideBatch() {} }

public sealed class InputAuthoredDecideBatchAlternative0 : InputAuthoredDecideBatch {
public override void Write(Utf8JsonWriter writer) { writer.WriteStringValue("max"); }
}

public sealed class InputAuthoredDecideBatchAlternative1 : InputAuthoredDecideBatch {
public required long Value { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteNumberValue(Value); }
}

public sealed class InputAuthoredDescription : InputDocument {
public required InputAuthoredDescriptionValue Value { get; init; }
public override void Write(Utf8JsonWriter writer) { Value.Write(writer); }
}

public abstract class InputAuthoredDescriptionValue : InputDocument { private protected InputAuthoredDescriptionValue() {} }

public sealed class InputAuthoredDescriptionValueAlternative0 : InputAuthoredDescriptionValue {
public required string Value { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStringValue(Value); }
}

public sealed class InputAuthoredDescriptionValueAlternative1 : InputAuthoredDescriptionValue {
public required JsonElement Value { get; init; }
public override void Write(Utf8JsonWriter writer) { Value.WriteTo(writer); }
}

public sealed class InputAuthoredDescriptionValueAlternative2 : InputAuthoredDescriptionValue {
public required IReadOnlyList<JsonElement> Value { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartArray(); foreach (var item in Value) { item.WriteTo(writer); } writer.WriteEndArray(); }
}

public sealed class InputAuthoredDescriptionValueAlternative3 : InputAuthoredDescriptionValue {
public required JsonElement Value { get; init; }
public override void Write(Utf8JsonWriter writer) { Value.WriteTo(writer); }
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
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); if (ContextSchema.IsPresent) { writer.WritePropertyName("context_schema"); ContextSchema.Value.Write(writer); } writer.WritePropertyName("find"); Find.Write(writer); if (ItemSchema.IsPresent) { writer.WritePropertyName("item_schema"); ItemSchema.Value.Write(writer); } if (Model.IsPresent) { writer.WritePropertyName("model"); Model.Value.Write(writer); } if (Name.IsPresent) { writer.WritePropertyName("name"); writer.WriteStringValue(Name.Value); } if (On.IsPresent) { writer.WritePropertyName("on"); On.Value.Write(writer); } if (Profile.IsPresent) { writer.WritePropertyName("profile"); Profile.Value.Write(writer); } if (WordingVersion.IsPresent) { writer.WritePropertyName("wording_version"); writer.WriteNumberValue(WordingVersion.Value); } writer.WriteEndObject(); }
}

public abstract class InputAuthoredInputDeclaration : InputDocument { private protected InputAuthoredInputDeclaration() {} }

public sealed class InputAuthoredInputDeclarationObject : InputAuthoredInputDeclaration {
public required IReadOnlyDictionary<string, InputAuthoredInputProperty> Properties { get; init; }
public InputPresence<IReadOnlyList<string>> Required { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("properties"); writer.WriteStartObject(); foreach (var entry in Properties) { writer.WritePropertyName(entry.Key); entry.Value.Write(writer); } writer.WriteEndObject(); if (Required.IsPresent) { writer.WritePropertyName("required"); writer.WriteStartArray(); foreach (var item in Required.Value) { writer.WriteStringValue(item); } writer.WriteEndArray(); } writer.WritePropertyName("type"); writer.WriteStringValue("object"); writer.WriteEndObject(); }
}

public sealed class InputAuthoredInputDeclarationString : InputAuthoredInputDeclaration {
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("type"); writer.WriteStringValue("string"); writer.WriteEndObject(); }
}

public abstract class InputAuthoredInputProperty : InputDocument { private protected InputAuthoredInputProperty() {} }

public sealed class InputAuthoredInputPropertyArray : InputAuthoredInputProperty {
public required InputAuthoredInputPropertyArrayItems Items { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("items"); Items.Write(writer); writer.WritePropertyName("type"); writer.WriteStringValue("array"); writer.WriteEndObject(); }
}

public sealed class InputAuthoredInputPropertyArrayItems : InputDocument {
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("type"); writer.WriteStringValue("string"); writer.WriteEndObject(); }
}

public sealed class InputAuthoredInputPropertyBoolean : InputAuthoredInputProperty {
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("type"); writer.WriteStringValue("boolean"); writer.WriteEndObject(); }
}

public sealed class InputAuthoredInputPropertyNumber : InputAuthoredInputProperty {
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("type"); writer.WriteStringValue("number"); writer.WriteEndObject(); }
}

public sealed class InputAuthoredInputPropertyString : InputAuthoredInputProperty {
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("type"); writer.WriteStringValue("string"); writer.WriteEndObject(); }
}

public abstract class InputAuthoredLabels : InputDocument { private protected InputAuthoredLabels() {} }

public sealed class InputAuthoredLabelsAlternative0 : InputAuthoredLabels {
public required IReadOnlyList<InputAuthoredName> Value { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartArray(); foreach (var item in Value) { item.Write(writer); } writer.WriteEndArray(); }
}

public sealed class InputAuthoredLabelsAlternative1 : InputAuthoredLabels {
public required IReadOnlyDictionary<string, InputAuthoredDescription> Value { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); foreach (var entry in Value) { writer.WritePropertyName(entry.Key); entry.Value.Write(writer); } writer.WriteEndObject(); }
}

public abstract class InputAuthoredLevels : InputDocument { private protected InputAuthoredLevels() {} }

public sealed class InputAuthoredLevelsAlternative0 : InputAuthoredLevels {
public required IReadOnlyList<InputAuthoredName> Value { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartArray(); foreach (var item in Value) { item.Write(writer); } writer.WriteEndArray(); }
}

public sealed class InputAuthoredLevelsAlternative1 : InputAuthoredLevels {
public required IReadOnlyDictionary<string, InputAuthoredCriterion> Value { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); foreach (var entry in Value) { writer.WritePropertyName(entry.Key); entry.Value.Write(writer); } writer.WriteEndObject(); }
}

public sealed class InputAuthoredName : InputDocument {
public required string Value { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStringValue(Value); }
}

public abstract class InputAuthoredOptions : InputDocument { private protected InputAuthoredOptions() {} }

public sealed class InputAuthoredOptionsAlternative0 : InputAuthoredOptions {
public required IReadOnlyList<InputAuthoredName> Value { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartArray(); foreach (var item in Value) { item.Write(writer); } writer.WriteEndArray(); }
}

public sealed class InputAuthoredOptionsAlternative1 : InputAuthoredOptions {
public required IReadOnlyDictionary<string, InputAuthoredDescription> Value { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); foreach (var entry in Value) { writer.WritePropertyName(entry.Key); entry.Value.Write(writer); } writer.WriteEndObject(); }
}

public abstract class InputAuthoredPointers : InputDocument { private protected InputAuthoredPointers() {} }

public sealed class InputAuthoredPointersAlternative0 : InputAuthoredPointers {
public required string Value { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStringValue(Value); }
}

public sealed class InputAuthoredPointersAlternative1 : InputAuthoredPointers {
public required IReadOnlyList<string> Value { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartArray(); foreach (var item in Value) { writer.WriteStringValue(item); } writer.WriteEndArray(); }
}

public sealed class InputAuthoredProfile : InputDocument {
public required string Value { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStringValue(Value); }
}

public abstract class InputAuthoredQuestionText : InputDocument { private protected InputAuthoredQuestionText() {} }

public sealed class InputAuthoredQuestionTextAlternative0 : InputAuthoredQuestionText {
public required string Value { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStringValue(Value); }
}

public sealed class InputAuthoredQuestionTextAlternative1 : InputAuthoredQuestionText {
public required JsonElement Value { get; init; }
public override void Write(Utf8JsonWriter writer) { Value.WriteTo(writer); }
}

public sealed class InputAuthoredQuestionTextAlternative2 : InputAuthoredQuestionText {
public required IReadOnlyList<JsonElement> Value { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartArray(); foreach (var item in Value) { item.WriteTo(writer); } writer.WriteEndArray(); }
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
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); if (ContextSchema.IsPresent) { writer.WritePropertyName("context_schema"); ContextSchema.Value.Write(writer); } if (ItemSchema.IsPresent) { writer.WritePropertyName("item_schema"); ItemSchema.Value.Write(writer); } if (Model.IsPresent) { writer.WritePropertyName("model"); Model.Value.Write(writer); } if (Name.IsPresent) { writer.WritePropertyName("name"); writer.WriteStringValue(Name.Value); } if (Profile.IsPresent) { writer.WritePropertyName("profile"); Profile.Value.Write(writer); } writer.WritePropertyName("relate"); Relate.Write(writer); if (Threshold.IsPresent) { writer.WritePropertyName("threshold"); Threshold.Value.Write(writer); } writer.WritePropertyName("version"); writer.WriteNumberValue(1); if (WordingVersion.IsPresent) { writer.WritePropertyName("wording_version"); writer.WriteNumberValue(WordingVersion.Value); } writer.WriteEndObject(); }
}

public sealed class InputAuthoredRelateRelate : InputDocument {
public InputPresence<InputAuthoredRelateRelateFields> Fields { get; init; }
public required IReadOnlyList<InputAuthoredRelation> Relations { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); if (Fields.IsPresent) { writer.WritePropertyName("fields"); Fields.Value.Write(writer); } writer.WritePropertyName("relations"); writer.WriteStartArray(); foreach (var item in Relations) { item.Write(writer); } writer.WriteEndArray(); writer.WriteEndObject(); }
}

public sealed class InputAuthoredRelateRelateFields : InputDocument {
public required string Kind { get; init; }
public required string Name { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("kind"); writer.WriteStringValue(Kind); writer.WritePropertyName("name"); writer.WriteStringValue(Name); writer.WriteEndObject(); }
}

public sealed class InputAuthoredRelation : InputDocument {
public InputPresence<bool> Either { get; init; }
public required InputAuthoredName Name { get; init; }
public InputPresence<InputAuthoredName> Reads { get; init; }
public InputPresence<bool> Single { get; init; }
public InputPresence<InputAuthoredName> Source { get; init; }
public InputPresence<InputAuthoredName> Target { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); if (Either.IsPresent) { writer.WritePropertyName("either"); writer.WriteBooleanValue(Either.Value); } writer.WritePropertyName("name"); Name.Write(writer); if (Reads.IsPresent) { writer.WritePropertyName("reads"); Reads.Value.Write(writer); } if (Single.IsPresent) { writer.WritePropertyName("single"); writer.WriteBooleanValue(Single.Value); } if (Source.IsPresent) { writer.WritePropertyName("source"); Source.Value.Write(writer); } if (Target.IsPresent) { writer.WritePropertyName("target"); Target.Value.Write(writer); } writer.WriteEndObject(); }
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
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); if (Batch.IsPresent) { writer.WritePropertyName("batch"); Batch.Value.Write(writer); } if (ContextSchema.IsPresent) { writer.WritePropertyName("context_schema"); ContextSchema.Value.Write(writer); } if (ItemSchema.IsPresent) { writer.WritePropertyName("item_schema"); ItemSchema.Value.Write(writer); } if (Levels.IsPresent) { writer.WritePropertyName("levels"); Levels.Value.Write(writer); } if (Model.IsPresent) { writer.WritePropertyName("model"); Model.Value.Write(writer); } if (Name.IsPresent) { writer.WritePropertyName("name"); writer.WriteStringValue(Name.Value); } if (On.IsPresent) { writer.WritePropertyName("on"); On.Value.Write(writer); } if (Profile.IsPresent) { writer.WritePropertyName("profile"); Profile.Value.Write(writer); } writer.WritePropertyName("score"); Score.Write(writer); if (WordingVersion.IsPresent) { writer.WritePropertyName("wording_version"); writer.WriteNumberValue(WordingVersion.Value); } writer.WriteEndObject(); }
}

public abstract class InputAuthoredScoreBatch : InputDocument { private protected InputAuthoredScoreBatch() {} }

public sealed class InputAuthoredScoreBatchAlternative0 : InputAuthoredScoreBatch {
public override void Write(Utf8JsonWriter writer) { writer.WriteStringValue("max"); }
}

public sealed class InputAuthoredScoreBatchAlternative1 : InputAuthoredScoreBatch {
public required long Value { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteNumberValue(Value); }
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
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); if (Batch.IsPresent) { writer.WritePropertyName("batch"); Batch.Value.Write(writer); } if (ContextSchema.IsPresent) { writer.WritePropertyName("context_schema"); ContextSchema.Value.Write(writer); } if (ItemSchema.IsPresent) { writer.WritePropertyName("item_schema"); ItemSchema.Value.Write(writer); } if (Labels.IsPresent) { writer.WritePropertyName("labels"); Labels.Value.Write(writer); } if (Model.IsPresent) { writer.WritePropertyName("model"); Model.Value.Write(writer); } if (Name.IsPresent) { writer.WritePropertyName("name"); writer.WriteStringValue(Name.Value); } if (On.IsPresent) { writer.WritePropertyName("on"); On.Value.Write(writer); } if (Profile.IsPresent) { writer.WritePropertyName("profile"); Profile.Value.Write(writer); } writer.WritePropertyName("tag"); Tag.Write(writer); if (Threshold.IsPresent) { writer.WritePropertyName("threshold"); Threshold.Value.Write(writer); } if (WordingVersion.IsPresent) { writer.WritePropertyName("wording_version"); writer.WriteNumberValue(WordingVersion.Value); } writer.WriteEndObject(); }
}

public abstract class InputAuthoredTagBatch : InputDocument { private protected InputAuthoredTagBatch() {} }

public sealed class InputAuthoredTagBatchAlternative0 : InputAuthoredTagBatch {
public override void Write(Utf8JsonWriter writer) { writer.WriteStringValue("max"); }
}

public sealed class InputAuthoredTagBatchAlternative1 : InputAuthoredTagBatch {
public required long Value { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteNumberValue(Value); }
}

public abstract class InputAuthoredThreshold : InputDocument { private protected InputAuthoredThreshold() {} }

public sealed class InputAuthoredThresholdAlternative0 : InputAuthoredThreshold {
public required double Value { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteNumberValue(Value); }
}

public sealed class InputAuthoredThresholdAlternative1 : InputAuthoredThreshold {
public required string Value { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStringValue(Value); }
}

public abstract class InputContextSchema : InputDocument { private protected InputContextSchema() {} }

public sealed class InputContextSchemaAlternative0 : InputContextSchema {
public required string Value { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStringValue(Value); }
}

public sealed class InputContextSchemaAlternative1 : InputContextSchema {
public required JsonElement Value { get; init; }
public override void Write(Utf8JsonWriter writer) { Value.WriteTo(writer); }
}

public abstract class InputImageMedia : InputDocument { private protected InputImageMedia() {} }

public sealed class InputImageMediaAlternative0 : InputImageMedia {
public override void Write(Utf8JsonWriter writer) { writer.WriteStringValue("image/jpeg"); }
}

public sealed class InputImageMediaAlternative1 : InputImageMedia {
public override void Write(Utf8JsonWriter writer) { writer.WriteStringValue("image/png"); }
}

public sealed class InputOptionSchema : InputDocument {
public InputPresence<JsonElement> Description { get; init; }
public required string Name { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); if (Description.IsPresent) { writer.WritePropertyName("description"); Description.Value.WriteTo(writer); } writer.WritePropertyName("name"); writer.WriteStringValue(Name); writer.WriteEndObject(); }
}

public abstract class InputReaderMedia : InputDocument { private protected InputReaderMedia() {} }

public sealed class InputReaderMediaAlternative0 : InputReaderMedia {
public override void Write(Utf8JsonWriter writer) { writer.WriteStringValue("text"); }
}

public sealed class InputReaderMediaAlternative1 : InputReaderMedia {
public override void Write(Utf8JsonWriter writer) { writer.WriteStringValue("image"); }
}

public abstract class InputRecognitionExample : InputDocument { private protected InputRecognitionExample() {} }

public sealed class InputRecognitionExampleEntity : InputDocument {
public required long End { get; init; }
public required string Kind { get; init; }
public required long Start { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("end"); writer.WriteNumberValue(End); writer.WritePropertyName("kind"); writer.WriteStringValue(Kind); writer.WritePropertyName("start"); writer.WriteNumberValue(Start); writer.WriteEndObject(); }
}

public sealed class InputRecognitionExampleText : InputDocument {
public required IReadOnlyList<InputRecognitionExampleEntity> Entities { get; init; }
public InputPresence<IReadOnlyList<string>> Kinds { get; init; }
public required string Text { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("entities"); writer.WriteStartArray(); foreach (var item in Entities) { item.Write(writer); } writer.WriteEndArray(); if (Kinds.IsPresent) { writer.WritePropertyName("kinds"); writer.WriteStartArray(); foreach (var item in Kinds.Value) { writer.WriteStringValue(item); } writer.WriteEndArray(); } writer.WritePropertyName("text"); writer.WriteStringValue(Text); writer.WriteEndObject(); }
}

public sealed class InputRecognitionExampleAlternative0 : InputRecognitionExample {
public required string Value { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStringValue(Value); }
}

public sealed class InputRecognitionExampleAlternative1 : InputRecognitionExample {
public required InputRecognitionExampleText Value { get; init; }
public override void Write(Utf8JsonWriter writer) { Value.Write(writer); }
}

public abstract class InputRecognitionMode : InputDocument { private protected InputRecognitionMode() {} }

public sealed class InputRecognitionModeAlternative0 : InputRecognitionMode {
public override void Write(Utf8JsonWriter writer) { writer.WriteStringValue("whole"); }
}

public sealed class InputRecognitionModeAlternative1 : InputRecognitionMode {
public override void Write(Utf8JsonWriter writer) { writer.WriteStringValue("boundary_only"); }
}

public sealed class InputRecognitionSeedSpan : InputDocument {
public required long End { get; init; }
public InputPresence<string> Kind { get; init; }
public required long Start { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("end"); writer.WriteNumberValue(End); if (Kind.IsPresent) { writer.WritePropertyName("kind"); writer.WriteStringValue(Kind.Value); } writer.WritePropertyName("start"); writer.WriteNumberValue(Start); writer.WriteEndObject(); }
}

public sealed class InputRecognitionStageContext : InputDocument {
public InputPresence<string> Boundary { get; init; }
public InputPresence<string> KindEdge { get; init; }
public InputPresence<string> Relation { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); if (Boundary.IsPresent) { writer.WritePropertyName("boundary"); writer.WriteStringValue(Boundary.Value); } if (KindEdge.IsPresent) { writer.WritePropertyName("kind_edge"); writer.WriteStringValue(KindEdge.Value); } if (Relation.IsPresent) { writer.WritePropertyName("relation"); writer.WriteStringValue(Relation.Value); } writer.WriteEndObject(); }
}

public sealed class InputRequest : InputDocument {
public required InputRequestCall Call { get; init; }
public required InputRequestVersion Schema { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("call"); Call.Write(writer); writer.WritePropertyName("schema"); Schema.Write(writer); writer.WriteEndObject(); }
}

public abstract class InputRequestBatch : InputDocument { private protected InputRequestBatch() {} }

public sealed class InputRequestBatchAlternative0 : InputRequestBatch {
public required long Value { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteNumberValue(Value); }
}

public sealed class InputRequestBatchAlternative1 : InputRequestBatch {
public required string Value { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStringValue(Value); }
}

public abstract class InputRequestCall : InputDocument { private protected InputRequestCall() {} }

public sealed class InputRequestCallAnnotate : InputRequestCall {
public required InputRequestInput Input { get; init; }
public InputPresence<InputRequestOptions> Options { get; init; }
public required InputRequestQuestion Question { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("function"); writer.WriteStringValue("annotate"); writer.WritePropertyName("input"); Input.Write(writer); if (Options.IsPresent) { writer.WritePropertyName("options"); Options.Value.Write(writer); } writer.WritePropertyName("question"); Question.Write(writer); writer.WriteEndObject(); }
}

public sealed class InputRequestCallChoose : InputRequestCall {
public required InputRequestInput Input { get; init; }
public InputPresence<InputRequestOptions> Options { get; init; }
public required InputRequestQuestion Question { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("function"); writer.WriteStringValue("choose"); writer.WritePropertyName("input"); Input.Write(writer); if (Options.IsPresent) { writer.WritePropertyName("options"); Options.Value.Write(writer); } writer.WritePropertyName("question"); Question.Write(writer); writer.WriteEndObject(); }
}

public sealed class InputRequestCallDecide : InputRequestCall {
public required InputRequestInput Input { get; init; }
public InputPresence<InputRequestOptions> Options { get; init; }
public required InputRequestQuestion Question { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("function"); writer.WriteStringValue("decide"); writer.WritePropertyName("input"); Input.Write(writer); if (Options.IsPresent) { writer.WritePropertyName("options"); Options.Value.Write(writer); } writer.WritePropertyName("question"); Question.Write(writer); writer.WriteEndObject(); }
}

public sealed class InputRequestCallFilter : InputRequestCall {
public required InputRequestInput Input { get; init; }
public InputPresence<InputRequestOptions> Options { get; init; }
public required InputRequestQuestion Question { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("function"); writer.WriteStringValue("filter"); writer.WritePropertyName("input"); Input.Write(writer); if (Options.IsPresent) { writer.WritePropertyName("options"); Options.Value.Write(writer); } writer.WritePropertyName("question"); Question.Write(writer); writer.WriteEndObject(); }
}

public sealed class InputRequestCallFind : InputRequestCall {
public required InputRequestInput Input { get; init; }
public InputPresence<InputRequestOptions> Options { get; init; }
public required InputRequestQuestion Question { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("function"); writer.WriteStringValue("find"); writer.WritePropertyName("input"); Input.Write(writer); if (Options.IsPresent) { writer.WritePropertyName("options"); Options.Value.Write(writer); } writer.WritePropertyName("question"); Question.Write(writer); writer.WriteEndObject(); }
}

public sealed class InputRequestCallRank : InputRequestCall {
public required InputRequestInput Input { get; init; }
public InputPresence<InputRequestOptions> Options { get; init; }
public required InputRequestQuestion Question { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("function"); writer.WriteStringValue("rank"); writer.WritePropertyName("input"); Input.Write(writer); if (Options.IsPresent) { writer.WritePropertyName("options"); Options.Value.Write(writer); } writer.WritePropertyName("question"); Question.Write(writer); writer.WriteEndObject(); }
}

public sealed class InputRequestCallRecognize : InputRequestCall {
public required InputRequestInput Input { get; init; }
public InputPresence<InputRequestOptions> Options { get; init; }
public required InputRequestQuestion Question { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("function"); writer.WriteStringValue("recognize"); writer.WritePropertyName("input"); Input.Write(writer); if (Options.IsPresent) { writer.WritePropertyName("options"); Options.Value.Write(writer); } writer.WritePropertyName("question"); Question.Write(writer); writer.WriteEndObject(); }
}

public sealed class InputRequestCallRelate : InputRequestCall {
public required InputRequestInput Input { get; init; }
public InputPresence<InputRequestOptions> Options { get; init; }
public required InputRequestQuestion Question { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("function"); writer.WriteStringValue("relate"); writer.WritePropertyName("input"); Input.Write(writer); if (Options.IsPresent) { writer.WritePropertyName("options"); Options.Value.Write(writer); } writer.WritePropertyName("question"); Question.Write(writer); writer.WriteEndObject(); }
}

public sealed class InputRequestCallScore : InputRequestCall {
public required InputRequestInput Input { get; init; }
public InputPresence<InputRequestOptions> Options { get; init; }
public required InputRequestQuestion Question { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("function"); writer.WriteStringValue("score"); writer.WritePropertyName("input"); Input.Write(writer); if (Options.IsPresent) { writer.WritePropertyName("options"); Options.Value.Write(writer); } writer.WritePropertyName("question"); Question.Write(writer); writer.WriteEndObject(); }
}

public sealed class InputRequestCallTag : InputRequestCall {
public required InputRequestInput Input { get; init; }
public InputPresence<InputRequestOptions> Options { get; init; }
public required InputRequestQuestion Question { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("function"); writer.WriteStringValue("tag"); writer.WritePropertyName("input"); Input.Write(writer); if (Options.IsPresent) { writer.WritePropertyName("options"); Options.Value.Write(writer); } writer.WritePropertyName("question"); Question.Write(writer); writer.WriteEndObject(); }
}

public abstract class InputRequestDefinition : InputDocument { private protected InputRequestDefinition() {} }

public sealed class InputRequestDefinitionAlternative0 : InputRequestDefinition {
public required InputAuthoredDecide Value { get; init; }
public override void Write(Utf8JsonWriter writer) { Value.Write(writer); }
}

public sealed class InputRequestDefinitionAlternative1 : InputRequestDefinition {
public required InputAuthoredChoose Value { get; init; }
public override void Write(Utf8JsonWriter writer) { Value.Write(writer); }
}

public sealed class InputRequestDefinitionAlternative2 : InputRequestDefinition {
public required InputAuthoredTag Value { get; init; }
public override void Write(Utf8JsonWriter writer) { Value.Write(writer); }
}

public sealed class InputRequestDefinitionAlternative3 : InputRequestDefinition {
public required InputAuthoredScore Value { get; init; }
public override void Write(Utf8JsonWriter writer) { Value.Write(writer); }
}

public sealed class InputRequestDefinitionAlternative4 : InputRequestDefinition {
public required InputAuthoredRelate Value { get; init; }
public override void Write(Utf8JsonWriter writer) { Value.Write(writer); }
}

public sealed class InputRequestDefinitionAlternative5 : InputRequestDefinition {
public required InputAuthoredFind Value { get; init; }
public override void Write(Utf8JsonWriter writer) { Value.Write(writer); }
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
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); if (ContextSchema.IsPresent) { writer.WritePropertyName("context_schema"); ContextSchema.Value.Write(writer); } if (ItemSchema.IsPresent) { writer.WritePropertyName("item_schema"); ItemSchema.Value.Write(writer); } if (Model.IsPresent) { writer.WritePropertyName("model"); Model.Value.Write(writer); } if (Name.IsPresent) { writer.WritePropertyName("name"); writer.WriteStringValue(Name.Value); } if (On.IsPresent) { writer.WritePropertyName("on"); On.Value.Write(writer); } if (Profile.IsPresent) { writer.WritePropertyName("profile"); Profile.Value.Write(writer); } writer.WritePropertyName("recognize"); Recognize.Write(writer); if (RelationThreshold.IsPresent) { writer.WritePropertyName("relation_threshold"); RelationThreshold.Value.Write(writer); } if (Threshold.IsPresent) { writer.WritePropertyName("threshold"); Threshold.Value.Write(writer); } writer.WritePropertyName("version"); writer.WriteNumberValue(1); if (WordingVersion.IsPresent) { writer.WritePropertyName("wording_version"); writer.WriteNumberValue(WordingVersion.Value); } writer.WriteEndObject(); }
}

public sealed class InputRequestDefinitionAlternative6Recognize : InputDocument {
public InputPresence<InputAuthoredQuestionText> EntityDefinition { get; init; }
public InputPresence<InputAuthoredQuestionText> Instructions { get; init; }
public InputPresence<IReadOnlyDictionary<string, InputAuthoredDescription>> Kinds { get; init; }
public InputPresence<InputRecognitionMode> Mode { get; init; }
public InputPresence<IReadOnlyList<InputAuthoredRelation>> Relations { get; init; }
public InputPresence<long> SnippetPieces { get; init; }
public InputPresence<InputRecognitionStageContext> StageContext { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); if (EntityDefinition.IsPresent) { writer.WritePropertyName("entity_definition"); EntityDefinition.Value.Write(writer); } if (Instructions.IsPresent) { writer.WritePropertyName("instructions"); Instructions.Value.Write(writer); } if (Kinds.IsPresent) { writer.WritePropertyName("kinds"); writer.WriteStartObject(); foreach (var entry in Kinds.Value) { writer.WritePropertyName(entry.Key); entry.Value.Write(writer); } writer.WriteEndObject(); } if (Mode.IsPresent) { writer.WritePropertyName("mode"); Mode.Value.Write(writer); } if (Relations.IsPresent) { writer.WritePropertyName("relations"); writer.WriteStartArray(); foreach (var item in Relations.Value) { item.Write(writer); } writer.WriteEndArray(); } if (SnippetPieces.IsPresent) { writer.WritePropertyName("snippet_pieces"); writer.WriteNumberValue(SnippetPieces.Value); } if (StageContext.IsPresent) { writer.WritePropertyName("stage_context"); StageContext.Value.Write(writer); } writer.WriteEndObject(); }
}

public sealed class InputRequestDefinitionAlternative7 : InputRequestDefinition {
public InputPresence<JsonElement> Batch { get; init; }
public InputPresence<InputAuthoredProfile> Profile { get; init; }
public required IReadOnlyDictionary<string, InputRequestDefinitionAlternative7QuestionsEntry> Questions { get; init; }
public InputPresence<InputAuthoredThreshold> Threshold { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); if (Batch.IsPresent) { writer.WritePropertyName("batch"); Batch.Value.WriteTo(writer); } if (Profile.IsPresent) { writer.WritePropertyName("profile"); Profile.Value.Write(writer); } writer.WritePropertyName("questions"); writer.WriteStartObject(); foreach (var entry in Questions) { writer.WritePropertyName(entry.Key); entry.Value.Write(writer); } writer.WriteEndObject(); if (Threshold.IsPresent) { writer.WritePropertyName("threshold"); Threshold.Value.Write(writer); } writer.WritePropertyName("version"); writer.WriteNumberValue(1); writer.WriteEndObject(); }
}

public abstract class InputRequestDefinitionAlternative7QuestionsEntry : InputDocument { private protected InputRequestDefinitionAlternative7QuestionsEntry() {} }

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
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); if (ContextSchema.IsPresent) { writer.WritePropertyName("context_schema"); ContextSchema.Value.Write(writer); } writer.WritePropertyName("decide"); Decide.Write(writer); if (False.IsPresent) { writer.WritePropertyName("false"); False.Value.Write(writer); } if (ItemSchema.IsPresent) { writer.WritePropertyName("item_schema"); ItemSchema.Value.Write(writer); } if (Name.IsPresent) { writer.WritePropertyName("name"); writer.WriteStringValue(Name.Value); } if (On.IsPresent) { writer.WritePropertyName("on"); On.Value.Write(writer); } if (Threshold.IsPresent) { writer.WritePropertyName("threshold"); Threshold.Value.Write(writer); } if (True.IsPresent) { writer.WritePropertyName("true"); True.Value.Write(writer); } if (WordingVersion.IsPresent) { writer.WritePropertyName("wording_version"); writer.WriteNumberValue(WordingVersion.Value); } writer.WriteEndObject(); }
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
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("choose"); Choose.Write(writer); if (ContextSchema.IsPresent) { writer.WritePropertyName("context_schema"); ContextSchema.Value.Write(writer); } if (ItemSchema.IsPresent) { writer.WritePropertyName("item_schema"); ItemSchema.Value.Write(writer); } if (Name.IsPresent) { writer.WritePropertyName("name"); writer.WriteStringValue(Name.Value); } if (On.IsPresent) { writer.WritePropertyName("on"); On.Value.Write(writer); } if (Options.IsPresent) { writer.WritePropertyName("options"); Options.Value.Write(writer); } if (Threshold.IsPresent) { writer.WritePropertyName("threshold"); Threshold.Value.Write(writer); } if (WordingVersion.IsPresent) { writer.WritePropertyName("wording_version"); writer.WriteNumberValue(WordingVersion.Value); } writer.WriteEndObject(); }
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
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); if (ContextSchema.IsPresent) { writer.WritePropertyName("context_schema"); ContextSchema.Value.Write(writer); } if (ItemSchema.IsPresent) { writer.WritePropertyName("item_schema"); ItemSchema.Value.Write(writer); } if (Labels.IsPresent) { writer.WritePropertyName("labels"); Labels.Value.Write(writer); } if (Name.IsPresent) { writer.WritePropertyName("name"); writer.WriteStringValue(Name.Value); } if (On.IsPresent) { writer.WritePropertyName("on"); On.Value.Write(writer); } writer.WritePropertyName("tag"); Tag.Write(writer); if (Threshold.IsPresent) { writer.WritePropertyName("threshold"); Threshold.Value.Write(writer); } if (WordingVersion.IsPresent) { writer.WritePropertyName("wording_version"); writer.WriteNumberValue(WordingVersion.Value); } writer.WriteEndObject(); }
}

public sealed class InputRequestDefinitionAlternative7QuestionsEntryAlternative3 : InputRequestDefinitionAlternative7QuestionsEntry {
public InputPresence<InputAuthoredInputDeclaration> ContextSchema { get; init; }
public InputPresence<InputAuthoredInputDeclaration> ItemSchema { get; init; }
public InputPresence<InputAuthoredLevels> Levels { get; init; }
public InputPresence<string> Name { get; init; }
public InputPresence<InputAuthoredPointers> On { get; init; }
public required InputAuthoredQuestionText Score { get; init; }
public InputPresence<long> WordingVersion { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); if (ContextSchema.IsPresent) { writer.WritePropertyName("context_schema"); ContextSchema.Value.Write(writer); } if (ItemSchema.IsPresent) { writer.WritePropertyName("item_schema"); ItemSchema.Value.Write(writer); } if (Levels.IsPresent) { writer.WritePropertyName("levels"); Levels.Value.Write(writer); } if (Name.IsPresent) { writer.WritePropertyName("name"); writer.WriteStringValue(Name.Value); } if (On.IsPresent) { writer.WritePropertyName("on"); On.Value.Write(writer); } writer.WritePropertyName("score"); Score.Write(writer); if (WordingVersion.IsPresent) { writer.WritePropertyName("wording_version"); writer.WriteNumberValue(WordingVersion.Value); } writer.WriteEndObject(); }
}

public abstract class InputRequestFraming : InputDocument { private protected InputRequestFraming() {} }

public sealed class InputRequestFramingAlternative0 : InputRequestFraming {
public override void Write(Utf8JsonWriter writer) { writer.WriteStringValue("document"); }
}

public sealed class InputRequestFramingAlternative1 : InputRequestFraming {
public override void Write(Utf8JsonWriter writer) { writer.WriteStringValue("lines"); }
}

public sealed class InputRequestFramingAlternative2 : InputRequestFraming {
public override void Write(Utf8JsonWriter writer) { writer.WriteStringValue("jsonl"); }
}

public sealed class InputRequestFramingAlternative3 : InputRequestFraming {
public override void Write(Utf8JsonWriter writer) { writer.WriteStringValue("csv"); }
}

public sealed class InputRequestFramingAlternative4 : InputRequestFraming {
public override void Write(Utf8JsonWriter writer) { writer.WriteStringValue("tsv"); }
}

public abstract class InputRequestImage : InputDocument { private protected InputRequestImage() {} }

public sealed class InputRequestImageBytes : InputRequestImage {
public required string Bytes { get; init; }
public required InputImageMedia Media { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("bytes"); writer.WriteStringValue(Bytes); writer.WritePropertyName("kind"); writer.WriteStringValue("bytes"); writer.WritePropertyName("media"); Media.Write(writer); writer.WriteEndObject(); }
}

public sealed class InputRequestImageFile : InputRequestImage {
public InputPresence<InputImageMedia> Media { get; init; }
public required string Path { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("kind"); writer.WriteStringValue("file"); if (Media.IsPresent) { writer.WritePropertyName("media"); Media.Value.Write(writer); } writer.WritePropertyName("path"); writer.WriteStringValue(Path); writer.WriteEndObject(); }
}

public abstract class InputRequestInput : InputDocument { private protected InputRequestInput() {} }

public sealed class InputRequestInputEntities : InputRequestInput {
public required IReadOnlyList<InputRequestItem> Items { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("items"); writer.WriteStartArray(); foreach (var item in Items) { item.Write(writer); } writer.WriteEndArray(); writer.WritePropertyName("kind"); writer.WriteStringValue("entities"); writer.WriteEndObject(); }
}

public sealed class InputRequestInputFeed : InputRequestInput {
public InputPresence<InputRequestFraming> Framing { get; init; }
public InputPresence<IReadOnlyList<InputRequestImage>> Images { get; init; }
public required string Name { get; init; }
public InputPresence<InputRequestReader> Reading { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); if (Framing.IsPresent) { writer.WritePropertyName("framing"); Framing.Value.Write(writer); } if (Images.IsPresent) { writer.WritePropertyName("images"); writer.WriteStartArray(); foreach (var item in Images.Value) { item.Write(writer); } writer.WriteEndArray(); } writer.WritePropertyName("kind"); writer.WriteStringValue("feed"); writer.WritePropertyName("name"); writer.WriteStringValue(Name); if (Reading.IsPresent) { writer.WritePropertyName("reading"); Reading.Value.Write(writer); } writer.WriteEndObject(); }
}

public sealed class InputRequestInputJson : InputRequestInput {
public InputPresence<IReadOnlyList<InputRequestImage>> Images { get; init; }
public required JsonElement Value { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); if (Images.IsPresent) { writer.WritePropertyName("images"); writer.WriteStartArray(); foreach (var item in Images.Value) { item.Write(writer); } writer.WriteEndArray(); } writer.WritePropertyName("kind"); writer.WriteStringValue("json"); writer.WritePropertyName("value"); Value.WriteTo(writer); writer.WriteEndObject(); }
}

public sealed class InputRequestInputRecords : InputRequestInput {
public required IReadOnlyList<InputRequestItem> Items { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("items"); writer.WriteStartArray(); foreach (var item in Items) { item.Write(writer); } writer.WriteEndArray(); writer.WritePropertyName("kind"); writer.WriteStringValue("records"); writer.WriteEndObject(); }
}

public sealed class InputRequestInputSource : InputRequestInput {
public required InputRequestSource Source { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("kind"); writer.WriteStringValue("source"); writer.WritePropertyName("source"); Source.Write(writer); writer.WriteEndObject(); }
}

public sealed class InputRequestInputText : InputRequestInput {
public InputPresence<IReadOnlyList<InputRequestImage>> Images { get; init; }
public required string Text { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); if (Images.IsPresent) { writer.WritePropertyName("images"); writer.WriteStartArray(); foreach (var item in Images.Value) { item.Write(writer); } writer.WriteEndArray(); } writer.WritePropertyName("kind"); writer.WriteStringValue("text"); writer.WritePropertyName("text"); writer.WriteStringValue(Text); writer.WriteEndObject(); }
}

public sealed class InputRequestInputUnits : InputRequestInput {
public required IReadOnlyList<InputRequestItem> Items { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("items"); writer.WriteStartArray(); foreach (var item in Items) { item.Write(writer); } writer.WriteEndArray(); writer.WritePropertyName("kind"); writer.WriteStringValue("units"); writer.WriteEndObject(); }
}

public sealed class InputRequestItem : InputDocument {
public InputPresence<InputContextSchema> Context { get; init; }
public InputPresence<IReadOnlyList<InputRecognitionExample>> Examples { get; init; }
public InputPresence<IReadOnlyList<InputRequestImage>> Images { get; init; }
public InputPresence<IReadOnlyList<InputOptionSchema>> Options { get; init; }
public InputPresence<InputRequestOriginal> Original { get; init; }
public InputPresence<IReadOnlyList<InputRecognitionSeedSpan>> SeedSpans { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); if (Context.IsPresent) { writer.WritePropertyName("context"); Context.Value.Write(writer); } if (Examples.IsPresent) { writer.WritePropertyName("examples"); writer.WriteStartArray(); foreach (var item in Examples.Value) { item.Write(writer); } writer.WriteEndArray(); } if (Images.IsPresent) { writer.WritePropertyName("images"); writer.WriteStartArray(); foreach (var item in Images.Value) { item.Write(writer); } writer.WriteEndArray(); } if (Options.IsPresent) { writer.WritePropertyName("options"); writer.WriteStartArray(); foreach (var item in Options.Value) { item.Write(writer); } writer.WriteEndArray(); } if (Original.IsPresent) { writer.WritePropertyName("original"); Original.Value.Write(writer); } if (SeedSpans.IsPresent) { writer.WritePropertyName("seed_spans"); writer.WriteStartArray(); foreach (var item in SeedSpans.Value) { item.Write(writer); } writer.WriteEndArray(); } writer.WriteEndObject(); }
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
public InputPresence<long> MaxRequestsTotal { get; init; }
public InputPresence<InputRecognitionMode> Mode { get; init; }
public InputPresence<string> Model { get; init; }
public InputPresence<bool> None { get; init; }
public InputPresence<string> OptionsField { get; init; }
public InputPresence<InputRequestThreshold> RelationThreshold { get; init; }
public InputPresence<IReadOnlyList<InputRecognitionSeedSpan>> SeedSpans { get; init; }
public InputPresence<string> SeedSpansField { get; init; }
public InputPresence<long> SnippetPieces { get; init; }
public InputPresence<InputRecognitionStageContext> StageContext { get; init; }
public InputPresence<InputRequestThreshold> Threshold { get; init; }
public InputPresence<long> Top { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); if (Attempts.IsPresent) { writer.WritePropertyName("attempts"); writer.WriteBooleanValue(Attempts.Value); } if (Batch.IsPresent) { writer.WritePropertyName("batch"); Batch.Value.Write(writer); } if (Context.IsPresent) { writer.WritePropertyName("context"); writer.WriteStringValue(Context.Value); } if (ContextField.IsPresent) { writer.WritePropertyName("context_field"); writer.WriteStringValue(ContextField.Value); } if (DeadlineMs.IsPresent) { writer.WritePropertyName("deadline_ms"); writer.WriteNumberValue(DeadlineMs.Value); } if (Details.IsPresent) { writer.WritePropertyName("details"); writer.WriteBooleanValue(Details.Value); } if (Examples.IsPresent) { writer.WritePropertyName("examples"); writer.WriteStartArray(); foreach (var item in Examples.Value) { item.Write(writer); } writer.WriteEndArray(); } if (ExamplesField.IsPresent) { writer.WritePropertyName("examples_field"); writer.WriteStringValue(ExamplesField.Value); } if (Field.IsPresent) { writer.WritePropertyName("field"); writer.WriteStartArray(); foreach (var item in Field.Value) { writer.WriteStringValue(item); } writer.WriteEndArray(); } if (FilesOnly.IsPresent) { writer.WritePropertyName("files_only"); writer.WriteBooleanValue(FilesOnly.Value); } if (MaxRequestsTotal.IsPresent) { writer.WritePropertyName("max_requests_total"); writer.WriteNumberValue(MaxRequestsTotal.Value); } if (Mode.IsPresent) { writer.WritePropertyName("mode"); Mode.Value.Write(writer); } if (Model.IsPresent) { writer.WritePropertyName("model"); writer.WriteStringValue(Model.Value); } if (None.IsPresent) { writer.WritePropertyName("none"); writer.WriteBooleanValue(None.Value); } if (OptionsField.IsPresent) { writer.WritePropertyName("options_field"); writer.WriteStringValue(OptionsField.Value); } if (RelationThreshold.IsPresent) { writer.WritePropertyName("relation_threshold"); RelationThreshold.Value.Write(writer); } if (SeedSpans.IsPresent) { writer.WritePropertyName("seed_spans"); writer.WriteStartArray(); foreach (var item in SeedSpans.Value) { item.Write(writer); } writer.WriteEndArray(); } if (SeedSpansField.IsPresent) { writer.WritePropertyName("seed_spans_field"); writer.WriteStringValue(SeedSpansField.Value); } if (SnippetPieces.IsPresent) { writer.WritePropertyName("snippet_pieces"); writer.WriteNumberValue(SnippetPieces.Value); } if (StageContext.IsPresent) { writer.WritePropertyName("stage_context"); StageContext.Value.Write(writer); } if (Threshold.IsPresent) { writer.WritePropertyName("threshold"); Threshold.Value.Write(writer); } if (Top.IsPresent) { writer.WritePropertyName("top"); writer.WriteNumberValue(Top.Value); } writer.WriteEndObject(); }
}

public abstract class InputRequestOriginal : InputDocument { private protected InputRequestOriginal() {} }

public sealed class InputRequestOriginalJson : InputRequestOriginal {
public required JsonElement Value { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("kind"); writer.WriteStringValue("json"); writer.WritePropertyName("value"); Value.WriteTo(writer); writer.WriteEndObject(); }
}

public sealed class InputRequestOriginalText : InputRequestOriginal {
public required string Text { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("kind"); writer.WriteStringValue("text"); writer.WritePropertyName("text"); writer.WriteStringValue(Text); writer.WriteEndObject(); }
}

public abstract class InputRequestQuestion : InputDocument { private protected InputRequestQuestion() {} }

public sealed class InputRequestQuestionDefinition : InputRequestQuestion {
public required InputRequestDefinition Value { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("kind"); writer.WriteStringValue("definition"); writer.WritePropertyName("value"); Value.Write(writer); writer.WriteEndObject(); }
}

public sealed class InputRequestQuestionFile : InputRequestQuestion {
public required string Path { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("kind"); writer.WriteStringValue("file"); writer.WritePropertyName("path"); writer.WriteStringValue(Path); writer.WriteEndObject(); }
}

public sealed class InputRequestQuestionName : InputRequestQuestion {
public required string Name { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("kind"); writer.WriteStringValue("name"); writer.WritePropertyName("name"); writer.WriteStringValue(Name); writer.WriteEndObject(); }
}

public sealed class InputRequestQuestionReference : InputRequestQuestion {
public required string Reference { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("kind"); writer.WriteStringValue("reference"); writer.WritePropertyName("reference"); writer.WriteStringValue(Reference); writer.WriteEndObject(); }
}

public sealed class InputRequestQuestionText : InputRequestQuestion {
public required string Text { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("kind"); writer.WriteStringValue("text"); writer.WritePropertyName("text"); writer.WriteStringValue(Text); writer.WriteEndObject(); }
}

public sealed class InputRequestReader : InputDocument {
public InputPresence<InputSourceUnit> Unit { get; init; }
public InputPresence<long> Window { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); if (Unit.IsPresent) { writer.WritePropertyName("unit"); Unit.Value.Write(writer); } if (Window.IsPresent) { writer.WritePropertyName("window"); writer.WriteNumberValue(Window.Value); } writer.WriteEndObject(); }
}

public sealed class InputRequestSessionDescriptor : InputDocument {
public required InputRequestItem Item { get; init; }
public InputPresence<InputSessionSourceLocation> Location { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("item"); Item.Write(writer); if (Location.IsPresent) { writer.WritePropertyName("location"); Location.Value.Write(writer); } writer.WriteEndObject(); }
}

public sealed class InputRequestSource : InputDocument {
public InputPresence<InputReaderMedia> Media { get; init; }
public required IReadOnlyList<string> Paths { get; init; }
public InputPresence<InputRequestReader> Reading { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); if (Media.IsPresent) { writer.WritePropertyName("media"); Media.Value.Write(writer); } writer.WritePropertyName("paths"); writer.WriteStartArray(); foreach (var item in Paths) { writer.WriteStringValue(item); } writer.WriteEndArray(); if (Reading.IsPresent) { writer.WritePropertyName("reading"); Reading.Value.Write(writer); } writer.WriteEndObject(); }
}

public abstract class InputRequestThreshold : InputDocument { private protected InputRequestThreshold() {} }

public sealed class InputRequestThresholdAlternative0 : InputRequestThreshold {
public required double Value { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteNumberValue(Value); }
}

public sealed class InputRequestThresholdAlternative1 : InputRequestThreshold {
public required string Value { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStringValue(Value); }
}

public abstract class InputRequestVersion : InputDocument { private protected InputRequestVersion() {} }

public sealed class InputRequestVersionAlternative0 : InputRequestVersion {
public override void Write(Utf8JsonWriter writer) { writer.WriteStringValue("thinkthen.request/1"); }
}

public sealed class InputSessionSourceLocation : InputDocument {
public required string File { get; init; }
public InputPresence<long> FirstLine { get; init; }
public InputPresence<long> LastLine { get; init; }
public override void Write(Utf8JsonWriter writer) { writer.WriteStartObject(); writer.WritePropertyName("file"); writer.WriteStringValue(File); if (FirstLine.IsPresent) { writer.WritePropertyName("first_line"); writer.WriteNumberValue(FirstLine.Value); } if (LastLine.IsPresent) { writer.WritePropertyName("last_line"); writer.WriteNumberValue(LastLine.Value); } writer.WriteEndObject(); }
}

public abstract class InputSourceUnit : InputDocument { private protected InputSourceUnit() {} }

public sealed class InputSourceUnitAlternative0 : InputSourceUnit {
public override void Write(Utf8JsonWriter writer) { writer.WriteStringValue("line"); }
}

public sealed class InputSourceUnitAlternative1 : InputSourceUnit {
public override void Write(Utf8JsonWriter writer) { writer.WriteStringValue("window"); }
}

public sealed class InputSourceUnitAlternative2 : InputSourceUnit {
public override void Write(Utf8JsonWriter writer) { writer.WriteStringValue("file"); }
}
