namespace ThinkThen;
public sealed record DecideOptions
{
    public Pointer? Evidence { get; init; }
    public Threshold? Threshold { get; init; }
}
public sealed record ChooseOptions
{
    public Pointer? Evidence { get; init; }
    public Threshold? Threshold { get; init; }
}
public sealed record TagOptions
{
    public Pointer? Evidence { get; init; }
    public Cut? Threshold { get; init; }
}
public sealed record ScoreOptions
{
    public Pointer? Evidence { get; init; }
}
public sealed record FilterOptions
{
    public Pointer? Evidence { get; init; }
    public Threshold? Threshold { get; init; }
}
public sealed record RankOptions
{
    public Pointer? Evidence { get; init; }
    public int? Top { get; init; }
}
public sealed record FindOptions
{
    public Pointer? Evidence { get; init; }
    public Cut? Threshold { get; init; }
}
public sealed record RecognizeOptions
{
    public Pointer? Evidence { get; init; }
    public Cut? Threshold { get; init; }
}
public sealed record RelateOptions
{
    public Pointer? Evidence { get; init; }
    public Cut? Threshold { get; init; }
}
public sealed record AnnotateOptions
{
    public Pointer? Evidence { get; init; }
}
