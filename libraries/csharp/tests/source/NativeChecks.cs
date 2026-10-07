using ThinkThen;
using System.Text.Json;
static class NativeChecks {
 static void Check(bool yes,string label){if(!yes)throw new Exception(label);}
 static Content Text(string s)=>new(ContentKind.Text,s,default);
 static Content Json(string s){using var d=JsonDocument.Parse(s);return new(ContentKind.Json,"",d.RootElement.Clone());}
 static RecordInput Record(Content c)=>new(Optional.Some(c),default,Array.Empty<Choice>(),Array.Empty<ImageInput>());
 static Question Question(Function f){var q=f switch{Function.Decide=>Questions.Decide(Text("Need attention?")),Function.Choose=>Questions.Choose(Text("Which?")),Function.Tag=>Questions.Tag(Text("Which?")),Function.Score=>Questions.Score(Text("How much?")),Function.Filter=>Questions.Filter(Text("Need attention?")),Function.Rank=>Questions.Rank(Text("Need attention?")),Function.Find=>Questions.Find(Text("Which?")),Function.Annotate=>Questions.Annotate(Text("")),Function.Recognize=>Questions.Recognize(Text("")),_=>Questions.Relate(Text(""))};
  if(f==Function.Choose||f==Function.Tag||f==Function.Score)q=q with {Choices=new[]{new Choice("first",default,default),new Choice("second",default,default)}};
  if(f==Function.Filter)q=q with {Threshold=new(RuleKind.Cut,.95,0)};
  if(f==Function.Annotate)q=q with {Members=new[]{new QuestionMember("check",Question(Function.Decide)),new QuestionMember("team",Question(Function.Choose))}};
  if(f==Function.Relate)q=q with {Relations=new[]{new Relation("supports","*","*",default,false,false)}};return q;
 }
 static void Call<T>(CompleteCall<T> c,int rows){Check(c.Schema=="thinkthen.result/2"&&c.Rows.Count==rows,"schema/row count");Check(c.AnswerId.Present==(rows==1)&&c.Meta.Present==(rows==1),"summary presence");Check(c.Facts.Present&&c.Facts.Value.CallId.Value.Length==64&&c.Facts.Value.RequestsSent>0,"actual facts");Check(c.Attempts.Present&&c.Attempts.Value.Count>0&&c.Observations.Count>0,"observations");foreach(var a in c.Attempts.Value)Check(a.SdkRequestId.Value.Length==64,"SDK request identity");}
 public static void Run(){using var e=Engine.Open(Environment.GetEnvironmentVariable("TT_NATIVE_SETTINGS"));var c=new CallControls(default,default,false,true);
  foreach(bool files in new[]{false,true}){
   var source=files?InputSource.FromFiles(new FileSource(new[]{Environment.GetEnvironmentVariable("TT_NATIVE_FILE")!},SourceUnit.Line,0)):InputSource.FromRecords(new[]{Record(Text("Maria Chen")),Record(Text("Alex Lee"))});
   var d=e.DecideComplete(QuestionInput.Asked(Question(Function.Decide)),source,c);Call(d,2);Check(d.Rows[0].Value.Boolean&&d.Rows[0].Common.Answer.Value.Probability.Value==.9,"decision");if(files)Check(d.Rows[0].Common.Position.Value.FirstLine.Value==1&&d.Rows[1].Common.Position.Value.LastLine.Value==2,"physical lines");
   var choose=e.ChooseComplete(QuestionInput.Asked(Question(Function.Choose)),source,c);Call(choose,2);Check(choose.Rows[0].Value.Value=="first"&&choose.Rows[0].Common.Answer.Value.Probabilities.Count==2,"choice distribution");
   var tag=e.TagComplete(QuestionInput.Asked(Question(Function.Tag)),source,c);Call(tag,2);Check(tag.Rows[0].Value.Count==2,"tag labels");
   var score=e.ScoreComplete(QuestionInput.Asked(Question(Function.Score)),source,c);Call(score,2);Check(score.Rows[0].Value==.1,"score");
   var filter=e.FilterComplete(QuestionInput.Asked(Question(Function.Filter)),source,c);Call(filter,2);Check(!filter.Rows[0].Value,"rejected filter row");
   var rank=e.RankComplete(QuestionInput.Asked(Question(Function.Rank)),source,c);Call(rank,2);Check(rank.Rows[0].Value.Value==1,"rank ordinal");
   var find=e.FindComplete(QuestionInput.Asked(Question(Function.Find)),source,c);Call(find,1);Check(find.Rows[0].Common.Answer.Value.Probabilities.Count==2&&find.Rows[0].Common.Details!.Inputs.Count==2,"find candidates");
   var annotate=e.AnnotateComplete(QuestionInput.Asked(Question(Function.Annotate)),source,c);Call(annotate,2);Check(annotate.Rows[0].Answers.Count==2&&annotate.Rows[0].Answers[0].Success.Value.Answer.Probability.Value==.9,"annotation members");
   var recognize=e.RecognizeComplete(QuestionInput.Asked(Question(Function.Recognize)),source,c);Call(recognize,2);Check(recognize.Rows[0].Value.Entities.Count>0&&recognize.Rows[0].Answer.Pieces.Count>0&&recognize.Rows[0].Located!.Present==files,"recognized spans");
   var entities=files?source:InputSource.FromRecords(new[]{Record(Json("{\"name\":\"Maria Chen\",\"kind\":\"person\"}")),Record(Json("{\"name\":\"Alex Lee\",\"kind\":\"person\"}"))});
   var relate=e.RelateComplete(QuestionInput.Asked(Question(Function.Relate)),entities,c);Call(relate,1);Check(relate.Rows[0].Value.Count==2&&relate.Rows[0].Questions.Count==2&&relate.Rows[0].Located!.Present==files,"relation endpoints");
  }
 }
}
