using System.Text.Json.Serialization;
using ThinkThen;

internal static partial class Program
{
    // Compile this caller as part of the normal consumer; the separate AOT entry point
    // invokes the same metadata overloads without rooting reflection fallbacks.
    private static async Task MetadataCalls(ThinkThenClient client, Message message)
    {
        var metadata = ConsumerJson.Default.Message;
        Successful(await client.DecideAsync("Ready?", message, metadata));
        Successful(await client.ChooseAsync("Which team?", message, ["billing", "shipping"], metadata));
        Successful(await client.TagAsync("Which labels?", message, ["billing"], metadata));
        Successful(await client.ScoreAsync("How urgent?", message, ["routine", "urgent"], metadata));
        Successful(await client.FindAsync("Which line?", message, metadata));
        Successful(await client.RecognizeAsync(message, ["person"], metadata));
        Successful(await client.RelateAsync(message,
            [new RelationRule { Source = "person", Target = "company", Relation = "works_for" }], metadata));
        int streamed = 0;
        await foreach (var row in client.DecideStreamAsync("Ready?", Records(new[] { message }), metadata))
        {
            Check(row.Position == 1 && row.Status != RowStatus.Failed, "metadata stream lost input position");
            streamed++;
        }
        Check(streamed == 1, "metadata stream dropped the record");
        int filtered = 0;
        await foreach (var row in client.FilterAsync("Ready?", Records(new[] { message }), metadata))
        {
            Check(ReferenceEquals(row.Item, message) && row.Row.Status != RowStatus.Failed, "metadata filter lost original item");
            filtered++;
        }
        Check(filtered == 1, "metadata filter dropped a row");
        int ranked = 0;
        await foreach (var row in client.RankAsync("How urgent?", Records(new[] { message }),
            ["routine", "urgent"], metadata))
        {
            Check(ReferenceEquals(row.Item, message) && row.Row.Status != RowStatus.Failed, "metadata rank lost original item");
            ranked++;
        }
        Check(ranked == 1, "metadata rank dropped the record");
        var toolbox = AnnotationQuestions.Parse("""{"ready":{"function":"decide","question":"Ready?"}}""");
        Successful(await client.AnnotateAsync(toolbox, message, metadata));
        Successful(await client.Toolbox(toolbox).AnnotateAsync(message, metadata));
        int annotated = 0;
        await foreach (var row in client.Toolbox(toolbox).AnnotateStreamAsync(Records(new[] { message }), metadata))
        {
            Check(row.Position == 1 && row.Status != RowStatus.Failed, "metadata toolbox stream lost position");
            annotated++;
        }
        Check(annotated == 1, "metadata toolbox stream dropped the record");
    }
    private static void Successful<A, D>(Row<A, D> row) =>
        Check(row.Status != RowStatus.Failed, "explicit record metadata produced a failed row");
}

[JsonSourceGenerationOptions(PropertyNamingPolicy = JsonKnownNamingPolicy.CamelCase)]
[JsonSerializable(typeof(Message))]
internal partial class ConsumerJson : JsonSerializerContext { }
