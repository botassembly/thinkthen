using System;
using System.Runtime.InteropServices;
using System.Text;
using System.Text.Json;
using System.Threading;
using System.Linq;

namespace ThinkThen;

public enum Outcome { No = 0, Yes = 1, NotSure = 2 }
public enum FailureKind { Usage = 1, Backend = 2, Deadline = 3, Local = 4, Cancelled = 5, Defect = 6 }
[StructLayout(LayoutKind.Sequential)]
public struct Answer {
    public int Outcome;
    public double Probability;
    public Outcome OutcomeKind => Outcome switch { 0 => global::ThinkThen.Outcome.No, 1 => global::ThinkThen.Outcome.Yes, 2 => global::ThinkThen.Outcome.NotSure, _ => throw new InvalidOperationException("invalid native outcome") };
}
public sealed record CallResult(JsonElement Value, JsonElement Facts);

public static class Native
{
    const string Library = "libthinkthen.so.0";
    [DllImport(Library)] public static extern IntPtr thinkthen_engine_new();
    [DllImport(Library)] public static extern IntPtr thinkthen_engine_new_with(byte[] settings);
    [DllImport(Library)] public static extern void thinkthen_engine_free(IntPtr engine);
    [DllImport(Library)] public static extern IntPtr thinkthen_cancel_token_new();
    [DllImport(Library)] public static extern void thinkthen_cancel(IntPtr token);
    [DllImport(Library)] public static extern void thinkthen_cancel_token_free(IntPtr token);
    [DllImport(Library)] public static extern int thinkthen_error_code(IntPtr engine);
    [DllImport(Library)] public static extern int thinkthen_error_retryable(IntPtr engine);
    [DllImport(Library)] public static extern IntPtr thinkthen_error_message(IntPtr engine);
    [DllImport(Library)] public static extern IntPtr thinkthen_error_facts_json(IntPtr engine);
    [DllImport(Library)] public static extern int thinkthen_decide_opts(IntPtr engine, byte[] question, byte[] text, nuint textLength, long deadlineMs, IntPtr token, ref Answer answer);
    [DllImport(Library)] public static extern int thinkthen_decide_many_opts(IntPtr engine, byte[] question, IntPtr texts, IntPtr lengths, nuint count, long deadlineMs, IntPtr token, IntPtr answers);
    [DllImport(Library)] public static extern IntPtr thinkthen_call_opts(IntPtr engine, byte[] request, long deadlineMs, IntPtr token);
    [DllImport(Library)] public static extern int thinkthen_recognize_opts(IntPtr engine, byte[] spec, byte[] text, nuint length, long deadlineMs, IntPtr token, ref IntPtr output, ref nuint outputLength);
    [DllImport(Library)] public static extern int thinkthen_relate_opts(IntPtr engine, byte[] spec, IntPtr texts, IntPtr lengths, nuint count, long deadlineMs, IntPtr token, ref IntPtr output, ref nuint outputLength);
    [DllImport(Library)] public static extern void thinkthen_free_string(IntPtr value);
}

public sealed class Failure : Exception
{
    public int Code { get; }
    public FailureKind Kind { get; }
    public bool Retryable { get; }
    public string? FactsJson { get; }
    public Failure(int code, bool retryable, string message, string? factsJson) : base(message) {
        if (!Enum.IsDefined(typeof(FailureKind), code)) throw new InvalidOperationException("invalid native failure kind");
        Code = code; Kind = (FailureKind)code; Retryable = retryable; FactsJson = factsJson;
    }
}

public sealed class Engine : IDisposable
{
    private readonly ReaderWriterLockSlim lifetime = new();
    private IntPtr engine;
    private static readonly Encoding StrictUtf8 = new UTF8Encoding(false, true);
    private Engine(IntPtr engine) => this.engine = engine;
    public static Engine Open(string? settingsJson = null)
    {
        IntPtr ptr = settingsJson is null ? Native.thinkthen_engine_new() : Native.thinkthen_engine_new_with(CString(settingsJson));
        if (ptr == IntPtr.Zero) throw ReadFailure(ptr, Native.thinkthen_error_code(ptr));
        return new Engine(ptr);
    }
    public static byte[] CString(string value)
    {
        ArgumentNullException.ThrowIfNull(value);
        if (value.Contains('\0')) throw new ArgumentException("embedded NUL in C string", nameof(value));
        byte[] bytes = StrictUtf8.GetBytes(value);
        Array.Resize(ref bytes, bytes.Length + 1);
        return bytes;
    }
    public static byte[] Text(string value) => StrictUtf8.GetBytes(value);
    private static Failure ReadFailure(IntPtr engine, int code)
    {
        int observed = Native.thinkthen_error_code(engine);
        int retryable = Native.thinkthen_error_retryable(engine);
        string message = Marshal.PtrToStringUTF8(Native.thinkthen_error_message(engine)) ?? "missing native error";
        string? facts = Marshal.PtrToStringUTF8(Native.thinkthen_error_facts_json(engine));
        if (observed != code) throw new InvalidOperationException($"native error mismatch {code}/{observed}");
        return new Failure(code, retryable != 0, message, facts);
    }
    private TResult Invoke<TResult>(TimeSpan? budget, CancellationToken cancellation, Func<IntPtr, long, IntPtr, TResult> body)
    {
        lifetime.EnterReadLock();
        try
        {
            if (engine == IntPtr.Zero) throw new ObjectDisposedException(nameof(Engine));
            long deadline = budget.HasValue ? (long)Math.Max(0, Math.Ceiling(budget.Value.TotalMilliseconds)) : -1;
            IntPtr token = Native.thinkthen_cancel_token_new();
            if (token == IntPtr.Zero) throw new OutOfMemoryException("native token allocation failed");
            try
            {
                // Dispose waits for any running callback; native token remains live until the call returns.
                using (cancellation.Register(() => Native.thinkthen_cancel(token)))
                    return body(engine, deadline, token);
            }
            finally { Native.thinkthen_cancel_token_free(token); }
        }
        finally { lifetime.ExitReadLock(); }
    }
    public Answer Decide(string question, string text, TimeSpan? budget = null, CancellationToken cancellation = default)
    {
        byte[] q = CString(question), t = Text(text);
        return Invoke(budget, cancellation, (ptr, deadline, token) => {
            Answer answer = new() { Outcome = 123, Probability = -1.0 };
            int rc = Native.thinkthen_decide_opts(ptr, q, t, (nuint)t.Length, deadline, token, ref answer);
            if (rc != 0) { if (answer.Outcome != 123 || answer.Probability != -1.0) throw new InvalidOperationException("failed scalar changed output"); throw ReadFailure(ptr, rc); }
            return answer;
        });
    }
    public Answer[] DecideMany(string question, params string[] texts) => DecideManyWithOptions(question, texts, null, default);
    public Answer[] DecideManyWithOptions(string question, string[] texts, TimeSpan? budget, CancellationToken cancellation)
    {
        byte[] q = CString(question);
        byte[][] encoded = Array.ConvertAll(texts, Text);
        return Invoke(budget, cancellation, (ptr, deadline, token) => {
            IntPtr pointers = IntPtr.Zero, lengths = IntPtr.Zero, answers = IntPtr.Zero;
            IntPtr[] entries = new IntPtr[texts.Length];
            int stride = Marshal.SizeOf<Answer>();
            try
            {
                pointers = Marshal.AllocHGlobal(Math.Max(1, entries.Length * IntPtr.Size));
                lengths = Marshal.AllocHGlobal(Math.Max(1, entries.Length * IntPtr.Size));
                answers = Marshal.AllocHGlobal(Math.Max(1, entries.Length * stride));
                for (int i = 0; i < entries.Length; i++)
                {
                    entries[i] = Marshal.AllocHGlobal(Math.Max(1, encoded[i].Length));
                    Marshal.Copy(encoded[i], 0, entries[i], encoded[i].Length);
                    Marshal.WriteIntPtr(pointers, i * IntPtr.Size, entries[i]);
                    Marshal.WriteIntPtr(lengths, i * IntPtr.Size, new IntPtr(encoded[i].Length));
                    Marshal.StructureToPtr(new Answer { Outcome = 123, Probability = -1 }, answers + i * stride, false);
                }
                int rc = Native.thinkthen_decide_many_opts(ptr, q, pointers, lengths, (nuint)entries.Length, deadline, token, answers);
                Answer[] result = new Answer[entries.Length];
                for (int i = 0; i < result.Length; i++) result[i] = Marshal.PtrToStructure<Answer>(answers + i * stride);
                if (rc != 0)
                {
                    if (Array.Exists(result, item => item.Outcome != 123 || item.Probability != -1)) throw new InvalidOperationException("failed bulk changed output");
                    throw ReadFailure(ptr, rc);
                }
                return result;
            }
            finally
            {
                foreach (IntPtr entry in entries) if (entry != IntPtr.Zero) Marshal.FreeHGlobal(entry);
                if (pointers != IntPtr.Zero) Marshal.FreeHGlobal(pointers);
                if (lengths != IntPtr.Zero) Marshal.FreeHGlobal(lengths);
                if (answers != IntPtr.Zero) Marshal.FreeHGlobal(answers);
            }
        });
    }
    public string Call(string request, TimeSpan? budget = null, CancellationToken cancellation = default)
    {
        byte[] q = CString(request);
        return Invoke(budget, cancellation, (ptr, deadline, token) => {
            IntPtr output = Native.thinkthen_call_opts(ptr, q, deadline, token);
            if (output == IntPtr.Zero) throw ReadFailure(ptr, Native.thinkthen_error_code(ptr));
            try { return Marshal.PtrToStringUTF8(output) ?? throw new InvalidOperationException("null string"); }
            finally { Native.thinkthen_free_string(output); }
        });
    }
    public CallResult CallTyped(string request, TimeSpan? budget = null, CancellationToken cancellation = default)
    {
        using JsonDocument document = JsonDocument.Parse(Call(request, budget, cancellation));
        JsonElement root = document.RootElement;
        if (!root.TryGetProperty("value", out JsonElement value) || !root.TryGetProperty("facts", out JsonElement facts)
            || root.EnumerateObject().Count() != 2 || facts.ValueKind != JsonValueKind.Object)
            throw new InvalidOperationException("invalid native result envelope");
        return new CallResult(value.Clone(), facts.Clone());
    }
    public string Recognize(string spec, string text, TimeSpan? budget = null, CancellationToken cancellation = default)
    {
        byte[] q = CString(spec), t = Text(text);
        return Invoke(budget, cancellation, (ptr, deadline, token) => {
            IntPtr output = IntPtr.Zero; nuint length = unchecked((nuint)123);
            int rc = Native.thinkthen_recognize_opts(ptr, q, t, (nuint)t.Length, deadline, token, ref output, ref length);
            if (rc != 0) { if (output != IntPtr.Zero || length != 123) throw new InvalidOperationException("failed recognize changed output"); throw ReadFailure(ptr, rc); }
            try { byte[] result = new byte[checked((int)length)]; Marshal.Copy(output, result, 0, result.Length); return StrictUtf8.GetString(result); }
            finally { Native.thinkthen_free_string(output); }
        });
    }
    public string Relate(string spec, params string[] records)
    {
        byte[] q = CString(spec);
        byte[][] encoded = Array.ConvertAll(records, Text);
        return Invoke(null, default, (ptr, deadline, token) => {
            IntPtr pointers = IntPtr.Zero, lengths = IntPtr.Zero, output = IntPtr.Zero;
            nuint outputLength = 123;
            IntPtr[] entries = new IntPtr[encoded.Length];
            try
            {
                pointers = Marshal.AllocHGlobal(Math.Max(1, entries.Length * IntPtr.Size));
                lengths = Marshal.AllocHGlobal(Math.Max(1, entries.Length * IntPtr.Size));
                for (int i = 0; i < entries.Length; i++)
                {
                    entries[i] = Marshal.AllocHGlobal(Math.Max(1, encoded[i].Length));
                    Marshal.Copy(encoded[i], 0, entries[i], encoded[i].Length);
                    Marshal.WriteIntPtr(pointers, i * IntPtr.Size, entries[i]);
                    Marshal.WriteIntPtr(lengths, i * IntPtr.Size, new IntPtr(encoded[i].Length));
                }
                int rc = Native.thinkthen_relate_opts(ptr, q, pointers, lengths, (nuint)entries.Length, deadline, token, ref output, ref outputLength);
                if (rc != 0) { if (output != IntPtr.Zero || outputLength != 123) throw new InvalidOperationException("failed relate changed output"); throw ReadFailure(ptr, rc); }
                byte[] bytes = new byte[checked((int)outputLength)];
                Marshal.Copy(output, bytes, 0, bytes.Length);
                return StrictUtf8.GetString(bytes);
            }
            finally
            {
                if (output != IntPtr.Zero) Native.thinkthen_free_string(output);
                foreach (IntPtr entry in entries) if (entry != IntPtr.Zero) Marshal.FreeHGlobal(entry);
                if (pointers != IntPtr.Zero) Marshal.FreeHGlobal(pointers);
                if (lengths != IntPtr.Zero) Marshal.FreeHGlobal(lengths);
            }
        });
    }
    public void Dispose()
    {
        lifetime.EnterWriteLock();
        try { if (engine != IntPtr.Zero) { Native.thinkthen_engine_free(engine); engine = IntPtr.Zero; } }
        finally { lifetime.ExitWriteLock(); }
    }
}
