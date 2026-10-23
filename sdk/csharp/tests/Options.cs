using System.Text.Json;
using System.Text.Json.Nodes;
using ThinkThen;
using Record = ThinkThen.Record;

internal static partial class Program
{
    private static async Task OptionCases(string path)
    {
        using var client = new ThinkThenClient(new() { Adapter = "stub" });
        foreach (var sample in JsonNode.Parse(await File.ReadAllTextAsync(path))!.AsArray())
        foreach (bool single in new[] { false, true })
        {
            var actual = await OptionRows(client, sample!["request"]!, single);
            var expected = sample["expected_error"] is JsonNode code
                ? new JsonObject { ["error"] = code.DeepClone() }
                : await OptionRows(client, sample["equivalent_request"]!, single);
            Check(JsonNode.DeepEquals(actual, expected), $"{sample["name"]}: {actual} != {expected}");
            if (sample["expected_error"] is null)
                Check(actual.AsArray().Count == sample["request"]!["records"]!.AsArray().Count, "option row count");
        }
        Console.WriteLine("Installed C# shared option combinations passed.");
    }
    private static async Task<JsonNode> OptionRows(ThinkThenClient client, JsonNode request, bool single)
    {
        var tag = request["question"]!;
        bool saved = tag["type"]!.GetValue<string>() == "toolbox";
        string function = request["function"]!.GetValue<string>();
        Question question = saved ? Question.Toolbox(JsonSerializer.SerializeToElement(tag["value"]), tag["name"]!.GetValue<string>())
            : function == "annotate" ? "unused" : tag["value"]!.GetValue<string>();
        var values = request["options"];
        var options = new CallOptions { Threshold = values?["threshold"] is JsonNode cut ? (Threshold)cut.GetValue<string>() : (Threshold?)null };
        string[]? Domain(string key) => values?[key]?.Deserialize<string[]>();
        var records = request["records"]!.AsArray().Select(record => Record.Text(record!.GetValue<string>())).ToArray();
        try
        {
            if (single && function != "rank")
            {
                var output = new JsonArray();
                foreach (var record in records)
                    output.Add(function switch
                    {
                        "choose" => Row(saved ? await client.ChooseAsync(question, record) : await client.ChooseAsync(question, record, Domain("options"), options)),
                        "tag" => Row(saved ? await client.TagAsync(question, record) : await client.TagAsync(question, record, Domain("labels"), options)),
                        "score" => Row(saved ? await client.ScoreAsync(question, record) : await client.ScoreAsync(question, record, Domain("levels"), options)),
                        "annotate" => Row(await client.AnnotateAsync(JsonSerializer.SerializeToElement(tag["value"]), record, options)),
                        _ => throw new Exception(function)
                    });
                return output;
            }
            var source = Records(records);
            return function switch
            {
                "choose" => await Rows(saved ? client.ChooseStreamAsync(question, source) : client.ChooseStreamAsync(question, source, Domain("options"), options)),
                "tag" => await Rows(saved ? client.TagStreamAsync(question, source) : client.TagStreamAsync(question, source, Domain("labels"), options)),
                "score" => await Rows(saved ? client.ScoreStreamAsync(question, source) : client.ScoreStreamAsync(question, source, Domain("levels"), options)),
                "rank" => await Rows(client.RankAsync(question, source, new() { Levels = Domain("levels"), Threshold = options.Threshold })),
                "annotate" => await Rows(client.AnnotateStreamAsync(JsonSerializer.SerializeToElement(tag["value"]), source, options)),
                _ => throw new Exception(function)
            };
        }
        catch (ThinkThenException error) { return new JsonObject { ["error"] = error.Code }; }
    }
}
