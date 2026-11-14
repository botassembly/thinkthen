using System.Text.Json;
using System.Text.Json.Serialization;
using ThinkThen;

internal static class CSharpIdealCases
{
    private enum Topic { Billing, Shipping, Other }
    private enum Urgency { Routine=10, Urgent=0 }
    private enum Phrase { LateDelivery, Routine }
    private static void Check(bool condition,string reason) { if(!condition) throw new Exception(reason); }
    internal static async Task Run(string scenario)
    {
        if(scenario=="yes") await ScalarTiming();
        if(scenario=="failed") await ScalarServiceFailure();
        Row<TextSpan?,string>? retained=null;
        Row<Annotations<Dictionary<string,object>>,Dictionary<string,object>>? retainedForm=null;
        await using(var client=new ThinkThenClient(new() { Retry=new RetryPolicy { MaxRetries=0 } }))
        {
            var original=new Dictionary<string,object> { ["text"]="Please refund my order." };
            var options=new DecideOptions { Threshold=Threshold.Band(.2,.8) };
            var first=await First(client.DecideStreamAsync("Ready?",new[]{original},options));
            var equal=await First(client.DecideStreamAsync("Ready?",new[]{new Dictionary<string,object>(original)},options));
            Check(ReferenceEquals(first.Item,original),"single results retain the exact original record");
            Check(first.Equals(equal)&&first.GetHashCode()==equal.GetHashCode(),"original identity does not participate in equality or hashing");
            var set=new HashSet<Row<bool,Dictionary<string,object>>> { first };
            original["mutable"]=new List<string> { "changed" };
            Check(set.Contains(first)&&first.Equals(equal),"changing an original cannot change result hashing or equality");
            Check(!first.Equals(true),"a result never equals a bare answer");
            Check(!first.Equals(await First(client.DecideStreamAsync("Different?",new[]{original},options))),"source identity participates in result equality");
            using(var viewed=JsonDocument.Parse(JsonSerializer.Serialize<IRow>(first)))
                Check(!viewed.RootElement.TryGetProperty("item",out _)&&viewed.RootElement.GetProperty("status").GetString()==first.Status.ToString().ToLowerInvariant(),"non-generic row views serialize the same safe saved-row contract");
            Check(first.Detail is null,"no detail is safe to read in every state");
            if(first.IsAnswered) Check(first.ToString()==(first.Answer ? "true" : "false"),"decide prints lowercase command words");
            else
            {
                try { _=first.Answer; throw new Exception("unanswered row read succeeded"); }
                catch(NoAnswerException error)
                {
                    if(first.IsUnsure) Check(error is UnsureException unsure&&ReferenceEquals(unsure.Row,first),"uncertainty retains its row and no-answer family");
                    else
                    {
                        Check(error is FailedException failure&&ReferenceEquals(failure.Rows[0],first),"record failure retains its row and no-answer family");
                        using var savedFailure=JsonDocument.Parse(JsonSerializer.Serialize(((FailedException)error).Rows[0]));
                        Check(savedFailure.RootElement.TryGetProperty("error",out _)&&!savedFailure.RootElement.TryGetProperty("item",out _),"failed non-generic views save their error without the original item");
                    }
                }
                Check(first.IsUnsure ? first.ToString()=="unsure" : first is IFailed failed && first.ToString().StartsWith("failed: "+failed.Error.Code+": ",StringComparison.Ordinal),"printing an unanswered result is safe");
            }
            var scale=ScoreQuestion.Create("Urgency?",new[]{Urgency.Routine,Urgency.Urgent});
            var ranks=RankQuestion.Create("Urgency?",new[]{Urgency.Routine,Urgency.Urgent});
            Check(scale.Levels.SequenceEqual(new[]{"routine","urgent"})&&ranks.Levels.SequenceEqual(scale.Levels),"the supplied scale list overrides enum numeric order");
            var score=await First(client.ScoreStreamAsync(scale,new[]{"text"}));
            var direct=await First(client.ScoreStreamAsync("Urgency?",new[]{"text"},new[]{Urgency.Routine,Urgency.Urgent}));
            Check(score.Source.QuestionIdentity==direct.Source.QuestionIdentity,"direct score calls preserve the admitted ordered scale");
            if(score.IsAnswered) Check(score.Answer==(scenario=="low-score" ? 0 : scenario=="unsure" ? .5 : 1),"score stays a number on the supplied scale");
            var questions=new AnnotationQuestions
            {
                {"refund",DecideQuestion.Create("Refund?",options)},
                {"topic",ChooseQuestion.Create("Topic?",Enum.GetValues<Topic>())},
                {"urgency",scale},
            };
            var formOriginal=new Dictionary<string,object> { ["text"]="text" };
            var form=await client.AnnotateAsync(questions,formOriginal);
            var same=await client.AnnotateAsync(questions,new Dictionary<string,object>(formOriginal));
            Check(form.Equals(same)&&form.GetHashCode()==same.GetHashCode(),"forms compare the values of their ordinary nested rows");
            var forms=new HashSet<Row<Annotations<Dictionary<string,object>>,Dictionary<string,object>>> {form};
            formOriginal["changed"]=new List<string>{"mutable"};
            Check(forms.Contains(form)&&form.Equals(same),"nested form item mutation never affects hashing");
            if(scenario=="failed"||scenario.EndsWith("-failed",StringComparison.Ordinal))
            {
                try { form.ThrowIfFailed(); throw new Exception("failed form or entry did not throw"); }
                catch(FailedException error)
                {
                    Check(error.Rows.Count==1&&ReferenceEquals(error.Rows[0].Item,formOriginal),"one affected form retains the exact original record");
                    var expected=form.IsFailed ? Function.Annotate : scenario switch {"topic-failed"=>Function.Choose,"urgency-failed"=>Function.Score,_=>Function.Decide};
                    Check(error.Rows[0].Function==expected,"raise-if-failed finds the independently failed child");
                }
            }
            else Check(ReferenceEquals(form.ThrowIfFailed(),form),"answered and unsure-only forms return unchanged");
            if(form.IsAnswered)
            {
                var entries=form.Answer;
                Check(ReferenceEquals(entries.Decide("refund").Item,formOriginal)&&ReferenceEquals(entries.Choose<Topic>("topic").Item,formOriginal),"ordinary and enum-mapped child rows retain the form original");
                foreach(var entry in entries.Entries.Values)
                {
                    _=entry.Probability; _=entry.Detail;
                    Check(entry.Position==form.Position,"ordinary child rows retain their parent position");
                }
            }
            var choice=await First(client.ChooseStreamAsync("Topic?",new[]{"text"},Enum.GetValues<Topic>()));
            Check(!choice.Equals(Topic.Billing),"enum results do not compare as bare members");
            if(choice.IsAnswered) { Topic member=choice.Answer; Check(Enum.IsDefined(member),"enum choices return real members"); }
            if(scenario=="yes")
            {
                var named=await client.DecideAsync("Ready?","text",name:"execution");
                Check(named.Source.Name=="execution"&&first.Source.Name is null,"valid source names remain optional and preserve their value");
                foreach(string blank in new[]{"","   "})
                {
                    var row=await CallFailed(client.DecideAsync("Ready?",blank));
                    Check(row is Failed<bool,string> { Error.Code:"record.empty" }&&ReferenceEquals(row.Item,blank),"blank input is a failed row retaining its original");
                }
                ulong attemptsBefore=client.Usage.Attempts;
                var failedParent=await CallFailed(client.AnnotateAsync(questions,""));
                Check(failedParent.IsFailed,"a blank form record fails its parent");
                try { failedParent.ThrowIfFailed(); throw new Exception("failed form parent was ignored"); }
                catch(FailedException error) { Check(error.Rows.Count==1&&error.Rows[0].Function==Function.Annotate&&error.Rows[0].Position==failedParent.Position,"failed parents count as one affected form"); }
                Check(client.Usage.Attempts==attemptsBefore,"blank form parents make no backend request");
                Check(await CallFailed(client.DecideAsync<string?>("Ready?",null)) is Failed<bool,string?> { Error.Code:"record.missing" },"null input is a record failure");
                var nullInput=new Dictionary<string,object?>{{"id",null},{"text","text"}};
                var nullId=await CallFailed(client.DecideAsync("Ready?",nullInput));
                Check(nullId is IFailed {Error.Code:"record.invalid"}&&nullId.Id is null&&ReferenceEquals(nullId.Item,nullInput),"rejected identifiers leave no admitted ID and preserve the original input");
                using(var savedId=JsonDocument.Parse(JsonSerializer.Serialize(nullId))) Check(!savedId.RootElement.TryGetProperty("id",out _),"saved ordinary rows omit rejected identifiers");
                string text="é 🦀\nPlease refund my order.";
                retained=await client.FindAsync("Which line?",text);
                var span=retained.Answer!;
                Check(span.Start==5&&span.End==text.Length&&text[span.Start..span.End]==span.Text,"live spans slice UTF-16 strings");
                using(var json=JsonDocument.Parse(JsonSerializer.Serialize(retained))) Check(json.RootElement.GetProperty("answer").GetProperty("start").GetInt32()==4&&!json.RootElement.TryGetProperty("item",out _),"saved offsets count characters and omit originals");
                var record=new Dictionary<string,object>{{"action",text},{"names","Morgan 🦀 works for Example Group."}};
                var fields=new AnnotationQuestions
                {
                    {"action",FindQuestion.Create("Which line?",new(){Field=Pointer.From("/action")})},
                    {"names",RecognizeQuestion.Create(new[]{"person","organization"},new(){Field=Pointer.From("/names")})},
                };
                retainedForm=await client.AnnotateAsync(fields,record);
                var hash=retainedForm.GetHashCode(); record["action"]="changed"; record["names"]="changed";
                Check(retainedForm.GetHashCode()==hash&&retainedForm.Answer.Find("action").Answer!.Start==5,"detached form entries retain immutable native coordinates");
                using(var saved=JsonDocument.Parse(JsonSerializer.Serialize(retainedForm.Answer.Find("action")))) Check(saved.RootElement.GetProperty("answer").GetProperty("start").GetInt32()==4,"detached entries save native canonical coordinates");
                var metadataInput=new Dictionary<string,object>{{"id",new Dictionary<string,object>{{"a",1},{"b",2}}},{"text","text"}};
                var invalidId=await CallFailed(client.DecideAsync("Ready?",metadataInput));
                Check(invalidId is IFailed {Error.Code:"record.invalid"}&&invalidId.Id is null&&ReferenceEquals(invalidId.Item,metadataInput),"invalid ID objects fail their record without losing the original");
                var invalidRows=new HashSet<Row<bool,Dictionary<string,object>>>{invalidId};
                ((Dictionary<string,object>)metadataInput["id"])["changed"]=true;
                Check(invalidRows.Contains(invalidId),"mutating rejected input metadata cannot change a result hash");
                var unicodeNames=new AnnotationQuestions
                {
                    {"\U00010000",DecideQuestion.Create("Ready?",new(){Field=Pointer.From("/empty")})},
                    {"\uE000",DecideQuestion.Create("Ready?",new(){Field=Pointer.From("/missing")})},
                };
                var unicodeInput=new Dictionary<string,object>{{"text","present"},{"empty",""}};
                var unicodeForm=await client.AnnotateAsync(unicodeNames,unicodeInput);
                try { unicodeForm.ThrowIfFailed(); throw new Exception("two failed children were ignored"); }
                catch(FailedException error)
                { Check(error.Rows.Count==1&&error.Code=="record.missing"&&ReferenceEquals(error.Rows[0].Item,unicodeInput)&&error.Rows[0].Position==unicodeForm.Position,"one affected form chooses its first failure in native UTF-8 name order"); }
                Check(unicodeForm.ToString().StartsWith("\uE000=failed: record.missing:",StringComparison.Ordinal)&&unicodeForm.ToString().Contains("\t\U00010000=failed: record.empty:",StringComparison.Ordinal),"form printing uses the same UTF-8 question-name order");
                await ReentrantConverterClose();
                var poisonItem=new PoisonRecord();
                var poison=await client.DecideAsync("Ready?",poisonItem);
                var poisonCopy=await client.DecideAsync("Ready?",new PoisonRecord());
                var stored=new HashSet<Row<bool,PoisonRecord>>{poison};
                Check(ReferenceEquals(poison.Item,poisonItem)&&stored.Contains(poison)&&poison.Equals(poisonCopy)&&poison.ToString()=="true","result printing, equality and hashing never invoke methods on the original item");
                ModelFormatting();
            }
            if(scenario=="relations")
            {
                Check(choice.ToString()=="billing","enum choices print model-facing words");
                var tags=await client.TagAsync("Topics?","text",Enum.GetValues<Topic>());
                Check(tags.ToString()=="billing\tshipping\tother","tag prints labels separated by structural tabs");
                Check(score.ToString()=="1","score prints a command number");
                await foreach(var row in client.RankAsync(ranks,new[]{original})) Check(ReferenceEquals(row.Item,original)&&row.ToString()=="1","rank retains exact originals and prints its rank");
                Check(form.ToString()=="refund=true\ttopic=billing\turgency=1","forms print sorted named ordinary rows");
                string text="Morgan 🦀 works for Example Group.";
                var names=await client.RecognizeAsync(text,new[]{"person","organization"});
                Check(names.ToString()=="person: Morgan (0..6)\torganization: Example Group (19..32)","recognized name printing uses canonical characters");
                Check(names.Answer[1].Start==20&&text[names.Answer[1].Start..names.Answer[1].End]=="Example Group","recognized names expose UTF-16 slices");
                var relation=await client.RelateAsync(text,new[]{new RelationRule{Source="person",Target="organization",Relation="works_for"}});
                Check(relation.ToString()=="Morgan --works_for--> Example Group"&&relation.Answer[0].Target.Start==20,"relations print command links and expose host slices");
                using(var saved=JsonDocument.Parse(JsonSerializer.Serialize(relation))) Check(saved.RootElement.GetProperty("answer")[0].GetProperty("target").GetProperty("start").GetInt32()==19,"relation endpoints save canonical coordinates");
                var toolbox=Toolbox.Create(new Dictionary<string,ToolboxQuestion>{{"topic",new AnnotationQuestion.Choose(ChooseQuestion.Create("Topic?",new[]{"billing","shipping","other"}))}});
                Check((await client.ChooseAsync(toolbox.Choose<Topic>("topic"),"text")).Answer==Topic.Billing,"typed toolbox selectors decode enum members");
            }
        }
        if(retained is not null) Check(retained.Answer!.Start==5&&JsonSerializer.Serialize(retained).Contains("\"start\":4",StringComparison.Ordinal),"retained rows and serialization outlive their client");
        if(retainedForm is not null) Check(JsonSerializer.Serialize(retainedForm).Contains("\"start\":4",StringComparison.Ordinal),"nested native coordinate snapshots outlive their client");
        Console.WriteLine("C# ideal host contracts passed: "+scenario);
    }
    private static async Task<Row<A,T>> First<A,T>(IAsyncEnumerable<Row<A,T>> rows)
    {
        await foreach(var row in rows) return row;
        throw new Exception("stream returned no result");
    }
    private static async Task<Row<A,T>> CallFailed<A,T>(Task<Row<A,T>> task)
    {
        try { await task; }
        catch(FailedException error) { Check(error.Rows.Count==1,"scalar own failure payload"); return (Row<A,T>)error.Rows[0]; }
        throw new Exception("scalar failure did not fail the call");
    }
    private static async Task ScalarServiceFailure()
    {
        await using var client=new ThinkThenClient(new(){Retry=new RetryPolicy{MaxRetries=0}});
        var original=new Dictionary<string,object>{{"text","record"}};
        Task<Row<bool,Dictionary<string,object>>> pending=client.DecideAsync("Ready?",original);
        try { await pending; throw new Exception("service failure did not fault the scalar task"); }
        catch(FailedException error)
        {
            Check(error.Code=="backend.status"&&error.HttpStatus==503&&error.Rows.Count==1,"scalar service failure retains status, code and one row");
            var row=error.Rows[0];
            Check(row.Function==Function.Decide&&row.IsFailed&&ReferenceEquals(row.Item,original)&&error.Message==row.Error.Message,"scalar service failure retains its original record and message");
            Check(pending.IsFaulted&&ReferenceEquals(pending.Exception!.InnerException,error),"await preserves the original service exception");
            Check(client.Usage.ActiveCalls==0&&client.Usage.ActiveAttempts==0,"service failure settles its call and attempts before reaching the caller");
        }
    }
    private static async Task ScalarTiming()
    {
        await using var client=new ThinkThenClient(new(){Retry=new RetryPolicy{MaxRetries=0}});
        var questions=new AnnotationQuestions{{"ready",DecideQuestion.Create("Ready?")}};
        (Function Function,Func<string?,Task<IRow>> Call)[] calls=
        [
            (Function.Decide,async record=>await client.DecideAsync("Ready?",record)),
            (Function.Choose,async record=>await client.ChooseAsync("Which?",record,new[]{"a","b"})),
            (Function.Tag,async record=>await client.TagAsync("Tags?",record,new[]{"a","b"})),
            (Function.Score,async record=>await client.ScoreAsync("Scale?",record,new[]{"low","high"})),
            (Function.Find,async record=>await client.FindAsync("Action?",record)),
            (Function.Recognize,async record=>await client.RecognizeAsync(record,new[]{"person"})),
            (Function.Relate,async record=>await client.RelateAsync(record,new[]{new RelationRule{Source="person",Target="person",Relation="knows"}})),
            (Function.Annotate,async record=>await client.AnnotateAsync(questions,record)),
        ];
        ulong attempts=client.Usage.Attempts;
        foreach(var (function,call) in calls)
        {
            foreach(string? record in new string?[]{null,"","   "})
            {
                Task<IRow> pending=call(record);
                try { await pending; throw new Exception(function+": scalar failure did not fail the call"); }
                catch(FailedException error)
                {
                    Check(error is NoAnswerException&&error.Rows.Count==1,"scalar failure retains its no-answer ancestry and one row");
                    var row=error.Rows[0];
                    Check(row.Function==function&&row.IsFailed&&row.Position==1&&ReferenceEquals(row.Item,record),"scalar failure retains its own parent and original record");
                    Check(error.Code==(record is null ? "record.missing" : "record.empty")&&error.Code==row.Error.Code&&error.HttpStatus==row.Error.HttpStatus&&error.Message==row.Error.Message,"scalar failure preserves code and payload");
                    Check(pending.IsFaulted&&ReferenceEquals(pending.Exception!.InnerException,error),"await preserves the original task exception");
                }
                Check(client.Usage.ActiveCalls==0,"scalar failure disposes its async enumerator");
            }
        }
        var metadata=ScalarJson.Default.String;
        Func<Task<IRow>>[] metadataCalls=
        [
            async ()=>await client.DecideWithMetadataAsync("Ready?","",metadata),
            async ()=>await client.ChooseWithMetadataAsync("Which?","",new[]{"a","b"},metadata),
            async ()=>await client.TagWithMetadataAsync("Tags?","",new[]{"a","b"},metadata),
            async ()=>await client.ScoreWithMetadataAsync("Scale?","",new[]{"low","high"},metadata),
            async ()=>await client.FindWithMetadataAsync("Action?","",metadata),
            async ()=>await client.RecognizeWithMetadataAsync("",new[]{"person"},metadata),
            async ()=>await client.RelateWithMetadataAsync("",new[]{new RelationRule{Source="person",Target="person",Relation="knows"}},metadata),
            async ()=>await client.AnnotateWithMetadataAsync(questions,"",metadata),
        ];
        foreach(var call in metadataCalls)
        {
            try { await call(); throw new Exception("explicit metadata scalar failure returned"); }
            catch(FailedException error) { Check(error.Code=="record.empty"&&error.Rows.Count==1,"explicit metadata keeps the same failure boundary"); }
            Check(client.Usage.ActiveCalls==0,"metadata scalar failure disposes its call");
        }
        Check(typeof(ThinkThenClient).Assembly.GetType("ThinkThen.RecordException") is null,"removed exception has no alias");
        Check(client.Usage.Attempts==attempts,"blank and missing scalar records make no transport request");
        Check((await client.DecideAsync("Ready?","usable after failures")).IsAnswered&&client.Usage.ActiveCalls==0,"client remains usable without retained scalar calls");
        var partial=new AnnotationQuestions
        {
            {"refund",DecideQuestion.Create("Refund?",new(){Field=Pointer.From("/absent")})},
            {"topic",ChooseQuestion.Create("Topic?",new[]{"billing","shipping"},new(){Field=Pointer.From("/absent")})},
            {"urgency",ScoreQuestion.Create("Urgency?",new[]{"routine","urgent"})},
        };
        var form=await client.AnnotateAsync(partial,new Dictionary<string,object>{{"text","record"}});
        Check(form.IsAnswered&&form.Answer.Entries.Values.Count(row=>row.IsFailed)==2&&form.Answer.Score("urgency").IsAnswered,"an answered scalar form retains several failed children");
        try { form.ThrowIfFailed(); throw new Exception("partial form check ignored its failed children"); }
        catch(FailedException) { }
        Check(client.Usage.ActiveCalls==0,"returned partial form closes its call");
    }
    private static void ModelFormatting()
    {
        var source=new Source("fixture",Backend.Stub,"stub");
        var choice=ThinkThenModelFactory.Answered<string,string>(Function.Choose,"a\r\nb\tc",source,item:"original");
        Check(choice.ToString()=="a  b c"&&$"{choice}"=="a  b c"&&string.Format("{0}",choice)=="a  b c"&&"value="+choice=="value=a  b c","all result formatting folds only leaf controls");
        using(var saved=JsonDocument.Parse(JsonSerializer.Serialize(choice))) Check(saved.RootElement.GetProperty("answer").GetString()=="a\r\nb\tc","saving never folds bare answer text");
        var tags=ThinkThenModelFactory.Answered<ValueList<string>,string>(Function.Tag,["a\tb","c\nd"],source);
        Check(tags.ToString()=="a b\tc d","leaf folding preserves structural tag separators");
        var found=ThinkThenModelFactory.Answered<TextSpan?,string>(Function.Find,new("a\r\nb\tc",0,6),source);
        Check(found.ToString()=="a  b c","find prints a safely folded leaf");
        var small=ThinkThenModelFactory.Answered<double,string>(Function.Score,.0000001,source);
        var smallest=ThinkThenModelFactory.Answered<double,string>(Function.Score,double.Epsilon,source);
        Check(small.ToString()=="0.0000001"&&smallest.ToString()=="0."+new string('0',323)+"5","score printing matches the command's plain decimal representation");
        var phrase=ThinkThenModelFactory.Answered<Phrase,string>(Function.Choose,Phrase.LateDelivery,source);
        Check(phrase.ToString()=="late delivery","enum naming follows the shared lowercase-word policy");
        var failed=ThinkThenModelFactory.Failed<bool,string>(Function.Decide,new(ErrorCode.RecordMissing,"a\r\nb\tc"),source);
        Check(failed.ToString()=="failed: record.missing: a  b c","failure messages fold leaf controls safely");
    }
    private sealed class PoisonRecord
    {
        public string Body=>"text";
        public override bool Equals(object? other)=>throw new InvalidOperationException("Do not compare the original item.");
        public override int GetHashCode()=>throw new InvalidOperationException("Do not hash the original item.");
        public override string ToString()=>throw new InvalidOperationException("Do not format the original item.");
    }
    private sealed class CloseRecord { }
    private sealed class ClosingConverter : JsonConverter<CloseRecord>
    {
        internal ThinkThenClient? Client;
        public override CloseRecord? Read(ref Utf8JsonReader reader,Type type,JsonSerializerOptions options)=>throw new NotSupportedException();
        public override void Write(Utf8JsonWriter writer,CloseRecord value,JsonSerializerOptions options)
        { Client!.Dispose(); writer.WriteStartObject(); writer.WriteBoolean("closed",true); writer.WriteEndObject(); }
    }
    private static async Task ReentrantConverterClose()
    {
        var converter=new ClosingConverter(); var json=new JsonSerializerOptions(); json.Converters.Add(converter);
        await using var client=new ThinkThenClient(new(){Json=json}); converter.Client=client;
        try { await client.DecideAsync("Ready?",new CloseRecord()); throw new Exception("reentrant close did not cancel the active call"); }
        catch(OperationCanceledException) { }
    }
}

[JsonSerializable(typeof(string))]
internal partial class ScalarJson : JsonSerializerContext { }
