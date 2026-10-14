#include <cassert>
#include <thinkthen/client.hpp>

using namespace tt::inputs;

int main() {
    tt::Client client;
    auto form = RequestQuestionFile().set_path("form.json");
    auto report = RequestInputText().set_text(
        "Steps: click Log in. Nobody gets in.");
    auto call = client.annotate(form, report);
    auto annotations = call.result().value;
    bool steps = false;
    std::string area;
    double impact = 0;
    for (const auto& [name, field] : annotations) {
        if (name == "steps") {
            steps = std::get<bool>(field.value());
        } else if (name == "area") {
            area = std::get<std::string>(field.value());
        } else if (name == "impact") {
            impact = std::get<double>(field.value());
        }
    }
    assert(steps && area == "login" && impact == 1.98);
}
