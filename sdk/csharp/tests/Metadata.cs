using System.Text.Json.Serialization;
using ThinkThen;

internal static partial class Program
{
    // Compile this caller as part of the normal consumer; the separate AOT entry point
    // invokes the same metadata overloads without rooting reflection fallbacks.
    private static async Task MetadataCalls(ThinkThenClient client, Message message)
    {
        var metadata = ConsumerJson.Default.Message;
        var decision=await client.DecideWithMetadataAsync("Ready?",message,metadata);
        Successful(decision);
        Successful(await client.ChooseWithMetadataAsync("Which team?", message, ["billing", "shipping"], metadata));
        Successful(await client.TagWithMetadataAsync("Which labels?", message, ["billing"], metadata));
        Successful(await client.ScoreWithMetadataAsync("How urgent?", message, ["routine", "urgent"], metadata));
        Successful(await client.FindWithMetadataAsync("Which line?", message, metadata));
        Successful(await client.RecognizeWithMetadataAsync(message, ["person"], metadata));
        Successful(await client.RelateWithMetadataAsync(message,
            [new RelationRule { Source = "person", Target = "company", Relation = "works_for" }], metadata));
        int streamed = 0;
        await foreach (var row in client.DecideStreamWithMetadataAsync("Ready?", Records(new[] { message }), metadata))
        {
            Check(row.Position == 1 && row.Status != Status.Failed, "metadata stream lost input position");
            streamed++;
        }
        Check(streamed == 1, "metadata stream dropped the record");
        await using var yes=await Fixture.Start(Environment.GetEnvironmentVariable("THINKTHEN_LOOPBACK")!,"--guide-scenario","yes");
        await using var positive=new ThinkThenClient(new(){Backend=Backend.SystemOne,Url=new Uri(yes.Url),ApiKey=ApiKey.From("synthetic-loopback-key")});
        int filtered = 0;
        await foreach (var row in positive.FilterWithMetadataAsync("Ready?", Records(new[] { message }), metadata))
        {
            Check(ReferenceEquals(row, message), "metadata filter lost original item");
            filtered++;
        }
        Check(filtered == 1, "metadata filter dropped a row");
        int ranked = 0;
        await foreach (var row in client.RankWithMetadataAsync("How urgent?", Records(new[] { message }),
            ["routine", "urgent"], metadata))
        {
            Check(row.Item == message && !ReferenceEquals(row, message), "metadata rank lost original item");
            ranked++;
        }
        Check(ranked == 1, "metadata rank dropped the record");
        var toolbox = AnnotationQuestions.Parse("""{"ready":{"function":"decide","question":"Ready?"}}""");
        Successful(await client.AnnotateWithMetadataAsync(toolbox, message, metadata));
        Successful(await client.Toolbox(toolbox).AnnotateWithMetadataAsync(message, metadata));
        int annotated = 0;
        await foreach (var row in client.Toolbox(toolbox).AnnotateStreamWithMetadataAsync(Records(new[] { message }), metadata))
        {
            Check(row.Position == 1 && row.Status != Status.Failed, "metadata toolbox stream lost position");
            annotated++;
        }
        Check(annotated == 1, "metadata toolbox stream dropped the record");
    }
    private static void Successful<A, D>(Row<A, D> row) =>
        Check(row.Status != Status.Failed, "explicit record metadata produced a failed row");
}

[JsonSourceGenerationOptions(PropertyNamingPolicy = JsonKnownNamingPolicy.CamelCase)]
[JsonSerializable(typeof(Message))]
internal partial class ConsumerJson : JsonSerializerContext { }
