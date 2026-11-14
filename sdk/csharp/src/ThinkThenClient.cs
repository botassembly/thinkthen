using System.Diagnostics.CodeAnalysis;
using System.Runtime.CompilerServices;
using System.Text;
using System.Text.Json;
using System.Text.Json.Nodes;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Logging.Abstractions;

namespace ThinkThen;

/// <summary>Runs typed questions over records and owns active calls.</summary>
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
    private Usage? finalUsage;
    /// <summary>Runs typed questions over records and owns active calls.</summary>
    public ThinkThenClient(ThinkThenOptions? options = null, ILogger<ThinkThenClient>? logger = null)
    {
        options ??= new();
        this.logger = logger ?? NullLogger<ThinkThenClient>.Instance;
        serializer = new(options.Json);
        reflectionDefault = serializer.TypeInfoResolver is null;
        serializer.TypeInfoResolver ??= ContractJson.Default;
        serializer.MakeReadOnly();
        JsonObject settings = ThinkThenOptionsConverter.Node(options, true);
        if (options.Backend is null && Environment.GetEnvironmentVariable("THINKTHEN_BACKEND") is string backend)
            settings["backend"] = Json.HostText(backend, settings: true);
        byte[] bytes = Admission.Bytes(settings);
        NativeError error = NativeError.Empty;
        nint pointer;
        try { Check(Native.ClientOpen(bytes,(nuint)bytes.Length,out pointer,ref error),error); }
        catch(ThinkThenException failure) when(failure.ErrorCode==ErrorCode.UsageAbi&&failure is not SetupException)
        { throw new SetupException(new(failure.ErrorCode,failure.Message,failure.HttpStatus),failure); }
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
    /// <summary>Returns a copied cumulative snapshot, also available after disposal.</summary>
    public Usage Usage
    {
        get { lock (lifecycle) return finalUsage ?? ReadUsage(false); }
    }
    private Usage ReadUsage(bool settled)
    {
        NativeError error = NativeError.Empty;
        Buffer output;
        int status = settled ? Native.ClientUsageSettled(client, out output, ref error)
            : Native.ClientUsage(client, out output, ref error);
        try
        {
            Check(status, error);
            return JsonSerializer.Deserialize(Native.Copy(output), ContractJson.Default.Usage) ?? throw Json.Abi();
        }
        finally { Native.BufferFree(ref output); }
    }
    /// <summary>Releases the owned native resource.</summary>
    public void Dispose() => DisposeAsync().AsTask().GetAwaiter().GetResult();
    /// <summary>Cancels active calls, waits for their cleanup and releases the native client.</summary>
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
        try
        {
            Usage snapshot = ReadUsage(true);
            lock (lifecycle) { finalUsage = snapshot; client.Dispose(); }
        }
        catch { lock (lifecycle) client.Dispose(); throw; }
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
    private void Forget(CallState call) { lock (lifecycle) active.Remove(call); }
    private sealed record Pull(int Status,ulong Position,byte[] Host,byte[] Saved);
    private static Pull Next(CallHandle call,CancellationToken token)
    {
        NativeRow row=NativeRow.Empty;
        NativeError error=NativeError.Empty;
        int status=Native.Next(call,ref row,ref error);
        try
        {
            if(status<0) { token.ThrowIfCancellationRequested(); var owned=error; error=NativeError.Empty; Json.Check(status,owned); }
            if(status!=0) return new(status,0,[],[]);
            int hostStatus=Native.RowHostJson(ref row,out Buffer host,ref error);
            try
            {
                var owned=error; error=NativeError.Empty; Json.Check(hostStatus,owned);
                int savedStatus=Native.RowToJson(ref row,out Buffer saved,ref error);
                try
                {
                    owned=error; error=NativeError.Empty; Json.Check(savedStatus,owned);
                    return new(status,row.Position,Native.Copy(host),Native.Copy(saved));
                }
                finally { Native.BufferFree(ref saved); }
            }
            finally { Native.BufferFree(ref host); }
        }
        finally { Native.RowFree(ref row); Native.ErrorFree(ref error); }
    }
    private async IAsyncEnumerable<Row<A,T>> Run<T,A,D>(Question<A,D> question,IAsyncEnumerable<T> records,Func<T,byte[]> encode,string? name,[EnumeratorCancellation] CancellationToken cancellationToken)
    {
        cancellationToken.ThrowIfCancellationRequested();
        ArgumentNullException.ThrowIfNull(question); ArgumentNullException.ThrowIfNull(records);
        CallState call;
        using(var admitted=Admission.Admit(question.Definition)) call=Open(admitted,name,cancellationToken);
        var lease=new CallLease(call,Forget);
        Source<T>? source=null;
        bool failed=false;
        var originals=new Dictionary<ulong,T>();
        var causes=new Dictionary<ulong,Exception>();
        ulong position=0;
        Task<bool>? input=null;
        try
        {
            source=new(records,call.Token);
            while(true)
            {
                Pull pull; ulong version=0;
                try { pull=await call.Dispatch(handle=> { version=call.Version(); return Next(handle,call.Token); }).ConfigureAwait(false); }
                catch(OperationCanceledException) { failed=true; throw call.Cancelled(); }
                catch { failed=true; throw; }
                if(pull.Status==1) break;
                if(pull.Status==2)
                {
                    try
                    {
                        input ??=source.Advance();
                        await call.WaitForInput(input,version).ConfigureAwait(false);
                        if(!input.IsCompleted) continue;
                        bool more=await input.ConfigureAwait(false); input=null;
                        if(more)
                        {
                            T record=source.Current; originals.Add(++position,record);
                            byte[]? prepared=null;
                            if(record is not null && record is not string && record is not JsonElement { ValueKind:JsonValueKind.String or JsonValueKind.Null or JsonValueKind.Undefined })
                            {
                                try { prepared=encode(record); }
                                catch(Exception cause) when(cause is JsonException or NotSupportedException) { causes.Add(position,cause); }
                            }
                            await call.Dispatch(handle=>Send(handle,record,prepared)).ConfigureAwait(false);
                        }
                        else await call.Dispatch(handle=> { Native.EndInput(handle); return 0; }).ConfigureAwait(false);
                    }
                    catch(OperationCanceledException) { failed=true; throw call.Cancelled(); }
                    catch { failed=true; throw; }
                }
                else if(pull.Status==0)
                {
                    Row<A,T> row;
                    try
                    {
                        if(!originals.Remove(pull.Position,out var item)) throw Json.Abi();
                        row=Codecs.Row(pull.Host,pull.Saved,question,item);
                        if(causes.Remove(pull.Position,out var cause) && row is Failed<A,T> invalid)
                            row=new Failed<A,T>(invalid.Function,invalid.Position,invalid.Id,invalid.Source,item,invalid.Error with { Cause=cause }) { Canonical=invalid.Canonical };
                    }
                    catch { failed=true; throw; }
                    if(row is IFailed failure) logger.LogDebug("ThinkThen row failed: {Code}",failure.Error.Code);
                    yield return row;
                }
                else throw Json.Abi();
            }
        }
        finally
        {
            GC.SuppressFinalize(lease);
            bool cancelled=call.Token.IsCancellationRequested;
            await call.Close().ConfigureAwait(false); Forget(call); originals.Clear();
            if(source is not null) await source.Finish(failed||cancelled,cancellationToken).ConfigureAwait(false);
        }
    }
    private static int Send<T>(CallHandle handle,T record,byte[]? prepared)
    {
        NativeError error=NativeError.Empty; int sent;
        if(record is null || record is JsonElement { ValueKind:JsonValueKind.Null or JsonValueKind.Undefined }) sent=Native.SendMissing(handle,ref error);
        else if(record is string || record is JsonElement { ValueKind:JsonValueKind.String })
        {
            string value=record is string text ? text : ((JsonElement)(object)record!).GetString()!;
            sent=Native.SendText(handle,value.ToCharArray(),(nuint)value.Length,ref error);
        }
        else if(prepared is null) sent=Native.SendInvalid(handle,ref error);
        else sent=Native.SendJson(handle,prepared,(nuint)prepared.Length,ref error);
        Json.Check(sent,error); return sent;
    }
    [RequiresUnreferencedCode("Provide JsonTypeInfo for records in trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for records in AOT applications.")]
    private JsonSerializerOptions ReflectionOptions()
    {
        if (!reflectionDefault) return serializer;
        lock (lifecycle)
        {
            if (reflectionSerializer is null)
            {
                reflectionSerializer = new(serializer) { TypeInfoResolver = new System.Text.Json.Serialization.Metadata.DefaultJsonTypeInfoResolver() };
                reflectionSerializer.MakeReadOnly();
            }
            return reflectionSerializer;
        }
    }
    [RequiresUnreferencedCode("Provide JsonTypeInfo for records in trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for records in AOT applications.")]
    private byte[] Serialize<T>(T record) => JsonSerializer.SerializeToUtf8Bytes(record, ReflectionOptions());
    [RequiresUnreferencedCode("Provide JsonTypeInfo for records in trimmed applications.")]
    [RequiresDynamicCode("Provide JsonTypeInfo for records in AOT applications.")]
    private bool SingleCollection<T>(T record)
    {
        if (record is null) return false;
        JsonSerializerOptions options = ReflectionOptions();
        Type type = typeof(T) == typeof(object) ? record.GetType() : typeof(T);
        return options.GetTypeInfo(type).Kind == System.Text.Json.Serialization.Metadata.JsonTypeInfoKind.Enumerable;
    }
    private async Task<Row<A,T>> Single<T,A,D>(Question<A,D> question,T record,Func<T,byte[]> encode,string? name,CancellationToken token,Func<T,bool> collection)
    {
        if(record is Array || record is System.Collections.IList || record is IAsyncEnumerable<object> || record is JsonArray || record is JsonElement { ValueKind:JsonValueKind.Array } || collection(record))
            throw new ArgumentException($"A single-record call cannot take a collection; use {question.Function}StreamAsync.",nameof(record));
        await foreach(var row in Run(question,One(record),encode,name,token).ConfigureAwait(false))
        {
            if(row is IFailed failed) throw new FailedException([failed]);
            return row;
        }
        throw Json.Abi();
    }
    private static async IAsyncEnumerable<T> One<T>(T record) { yield return record; await Task.CompletedTask; }
    private static async IAsyncEnumerable<T> Input<T>(IEnumerable<T> records)
    { foreach(var record in records) yield return record; await Task.CompletedTask; }
}
