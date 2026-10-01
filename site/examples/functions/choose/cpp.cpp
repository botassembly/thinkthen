#include <iostream>
#include <string>
#include <vector>
#include <thinkthen/door.hpp>

int main() {
    auto engine = tt::create();

    const char *question = "Which team owns this?";
    tt::Json teams = {
        {"billing", "Invoices, fees, and refunds."},
        {"shipping", "Parcels and delivery."},
        {"account", "Logins and passwords."},
    };
    std::vector<std::string> texts = {
        "Please refund the extra fee on my invoice.",
        "My parcel went to the wrong address.",
        "I cannot reset my password.",
        "My parcel never came, and now "
        "I cannot log in to track it.",
    };
    for (const auto &text : texts) {
        auto owner = tt::call(engine, {
            {"choose", question},
            {"options", teams},
            {"threshold", 0.9},
            {"evidence", text},
        }).at("value");
        std::cout << (owner.is_null() ? "null"
            : owner.get<std::string>()) << "\n";
    }
}
