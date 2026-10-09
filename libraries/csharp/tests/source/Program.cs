using System.Text.Json;
using ThinkThen;
using ThinkThen.Inputs;
using R=ThinkThen.Results;
static class Program
{
 static void Check(bool condition,string label){if(!condition)throw new Exception(label);}
 static InputRequestInputRecords Records(params string[] texts)=>new(){Items=texts.Select(t=>new InputRequestItem{Original=new InputRequestOriginalText{Text=t}}).ToArray()};
 static InputRequestQuestionText Question=>new(){Text="Is it?"};
 static Task<OwnedCall> Decide(Engine e,string text,InputRequestOptions? options=null,CancellationToken cancellation=default)=>e.DecideAsync(Question,Records(text),options,cancellation);
 static R.SessionPacketDecideRow[] Rows(OwnedCall c)=>c.Packets.OfType<R.SessionPacketDecideRow>().ToArray();
 static R.Facts Facts(OwnedCall c)=>c.Terminal.Facts.Value;
 static void Answer(OwnedCall c,bool? expected,double p){var row=Rows(c).Single().Value;Check(row.Value.ValueKind==(expected is null?JsonValueKind.Null:expected.Value?JsonValueKind.True:JsonValueKind.False)&&row.Answer is R.AnswerYesNo {Probability:var actual}&&actual==p,"decision/probability");}
 static int Code(Exception e)=>e switch{Failure f=>f.Code,OperationCanceledException=>5,SessionFailure f=>f.Failure.Error.Kind.Value switch{"usage"=>1,"backend"=>2,"deadline"=>3,"local"=>4,"cancelled"=>5,_=>6},_=>0};
 static string Message(Exception e)=>e is SessionFailure f?f.Failure.Error.Message:e.Message;
 static async Task<Exception> Fails(Func<Task> call,int code){try{await call();}catch(Exception e)when(Code(e)==code){Check(Message(e).Length>0,"error message");return e;}throw new Exception("expected error "+code);}
 static void Marker(string state){var path=Path.Combine(Environment.GetEnvironmentVariable("TT_BARRIER_DIR")!,"arrived-"+state);Check(SpinWait.SpinUntil(()=>File.Exists(path),TimeSpan.FromSeconds(30)),"arrival "+state);}
 static void Release(string state)=>File.WriteAllText(Path.Combine(Environment.GetEnvironmentVariable("TT_BARRIER_DIR")!,"release-"+state),"");
 static async Task Held(Engine e,string state,int expected,bool many=false){
  using var cancel=new CancellationTokenSource();var call=Fails(async()=>{await e.DecideAsync(Question,many?Records(Enumerable.Range(1,6).Select(i=>"hold-bulk-"+i).ToArray()):Records(state),expected==3?new InputRequestOptions{DeadlineMs=1000}:null,cancel.Token);},expected);
  Marker(state);if(expected==5){cancel.Cancel();cancel.Cancel();}
  bool returnedHeld=expected==3&&await Task.WhenAny(call,Task.Delay(30000))==call;
  if(many)for(int i=1;i<=6;i++)Release("hold-bulk-"+i);else Release(state);
  await call.WaitAsync(TimeSpan.FromSeconds(60));Check(expected!=3||returnedHeld,"deadline while held");
 }
 static async Task Direct(){using var e=Engine.Open(new InputEngineSettings());Answer(await Decide(e,"direct"),true,.9);using var spent=new CancellationTokenSource();spent.Cancel();await Fails(async()=>{await Decide(e,"no-arrival-spent-token",cancellation:spent.Token);},5);Console.WriteLine("TYPED_DIRECT_PASS");}
 static async Task Matrix(){
  using var e=Engine.Open(new InputEngineSettings());
  foreach(var sample in new[]{("yes",(bool?)true,.9),("no",(bool?)false,.1),("unsure",(bool?)null,.5),("café",(bool?)true,.9),("a\0b",(bool?)true,.9)}) {
   var q=sample.Item1=="unsure"?(InputRequestQuestion)e.ParseQuestion(AuthoredQuestionKind.Atomic,"{\"decide\":\"Is it?\",\"threshold\":\"0.4:0.8\"}"):Question;
   Answer(await e.DecideAsync(q,Records(sample.Item1)),sample.Item2,sample.Item3);
  }
  var noUsage=await Decide(e,"no-usage");Answer(noUsage,true,.9);Check(Facts(noUsage).Records==1&&Facts(noUsage).RequestsSent==1&&Facts(noUsage).Model.Value=="jev-1.13.0"&&Facts(noUsage).InputTokens.State==R.PresenceState.Missing&&Facts(noUsage).OutputTokens.State==R.PresenceState.Missing,"no usage native facts");
  var first=await e.DecideAsync(Question,Records("first","second","third"));Check(Rows(first).Select(r=>((R.AnswerYesNo)r.Value.Answer).Probability).SequenceEqual(new[]{.9,.1,.6})&&Facts(first).Records==3&&Facts(first).Model.Value=="jev-1.13.0","packed answers/facts");
  var cached=await e.DecideAsync(Question,Records("first","second","third"));Check(Facts(cached).CacheAnswers==3&&Facts(cached).RequestsSent==0&&Rows(cached).Select(r=>((R.AnswerYesNo)r.Value.Answer).Probability).SequenceEqual(new[]{.9,.1,.6}),"cache facts/order");
  Check(Rows(await e.DecideAsync(Question,Records("first","second","first","second"))).Select(r=>((R.AnswerYesNo)r.Value.Answer).Probability).SequenceEqual(new[]{.9,.1,.9,.1}),"duplicate order");
  var empty=await e.DecideAsync(Question,Records());Check(Rows(empty).Length==0&&Facts(empty).Records==0&&Facts(empty).RequestsSent==0&&Facts(empty).Model.State==R.PresenceState.Missing,"empty facts");
  try{await Decide(e,"\ud800");throw new Exception("invalid UTF8 accepted");}catch(System.Text.EncoderFallbackException){}
  var detailed=await Decide(e,"json-decide");Answer(detailed,true,.9);Check(Rows(detailed)[0].Value.Schema.Value=="thinkthen.result/2","typed complete result schema");
  InputRequestQuestion Auth(AuthoredQuestionKind k,string value)=>e.ParseQuestion(k,value);
  var choose=await e.ChooseAsync(Auth(AuthoredQuestionKind.Atomic,"{\"choose\":\"Which team?\",\"options\":[\"first\",\"second\"]}"),Records("choose"));Check(choose.Packets.OfType<R.SessionPacketChooseRow>().Single().Value.Value.Value=="first","choice");
  var tag=await e.TagAsync(Auth(AuthoredQuestionKind.Atomic,"{\"tag\":\"Which labels?\",\"labels\":[\"first\",\"second\"]}"),Records("tag"));Check(tag.Packets.OfType<R.SessionPacketTagRow>().Single().Value.Value.SequenceEqual(new[]{"first","second"}),"labels");
  var score=await e.ScoreAsync(Auth(AuthoredQuestionKind.Atomic,"{\"score\":\"What level?\",\"levels\":[\"Low.\",\"High.\"]}"),Records("score"));Check(score.Packets.OfType<R.SessionPacketScoreRow>().Single().Value.Value==.1,"score");
  var filter=await e.FilterAsync(Question,Records("filter-one","filter-two"));Check(filter.Packets.OfType<R.SessionPacketFilterRow>().Count()==2,"filter");
  var rank=await e.RankAsync(Question,Records("rank-one","rank-two"));var ranked=rank.Packets.OfType<R.SessionPacketRankAggregate>().Single().Value;Check(ranked.Count==2&&ranked[0].Index.Value==0&&ranked[0].Input.Value.GetString()=="rank-one"&&ranked[0].Answer is R.AnswerYesNo {Probability:.9},"rank");
  var find=await e.FindAsync(Auth(AuthoredQuestionKind.Find,"{\"find\":\"Which line?\"}"),Records("find-one","find-two"));var found=find.Packets.OfType<R.SessionPacketFindAggregate>().Single().Value;Check(found.Index.Value==0&&found.Value.Value.GetString()=="find-one"&&found.Candidates.Value[0].Probability==.9,"find");
  var annotation=await e.AnnotateAsync(Auth(AuthoredQuestionKind.Set,"{\"version\":1,\"questions\":{\"check\":{\"decide\":\"Is it?\"}}}"),Records("annotate-one"));Check(annotation.Packets.OfType<R.SessionPacketAnnotateRow>().Single().Value.Value["check"] is R.AnnotatedFieldBoolean{Value:true},"annotation");
  var recognition=Auth(AuthoredQuestionKind.Recognize,"{\"version\":1,\"recognize\":{\"kinds\":{\"person\":\"A person's name.\"}}}");
  foreach(string name in new[]{"Maria Chen","John Smith"}){var recognized=await e.RecognizeAsync(recognition,Records(name));Check(((R.RecognizeFieldsEntities)recognized.Packets.OfType<R.SessionPacketRecognizeAggregate>().Single().Value.Single().Value).Entities.Count==1&&Facts(recognized).Records==1,"recognition");}
  var relation=Auth(AuthoredQuestionKind.Relate,"{\"version\":1,\"relate\":{\"relations\":[{\"name\":\"caused_by\",\"source\":\"alert\",\"target\":\"alert\"}]}}");
  foreach(var names in new[]{new[]{"First","Second"},new[]{"Third","Fourth"}}){var related=await e.RelateAsync(relation,new InputRequestInputRecords{Items=names.Select(n=>new InputRequestItem{Original=new InputRequestOriginalJson{Value=JsonSerializer.SerializeToElement(new{name=n,kind="alert"})}}).ToArray()});Check(related.Packets.OfType<R.SessionPacketRelateAggregate>().Single().Value.Value.Count==2&&Facts(related).Records==1,"relation");}
  Exception saved;try{e.ParseQuestion(AuthoredQuestionKind.Atomic,"{invalid");throw new Exception("invalid authored data accepted");}catch(Failure f){saved=f;}
  var backend=await Fails(async()=>{await Decide(e,"status-401");},2);Check(backend is SessionFailure b&&Facts(b.Call).RequestsSent==1,"failure actual started facts");string copied=Message(backend);
  await Fails(async()=>{await Decide(e,"zero-deadline",new InputRequestOptions{DeadlineMs=0});},3);
  using(var canceled=new CancellationTokenSource()){canceled.Cancel();await Fails(async()=>{await Decide(e,"no-arrival-pre-cancelled",cancellation:canceled.Token);},5);}
  var failures=await Task.WhenAll(new[]{"failure-one","failure-two"}.Select(state=>Fails(async()=>{await Decide(e,state);},2)));Check(Message(failures[0]).Contains("401")&&Message(failures[1]).Contains("403"),"independent owned errors");Answer(await Decide(e,"success"),true,.9);
  var one=Decide(e,"hold-facts-one");var two=Decide(e,"hold-facts-two");try{Marker("hold-facts-one");Marker("hold-facts-two");}finally{Release("hold-facts-one");Release("hold-facts-two");}var owned=await Task.WhenAll(one,two);Check(owned.All(c=>Facts(c).RequestsSent==1&&Facts(c).Seconds>0),"overlapping native facts");
  await Held(e,"hold-deadline",3);await Held(e,"hold-bulk-1",5,true);await Held(e,"hold-scalar",5);Answer(await Decide(e,"recovery-scalar"),true,.9);
  string copiedError=saved.Message;e.Dispose();Check(saved.Message==copiedError&&Message(backend)==copied&&owned.All(c=>Facts(c).Model.Value=="jev-1.13.0"),"owned facts/errors after close");try{await Decide(e,"closed");throw new Exception("closed admitted");}catch(ObjectDisposedException){}
  Console.WriteLine("MATRIX_PASS");
 }
 public static async Task<int> Main(string[] args){try{if(args[0]=="direct")await Direct();else if(args[0]=="matrix")await Matrix();else if(args[0]=="named"){using var e=Engine.Open(new InputEngineSettings{Backend="local",Cache=new InputCacheDocumentAlternative1{Value=new InputDisabledCache()}});var call=await Decide(e,"named-csharp");Answer(call,true,.9);Check(Facts(call).RequestsSent==1,"named count");Console.WriteLine("CSHARP_NAMED_BACKEND_PASS");}else throw new ArgumentException("mode");return 0;}catch(Exception e){Console.Error.WriteLine(e);return 1;}}
}
