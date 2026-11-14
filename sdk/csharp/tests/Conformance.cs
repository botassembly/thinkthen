using System.Text.Json;
using System.Text.Json.Nodes;
using ThinkThen;

internal static partial class Program
{
    private static async Task AdmissionCases(string directory)
    {
        JsonArray samples = JsonNode.Parse(await File.ReadAllTextAsync(Path.Combine(directory, "public-contract", "usage-admission.json")))!.AsArray();
        int count = 0;
        foreach (var sample in samples)
        {
            if (sample!["kind"]!.GetValue<string>() != "admission") continue;
            string name = sample["name"]!.GetValue<string>();
            string boundary = sample["boundary"]!.GetValue<string>();
            string expected = sample["message"]!.GetValue<string>();
            JsonNode input = sample["input"]!;
            string route;
            switch (boundary)
            {
                case "settings":
                    route = "direct: JsonSerializer.Deserialize<ThinkThenOptions> admits raw settings";
                    break;
                case "toolbox":
                    route = "direct: Toolbox.FromJson admits the saved toolbox unchanged";
                    break;
                case "question":
                    input = new JsonObject { ["entries"] = new JsonObject { ["case"] = new JsonObject { ["question"] = input.DeepClone() } } };
                    Check(expected.StartsWith("question.", StringComparison.Ordinal), "unexpected question diagnostic context");
                    expected = "toolbox.entries.[0].question." + expected["question.".Length..];
                    route = "public projection: raw question inside a saved Toolbox.FromJson entry";
                    break;
                default: throw new Exception($"Unclassified admission boundary: {boundary}");
            }
            try
            {
                if (boundary == "settings") _ = JsonSerializer.Deserialize<ThinkThenOptions>(input.ToJsonString(), JsonOptions);
                else _ = Toolbox.FromJson(input.ToJsonString());
                throw new Exception($"Accepted admission case: {name}");
            }
            catch (ThinkThenException error)
            {
                Check(JsonNode.DeepEquals(JsonSerializer.SerializeToNode(error.ErrorCode, JsonOptions), sample["code"]), $"admission code: {name}");
                Check(error.Message == expected, $"admission message: {name}: {error.Message} != {expected}");
                Check(!error.Message.Contains("secret-placeholder", StringComparison.Ordinal), $"admission leaked a secret: {name}");
            }
            Console.WriteLine($"C# admission: {name}: {route}");
            count++;
        }
        Check(count > 0, "No C# admission cases classified");
    }

    // Feature: installed C# questions execute the shared option boundaries.
    private static async Task SharedOptions(string path)
    {
        var samples = JsonNode.Parse(await File.ReadAllTextAsync(path))!.AsArray();
        foreach (var sample in samples)
        {
            ThinkThenOptions settings = sample!["client"]!.Deserialize<ThinkThenOptions>(JsonOptions)!;
            await using var client = new ThinkThenClient(settings);
            JsonNode request = sample["request"]!;
            if (sample["expected_error"] is JsonNode code)
                await Code(code.Deserialize<ErrorCode>(JsonOptions), () =>
                {
                    if (request["question"]!["function"]!.GetValue<string>() == "annotate")
                    {
                        var rejected = AnnotationQuestions.Parse(request["question"]!["questions"]!.ToJsonString());
                    }
                    else
                    {
                        var rejected = Toolbox.Parse(new JsonObject { ["case"] = request["question"]!.DeepClone() }.ToJsonString());
                    }
                    return Task.CompletedTask;
                });
            else
            {
                JsonNode actual = await ContractRows(client,request,default,new());
                JsonNode equivalent = await ContractRows(client,sample["equivalent_request"]!,default,new());
                Check(JsonNode.DeepEquals(WithoutMessages(actual),WithoutMessages(ExpectedProjection(request,sample["expected"]!))), $"shared C# options changed: {sample["name"]}");
                Check(JsonNode.DeepEquals(actual,equivalent), "equivalent typed options changed the rows");
            }
        }
    }

    private static async Task Conformance(string directory, string loopback)
    {
        JsonNode manifest = JsonNode.Parse(await File.ReadAllTextAsync(Path.Combine(directory, "manifest.json")))!;
        foreach (var backend in manifest["backends"]!.AsArray())
        foreach (var name in manifest["cases"]!.AsArray())
        {
            string path = Path.Combine(directory, name!.GetValue<string>());
            JsonNode fixture = JsonNode.Parse(await File.ReadAllTextAsync(path))!;
            JsonNode request = fixture["request"]!;
            await using var server = await Fixture.Start(loopback, path);
            JsonObject settings = manifest["client"]!.DeepClone().AsObject();
            if (fixture["client"] is JsonObject overrides)
                foreach (var field in overrides) settings[field.Key] = field.Value?.DeepClone();
            settings["backend"] = backend!.DeepClone();
            settings["url"] = server.Url;
            ThinkThenClient Client()
            {
                ThinkThenOptions options = settings.Deserialize<ThinkThenOptions>(JsonOptions)!;
                options.ApiKey = ApiKey.From("synthetic-loopback-key");
                return new ThinkThenClient(options);
            }
            using var stop = new CancellationTokenSource();
            bool cancelled = fixture["cancelled"]?.GetValue<bool>() ?? false;
            if (cancelled) stop.Cancel();
            if (fixture["expected"] is JsonObject failure)
            {
                var output = new JsonArray();
                try
                {
                    await using var rejected = Client();
                    await ContractRows(rejected, request, stop.Token, output);
                    throw new Exception($"Expected admission failure: {name}");
                }
                catch (ThinkThenException error)
                {
                    Check(output.Count == 0, "admission returned rows");
                    Check(JsonNode.DeepEquals(JsonSerializer.SerializeToNode(error.ErrorCode, JsonOptions), failure["error"]), "admission error code");
                    Check(error.HttpStatus == failure["http_status"]?.GetValue<int>(), "call error HTTP status");
                }
                catch (OperationCanceledException error)
                {
                    Check(cancelled && output.Count == 0 && error.CancellationToken == stop.Token, "pre-cancelled call lost its token or returned rows");
                    Check(failure["error"]!.GetValue<string>() == "local.cancelled", "unexpected cancellation");
                }
                Check(int.Parse(await server.Command("count")) == fixture["requests"]!.GetValue<int>(), "admission sent a request");
                continue;
            }
            await using var client = Client();
            JsonNode actual = await ContractRows(client, request, stop.Token, new());
            var checkedRows=actual is JsonArray array ? array : actual["failed"]!.AsArray();
            foreach (var row in checkedRows)
            {
                Check(row!["source"]!["backend"]!.GetValue<string>() == backend.GetValue<string>(), "row source backend");
                Check(row["source"]!["model"]!.GetValue<string>() == settings["model"]!.GetValue<string>(), "row source model");
                Check(row["source"]!["question_identity"]!.GetValue<string>().Length > 0, "row source identity");
                Check(row["source"]!["name"]?.GetValue<string>() == request["name"]?.GetValue<string>(), "execution name changed");
                if (row["source"]!["name"] is JsonNode executionName)
                    row["source"] = new JsonObject { ["name"] = executionName.DeepClone() };
                else row.AsObject().Remove("source");
            }
            JsonNode expected = ExpectedProjection(request,fixture["expected"]!);
            Check(JsonNode.DeepEquals(WithoutMessages(actual), WithoutMessages(expected)),
                $"{backend}/{name}: {actual} != {expected}");
            int requests = int.Parse(await server.Command("count"));
            int finiteRequests = fixture["requests"]!.GetValue<int>();
            Check(requests == finiteRequests,
                $"{backend}/{name}: sent {requests} requests, expected exactly {finiteRequests}");
        }
    }

    private static async Task<JsonNode> ContractRows(
        ThinkThenClient client, JsonNode request, CancellationToken token, JsonArray output)
    {
        JsonNode question = request["question"]!;
        string function = question["function"]!.GetValue<string>();
        // This driver maps host arguments only. Shared admission owns validation.
        JsonElement[] records = request["records"]!.AsArray()
            .Select(node => JsonSerializer.SerializeToElement(node)).ToArray();
        var source = Records(records, token);
        string? executionName = request["name"]?.GetValue<string>();
        string entryName = "case";
        Toolbox? toolbox = function == "annotate" ? null : Toolbox.Parse(
            new JsonObject { [entryName] = question.DeepClone() }.ToJsonString(), token);
        switch (function)
        {
            case "decide":
                return await Rows(client.DecideStreamAsync(toolbox!.Decide(entryName), source, name: executionName, cancellationToken: token), output);
            case "choose":
                return await Rows(client.ChooseStreamAsync(toolbox!.Choose(entryName), source, name: executionName, cancellationToken: token), output);
            case "tag":
                return await Rows(client.TagStreamAsync(toolbox!.Tag(entryName), source, name: executionName, cancellationToken: token), output);
            case "score":
                return await Rows(client.ScoreStreamAsync(toolbox!.Score(entryName), source, name: executionName, cancellationToken: token), output);
            case "find":
                return await Rows(client.FindStreamAsync(toolbox!.Find(entryName), source, name: executionName, cancellationToken: token), output);
            case "recognize":
                return await Rows(client.RecognizeStreamAsync(toolbox!.Recognize(entryName), source, name: executionName, cancellationToken: token), output);
            case "relate":
                return await Rows(client.RelateStreamAsync(toolbox!.Relate(entryName), source, name: executionName, cancellationToken: token), output);
            case "annotate":
                Pointer? field = question["field"] is JsonNode pointer ? Pointer.From(pointer.GetValue<string>()) : null;
                return await Rows(client.AnnotateStreamAsync(AnnotationQuestions.Parse(question["questions"]!.ToJsonString(), token), source,
                    new() { Field = field }, name: executionName, cancellationToken: token), output);
            case "filter":
                var projection=new JsonObject { ["kept"]=new JsonArray(),["failed"]=new JsonArray() };
                try
                {
                    await foreach(var item in client.FilterAsync(toolbox!.Filter(entryName),source,name:executionName,cancellationToken:token))
                        projection["kept"]!.AsArray().Add(JsonNode.Parse(item.GetRawText()));
                }
                catch(FailedException error)
                {
                    foreach(var failure in error.Rows)
                    {
                        Check(((JsonElement)failure.Item!).GetRawText()==records[checked((int)failure.Position-1)].GetRawText(),"failed filter rows retain the original record value");
                        projection["failed"]!.AsArray().Add(Row((Row<bool,JsonElement>)failure));
                    }
                }
                return projection;
            case "rank":
                await foreach(var row in client.RankAsync(toolbox!.Rank(entryName),source,name:executionName,cancellationToken:token))
                {
                    Check(row.Item.GetRawText()==records[checked((int)row.Position-1)].GetRawText(),"rank joins originals by input position");
                    output.Add(Row(row));
                }
                return output;
            default: throw new Exception($"unknown canonical function {function}");
        }
    }

    private static JsonNode ExpectedProjection(JsonNode request,JsonNode expected)
    {
        if(request["question"]!["function"]!.GetValue<string>()!="filter"||expected is not JsonArray rows) return expected.DeepClone();
        var result=new JsonObject { ["kept"]=new JsonArray(),["failed"]=new JsonArray() };
        foreach(var row in rows)
        {
            if(row!["status"]!.GetValue<string>()=="failed") result["failed"]!.AsArray().Add(row.DeepClone());
            else if(row["answer"]!.GetValue<bool>()) result["kept"]!.AsArray().Add(request["records"]![checked((int)row["position"]!.GetValue<ulong>()-1)]?.DeepClone());
        }
        return result;
    }

    private static JsonNode WithoutMessages(JsonNode value)
    {
        JsonNode copy = value.DeepClone();
        void Visit(JsonNode? node)
        {
            if (node is JsonArray array) foreach (var child in array) Visit(child);
            if (node is not JsonObject obj) return;
            if (obj["error"] is JsonObject error) error.Remove("message");
            foreach (var child in obj) Visit(child.Value);
        }
        Visit(copy);
        return copy;
    }
}
