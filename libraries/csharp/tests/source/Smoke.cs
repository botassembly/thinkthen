// The replay smoke (ticket 0335): one Decide through Engine.Open, which reads
// the environment, with the question and text sdlc/scripts/smoke names.
using ThinkThen;
using var engine = Engine.Open();
var answer = engine.Decide(Environment.GetEnvironmentVariable("THINKTHEN_SMOKE_QUESTION")!, Environment.GetEnvironmentVariable("THINKTHEN_SMOKE_TEXT")!).Value;
Console.WriteLine("smoke: " + answer.OutcomeKind switch { Outcome.Yes => "true", Outcome.No => "false", _ => "null" });
