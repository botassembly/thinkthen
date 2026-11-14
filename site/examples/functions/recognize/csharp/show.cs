using ThinkThen;

string message = File.ReadAllText("message.txt");
await using var client = new ThinkThenClient();

var names = await client.RecognizeAsync(
    message, ["person", "organization"]);
foreach (var name in names.Answer)
    Console.WriteLine($"{name.Kind}: {name.Text}");
