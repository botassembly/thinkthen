using ThinkThen;
using ThinkThen.Inputs;
static class CarrierChecks
{
 public static void Run() {
  using var engine=Engine.Open();
  var request=new Request {Schema=new RequestVersionThinkthenRequest1(),Call=new RequestCall { Function = new RequestFunctionDecide(),Question=new RequestQuestionText {Text="Is it?"},Input=new RequestInputRecords {Items=new[] {new RequestItem {Original=new RequestOriginalText {Text="first"}},new RequestItem {Original=new RequestOriginalText {Text="second"}}}},Options=new RequestOptions {None=false}}};
  var plan=engine.Plan(request);
  if(plan.Records!=2 || plan.Requests==0 || !request.Call.Options.Value.None.IsPresent || request.Call.Options.Value.None.Value) throw new Exception("typed records or explicit false changed");
  engine.Dispose();
  if(plan.Records!=2 || plan.EstimatedInputTokens.Lower>plan.EstimatedInputTokens.Upper) throw new Exception("owned typed plan changed");
 }
}
