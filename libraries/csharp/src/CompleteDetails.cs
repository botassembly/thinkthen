namespace ThinkThen;
public enum DeclarationKind:uint { Absent,String,Object }
public enum PropertyKind:uint { String=1,Number,Boolean,StringList }
public sealed record InputProperty(string Name,PropertyKind Kind);
public sealed record InputDeclaration(DeclarationKind Kind,IReadOnlyList<InputProperty> Properties,IReadOnlyList<string> Required);
public sealed record QuestionAuthor(Optional<string> Name,Optional<ulong> WordingVersion,InputDeclaration ItemSchema,InputDeclaration ContextSchema);
public sealed record ReportedUsage(bool Present,Optional<ulong> InputTokens,Optional<ulong> OutputTokens);
public sealed record SourceDetail(Origin Origin,string AnsweredBy,Optional<ulong> BatchSize);
public sealed record InputView(Optional<Content> Original,Optional<Location> Position,Optional<IReadOnlyList<ImageView>> Images);
public sealed record Details(Optional<Question> Question,Optional<Rule> Threshold,Optional<string> RawPick,ReportedUsage Usage,IReadOnlyList<SourceDetail> QuestionSources,IReadOnlyList<ObservationIdentity> Observations,IReadOnlyList<InputView> Inputs);
public sealed record SourceEntity(Entity Entity,Optional<Location> Position);
public sealed record SourceEntityEdge(string Relation,SourceEntity Source,SourceEntity Target,double Probability,bool Either);
public sealed record SourceRecognition(bool Present,IReadOnlyList<SourceEntity> Entities,Optional<IReadOnlyList<SourceEntityEdge>> Relations);
public sealed record SourceEndpoint(ulong Ordinal,Endpoint Endpoint,Content Record,Optional<Location> Position);
public sealed record SourceEdge(string Relation,SourceEndpoint Source,SourceEndpoint Target,double Probability,bool Either);
public sealed record SourceRelations(bool Present,IReadOnlyList<SourceEdge> Edges);
