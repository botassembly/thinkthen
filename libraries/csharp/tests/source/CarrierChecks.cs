using ThinkThen;
using ThinkThen.Inputs;
using ThinkThen.Results;
static class CarrierChecks
{
 public static void Run() {
  using var engine=Engine.Open();
  var input=new RequestInputRecords {Items=new[] {new RequestItem {Original=new RequestOriginalText {Text="first"}},new RequestItem {Original=new RequestOriginalText {Text="second"}}}};
  var decide=new RequestQuestionText {Text="Is it?"};
  var calls=new (RequestFunction,RequestQuestion)[] {
   (new RequestFunctionDecide(),decide),
   (new RequestFunctionChoose(),FixtureQuestions.Parse(AuthoredQuestionKind.Atomic,"{\"choose\":\"Which?\",\"options\":[\"first\",\"second\"]}")),
   (new RequestFunctionTag(),FixtureQuestions.Parse(AuthoredQuestionKind.Atomic,"{\"tag\":\"Which?\",\"labels\":[\"first\",\"second\"]}")),
   (new RequestFunctionScore(),FixtureQuestions.Parse(AuthoredQuestionKind.Atomic,"{\"score\":\"What level?\",\"levels\":[\"Low.\",\"High.\"]}")),
   (new RequestFunctionFilter(),decide),
   (new RequestFunctionRank(),decide)
  };
  var retained=new List<Plan>();
  foreach(var (function,question) in calls) {
   var options=new RequestOptions {None=false};
   var plan=engine.Plan(new Request {Schema=new RequestVersionThinkthenRequest1(),Call=new RequestCall {Function=function,Question=question,Input=input,Options=options}});
   if(plan.Records!=2 || plan.Requests==0 || plan.EstimatedBytes==0 || plan.LargestRequestBytes>plan.EstimatedBytes || plan.EstimatedInputTokens.Lower>plan.EstimatedInputTokens.Upper || plan.LargestRequestEstimatedInputTokens==0 || plan.TokenEstimateMethod.Length==0 || plan.FirstBodyUtf8.State!=PresenceState.Value || plan.UpperBound || !options.None.IsPresent || options.None.Value) throw new Exception("typed canonical plan fields or explicit false changed");
   retained.Add(plan);
  }
  foreach(RequestFunction function in new RequestFunction[] {new RequestFunctionFind(),new RequestFunctionAnnotate(),new RequestFunctionRecognize(),new RequestFunctionRelate()}) {
   try {engine.Plan(new Request {Schema=new RequestVersionThinkthenRequest1(),Call=new RequestCall {Function=function,Question=decide,Input=input}});throw new Exception("unsupported plan admitted");}
   catch(Failure failure) when(failure.Kind==FailureKind.Usage && failure.Code==1 && !failure.Retryable && failure.Facts is null) { }
  }
  var empty=engine.Plan(new Request {Schema=new RequestVersionThinkthenRequest1(),Call=new RequestCall {Function=new RequestFunctionDecide(),Question=decide,Input=new RequestInputRecords {Items=Array.Empty<RequestItem>()}}});
  if(empty.Records!=0 || empty.Requests!=0 || empty.FirstBodyUtf8.State!=PresenceState.Null) throw new Exception("empty plan lost nullable body");
  engine.Dispose();
  if(retained.Any(plan=>plan.Records!=2 || plan.EstimatedInputTokens.Lower>plan.EstimatedInputTokens.Upper)) throw new Exception("owned typed plan changed");
 }
}
