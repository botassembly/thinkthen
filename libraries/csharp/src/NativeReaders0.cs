namespace ThinkThen;
internal static partial class NativeComplete {
 internal static Rule ReadRule(RuleV1 v) => new((RuleKind)checked((int)v.kind),(double)v.low,(double)v.high);
 internal static Choice ReadChoice(ChoiceV1 v) => new(String(v.name),Opt(v.description.present,()=>ReadContent(v.description.value)),Opt(v.weight.present,()=>(double)v.weight.value));
 internal static Relation ReadRelation(RelationV1 v) => new(String(v.name),String(v.source),String(v.target),Opt(v.reads.present,()=>String(v.reads.value)),v.either!=0,v.single!=0);
 internal static QuestionMember ReadQuestionMember(QuestionMemberV1 v) => new(String(v.name),ReadQuestion(System.Runtime.InteropServices.Marshal.PtrToStructure<QuestionViewV1>(v.question)));
 internal static Location ReadLocation(LocationV1 v) => new(Opt(v.file.present,()=>String(v.file.value)),Opt(v.first_line.present,()=>checked((ulong)v.first_line.value)),Opt(v.last_line.present,()=>checked((ulong)v.last_line.value)));
 internal static MemberFailure ReadMemberFailure(MemberFailureV1 v) => new(new FailureId(String(v.failure_id)),(MemberCause)checked((int)v.cause-1));
 internal static MemberSuccess ReadMemberSuccess(MemberSuccessV1 v) => new(new AnswerId(String(v.answer_id)),ReadMemberValue(v.value),ReadAtomicAnswer(v.answer),ReadRule(v.threshold));
 internal static Entity ReadEntity(EntityV1 v) => new(String(v.text),checked((ulong)v.start),checked((ulong)v.end),checked((ulong)v.length),String(v.kind),(double)v.strength);
 internal static EntityEdge ReadEntityEdge(EntityEdgeV1 v) => new(String(v.relation),ReadEntity(v.source),ReadEntity(v.target),(double)v.probability,v.either!=0);
 internal static Place ReadPlace(PlaceV1 v) => new(checked((ulong)v.start),checked((ulong)v.end));
 internal static Piece ReadPiece(PieceV1 v) => new(checked((ulong)v.start),checked((ulong)v.end),Array<ProbabilityV1,Probability>(v.tags.data,v.tags.len,x=>ReadProbability(x)));
 internal static NameSpan ReadNameSpan(NameV1 v) => new(checked((ulong)v.start),checked((ulong)v.end),Opt(v.kinds.present,()=>Array<ProbabilityV1,Probability>(v.kinds.value.data,v.kinds.value.len,x=>ReadProbability(x))),Opt(v.edges.present,()=>Array<ProbabilityV1,Probability>(v.edges.value.data,v.edges.value.len,x=>ReadProbability(x))));
 internal static PairSpan ReadPairSpan(PairV1 v) => new(String(v.relation),ReadPlace(v.source),ReadPlace(v.target),(double)v.probability);
 internal static RecognizeValue ReadRecognizeValue(RecognizeValueV1 v) => new(Array<EntityV1,Entity>(v.entities.data,v.entities.len,x=>ReadEntity(x)),Opt(v.relations.present,()=>Array<EntityEdgeV1,EntityEdge>(v.relations.value.data,v.relations.value.len,x=>ReadEntityEdge(x))));
}
