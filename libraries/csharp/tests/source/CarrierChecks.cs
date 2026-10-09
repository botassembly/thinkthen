using System.Text.Json;
using ThinkThen.Inputs;
using ThinkThen.Results;
static class CarrierChecks
{
 static void Check(bool value) {if(!value)throw new Exception("owned generated carrier contract");}
 static void Refuses(Action action) {try{action();}catch(Exception e) when(e is JsonException or KeyNotFoundException or InvalidOperationException or NotSupportedException){return;}throw new Exception("invalid field accepted");}
 public static void Run() {
  using var doc=JsonDocument.Parse("{\"call_id\":\"owned\",\"cache_answers\":0,\"records\":0,\"requests_sent\":0,\"seconds\":0,\"input_tokens\":0,\"estimated_cost_usd\":\"0.000000\",\"largest_request_bytes\":0,\"largest_request_estimated_input_tokens\":null,\"token_estimate_method\":\"native\",\"future\":{\"n\":1}}");
  var facts=new Facts(doc.RootElement);Check(facts.CallId=="owned"&&facts.InputTokens.State==PresenceState.Value&&facts.InputTokens.Value==0&&facts.OutputTokens.State==PresenceState.Missing&&facts.EstimatedCostUsd.Value=="0.000000"&&facts.LargestRequestEstimatedInputTokens.State==PresenceState.Null);
  Check(facts.ToJson().GetProperty("future").GetProperty("n").GetInt32()==1);
  using var missing=JsonDocument.Parse("{}");Refuses(()=>_=new Facts(missing.RootElement).CallId);Refuses(()=>_=new Facts(missing.RootElement).LargestRequestEstimatedInputTokens);
  using var odds=JsonDocument.Parse("{\"kind\":\"choice\",\"pick\":\"β\",\"probabilities\":{\"β\":0.7,\"a\":0.3},\"confidence\":0}");
  var choice=(AnswerChoice)Answer.Read(odds.RootElement);Check(choice.Probabilities.Keys.First()=="β"&&choice.Confidence.State==PresenceState.Value&&choice.Confidence.Value==0);
  Refuses(()=>((IDictionary<string,double>)choice.Probabilities).Clear());
  using var unknown=JsonDocument.Parse("{\"kind\":\"other\"}");Refuses(()=>Answer.Read(unknown.RootElement));
  using var nullOdds=JsonDocument.Parse("{\"kind\":\"yes_no\",\"probability\":null}");Refuses(()=>_=((AnswerYesNo)Answer.Read(nullOdds.RootElement)).Probability);
  var question=new InputRequestQuestionFile {Path="β.json"};var source=new InputRequestInputSource {Source=new InputRequestSource {Paths=new[]{"β.png","β.png"},Media=new InputReaderMediaAlternative1(),Reading=new InputRequestReader {Unit=new InputSourceUnitAlternative2()}}};
  var request=new InputRequest {Schema=new InputRequestVersionAlternative0(),Call=new InputRequestCallDecide {Question=question,Input=source,Options=new InputRequestOptions {None=false}}};
  using var stream=new MemoryStream();using(var writer=new Utf8JsonWriter(stream))request.Write(writer);using var input=JsonDocument.Parse(stream.ToArray());
  Check(input.RootElement.GetProperty("call").GetProperty("input").GetProperty("source").GetProperty("paths").GetArrayLength()==2);
  foreach(string member in new[]{"true","null","\"billing\"","[\"billing\",\"urgent\"]","1.2","{\"failed\":{\"kind\":\"backend\",\"cause\":\"missing_probability\",\"later\":1}}"}){using var value=JsonDocument.Parse(member);Check(AnnotatedField.Read(value.RootElement).ToJsonString()==member);}
 }
}
