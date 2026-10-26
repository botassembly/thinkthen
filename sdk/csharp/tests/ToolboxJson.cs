using System.Text.Json.Nodes;
using ThinkThen;

internal static partial class Program
{
    private static async Task SavedToolbox(string directory)
    {
        string fixture = await File.ReadAllTextAsync(Path.Combine(directory, "questions.json"));
        var loaded = Toolbox.FromJson(fixture);
        Check(JsonNode.DeepEquals(JsonNode.Parse(loaded.ToJson()), JsonNode.Parse(fixture)), "saved toolbox changed");
        Check(loaded.Filter("ready").Text == "Ready?", "saved filter lookup");
        _ = loaded.Relate("names");
        await using var client = new ThinkThenClient(new() { Backend = Backend.Stub });
        var original = ChooseQuestion.Create("Which topic?", new[] { "billing", "shipping" }, new() { Threshold = Threshold.Cut(0.7), Evidence = Pointer.From("/body") });
        var record = new { body = "Bill" };
        Check(JsonNode.DeepEquals(Row(await client.ChooseAsync(loaded.Choose("topic"), record)), Row(await client.ChooseAsync(original, record))), "loaded execution changed");
        var saved = Toolbox.Create(new Dictionary<string, ToolboxQuestion> { ["topic"] = new AnnotationQuestion.Choose(original) });
        Check(Toolbox.FromJson(saved.ToJson()).Choose("topic").Choices.SequenceEqual(original.Choices), "constructed save lost choices");
        await Code(ErrorCode.UsageName, () => Task.FromResult(loaded.Choose("missing")));
        foreach (var malformed in JsonNode.Parse(await File.ReadAllTextAsync(Path.Combine(directory, "malformed.json")))!.AsArray())
            await Code(ErrorCode.UsageQuestion, () => Task.FromResult(Toolbox.FromJson(malformed!.GetValue<string>())));
    }
}
