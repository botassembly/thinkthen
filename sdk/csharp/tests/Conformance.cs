using System.Text.Json;
using System.Text.Json.Nodes;
using ThinkThen;
using Record = ThinkThen.Record;

internal static partial class Program
{
    private static async Task Conformance(string directory, string loopback)
    {
        foreach (string path in Directory.GetFiles(directory, "*.json").Order())
        {
            var fixture = JsonNode.Parse(await File.ReadAllTextAsync(path))!;
            await using var server = await Fixture.Start(loopback, path);
                JsonNode actual;
                var delivered = new JsonArray();
                try
                {
                    var settings = fixture["client"]!.Deserialize<ClientSettings>(JsonOptions)!;
                    using var client = new ThinkThenClient(settings with { Url = server.Url, KeySource = "THINKTHEN_API_KEY" });
                    using var stop = new CancellationTokenSource();
                    if (fixture["cancelled"]?.GetValue<bool>() == true) stop.Cancel();
                    actual = await ContractRows(client, fixture["request"]!, stop.Token, delivered);
                }
                catch (ThinkThenException error) { actual = new JsonObject { ["error"] = error.Code }; }
                catch (OperationCanceledException) when (fixture["cancelled"]?.GetValue<bool>() == true) { actual = new JsonObject { ["error"] = "local.cancelled" }; }
                if (fixture["expected_rows"] is JsonArray expectedRows)
                {
                    StripMessages(delivered);
                    Check(JsonNode.DeepEquals(delivered, expectedRows), $"{Path.GetFileName(path)} rows before terminal error: {delivered} != {expectedRows}");
                }
                var expected = fixture["expected"]!.DeepClone();
                if (fixture["request"]!["function"]!.GetValue<string>() == "filter" && expected is JsonArray filterRows)
                {
                    var records = fixture["request"]!["records"]!.AsArray();
                    expected = new JsonArray(filterRows.Select((row, index) => (row, index))
                        .Where(pair => pair.row!["answer"]?.GetValue<bool>() == true)
                        .Select(pair => records[pair.index]!.DeepClone()).ToArray());
                }
                else if (expected is JsonArray rows)
                {
                    string function = fixture["request"]!["function"]!.GetValue<string>();
                    var records = fixture["request"]!["records"]!.AsArray();
                    for (int i = 0; i < rows.Count; i++)
                    {
                        string evidence = Evidence(records[i]!, fixture["request"]!["options"]?["on"]?.GetValue<string>());
                        if (function == "annotate" && rows[i]!["answer"] is JsonObject annotations)
                        {
                            foreach (var annotation in annotations)
                            {
                                string? on = fixture["request"]!["question"]!["value"]!["questions"]?[annotation.Key]?["on"]?.GetValue<string>();
                                ProjectOffsets(annotation.Value, on is null ? evidence : Evidence(records[i]!, on));
                            }
                        }
                        else
                        {
                            ProjectOffsets(rows[i]!["answer"], evidence);
                            ProjectOffsets(rows[i]!["detail"], evidence);
                        }
                    }
                    StripMessages(actual);
                }
                Check(JsonNode.DeepEquals(actual, expected), $"{Path.GetFileName(path)}: {actual} != {expected}");
                Check(int.Parse(await server.Command("count")) == fixture["requests"]!.GetValue<int>(), $"{path}: request count");
        }
        Console.WriteLine("All shared conformance cases passed.");
    }
    private static async Task<JsonNode> ContractRows(ThinkThenClient client, JsonNode request, CancellationToken token, JsonArray output)
    {
        string function = request["function"]!.GetValue<string>();
        var value = request["question"]!["value"]!;
        var records = request["records"]!.AsArray().Select(node => node is JsonValue text && text.TryGetValue<string>(out var s)
            ? Record.Text(s) : Record.Json(JsonSerializer.SerializeToElement(node))).ToArray();
        Question question = function switch
        {
            "recognize" => Question.Kinds(value.Deserialize<string[]>()!),
            "relate" => Question.Relations(value.Deserialize<RelationRule[]>(JsonOptions)!),
            "annotate" => "unused",
            _ => value.GetValue<string>()
        };
        var options = request["options"];
        Threshold? threshold = options?["threshold"] is JsonValue cut
            ? cut.TryGetValue<string>(out var band) ? (Threshold)band : (Threshold)cut.GetValue<double>() : (Threshold?)null;
        var settings = new CallOptions { Threshold = threshold, On = options?["on"]?.GetValue<string>() };
        var source = Records(records);
        string[] Choices(string name) => options![name]!.Deserialize<string[]>()!;
        if (function == "filter")
        {
            await foreach (var record in client.FilterAsync(question, source, settings, token))
                output.Add(record.IsText ? JsonValue.Create(record.TextValue) : JsonSerializer.SerializeToNode(record.JsonValue));
            return output;
        }
        return function switch
        {
            "decide" => await Rows(client.DecideStreamAsync(question, source, settings, token), output),
            "choose" => await Rows(client.ChooseStreamAsync(question, source, Choices("options"), settings, token), output),
            "tag" => await Rows(client.TagStreamAsync(question, source, Choices("labels"), settings, token), output),
            "score" => await Rows(client.ScoreStreamAsync(question, source, Choices("levels"), settings, token), output),
            "rank" => await Rows(client.RankAsync(question, source, new() { Threshold = threshold, On = settings.On, Top = options?["top"]?.GetValue<double>() }, token), output),
            "find" => await Rows(client.FindStreamAsync(question, source, settings, token), output),
            "annotate" => await Rows(client.AnnotateStreamAsync(JsonSerializer.SerializeToElement(value), source, settings, token), output),
            "recognize" => await Rows(client.RecognizeStreamAsync(question, source, settings, token), output),
            "relate" => await Rows(client.RelateStreamAsync(question, source, settings, token), output),
            _ => throw new Exception(function)
        };
    }
    private static void StripMessages(JsonNode node)
    {
        foreach (var row in node.AsArray())
        {
            if (row?["error"] is JsonObject error) error.Remove("message");
            if (row?["answer"] is JsonObject annotation)
                foreach (var entry in annotation)
                    if (entry.Value is JsonObject outcome && outcome["error"] is JsonObject entryError) entryError.Remove("message");
        }
    }
    private static string Evidence(JsonNode record, string? pointer)
    {
        if (record is JsonValue text && text.TryGetValue<string>(out string? value)) return value;
        JsonNode? selected = record;
        if (pointer is not null)
            foreach (string segment in pointer.Split('/').Skip(1)) selected = selected?[segment.Replace("~1", "/").Replace("~0", "~")];
        return selected is JsonValue scalar && scalar.TryGetValue<string>(out value) ? value : selected?.ToJsonString() ?? "null";
    }
    private static void ProjectOffsets(JsonNode? answer, string evidence)
    {
        if (answer is JsonArray values) { foreach (var item in values) ProjectOffsets(item, evidence); }
        else if (answer is JsonObject value)
        {
            if (value["start"] is not null && value["end"] is not null)
            {
                int start = value["start"]!.GetValue<int>(), end = value["end"]!.GetValue<int>();
                var runes = evidence.EnumerateRunes().ToArray();
                value["start"] = runes.Take(start).Sum(rune => rune.Utf16SequenceLength);
                value["end"] = runes.Take(end).Sum(rune => rune.Utf16SequenceLength);
            }
            foreach (var child in value) ProjectOffsets(child.Value, evidence);
        }
    }
}
