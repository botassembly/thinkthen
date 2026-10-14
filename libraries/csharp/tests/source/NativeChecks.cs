using ThinkThen;
using ThinkThen.Inputs;
using R = ThinkThen.Results;
using System.Text.Json;
static class NativeChecks
{
    static void Check(bool yes, string label) { if (!yes) throw new Exception(label); }
    static AuthoredQuestionText Text(string value) => new AuthoredQuestionTextString { Value = value };
    static IReadOnlyList<AuthoredName> Names => new[] { new AuthoredName { Value = "first" }, new AuthoredName { Value = "second" } };
    static RequestQuestion Define(RequestDefinition value) => new RequestQuestionDefinition { Value = value };
    static RequestQuestion Decide => Define(new RequestDefinitionAuthoredDecide { Decide = Text("Need attention?") } );
    static RequestQuestion Choose => Define(new RequestDefinitionAuthoredChoose { Choose = Text("Which?"), Options = new AuthoredOptionsList { Value = Names } } );
    static RequestItem Item(string value) => new() { Original = new RequestOriginalText { Text = value } };
    static RequestInput Records => new RequestInputRecords { Items = new[] { Item("Maria Chen"), Item("Alex Lee") } };
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
        using var engine = Engine.Open(new EngineSettings {
            BaseUrl = settings.RootElement.GetProperty("base_url").GetString()!, Model = settings.RootElement.GetProperty("model").GetString()!,
            Cache = new CacheDocumentDisabledCache { Value = new DisabledCache() },
            Batch = new RequestBatchString { Value = "max" }, Throttle = 1, MaxRetries = 0
        });
        var unknownChoose = await engine.ChooseAsync(Choose, new RequestInputRecords { Items = new[] { Item("text"), Item("other") } }, new RequestOptions { Threshold = new RequestThresholdNumber { Value = .95 } });
        Check(unknownChoose.Value.All(value => value is null) && unknownChoose.Probability.All(chance => chance is null), "unresolved choice probability");
        var findQuestionForNull = Define(new RequestDefinitionAuthoredFind { Find = Text("Which?") });
        var absentFind = await engine.FindAsync(findQuestionForNull, new RequestInputRecords { Items = new[] { Item("text"), Item("other") } }, new RequestOptions { None = true, Threshold = new RequestThresholdNumber { Value = .95 } });
        Check(absentFind.Value.ValueKind == JsonValueKind.Null && absentFind.Probability is null, "unresolved find probability");
        var nullFind = await engine.FindAsync(findQuestionForNull, new RequestInputRecords { Items = new[] { new RequestItem { Original = new RequestOriginalJson { Value = JsonSerializer.SerializeToElement((string?)null) } }, Item("other") } }, new RequestOptions { None = true });
        Check(nullFind.Value.ValueKind == JsonValueKind.Null && nullFind.Probability == .9 && nullFind.Results.Single().Index.State == R.PresenceState.Value && nullFind.Results.Single().Index.Value == 0, "selected null original probability");
        var controls = new RequestOptions { Attempts = true };
        var completeControls = new RequestOptions { Attempts = true };
        foreach (bool files in new[] { false, true })
        {
            RequestInput source = files ? new RequestInputSource { Source = new RequestSource {
                Paths = new[] { Environment.GetEnvironmentVariable("TT_NATIVE_FILE")! }, Reading = new RequestReader { Unit = new SourceUnitLine() }
            } } : Records;
            var decide = ReadDecide((OwnedCall)await engine.DecideAsync(Decide, (dynamic)source, controls));
            Check(decide.Count == 2 && decide[0].Value.GetBoolean() && decide[0].Answer is R.AnswerYesNo { Probability: .9 }, "decision");
            Check(decide[0].Schema.Value == "thinkthen.result/2" && decide[0].AnswerId.Length == 64, "row schema and identity");
            if (files) Check(decide[0].Source.Value.FirstLine.Value == 1 && decide[1].Source.Value.LastLine.Value == 2, "physical lines");
            var choose = ReadChoose((OwnedCall)await engine.ChooseAsync(Choose, (dynamic)source, controls));
            Check(choose.Count == 2 && choose[0].Value.Value == "first" && choose[0].Answer is R.AnswerChoice answer && answer.Probabilities.Count == 2, "choice distribution");
            var tagQuestion = Define(new RequestDefinitionAuthoredTag { Tag = Text("Which?"), Labels = new AuthoredLabelsList { Value = Names } } );
            var tag = ReadTag((OwnedCall)await engine.TagAsync(tagQuestion, (dynamic)source, controls));
            Check(tag.Count == 2 && tag[0].Value.Count == 2, "tag labels");
            var scoreQuestion = Define(new RequestDefinitionAuthoredScore { Score = Text("How much?"), Levels = new AuthoredLevelsList { Value = Names } } );
            var score = ReadScore((OwnedCall)await engine.ScoreAsync(scoreQuestion, (dynamic)source, controls));
            Check(score.Count == 2 && score[0].Value == .1, "score");
            OwnedCall filter = await engine.FilterAsync(new RequestQuestionText { Text = "Need attention?" }, (dynamic)source, new RequestOptions { Attempts = true, Threshold = new RequestThresholdNumber { Value = .95 } });
            Call(filter);
            Check(!filter.Packets.OfType<R.SessionPacketFilterRow>().Any(), "rejected filter selection");
            var filterEvents = filter.Packets.OfType<R.SessionPacketObservation>().Select(p => p.Value).OfType<R.SessionObservationRow>().ToArray();
            Check(filterEvents.Length == 2 && filterEvents[0].Value is R.SessionObservedRowJudgment { Value: R.SessionJudgmentDecision judgment } && judgment.Value.State == R.PresenceState.Value && !judgment.Value.Value, "rejected filter observation");
            var rank = Aggregate<R.SessionPacketRankAggregate>((OwnedCall)await engine.RankAsync(Decide, (dynamic)source, completeControls)).Value;
            Check(rank.Count == 2 && rank[0].Value == 1, "rank ordinal");
            var findQuestion = Define(new RequestDefinitionAuthoredFind { Find = Text("Which?") } );
            var find = Aggregate<R.SessionPacketFindAggregate>((OwnedCall)await engine.FindAsync(findQuestion, (dynamic)source, completeControls)).Value;
            Check(find.Candidates.State == R.PresenceState.Value && find.Candidates.Value.Count == 2, "find candidates");
            var annotateQuestion = Define(new RequestDefinitionAnnotateDefinition { Questions = new KeyValuePair<string, AnnotateQuestion>[] {
                new("check", new AnnotateQuestionDecide { Decide = Text("Need attention?") }),
                new("team", new AnnotateQuestionChoose { Choose = Text("Which?"), Options = new AuthoredOptionsList { Value = Names } })
            } });
            var annotate = ReadAnnotate((OwnedCall)await engine.AnnotateAsync(annotateQuestion, (dynamic)source, completeControls));
            Check(annotate.Count == 2 && annotate[0].Answers.Count == 2 && annotate[0].Answers["check"] is R.AnnotationMemberAnswerId { Answer: R.AnswerYesNo { Probability: .9 } }, "annotation members");
            var recognizeQuestion = Define(new RequestDefinitionRecognizeDefinition { Recognize = new RecognizeQuestion() });
            OwnedCall recognized = await engine.RecognizeAsync(recognizeQuestion, (dynamic)source, completeControls);
            Call(recognized);
            var recognize = recognized.Packets.OfType<R.SessionPacketRecognizeAggregate>().SelectMany(p => p.Value).ToArray();
            Check(recognize.Length == 2 && recognize.Select(r => r.Index.Value).SequenceEqual(new ulong[] { 0, 1 }), "recognized ordered occurrences");
            Check(recognize.Select(r => r.Input.Value.GetString()).SequenceEqual(new[] { "Maria Chen", "Alex Lee" }), "recognized originals");
            Check(recognize.All(r => r.Value is R.RecognizeEntities entities && entities.Entities.Count > 0), "recognized spans");
            Check(recognize.All(r => (r.Source.State == R.PresenceState.Value) == files), "recognized location");
            if (files) Check(recognize[0].Source.Value.FirstLine.Value == 1 && recognize[1].Source.Value.LastLine.Value == 2, "recognized physical lines");
            RequestInput relationInput = source;
            if (!files)
            {
                using var first = JsonDocument.Parse("{\"name\":\"Maria Chen\",\"kind\":\"person\"}");
                using var second = JsonDocument.Parse("{\"name\":\"Alex Lee\",\"kind\":\"person\"}");
                relationInput = new RequestInputRecords { Items = new[] {
                    new RequestItem { Original = new RequestOriginalJson { Value = first.RootElement.Clone() } },
                    new RequestItem { Original = new RequestOriginalJson { Value = second.RootElement.Clone() } }
                } };
            }
            var relateQuestion = Define(new RequestDefinitionAuthoredRelate { Relate = new RelateQuestion {
                Relations = new[] { new AuthoredRelation { Name = new AuthoredName { Value = "supports" } } }
            } } );
            var relate = Aggregate<R.SessionPacketRelateAggregate>((OwnedCall)await engine.RelateAsync(relateQuestion, (dynamic)relationInput, completeControls)).Value;
            Check(relate.Value.Count == 2 && relate.Answer.Questions.Count == 2, "relation endpoints");
        }
    }
}
