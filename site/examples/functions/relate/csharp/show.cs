using ThinkThen;

string message = File.ReadAllText("message.txt");
await using var client = new ThinkThenClient();

var worksFor = new RelationRule
{
    Source = "person",
    Target = "organization",
    Relation = "works_for",
};
var relations = await client.RelateAsync(message, [worksFor]);
foreach (var relation in relations.Answer)
    Console.WriteLine($"{relation.Source.Text} --{relation.Relation}--> "
        + $"{relation.Target.Text}");
