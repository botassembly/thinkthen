#include <cassert>
#include <thinkthen/door.hpp>

int main() {
    auto engine = tt::create();

    const char *question = "Which team owns this?";
    tt::Json teams = {
        {"billing", "Invoices, fees, and refunds."},
        {"shipping", "Parcels and delivery."},
        {"account", "Logins and passwords."},
    };
    const char *text =
        "Please refund the extra fee on my invoice.";
    auto owner = tt::call(engine, {
        {"choose", question},
        {"options", teams},
        {"evidence", text},
    }).at("value");
    assert(owner == "billing");
}
