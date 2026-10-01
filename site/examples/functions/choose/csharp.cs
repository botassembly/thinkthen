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
const string text =
    "Please refund the extra fee on my invoice.";
var request = JsonSerializer.Serialize(new
{
    choose = question,
    options = teams,
    evidence = text,
});
var owner = tt.CallTyped(request).Value;
Trace.Assert(owner.GetString() == "billing");
