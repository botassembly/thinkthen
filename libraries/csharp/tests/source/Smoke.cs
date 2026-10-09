using ThinkThen;
using ThinkThen.Inputs;
using ThinkThen.Results;
using var engine=Engine.Open(new InputEngineSettings());
var call=await engine.DecideAsync(new InputRequestQuestionText {Text=Environment.GetEnvironmentVariable("THINKTHEN_TEST_SMOKE_QUESTION")!},new InputRequestInputText {Text=Environment.GetEnvironmentVariable("THINKTHEN_TEST_SMOKE_TEXT")!});
Console.WriteLine("smoke: "+call.Packets.OfType<SessionPacketDecideRow>().Single().Value.Value.GetRawText());
