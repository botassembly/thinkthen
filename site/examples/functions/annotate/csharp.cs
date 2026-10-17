using System.Diagnostics;
using ThinkThen;
using ThinkThen.Inputs;
using ThinkThen.Results;

using var engine = Engine.Open();
var report = "Steps: click Log in. Nobody gets in.";
var records = new RequestInputRecords
{
    Items =
    [
        new RequestItem
        {
            Original = new RequestOriginalText
            {
                Text = report,
            },
        },
    ],
};
var call = await engine.AnnotateAsync(
    new RequestQuestionFile { Path = "form.json" },
    records);
var triage = call.Value[0];
Trace.Assert(triage["steps"] is AnnotatedFieldBoolean
{
    Value: true,
});
Trace.Assert(triage["area"] is AnnotatedFieldString
{
    Value: "login",
});
Trace.Assert(triage["impact"] is AnnotatedFieldNumber
{
    Value: 1.98,
});
