#include <assert.h>
#include <string.h>
#include <thinkthen.h>

thinkthen_engine *tt = thinkthen_engine_new();
assert(tt);

const char *kinds =
    "{\"version\": 1, \"recognize\": {"
    "\"kinds\": {\"person\": null, "
    "\"organization\": null, \"place\": null}}}";
const char *text =
    "Maria Chen joined Northwind Freight, "
    "a company in Chicago.";
char *facts;
size_t facts_len;
int rc = thinkthen_recognize(
    tt,
    kinds,
    text,
    strlen(text),
    &facts,
    &facts_len
);
assert(rc == THINKTHEN_OK);
const char *person = strstr(facts,
    "\"text\":\"Maria Chen\"");
const char *organization = strstr(facts,
    "\"text\":\"Northwind Freight\"");
const char *place = strstr(facts,
    "\"text\":\"Chicago\"");
assert(person && organization && place);
thinkthen_free_string(facts);
thinkthen_engine_free(tt);
