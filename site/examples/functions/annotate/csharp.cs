using System.Diagnostics;
using ThinkThen;
using ThinkThen.Inputs;
using ThinkThen.Results;

using var engine = Engine.Open();
var report = "Steps: click Log in. Nobody gets in.";
var records = new InputRequestInputRecords
{
    Items =
    [
        new InputRequestItem
        {
            Original = new InputRequestOriginalText
            {
                Text = report,
            },
        },
    ],
};
var call = await engine.AnnotateAsync(
    new InputRequestQuestionFile { Path = "form.json" },
    records);
var triage = call.Annotate()[0].Value;
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
