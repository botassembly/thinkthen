using ThinkThen;
using ThinkThen.Inputs;
using R = ThinkThen.Results;
using System.Text.Json;
static class NativeChecks
{
    static void Check(bool yes, string label) { if (!yes) throw new Exception(label); }
    static InputAuthoredQuestionText Text(string value) => new InputAuthoredQuestionTextAlternative0 { Value = value };
    static IReadOnlyList<InputAuthoredName> Names => new[] { new InputAuthoredName { Value = "first" }, new InputAuthoredName { Value = "second" } };
    static InputRequestQuestion Define(InputRequestDefinition value) => new InputRequestQuestionDefinition { Value = value };
    static InputRequestQuestion Decide => Define(new InputRequestDefinitionAlternative0 { Value = new InputAuthoredDecide { Decide = Text("Need attention?") } });
    static InputRequestQuestion Choose => Define(new InputRequestDefinitionAlternative1 { Value = new InputAuthoredChoose { Choose = Text("Which?"), Options = new InputAuthoredOptionsAlternative0 { Value = Names } } });
    static InputRequestItem Item(string value) => new() { Original = new InputRequestOriginalText { Text = value } };
    static InputRequestInput Records => new InputRequestInputRecords { Items = new[] { Item("Maria Chen"), Item("Alex Lee") } };
    static void Call(OwnedCall call)
    {
        Check(call.Terminal.Failure.State == R.PresenceState.Missing && call.Terminal.Facts.State == R.PresenceState.Value, "actual terminal");
        var facts = call.Terminal.Facts.Value;
        Check(facts.CallId.Length == 64 && facts.RequestsSent > 0, "actual facts");
        Check(facts.Attempts.State == R.PresenceState.Value && facts.Attempts.Value.Count > 0, "attempts");
        Check(call.Packets.OfType<R.SessionPacketObservation>().Any(), "actual observations");
        foreach (var attempt in facts.Attempts.Value) Check(attempt.SdkRequestId.Length == 64, "SDK request identity");
    }
    static T Aggregate<T>(OwnedCall call) where T : R.SessionPacket { Call(call); return call.Packets.OfType<T>().Single(); }
    static IReadOnlyList<R.AtomicDecideValue> ReadDecide(OwnedCall call) { Call(call); return call.Packets.OfType<R.SessionPacketDecideRow>().Select(p => p.Value).ToArray(); }
    static IReadOnlyList<R.AtomicNullableString> ReadChoose(OwnedCall call) { Call(call); return call.Packets.OfType<R.SessionPacketChooseRow>().Select(p => p.Value).ToArray(); }
    static IReadOnlyList<R.AtomicArrayOfString> ReadTag(OwnedCall call) { Call(call); return call.Packets.OfType<R.SessionPacketTagRow>().Select(p => p.Value).ToArray(); }
    static IReadOnlyList<R.AtomicDouble> ReadScore(OwnedCall call) { Call(call); return call.Packets.OfType<R.SessionPacketScoreRow>().Select(p => p.Value).ToArray(); }
    static IReadOnlyList<R.Annotation> ReadAnnotate(OwnedCall call) { Call(call); return call.Packets.OfType<R.SessionPacketAnnotateRow>().Select(p => p.Value).ToArray(); }
    public static void Run() => RunAsync().GetAwaiter().GetResult();
    static async Task RunAsync()
    {
        using var settings = JsonDocument.Parse(Environment.GetEnvironmentVariable("TT_NATIVE_SETTINGS")!);
        using var engine = Engine.Open(new InputEngineSettings {
            BaseUrl = settings.RootElement.GetProperty("base_url").GetString()!, Model = settings.RootElement.GetProperty("model").GetString()!,
            Cache = new InputCacheDocumentAlternative1 { Value = new InputDisabledCache() },
            Batch = new InputRequestBatchAlternative1 { Value = "max" }, Throttle = 1, MaxRetries = 0
        });
        var controls = new InputRequestOptions { Attempts = true, Details = true };
        var completeControls = new InputRequestOptions { Attempts = true };
        foreach (bool files in new[] { false, true })
        {
            InputRequestInput source = files ? new InputRequestInputSource { Source = new InputRequestSource {
                Paths = new[] { Environment.GetEnvironmentVariable("TT_NATIVE_FILE")! }, Reading = new InputRequestReader { Unit = new InputSourceUnitAlternative0() }
            } } : Records;
            var decide = ReadDecide(await engine.DecideAsync(Decide, source, controls));
            Check(decide.Count == 2 && decide[0].Value.GetBoolean() && decide[0].Answer is R.AnswerYesNo { Probability: .9 }, "decision");
            Check(decide[0].Schema.Value == "thinkthen.result/2" && decide[0].AnswerId.Length == 64, "row schema and identity");
            if (files) Check(decide[0].Source.Value.FirstLine.Value == 1 && decide[1].Source.Value.LastLine.Value == 2, "physical lines");
            var choose = ReadChoose(await engine.ChooseAsync(Choose, source, controls));
            Check(choose.Count == 2 && choose[0].Value.Value == "first" && choose[0].Answer is R.AnswerChoice answer && answer.Probabilities.Count == 2, "choice distribution");
            var tagQuestion = Define(new InputRequestDefinitionAlternative2 { Value = new InputAuthoredTag { Tag = Text("Which?"), Labels = new InputAuthoredLabelsAlternative0 { Value = Names } } });
            var tag = ReadTag(await engine.TagAsync(tagQuestion, source, controls));
            Check(tag.Count == 2 && tag[0].Value.Count == 2, "tag labels");
            var scoreQuestion = Define(new InputRequestDefinitionAlternative3 { Value = new InputAuthoredScore { Score = Text("How much?"), Levels = new InputAuthoredLevelsAlternative0 { Value = Names } } });
            var score = ReadScore(await engine.ScoreAsync(scoreQuestion, source, controls));
            Check(score.Count == 2 && score[0].Value == .1, "score");
            var filter = await engine.FilterAsync(new InputRequestQuestionText { Text = "Need attention?" }, source, new InputRequestOptions { Attempts = true, Threshold = new InputRequestThresholdAlternative0 { Value = .95 } });
            Call(filter);
            Check(!filter.Packets.OfType<R.SessionPacketFilterRow>().Any(), "rejected filter selection");
            var filterEvents = filter.Packets.OfType<R.SessionPacketObservation>().Select(p => p.Value).OfType<R.SessionObservationRow>().ToArray();
            Check(filterEvents.Length == 2 && filterEvents[0].Value is R.SessionObservedRowJudgment { Value: R.SessionJudgmentDecision judgment } && judgment.Value.State == R.PresenceState.Value && !judgment.Value.Value, "rejected filter observation");
            var rank = Aggregate<R.SessionPacketRankAggregate>(await engine.RankAsync(Decide, source, completeControls)).Value;
            Check(rank.Count == 2 && rank[0].Value == 1, "rank ordinal");
            var findQuestion = Define(new InputRequestDefinitionAlternative5 { Value = new InputAuthoredFind { Find = Text("Which?") } });
            var find = Aggregate<R.SessionPacketFindAggregate>(await engine.FindAsync(findQuestion, source, completeControls)).Value;
            Check(find.Candidates.State == R.PresenceState.Value && find.Candidates.Value.Count == 2, "find candidates");
            var annotateQuestion = Define(new InputRequestDefinitionAlternative7 { Questions = new Dictionary<string, InputRequestDefinitionAlternative7QuestionsEntry> {
                ["check"] = new InputRequestDefinitionAlternative7QuestionsEntryAlternative0 { Decide = Text("Need attention?") },
                ["team"] = new InputRequestDefinitionAlternative7QuestionsEntryAlternative1 { Choose = Text("Which?"), Options = new InputAuthoredOptionsAlternative0 { Value = Names } }
            } });
            var annotate = ReadAnnotate(await engine.AnnotateAsync(annotateQuestion, source, completeControls));
            Check(annotate.Count == 2 && annotate[0].Answers.Count == 2 && annotate[0].Answers["check"] is R.AnnotationMemberAnswerId { Answer: R.AnswerYesNo { Probability: .9 } }, "annotation members");
            var recognizeQuestion = Define(new InputRequestDefinitionAlternative6 { Recognize = new InputRequestDefinitionAlternative6Recognize() });
            var recognize = Aggregate<R.SessionPacketRecognizeAggregate>(await engine.RecognizeAsync(recognizeQuestion, source, completeControls)).Value;
            Check(recognize.Count == 2 && recognize[0].Value is R.RecognizeFieldsEntities entities && entities.Entities.Count > 0, "recognized spans");
            Check((recognize[0].Source.State == R.PresenceState.Value) == files, "recognized location");
            InputRequestInput relationInput = source;
            if (!files)
            {
                using var first = JsonDocument.Parse("{\"name\":\"Maria Chen\",\"kind\":\"person\"}");
                using var second = JsonDocument.Parse("{\"name\":\"Alex Lee\",\"kind\":\"person\"}");
                relationInput = new InputRequestInputRecords { Items = new[] {
                    new InputRequestItem { Original = new InputRequestOriginalJson { Value = first.RootElement.Clone() } },
                    new InputRequestItem { Original = new InputRequestOriginalJson { Value = second.RootElement.Clone() } }
                } };
            }
            var relateQuestion = Define(new InputRequestDefinitionAlternative4 { Value = new InputAuthoredRelate { Relate = new InputAuthoredRelateRelate {
                Relations = new[] { new InputAuthoredRelation { Name = new InputAuthoredName { Value = "supports" } } }
            } } });
            var relate = Aggregate<R.SessionPacketRelateAggregate>(await engine.RelateAsync(relateQuestion, relationInput, completeControls)).Value;
            Check(relate.Value.Count == 2 && relate.Answer.Questions.Count == 2, "relation endpoints");
        }
    }
}
