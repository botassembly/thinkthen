/*
 * The C sections of `recognize-surfaces.md`, run as drawn against the
 * stand-in: one `thinkthen_recognize` over the Maria Chen sentence, the
 * offset proof on the emoji text, and one `thinkthen_relate` over the four
 * alerts. Every answer comes from a recording.
 *
 * The deck's snippet frees with `thinkthen_string_free`; the header and
 * this library name that function `thinkthen_free_string`, and this file
 * calls the real name (pinned in NOTES.md, finding 4).
 *
 * The recognize section's call, as drawn:
 *
 *     char *out; size_t out_len;
 *     int rc = thinkthen_recognize(tt, spec_json, text, text_len, &out, &out_len);
 *     thinkthen_free_string(out);
 *
 * The relate section's call, as drawn:
 *
 *     int rc = thinkthen_relate(tt, spec_json, texts, lens, COUNT, &out, &out_len);
 *
 * Exit 0 means every expected answer held.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <thinkthen.h>

/* The recordings' own texts. The sentence's answer: "Maria Chen" person
 * 0..10 strength 0.98, "Northwind Freight" organization 18..35, "Chicago"
 * place 39..46, and works_for 1 -> 2 at probability 1.0. */
static const char SENTENCE[] =
    "Maria Chen joined Northwind Freight in Chicago last spring.";
/* The offset case: one accented letter and one emoji before the name. Its
 * answer: "Maria Chen" person at code points 10..20, strength 0.94. */
static const char EMOJI[] = "Le café 😀 Maria Chen arrived.";

/* The four alerts, recorded as one text; the method-H relate answer at the
 * 0.7 bar: caused_by 1 -> 4 at 0.71 and caused_by 2 -> 4 at 0.73. */
static const char *ALERTS[4] = {
    "Alert 1: Checkout returns 500 at the payment step.",
    "Alert 2: Card charges are failing for every customer.",
    "Alert 3: The nightly export ran two hours late.",
    "Alert 4: The payments database ran out of disk space.",
};

/* `spec_json` is the recognize section of the question file. The
 * recording for the sentence covers `works_for` and `based_in`; the deck's
 * own rule set also asks `located_in`, which no recording holds. */
static const char SPEC_RECORDED[] =
    "{\"kinds\": [\"person\", \"organization\", \"place\"],"
    " \"relations\": [{\"name\": \"works_for\", \"source\": \"person\","
    " \"target\": \"organization\"}]}";
static const char SPEC_DECK[] =
    "{\"kinds\": [\"person\", \"organization\", \"place\"],"
    " \"relations\": [{\"name\": \"works_for\", \"source\": \"person\","
    " \"target\": \"organization\"},"
    " {\"name\": \"located_in\", \"source\": \"*\", \"target\": \"place\"}]}";
static const char SPEC_EMOJI[] = "{\"kinds\": [\"person\"]}";
static const char SPEC_RELATE[] =
    "{\"relations\": [{\"name\": \"caused_by\", \"source\": \"*\", \"target\": \"*\"}],"
    " \"either\": [\"same_as\"], \"threshold\": 0.7}";

static int contains(const char *haystack, const char *needle) {
    return strstr(haystack, needle) != NULL;
}

/* The first entity's start and end, read the way a C host without a JSON
 * library does at its simplest: find the keys, parse the numbers. */
static int first_offsets(const char *json, unsigned long *start,
                         unsigned long *end) {
    const char *at = strstr(json, "\"start\":");
    if (at == NULL) {
        return 0;
    }
    *start = strtoul(at + 8, NULL, 10);
    at = strstr(json, "\"end\":");
    if (at == NULL) {
        return 0;
    }
    *end = strtoul(at + 6, NULL, 10);
    return 1;
}

/* C's own indexing: one conversion from the answer's code points to byte
 * offsets. The emoji is four bytes and one code point, so the units part
 * company after it. */
static void byte_range(const char *text, unsigned long start,
                       unsigned long end, unsigned long *from,
                       unsigned long *to) {
    unsigned long points = 0;
    unsigned long byte = 0;
    *from = 0;
    *to = strlen(text);
    while (text[byte] != '\0') {
        if (points == start) {
            *from = byte;
        }
        if (points == end) {
            *to = byte;
            return;
        }
        unsigned char lead = (unsigned char)text[byte];
        byte += lead < 0x80 ? 1 : lead < 0xE0 ? 2 : lead < 0xF0 ? 3 : 4;
        points += 1;
    }
}

int main(void) {
    thinkthen_engine *tt = thinkthen_engine_new();
    if (tt == NULL) {
        fprintf(stderr, "no engine\n");
        return 1;
    }
    char *out;
    size_t out_len;
    int ok = 1;

    /* The recording's rules: the answer holds. */
    const char *text = SENTENCE;
    size_t text_len = strlen(text);
    int rc = thinkthen_recognize(tt, SPEC_RECORDED, text, text_len, &out,
                                 &out_len);
    if (rc != 0) {
        fprintf(stderr, "recognize rc=%d: %s\n", rc, thinkthen_error_message(tt));
        thinkthen_engine_free(tt);
        return 1;
    }
    printf("recognize: %.*s\n", (int)out_len, out);
    ok &= contains(out, "\"text\":\"Maria Chen\"");
    ok &= contains(out, "\"kind\":\"person\"");
    ok &= contains(out, "\"name\":\"works_for\"");
    ok &= contains(out, "\"strength\":0.98");
    ok &= contains(out, "\"source\":1");
    ok &= contains(out, "\"target\":2");
    ok &= !contains(out, "\"confidence\"");
    ok &= !contains(out, "\"from\"");
    thinkthen_free_string(out);

    /* The deck's own rules: located_in has no recording, so the call
     * refuses and names what the recording covers. Pinned, not bent. */
    rc = thinkthen_recognize(tt, SPEC_DECK, text, text_len, &out, &out_len);
    if (rc != 0) {
        printf("recognize (the deck's located_in rule): refused, %s\n",
               thinkthen_error_message(tt));
        ok &= contains(thinkthen_error_message(tt), "located_in");
        ok &= contains(thinkthen_error_message(tt), "works_for, based_in");
    } else {
        printf("recognize (the deck's located_in rule): answered, unexpected\n");
        ok = 0;
        thinkthen_free_string(out);
    }

    /* The offset proof: the answer's code points to C's byte offsets. */
    rc = thinkthen_recognize(tt, SPEC_EMOJI, EMOJI, strlen(EMOJI), &out,
                             &out_len);
    if (rc != 0) {
        fprintf(stderr, "recognize (emoji) rc=%d: %s\n", rc,
                thinkthen_error_message(tt));
        thinkthen_engine_free(tt);
        return 1;
    }
    unsigned long start;
    unsigned long end;
    unsigned long from;
    unsigned long to;
    if (!first_offsets(out, &start, &end)) {
        fprintf(stderr, "no offsets in: %.*s\n", (int)out_len, out);
        thinkthen_engine_free(tt);
        return 1;
    }
    byte_range(EMOJI, start, end, &from, &to);
    printf("offsets: code points %lu..%lu -> bytes %lu..%lu -> \"%.*s\"\n",
           start, end, from, to, (int)(to - from), EMOJI + from);
    ok &= start == 10 && end == 20;
    ok &= from == 14 && to == 24;
    ok &= (to - from) == 10 && memcmp(EMOJI + from, "Maria Chen", 10) == 0;
    thinkthen_free_string(out);

    /* The relate section, as drawn. */
    size_t lens[4];
    for (int at = 0; at < 4; at += 1) {
        lens[at] = strlen(ALERTS[at]);
    }
    rc = thinkthen_relate(tt, SPEC_RELATE, ALERTS, lens, 4, &out, &out_len);
    if (rc != 0) {
        fprintf(stderr, "relate rc=%d: %s\n", rc, thinkthen_error_message(tt));
        thinkthen_engine_free(tt);
        return 1;
    }
    printf("relate: %.*s\n", (int)out_len, out);
    ok &= contains(out, "\"name\":\"caused_by\"");
    ok &= contains(out, "\"probability\":0.71");
    ok &= !contains(out, "\"probability\":0.55");
    ok &= !contains(out, "\"from\"");
    thinkthen_free_string(out);

    thinkthen_engine_free(tt);
    if (!ok) {
        fprintf(stderr, "recognize/relate: a check failed\n");
        return 1;
    }
    printf("recognize and relate ok\n");
    return 0;
}
