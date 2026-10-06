namespace ThinkThen;
/// <summary>Typed defaults; validation and question-file parsing stay native.</summary>
public static class Questions {
private static Question New(Function kind, Content text) => new(kind, text, default, default, Array.Empty<Choice>(), new(RuleKind.Default, 0, 0), new(RuleKind.Default, 0, 0), default, default, default, false, false, Array.Empty<string>(), Array.Empty<QuestionMember>(), Array.Empty<Choice>(), Array.Empty<Relation>(), default, default);
public static Question Decide(Content text) => New(Function.Decide, text);
public static Question Choose(Content text) => New(Function.Choose, text);
public static Question Tag(Content text) => New(Function.Tag, text);
public static Question Score(Content text) => New(Function.Score, text);
public static Question Filter(Content text) => New(Function.Filter, text);
public static Question Rank(Content text) => New(Function.Rank, text);
public static Question Find(Content text) => New(Function.Find, text);
public static Question Annotate(Content text) => New(Function.Annotate, text);
public static Question Recognize(Content text) => New(Function.Recognize, text);
public static Question Relate(Content text) => New(Function.Relate, text);
}
