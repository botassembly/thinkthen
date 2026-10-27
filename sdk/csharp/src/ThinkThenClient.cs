using System.Diagnostics.CodeAnalysis;
using System.Runtime.CompilerServices;
using System.Text;
using System.Text.Json;
using System.Text.Json.Nodes;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Logging.Abstractions;

namespace ThinkThen;

public sealed partial class ThinkThenClient : IDisposable, IAsyncDisposable
{
    private readonly ClientHandle client;
    private readonly JsonSerializerOptions serializer;
    private readonly bool reflectionDefault;
    private JsonSerializerOptions? reflectionSerializer;
    private readonly ILogger<ThinkThenClient> logger;
    private readonly object lifecycle = new();
    private readonly HashSet<CallState> active = new();
    private Task? disposal;
    public ThinkThenClient(ThinkThenOptions? options = null, ILogger<ThinkThenClient>? logger = null)
    {
        options ??= new();
        this.logger = logger ?? NullLogger<ThinkThenClient>.Instance;
        serializer = new(options.Json);
        reflectionDefault = serializer.TypeInfoResolver is null;
        serializer.TypeInfoResolver ??= ContractJson.Default;
        serializer.MakeReadOnly();
        JsonObject settings = ThinkThenOptionsConverter.Node(options, true);
        byte[] bytes = Admission.Bytes(settings);
        NativeError error = NativeError.Empty;
        Check(Native.ClientOpen(bytes, (nuint)bytes.Length, out nint pointer, ref error), error);
        client = new(pointer);
        this.logger.LogInformation("ThinkThen client opened");
    }
    private void Check(int status, NativeError error)
    {
        try { Json.Check(status, error); }
        catch (ThinkThenException failure)
        {
            logger.LogWarning("ThinkThen operation failed: {Code}", failure.Code);
            throw;
        }
    }
    public void Dispose() => DisposeAsync().AsTask().GetAwaiter().GetResult();
    public ValueTask DisposeAsync()
    {
        lock (lifecycle)
        {
            if (disposal is null)
            {
                CallState[] calls = active.ToArray();
                disposal = Task.Run(() => DisposeCore(calls));
            }
            return new(disposal);
        }
    }
    private async Task DisposeCore(CallState[] calls)
    {
        await Task.WhenAll(calls.Select(call => call.Close())).ConfigureAwait(false);
        client.Dispose();
        logger.LogInformation("ThinkThen client closed");
    }
    private unsafe CallState Open(QuestionHandle question, string? name, CancellationToken token)
    {
        lock (lifecycle)
        {
            ObjectDisposedException.ThrowIf(disposal is not null, this);
            token.ThrowIfCancellationRequested();
            byte[]? nameBytes = name is null ? null : Encoding.UTF8.GetBytes(Json.HostText(name));
            NativeError error = NativeError.Empty;
            nint pointer;
            fixed (byte* data = nameBytes)
            {
                NativeText view = new() { Data = (nint)data, Length = (nuint)(nameBytes?.Length ?? 0) };
                Check(Native.Open(client, question, name is null ? 0 : (nint)(&view), 1, out pointer, ref error), error);
            }
            var call = new CallState(new(pointer), token);
            active.Add(call);
            return call;
        }
    }
    private sealed record Pull(int Status, uint Function, uint RowStatus, int ErrorCode, double Probability, ulong Position,
        byte[] Id, byte[] Answer, byte[] Source, byte[] Error, byte[] Detail);
    private static Pull Next(CallHandle call, CancellationToken token)
    {
        NativeRow row = NativeRow.Empty;
        NativeError error = NativeError.Empty;
        int status = Native.Next(call, ref row, ref error);
        try
        {
            if (status == -1)
            {
                token.ThrowIfCancellationRequested();
                NativeError owned = error; error = NativeError.Empty;
                Json.Check(status, owned);
            }
            return new(status, row.Function, row.Status, row.ErrorCode, row.Probability, row.Position, Native.Copy(row.Id),
                Native.Copy(row.Answer), Native.Copy(row.Source), Native.Copy(row.Error), Native.Copy(row.Detail));
        }
        finally { Native.RowFree(ref row); Native.ErrorFree(ref error); }
    }
    private sealed record Output<T,A,D>(T Item, Row<A,D> Row);
    private async IAsyncEnumerable<Output<T,A,D>> Run<T,A,D>(Question<A,D> question, IAsyncEnumerable<T> records,
        Func<T, byte[]> encode, bool originals, string? name, [EnumeratorCancellation] CancellationToken cancellationToken, bool owned = false)
    {
        cancellationToken.ThrowIfCancellationRequested();
        if (question is null) throw Json.Conversion(new ArgumentNullException(nameof(question)));
        if (records is null) throw Json.Conversion(new ArgumentNullException(nameof(records)));
        CallState call;
        try { call = Open(question.Handle, name, cancellationToken); }
        finally { if (owned) question.Dispose(); }
        Source<T>? source = null;
        bool failed = false;
        var original = originals ? new Dictionary<ulong,T>() : null;
        ulong position = 0;
        Task<bool>? input = null;
        var causes = new Dictionary<ulong,Exception>();
        try
        {
            source = new(records, call.Token);
            while (true)
            {
                Pull pull;
                ulong version = 0;
                try
                {
                    pull = await call.Dispatch(handle =>
                    {
                        version = call.Version();
                        return Next(handle, call.Token);
                    }).ConfigureAwait(false);
                }
                catch (OperationCanceledException) { failed = true; throw call.Cancelled(); }
                catch { failed = true; throw; }
                if (pull.Status == 1) break;
                if (pull.Status == 2)
                {
                    bool more;
                    try
                    {
                        input ??= source.Advance();
                        await call.WaitForInput(input, version).ConfigureAwait(false);
                        if (!input.IsCompleted) continue;
                        more = await input.ConfigureAwait(false);
                        input = null;
                    }
                    catch (OperationCanceledException) { failed = true; throw call.Cancelled(); }
                    catch { failed = true; throw; }
                    try
                    {
                        if (more)
                        {
                            T record = source.Current;
                            position++;
                            if (originals) original!.Add(position, record);
                            await call.Dispatch(handle => Send(handle, record, encode, cause => causes.Add(position, cause))).ConfigureAwait(false);
                        }
                        else await call.Dispatch(handle => { Native.EndInput(handle); return 0; }).ConfigureAwait(false);
                    }
                    catch (OperationCanceledException) { failed = true; throw call.Cancelled(); }
                    catch { failed = true; throw; }
                }
                else if (pull.Status == 0)
                {
                    T item = default!;
                    if (originals) { item = original![pull.Position]; original.Remove(pull.Position); }
                    Row<A,D> row;
                    try { row = Codecs.Row<A,D>(pull.Function, pull.RowStatus, pull.ErrorCode, pull.Position, pull.Probability,
                        pull.Id, pull.Answer, pull.Detail, pull.Source, pull.Error, question.Definition["function"]!.GetValue<string>()); }
                    catch { failed = true; throw; }
                    if (causes.Remove(pull.Position, out Exception? cause) && row is Failed<A,D> invalid)
                        row = new Failed<A,D>(invalid.Function, invalid.Position, invalid.Id, invalid.Source,
                            invalid.Error with { Cause = cause }) { Canonical = invalid.Canonical };
                    if (row is Failed<A,D> failure) logger.LogWarning("ThinkThen row failed: {Code}", failure.Error.Code);
                    yield return new(item, row);
                }
                else throw Json.Abi();
            }
        }
        finally
        {
            bool cancelled = call.Token.IsCancellationRequested;
            await call.Close().ConfigureAwait(false);
            lock (lifecycle) active.Remove(call);
            original?.Clear();
            if (source is not null) await source.Finish(failed || cancelled, cancellationToken).ConfigureAwait(false);
        }
    }
    private static int Send<T>(CallHandle handle, T record, Func<T,byte[]> encode, Action<Exception> failed)
    {
        NativeError error = NativeError.Empty;
        int sent;
        if (record is null || record is JsonElement { ValueKind: JsonValueKind.Null or JsonValueKind.Undefined })
            sent = Native.SendMissing(handle, ref error);
        else if (record is string || record is JsonElement { ValueKind: JsonValueKind.String })
        {
            string value = record is string input ? input : ((JsonElement)(object)record!).GetString()!;
            char[] units = value.ToCharArray();
            sent = Native.SendText(handle, units, (nuint)units.Length, ref error);
        }
        else
        {
            byte[] bytes;
            try { bytes = encode(record); }
            catch (Exception failure) when (failure is JsonException or NotSupportedException)
            { Json.Check(Native.SendInvalid(handle, ref error), error); failed(failure); return 0; }
            sent = Native.SendJson(handle, bytes, (nuint)bytes.Length, ref error);
        }
        Json.Check(sent, error);
        return sent;
    }
    [RequiresUnreferencedCode("Provide JsonTypeInfo for records in trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for records in AOT applications.")]
    private byte[] Serialize<T>(T record)
    {
        if (!reflectionDefault) return JsonSerializer.SerializeToUtf8Bytes(record, serializer);
        JsonSerializerOptions resolved;
        lock (lifecycle)
        {
            if (reflectionSerializer is null)
            {
                reflectionSerializer = new(serializer) { TypeInfoResolver = new System.Text.Json.Serialization.Metadata.DefaultJsonTypeInfoResolver() };
                reflectionSerializer.MakeReadOnly();
            }
            resolved = reflectionSerializer;
        }
        return JsonSerializer.SerializeToUtf8Bytes(record, resolved);
    }
    private async Task<Row<A,D>> Single<T,A,D>(Question<A,D> question, T record, Func<T,byte[]> encode, string? name, CancellationToken token, bool owned = false)
    {
        Row<A,D>? result = null;
        await foreach (var output in Run(question, One(record), encode, false, name, token, owned).ConfigureAwait(false)) result = output.Row;
        return result ?? throw Json.Abi();
    }
    private async IAsyncEnumerable<Row<A,D>> Stream<T,A,D>(Question<A,D> question, IAsyncEnumerable<T> records,
        Func<T,byte[]> encode, string? name, [EnumeratorCancellation] CancellationToken token, bool owned = false)
    {
        await foreach (var output in Run(question, records, encode, false, name, token, owned).ConfigureAwait(false)) yield return output.Row;
    }
    private static async Task<TResult> Own<TQuestion,TResult>(Func<TQuestion> create, Func<TQuestion,Task<TResult>> run)
        where TQuestion : IDisposable
    {
        using var question = create();
        return await run(question).ConfigureAwait(false);
    }
    private static async IAsyncEnumerable<TResult> OwnStream<TQuestion,TResult>(Func<TQuestion> create,
        Func<TQuestion,IAsyncEnumerable<TResult>> run, [EnumeratorCancellation] CancellationToken token = default)
        where TQuestion : IDisposable
    {
        using var question = create();
        await foreach (var result in run(question).WithCancellation(token).ConfigureAwait(false)) yield return result;
    }
    private static async IAsyncEnumerable<T> One<T>(T record) { yield return record; await Task.CompletedTask; }
}
