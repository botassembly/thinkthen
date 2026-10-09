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

    static T Read<T>(JsonNode source, Func<JsonElement, T> read)
    {
        using var document = JsonDocument.Parse(source.ToJsonString());
        return read(document.RootElement);
    }

    static void Retains(JsonNode source, ResultObject result)
    {
        Check(JsonNode.DeepEquals(source, result.ToPlain()), "variant retains all nested members after disposal");
        Check(JsonNode.DeepEquals(source, JsonNode.Parse(result.ToJsonString())), "variant round trip retains JSON");
        Check(JsonNode.DeepEquals(source, JsonNode.Parse(result.ToJson().GetRawText())), "variant owns JSON after disposal");
    }

    static void IdentityVariants<T>(Func<JsonElement, T> read,
        params (string Member, Func<T, string> Value)[] identities) where T : ResultObject
    {
        foreach (var (member, value) in identities)
        {
            var source = new JsonObject { [member] = "future identity", ["future"] = JsonNode.Parse("{\"nested\":[null,false,0]}") };
            var result = Read(source, read);
            Check(value(result) == "future identity", "string identity selects its typed variant without pattern validation");
            Retains(source, result);
            foreach (var text in new[] { "null", "0", "{}", "[]", "true" })
            {
                source[member] = JsonNode.Parse(text);
                try { Read(source, read); throw new Exception("null or wrong JSON kind selected an identity variant"); }
                catch (JsonException) { }
                var other = identities.Single(identity => identity.Member != member).Member;
                source[other] = "other identity";
                try { Read(source, read); throw new Exception("invalid identity beside a valid identity accepted"); }
                catch (JsonException) { }
                source.Remove(other);
            }
        }
        foreach (var source in new[] {
            new JsonObject(), new JsonObject { ["future"] = new JsonObject() },
            new JsonObject { [identities[0].Member] = "first", [identities[1].Member] = "second" } })
        {
            try { Read(source, read); throw new Exception("ambiguous or unknown identity accepted"); }
            catch (JsonException) { }
        }
    }

    static void Variants(JsonObject fixture)
    {
        foreach (var row in fixture["results"]!.AsArray())
        {
            var source = row!["result"]!["answer"];
            if (source is null) continue;
            var tag = source["kind"]?.GetValue<string>();
            if (tag is null) continue;
            source = source.DeepClone();
            source["future"] = JsonNode.Parse("{\"nested\":[false,null,0]}");
            if (tag == "find")
            {
                var find = Read(source, value => new FindAnswer(value));
                Check(find.Pick == source["pick"]!.GetValue<string>(), "typed find answer");
                Check(find.Probabilities.Count == source["probabilities"]!.AsObject().Count, "find map");
                Retains(source, find);
                continue;
            }
            var answer = Read(source, Answer.Read);
            switch (answer)
            {
                case AnswerYesNo yesNo:
                    Check(yesNo.Probability == source["probability"]!.GetValue<double>(), "yes/no variant probability");
                    break;
                case AnswerChoice choice:
                    Check(choice.Pick == source["pick"]!.GetValue<string>(), "choice variant pick");
                    Check(choice.Probabilities.All(entry => entry.Value == source["probabilities"]![entry.Key]!.GetValue<double>()), "typed choice map");
                    source["confidence"] = null;
                    Check(((AnswerChoice)Read(source, Answer.Read)).Confidence.State == PresenceState.Null, "variant explicit null");
                    source.AsObject().Remove("confidence");
                    Check(((AnswerChoice)Read(source, Answer.Read)).Confidence.State == PresenceState.Missing, "variant missing confidence");
                    source["confidence"] = 0;
                    Check(((AnswerChoice)Read(source, Answer.Read)).Confidence.Value == 0, "variant zero confidence");
                    source["probabilities"]!["future label"] = 0;
                    var map = ((AnswerChoice)Read(source, Answer.Read)).Probabilities;
                    Check(map["future label"] == 0, "map retains arbitrary keys and zero values");
                    Check(map is System.Collections.Generic.IDictionary<string, double> mutable && mutable.IsReadOnly, "map prevents caller mutation");
                    break;
                case AnswerTag tagged:
                    Check(tagged.Probabilities.Count == source["probabilities"]!.AsObject().Count, "tag variant map");
                    break;
                case AnswerScore score:
                    Check(score.Level == source["level"]!.GetValue<string>(), "score variant level");
                    Check(score.Probabilities.Count == source["probabilities"]!.AsObject().Count, "score variant map");
                    break;
                default: throw new Exception("known answer did not select its variant");
            }
            // The owned view still retains its original source after caller mutation.
            source = JsonNode.Parse(answer.ToJsonString())!;
            Retains(source, answer);
        }
        var relation = fixture["results"]!.AsArray().Single(row => row!["type"]!.GetValue<string>() == "RelateResult")!["result"]!;
        foreach (var source in relation["answer"]!["questions"]!.AsArray())
        {
            var member = Read(source!, RelationMember.Read);
            Check(member is RelationMemberAnswerId or RelationMemberFailureId, "relation identity selects typed member");
            if (member is RelationMemberAnswerId success)
            {
                Check(success.Accepted && success.Probability == source!["probability"]!.GetValue<double>(), "typed successful relation member");
                Check(success.Source.Name == source!["source"]!["name"]!.GetValue<string>(), "typed relation endpoint");
                Retains(source!["source"]!, success.Source);
            }
            if (member is RelationMemberFailureId failure)
            {
                Check(failure.Failure.Kind == "backend" && failure.Failure.Cause == FailureCause.MissingAnswer, "typed member failure and enum literal");
                Check(failure.Target.State == PresenceState.Null, "nullable relation endpoint");
            }
            Retains(source!, member);
        }
        foreach (var source in relation["meta"]!["observations"]!.AsArray())
        {
            var observation = Read(source!, Observation.Read);
            if (source!["observation_id"] is not null)
                Check(observation is ObservationObservationId success && success.ObservationId == source["observation_id"]!.GetValue<string>(), "success observation identity");
            else
                Check(observation is ObservationFailureId failure && failure.FailureId == source["failure_id"]!.GetValue<string>(), "failure observation identity");
            Retains(source!, observation);
        }
        foreach (var source in fixture["errors"]!.AsArray())
        {
            var error = Read(source!, value => new Error(value));
            Check(error.Kind.Value == source!["kind"]!.GetValue<string>() && !error.Retryable, "typed error kind and false retryability");
            Retains(source!, error);
        }
        var callSource = new JsonObject { ["error"] = fixture["started_error"]!.DeepClone(), ["facts"] = fixture["facts"]!.DeepClone() };
        var call = Read(callSource, value => new CallError(value));
        Check(call.Error.Kind == FailureKind.Backend && call.Facts.State == PresenceState.Value, "typed call failure facts");
        Retains(callSource, call);
        Retains(callSource["error"]!, call.Error);
        var usageSource = relation["meta"]!["usage"]!;
        var usage = Read(usageSource, value => new Usage(value));
        Check(usage.InputTokens.Value == 0 && usage.OutputTokens.Value == 0, "required-token assertion preserves both optional token fields");
        foreach (var text in new[] {
            "{\"kind\":\"initial_request\",\"limit\":0}",
            "{\"kind\":\"additional_request\",\"limit\":0}",
            "{\"kind\":\"retry\",\"limit\":0,\"last_status\":429}" })
        {
            var source = JsonNode.Parse(text)!;
            source["future"] = new JsonObject { ["nested"] = false };
            var denial = Read(source, EstimatedInputDenial.Read);
            Check(source["kind"]!.GetValue<string>() switch {
                "initial_request" => denial is EstimatedInputDenialInitialRequest initial && initial.Limit == 0,
                "additional_request" => denial is EstimatedInputDenialAdditionalRequest additional && additional.Limit == 0,
                "retry" => denial is EstimatedInputDenialRetry retry && retry.LastStatus == 429 && retry.Limit == 0,
                _ => false }, "estimated input denial tag and fields");
            Retains(source, denial);
        }
        foreach (var text in new[] {
            "{\"kind\":\"before_first_send\"}", "{\"kind\":\"before_additional_send\"}",
            "{\"kind\":\"before_retry\",\"last_status\":429}" })
        {
            var source = JsonNode.Parse(text)!;
            var denial = Read(source, SendBudgetDenial.Read);
            Check(source["kind"]!.GetValue<string>() switch {
                "before_first_send" => denial is SendBudgetDenialBeforeFirstSend,
                "before_additional_send" => denial is SendBudgetDenialBeforeAdditionalSend,
                "before_retry" => denial is SendBudgetDenialBeforeRetry retry && retry.LastStatus == 429,
                _ => false }, "send budget denial tag and fields");
            Retains(source, denial);
        }
        IdentityVariants(Observation.Read,
            ("observation_id", value => ((ObservationObservationId)value).ObservationId),
            ("failure_id", value => ((ObservationFailureId)value).FailureId));
        IdentityVariants(RelationMember.Read,
            ("answer_id", value => ((RelationMemberAnswerId)value).AnswerId),
            ("failure_id", value => ((RelationMemberFailureId)value).FailureId));
        try { Read(JsonNode.Parse("{\"kind\":\"future_kind\"}")!, Answer.Read); throw new Exception("unknown tag accepted as known variant"); }
        catch (JsonException) { }
    }

    static void NativePresentation(JsonObject fixture)
    {
        var questions = fixture["native_presentation"]!["questions"]!.AsArray();
        var choice = (QuestionChoose)Read(questions[0]!, Question.Read);
        Check(choice.Options.SequenceEqual(new[] { "second", "first" }), "native choice order survives");
        Check(choice.LabelDetails.Value[0].Description.Value.GetProperty("z").ValueKind == JsonValueKind.Null && choice.LabelDetails.Value[1].Description.State == PresenceState.Missing, "native description content and absence survive");
        Check(choice.Model.Value == "fixed" && choice.Profile.Value == "authored" && choice.Batch.Value is BatchString maximum && maximum.Value == "max" && choice.On.Value.Single() == "", "native authored reading survives");
        var score = (QuestionScore)Read(questions[1]!, Question.Read);
        Check(score.Levels.SequenceEqual(new[] { "low", "high" }) && score.LabelDetails.Value[0].Description.Value.GetString() == "Lower." && score.LabelDetails.Value[1].Description.Value.GetString() == "Higher.", "native typed score retains descriptions");
        var nullScore = (QuestionScore)Read(questions[2]!, Question.Read);
        Check(nullScore.LabelDetails.Value[0].Description.State == PresenceState.Null && nullScore.LabelDetails.Value[1].Description.Value.ValueKind == JsonValueKind.Array, "native null score description differs from absence");
        var tag = (QuestionTag)Read(questions[3]!, Question.Read);
        Check(tag.LabelDetails.Value[0].Description.Value.GetString() == "Topic." && tag.LabelDetails.Value[1].Description.State == PresenceState.Missing, "native tag retains described and bare labels");
        var decision = (QuestionDecide)Read(questions[4]!, Question.Read);
        Check(decision.Model.Value == "fixed" && decision.Profile.State == PresenceState.Missing && decision.Batch.Value is BatchInteger records && records.Value == 2 && decision.On.Value.Single() == "", "native numeric batch and absent profile survive");
        Check(decision.ItemSchema.Value is InputDeclarationString, "native declared input remains typed");
        for (var at = 0; at < questions.Count; at++) Retains(questions[at]!, Read(questions[at]!, Question.Read));
        var endpoints = fixture["native_presentation"]!["endpoints"]!.AsArray();
        for (var at = 0; at < endpoints.Count; at++)
        {
            var endpoint = Read(endpoints[at]!, value => new SourceRelationEndpoint(value));
            Check(endpoint.Ordinal == (ulong)new[] { 0, 1, 2, 1 }[at], "native endpoint ordinal survives duplicate expansion");
            Retains(endpoints[at]!, endpoint);
        }
    }

    public static void Main(string[] args)
    {
        var fixture = JsonNode.Parse(File.ReadAllText(args[0]))!.AsObject();
        var annotation = fixture["results"]!.AsArray().Single(row => row!["type"]!.GetValue<string>() == "AnnotateResult")!["result"]!;
        var member = annotation["answers"]!["ok"]!;
        var threshold = Read(member["threshold"]!, Threshold.Read);
        Check(threshold is ThresholdString band && band.Value == "0.3:0.7", "typed string threshold retains authored band");
        Check(JsonNode.DeepEquals(member["threshold"], threshold.ToPlain()), "threshold owns primitive JSON after disposal");
        var relation = fixture["results"]!.AsArray().Single(row => row!["type"]!.GetValue<string>() == "RelateResult")!["result"]!;
        var cut = Read(relation["question"]!["threshold"]!, Threshold.Read);
        Check(cut is ThresholdNumber number && number.Value == 0.5, "typed numeric threshold retains cut");
        Check(JsonNode.DeepEquals(relation["question"]!["threshold"], JsonNode.Parse(cut.ToJsonString())), "numeric threshold round trips");
        foreach (var invalid in new[] { "null", "true", "[]", "{}" })
        {
            using var document = JsonDocument.Parse(invalid);
            try { Threshold.Read(document.RootElement); throw new Exception("unsupported threshold kind accepted"); }
            catch (JsonException) { }
        }
        var question = (QuestionDecide)Read(member["question"]!, Question.Read);
        Check(question.Text.ValueKind == JsonValueKind.Array, "authored question content remains an owned array");
        Check(question.True.State == PresenceState.Null && question.False.Value.ValueKind == JsonValueKind.Object, "authored readings retain null and object presence");
        Retains(member["question"]!, question);
        var missingReading = member["question"]!.DeepClone().AsObject();
        missingReading.Remove("true");
        Check(((QuestionDecide)Read(missingReading, Question.Read)).True.State == PresenceState.Missing, "missing authored reading differs from null");
        if (args.Length > 1)
        {
            var native = JsonNode.Parse(File.ReadAllText(args[1]))!["results"]!.AsArray();
            var nativeRelation = native.Single(row => row!["type"]!.GetValue<string>() == "RelateResult")!["result"]!;
            foreach (var source in nativeRelation["answer"]!["questions"]!.AsArray())
            {
                var nativeMember = Read(source!, RelationMember.Read);
                var sources = nativeMember is RelationMemberAnswerId success ? success.QuestionSources : ((RelationMemberFailureId)nativeMember).QuestionSources;
                var nativeObservations = nativeMember is RelationMemberAnswerId answered ? answered.Observations : ((RelationMemberFailureId)nativeMember).Observations;
                var reading = nativeMember is RelationMemberAnswerId answer ? answer.Question : ((RelationMemberFailureId)nativeMember).Question;
                var rule = nativeMember is RelationMemberAnswerId accepted ? accepted.Threshold : ((RelationMemberFailureId)nativeMember).Threshold;
                Check(sources.Single().AnsweredBy == "fixed" && sources.Single().BatchSize.Value == 2, "generated member retains actual native batch source");
                Check(nativeObservations.Count == 1, "generated member retains actual native observation");
                Retains(source!["observations"]![0]!, nativeObservations.Single());
                Check(((QuestionDecide)reading).Text.GetString() == source!["question"]!["text"]!.GetValue<string>(), "generated member retains actual native question");
                Check(rule is ThresholdNumber numeric && numeric.Value == 0.5, "generated member retains actual native threshold");
                Retains(source!, nativeMember);
            }
        }
        NativePresentation(fixture);
        Variants(fixture);
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
        Console.WriteLine("Generated C# results: facts, variants, maps, presence and unknown round trips pass");
    }
}
