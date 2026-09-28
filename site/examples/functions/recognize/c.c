#include <assert.h>
#include <string.h>
#include <thinkthen.h>

thinkthen_engine *tt = thinkthen_engine_new();

const char *text =
    "Maria Chen joined Northwind Freight, "
    "a company in Chicago.";
const char *spec =
    "{\"version\": 1, \"recognize\": {"
    "\"kinds\": {\"person\": null, "
    "\"organization\": null, \"place\": null}, "
    "\"relations\": ["
    "{\"name\": \"works_for\", \"source\": \"person\", "
    "\"target\": \"organization\"}, "
    "{\"name\": \"based_in\", "
    "\"source\": \"organization\", "
    "\"target\": \"place\"}]}}";
char *facts;
size_t facts_len;
int rc = thinkthen_recognize(
    tt,
    spec,
    text,
    strlen(text),
    &facts,
    &facts_len
);
assert(rc == THINKTHEN_OK);
assert(strstr(facts, "\"Maria Chen\""));
assert(strstr(facts, "\"Northwind Freight\""));
assert(strstr(facts, "\"Chicago\""));
assert(strstr(facts, "\"works_for\""));
assert(strstr(facts, "\"based_in\""));
thinkthen_free_string(facts);
