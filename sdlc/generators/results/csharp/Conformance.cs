using System;
using System.IO;
using System.Linq;
using System.Text.Json;
using System.Text.Json.Nodes;
using ThinkThen.Results;

static class Conformance
{
    static void Check(bool condition, string behavior)
    {
        if (!condition) throw new Exception(behavior);
    }

    static Facts Read(JsonNode node)
    {
        using var document = JsonDocument.Parse(node.ToJsonString());
        return new Facts(document.RootElement);
    }

    static void RoundTrip(JsonNode source)
    {
        var facts = Read(source);
        Check(JsonNode.DeepEquals(source, facts.ToPlain()), "plain conversion retains facts");
        Check(JsonNode.DeepEquals(source, JsonNode.Parse(facts.ToJsonString())), "round trip retains facts");
        Check(JsonNode.DeepEquals(source, JsonNode.Parse(facts.ToJson().GetRawText())), "JSON ownership survives document disposal");
    }

    public static void Main(string[] args)
    {
        var fixture = JsonNode.Parse(File.ReadAllText(args[0]))!.AsObject();
        RoundTrip(fixture["facts"]!);
        // Final failure facts use exactly the same generated carrier.
        RoundTrip(fixture["started_error"]!["facts"]!);
        var observations = fixture["observed_facts"]!.AsArray();
        foreach (var observation in observations)
        {
            var source = observation!.AsObject();
            RoundTrip(source);
            var facts = Read(source);
            Check(facts.LargestRequestBytes == source["largest_request_bytes"]!.GetValue<ulong>(), "typed byte count");
            Check(facts.TokenEstimateMethod == source["token_estimate_method"]!.GetValue<string>(), "typed estimate method");
            var expected = source["largest_request_estimated_input_tokens"];
            Check(facts.LargestRequestEstimatedInputTokens.State == (expected is null ? PresenceState.Null : PresenceState.Value), "nullable estimated count retains presence");
            if (expected is not null)
                Check(facts.LargestRequestEstimatedInputTokens.Value == expected.GetValue<ulong>(), "zero and nonzero estimates");
            var persistence = facts.UsagePersistence.Value;
            Check(persistence.State.Value == source["usage_persistence"]!["state"]!.GetValue<string>(), "typed persistence state");
            Check(persistence.ObservedAt == "facts_snapshot", "typed persistence observation");
            if (persistence.State == UsagePersistence.Failed)
                Check(persistence.Advice.Value == source["usage_persistence"]!["advice"]!.GetValue<string>(), "failure advice survives");
        }
        var future = observations[0]!.DeepClone().AsObject();
        future["future"] = JsonNode.Parse("{\"proxy\":{\"enabled\":true},\"null\":null,\"values\":[false,0,18446744073709551615]}");
        future["usage_persistence"]!["future"] = JsonNode.Parse("{\"nested\":[null,false]}");
        future["held_model_mismatch"] = false;
        RoundTrip(future);
        var owned = Read(future);
        Check(!owned.HeldModelMismatch.Value, "false is a present value");
        Check(JsonNode.DeepEquals(future["usage_persistence"], owned.UsagePersistence.Value.ToPlain()), "nested unknown members survive typed conversion");
        future["future"] = null;
        Check(owned.ToPlain()["future"] is JsonObject, "caller mutation cannot alter result");
        future.Remove("model");
        Check(Read(future).Model.State == PresenceState.Missing, "missing optional field");
        future["model"] = null;
        Check(Read(future).Model.State == PresenceState.Null, "explicit optional null");
        future["model"] = "reported";
        Check(Read(future).Model.Value == "reported", "present optional value");
        future["usage_persistence"]!["state"] = "future_state";
        Check(Read(future).UsagePersistence.Value.State.Value == "future_state", "future enum data survives without activating behavior");
        future["attempts"] = fixture["started_error"]!["attempts"]!.DeepClone();
        var attemptSource = future["attempts"]![0]!.AsObject();
        attemptSource["future"] = JsonNode.Parse("{\"nested\":[null,false]}");
        var attempt = Read(future).Attempts.Value.Single();
        Check(attempt.Outcome == AttemptOutcome.Status && attempt.Status.Value == 429, "typed failure attempt");
        Check(JsonNode.DeepEquals(attemptSource, attempt.ToPlain()), "unknown attempt members survive");
        RoundTrip(future);
        Console.WriteLine("Generated C# facts: presence, observations, failure and unknown round trips pass");
    }
}
