using ThinkThen;
using ThinkThen.Inputs;
using ThinkThen.Results;
using var engine=Engine.Open(new EngineSettings());
var call=await engine.DecideAsync(new RequestQuestionText {Text=Environment.GetEnvironmentVariable("THINKTHEN_TEST_SMOKE_QUESTION")!},new RequestInputText {Text=Environment.GetEnvironmentVariable("THINKTHEN_TEST_SMOKE_TEXT")!});
Console.WriteLine("smoke: "+call.Packets.OfType<SessionPacketDecideRow>().Single().Value.Value.GetRawText());
