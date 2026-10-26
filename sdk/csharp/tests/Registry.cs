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
        var expected = schema["codes"]!.AsArray()
            .Select(value => value!["name"]!.GetValue<string>()).ToHashSet(StringComparer.Ordinal);
        var actual = new HashSet<string>(StringComparer.Ordinal);
        foreach (ErrorCode code in Enum.GetValues<ErrorCode>())
        {
            string name = JsonSerializer.SerializeToNode(code, JsonOptions)!.GetValue<string>();
            string? engineName = Marshal.PtrToStringUTF8(RegistryNative.CodeName(Convert.ToInt32(code)));
            Check(engineName == name, $"generated code {code} differs from its stable native value");
            actual.Add(name);
        }
        Check(actual.SetEquals(expected), "generated error enum omitted or invented an engine code");
    }
}

internal static class RegistryNative
{
    [DllImport("thinkthen_ffi", EntryPoint = "thinkthen_code_name", CallingConvention = CallingConvention.Cdecl)]
    internal static extern nint CodeName(int code);
}
