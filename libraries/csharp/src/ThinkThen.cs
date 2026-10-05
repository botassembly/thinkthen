using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
using System.Text.Json;
using System.Threading;

namespace ThinkThen;

public enum Outcome { No = 0, Yes = 1, NotSure = 2 }
public enum FailureKind { Usage = 1, Backend = 2, Deadline = 3, Local = 4, Cancelled = 5, Defect = 6 }
[StructLayout(LayoutKind.Sequential)]
public struct Answer {
    public int Outcome;
    public double Probability;
    public Outcome OutcomeKind => Outcome switch { 0 => global::ThinkThen.Outcome.No, 1 => global::ThinkThen.Outcome.Yes, 2 => global::ThinkThen.Outcome.NotSure, _ => throw new InvalidOperationException("invalid native outcome") };
}
/// <summary>A call's value and its facts object. The result schema describes the facts.</summary>
public sealed record TypedResult<T>(T Value, JsonElement Facts);

/// <summary>One member of an annotate row's value or answers (ADR 0112 section 4).</summary>
public abstract record AnnotatedField
{
    private AnnotatedField() { }
    /// <summary>JSON null: the question was not sure.</summary>
    public sealed record Unresolved : AnnotatedField;
    public sealed record Answered(JsonElement Value) : AnnotatedField;
    /// <summary>The one-member object {"failed": {...}}.</summary>
    public sealed record Failed(string Kind, string Cause) : AnnotatedField;

    /// <summary>No answered value is an object, so an object that is not a failure is an error.</summary>
    public static AnnotatedField Read(JsonElement member)
    {
        if (member.ValueKind == JsonValueKind.Null) return new Unresolved();
        if (member.ValueKind != JsonValueKind.Object) return new Answered(member.Clone());
        if (!member.TryGetProperty("failed", out JsonElement failed) || failed.ValueKind != JsonValueKind.Object)
            throw new InvalidOperationException("annotate member is an object but not a failure");
        static string Text(JsonElement failed, string name) =>
            failed.TryGetProperty(name, out JsonElement value) && value.ValueKind == JsonValueKind.String ? value.GetString()! : "";
        return new Failed(Text(failed, "kind"), Text(failed, "cause"));
    }
}

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
    [DllImport(Library)] public static extern int thinkthen_decide_with_facts_opts(IntPtr engine, byte[] question, byte[] text, nuint textLength, long deadlineMs, IntPtr token, ref Answer answer, ref IntPtr facts, ref nuint factsLength);
    [DllImport(Library)] public static extern int thinkthen_decide_many_opts(IntPtr engine, byte[] question, IntPtr texts, IntPtr lengths, nuint count, long deadlineMs, IntPtr token, IntPtr answers);
    [DllImport(Library)] public static extern int thinkthen_decide_many_with_facts_opts(IntPtr engine, byte[] question, IntPtr texts, IntPtr lengths, nuint count, long deadlineMs, IntPtr token, IntPtr answers, ref IntPtr facts, ref nuint factsLength);
    [DllImport(Library)] public static extern IntPtr thinkthen_call_opts(IntPtr engine, byte[] request, long deadlineMs, IntPtr token);
    [DllImport(Library)] public static extern int thinkthen_recognize_opts(IntPtr engine, byte[] spec, byte[] text, nuint length, long deadlineMs, IntPtr token, ref IntPtr output, ref nuint outputLength);
    [DllImport(Library)] public static extern int thinkthen_recognize_with_facts_opts(IntPtr engine, byte[] spec, byte[] text, nuint length, long deadlineMs, IntPtr token, ref IntPtr output, ref nuint outputLength, ref IntPtr facts, ref nuint factsLength);
    [DllImport(Library)] public static extern int thinkthen_relate_opts(IntPtr engine, byte[] spec, IntPtr texts, IntPtr lengths, nuint count, long deadlineMs, IntPtr token, ref IntPtr output, ref nuint outputLength);
    [DllImport(Library)] public static extern int thinkthen_relate_with_facts_opts(IntPtr engine, byte[] spec, IntPtr texts, IntPtr lengths, nuint count, long deadlineMs, IntPtr token, ref IntPtr output, ref nuint outputLength, ref IntPtr facts, ref nuint factsLength);
    [DllImport(Library)] public static extern int thinkthen_plan_json(IntPtr engine, byte[] planJson, ref IntPtr output, ref nuint outputLength);
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
    private static byte[] CopyBytes(IntPtr pointer, nuint length)
    {
        if (pointer == IntPtr.Zero) throw new InvalidOperationException("missing native output");
        byte[] bytes = new byte[checked((int)length)];
        Marshal.Copy(pointer, bytes, 0, bytes.Length);
        return bytes;
    }
    private static JsonElement ReadJson(IntPtr pointer, nuint length)
    {
        using JsonDocument document = JsonDocument.Parse(CopyBytes(pointer, length));
        return document.RootElement.Clone();
    }
    private TResult Live<TResult>(Func<IntPtr, TResult> body)
    {
        lifetime.EnterReadLock();
        try
        {
            if (engine == IntPtr.Zero) throw new ObjectDisposedException(nameof(Engine));
            return body(engine);
        }
        finally { lifetime.ExitReadLock(); }
    }
    private TResult Invoke<TResult>(TimeSpan? budget, CancellationToken cancellation, Func<IntPtr, long, IntPtr, TResult> body) => Live(ptr =>
    {
        long deadline = budget.HasValue ? (long)Math.Max(0, Math.Ceiling(budget.Value.TotalMilliseconds)) : -1;
        IntPtr token = Native.thinkthen_cancel_token_new();
        if (token == IntPtr.Zero) throw new OutOfMemoryException("native token allocation failed");
        try
        {
            // Dispose waits for any running callback; native token remains live until the call returns.
            using (cancellation.Register(() => Native.thinkthen_cancel(token)))
                return body(ptr, deadline, token);
        }
        finally { Native.thinkthen_cancel_token_free(token); }
    });
    public TypedResult<Answer> Decide(string question, string text, TimeSpan? budget = null, CancellationToken cancellation = default)
    {
        byte[] q = CString(question), t = Text(text);
        return Invoke(budget, cancellation, (ptr, deadline, token) => {
            Answer answer = new() { Outcome = 123, Probability = -1.0 };
            IntPtr facts = IntPtr.Zero; nuint factsLength = 123;
            int rc = Native.thinkthen_decide_with_facts_opts(ptr, q, t, (nuint)t.Length, deadline, token, ref answer, ref facts, ref factsLength);
            if (rc != 0) { if (answer.Outcome != 123 || answer.Probability != -1.0 || facts != IntPtr.Zero || factsLength != 123) throw new InvalidOperationException("failed scalar changed output"); throw ReadFailure(ptr, rc); }
            try { return new TypedResult<Answer>(answer, ReadJson(facts, factsLength)); }
            finally { Native.thinkthen_free_string(facts); }
        });
    }
    public TypedResult<Answer[]> DecideMany(string question, params string[] texts) => DecideManyWithOptions(question, texts, null, default);
    public TypedResult<Answer[]> DecideManyWithOptions(string question, string[] texts, TimeSpan? budget, CancellationToken cancellation)
    {
        byte[] q = CString(question);
        byte[][] encoded = Array.ConvertAll(texts, Text);
        return Invoke(budget, cancellation, (ptr, deadline, token) => {
            IntPtr pointers = IntPtr.Zero, lengths = IntPtr.Zero, answers = IntPtr.Zero;
            IntPtr facts = IntPtr.Zero; nuint factsLength = 123;
            IntPtr[] entries = new IntPtr[texts.Length];
            int stride = Marshal.SizeOf<Answer>();
            int pointerBytes = checked(entries.Length * IntPtr.Size);
            int answerBytes = checked(entries.Length * stride);
            try
            {
                pointers = Marshal.AllocHGlobal(Math.Max(1, pointerBytes));
                lengths = Marshal.AllocHGlobal(Math.Max(1, pointerBytes));
                answers = Marshal.AllocHGlobal(Math.Max(1, answerBytes));
                for (int i = 0; i < entries.Length; i++)
                {
                    entries[i] = Marshal.AllocHGlobal(Math.Max(1, encoded[i].Length));
                    Marshal.Copy(encoded[i], 0, entries[i], encoded[i].Length);
                    Marshal.WriteIntPtr(pointers, checked(i * IntPtr.Size), entries[i]);
                    Marshal.WriteIntPtr(lengths, checked(i * IntPtr.Size), new IntPtr(encoded[i].Length));
                    Marshal.StructureToPtr(new Answer { Outcome = 123, Probability = -1 }, answers + checked(i * stride), false);
                }
                int rc = Native.thinkthen_decide_many_with_facts_opts(ptr, q, pointers, lengths, (nuint)entries.Length, deadline, token, answers, ref facts, ref factsLength);
                Answer[] result = new Answer[entries.Length];
                for (int i = 0; i < result.Length; i++) result[i] = Marshal.PtrToStructure<Answer>(answers + checked(i * stride));
                if (rc != 0)
                {
                    if (Array.Exists(result, item => item.Outcome != 123 || item.Probability != -1) || facts != IntPtr.Zero || factsLength != 123) throw new InvalidOperationException("failed bulk changed output");
                    throw ReadFailure(ptr, rc);
                }
                return new TypedResult<Answer[]>(result, ReadJson(facts, factsLength));
            }
            finally
            {
                foreach (IntPtr entry in entries) if (entry != IntPtr.Zero) Marshal.FreeHGlobal(entry);
                if (pointers != IntPtr.Zero) Marshal.FreeHGlobal(pointers);
                if (lengths != IntPtr.Zero) Marshal.FreeHGlobal(lengths);
                if (answers != IntPtr.Zero) Marshal.FreeHGlobal(answers);
                if (facts != IntPtr.Zero) Native.thinkthen_free_string(facts);
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
    /// <summary>Explicit native reader for any of the ten JSON question grammars.</summary>
    public TypedResult<JsonElement> Files(IReadOnlyDictionary<string, object?> question, string[] paths,
        string unit = "line", int? window = null, TimeSpan? budget = null, CancellationToken cancellation = default)
    {
        var request = new Dictionary<string, object?>(question);
        if (request.ContainsKey("source")) throw new ArgumentException("source is supplied by Files");
        var source = new Dictionary<string, object?> { ["paths"] = paths, ["unit"] = unit };
        if (window.HasValue) source["window"] = window.Value;
        request["source"] = source;
        return CallTyped(JsonSerializer.Serialize(request), budget, cancellation);
    }
    /// <summary>The JSON door's reply read as its value and facts; other members are ignored.</summary>
    public TypedResult<JsonElement> CallTyped(string request, TimeSpan? budget = null, CancellationToken cancellation = default)
    {
        using JsonDocument document = JsonDocument.Parse(Call(request, budget, cancellation));
        JsonElement root = document.RootElement;
        return new TypedResult<JsonElement>(root.GetProperty("value").Clone(), root.GetProperty("facts").Clone());
    }
    /// <summary>
    /// Preview a judgment call through thinkthen_plan_json and return the result schema's plan object.
    /// verb is decide, choose, score or tag. question is bare question text, or one question object
    /// when it starts with "{", as Decide reads it. settingsJson is null or a thinkthen.settings/1 object.
    /// The preview needs no key, reads no cache and sends nothing.
    /// </summary>
    public JsonElement Plan(string verb, string question, string[] input, string? settingsJson = null)
    {
        var buffer = new System.IO.MemoryStream();
        try
        {
            using var writer = new Utf8JsonWriter(buffer);
            writer.WriteStartObject();
            writer.WriteString("verb", verb);
            writer.WritePropertyName("question");
            if (question.TrimStart().StartsWith('{')) writer.WriteRawValue(question); else writer.WriteStringValue(question);
            writer.WriteStartArray("input");
            foreach (string text in input) writer.WriteStringValue(text);
            writer.WriteEndArray();
            if (settingsJson is not null) { writer.WritePropertyName("settings"); writer.WriteRawValue(settingsJson); }
            writer.WriteEndObject();
        }
        catch (JsonException error) { throw new Failure((int)FailureKind.Usage, false, error.Message, null); }
        byte[] request = CString(Encoding.UTF8.GetString(buffer.ToArray()));
        return Live(ptr => {
            IntPtr output = IntPtr.Zero; nuint length = 0;
            int rc = Native.thinkthen_plan_json(ptr, request, ref output, ref length);
            if (rc != 0) throw ReadFailure(ptr, rc);
            try { return ReadJson(output, length); }
            finally { Native.thinkthen_free_string(output); }
        });
    }
    public TypedResult<JsonElement> Recognize(string spec, string text, TimeSpan? budget = null, CancellationToken cancellation = default)
    {
        byte[] q = CString(spec), t = Text(text);
        return Invoke(budget, cancellation, (ptr, deadline, token) => {
            IntPtr output = IntPtr.Zero, facts = IntPtr.Zero; nuint length = 123, factsLength = 123;
            int rc = Native.thinkthen_recognize_with_facts_opts(ptr, q, t, (nuint)t.Length, deadline, token, ref output, ref length, ref facts, ref factsLength);
            if (rc != 0) { if (output != IntPtr.Zero || length != 123 || facts != IntPtr.Zero || factsLength != 123) throw new InvalidOperationException("failed recognize changed output"); throw ReadFailure(ptr, rc); }
            try { return new TypedResult<JsonElement>(ReadJson(output, length), ReadJson(facts, factsLength)); }
            finally { try { Native.thinkthen_free_string(output); } finally { Native.thinkthen_free_string(facts); } }
        });
    }
    public TypedResult<JsonElement> Relate(string spec, params string[] records) => RelateWithOptions(spec, records, null, default);
    public TypedResult<JsonElement> RelateWithOptions(string spec, string[] records, TimeSpan? budget, CancellationToken cancellation)
    {
        byte[] q = CString(spec);
        byte[][] encoded = Array.ConvertAll(records, Text);
        return Invoke(budget, cancellation, (ptr, deadline, token) => {
            IntPtr pointers = IntPtr.Zero, lengths = IntPtr.Zero, output = IntPtr.Zero, facts = IntPtr.Zero;
            nuint outputLength = 123, factsLength = 123;
            IntPtr[] entries = new IntPtr[encoded.Length];
            int pointerBytes = checked(entries.Length * IntPtr.Size);
            try
            {
                pointers = Marshal.AllocHGlobal(Math.Max(1, pointerBytes));
                lengths = Marshal.AllocHGlobal(Math.Max(1, pointerBytes));
                for (int i = 0; i < entries.Length; i++)
                {
                    entries[i] = Marshal.AllocHGlobal(Math.Max(1, encoded[i].Length));
                    Marshal.Copy(encoded[i], 0, entries[i], encoded[i].Length);
                    Marshal.WriteIntPtr(pointers, checked(i * IntPtr.Size), entries[i]);
                    Marshal.WriteIntPtr(lengths, checked(i * IntPtr.Size), new IntPtr(encoded[i].Length));
                }
                int rc = Native.thinkthen_relate_with_facts_opts(ptr, q, pointers, lengths, (nuint)entries.Length, deadline, token, ref output, ref outputLength, ref facts, ref factsLength);
                if (rc != 0) { if (output != IntPtr.Zero || outputLength != 123 || facts != IntPtr.Zero || factsLength != 123) throw new InvalidOperationException("failed relate changed output"); throw ReadFailure(ptr, rc); }
                return new TypedResult<JsonElement>(ReadJson(output, outputLength), ReadJson(facts, factsLength));
            }
            finally
            {
                try { if (output != IntPtr.Zero) Native.thinkthen_free_string(output); }
                finally { if (facts != IntPtr.Zero) Native.thinkthen_free_string(facts); }
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
