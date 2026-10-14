using System;
using System.Linq;
using System.IO;
using System.Collections.Generic;
using System.Text.Json;
using System.Threading;
using ThinkThen;

// Shared fixture framing stays test data; execution uses the shipped typed API.
static class AsyncFixtureCases
{
    static JsonElement Get(JsonElement value, string key) => value.ValueKind == JsonValueKind.Object && value.TryGetProperty(key, out var found) ? found : default;
    static string Text(JsonElement value, string key) => Get(value, key).ValueKind == JsonValueKind.String ? Get(value, key).GetString()! : "";
    static bool Yes(JsonElement value, string key) => Get(value, key).ValueKind == JsonValueKind.True;
    static ThinkThen.Inputs.AuthoredPresence<string> String(JsonElement value, string key) => Get(value, key).ValueKind == JsonValueKind.Undefined ? default(ThinkThen.Inputs.AuthoredPresence<string>) : Text(value, key);
    static ThinkThen.Inputs.AuthoredPresence<ulong> Unsigned(JsonElement value, string key) => Get(value, key).ValueKind == JsonValueKind.Number ? Get(value, key).GetUInt64() : default(ThinkThen.Inputs.AuthoredPresence<ulong>);
    static ThinkThen.Inputs.RequestBatch Batch(JsonElement value) => value.ValueKind == JsonValueKind.Number ? new ThinkThen.Inputs.RequestBatchInteger { Value = value.GetUInt64() } : new ThinkThen.Inputs.RequestBatchString { Value = value.GetString()! };
    static ThinkThen.Inputs.EngineSettings Settings(JsonElement value) => new() {
        Proxy = Get(value,"proxy").ValueKind==JsonValueKind.Undefined ? default(ThinkThen.Inputs.AuthoredPresence<JsonElement>) : Get(value,"proxy").Clone(),
        BaseUrl = String(value,"base_url"), Model = String(value,"model"), Backend = String(value,"backend"), Profile = String(value,"profile"),
        Record = String(value,"record"), Replay = String(value,"replay"), UsdPerMillionInput = String(value,"usd_per_million_input"), UsdPerMillionOutput = String(value,"usd_per_million_output"),
        Cache = Get(value,"cache").ValueKind == JsonValueKind.Undefined ? default : Get(value,"cache").ValueKind == JsonValueKind.String ? (ThinkThen.Inputs.AuthoredPresence<ThinkThen.Inputs.CacheDocument>)new ThinkThen.Inputs.CacheDocumentString { Value = Text(value,"cache") } : new ThinkThen.Inputs.CacheDocumentDisabledCache { Value = new ThinkThen.Inputs.DisabledCache() },
        Batch = Get(value,"batch").ValueKind == JsonValueKind.Undefined ? default(ThinkThen.Inputs.AuthoredPresence<ThinkThen.Inputs.RequestBatch>) : Batch(Get(value,"batch")),
        MaxRequests = Get(value,"max_requests").ValueKind == JsonValueKind.Undefined ? default(ThinkThen.Inputs.AuthoredPresence<ThinkThen.Inputs.EngineSettingsMaxRequests>) : Get(value,"max_requests").ValueKind == JsonValueKind.Null ? (ThinkThen.Inputs.AuthoredPresence<ThinkThen.Inputs.EngineSettingsMaxRequests>)new ThinkThen.Inputs.EngineSettingsMaxRequestsNull() : new ThinkThen.Inputs.EngineSettingsMaxRequestsInteger { Value = Get(value,"max_requests").GetUInt64() },
        MaxRequestsTotal = Get(value,"max_requests_total").ValueKind == JsonValueKind.Undefined ? default(ThinkThen.Inputs.AuthoredPresence<ThinkThen.Inputs.EngineSettingsMaxRequestsTotal>) : Get(value,"max_requests_total").ValueKind == JsonValueKind.Null ? (ThinkThen.Inputs.AuthoredPresence<ThinkThen.Inputs.EngineSettingsMaxRequestsTotal>)new ThinkThen.Inputs.EngineSettingsMaxRequestsTotalNull() : new ThinkThen.Inputs.EngineSettingsMaxRequestsTotalInteger { Value = Get(value,"max_requests_total").GetUInt64() },
        MaxEstimatedInputTokensTotal = Get(value,"max_estimated_input_tokens_total").ValueKind == JsonValueKind.Undefined ? default(ThinkThen.Inputs.AuthoredPresence<ThinkThen.Inputs.EngineSettingsMaxEstimatedInputTokensTotal>) : Get(value,"max_estimated_input_tokens_total").ValueKind == JsonValueKind.Null ? (ThinkThen.Inputs.AuthoredPresence<ThinkThen.Inputs.EngineSettingsMaxEstimatedInputTokensTotal>)new ThinkThen.Inputs.EngineSettingsMaxEstimatedInputTokensTotalNull() : new ThinkThen.Inputs.EngineSettingsMaxEstimatedInputTokensTotalInteger { Value = Get(value,"max_estimated_input_tokens_total").GetUInt64() },
        MaxRetries = Get(value,"max_retries").ValueKind == JsonValueKind.Number ? Get(value,"max_retries").GetUInt32() : default(ThinkThen.Inputs.AuthoredPresence<uint>),
        MaxRequestBytes = Unsigned(value,"max_request_bytes"), Timeout = Unsigned(value,"timeout"),
        Throttle = Get(value,"throttle").ValueKind == JsonValueKind.Number ? Get(value,"throttle").GetInt64() : default(ThinkThen.Inputs.AuthoredPresence<long>),
        RefreshCache = Get(value,"refresh_cache").ValueKind == JsonValueKind.Undefined ? default(ThinkThen.Inputs.AuthoredPresence<bool>) : Yes(value,"refresh_cache")
    };
    static ThinkThen.Inputs.RequestQuestion Question(Engine engine,JsonElement data)
    {
        string reference = Text(data,"reference");
        switch(Text(data,"loader")) {
            case "load_named": case "named": return new ThinkThen.Inputs.RequestQuestionName { Name = reference };
            case "load_reference": case "reference": return new ThinkThen.Inputs.RequestQuestionReference { Reference = reference };
            case "load": case "file": return new ThinkThen.Inputs.RequestQuestionFile { Path = reference };
        }
        if (Text(data,"question_form") == "file") return new ThinkThen.Inputs.RequestQuestionFile { Path = "fixture-question.json" };
        var q = Get(data,"question");
        if (Text(data,"question_form") == "text") return new ThinkThen.Inputs.RequestQuestionDefinition { Value = new ThinkThen.Inputs.RequestDefinitionAuthoredDecide {
            Decide = new ThinkThen.Inputs.AuthoredQuestionTextString { Value = Text(q,"decide") },
            Threshold = Get(q,"threshold").ValueKind == JsonValueKind.Number ? new ThinkThen.Inputs.AuthoredThresholdNumber { Value = Get(q,"threshold").GetDouble() } : default(ThinkThen.Inputs.AuthoredPresence<ThinkThen.Inputs.AuthoredThreshold>)
        }  };
        if (Text(data,"verb") == "find" && Yes(q,"none")) return new ThinkThen.Inputs.RequestQuestionDefinition { Value = new ThinkThen.Inputs.RequestDefinitionAuthoredFind {
            Find = Get(q,"find").ValueKind == JsonValueKind.String ? new ThinkThen.Inputs.AuthoredQuestionTextString { Value = Text(q,"find") } : new ThinkThen.Inputs.AuthoredQuestionTextObject { Value = Get(q,"find").Clone() },
            Name = String(q,"name"), WordingVersion = Get(q,"wording_version").ValueKind == JsonValueKind.Number ? Get(q,"wording_version").GetInt64() : default(ThinkThen.Inputs.AuthoredPresence<long>)
        }  };
        var kind = Text(data,"verb") switch {
            "annotate" => AuthoredQuestionKind.Set, "rank" => Get(q,"questions").ValueKind!=JsonValueKind.Undefined ? AuthoredQuestionKind.RankSet : AuthoredQuestionKind.Rank,
            "choose" => Get(q,"options").ValueKind==JsonValueKind.Undefined ? AuthoredQuestionKind.DynamicChoose : AuthoredQuestionKind.Atomic,
            "find" => AuthoredQuestionKind.Find, "recognize" => AuthoredQuestionKind.Recognize, "relate" => AuthoredQuestionKind.Relate, _ => AuthoredQuestionKind.Atomic
        };
        return FixtureQuestions.Parse(kind,Get(data,"raw").ValueKind==JsonValueKind.String ? Text(data,"raw") : q.GetRawText());
    }
    static ThinkThen.Inputs.RequestInput Source(JsonElement data)
    {
        if (Text(data,"arm") == "case/library-call-result/v1") return new ThinkThen.Inputs.RequestInputText { Text = Get(data,"items")[0].GetString()! };
        var paths = Get(data,"paths");
        if (paths.ValueKind == JsonValueKind.Array && paths.GetArrayLength() > 0) {
            int unit = Get(data,"source_unit").GetInt32();
            ThinkThen.Inputs.SourceUnit reading = unit == 1 ? new ThinkThen.Inputs.SourceUnitLine() : unit == 2 ? new ThinkThen.Inputs.SourceUnitWindow() : new ThinkThen.Inputs.SourceUnitFile();
            return new ThinkThen.Inputs.RequestInputSource { Source = new ThinkThen.Inputs.RequestSource {
                Paths = paths.EnumerateArray().Select(p => p.GetString()!).ToArray(),
                Reading = new ThinkThen.Inputs.RequestReader { Unit = reading, Window = Unsigned(data,"window") },
                Media = unit == 4 || Yes(data,"image_reader") ? (ThinkThen.Inputs.AuthoredPresence<ThinkThen.Inputs.ReaderMedia>)new ThinkThen.Inputs.ReaderMediaImage() : default
            } };
        }
        if (Text(data,"verb")=="annotate" && Yes(data,"text") && Get(data,"items").GetArrayLength()==1 && Get(data,"items")[0].ValueKind==JsonValueKind.String && !Yes(data,"context_present") && !(Get(data,"image_paths").ValueKind==JsonValueKind.Array && Get(data,"image_paths").GetArrayLength()>0)) return new ThinkThen.Inputs.RequestInputText { Text = Get(data,"items")[0].GetString()! };
        var images = new List<ThinkThen.Inputs.RequestImage>();
        var imagePaths = Get(data,"image_paths");
        if (imagePaths.ValueKind == JsonValueKind.Array) foreach(var path in imagePaths.EnumerateArray()) images.Add(new ThinkThen.Inputs.RequestImageBytes {
            Bytes = Convert.ToBase64String(File.ReadAllBytes(path.GetString()!)),
            Media = Text(data,"media") == "image/jpeg" ? new ThinkThen.Inputs.ImageMediaImageJpeg() : new ThinkThen.Inputs.ImageMediaImagePng()
        });
        var records = new List<ThinkThen.Inputs.RequestItem>(); int at = 0;
        foreach (var value in Get(data,"items").EnumerateArray()) {
            var context = Get(data,"contexts").ValueKind == JsonValueKind.Array ? Get(data,"contexts")[at] : Get(data,"context");
            var orders = Get(data,"candidate_orders");
            var item = new ThinkThen.Inputs.RequestItem {
                Original = Yes(data,"image_only") ? default : Yes(data,"caption_files") ? (ThinkThen.Inputs.AuthoredPresence<ThinkThen.Inputs.RequestOriginal>)new ThinkThen.Inputs.RequestOriginalText { Text = File.ReadAllText($"caption-{at}.txt") } : Yes(data,"text") && value.ValueKind == JsonValueKind.String ? new ThinkThen.Inputs.RequestOriginalText { Text = value.GetString()! } : new ThinkThen.Inputs.RequestOriginalJson { Value = value.Clone() },
                Context = !Yes(data,"context_present") && Get(data,"contexts").ValueKind != JsonValueKind.Array ? default(ThinkThen.Inputs.AuthoredPresence<ThinkThen.Inputs.ContextSchema>) : context.ValueKind == JsonValueKind.String ? (ThinkThen.Inputs.AuthoredPresence<ThinkThen.Inputs.ContextSchema>)new ThinkThen.Inputs.ContextSchemaString { Value = context.GetString()! } : new ThinkThen.Inputs.ContextSchemaObject { Value = context.Clone() },
                Options = orders.ValueKind == JsonValueKind.Array ? orders[at].EnumerateArray().Select(v => new ThinkThen.Inputs.OptionSchema { Name = v.GetString()! }).ToArray() : default(ThinkThen.Inputs.AuthoredPresence<IReadOnlyList<ThinkThen.Inputs.OptionSchema>>),
                Images = images.Count > 0 ? images.AsReadOnly() : default(ThinkThen.Inputs.AuthoredPresence<IReadOnlyList<ThinkThen.Inputs.RequestImage>>)
            };
            at++; records.Add(item);
        }
        return new ThinkThen.Inputs.RequestInputRecords { Items = records };
    }
    static async IAsyncEnumerable<ThinkThen.Inputs.RequestSessionDescriptor> Feed(JsonElement data, ThinkThen.Inputs.RequestInput? input)
    {
        if (Yes(data,"owned_jsonl")) {
            string path = Get(data,"paths")[0].GetString()!; ulong line = 0;
            foreach (string raw in File.ReadLines(path, new System.Text.UTF8Encoding(false,true))) {
                line++; using var record = JsonDocument.Parse(raw);
                yield return new ThinkThen.Inputs.RequestSessionDescriptor {
                    Item = new ThinkThen.Inputs.RequestItem { Original = new ThinkThen.Inputs.RequestOriginalJson { Value = record.RootElement.Clone() } },
                    Location = new ThinkThen.Inputs.SessionSourceLocation { File = path, FirstLine = line, LastLine = line }
                };
                await System.Threading.Tasks.Task.Yield();
            }
        } else if (input is ThinkThen.Inputs.RequestInputRecords records) {
            foreach (var item in records.Items) { yield return new ThinkThen.Inputs.RequestSessionDescriptor { Item = item }; await System.Threading.Tasks.Task.Yield(); }
        } else throw new InvalidOperationException("fixture feed requires records");
    }
    public static string Run(string raw) => RunAsync(raw).GetAwaiter().GetResult();
    static async System.Threading.Tasks.Task<string> RunAsync(string raw)
    {
        using var document = JsonDocument.Parse(raw); var data = document.RootElement; Thread? signal = null;
        try {
            using var settings = JsonDocument.Parse(Text(data,"engine_settings")); using var engine = Engine.Open(Settings(settings.RootElement));
            var question = Question(engine,data); var input = Yes(data,"owned_jsonl") ? null : Source(data); using var cancel = new CancellationTokenSource();
            string injection = Text(Get(data,"operation"),"injection"); if (injection == "cancel_token") cancel.Cancel();
            if (Yes(data,"held_cancel")) { signal = new Thread(() => { if (Console.Read() != '!') throw new Exception("missing cancellation signal"); cancel.Cancel(); Console.WriteLine("cancel-fired"); Console.Out.Flush(); }); signal.Start(); }
            bool nullContext = Yes(data,"context_present") && Get(data,"context").ValueKind==JsonValueKind.Null;
            if (nullContext) input = new ThinkThen.Inputs.RequestInputJson { Value = JsonSerializer.SerializeToElement(new Dictionary<string,object?> { ["item"] = Get(data,"items")[0].Clone(), ["context"] = null }) };
            var context = Get(data,"shared_context"); var options = new ThinkThen.Inputs.RequestOptions {
                Field = nullContext ? new[] { "/item" } : default(ThinkThen.Inputs.AuthoredPresence<IReadOnlyList<string>>), ContextField = nullContext ? "/context" : default(ThinkThen.Inputs.AuthoredPresence<string>),
                Attempts = true, None = Text(data,"verb") == "find" ? Yes(Get(data,"question"),"none") : default(ThinkThen.Inputs.AuthoredPresence<bool>), Context = context.ValueKind == JsonValueKind.String ? context.GetString()! : default(ThinkThen.Inputs.AuthoredPresence<string>),
                DeadlineMs = injection == "expired_deadline" ? 0L : default(ThinkThen.Inputs.AuthoredPresence<long>)
            };
            try {
                OwnedCall call = (Yes(data,"incremental") ? Text(data,"verb") switch {
                    "decide" => await engine.DecideAsync(question,Feed(data,input),options,cancel.Token), "choose" => await engine.ChooseAsync(question,Feed(data,input),options,cancel.Token),
                    "tag" => await engine.TagAsync(question,Feed(data,input),options,cancel.Token), "score" => await engine.ScoreAsync(question,Feed(data,input),options,cancel.Token),
                    "filter" => await engine.FilterAsync(question,Feed(data,input),options,cancel.Token), "rank" => await engine.RankAsync(question,Feed(data,input),options,cancel.Token),
                    "find" => await engine.FindAsync(question,Feed(data,input),options,cancel.Token), "annotate" => await engine.AnnotateAsync(question,Feed(data,input),options,cancel.Token),
                    "recognize" => await engine.RecognizeAsync(question,Feed(data,input),options,cancel.Token), "relate" => await engine.RelateAsync(question,Feed(data,input),options,cancel.Token),
                    _ => throw new InvalidOperationException("unknown fixture function")
                } : Text(data,"verb") switch {
                    "decide" => await engine.DecideAsync(question,(dynamic)input!,options,cancel.Token), "choose" => await engine.ChooseAsync(question,(dynamic)input!,options,cancel.Token),
                    "tag" => await engine.TagAsync(question,(dynamic)input!,options,cancel.Token), "score" => await engine.ScoreAsync(question,(dynamic)input!,options,cancel.Token),
                    "filter" => await engine.FilterAsync(question,(dynamic)input!,options,cancel.Token), "rank" => await engine.RankAsync(question,(dynamic)input!,options,cancel.Token),
                    "find" => await engine.FindAsync(question,(dynamic)input!,options,cancel.Token), "annotate" => await engine.AnnotateAsync(question,(dynamic)input!,options,cancel.Token),
                    "recognize" => await engine.RecognizeAsync(question,(dynamic)input!,options,cancel.Token), "relate" => await engine.RelateAsync(question,(dynamic)input!,options,cancel.Token),
                    _ => throw new InvalidOperationException("unknown fixture function")
                });
                if (Text(data,"verb") == "find" && call.Packets.OfType<ThinkThen.Results.SessionPacketFindAggregate>().Single().Value.Index.State != ThinkThen.Results.PresenceState.Value && call.GetType().GetProperty("Probability")!.GetValue(call) is not null) throw new Exception("unresolved find probability");
                var output = Normalize(call, Text(data,"verb"),Yes(data,"incremental"),Get(data,"image_paths").ValueKind==JsonValueKind.Array && Get(data,"image_paths").GetArrayLength()>0 || Get(data,"source_unit").ValueKind==JsonValueKind.Number && Get(data,"source_unit").GetInt32()==4);
                if (Text(data,"arm") == "case/library-call-result/v1") {
                    output["call_result"] = Text(data,"verb") == "decide"
                        ? ResultShape((CallResult<JsonElement,double?,IReadOnlyList<ThinkThen.Results.AtomicDecideValue>>)call)
                        : ResultShape((CallResult<string?,double?,IReadOnlyList<ThinkThen.Results.AtomicNullableString>>)call);
                }
                return JsonSerializer.Serialize(output);
            } catch(SessionFailure failure) { return JsonSerializer.Serialize(Normalize(failure.Call,Text(data,"verb"),Yes(data,"incremental"),Get(data,"image_paths").ValueKind==JsonValueKind.Array && Get(data,"image_paths").GetArrayLength()>0 || Get(data,"source_unit").ValueKind==JsonValueKind.Number && Get(data,"source_unit").GetInt32()==4)); }
        } catch(Failure failure) { return JsonSerializer.Serialize(new { code = failure.Code, message = failure.Message }); }
        catch(JsonException failure) { return JsonSerializer.Serialize(new { code = 1, message = failure.Message }); }
        catch(OperationCanceledException) { return JsonSerializer.Serialize(new { code = 5, message = "Operation cancelled." }); }
        finally { signal?.Join(); }
    }
    static object ResultShape<T,P,R>(CallResult<T,P,IReadOnlyList<R>> result) where R : ThinkThen.Results.ResultObject => new {
        value = result.Value, probability = result.Probability,
        results = result.Results.Select(record => new { answer_id = Property(record,"AnswerId") }).ToArray(),
        facts = new { requests_sent = result.Facts.RequestsSent, records = result.Facts.Records }
    };
    static object? Optional(object? value)
    {
        if (value is null) return null;
        var type = value.GetType(); var state = type.GetProperty("State");
        return state is null ? value : (ThinkThen.Results.PresenceState)state.GetValue(value)! == ThinkThen.Results.PresenceState.Value ? type.GetProperty("Value")!.GetValue(value) : null;
    }
    static object? Property(object value, string name) => Optional(value.GetType().GetProperty(name)?.GetValue(value));
    static Dictionary<string,object?> Author(object question) {
        var result = new Dictionary<string,object?>(); foreach (var pair in new[] { ("Name","name"),("WordingVersion","wording_version") }) if(Property(question,pair.Item1) is {} value) result[pair.Item2] = value; return result;
    }
    static Dictionary<string,object?> Normalize(OwnedCall call, string verb, bool incremental, bool imageInput)
    {
        var rows = new List<Dictionary<string,object?>>();
        foreach(var packet in call.Packets) switch(packet) {
            case ThinkThen.Results.SessionPacketDecideRow p: rows.Add(Row(p.Value,Optional(p.Value.Value),call)); break;
            case ThinkThen.Results.SessionPacketChooseRow p: rows.Add(Row(p.Value,Optional(p.Value.Value),call)); break;
            case ThinkThen.Results.SessionPacketTagRow p: rows.Add(Row(p.Value,p.Value.Value,call)); break;
            case ThinkThen.Results.SessionPacketScoreRow p: rows.Add(Row(p.Value,p.Value.Value,call)); break;
            case ThinkThen.Results.SessionPacketFilterRow p: rows.Add(Row(p.Value,p.Value.Value,call)); break;
            case ThinkThen.Results.SessionPacketAnnotateRow p: rows.Add(Row(p.Value,p.Value.Value.ToDictionary(v=>v.Key,v=>Annotated(v.Value)),call)); break;
            case ThinkThen.Results.SessionPacketRankAggregate p: rows.AddRange(p.Value.Select(v=>Row(v,v.Value,call))); break;
            case ThinkThen.Results.SessionPacketFindAggregate p: rows.Add(Row(p.Value,Optional(p.Value.Value),call)); break;
            case ThinkThen.Results.SessionPacketRecognizeAggregate p: rows.AddRange(p.Value.Select(v=>Row(v,Recognized(v.Value),call))); break;
            case ThinkThen.Results.SessionPacketRelateAggregate p: rows.Add(Row(p.Value,p.Value.Value.Select(Edge).ToArray(),call)); break;
        }
        if (verb == "filter") {
            var questions = call.Packets.OfType<ThinkThen.Results.SessionPacketObservation>().Select(p=>p.Value).OfType<ThinkThen.Results.SessionObservationQuestion>().ToArray();
            var observedRows = call.Packets.OfType<ThinkThen.Results.SessionPacketObservation>().Select(p=>p.Value).OfType<ThinkThen.Results.SessionObservationRow>().ToArray();
            var all = new List<Dictionary<string,object?>>();
            foreach (var observed in observedRows) {
                var detail = questions.Last(q=>q.Index == observed.Index).Detail;
                var selected = rows.FirstOrDefault(r=>Equals(r["answer_id"],Optional(detail.AnswerId)));
                if (selected is not null) { all.Add(selected); continue; }
                var judgment = (ThinkThen.Results.SessionObservedRowJudgment)observed.Value;
                var decision = (ThinkThen.Results.SessionJudgmentDecision)judgment.Value;
                var source = detail.QuestionSources.FirstOrDefault();
                var row = new Dictionary<string,object?> { ["value"]=Optional(decision.Value),["index"]=observed.Index,["answer_id"]=Optional(detail.AnswerId),
                    ["origin"]=source is null ? null : source.Origin.Value switch {"live"=>1,"cache"=>2,"replay"=>3,_=>throw new InvalidOperationException("unknown native origin")},
                    ["answered_by"]=source?.AnsweredBy,["observations"]=detail.Observations.Count,["sources"]=detail.QuestionSources.Count,["input"]=Optional(detail.Input),
                    ["observation_ids"]=detail.Observations.OfType<ThinkThen.Results.ObservationObservationId>().Select(v=>v.ObservationId).ToArray(),["detail_inputs"]=Array.Empty<object>(),["answer_kind"]="YesNo"
                };
                Location(row,Optional(detail.InputSource));
                if(detail.Probabilities.State==ThinkThen.Results.PresenceState.Value && detail.Probabilities.Value is ThinkThen.Results.SessionProbabilitiesYesNo odds)row["probability"]=odds.Value;
                foreach(var author in Author(detail.Question))row[author.Key]=author.Value;
                all.Add(row);
            }
            rows = all;
        }
        var output = new Dictionary<string,object?> { ["code"] = 0, ["schema"] = "thinkthen.result/2", ["observations"] = call.Packets.OfType<ThinkThen.Results.SessionPacketObservation>().Count(), ["rows"] = rows };
        if(call.Terminal.Facts.State == ThinkThen.Results.PresenceState.Value) {
            var facts=call.Terminal.Facts.Value; output["call_id"]=facts.CallId; output["requests_sent"]=facts.RequestsSent; output["cache_answers"]=facts.CacheAnswers; output["records"]=facts.Records;
            if(facts.InputTokens.State == ThinkThen.Results.PresenceState.Value) output["input_tokens"]=facts.InputTokens.Value;
            if(facts.OutputTokens.State == ThinkThen.Results.PresenceState.Value) output["output_tokens"]=facts.OutputTokens.Value;
        }
        if(call.Terminal.Failure.State == ThinkThen.Results.PresenceState.Value) {
            var failure=call.Terminal.Failure.Value.Error; output["code"]=failure.Kind.Value switch {"usage"=>1,"backend"=>2,"deadline"=>3,"local"=>4,"cancelled"=>5,_=>6}; output["message"]=failure.Message;
            if(failure.Stopped.At.State == ThinkThen.Results.PresenceState.Value) output["stopped_at"]=failure.Stopped.At.Value;
        }
        if(imageInput) foreach(var row in rows) if(row["input"] is JsonElement original && original.ValueKind==JsonValueKind.Object && original.TryGetProperty("text",out var caption)) row["input"]=caption.Clone();
        if(incremental) output["completed"]=rows;
        return output;
    }
    static object? Annotated(ThinkThen.Results.AnnotatedField value) => value switch {
        ThinkThen.Results.AnnotatedFieldBoolean v => v.Value, ThinkThen.Results.AnnotatedFieldNull => null,
        ThinkThen.Results.AnnotatedFieldString v => v.Value, ThinkThen.Results.AnnotatedFieldArray v => v.Value,
        ThinkThen.Results.AnnotatedFieldNumber v => v.Value, ThinkThen.Results.AnnotatedFieldObject v => new {failed=new {kind=v.Value.FailedValue.Kind,cause=v.Value.FailedValue.Cause.Value}},
        _=>throw new InvalidOperationException("unknown generated member")
    };
    static object Entity(ThinkThen.Results.Entity entity) => new {text=entity.Text,start=entity.Start,end=entity.End,length=entity.Length,kind=entity.Kind,strength=entity.Strength};
    static object Recognized(ThinkThen.Results.Recognize value) {
        var entityValue=(ThinkThen.Results.RecognizeEntities)value; var result=new Dictionary<string,object?> { ["entities"] = entityValue.Entities.Select(Entity).ToArray() };
        if(entityValue.Relations.State == ThinkThen.Results.PresenceState.Value) result["relations"] = entityValue.Relations.Value.Select(e=>Edge(e.Relation,Entity(e.Source),Entity(e.Target),e.Probability,e.Either.State==ThinkThen.Results.PresenceState.Value&&e.Either.Value)).ToArray(); return result;
    }
    static object Edge(string relation,object source,object target,double probability,bool either) {var result=new Dictionary<string,object?>{{"relation",relation},{"source",source},{"target",target},{"probability",probability}};if(either)result["either"]=true;return result;}
    static object Endpoint(ThinkThen.Results.RelationEndpoint endpoint) => endpoint switch {
        ThinkThen.Results.RelationEndpointRelatedEntity e => new {name=e.Name,kind=e.Kind},
        ThinkThen.Results.RelationEndpointSource e => new {name=e.Name,kind=e.Kind},
        _=>throw new InvalidOperationException("unknown generated endpoint")
    };
    static object Edge(ThinkThen.Results.RelatedEntityEdge e) => Edge(e.Relation,Endpoint(e.Source),Endpoint(e.Target),e.Probability,e.Either.State==ThinkThen.Results.PresenceState.Value&&e.Either.Value);
    static void Location(Dictionary<string,object?> result,object? source) {if(source is null)return;foreach(var pair in new[]{("File","file"),("FirstLine","first_line"),("LastLine","last_line")})if(Property(source,pair.Item1)is{} value)result[pair.Item2]=value;}
    static Dictionary<string,object?> Row(dynamic value,object? reading,OwnedCall call)
    {
        ThinkThen.Results.Meta meta=value.Meta;
        object raw=value;
        var result = new Dictionary<string,object?> { ["value"]=reading, ["index"]=Property(raw,"Index"), ["answer_id"]=value.AnswerId,
            ["origin"]=meta.Origin.State==ThinkThen.Results.PresenceState.Value ? meta.Origin.Value.Value switch {"live"=>1,"cache"=>2,"replay"=>3,_=>throw new InvalidOperationException("unknown native origin")} : null,
            ["answered_by"]=Optional(meta.AnsweredBy),["observations"]=meta.Observations.Count,["sources"]=meta.QuestionSources.Count,["input"]=Property(raw,"Input"),
            ["observation_ids"]=meta.Observations.OfType<ThinkThen.Results.ObservationObservationId>().Select(v=>v.ObservationId).ToArray()
        };
        if (result["index"] is null && raw is not ThinkThen.Results.Find) {
            var occurrence = call.Packets.OfType<ThinkThen.Results.SessionPacketObservation>().Select(p=>p.Value).OfType<ThinkThen.Results.SessionObservationQuestion>().FirstOrDefault(q=>Equals(Optional(q.Detail.AnswerId),result["answer_id"]));
            result["index"] = occurrence?.Index ?? 0UL;
        }
        Location(result,Property(raw,"Source"));Location(result,raw);
        if(Property(raw,"Question") is {} question) foreach(var author in Author(question))result[author.Key]=author.Value;
        if(Property(raw,"Images") is IReadOnlyList<ThinkThen.Results.Image> images) {result["image_properties"]=images.Select(i=>new ulong[]{i.Media.Value=="image/jpeg"?1UL:2UL,i.Width,i.Height}).ToArray();result["images"]=images.Select(i=>Convert.ToHexString(Convert.FromBase64String(i.Base64)).ToLowerInvariant()).ToArray();}
        if(Property(raw,"Answer") is ThinkThen.Results.Answer answer) {result["answer_kind"]=answer switch {ThinkThen.Results.AnswerYesNo=>"YesNo",ThinkThen.Results.AnswerChoice=>"Choice",ThinkThen.Results.AnswerScore=>"Score",_=>"Tag"}; if(answer is ThinkThen.Results.AnswerYesNo yesNo) result["probability"]=yesNo.Probability;else result["probabilities"]=Property(answer,"Probabilities");}
        if(raw is ThinkThen.Results.Find found) {var chosen=found.Candidates.State==ThinkThen.Results.PresenceState.Value?found.Candidates.Value.FirstOrDefault(c=>c.Index.State==found.Index.State&&(c.Index.State!=ThinkThen.Results.PresenceState.Value||c.Index.Value==found.Index.Value)):null;result["input"]=chosen is null?null:Optional(chosen.Input);result["probability"]=chosen?.Probability;result["answer_kind"]="find";result["probabilities"]=found.Answer.Probabilities;result["detail_inputs"]=found.Candidates.State==ThinkThen.Results.PresenceState.Value?found.Candidates.Value.Where(c=>c.Index.State==ThinkThen.Results.PresenceState.Value).Select(c=>{var input=new Dictionary<string,object?>{{"input",Optional(c.Input)}};Location(input,Optional(c.Source));return input;}).ToArray():Array.Empty<object>();}
        else result["detail_inputs"]=Array.Empty<object>();
        if(raw is ThinkThen.Results.Annotation annotation) result["member_authors"]=annotation.Answers.Values.Select(member=>Author(Property(member,"Question")!)).ToArray();
        if(raw is ThinkThen.Results.AtomicNonZeroUsize rank && rank.Members.State==ThinkThen.Results.PresenceState.Value) {RankFacts(result,meta);result["question_name"]=Optional(rank.QuestionName);result["members"]=rank.Members.Value.Select(member=>{var child=Row(member.Result,member.Result.Value,call);child["name"]=member.Name;child["author"]=Property(member.Result.Question,"Name");RankFacts(child,member.Result.Meta);return child;}).ToArray();}
        if(raw is ThinkThen.Results.Relation related) {
            var sources=related.Value.SelectMany(e=>new[]{e.Source,e.Target}).OfType<ThinkThen.Results.RelationEndpointSource>().GroupBy(e=>e.Ordinal).Select(g=>g.First());
            result["detail_inputs"]=sources.Select(s=>{var original=new Dictionary<string,object?>{{"input",s.Record}};Location(original,s);return original;}).ToArray();
        }
        return result;
    }
    static void RankFacts(Dictionary<string,object?> output,ThinkThen.Results.Meta meta) {
        output["model"]=meta.Model;output["context_digest"]=Optional(meta.ContextSha256);var usage=new Dictionary<string,object?>();
        if(meta.Usage.State==ThinkThen.Results.PresenceState.Value){if(meta.Usage.Value.InputTokens.State==ThinkThen.Results.PresenceState.Value)usage["input_tokens"]=meta.Usage.Value.InputTokens.Value;if(meta.Usage.Value.OutputTokens.State==ThinkThen.Results.PresenceState.Value)usage["output_tokens"]=meta.Usage.Value.OutputTokens.Value;}
        output["usage"]=usage;output["source_batch_sizes"]=meta.QuestionSources.Select(s=>Optional(s.BatchSize)).ToArray();
    }
}
