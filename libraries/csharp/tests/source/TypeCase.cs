using System;
using System.Text.Json;
using System.Collections.Generic;
using ThinkThen;

using var engine = Engine.Open();
try {
    string request = args[0];
    using JsonDocument input = JsonDocument.Parse(request);
    if (input.RootElement.TryGetProperty("usage", out _)) Console.WriteLine(engine.Call(request));
    else {
        CallResult result = engine.CallTyped(request);
        Console.WriteLine(JsonSerializer.Serialize(new Dictionary<string, JsonElement> { ["value"] = result.Value, ["facts"] = result.Facts }));
    }
} catch (Failure error) {
    Console.WriteLine(JsonSerializer.Serialize(new { error = error.Kind.ToString().ToLowerInvariant(), retryable = error.Retryable, facts = error.FactsJson }));
}
