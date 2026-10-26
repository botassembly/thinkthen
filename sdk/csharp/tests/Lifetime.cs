using System.Runtime.CompilerServices;
using ThinkThen;

internal static partial class Program
{
    private static async Task LifetimeCases(string loopback)
    {
        await using var client = new ThinkThenClient(new() { Backend = Backend.Stub });
        var source = new HeldSource();
        using var stop = new CancellationTokenSource();
        var enumerator = client.DecideStreamAsync("Ready?", source).GetAsyncEnumerator(stop.Token);
        Task<bool> move = enumerator.MoveNextAsync().AsTask();
        await source.Entered.Task;
        stop.Cancel();
        await Cancelled(move, stop.Token);
        await enumerator.DisposeAsync();
        Check(!source.Disposed.Task.IsCompleted, "source disposal overlapped an unfinished MoveNextAsync");
        source.Release.SetResult();
        await source.Disposed.Task;
        Check(source.DisposalCount == 1, "cancelled source must dispose exactly once");

        using var already = new CancellationTokenSource();
        already.Cancel();
        var untouched = new HeldSource();
        await using (var cancelled = client.DecideStreamAsync("Ready?", untouched,
            cancellationToken: already.Token).GetAsyncEnumerator())
            await Cancelled(cancelled.MoveNextAsync().AsTask(), already.Token);
        Check(!untouched.Entered.Task.IsCompleted, "pre-cancelled call touched its input");

        TaskCompletionSource disposed = new(TaskCreationOptions.RunContinuationsAsynchronously);
        async IAsyncEnumerable<string> Input([EnumeratorCancellation] CancellationToken ct = default)
        {
            try
            {
                ct.ThrowIfCancellationRequested();
                yield return "alpha";
                await Task.Yield();
                yield return "beta";
            }
            finally { disposed.TrySetResult(); }
        }
        await foreach (var row in client.DecideStreamAsync("Ready?", Input())) break;
        await disposed.Task;
        // Disposing a completed enumerator unregisters cancellation before its handle is freed.
        using var after = new CancellationTokenSource();
        await using (var finished = client.DecideStreamAsync("Ready?", Records(new[] { "alpha" }),
            cancellationToken: after.Token).GetAsyncEnumerator())
            while (await finished.MoveNextAsync()) { }
        after.Cancel();
        Check(await client.DecideAsync("Ready?", "beta") is not Failed<bool, NoDetail>,
            "late cancellation invalidated another call");

        foreach (Backend backend in new[] { Backend.SystemOne, Backend.OpenAI })
        {
            await using var held = await Fixture.Start(loopback, "--held", "headers");
            await using var remote = new ThinkThenClient(new()
            {
                Backend = backend, Url = new Uri(held.Url), ApiKey = ApiKey.From("synthetic-loopback-key")
            });
            using var cancelled = new CancellationTokenSource();
            using var other = new CancellationTokenSource();
            Task<Row<bool, NoDetail>> first = remote.DecideAsync("Ready?", "alpha", cancellationToken: cancelled.Token);
            Task<Row<bool, NoDetail>> second = remote.DecideAsync("Ready?", "beta", cancellationToken: other.Token);
            Check(await held.Command("arrived") == "arrived", "first call never reached the held backend");
            Check(await held.Command("arrived") == "arrived", "second call never reached the held backend");
            cancelled.Cancel();
            await Cancelled(first, cancelled.Token);
            Check(await held.Command("closed") == "closed", "token cancellation left an open native request");
            Check(!second.IsCompleted, "cancelling one call cancelled an independent call");
            Task dispose = remote.DisposeAsync().AsTask();
            Task repeated = remote.DisposeAsync().AsTask();
            await Task.WhenAll(dispose, repeated);
            await Cancelled(second);
            Check(await held.Command("closed") == "closed", "client disposal left an active request");
        }
        await DisposalDuringSourceAdvance();
        await DisposalBeforeFirstEnumeration();
        await SourceFailure();
        await AdmissionRace(loopback);
    }

    private static async Task AdmissionRace(string loopback)
    {
        foreach (bool deferred in new[] { false, true })
        {
            await using var held = await Fixture.Start(loopback, "--held", "headers");
            await using var client = new ThinkThenClient(new()
            { Backend = Backend.SystemOne, Url = new Uri(held.Url), ApiKey = ApiKey.From("synthetic-loopback-key") });
            TaskCompletionSource start = new(TaskCreationOptions.RunContinuationsAsynchronously);
            var stream = client.DecideStreamAsync("Ready?", Records(new[] { "alpha" }));
            Task admission = Task.Run(async () =>
            {
                await start.Task;
                try
                {
                    if (deferred)
                    {
                        await using var iterator = stream.GetAsyncEnumerator();
                        await iterator.MoveNextAsync();
                    }
                    else await client.DecideAsync("Ready?", "alpha");
                    throw new Exception("held call completed without disposal rejecting or cancelling it");
                }
                catch (ObjectDisposedException) { }
                catch (OperationCanceledException) { }
            });
            Task disposal = Task.Run(async () => { await start.Task; await client.DisposeAsync(); });
            start.SetResult();
            await Task.WhenAll(admission, disposal);
            int count = int.Parse(await held.Command("count"));
            Check(count is 0 or 1, "admission race opened more than one native call");
            if (count == 1)
                Check(await held.Command("closed") == "closed", "admission race left a native request alive after disposal");
        }
    }

    private static async Task DisposalDuringSourceAdvance()
    {
        var client = new ThinkThenClient(new() { Backend = Backend.Stub });
        var source = new HeldSource();
        await using var enumerator = client.DecideStreamAsync("Ready?", source).GetAsyncEnumerator();
        Task<bool> move = enumerator.MoveNextAsync().AsTask();
        await source.Entered.Task;
        await Task.WhenAll(client.DisposeAsync().AsTask(), client.DisposeAsync().AsTask());
        await Cancelled(move);
        Check(!source.Disposed.Task.IsCompleted, "client disposal raced an uncooperative producer");
        source.Release.SetResult();
        await source.Disposed.Task;
        Check(source.DisposalCount == 1, "client cleanup disposed the producer more than once");
    }

    private static async Task DisposalBeforeFirstEnumeration()
    {
        var client = new ThinkThenClient(new() { Backend = Backend.Stub });
        var source = new HeldSource();
        var deferred = client.DecideStreamAsync("Ready?", source);
        await client.DisposeAsync();
        await using var enumerator = deferred.GetAsyncEnumerator();
        await Rejected(async () => { await enumerator.MoveNextAsync(); });
        await Rejected(async () => { await client.DecideAsync("Ready?", "alpha"); });
        Check(!source.Entered.Task.IsCompleted, "disposed client admitted a deferred stream");
    }

    private static async Task Rejected(Func<Task> call)
    {
        try { await call(); }
        catch (ObjectDisposedException) { return; }
        throw new Exception("disposed client accepted a new call");
    }

    private static async Task SourceFailure()
    {
        await using var client = new ThinkThenClient(new() { Backend = Backend.Stub });
        var primary = new InvalidOperationException("synthetic producer failure");
        var cleanup = new InvalidOperationException("synthetic cleanup failure");
        foreach (Exception? advancing in new Exception?[] { primary, null })
        {
            try { await Rows(client.DecideStreamAsync("Ready?", new ErrorSource(advancing, cleanup))); }
            catch (Exception error)
            {
                Check(ReferenceEquals(error, advancing ?? cleanup) || ReferenceEquals(error.InnerException, advancing ?? cleanup),
                    "cleanup replaced the original producer failure or lost its own first failure");
                continue;
            }
            throw new Exception("producer or cleanup failure vanished");
        }
    }
}

internal sealed class HeldSource : IAsyncEnumerable<string>, IAsyncEnumerator<string>
{
    internal TaskCompletionSource Entered { get; } = new(TaskCreationOptions.RunContinuationsAsynchronously);
    internal TaskCompletionSource Release { get; } = new(TaskCreationOptions.RunContinuationsAsynchronously);
    internal TaskCompletionSource Disposed { get; } = new(TaskCreationOptions.RunContinuationsAsynchronously);
    internal int DisposalCount { get; private set; }
    public string Current => "alpha";
    public IAsyncEnumerator<string> GetAsyncEnumerator(CancellationToken cancellationToken = default) => this;
    public async ValueTask<bool> MoveNextAsync()
    {
        Entered.TrySetResult();
        await Release.Task;
        return true;
    }
    public ValueTask DisposeAsync()
    {
        DisposalCount++;
        Disposed.TrySetResult();
        return ValueTask.CompletedTask;
    }
}

internal sealed class ErrorSource(Exception? primary, Exception cleanup) : IAsyncEnumerable<string>, IAsyncEnumerator<string>
{
    public string Current => "unused";
    public IAsyncEnumerator<string> GetAsyncEnumerator(CancellationToken cancellationToken = default) => this;
    public ValueTask<bool> MoveNextAsync() => primary is null ? ValueTask.FromResult(false) : throw primary;
    public ValueTask DisposeAsync() => throw cleanup;
}
