using System.Diagnostics;
using ThinkThen;
using ThinkThen.Inputs;
using ThinkThen.Results;

using var engine = Engine.Open();
var teamQuestion = new RequestQuestionFile
{
    Path = "teams.json",
};
var parcel = "My parcel went to the wrong address.";
var call = await engine.ChooseAsync(
    teamQuestion,
    new RequestInputText { Text = parcel });
var team = call.Choice();
Trace.Assert(team == "shipping");
