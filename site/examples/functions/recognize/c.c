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
char *found;
size_t found_len;
int rc = thinkthen_recognize(
    tt,
    spec,
    text,
    strlen(text),
    &found,
    &found_len
);
assert(rc == THINKTHEN_OK);
assert(strstr(found, "\"Maria Chen\""));
assert(strstr(found, "\"Northwind Freight\""));
assert(strstr(found, "\"Chicago\""));
assert(strstr(found, "\"works_for\""));
assert(strstr(found, "\"based_in\""));
thinkthen_free_string(found);
