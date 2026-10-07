import java.util.List;
import thinkthen.*;
import thinkthen.Complete.*;
import thinkthen.CompleteEngine.CompleteCall;
import thinkthen.Requests.*;

/** Actual public named calls and owned typed views, shared assertions for each JVM consumer. */
public final class NativeChecks {
 static void check(boolean yes,String label){if(!yes)throw new AssertionError(label);}
 static Content text(String s){return new Content(ContentKind.TEXT,s,null);}
 static Question question(Function f){Question q=Questions.create(f,text(f.ordinal()<7?"Does this need attention?":""));
  List<Choice> choices=List.of();if(f==Function.CHOOSE||f==Function.TAG||f==Function.SCORE)choices=List.of(new Choice("first",OptionalValue.absent(),OptionalValue.absent()),new Choice("second",OptionalValue.absent(),OptionalValue.absent()));
  List<QuestionMember> members=f==Function.ANNOTATE?List.of(new QuestionMember("check",question(Function.DECIDE)),new QuestionMember("team",question(Function.CHOOSE))):List.of();
  List<Relation> relations=f==Function.RELATE?List.of(new Relation("supports","*","*",OptionalValue.absent(),false,false)):List.of();
  return new Question(f,q.text(),q.yes(),q.no(),choices,f==Function.FILTER?new Rule(RuleKind.CUT,.95,0.0):q.threshold(),q.relationThreshold(),q.model(),q.profile(),q.batch(),q.batchMax(),q.none(),q.on(),members,q.kinds(),relations,q.namePointer(),q.kindPointer());
 }
 static RecordInput record(Content content){return new RecordInput(OptionalValue.of(content),OptionalValue.absent(),List.of(),List.of());}
 static <T> void call(CompleteCall<T> c,int rows){check(c.schema().equals("thinkthen.result/2")&&c.rows().size()==rows,"row count/schema");check(c.answerId().present()==(rows==1)&&c.meta().present()==(rows==1),"summary presence");check(c.facts().present()&&c.facts().value().callId().value().length()==64&&c.facts().value().requestsSent()>0,"actual facts");check(c.attempts().present()&&!c.attempts().value().isEmpty()&&!c.observations().isEmpty(),"actual observations");for(Attempt a:c.attempts().value())check(a.sdkRequestId().value().length()==64,"request identity");}
 public static CompleteCall<DecideRow> run(CompleteEngine engine){CallControls controls=new CallControls(OptionalValue.absent(),OptionalValue.absent(),false,true);CompleteCall<DecideRow> last=null;
  for(boolean files:List.of(false,true)){
   InputSource source=files?InputSource.files(new FileSource(List.of(System.getenv("TT_NATIVE_FILE")),SourceUnit.LINE,0L)):InputSource.records(List.of(record(text("Maria Chen")),record(text("Alex Lee"))));
   var d=engine.decideComplete(QuestionInput.asked(question(Function.DECIDE)),source,controls,-1,null);call(d,2);check(d.rows().get(0).value().booleanValue()&&d.rows().get(0).common().answer().value().probability().value()==.9,"decision probability");if(files)check(d.rows().get(0).common().position().value().firstLine().value()==1&&d.rows().get(1).common().position().value().lastLine().value()==2,"physical CRLF lines");last=d;
   var c=engine.chooseComplete(QuestionInput.asked(question(Function.CHOOSE)),source,controls,-1,null);call(c,2);check(c.rows().get(0).value().value().equals("first")&&c.rows().get(0).common().answer().value().probabilities().size()==2,"choice distribution");
   var tag=engine.tagComplete(QuestionInput.asked(question(Function.TAG)),source,controls,-1,null);call(tag,2);check(tag.rows().get(0).value().size()==2,"tag labels");
   var score=engine.scoreComplete(QuestionInput.asked(question(Function.SCORE)),source,controls,-1,null);call(score,2);check(score.rows().get(0).value()==.1,"score value");
   var filter=engine.filterComplete(QuestionInput.asked(question(Function.FILTER)),source,controls,-1,null);call(filter,2);check(!filter.rows().get(0).value(),"rejected filter row");
   var rank=engine.rankComplete(QuestionInput.asked(question(Function.RANK)),source,controls,-1,null);call(rank,2);check(rank.rows().get(0).value().value()==1,"rank ordinal");
   var find=engine.findComplete(QuestionInput.asked(question(Function.FIND)),source,controls,-1,null);call(find,1);check(find.rows().get(0).common().answer().value().probabilities().size()==2&&find.rows().get(0).common().details().inputs().size()==2,"find candidates");
   var annotate=engine.annotateComplete(QuestionInput.asked(question(Function.ANNOTATE)),source,controls,-1,null);call(annotate,2);check(annotate.rows().get(0).answers().size()==2&&annotate.rows().get(0).answers().get(0).success().value().answer().probability().value()==.9,"annotation members");
   var recognize=engine.recognizeComplete(QuestionInput.asked(question(Function.RECOGNIZE)),source,controls,-1,null);call(recognize,2);check(!recognize.rows().get(0).value().entities().isEmpty()&&!recognize.rows().get(0).answer().pieces().isEmpty()&&recognize.rows().get(0).located().present()==files,"recognized spans");
   InputSource entities=files?source:InputSource.records(List.of(record(new Content(ContentKind.JSON,"",Json.parse("{\"name\":\"Maria Chen\",\"kind\":\"person\"}"))),record(new Content(ContentKind.JSON,"",Json.parse("{\"name\":\"Alex Lee\",\"kind\":\"person\"}")))));
   var relate=engine.relateComplete(QuestionInput.asked(question(Function.RELATE)),entities,controls,-1,null);call(relate,1);check(relate.rows().get(0).value().size()==2&&relate.rows().get(0).questions().size()==2&&relate.rows().get(0).located().present()==files,"relation endpoints");
  }
  return last;
 }
}
