#include <cassert>
#include <thinkthen/client.hpp>

using namespace tt::inputs;

int main() {
    tt::Client client;
    auto form = RequestQuestionFile().set_path("form.json");
    auto report = RequestInputText().set_text(
        "Steps: click Log in. Nobody gets in.");
    auto call = client.annotate(form, report);
    auto annotations = call.answer();
    auto triage =
        annotations.at(0).value().value->document();
    tt::Json expected{
        {"steps", true},
        {"area", "login"},
        {"impact", 1.98},
    };
    assert(triage == expected);
}
