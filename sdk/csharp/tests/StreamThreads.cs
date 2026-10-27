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
                Check(row.Status != RowStatus.Failed && row.Position == (ulong)++rows,
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
        Check(encoder.Threads.Count == 103, "streams shared their driver thread");
        Check(encoder.Threads.Values.Distinct().Count() == 103, "independent streams reused a driver thread");
        foreach (Thread thread in encoder.Threads.Values)
            Check(!thread.IsAlive, "completed or stopped stream left its driver thread running");
        Console.WriteLine("C# streams: one dedicated thread across 1000 rows; 100 concurrent streams; early stop and cancellation joined; pending input leaves output responsive.");
    }
}

internal sealed record ThreadRecord(int Stream);
internal sealed class ThreadRecordConverter : JsonConverter<ThreadRecord>
{
    internal ConcurrentDictionary<int, Thread> Threads { get; } = new();
    public override ThreadRecord Read(ref Utf8JsonReader reader, Type type, JsonSerializerOptions options) =>
        throw new NotSupportedException();
    public override void Write(Utf8JsonWriter writer, ThreadRecord value, JsonSerializerOptions options)
    {
        Thread thread = Thread.CurrentThread;
        if (thread.IsThreadPoolThread || !ReferenceEquals(Threads.GetOrAdd(value.Stream, thread), thread))
            throw new InvalidOperationException("stream started another driver or used a thread pool worker");
        writer.WriteStartObject();
        writer.WriteString("body", "alpha");
        writer.WriteEndObject();
    }
}
