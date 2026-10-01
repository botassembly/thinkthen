#include <assert.h>
#include <json-c/json.h>
#include <string.h>
#include <thinkthen.h>

thinkthen_engine *tt = thinkthen_engine_new();
assert(tt);

const char *names[] = {
    "{\"name\": \"Paul McCartney\", \"kind\": \"singer\"}",
    "{\"name\": \"Ringo Starr\", \"kind\": \"singer\"}",
    "{\"name\": \"Yesterday\", \"kind\": \"song\"}",
    "{\"name\": \"Octopus's Garden\", \"kind\": \"song\"}",
};
size_t lengths[4];
for (int i = 0; i < 4; i++) lengths[i] = strlen(names[i]);
const char *sings =
    "{\"version\": 1, \"relate\": {\"relations\": [{"
    "\"name\": \"sings\", \"source\": \"singer\", "
    "\"target\": \"song\"}]}}";
char *who_sings;
size_t who_sings_len;
int rc = thinkthen_relate(
    tt,
    sings,
    names,
    lengths,
    4,
    &who_sings,
    &who_sings_len
);
assert(rc == THINKTHEN_OK);
const char *expected =
    "{\"edges\": ["
    "{\"relation\": \"sings\", "
    "\"source\": {\"name\": \"Paul McCartney\", "
    "\"kind\": \"singer\"}, "
    "\"target\": {\"name\": \"Yesterday\", "
    "\"kind\": \"song\"}, \"probability\": 0.81}, "
    "{\"relation\": \"sings\", "
    "\"source\": {\"name\": \"Ringo Starr\", "
    "\"kind\": \"singer\"}, "
    "\"target\": {\"name\": \"Octopus's Garden\", "
    "\"kind\": \"song\"}, \"probability\": 0.88}]}";
struct json_object *edges = json_tokener_parse(who_sings);
struct json_object *wanted = json_tokener_parse(expected);
assert(edges && wanted);
json_bool same = json_object_equal(edges, wanted);
assert(same);
json_object_put(wanted);
json_object_put(edges);
thinkthen_free_string(who_sings);
thinkthen_engine_free(tt);
