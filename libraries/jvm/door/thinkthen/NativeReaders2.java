package thinkthen;
import java.lang.foreign.*;
import java.util.*;
import thinkthen.Complete.*;
import thinkthen.Ids.*;
import static thinkthen.NativeLayouts.*;
import static thinkthen.NativeRead.*;
import static java.lang.foreign.ValueLayout.*;
import static thinkthen.NativeReaders0.*;
import static thinkthen.NativeReaders1.*;
final class NativeReaders2 {
 static DecideRow readDecideRow(MemorySegment v) {return new DecideRow(readCommonRow(field(v,"thinkthen_decide_view_v1","common")),readDecideValue(field(v,"thinkthen_decide_view_v1","value")));}
 static ChooseRow readChooseRow(MemorySegment v) {return new ChooseRow(readCommonRow(field(v,"thinkthen_choose_view_v1","common")),optional(field(v,"thinkthen_choose_view_v1","value"), "thinkthen_optional_string_v1", x206 -> string(x206)));}
 static TagRow readTagRow(MemorySegment v) {return new TagRow(readCommonRow(field(v,"thinkthen_tag_view_v1","common")),array(field(v,"thinkthen_tag_view_v1","value"), "thinkthen_strings_v1", "thinkthen_string_v1", x209 -> string(x209)));}
 static ScoreRow readScoreRow(MemorySegment v) {return new ScoreRow(readCommonRow(field(v,"thinkthen_score_view_v1","common")),field(v,"thinkthen_score_view_v1","value").get(JAVA_DOUBLE,0));}
 static FilterRow readFilterRow(MemorySegment v) {return new FilterRow(readCommonRow(field(v,"thinkthen_filter_view_v1","common")),field(v,"thinkthen_filter_view_v1","value").get(JAVA_INT,0)!=0);}
 static RankRow readRankRow(MemorySegment v) {return new RankRow(readCommonRow(field(v,"thinkthen_rank_view_v1","common")),optional(field(v,"thinkthen_rank_view_v1","value"), "thinkthen_optional_size_v1", x216 -> (long)x216.get(JAVA_LONG,0)),optional(field(v,"thinkthen_rank_view_v1","question_name"), "thinkthen_optional_string_v1", x218 -> string(x218)));}
 static FindRow readFindRow(MemorySegment v) {return new FindRow(readCommonRow(field(v,"thinkthen_find_view_v1","common")),optional(field(v,"thinkthen_find_view_v1","value"), "thinkthen_optional_content_v1", x221 -> readContent(x221)),optional(field(v,"thinkthen_find_view_v1","index"), "thinkthen_optional_size_v1", x223 -> (long)x223.get(JAVA_LONG,0)));}
 static AnnotateRow readAnnotateRow(MemorySegment v) {return new AnnotateRow(readCommonRow(field(v,"thinkthen_annotate_view_v1","common")),array(field(v,"thinkthen_annotate_view_v1","answers"), "thinkthen_members_v1", "thinkthen_member_v1", x226 -> readAnnotationMember(x226)));}
 static RecognizeRow readRecognizeRow(MemorySegment v) {return new RecognizeRow(readCommonRow(field(v,"thinkthen_recognize_view_v1","common")),readRecognizeValue(field(v,"thinkthen_recognize_view_v1","value")),readRecognizeAnswer(field(v,"thinkthen_recognize_view_v1","answer")));}
 static RelateRow readRelateRow(MemorySegment v) {return new RelateRow(readCommonRow(field(v,"thinkthen_relate_view_v1","common")),array(field(v,"thinkthen_relate_view_v1","value"), "thinkthen_edges_v1", "thinkthen_edge_v1", x232 -> readEdge(x232)),array(field(v,"thinkthen_relate_view_v1","questions"), "thinkthen_relation_answers_v1", "thinkthen_relation_answer_v1", x234 -> readRelationAnswer(x234)));}
}
