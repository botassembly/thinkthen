#include <thinkthen/client.hpp>
#include <iostream>
using namespace tt;
using namespace tt::inputs;
int main() {
    try {
        Client client;
        const auto schema=::tt::detail::parse_original(R"({"type":"object","properties":{"enabled":{"type":"boolean"},"name":{"type":"string"},"count":{"type":"number"},"labels":{"type":"array","items":{"type":"string"}}},"required":["enabled","name","count","labels"]})");
        auto definition=::tt::results::detail::Loader::input<RequestDefinition>(Data{{"decide","Is it?"},{"item_schema",schema}});
        for(bool enabled:{false,true}) {
            Data original{{"enabled",enabled},{"name","one"},{"count",1.5},{"labels",Data::Array{"a","b"}}};
            auto item=RequestItem().set_original(RequestOriginalJson().set_value(original));
            auto call=client.decide(RequestQuestionDefinition().set_value(definition),RequestInputRecords().set_items({item}));
            call.finish(); bool answered=false;
            for(const auto& packet:call.collect()) if(auto row=packet.as_SessionPacketDecideRow()) {
                const auto& value=*row->value().value;
                if(::tt::results::detail::document(value).at("input")!=original) return 1;
                if(!value.value().value->value().get<bool>()) return 2;
                auto question=value.question().value->as_QuestionDocumentDecide();
                auto declaration=question->item_schema().value->as_InputDeclarationObjectRoot();
                if(!declaration || ::tt::results::detail::document(*declaration)!=schema) return 3;
                const auto& properties=*declaration->properties().value;
                if(properties.size()!=4) return 4;
                for(const auto& property:properties) {
                    const auto& name=property.first; const auto& type=property.second;
                    if(name=="enabled" && (!type.as_InputPropertyTypeBoolean() || *type.as_InputPropertyTypeBoolean()->type().value!="boolean")) return 5;
                    if(name=="name" && (!type.as_InputPropertyTypeString() || *type.as_InputPropertyTypeString()->type().value!="string")) return 6;
                    if(name=="count" && (!type.as_InputPropertyTypeNumber() || *type.as_InputPropertyTypeNumber()->type().value!="number")) return 7;
                    if(name=="labels" && (!type.as_InputPropertyTypeArray() || *type.as_InputPropertyTypeArray()->type().value!="array")) return 8;
                }
                answered=true;
            }
            if(!answered) return 9;
        }
        for(bool expected:{false,true}) {
            auto call=client.decide(RequestQuestionText().set_text("Is it?"),RequestInputText().set_text(expected ? "yes":"no"));
            call.finish(); bool answered=false;
            for(const auto& packet:call.collect()) if(auto row=packet.as_SessionPacketDecideRow()) {
                if(row->value().value->value().value->value().get<bool>()!=expected) return 10;
                answered=true;
            }
            if(!answered) return 11;
        }
        std::cout<<"CPP_INSTALLED_SCHEMA_PROJECTION_PASS boolean string number array scalar_false_true\n";
    } catch(const std::exception& error) { std::cerr<<error.what()<<'\n'; return 12; }
}
