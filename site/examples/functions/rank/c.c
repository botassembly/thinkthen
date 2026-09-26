#include <assert.h>
#include <string.h>
#include <thinkthen.h>

thinkthen_engine *tt = thinkthen_engine_new();

const char *rank =
    "{\"rank\": \"Is this urgent?\", "
    "\"records\": ["
    "\"Newsletter: our autumn catalog is here. "
    "No reply needed.\", "
    "\"Our checkout page is down and customers "
    "cannot pay\", "
    "\"Reminder: your invoice is due in 30 days\", "
    "\"Please send the signed quote by 5 pm today\"]}";
const char *expected =
    "[\"Our checkout page is down and customers "
    "cannot pay\","
    "\"Please send the signed quote by 5 pm today\","
    "\"Reminder: your invoice is due in 30 days\","
    "\"Newsletter: our autumn catalog is here. "
    "No reply needed.\"]";
char *ranked = thinkthen_call(tt, rank);
assert(ranked && strcmp(ranked, expected) == 0);
thinkthen_free_string(ranked);
