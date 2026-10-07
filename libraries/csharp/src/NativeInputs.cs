using System.Runtime.InteropServices;
using System.Text;
namespace ThinkThen;
internal sealed class NativeInputs:IDisposable {
 readonly List<IntPtr> memory=new(),questions=new(),images=new();internal IntPtr Source;
 public void Dispose(){NativeComplete.thinkthen_source_free(Source);foreach(var q in questions)NativeComplete.thinkthen_question_free(q);foreach(var i in images)NativeComplete.thinkthen_image_free(i);foreach(var p in memory)Marshal.FreeHGlobal(p);}
 internal IntPtr Allocate(int size){var p=Marshal.AllocHGlobal(Math.Max(1,size));memory.Add(p);return p;}
 internal StringV1 String(string s){byte[] b=new UTF8Encoding(false,true).GetBytes(s);IntPtr p=Allocate(b.Length);Marshal.Copy(b,0,p,b.Length);return new(){data=p,len=(nuint)b.Length};}
 internal OptionalStringV1 String(Optional<string> s)=>s.Present?new(){present=1,value=String(s.Value)}:default;
 internal ContentV1 Content(Content c)=>new(){kind=c.Kind==ContentKind.Absent?0:(uint)c.Kind+1,data=c.Kind==ContentKind.Absent?default:String(c.Kind==ContentKind.Text?c.Text:c.Json.GetRawText())};
 internal OptionalContentV1 Content(Optional<Content> c)=>c.Present?new(){present=1,value=Content(c.Value)}:default;
 internal IntPtr Array<T>(IReadOnlyList<T> values) where T:struct {int size=Marshal.SizeOf<T>();IntPtr p=Allocate(checked(size*values.Count));for(int i=0;i<values.Count;i++)Marshal.StructureToPtr(values[i],IntPtr.Add(p,checked(i*size)),false);return p;}
 internal StringsV1 Strings(IReadOnlyList<string> s)=>new(){data=Array(s.Select(String).ToArray()),len=(nuint)s.Count};
 internal ChoicesV1 Choices(IReadOnlyList<Choice> choices)=>new(){data=Array(choices.Select(c=>new ChoiceV1{name=String(c.Name),description=Content(c.Description),weight=new(){present=c.Weight.Present?1:0,value=c.Weight.Value}}).ToArray()),len=(nuint)choices.Count};
 internal static RuleV1 Rule(Rule r)=>new(){kind=(uint)r.Kind,low=r.Low,high=r.High};
 internal static OptionalSizeV1 Size(Optional<ulong> s)=>new(){present=s.Present?1:0,value=checked((nuint)s.Value)};
 internal IntPtr Question(IntPtr engine,Question q){
  var members=q.Members.Select(m=>new MemberSpecV1{name=String(m.Name),question=Question(engine,m.Question)}).ToArray();
  var relations=q.Relations.Select(r=>new RelationV1{name=String(r.Name),source=String(r.Source),target=String(r.Target),reads=String(r.Reads),either=r.Either?1:0,single=r.Single?1:0}).ToArray();
  var spec=new QuestionSpecV1{kind=(uint)q.Kind+1,text=q.Kind>=Function.Annotate&&q.Text.Kind==ContentKind.Text&&q.Text.Text==""?default:Content(q.Text),yes=Content(q.Yes),no=Content(q.No),choices=Choices(q.Choices),threshold=Rule(q.Threshold),relation_threshold=Rule(q.RelationThreshold),model=String(q.Model),profile=String(q.Profile),batch=Size(q.Batch),batch_max=q.BatchMax?1:0,none=q.None?1:0,on=Strings(q.On),members=new(){data=Array(members),len=(nuint)members.Length},kinds=Choices(q.Kinds),relations=new(){data=Array(relations),len=(nuint)relations.Length},name_pointer=String(q.NamePointer),kind_pointer=String(q.KindPointer)};
  var author=q.Author.Present?Author(q.Author.Value):default;IntPtr result;int rc=q.Author.Present?NativeComplete.thinkthen_question_new_authored(engine,ref spec,ref author,out result):NativeComplete.thinkthen_question_new(engine,ref spec,out result);NativeComplete.Check(engine,rc);questions.Add(result);return result;
 }
 internal IntPtr Asked(IntPtr engine,QuestionInput input){int selected=(input.Question.Present?1:0)+(input.File.Present?1:0)+(input.Saved.Present?1:0)+(input.Named.Present?1:0)+(input.Reference.Present?1:0);if(selected!=1)throw new ArgumentException("select exactly one question input");if(input.Question.Present)return Question(engine,input.Question.Value);
  IntPtr result;int rc;if(input.File.Present)rc=NativeComplete.thinkthen_question_load(engine,String(input.File.Value),out result);else if(input.Saved.Present)rc=NativeComplete.thinkthen_question_parse(engine,(uint)input.Role,String(input.Saved.Value),out result);else if(input.Named.Present)rc=NativeComplete.thinkthen_question_load_named(engine,(uint)input.Role,String(input.Named.Value),out result);else rc=NativeComplete.thinkthen_question_load_reference(engine,(uint)input.Role,String(input.Reference.Value),out result);NativeComplete.Check(engine,rc);questions.Add(result);return result;
 }
 internal IntPtr Input(IntPtr engine,InputSource input){if(input.Records.Present==input.Files.Present)throw new ArgumentException("select exactly one input source");int rc;
  if(input.Files.Present){var f=input.Files.Value;var spec=new SourceSpecV1{paths=Strings(f.Paths),unit=(uint)f.Unit+1,window=checked((nuint)f.Window)};rc=NativeComplete.thinkthen_source_files(engine,ref spec,out Source);}
  else {var rows=input.Records.Value.Select(r=>{var handles=r.Images.Select(image=>{var b=StringBytes(image.Bytes);int code=NativeComplete.thinkthen_image_clone(engine,b.data,b.len,(uint)image.Media+1,String(image.Filename),out var handle);NativeComplete.Check(engine,code);images.Add(handle);return handle;}).ToArray();return new RecordV1{original=Content(r.Original),context=Content(r.Context),options=Choices(r.Options),images=new(){data=Array(handles),len=(nuint)handles.Length}};}).ToArray();rc=NativeComplete.thinkthen_source_records(engine,Array(rows),(nuint)rows.Length,out Source);}
  NativeComplete.Check(engine,rc);return Source;
 }
 StringV1 StringBytes(byte[] bytes){var p=Allocate(bytes.Length);Marshal.Copy(bytes,0,p,bytes.Length);return new(){data=p,len=(nuint)bytes.Length};}
 internal ControlsV1 Controls(CallControls c,long deadline,IntPtr token)=>new(){deadline_ms=deadline,cancel=token,context=Content(c.Context),batch=Size(c.Batch),batch_max=c.BatchMax?1:0,attempts=c.Attempts?1:0,surface=String("csharp")};
internal InputDeclarationV1 Declaration(InputDeclaration d)=>new(){kind=(uint)d.Kind,properties=new(){data=Array(d.Properties.Select(p=>new InputPropertyV1{name=String(p.Name),kind=(uint)p.Kind}).ToArray()),len=(nuint)d.Properties.Count},required=Strings(d.Required)};
internal QuestionAuthorV1 Author(QuestionAuthor a)=>new(){name=String(a.Name),wording_version=new(){present=a.WordingVersion.Present?1:0,value=a.WordingVersion.Value},item_schema=Declaration(a.ItemSchema),context_schema=Declaration(a.ContextSchema)};
}

