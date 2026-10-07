namespace ThinkThen;
internal static partial class NativeComplete {
 internal static ChooseRow ReadChooseRow(ChooseViewV1 v) => new(ReadCommonRow(v.common),Opt(v.value.present,()=>String(v.value.value)));
 internal static TagRow ReadTagRow(TagViewV1 v) => new(ReadCommonRow(v.common),Array<StringV1,string>(v.value.data,v.value.len,x=>String(x)));
 internal static ScoreRow ReadScoreRow(ScoreViewV1 v) => new(ReadCommonRow(v.common),(double)v.value);
 internal static FilterRow ReadFilterRow(FilterViewV1 v) => new(ReadCommonRow(v.common),v.value!=0);
 internal static RankRow ReadRankRow(RankViewV1 v) => new(ReadCommonRow(v.common),Opt(v.value.present,()=>checked((ulong)v.value.value)),Opt(v.question_name.present,()=>String(v.question_name.value)));
 internal static FindRow ReadFindRow(FindViewV1 v) => new(ReadCommonRow(v.common),Opt(v.value.present,()=>ReadContent(v.value.value)),Opt(v.index.present,()=>checked((ulong)v.index.value)));
 internal static AnnotateRow ReadAnnotateRow(AnnotateViewV1 v) => new(ReadCommonRow(v.common),Array<MemberV1,AnnotationMember>(v.answers.data,v.answers.len,x=>ReadAnnotationMember(x)));
 internal static RecognizeRow ReadRecognizeRow(RecognizeViewV1 v) => new(ReadCommonRow(v.common),ReadRecognizeValue(v.value),ReadRecognizeAnswer(v.answer));
 internal static RelateRow ReadRelateRow(RelateViewV1 v) => new(ReadCommonRow(v.common),Array<EdgeV1,Edge>(v.value.data,v.value.len,x=>ReadEdge(x)),Array<RelationAnswerV1,RelationAnswer>(v.questions.data,v.questions.len,x=>ReadRelationAnswer(x)));
}
