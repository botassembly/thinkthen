package thinkthen;
import java.lang.foreign.*;
import java.util.*;
import thinkthen.Complete.*;
import thinkthen.CompleteDetails.*;
import static thinkthen.NativeRead.*;
import static thinkthen.NativeLayouts.*;
import static thinkthen.NativeReaders0.*;
import static thinkthen.NativeReaders1.*;
import static thinkthen.NativeReaders2.*;
import static thinkthen.NativeDetailReaders0.*;
import static thinkthen.NativeCalls.*;
import static java.lang.foreign.ValueLayout.*;
final class NativeDetails {
 static MemorySegment get(Arena arena,MemorySegment result,long i,String method,String type){MemorySegment v=arena.allocate(layout("thinkthen_"+type+"_v1"));if((int)call("thinkthen_result_"+method,result,i,v)!=0)throw new IllegalStateException("native "+method+" failed");return v;}
 static CommonRow common(MemorySegment result,long i,CommonRow c){try(Arena a=Arena.ofConfined()){return new CommonRow(c.answerId(),c.input(),c.question(),c.answer(),c.threshold(),c.position(),c.inputFile(),c.meta(),c.images(),readDetails(get(a,result,i,"details","details")),readQuestionAuthor(get(a,result,i,"question_author","question_author")));}}
 static ObservationEvent event(MemorySegment result,long i,ObservationEvent e){try(Arena a=Arena.ofConfined()){return new ObservationEvent(e.kind(),e.question(),e.row(),readDetails(get(a,result,i,"observation_details","details")),readQuestionAuthor(get(a,result,i,"observation_author","question_author")));}}
static DecideRow decide(MemorySegment result,long i,MemorySegment v){DecideRow r=readDecideRow(v);return new DecideRow(common(result,i,r.common()),r.value());}
static ChooseRow choose(MemorySegment result,long i,MemorySegment v){ChooseRow r=readChooseRow(v);return new ChooseRow(common(result,i,r.common()),r.value());}
static TagRow tag(MemorySegment result,long i,MemorySegment v){TagRow r=readTagRow(v);return new TagRow(common(result,i,r.common()),r.value());}
static ScoreRow score(MemorySegment result,long i,MemorySegment v){ScoreRow r=readScoreRow(v);return new ScoreRow(common(result,i,r.common()),r.value());}
static FilterRow filter(MemorySegment result,long i,MemorySegment v){FilterRow r=readFilterRow(v);return new FilterRow(common(result,i,r.common()),r.value());}
static RankRow rank(MemorySegment result,long i,MemorySegment v){RankRow r=readRankRow(v);List<RankRow> members=new ArrayList<>();try(Arena a=Arena.ofConfined()){MemorySegment n=a.allocate(JAVA_LONG);if((int)call("thinkthen_result_rank_member_count",result,i,n)!=0)throw new IllegalStateException("native rank members failed");for(long j=0;j<l(n);j++){MemorySegment m=a.allocate(layout("thinkthen_rank_view_v1"));if((int)call("thinkthen_result_rank_member",result,i,j,m)!=0)throw new IllegalStateException("native rank member failed");members.add(readRankRow(m));}}return new RankRow(common(result,i,r.common()),r.value(),r.questionName(),members);}
static FindRow find(MemorySegment result,long i,MemorySegment v){FindRow r=readFindRow(v);return new FindRow(common(result,i,r.common()),r.value(),r.index());}
static AnnotateRow annotate(MemorySegment result,long i,MemorySegment v){AnnotateRow r=readAnnotateRow(v);List<AnnotationMember> members=new ArrayList<>();try(Arena a=Arena.ofConfined()){for(int j=0;j<r.answers().size();j++){AnnotationMember m=r.answers().get(j);MemorySegment author=a.allocate(layout("thinkthen_question_author_v1"));if((int)call("thinkthen_result_member_author",result,i,(long)j,author)!=0)throw new IllegalStateException("native member author failed");members.add(new AnnotationMember(m.name(),m.request(),m.question(),m.state(),m.success(),m.failure(),readQuestionAuthor(author)));}}return new AnnotateRow(common(result,i,r.common()),members);}
static RecognizeRow recognize(MemorySegment result,long i,MemorySegment v){RecognizeRow r=readRecognizeRow(v);SourceRecognition located;try(Arena a=Arena.ofConfined()){located=readSourceRecognition(get(a,result,i,"source_recognition","source_recognition"));}return new RecognizeRow(common(result,i,r.common()),r.value(),r.answer(),located);}
static RelateRow relate(MemorySegment result,long i,MemorySegment v){RelateRow r=readRelateRow(v);SourceRelations located;try(Arena a=Arena.ofConfined()){located=readSourceRelations(get(a,result,i,"source_relations","source_relations"));}return new RelateRow(common(result,i,r.common()),r.value(),r.questions(),located);}
}
