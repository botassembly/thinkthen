using System.Diagnostics;
using System.Text.Json;
using ThinkThen;
using ThinkThen.Inputs;
using ThinkThen.Results;

using var engine = Engine.Open();
var question = "Which team owns this?";
var teams = new Dictionary<string, string>
{
    ["billing"] = "Invoices, fees, and refunds.",
    ["shipping"] = "Parcels and delivery.",
    ["account"] = "Logins and passwords.",
};
var parcel = "My parcel went to the wrong address.";
var teamQuestion = engine.ParseQuestion(
    AuthoredQuestionKind.Atomic,
    JsonSerializer.Serialize(
        new { choose = question, options = teams }));
var call = await engine.ChooseAsync(
    teamQuestion,
    new InputRequestInputText { Text = parcel });
var team = call.Choice();
Trace.Assert(team == "shipping");
