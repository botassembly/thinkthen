using System.Runtime.InteropServices;
using System.Text.Json;
using System.Text.Json.Serialization;
using ThinkThen;

internal static partial class Program
{
    private static async Task SdkFixes(string registry)
    {
        await RegistrationAndSecrets();
        await Registry(registry);
        await EncoderCauses();
        using var client = new ThinkThenClient(new() { Backend = Backend.Stub });
        using var question = DecideQuestion.Create("Ready?");
        int before = HandleCounts.Live();
        int admitted = HandleCounts.Questions();
        Threshold band = Threshold.Band(0.2, 0.8);
        Cut cut = Cut.From(0.6);
        Pointer pointer = Pointer.From("/body");
        Check(HandleCounts.Questions() == admitted, "value factories created native questions for validation");
        foreach (Threshold invalid in new[] { Threshold.Band(0.8, 0.2), Threshold.Band(double.NaN, 0.8), Threshold.Cut(double.PositiveInfinity), Threshold.Cut(0) })
            await Code(ErrorCode.UsageThreshold, () => client.DecideAsync("Ready?", "alpha", new DecideOptions { Threshold = invalid }));
        await Code(ErrorCode.UsagePointer, () => client.DecideAsync("Ready?", "alpha", new DecideOptions { Evidence = Pointer.From("bad") }));
        for (int i = 0; i < 64; i++)
        {
            Successful(await client.DecideAsync("Ready?", "alpha"));
            Successful(await client.DecideAsync(question, "beta"));
            using var annotations = AnnotationQuestions.Parse("""{"ready":{"function":"decide","question":"Ready?"}}""");
            Successful(await client.AnnotateAsync(annotations, "alpha"));
            Successful(await client.AnnotateAsync(annotations, JsonSerializer.Deserialize<JsonElement>("""{"body":"beta"}"""), new AnnotateOptions { Evidence = pointer }));
            await foreach (var row in client.DecideStreamAsync("Ready?", Records(new[] { "alpha", "beta" }))) Successful(row);
            await Code(ErrorCode.UsageName, () => client.DecideAsync("Ready?", "alpha", name: ""));
            _ = client.DecideStreamAsync("Ready?", Records(new[] { "unconsumed" }));
        }
        Check(HandleCounts.Live() == before, "completed, failed or unconsumed SDK calls retained native handles without finalizers");
        Successful(await client.DecideAsync(question, "still caller owned"));
        var source = new HeldSource();
        await using var enumerator = client.DecideStreamAsync("Ready?", source).GetAsyncEnumerator();
        Task<bool> move = enumerator.MoveNextAsync().AsTask();
        await source.Entered.Task;
        await Task.Run(client.Dispose);
        await Cancelled(move);
        source.Release.SetResult();
        await source.Disposed.Task;
        await Rejected(async () => { await client.DecideAsync(question, "closed"); });
        // The user question remains independent of the disposed client.
        using var other = new ThinkThenClient(new() { Backend = Backend.Stub });
        Successful(await other.DecideAsync(question, "another client"));
        Console.WriteLine("C# disposal, ownership, handle counts, encoder causes and enum contracts passed.");
    }
    private static async Task EncoderCauses()
    {
        var cause = new JsonException("synthetic converter failure");
        var json = new JsonSerializerOptions();
        var converter = new ThrowingConverter(cause);
        json.Converters.Add(converter);
        using var client = new ThinkThenClient(new() { Backend = Backend.Stub, Json = json });
        var invalid = Case<Failed<bool, NoDetail>>(await client.DecideAsync("Ready?", new BrokenRecord()));
        Check(invalid.Error.Code == ErrorCode.RecordInvalid && ReferenceEquals(invalid.Error.Cause, cause),
            "record encoding lost the converter cause");
        converter.Failure = new InvalidOperationException("synthetic converter bug");
        try { await client.DecideAsync("Ready?", new BrokenRecord()); }
        catch (InvalidOperationException error) when (ReferenceEquals(error, converter.Failure)) { return; }
        throw new Exception("converter programming failure became an invalid record");
    }
}
internal sealed class BrokenRecord { }
internal sealed class ThrowingConverter(Exception failure) : JsonConverter<BrokenRecord>
{
    internal Exception Failure { get; set; } = failure;
    public override BrokenRecord Read(ref Utf8JsonReader reader, Type type, JsonSerializerOptions options) => throw new NotSupportedException();
    public override void Write(Utf8JsonWriter writer, BrokenRecord value, JsonSerializerOptions options) => throw Failure;
}
internal static class HandleCounts
{
    [DllImport("thinkthen_ffi", EntryPoint = "thinkthen_test_live_handles")]
    internal static extern int Live();
    [DllImport("thinkthen_ffi", EntryPoint = "thinkthen_test_questions_created")]
    internal static extern int Questions();
}
