using System.Diagnostics;
using System.Text.Json;
using ThinkThen;

using var tt = Engine.Open();
const string question = "Which team owns this?";
var teams = new Dictionary<string, string>
{
    ["billing"] = "Invoices, fees, and refunds.",
    ["shipping"] = "Parcels and delivery.",
    ["account"] = "Logins and passwords.",
};
const string parcel =
    "My parcel went to the wrong address.";
var request = JsonSerializer.Serialize(new
{
    choose = question,
    options = teams,
    evidence = parcel,
});
var team = tt.CallTyped(request).Value;
Trace.Assert(team.GetString() == "shipping");
