using System.Text.Json;
using ThinkThen;
using ThinkThen.Inputs;
using R=ThinkThen.Results;
static class Program
{
 static void Check(bool condition,string label){if(!condition)throw new Exception(label);}
 static RequestInputRecords Records(params string[] texts)=>new(){Items=texts.Select(t=>new RequestItem{Original=new RequestOriginalText{Text=t}}).ToArray()};
 static RequestQuestionText Question=>new(){Text="Is it?"};
 static async Task<OwnedCall> Decide(Engine e,string text,RequestOptions? options=null,CancellationToken cancellation=default)=>await e.DecideAsync(Question,Records(text),options,cancellation);
 static R.SessionPacketDecideRow[] Rows(OwnedCall c)=>c.Packets.OfType<R.SessionPacketDecideRow>().ToArray();
 static R.Facts Facts(OwnedCall c)=>c.Terminal.Facts.Value;
 static void Answer(OwnedCall c,bool? expected,double p){var row=Rows(c).Single().Value;Check(row.Value.ValueKind==(expected is null?JsonValueKind.Null:expected.Value?JsonValueKind.True:JsonValueKind.False)&&row.Answer is R.AnswerYesNo {Probability:var actual}&&actual==p,"decision/probability");}
 static int Code(Exception e)=>e switch{Failure f=>f.Code,OperationCanceledException=>5,SessionFailure f=>f.Failure.Error.Kind.Value switch{"usage"=>1,"backend"=>2,"deadline"=>3,"local"=>4,"cancelled"=>5,_=>6},_=>0};
 static string Message(Exception e)=>e is SessionFailure f?f.Failure.Error.Message:e.Message;
 static async Task<Exception> Fails(Func<Task> call,int code){try{await call();}catch(Exception e)when(Code(e)==code){Check(Message(e).Length>0,"error message");return e;}throw new Exception("expected error "+code);}
 static void Marker(string state){var path=Path.Combine(Environment.GetEnvironmentVariable("TT_BARRIER_DIR")!,"arrived-"+state);Check(SpinWait.SpinUntil(()=>File.Exists(path),TimeSpan.FromSeconds(30)),"arrival "+state);}
 static void Release(string state)=>File.WriteAllText(Path.Combine(Environment.GetEnvironmentVariable("TT_BARRIER_DIR")!,"release-"+state),"");
 static async Task Held(Engine e,string state,int expected,bool many=false){
  using var cancel=new CancellationTokenSource();var call=Fails(async()=>{await e.DecideAsync(Question,many?Records(Enumerable.Range(1,6).Select(i=>"hold-bulk-"+i).ToArray()):Records(state),expected==3?new RequestOptions{DeadlineMs=1000}:null,cancel.Token);},expected);
  Marker(state);if(expected==5){cancel.Cancel();cancel.Cancel();}
  bool returnedHeld=expected==3&&await Task.WhenAny(call,Task.Delay(30000))==call;
  if(many)for(int i=1;i<=6;i++)Release("hold-bulk-"+i);else Release(state);
  await call.WaitAsync(TimeSpan.FromSeconds(60));Check(expected!=3||returnedHeld,"deadline while held");
 }
 static async Task Matrix(){
  using var e=Engine.Open(new EngineSettings());
  foreach(var sample in new[]{("yes",(bool?)true,.9),("no",(bool?)false,.1),("unsure",(bool?)null,.5),("café",(bool?)true,.9),("a\0b",(bool?)true,.9)}) {
   var q=sample.Item1=="unsure"?(RequestQuestion)FixtureQuestions.Parse(AuthoredQuestionKind.Atomic,"{\"decide\":\"Is it?\",\"threshold\":\"0.4:0.8\"}"):Question;
   var result=await e.DecideAsync(q,Records(sample.Item1));
   Answer(result,sample.Item2,sample.Item3);
   Check(result.Value.Count==1&&result.Value[0].GetRawText()==JsonSerializer.Serialize(sample.Item2)&&result.Probability.Count==1&&result.Probability[0]==sample.Item3&&result.Results.Count==1,"one-item list result");
  }
  foreach(var authored in new[]{"\"approved\"","{\"route\":\"billing\",\"nested\":[true,null,{\"count\":2}]}","[\"yes\",{\"nested\":[false,null,2]}]"}) {
   var question=FixtureQuestions.Parse(AuthoredQuestionKind.Atomic,"{\"decide\":\"Is it?\",\"true\":"+authored+"}");
   var scalar=await e.DecideAsync(question,new RequestInputText{Text="authored-value"});
   Check(scalar.Value.GetRawText()==authored&&scalar.Probability==.9&&scalar.Results.Count==1&&scalar.Facts.RequestsSent==1,"authored scalar value");
   var list=await e.DecideAsync(question,Records("authored-value"));
   Check(list.Value.Count==1&&list.Value[0].GetRawText()==authored&&list.Probability.Single()==.9&&list.Results.Count==1&&list.Facts.RequestsSent==0,"authored one-item list value");
  }
  var noUsage=await Decide(e,"no-usage");Answer(noUsage,true,.9);Check(Facts(noUsage).Records==1&&Facts(noUsage).RequestsSent==1&&Facts(noUsage).Model.Value=="jev-1.13.0"&&Facts(noUsage).InputTokens.State==R.PresenceState.Missing&&Facts(noUsage).OutputTokens.State==R.PresenceState.Missing,"no usage native facts");
  var first=await e.DecideAsync(Question,Records("first","second","third"));Check(Rows(first).Select(r=>((R.AnswerYesNo)r.Value.Answer).Probability).SequenceEqual(new[]{.9,.1,.6})&&Facts(first).Records==3&&Facts(first).Model.Value=="jev-1.13.0","packed answers/facts");Check(first.Value.Select(value=>value.GetBoolean()).SequenceEqual(new[]{true,false,true})&&first.Probability.SequenceEqual(new double?[]{.9,.1,.6})&&first.Results.Count==3&&first.Facts.Records==3,"plain ordered result");
  var cached=await e.DecideAsync(Question,Records("first","second","third"));Check(Facts(cached).CacheAnswers==3&&Facts(cached).RequestsSent==0&&Rows(cached).Select(r=>((R.AnswerYesNo)r.Value.Answer).Probability).SequenceEqual(new[]{.9,.1,.6}),"cache facts/order");
  Check(Rows(await e.DecideAsync(Question,Records("first","second","first","second"))).Select(r=>((R.AnswerYesNo)r.Value.Answer).Probability).SequenceEqual(new[]{.9,.1,.9,.1}),"duplicate order");
  var empty=await e.DecideAsync(Question,Records());Check(Rows(empty).Length==0&&Facts(empty).Records==0&&Facts(empty).RequestsSent==0&&Facts(empty).Model.State==R.PresenceState.Missing,"empty facts");Check(empty.Value.Count==0&&empty.Probability.Count==0&&empty.Results.Count==0&&empty.Facts.Records==0,"empty result lists");
  try{await Decide(e,"\ud800");throw new Exception("invalid UTF8 accepted");}catch(System.Text.EncoderFallbackException){}
  var detailed=await Decide(e,"json-decide");Answer(detailed,true,.9);Check(Rows(detailed)[0].Value.Schema.Value=="thinkthen.result/2","typed complete result schema");
  RequestQuestion Auth(AuthoredQuestionKind k,string value)=>FixtureQuestions.Parse(k,value);
  var choose=await e.ChooseAsync(Auth(AuthoredQuestionKind.Atomic,"{\"choose\":\"Which team?\",\"options\":[\"first\",\"second\"]}"),Records("choose"));Check(choose.Packets.OfType<R.SessionPacketChooseRow>().Single().Value.Value.Value=="first","choice");Check(choose.Value.Single()=="first"&&choose.Probability.Single()==.9&&choose.Results.Count==1,"plain choice result");
  var tag=await e.TagAsync(Auth(AuthoredQuestionKind.Atomic,"{\"tag\":\"Which labels?\",\"labels\":[\"first\",\"second\"]}"),Records("tag"));Check(tag.Packets.OfType<R.SessionPacketTagRow>().Single().Value.Value.SequenceEqual(new[]{"first","second"}),"labels");Check(tag.Value.Single().SequenceEqual(new[]{"first","second"})&&tag.Probability is null,"plain labels result");
  var score=await e.ScoreAsync(Auth(AuthoredQuestionKind.Atomic,"{\"score\":\"What level?\",\"levels\":[\"Low.\",\"High.\"]}"),Records("score"));Check(score.Packets.OfType<R.SessionPacketScoreRow>().Single().Value.Value==.1,"score");Check(score.Value.Single()==.1&&score.Probability is null,"plain score result");
  var filter=await e.FilterAsync(Question,Records("filter-one","filter-two"));Check(filter.Packets.OfType<R.SessionPacketFilterRow>().Count()==2,"filter");Check(filter.Value.Select(v=>v.GetString()).SequenceEqual(new[]{"filter-one","filter-two"})&&filter.Probability.SequenceEqual(new double?[]{.9,.9}),"plain filter result");
  var rank=await e.RankAsync(Question,Records("rank-one","rank-two"));var ranked=rank.Packets.OfType<R.SessionPacketRankAggregate>().Single().Value;Check(ranked.Count==2&&ranked[0].Index.Value==0&&ranked[0].Input.Value.GetString()=="rank-one"&&ranked[0].Answer is R.AnswerYesNo {Probability:.9},"rank");Check(rank.Value.SequenceEqual(new ulong[]{0,1})&&rank.Probability.SequenceEqual(new double?[]{.9,.9}),"plain rank result");
  var find=await e.FindAsync(Auth(AuthoredQuestionKind.Find,"{\"find\":\"Which line?\"}"),Records("find-one","find-two"));var found=find.Packets.OfType<R.SessionPacketFindAggregate>().Single().Value;Check(found.Index.Value==0&&found.Value.Value.GetString()=="find-one"&&found.Candidates.Value[0].Probability==.9,"find");Check(find.Value.GetString()=="find-one"&&find.Probability==.9&&find.Results.Count==1,"plain find result");
  var annotation=await e.AnnotateAsync(Auth(AuthoredQuestionKind.Set,"{\"version\":1,\"questions\":{\"check\":{\"decide\":\"Is it?\"}}}"),Records("annotate-one"));Check(annotation.Packets.OfType<R.SessionPacketAnnotateRow>().Single().Value.Value["check"] is R.AnnotatedFieldBoolean{Value:true},"annotation");Check(annotation.Value.Single()["check"] is R.AnnotatedFieldBoolean{Value:true}&&annotation.Probability is null,"plain annotation result");
  var recognition=Auth(AuthoredQuestionKind.Recognize,"{\"version\":1,\"recognize\":{\"kinds\":{\"person\":\"A person's name.\"}}}");
  foreach(string name in new[]{"Maria Chen","John Smith"}){var recognized=await e.RecognizeAsync(recognition,Records(name));Check(((R.RecognizeEntities)recognized.Packets.OfType<R.SessionPacketRecognizeAggregate>().SelectMany(p=>p.Value).Single().Value).Entities.Count==1&&Facts(recognized).Records==1,"recognition");Check(((R.RecognizeEntities)recognized.Value.Single()).Entities.Count==recognized.Probability.Single().Count&&recognized.Probability.Single().All(p=>p>=0&&p<=1),"plain recognition result");}
  var relation=Auth(AuthoredQuestionKind.Relate,"{\"version\":1,\"relate\":{\"relations\":[{\"name\":\"caused_by\",\"source\":\"alert\",\"target\":\"alert\"}]}}");
  foreach(var names in new[]{new[]{"First","Second"},new[]{"Third","Fourth"}}){var related=await e.RelateAsync(relation,new RequestInputRecords{Items=names.Select(n=>new RequestItem{Original=new RequestOriginalJson{Value=JsonSerializer.SerializeToElement(new{name=n,kind="alert"})}}).ToArray()});Check(related.Packets.OfType<R.SessionPacketRelateAggregate>().Single().Value.Value.Count==2&&Facts(related).Records==1,"relation");Check(related.Value.Count==2&&related.Probability.SequenceEqual(related.Value.Select(edge=>edge.Probability)),"plain relation result");}
  try{await e.DecideAsync(Question,(RequestInputText)null!);throw new Exception("null input admitted");}catch(Failure f) when(f.Kind==FailureKind.Usage&&f.Facts is null){}
  Exception saved;try{await e.DecideAsync(Question,Records("owned-invalid"),new RequestOptions{Threshold=new RequestThresholdNumber{Value=2}});throw new Exception("invalid authored data accepted");}catch(Failure f){saved=f;}
  var backend=await Fails(async()=>{await Decide(e,"status-401");},2);Check(backend is SessionFailure b&&Facts(b.Call).RequestsSent==1,"failure actual started facts");string copied=Message(backend);
  await Fails(async()=>{await Decide(e,"zero-deadline",new RequestOptions{DeadlineMs=0});},3);
  using(var canceled=new CancellationTokenSource()){canceled.Cancel();await Fails(async()=>{await Decide(e,"no-arrival-pre-cancelled",cancellation:canceled.Token);},5);}
  var failures=await Task.WhenAll(new[]{"failure-one","failure-two"}.Select(state=>Fails(async()=>{await Decide(e,state);},2)));Check(Message(failures[0]).Contains("401")&&Message(failures[1]).Contains("403"),"independent owned errors");Answer(await Decide(e,"success"),true,.9);
  var one=Decide(e,"hold-facts-one");var two=Decide(e,"hold-facts-two");try{Marker("hold-facts-one");Marker("hold-facts-two");}finally{Release("hold-facts-one");Release("hold-facts-two");}var owned=await Task.WhenAll(one,two);Check(owned.All(c=>Facts(c).RequestsSent==1&&Facts(c).Seconds>0),"overlapping native facts");
  await Held(e,"hold-deadline",3);await Held(e,"hold-bulk-1",5,true);await Held(e,"hold-scalar",5);Answer(await Decide(e,"recovery-scalar"),true,.9);
  string copiedError=saved.Message;e.Dispose();Check(saved.Message==copiedError&&Message(backend)==copied&&owned.All(c=>Facts(c).Model.Value=="jev-1.13.0"),"owned facts/errors after close");try{await Decide(e,"closed");throw new Exception("closed admitted");}catch(ObjectDisposedException){}
  Console.WriteLine("MATRIX_PASS");
 }
 public static async Task<int> Main(string[] args){try{if(args[0]=="matrix")await Matrix();else if(args[0]=="named"){using var e=Engine.Open(new EngineSettings{Backend="local",Cache=new CacheDocumentDisabledCache{Value=new DisabledCache()}});var call=await Decide(e,"named-csharp");Answer(call,true,.9);Check(Facts(call).RequestsSent==1,"named count");Console.WriteLine("CSHARP_NAMED_BACKEND_PASS");}else throw new ArgumentException("mode");return 0;}catch(Exception e){Console.Error.WriteLine(e);return 1;}}
}
