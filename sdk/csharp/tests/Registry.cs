using System.Runtime.InteropServices;
using System.Text.Json;
using System.Text.Json.Nodes;
using ThinkThen;

internal static partial class Program
{
    private static async Task Registry(string registry)
    {
        await using var client = new ThinkThenClient(new() { Backend = Backend.Stub });
        await client.DecideAsync("Ready?", "alpha");
        // Compare against the engine-owned schema and native registry, not another C# list.
        JsonNode schema = JsonNode.Parse(await File.ReadAllTextAsync(registry))!;
        CheckEnum<Backend>(schema, "backends");
        CheckEnum<Function>(schema, "functions");
        CheckEnum<RowStatus>(schema, "statuses");
        Check(Enum.GetNames<ErrorFamily>().Select(name => name.ToLowerInvariant()).ToHashSet()
            .SetEquals(schema["codes"]!.AsArray().Select(value => value!["family"]!.GetValue<string>())),
            "error families differ from the contract");
        var expected = schema["codes"]!.AsArray()
            .Select(value => value!["name"]!.GetValue<string>()).ToHashSet(StringComparer.Ordinal);
        var actual = new HashSet<string>(StringComparer.Ordinal);
        foreach (ErrorCode code in Enum.GetValues<ErrorCode>())
        {
            string name = JsonSerializer.SerializeToNode(code, JsonOptions)!.GetValue<string>();
            string? engineName = Marshal.PtrToStringUTF8(RegistryNative.CodeName(Convert.ToInt32(code)));
            Check(schema["codes"]!.AsArray().Single(value => value!["name"]!.GetValue<string>() == name)!["value"]!.GetValue<int>() == (int)code, "error numeric value differs from contract");
            Check(engineName == name, $"generated code {code} differs from its stable native value");
            Check(code.Family().ToString().ToLowerInvariant() == schema["codes"]!.AsArray().Single(value => value!["name"]!.GetValue<string>() == name)!["family"]!.GetValue<string>(), "native error family differs from contract");
            actual.Add(name);
        }
        Check(actual.SetEquals(expected), "generated error enum omitted or invented an engine code");
    }
    private static void CheckEnum<T>(JsonNode schema, string key) where T : struct, Enum
    {
        var actual = Enum.GetValues<T>().ToDictionary(value => value.ToString().ToLowerInvariant(), value => Convert.ToInt32(value));
        var expected = schema[key]!.AsArray().ToDictionary(value => value!["name"]!.GetValue<string>(), value => value!["value"]!.GetValue<int>());
        Check(actual.Count == expected.Count && expected.All(pair => actual.TryGetValue(pair.Key, out int number) && number == pair.Value),
            $"{typeof(T).Name} names or values differ from contract");
    }

}

internal static class RegistryNative
{
    [DllImport("thinkthen_ffi", EntryPoint = "thinkthen_code_name", CallingConvention = CallingConvention.Cdecl)]
    internal static extern nint CodeName(int code);
}
