using System.Text.Json;
using ThinkThen;

internal static partial class Program
{
    private static void InterfaceCases()
    {
        var source=new Source("fixture",Backend.Stub,"stub");
        var yes=ThinkThenModelFactory.Answered<bool,string>(Function.Decide,true,source,probability:.8,item:"first");
        var equal=ThinkThenModelFactory.Answered<bool,string>(Function.Decide,true,source,probability:.8,item:"other");
        Check(yes.Answer&&yes.Equals(equal)&&yes.GetHashCode()==equal.GetHashCode(),"row equality excludes original items");
        Check(yes ? true : false,"decide-only conditional operators read the answer");
        Check(!yes==false&&yes.ToString()=="true"&&!yes.Equals(true),"decide formatting and value boundaries");
        var unsure=ThinkThenModelFactory.Unsure<bool,string>(Function.Decide,.5,source);
        try { _=unsure.Answer; throw new Exception("unsure returned a default"); }
        catch(NoAnswerException error) { Check(error is UnsureException value&&ReferenceEquals(value.Row,unsure),"uncertainty retains its row"); }
        var failed=ThinkThenModelFactory.Failed<bool,string>(Function.Decide,new(ErrorCode.RecordMissing,"missing"),source);
        try { _=failed.Answer; throw new Exception("failure returned a default"); }
        catch(NoAnswerException error) { Check(error is FailedException value&&value.Code=="record.missing"&&ReferenceEquals(value.Rows[0],failed),"record failure retains its row"); }
        var labels=ValueMap<string,double>.Empty.Add("billing",.7).Add("refund",.6);
        var reversed=ValueMap<string,double>.Empty.Add("refund",.6).Add("billing",.7);
        var first=ThinkThenModelFactory.Answered<ValueList<string>,string>(Function.Tag,["billing","refund"],source,detail:new LabelsDetail(labels));
        var second=ThinkThenModelFactory.Answered<ValueList<string>,string>(Function.Tag,["billing","refund"],source,detail:new LabelsDetail(reversed));
        Check(first.Equals(second)&&new HashSet<Row<ValueList<string>,string>> {first,second}.Count==1,"immutable nested maps and lists compare structurally");
        Check(first.Detail!.Equals(second.Detail)&&first.Answer.Equals(second.Answer),"public answer and detail values agree");
        using(var serialized=JsonDocument.Parse(JsonSerializer.Serialize(yes))) Check(serialized.RootElement.GetProperty("answer").GetBoolean(),"model rows use the shared saved-row shape");
        static Task<Row<bool,string>> NullOptions(ThinkThenClient client)=>client.DecideAsync("Ready?","alpha",null);
        _=(Func<ThinkThenClient,Task<Row<bool,string>>>)NullOptions;
        try { DecideQuestion.Create(null!); throw new Exception("null question was accepted"); }
        catch(ArgumentNullException error) { Check(error.ParamName=="question","null arguments identify their parameter"); }
        Check(typeof(UnsureException).BaseType==typeof(NoAnswerException)&&typeof(FailedException).BaseType==typeof(NoAnswerException),"no-answer errors share one parent");
        Check(!typeof(IDisposable).IsAssignableFrom(typeof(DecideQuestion))&&!typeof(IDisposable).IsAssignableFrom(typeof(Toolbox)),"questions and toolboxes are plain values");
        Check(typeof(ThinkThenClient).GetMethods().Where(method=>method.Name.EndsWith("Async",StringComparison.Ordinal)&&method.Name!="DisposeAsync")
            .All(method=>method.GetParameters().Last().ParameterType==typeof(CancellationToken)),"async calls keep the final cancellation token");
        foreach(var method in typeof(ThinkThenClient).GetMethods().Where(method=>method.DeclaringType==typeof(ThinkThenClient)))
        {
            Check(method.GetGenericArguments().All(type=>type.Name is not ("A" or "D")),"public generic names explain their role");
            if(method.GetParameters().Any(parameter=>parameter.ParameterType.IsGenericType&&parameter.ParameterType.GetGenericTypeDefinition()==typeof(System.Text.Json.Serialization.Metadata.JsonTypeInfo<>)))
                Check(method.Name.Contains("WithMetadata",StringComparison.Ordinal),"metadata overloads have distinct names");
        }
        var recognition=new RecognizeDetail([new(new("Ada",0,3),ValueMap<BoundaryTag,double>.Empty.Add(BoundaryTag.Single,.9))],[new("Ada",0,3,ValueMap<string,double>.Empty.Add("person",.9))]);
        var entity=new Entity(new(new("Ada",0,3),"person"),.9);
        var names=ThinkThenModelFactory.Answered<ValueList<Entity>,string>(Function.Recognize,[entity],source,detail:recognition);
        var copy=ThinkThenModelFactory.Answered<ValueList<Entity>,string>(Function.Recognize,[new(new(new("Ada",0,3),"person"),.9)],source,
            detail:new RecognizeDetail([new(new("Ada",0,3),ValueMap<BoundaryTag,double>.Empty.Add(BoundaryTag.Single,.9))],[new("Ada",0,3,ValueMap<string,double>.Empty.Add("person",.9))]));
        Check(names==copy&&names.GetHashCode()==copy.GetHashCode(),"recognition values compare by their own fields");
        var caused=ThinkThenModelFactory.Failed<bool,string>(Function.Decide,new(ErrorCode.RecordMissing,"missing"){Cause=new InvalidOperationException("host")},source);
        Check(failed.Equals(caused)&&failed.GetHashCode()==caused.GetHashCode(),"exception causes do not participate in row values");
        Check(!yes.Equals(ThinkThenModelFactory.Answered<bool,string>(Function.Decide,false,source,probability:.8)),"answers participate in equality");
        Check(!yes.Equals(ThinkThenModelFactory.Answered<bool,string>(Function.Decide,true,source,position:2,probability:.8)),"positions participate in equality");
        Console.WriteLine("C# independent public interface checks passed.");
    }
}
