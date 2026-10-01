#include <assert.h>
#include <stdlib.h>
#include <string.h>
#include "ThinkThen.h"
#include "TTJSON.h"

int main(void) {
    TTClient *client = [TTClient create];
    TTFailure failure = {0};

    const char *kinds =
        "{\"version\": 1, \"recognize\": {"
        "\"kinds\": {\"person\": null, "
        "\"organization\": null, \"place\": null}}}";
    const char *text =
        "Maria Chen joined Northwind Freight, "
        "a company in Chicago.";
    char *names = NULL;
    size_t names_length = 0;
    char *facts = NULL;
    TTErrorKind names_error =
        [client recognize:kinds
                     text:text
                   length:strlen(text)
                   result:&names
                     size:&names_length
                 deadline:-1
                    token:nil
                    facts:&facts
                  failure:&failure];
    assert(names_error == TTErrorNone);
    TTJSON *found = tt_json_parse(names, names_length);
    const TTJSON *entities = tt_json_get(found, "entities");
    const char *spans[] = {
        "Maria Chen", "Northwind Freight", "Chicago",
    };
    const char *labels[] = {
        "person", "organization", "place",
    };
    assert(entities->count == 3);
    for (size_t i = 0; i < 3; i++) {
        const TTJSON *one = entities->children[i];
        const TTJSON *name = tt_json_get(one, "text");
        const TTJSON *kind = tt_json_get(one, "kind");
        assert(strcmp(name->text, spans[i]) == 0);
        assert(strcmp(kind->text, labels[i]) == 0);
    }
    tt_json_free(found);
    free(names);
    free(facts);

    tt_failure_clear(&failure);
    [client dealloc];
    return 0;
}
