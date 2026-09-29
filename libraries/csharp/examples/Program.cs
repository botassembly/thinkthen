using System;
using ThinkThen;
using var engine = Engine.Open();
var result = engine.Decide("Is it?", "example");
Answer answer = result.Value;
Console.WriteLine($"{answer.Outcome}: {answer.Probability}");
