using ThinkThen;

string message = File.ReadAllText("message.txt");
await using var client = new ThinkThenClient();

ValueList<Entity> names = await client.RecognizeAsync(
    message, ["person", "organization"]);
foreach (Entity name in names)
    Console.WriteLine($"{name.Mention.Kind}: {name.Mention.Span.Text}");
