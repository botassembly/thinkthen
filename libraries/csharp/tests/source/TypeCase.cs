using System.Text.Json;
using ThinkThen;
using ThinkThen.Inputs;

if(args.Length>=1 && args[0]=="complete"){Console.WriteLine(AsyncFixtureCases.Run(args.Length==2?args[1]:Console.ReadLine()!));return;}
if(args.Length==1 && args[0]=="native"){NativeChecks.Run();Console.WriteLine("{\"native\":\"pass\"}");return;}
if(args.Length==1 && args[0]=="carriers"){CarrierChecks.Run();Console.WriteLine("{\"carriers\":\"pass\"}");return;}
using var engine=Engine.Open(new EngineSettings());
try {
 if(args[0]=="limits") {
  using var capped=Engine.Open(new EngineSettings { MaxRequestsTotal=new EngineSettingsMaxRequestsTotalInteger { Value=0 }, Cache=new CacheDocumentDisabledCache {Value=new DisabledCache()} });
  try {await capped.DecideAsync(new RequestQuestionText {Text="Is it?"},new RequestInputText {Text="capped"});throw new Exception("cap admitted");}
  catch(Failure e) when(e.Kind==FailureKind.Usage && e.Message.Contains("process send budget")) {}
  catch(SessionFailure e) when(e.Failure.Error.Kind.Value=="usage" && e.Failure.Error.Message.Contains("process send budget")) {}
  foreach(string limitedVerb in new[]{"decide","recognize","relate"}) {
   var q=limitedVerb=="decide" ? (RequestQuestion)new RequestQuestionText {Text="Is it?"} : FixtureQuestions.Parse(limitedVerb=="recognize"?AuthoredQuestionKind.Recognize:AuthoredQuestionKind.Relate,limitedVerb=="recognize"?"{\"version\":1,\"recognize\":{\"kinds\":{\"person\":\"A person.\"}}}":"{\"version\":1,\"relate\":{\"relations\":[{\"name\":\"linked\",\"source\":\"alert\",\"target\":\"alert\"}]}}");
   RequestInput input=limitedVerb=="relate"?new RequestInputRecords {Items=new[]{"A","B"}.Select(n=>new RequestItem {Original=new RequestOriginalJson {Value=JsonSerializer.SerializeToElement(new {name=n,kind="alert"})}}).ToArray()}:new RequestInputText {Text="zero-"+limitedVerb};
   try {_ = (limitedVerb switch {"decide"=>await engine.DecideAsync(q,(dynamic)input,new RequestOptions {DeadlineMs=0}),"recognize"=>await engine.RecognizeAsync(q,(dynamic)input,new RequestOptions {DeadlineMs=0}),_=>await engine.RelateAsync(q,(dynamic)input,new RequestOptions {DeadlineMs=0})});throw new Exception("zero admitted");}
   catch(Failure e) when(e.Kind==FailureKind.Deadline) {}
   catch(SessionFailure e) when(e.Failure.Error.Kind.Value=="deadline") {}
  }
  Console.WriteLine("{\"limits\":\"pass\"}");return;
 }
 if(args[0]=="plan") {
  using var doc=JsonDocument.Parse(args[1]);var root=doc.RootElement;
  using var planned=Engine.Open(root.GetProperty("settings").TryGetProperty("batch",out var batch)?new EngineSettings {Batch=new RequestBatchInteger {Value=batch.GetUInt64()}}:new EngineSettings());
  var plan=planned.Plan(new Request {Schema=new RequestVersionThinkthenRequest1(),Call=new RequestCall {Function=new RequestFunctionDecide(),Question=new RequestQuestionText {Text=root.GetProperty("question").GetString()!},Input=new RequestInputRecords {Items=root.GetProperty("input").EnumerateArray().Select(v=>new RequestItem {Original=new RequestOriginalText {Text=v.GetString()!}}).ToArray()}}});
  Console.WriteLine(JsonSerializer.Serialize(new { records=plan.Records, requests=plan.Requests, estimated_bytes=plan.EstimatedBytes, upper_bound=plan.UpperBound, token_estimate_method=plan.TokenEstimateMethod, largest_request_bytes=plan.LargestRequestBytes, largest_request_estimated_input_tokens=plan.LargestRequestEstimatedInputTokens, estimated_input_tokens=new {lower=plan.EstimatedInputTokens.Lower,upper=plan.EstimatedInputTokens.Upper}, first_body_utf8=plan.FirstBodyUtf8.State==ThinkThen.Results.PresenceState.Value?plan.FirstBodyUtf8.Value:null }));return;
 }
 // Shared J1 fixtures are test data. Authored definitions go through the native parser;
 // the public execution request uses generated types in AsyncFixtureCases.
 using var request=JsonDocument.Parse(args[^1]);var r=request.RootElement;
 string verb=new[]{"decide","choose","tag","score","filter","rank","find","annotate","recognize","relate"}.Single(v=>r.TryGetProperty(v,out _));
 var question=new Dictionary<string,JsonElement>();
 if(verb=="annotate") foreach(var p in r.GetProperty(verb).EnumerateObject())question[p.Name]=p.Value.Clone();
 else foreach(var p in r.EnumerateObject())if(p.Name is not "evidence" and not "records" and not "units" and not "details" and not "call" and not "none")question[p.Name=="filter"||p.Name=="rank"?"decide":p.Name]=p.Value.Clone();
 var items=r.TryGetProperty("records",out var records)?records:r.TryGetProperty("units",out var units)?units:JsonSerializer.SerializeToElement(new[]{r.GetProperty("evidence").Clone()});
 if(r.TryGetProperty("none",out var none))question["none"]=none.Clone();
 var framing=new {verb,question,items,text=items.EnumerateArray().All(v=>v.ValueKind==JsonValueKind.String),engine_settings=r.TryGetProperty("call",out var call)&&call.TryGetProperty("batch",out var size)?JsonSerializer.Serialize(new {batch=size.GetUInt64()}):"{}"};
 Console.WriteLine(AsyncFixtureCases.Run(JsonSerializer.Serialize(framing)));
} catch(Failure error) {Console.WriteLine(JsonSerializer.Serialize(new {failed=new {kind=error.Kind.ToString().ToLowerInvariant(),code=error.Code}}));}
