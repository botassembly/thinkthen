#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <thinkthen.h>

thinkthen_engine *tt = thinkthen_engine_new();

const char *rules[] = {
    "Book economy class for every flight under six hours.",
    "Submit receipts within 30 days of the trip.",
    "Hotel stays are capped at 200 dollars a night.",
    "Employees may book business class on any flight.",
    "Rental cars need a manager's approval.",
    "Receipts may be submitted at any time, "
        "with no deadline.",
    "Meals are reimbursed up to 60 dollars a day.",
    "Use the company travel portal for all bookings.",
};
char records[8][96];
const char *texts[8];
size_t lengths[8];
for (int i = 0; i < 8; i++) {
    snprintf(
        records[i],
        sizeof records[i],
        "{\"name\": \"%s\", \"kind\": \"rule\"}",
        rules[i]
    );
    texts[i] = records[i];
    lengths[i] = strlen(records[i]);
}
const char *spec =
    "{\"version\": 1, \"relate\": {\"relations\": [{"
    "\"name\": \"contradicts\", \"source\": \"*\", "
    "\"target\": \"*\", \"either\": true}]}, "
    "\"threshold\": 0.5}";
char *edges;
size_t edges_len;
int rc = thinkthen_relate(
    tt,
    spec,
    texts,
    lengths,
    8,
    &edges,
    &edges_len
);
assert(rc == THINKTHEN_OK);
assert(strstr(edges, "\"probability\":0.84"));
assert(strstr(edges, "\"probability\":0.99"));
thinkthen_free_string(edges);
