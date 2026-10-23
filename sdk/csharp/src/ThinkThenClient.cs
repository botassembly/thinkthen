using System.Runtime.CompilerServices;
using System.Text.Json;

namespace ThinkThen;

public sealed partial class ThinkThenClient : IDisposable
{
    private readonly ClientHandle client;
    public ThinkThenClient(ClientSettings? settings = null)
    {
        byte[] bytes = JsonSerializer.SerializeToUtf8Bytes(settings ?? new(), Json.Options);
        int result = Native.ClientOpen(bytes, (nuint)bytes.Length, out nint pointer, out Buffer error);
        Json.Check(result, error);
        client = new(pointer);
    }
    public void Dispose() => client.Dispose();

    private sealed record Event<T>(ResultRow<T> Row, Record? Record, bool Kept);
    private sealed record Pull(int Status, byte[] Fields, byte[] Detail, nuint Position, bool Kept);
    private static Pull Next(CallHandle call, CancellationToken token)
    {
        int status = Native.Next(call, out NativeRow row, out Buffer error);
        try
        {
            if (status == -1)
            {
                token.ThrowIfCancellationRequested();
                Buffer owned = error;
                error = default;
                Json.Check(status, owned);
            }
            return new(status, Native.Copy(row.Fields), Native.Copy(row.Detail), row.Position, row.Kept != 0);
        }
        finally { Native.RowFree(ref row); Native.BufferFree(ref error); }
    }

    private async IAsyncEnumerable<Event<T>> Run<T>(string function, object question, IAsyncEnumerable<Record> records,
        CallOptions? options, object? choices, bool identity,
        [EnumeratorCancellation] CancellationToken cancellationToken)
    {
        cancellationToken.ThrowIfCancellationRequested();
        var request = new Dictionary<string, object?> { ["function"] = function, ["question"] = question };
        var callOptions = new Dictionary<string, object?>();
        if (options?.Threshold is Threshold threshold) callOptions["threshold"] = threshold.Value;
        if (options?.On is string on) callOptions["on"] = on;
        if (options is RankOptions rank && rank.Top is double top) callOptions["top"] = top;
        if (options is RankOptions { Levels: not null } ranked) callOptions["levels"] = ranked.Levels.ToArray();
        if (choices is not null) callOptions[function == "choose" ? "options" : function == "tag" ? "labels" : "levels"] = choices;
        if (callOptions.Count != 0) request["options"] = callOptions;
        byte[] bytes = JsonSerializer.SerializeToUtf8Bytes(request, Json.Options);
        int status = Native.Open(client, bytes, (nuint)bytes.Length, 1, out nint pointer, out Buffer error);
        Json.Check(status, error);
        using var call = new CallHandle(pointer);
        using var registration = cancellationToken.Register(() => Native.Cancel(call));
        Source? source = null;
        bool failed = false;
        var original = identity ? new Dictionary<nuint, Record>() : null;
        nuint position = 0;
        try
        {
            source = new(records, cancellationToken);
            while (true)
            {
                Pull pull;
                try { pull = await Task.Run(() => Next(call, cancellationToken), CancellationToken.None).ConfigureAwait(false); }
                catch { failed = true; throw; }
                if (pull.Status == 1) break;
                if (pull.Status == 2)
                {
                    bool more;
                    try { more = await source.Advance().ConfigureAwait(false); }
                    catch { failed = true; throw; }
                    if (more)
                    {
                        Record record;
                        try { record = source.Current; }
                        catch { failed = true; throw; }
                        position++;
                        if (identity) original!.Add(position, record);
                        int sent;
                        if (record.IsText)
                        {
                            char[] units = record.TextValue!.ToCharArray();
                            sent = Native.SendText(call, units, (nuint)units.Length, out error);
                        }
                        else
                        {
                            byte[] recordBytes = record.Bytes;
                            sent = Native.SendJson(call, recordBytes, (nuint)recordBytes.Length, out error);
                        }
                        try { Json.Check(sent, error); }
                        catch (ThinkThenException)
                        {
                            failed = true;
                            cancellationToken.ThrowIfCancellationRequested();
                            throw;
                        }
                    }
                    else Native.EndInput(call);
                }
                else
                {
                    Record? record = null;
                    if (identity) { record = original![pull.Position]; original.Remove(pull.Position); }
                    ResultRow<T> row;
                    try { row = new(pull.Fields, pull.Detail); }
                    catch { failed = true; throw; }
                    yield return new(row, record, pull.Kept);
                }
            }
        }
        finally
        {
            // No native call remains in progress here. Unregister before freeing.
            registration.Dispose();
            call.Dispose();
            if (source is not null) await source.Finish(failed).ConfigureAwait(false);
        }
    }

    private async Task<ResultRow<T>> Single<T>(string function, object question, Record record, CallOptions? options, object? choices, CancellationToken token)
    {
        ResultRow<T>? result = null;
        await foreach (var item in Run<T>(function, question, One(record), options, choices, false, token).ConfigureAwait(false))
            result = item.Row;
        return result!;
    }
    private async IAsyncEnumerable<ResultRow<T>> Stream<T>(string function, object question, IAsyncEnumerable<Record> records,
        CallOptions? options, object? choices, [EnumeratorCancellation] CancellationToken token)
    {
        await foreach (var item in Run<T>(function, question, records, options, choices, false, token).ConfigureAwait(false))
            yield return item.Row;
    }
    private static async IAsyncEnumerable<Record> One(Record record)
    {
        yield return record;
        await Task.CompletedTask;
    }
}
