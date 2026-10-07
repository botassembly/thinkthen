using System.Runtime.InteropServices;
namespace ThinkThen;
internal static partial class NativeComplete {
 [DllImport(Library)]internal static extern int thinkthen_question_new_authored(IntPtr e,ref QuestionSpecV1 spec,ref QuestionAuthorV1 author,out IntPtr q);
 [DllImport(Library)]internal static extern int thinkthen_result_details(IntPtr r,nuint i,out DetailsV1 d);
 [DllImport(Library)]internal static extern int thinkthen_result_observation_details(IntPtr r,nuint i,out DetailsV1 d);
 [DllImport(Library)]internal static extern int thinkthen_result_question_author(IntPtr r,nuint i,out QuestionAuthorV1 a);
 [DllImport(Library)]internal static extern int thinkthen_result_member_author(IntPtr r,nuint i,nuint j,out QuestionAuthorV1 a);
 [DllImport(Library)]internal static extern int thinkthen_result_observation_author(IntPtr r,nuint i,out QuestionAuthorV1 a);
 [DllImport(Library)]internal static extern int thinkthen_result_rank_member_count(IntPtr r,nuint i,out nuint count);
 [DllImport(Library)]internal static extern int thinkthen_result_rank_member(IntPtr r,nuint i,nuint j,out RankViewV1 v);
 [DllImport(Library)]internal static extern int thinkthen_result_source_recognition(IntPtr r,nuint i,out SourceRecognitionV1 v);
 [DllImport(Library)]internal static extern int thinkthen_result_source_relations(IntPtr r,nuint i,out SourceRelationsV1 v);
 internal static CommonRow Common(IntPtr result,nuint i,CommonRow row){if(thinkthen_result_details(result,i,out var details)!=0||thinkthen_result_question_author(result,i,out var author)!=0)throw new InvalidOperationException("native row detail failed");return row with {Details=ReadDetails(details),Author=ReadQuestionAuthor(author)};}
 internal static ObservationEvent Event(IntPtr result,nuint i,ObservationEvent row){if(thinkthen_result_observation_details(result,i,out var details)!=0||thinkthen_result_observation_author(result,i,out var author)!=0)throw new InvalidOperationException("native observation detail failed");return row with {Details=ReadDetails(details),Author=ReadQuestionAuthor(author)};}
}
