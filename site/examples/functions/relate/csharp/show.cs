using ThinkThen;

string message = File.ReadAllText("message.txt");
await using var client = new ThinkThenClient();

var worksFor = new RelationRule
{
    Source = "person",
    Target = "organization",
    Name = "works_for",
};
ValueList<Relation> relations = await client.RelateAsync(message, [worksFor]);
foreach (Relation relation in relations)
    Console.WriteLine($"{relation.Source.Span.Text} --{relation.Name}--> "
        + $"{relation.Target.Span.Text}");
