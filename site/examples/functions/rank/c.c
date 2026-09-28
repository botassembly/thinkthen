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
char *rank_call = thinkthen_call(tt, rank);
assert(rank_call);
assert(strncmp(rank_call, "{\"value\":", 9) == 0);
const char *by_urgency = rank_call + 9;
size_t urgency_len = strlen(expected);
assert(strncmp(by_urgency, expected, urgency_len) == 0);
assert(strncmp(
    by_urgency + urgency_len, ",\"facts\":{", 10
) == 0);
thinkthen_free_string(rank_call);
