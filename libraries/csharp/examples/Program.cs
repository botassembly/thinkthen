using System;
using ThinkThen;
using var engine = Engine.Open();
Answer answer = engine.Decide("Is it?", "example");
Console.WriteLine($"{answer.Outcome}: {answer.Probability}");
