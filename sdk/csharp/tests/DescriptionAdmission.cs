using System.Text.Json.Nodes;
using ThinkThen;

internal static partial class Program
{
    private static void DescriptionAdmissionCases()
    {
        static Toolbox Typed(string function, IReadOnlyDictionary<string, string> descriptions)
        {
            string[] names = ["low", "high"];
            using IDisposable question = function switch
            {
                "choose" => ChooseQuestion.Create("Which level?", names, new() { Descriptions = descriptions }),
                "tag" => TagQuestion.Create("Which level?", names, new() { Descriptions = descriptions }),
                "score" => ScoreQuestion.Create("Which level?", names, new() { Descriptions = descriptions }),
                "rank" => RankQuestion.Create("Which level?", names, new() { Descriptions = descriptions }),
                _ => throw new Exception("unexpected test function")
            };
            ToolboxQuestion definition = question switch
            {
                ChooseQuestion choose => new AnnotationQuestion.Choose(choose),
                TagQuestion tag => new AnnotationQuestion.Tag(tag),
                ScoreQuestion score => new AnnotationQuestion.Score(score),
                RankQuestion rank => new ToolboxQuestion.Rank(rank),
                _ => throw new Exception("unexpected test question")
            };
            return Toolbox.Create(new Dictionary<string, ToolboxQuestion> { ["level"] = definition });
        }
        static ThinkThenException Refusal(Action create)
        {
            try { create(); }
            catch (ThinkThenException error) { return error; }
            throw new Exception("unknown description key was admitted");
        }
        foreach (string function in new[] { "choose", "tag", "score", "rank" })
        {
            string field = function == "choose" ? "choices" : function == "tag" ? "labels" : "levels";
            var definition = new JsonObject
            {
                ["function"] = function, ["question"] = "Which level?",
                [field] = new JsonArray(new JsonObject { ["name"] = "low", ["description"] = "" },
                    new JsonObject { ["name"] = "high", ["description"] = "Highest priority" })
            };
            using var expected = Toolbox.Parse(new JsonObject { ["level"] = definition.DeepClone() }.ToJsonString());
            using var actual = Typed(function, new Dictionary<string, string> { ["low"] = "", ["high"] = "Highest priority" });
            Check(JsonNode.DeepEquals(JsonNode.Parse(actual.ToJson()), JsonNode.Parse(expected.ToJson())),
                $"{function} changed valid or empty description metadata");
            foreach (string description in new[] { "", "Unknown level" })
            {
                var invalid = (JsonObject)definition.DeepClone();
                invalid["descriptions"] = new JsonObject { ["outside"] = description };
                var native = Refusal(() => { using var _ = Toolbox.Parse(new JsonObject { ["level"] = invalid }.ToJsonString()); });
                var typed = Refusal(() => { using var _ = Typed(function, new Dictionary<string, string>
                    { ["low"] = "", ["high"] = "Highest priority", ["outside"] = description }); });
                // The raw toolbox parser adds its collection frame around the same question diagnostic.
                string expectedMessage = native.Message.Replace("questions.[0].", "question.", StringComparison.Ordinal);
                Check(typed.Code == native.Code && typed.Message == expectedMessage && typed.HttpStatus == native.HttpStatus,
                    $"{function} description refusal differs from native admission: {typed.Code}: {typed.Message} vs {native.Code}: {native.Message}");
                Check(typed.InnerException is null, $"{function} substituted a host semantic refusal");
            }
        }
        Console.WriteLine("C# four-function description metadata and native diagnostic parity passed.");
    }
}
