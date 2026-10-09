using System.Text.Json;
using ThinkThen;
using ThinkThen.Inputs;
using Failure = ThinkThen.Failure;
using FailureKind = ThinkThen.FailureKind;

if(args.Length>=1 && args[0]=="complete"){Console.WriteLine(NativeCases.Run(args.Length==2?args[1]:Console.ReadLine()!));return;}
if(args.Length==1 && args[0]=="native"){NativeChecks.Run();Console.WriteLine("{\"native\":\"pass\"}");return;}
if(args.Length==1 && args[0]=="carriers"){CarrierChecks.Run();Console.WriteLine("{\"carriers\":\"pass\"}");return;}
using var engine=Engine.Open(new InputEngineSettings());
try {
 if(args[0]=="limits") {
  using var capped=Engine.Open(new InputEngineSettings { MaxRequestsTotal=new InputEngineSettingsMaxRequestsTotalAlternative0 { Value=0 }, Cache=new InputCacheDocumentAlternative1 {Value=new InputDisabledCache()} });
  try {await capped.DecideAsync(new InputRequestQuestionText {Text="Is it?"},new InputRequestInputText {Text="capped"});throw new Exception("cap admitted");}
  catch(Failure e) when(e.Kind==FailureKind.Usage && e.Message.Contains("process send budget")) {}
  catch(SessionFailure e) when(e.Failure.Error.Kind.Value=="usage" && e.Failure.Error.Message.Contains("process send budget")) {}
  foreach(string limitedVerb in new[]{"decide","recognize","relate"}) {
   var q=limitedVerb=="decide" ? (InputRequestQuestion)new InputRequestQuestionText {Text="Is it?"} : engine.ParseQuestion(limitedVerb=="recognize"?AuthoredQuestionKind.Recognize:AuthoredQuestionKind.Relate,limitedVerb=="recognize"?"{\"version\":1,\"recognize\":{\"kinds\":{\"person\":\"A person.\"}}}":"{\"version\":1,\"relate\":{\"relations\":[{\"name\":\"linked\",\"source\":\"alert\",\"target\":\"alert\"}]}}");
   InputRequestInput input=limitedVerb=="relate"?new InputRequestInputRecords {Items=new[]{"A","B"}.Select(n=>new InputRequestItem {Original=new InputRequestOriginalJson {Value=JsonSerializer.SerializeToElement(new {name=n,kind="alert"})}}).ToArray()}:new InputRequestInputText {Text="zero-"+limitedVerb};
   try {await (limitedVerb switch {"decide"=>engine.DecideAsync(q,input,new InputRequestOptions {DeadlineMs=0}),"recognize"=>engine.RecognizeAsync(q,input,new InputRequestOptions {DeadlineMs=0}),_=>engine.RelateAsync(q,input,new InputRequestOptions {DeadlineMs=0})});throw new Exception("zero admitted");}
   catch(Failure e) when(e.Kind==FailureKind.Deadline) {}
   catch(SessionFailure e) when(e.Failure.Error.Kind.Value=="deadline") {}
  }
  Console.WriteLine("{\"limits\":\"pass\"}");return;
 }
 if(args[0]=="plan") {
  using var doc=JsonDocument.Parse(args[1]);var root=doc.RootElement;
  using var planned=Engine.Open(root.GetProperty("settings").TryGetProperty("batch",out var batch)?new InputEngineSettings {Batch=new InputRequestBatchAlternative0 {Value=batch.GetUInt64()}}:new InputEngineSettings());
  var plan=planned.Plan(new InputRequest {Schema=new InputRequestVersionAlternative0(),Call=new InputRequestCallDecide {Question=new InputRequestQuestionText {Text=root.GetProperty("question").GetString()!},Input=new InputRequestInputRecords {Items=root.GetProperty("input").EnumerateArray().Select(v=>new InputRequestItem {Original=new InputRequestOriginalText {Text=v.GetString()!}}).ToArray()}}});
  Console.WriteLine(plan.ToJsonString());return;
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
 // Native find uses generated options.None; preserve the fixture's explicit value.
 var framed=JsonSerializer.SerializeToElement(framing);
 var data=framed.EnumerateObject().ToDictionary(p=>p.Name,p=>(object?)p.Value.Clone());
 Console.WriteLine(AsyncFixtureCases.Run(JsonSerializer.Serialize(data)));
} catch(Failure error) {Console.WriteLine(JsonSerializer.Serialize(new {failed=new {kind=error.Kind.ToString().ToLowerInvariant(),code=error.Code}}));}
