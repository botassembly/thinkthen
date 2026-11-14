using System.Collections.Concurrent;
using System.Runtime.CompilerServices;
using System.Text.Json;
using System.Text.Json.Serialization;
using ThinkThen;

internal static partial class Program
{
    private static async Task StreamThreads()
    {
        var encoder = new ThreadRecordConverter();
        await using var client = new ThinkThenClient(new()
        {
            Backend = Backend.Stub,
            Batch = 1,
            Json = new JsonSerializerOptions { Converters = { encoder } }
        });
        async Task Consume(int stream, int count, bool stopEarly = false)
        {
            int rows = 0;
            await foreach (var row in client.DecideStreamAsync("Ready?",
                Records(Enumerable.Range(0, count).Select(_ => new ThreadRecord(stream)))))
            {
                Check(row.Status != Status.Failed && row.Position == (ulong)++rows,
                    "dedicated stream dropped or reordered a record");
                if (stopEarly) break;
            }
            Check(rows == (stopEarly ? 1 : count), "dedicated stream returned the wrong row count");
        }
        await Consume(0, 1000);
        await Task.WhenAll(Enumerable.Range(1, 100).Select(stream => Consume(stream, 10)));
        await Consume(101, 1000, true);
        using var stop = new CancellationTokenSource();
        TaskCompletionSource waiting = new(TaskCreationOptions.RunContinuationsAsynchronously);
        async IAsyncEnumerable<ThreadRecord> Held([EnumeratorCancellation] CancellationToken token = default)
        {
            yield return new ThreadRecord(102);
            waiting.TrySetResult();
            await Task.Delay(Timeout.Infinite, token);
        }
        await using (var iterator = client.DecideStreamAsync("Ready?", Held(), cancellationToken: stop.Token).GetAsyncEnumerator())
        {
            Task<bool> first = iterator.MoveNextAsync().AsTask();
            // A completed row must arrive even when the source has no second record.
            Check(await first, "pending caller input hid a ready native row");
            Task<bool> next = iterator.MoveNextAsync().AsTask();
            await waiting.Task;
            stop.Cancel();
            await Cancelled(next, stop.Token);
        }
        Check(encoder.Counts.Count==103&&encoder.Counts[0]==1000,"concurrent streams lost converter records");
        Check(Enumerable.Range(1,100).All(stream=>encoder.Counts[stream]==10),"independent streams lost or duplicated converted records");
        Check(encoder.Counts[101]>=1&&encoder.Counts[102]==1,"early stop and cancellation did not complete their first record");
        Console.WriteLine("C# concurrent streams preserve records, early stop and cancellation; pending input leaves output responsive.");
    }
}

internal sealed record ThreadRecord(int Stream);
internal sealed class ThreadRecordConverter : JsonConverter<ThreadRecord>
{
    internal ConcurrentDictionary<int,int> Counts { get; } = new();
    public override ThreadRecord Read(ref Utf8JsonReader reader, Type type, JsonSerializerOptions options) =>
        throw new NotSupportedException();
    public override void Write(Utf8JsonWriter writer, ThreadRecord value, JsonSerializerOptions options)
    {
        Counts.AddOrUpdate(value.Stream,1,(_,count)=>count+1);
        writer.WriteStartObject();
        writer.WriteString("body", "alpha");
        writer.WriteEndObject();
    }
}
