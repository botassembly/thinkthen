# C functions that answer questions about text

ThinkThen is a classifier you call like a function. It reads text, answers one bounded question, and hands back a result you can branch on.

Use a C11 compiler and pkg-config. Download the C archive for your platform from the release page and extract it into a folder named `thinkthen`. The archive includes the engine. Compile with `cc -std=c11 -Wall -Wextra -Werror example.c -o example $(PKG_CONFIG_PATH="$PWD/thinkthen/lib/pkgconfig" pkg-config --cflags --libs thinkthen-c) -Wl,-rpath,"$PWD/thinkthen/lib"`. The archive's README has the Windows steps. Set `THINKTHEN_API_KEY` in the environment first.

A one-record `thinkthen_run_one` call returns a negative error status when its record could not be judged. `THINKTHEN_OK` means it returned an answered or not-sure row, never a failed row. Many-record calls keep failed rows, so the shared row type has three states:

- **answered**: there is an answer, and an answer read such as `thinkthen_row_answer_bool(&result, &yes, &error)` gives it.
- **not sure**: the model's number fell between the two bars you set, or no choice was likely enough.
- **failed**: the record could not be judged, or the service did not respond.

Three rules hold for every function on this page.

1. **Reading the answer fails when there is none.** A returned one-record row can give `THINKTHEN_UNSURE` at its answer read. A many-record row, form entry or saved row can also give the record's negative error code. With no answer, the read leaves your variable alone. A missing answer never reads as "no" or as an empty value.
2. **Checking the state never fails.** `result.status`, `result.probability` and `result.error_code` are plain fields, always safe to read.
3. **Printing never fails.** `thinkthen_row_string(&result)` gives the answer, `unsure`, or `failed:` with the reason, as text for `printf`.

## decide

### The simplest if

C's `if` takes a number or a pointer, and a result is neither, so the answer is read into a `bool` first.

```c
#include <thinkthen.h>
#include <stdio.h>
#include <stdlib.h>

int main(void) {
    thinkthen_client *client = NULL;
    thinkthen_question *asks_refund = NULL;
    thinkthen_row is_refund = THINKTHEN_ROW_INIT;
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    bool yes = false;

    int rc = thinkthen_client_open(NULL, &client, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_new(THINKTHEN_FUNCTION_DECIDE,
                                    "Does the customer request a refund?",
                                    THINKTHEN_NUL_TERMINATED, &asks_refund, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_run_one(client, asks_refund, "Please refund my order.",
                               THINKTHEN_NUL_TERMINATED, THINKTHEN_RECORD_TEXT,
                               &is_refund, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_row_answer_bool(&is_refund, &yes, &error);

    if (rc != THINKTHEN_OK) {
        fprintf(stderr, "%s\n", thinkthen_error_string(&error));
    } else if (yes) {
        puts("refund queue");
    } else {
        puts("inbox");
    }

    thinkthen_row_free(&is_refund);
    thinkthen_error_free(&error);
    thinkthen_question_free(asks_refund);
    thinkthen_client_free(client);
    return rc == THINKTHEN_OK ? EXIT_SUCCESS : EXIT_FAILURE;
}
```

If the message could not be judged, `thinkthen_run_one` returns the record's negative error code and fills `error`. The answer read is skipped and `yes` is left alone, so the first branch runs. It never falls through to `inbox`. With one bar, a returned decide row answers yes or no; two bars can make its answer read return `THINKTHEN_UNSURE`.

Check the status before you look at `yes`. Don't write `if (rc == THINKTHEN_OK && yes) { ... } else { ... }`: that `else` would run for a failure as well as for a no.

Every function that can fail returns a status: `THINKTHEN_OK`, which is 0, or the negative of an error code, with the code and message put in `error`. An answer read can also return the positive `THINKTHEN_UNSURE`. So each step runs only if the one before it worked. Client and question construction retain their existing error classifications. The third step, `thinkthen_run_one`, can also fail for the judgement itself. It returns `THINKTHEN_OK` for an unsure row; only the fourth step, the answer read, reports `THINKTHEN_UNSURE`. This positive read status is not an engine error. Each status is marked so the compiler warns when you ignore one.

A result is a `thinkthen_row`, a small struct you declare yourself and start as `THINKTHEN_ROW_INIT`. You free what you make: the row, the error, the question and the client each have a free function, and each is safe to call on a value that was never filled. The later examples are the body of a `main` like this one, with `client`, `error` and `rc` already made.

### Yes, no, or not sure

Set two bars and decide can also be not sure. `is_refund.status` tells you whether there is a yes or no to read.

```c
thinkthen_question *asks_refund = NULL;
thinkthen_row is_refund = THINKTHEN_ROW_INIT;
bool yes = false;

rc = thinkthen_question_new(THINKTHEN_FUNCTION_DECIDE,
                            "Does the customer request a refund?",
                            THINKTHEN_NUL_TERMINATED, &asks_refund, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_question_set_band(asks_refund, 0.2, 0.8, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_run_one(client, asks_refund, "Where is my parcel?",
                           THINKTHEN_NUL_TERMINATED, THINKTHEN_RECORD_TEXT,
                           &is_refund, &error);

if (rc == THINKTHEN_OK) {
    if (is_refund.status == THINKTHEN_STATUS_ANSWERED) {
        rc = thinkthen_row_answer_bool(&is_refund, &yes, &error);
        if (rc == THINKTHEN_OK)
            puts(yes ? "refund queue" : "inbox");
    } else {
        puts("a person reads it");
    }
}
```

You can also branch on the read after checking that the scalar call succeeded:

```c
if (rc == THINKTHEN_OK) {
    rc = thinkthen_row_answer_bool(&is_refund, &yes, &error);
    if (rc == THINKTHEN_UNSURE) {
        puts("a person reads it");
    } else if (rc < 0) {
        fprintf(stderr, "%s\n", thinkthen_error_string(&error));
    } else {
        puts(yes ? "refund queue" : "inbox");
    }
}
```

Only not sure is a missing-answer state of this successfully returned scalar row. A negative read status can still report a mistake in the read, such as using the wrong answer type; it is not a failed judgement arriving late.

### Did the call work

```c
thinkthen_question *asks_refund = NULL;
thinkthen_row is_refund = THINKTHEN_ROW_INIT;

rc = thinkthen_question_new(THINKTHEN_FUNCTION_DECIDE,
                            "Does the customer request a refund?",
                            THINKTHEN_NUL_TERMINATED, &asks_refund, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_run_one(client, asks_refund, "Please refund my order.",
                           THINKTHEN_NUL_TERMINATED, THINKTHEN_RECORD_TEXT,
                           &is_refund, &error);
if (rc < 0)
    fprintf(stderr, "the call did not complete: %s\n", thinkthen_error_string(&error));
else
    printf("%s\n", thinkthen_row_string(&is_refund));
```

The negative-status branch handles all call errors, not only a failed judgement. The same existing code and `thinkthen_error` payload report usage, setup, authentication, cancellation and other failures; their classifications do not change. Do not inspect a result as though the call succeeded after a negative return.

Not sure is a successful call: `thinkthen_run_one` returns `THINKTHEN_OK`, fills the unsure row and printing is safe. Reading its answer then returns the positive `THINKTHEN_UNSURE` status, not a negative error code.

`thinkthen_row_string` gives a NUL-terminated string that belongs to the row. Don't free it, and don't use it after you free the row. The row and question still need their usual free functions after either branch.

### Both bars and the probability

```c
thinkthen_question *asks_refund = NULL;
thinkthen_row is_refund = THINKTHEN_ROW_INIT;

rc = thinkthen_question_new(THINKTHEN_FUNCTION_DECIDE,
                            "Does the customer request a refund?",
                            THINKTHEN_NUL_TERMINATED, &asks_refund, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_question_set_band(asks_refund, 0.2, 0.8, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_run_one(client, asks_refund, "Where is my parcel?",
                           THINKTHEN_NUL_TERMINATED, THINKTHEN_RECORD_TEXT,
                           &is_refund, &error);

if (rc == THINKTHEN_OK && !isnan(is_refund.probability))
    printf("%g\n", is_refund.probability);
```

- No bar set: one bar at 0.5. The answer is always yes or no.
- `thinkthen_question_set_cut(asks_refund, 0.7, &error)`: one bar. Yes at 0.7 or above, otherwise no.
- `thinkthen_question_set_band(asks_refund, 0.2, 0.8, &error)`: two bars. No below 0.2, not sure from 0.2 up to 0.8, yes at 0.8 or above.

`probability` is the number ThinkThen compared with your bars: the model's probability of yes. A confident no has a low probability, such as `0.05`. A returned decide row has a probability even when not sure; a failed bulk row has NaN. Test for NaN with `isnan` from `<math.h>`.

The number is the model's own. To choose bars for your data, run examples you have already labelled through `thinkthen_call_open`, save each `probability`, and compare with your labels.

## choose with your own choices

Give your choices as an enum, with an array that holds the word for each member. The answer comes back as a member of it.

```c
#include <thinkthen.h>
#include <stdio.h>
#include <stdlib.h>

enum topic { TOPIC_BILLING, TOPIC_SHIPPING, TOPIC_OTHER };

static const char *const topic_words[] = {
    [TOPIC_BILLING] = "billing",
    [TOPIC_SHIPPING] = "shipping",
    [TOPIC_OTHER] = "other",
};

static const char *team_for(enum topic topic) {
    switch (topic) {
    case TOPIC_BILLING:
        return "billing team";
    case TOPIC_SHIPPING:
        return "warehouse";
    case TOPIC_OTHER:
        return "front desk";
    }
    abort();
}

int main(void) {
    thinkthen_client *client = NULL;
    thinkthen_question *asks_topic = NULL;
    thinkthen_row topic = THINKTHEN_ROW_INIT;
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    int choice = 0;

    int rc = thinkthen_client_open(NULL, &client, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_new(THINKTHEN_FUNCTION_CHOOSE,
                                    "What is the message about?",
                                    THINKTHEN_NUL_TERMINATED, &asks_topic, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_set_choices(asks_topic, topic_words,
                                            sizeof topic_words / sizeof topic_words[0],
                                            &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_run_one(client, asks_topic, "I was charged twice for one invoice.",
                               THINKTHEN_NUL_TERMINATED, THINKTHEN_RECORD_TEXT,
                               &topic, &error);

    if (rc == THINKTHEN_OK) {
        if (topic.status == THINKTHEN_STATUS_UNSURE) {
            puts("a person reads it");
        } else {
            rc = thinkthen_row_answer_choice(&topic, &choice, &error);
            if (rc == THINKTHEN_OK)
                puts(team_for((enum topic)choice));
        }
    }
    if (rc != THINKTHEN_OK)
        fprintf(stderr, "%s\n", thinkthen_error_string(&error));

    thinkthen_row_free(&topic);
    thinkthen_error_free(&error);
    thinkthen_question_free(asks_topic);
    thinkthen_client_free(client);
    return rc == THINKTHEN_OK ? EXIT_SUCCESS : EXIT_FAILURE;
}
```

choose can be not sure even when you set no bar. Its bar is 0.5 on the best choice, and with three choices all three can fall below it. So check `topic.status` before reading the answer. `thinkthen_question_set_cut(asks_topic, 0.3, &error)` moves the bar.

The words in the array (`"billing"`, `"shipping"`, `"other"`) are what the model reads, so make each a plain word or short phrase, not a code. The member names are for your code. `thinkthen_row_answer_choice` writes the member's number into an `int`, which C turns into the enum, and `topic_words[choice]` is its word. Give every member a word: a gap in the array is refused by `thinkthen_question_set_choices`. `thinkthen_row_string(&topic)` prints the word, such as `billing`.

Add a member to `enum topic` and, with `-Wall -Werror`, the compiler refuses `team_for` until you handle it. C compilers only warn about a missing case, so keep those two flags on.

A choose result does not go in an `if`, and is not equal to its answer. `if (topic)` and `topic == TOPIC_BILLING` do not compile. Read the answer with `thinkthen_row_answer_choice`. One slip does compile: a pointer to a result in an `if`, as in `if (is_refund)` inside `where_it_goes`, tests only that the pointer is not NULL, so it is true for every result. GCC and Clang report `if (&topic)` under `-Wall`, and nothing reports a pointer you were passed.

With no enum, give the same array of words. The answer is then the word's place in the array, counted from 0.

## Many records

`thinkthen_call_open` takes the same question as `thinkthen_run_one` and many records, and `thinkthen_call_next` gives one result per record, in order. Each result carries your own record, which `thinkthen_row_original` reads.

```c
static const thinkthen_record_input messages[] = {
    THINKTHEN_TEXT_RECORD("Please refund my order."),
    THINKTHEN_TEXT_RECORD("Where is my parcel?"),
    THINKTHEN_TEXT_RECORD("I was charged twice for one invoice."),
};
thinkthen_question *asks_refund = NULL;
thinkthen_call *refund_checks = NULL;
thinkthen_row is_refund = THINKTHEN_ROW_INIT;
thinkthen_record_view message;
bool yes = false;

rc = thinkthen_question_new(THINKTHEN_FUNCTION_DECIDE,
                            "Does the customer request a refund?",
                            THINKTHEN_NUL_TERMINATED, &asks_refund, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_question_set_band(asks_refund, 0.2, 0.8, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_call_open(client, asks_refund, messages, 3, &refund_checks, &error);

while (rc == THINKTHEN_OK) {
    rc = thinkthen_call_next(refund_checks, &is_refund, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_row_original(&is_refund, &message, &error);
    if (rc != THINKTHEN_OK)
        break;
    if (is_refund.status == THINKTHEN_STATUS_UNSURE) {
        printf("a person reads it: %.*s\n", (int)message.len, message.data);
    } else if (is_refund.status == THINKTHEN_STATUS_FAILED) {
        printf("try again later: %.*s\n", (int)message.len, message.data);
    } else {
        rc = thinkthen_row_answer_bool(&is_refund, &yes, &error);
        if (rc == THINKTHEN_OK)
            printf("%s: %.*s\n", yes ? "refund queue" : "inbox",
                   (int)message.len, message.data);
    }
}
if (rc == THINKTHEN_END)
    rc = THINKTHEN_OK;

thinkthen_row_free(&is_refund);
thinkthen_call_free(refund_checks);
thinkthen_question_free(asks_refund);
```

One failed record does not stop the loop. This remains a many-record call when the count is 1; the input count does not switch it to scalar semantics. Each result keeps its own state, with `position` counted from 1 and the record's id, which `thinkthen_row_id` reads when it has one. `thinkthen_call_next` returns `THINKTHEN_END` after the last result. A negative status from it is for the call itself failing, such as when it is cancelled, and the loop ends after it.

Pass the whole collection to one call so ThinkThen can send the records in batches. The call copies your records before `thinkthen_call_open` returns, so they need not outlive it. `message` is the row's own copy of your record: its bytes are not NUL-terminated, so print them with their length, and they last until you free the row or read the next result into it. Reading into the same row frees what it held, so one variable serves the whole loop. Freeing the call stops it, so leaving the loop early is safe. To keep every result, read each into its own element of an array of rows.

To save every result, `thinkthen_row_to_json(&is_refund, &line, &error)` writes it into a `thinkthen_buffer` as a line of JSON. It includes the state, so nothing is lost. Free each buffer with `thinkthen_buffer_free(&line)` before you fill it again.

To stop when a bulk row failed, call `thinkthen_row_err_if_failed(&is_refund, &error)` inside the loop after `thinkthen_call_next` succeeded, assign its status to `rc`, and stop if it is negative before reading an answer. Not sure returns `THINKTHEN_OK` from this explicit check. The row stays yours to inspect and free. The shared helper remains available for bulk rows and forms; an ordinary successful scalar call needs no extra failure check.

## Many-result not sure or failed

Yes and no are the two answers. Not sure and failed are the two ways a many-record row can have none. To name every state and have the compiler confirm none is missed, `switch` on `status` with no `default`. Each case can also read what belongs to its state: the probability, or the error code.

```c
static int where_it_goes(const thinkthen_row *is_refund, char *out, size_t size) {
    bool yes = false;
    int rc = THINKTHEN_OK;
    switch (is_refund->status) {
    case THINKTHEN_STATUS_ANSWERED:
        rc = thinkthen_row_answer_bool(is_refund, &yes, NULL);
        if (rc == THINKTHEN_OK)
            snprintf(out, size, "%s", yes ? "refund queue" : "inbox");
        break;
    case THINKTHEN_STATUS_UNSURE:
        snprintf(out, size, "a person reads it: %g", is_refund->probability);
        break;
    case THINKTHEN_STATUS_FAILED:
        snprintf(out, size, "try again later: %s",
                 thinkthen_code_name(is_refund->error_code));
        break;
    }
    return rc;
}
```

Leave out a case and GCC and Clang warn, and with `-Werror`, as in the compile line at the top, the code does not compile. `status` has exactly these three values: `THINKTHEN_STATUS_ANSWERED`, `THINKTHEN_STATUS_UNSURE` and `THINKTHEN_STATUS_FAILED`. It is a plain field, so you can also count or store it.

The shared `thinkthen_status` enum still includes failed even for a row returned by a successful scalar call, where that case is unreachable at runtime. The complete switch stays valid for bulk rows, form entries and saved rows. Put `where_it_goes` outside `main`; the next example calls it on each actual bulk result.

```c
static const thinkthen_record_input messages[] = {
    THINKTHEN_TEXT_RECORD("Please refund my order."),
    THINKTHEN_TEXT_RECORD("Where is my parcel?"),
    THINKTHEN_TEXT_RECORD("I was charged twice for one invoice."),
};
thinkthen_question *asks_refund = NULL;
thinkthen_call *refund_checks = NULL;
thinkthen_row is_refund = THINKTHEN_ROW_INIT;
char route[128];

rc = thinkthen_question_new(THINKTHEN_FUNCTION_DECIDE,
                            "Does the customer request a refund?",
                            THINKTHEN_NUL_TERMINATED, &asks_refund, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_question_set_band(asks_refund, 0.2, 0.8, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_call_open(client, asks_refund, messages, 3, &refund_checks, &error);
while (rc == THINKTHEN_OK) {
    rc = thinkthen_call_next(refund_checks, &is_refund, &error);
    if (rc == THINKTHEN_OK)
        rc = where_it_goes(&is_refund, route, sizeof route);
    if (rc == THINKTHEN_OK)
        puts(route);
}
if (rc == THINKTHEN_END)
    rc = THINKTHEN_OK;
thinkthen_row_free(&is_refund);
thinkthen_call_free(refund_checks);
thinkthen_question_free(asks_refund);
```

## filter

filter means "give me the yeses". It hands back your own records, in order.

```c
static const thinkthen_record_input messages[] = {
    THINKTHEN_TEXT_RECORD("Please refund my order."),
    THINKTHEN_TEXT_RECORD("Where is my parcel?"),
    THINKTHEN_TEXT_RECORD("I was charged twice for one invoice."),
};
thinkthen_question *asks_refund = NULL;
thinkthen_call *call = NULL;
thinkthen_row refund_request = THINKTHEN_ROW_INIT;
thinkthen_record_view message;

rc = thinkthen_question_new(THINKTHEN_FUNCTION_FILTER,
                            "Does the customer request a refund?",
                            THINKTHEN_NUL_TERMINATED, &asks_refund, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_call_open(client, asks_refund, messages, 3, &call, &error);

while (rc == THINKTHEN_OK) {
    rc = thinkthen_call_next_kept(call, &refund_request, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_row_original(&refund_request, &message, &error);
    if (rc == THINKTHEN_OK)
        printf("refund queue: %.*s\n", (int)message.len, message.data);
}
```

filter takes one bar, so every record is a yes or a no and none is left out for being not sure. `thinkthen_question_set_cut(asks_refund, 0.7, &error)` moves the bar. Two bars are refused: `thinkthen_question_set_band` on a filter question returns `-THINKTHEN_CODE_USAGE_THRESHOLD`. When you want not sure, use a decide question and `thinkthen_call_next`, as in the section above.

A filter call is read with `thinkthen_call_next_kept`, and each kept record arrives in a row whose only job is to carry it. `thinkthen_call_next` refuses a filter call.

If a record fails, you still get every kept record first. Then, where it would have returned `THINKTHEN_END`, `thinkthen_call_next_kept` returns the first failed record's negative error code, and `thinkthen_call_next_failed` reads every failed result, each with its record.

```c
if (rc != THINKTHEN_END) {
    thinkthen_row failed = THINKTHEN_ROW_INIT;
    while (thinkthen_call_next_failed(call, &failed, &error) == THINKTHEN_OK) {
        if (thinkthen_row_original(&failed, &message, &error) == THINKTHEN_OK)
            printf("try again later: %.*s (%s)\n", (int)message.len, message.data,
                   thinkthen_row_string(&failed));
    }
    thinkthen_row_free(&failed);
}

thinkthen_row_free(&refund_request);
thinkthen_call_free(call);
thinkthen_question_free(asks_refund);
```

## tag

Labels work as choices do. The answer is one `bool` per label, in the order of your enum, and all of them can be false.

```c
enum topic { TOPIC_BILLING, TOPIC_SHIPPING };
static const char *const topic_words[] = {
    [TOPIC_BILLING] = "billing",
    [TOPIC_SHIPPING] = "shipping",
};
thinkthen_question *asks_topics = NULL;
thinkthen_row topics = THINKTHEN_ROW_INIT;
bool fits[2] = {false};

rc = thinkthen_question_new(THINKTHEN_FUNCTION_TAG, "Which topics appear?",
                            THINKTHEN_NUL_TERMINATED, &asks_topics, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_question_set_labels(asks_topics, topic_words, 2, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_run_one(client, asks_topics,
                           "I was charged twice and my parcel is late again.",
                           THINKTHEN_NUL_TERMINATED, THINKTHEN_RECORD_TEXT,
                           &topics, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_row_answer_tags(&topics, fits, 2, &error);

if (rc == THINKTHEN_OK) {
    if (fits[TOPIC_BILLING])
        puts("billing team");
    for (int topic = 0; topic < 2; topic++) {
        if (fits[topic])
            puts(topic_words[topic]);
    }
}
```

tag is never not sure. `thinkthen_question_set_cut` sets how likely a label must be to count. The array is yours, and it must have one `bool` for each label.

## score

Levels name a scale, lowest first. The answer is a number from 0 to 1 along it. The order of the array passed to `thinkthen_question_set_levels` defines the scale, lowest first. Here the enum values index `urgency_words`, so its array order is routine then urgent. Enum names do not define a separate order. Rank and score entries use that same array order.

```c
enum urgency { URGENCY_ROUTINE, URGENCY_URGENT };
static const char *const urgency_words[] = {
    [URGENCY_ROUTINE] = "routine",
    [URGENCY_URGENT] = "urgent",
};
thinkthen_question *asks_urgency = NULL;
thinkthen_row urgency = THINKTHEN_ROW_INIT;
double score = 0;

rc = thinkthen_question_new(THINKTHEN_FUNCTION_SCORE, "How urgent is the request?",
                            THINKTHEN_NUL_TERMINATED, &asks_urgency, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_question_set_levels(asks_urgency, urgency_words, 2, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_run_one(client, asks_urgency, "Please refund my order.",
                           THINKTHEN_NUL_TERMINATED, THINKTHEN_RECORD_TEXT,
                           &urgency, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_row_answer_score(&urgency, &score, &error);

if (rc == THINKTHEN_OK) {
    if (score > 0.5)
        puts("answer today");
    printf("%.2f\n", score);
}
```

The score is a position between your levels, not one of them, so it is a `double` and there is nothing to `switch` on. To get exactly one level back, ask with a choose question. score is never not sure.

## rank

rank puts many records in order along your levels, highest first.

```c
static const thinkthen_record_input messages[] = {
    THINKTHEN_TEXT_RECORD("Please refund my order."),
    THINKTHEN_TEXT_RECORD("Where is my parcel?"),
    THINKTHEN_TEXT_RECORD("I was charged twice for one invoice."),
};
thinkthen_question *asks_urgency = NULL;
thinkthen_call *call = NULL;
thinkthen_row ranked = THINKTHEN_ROW_INIT;
thinkthen_record_view message;
uint64_t rank = 0;

rc = thinkthen_question_new(THINKTHEN_FUNCTION_RANK, "How urgent is the request?",
                            THINKTHEN_NUL_TERMINATED, &asks_urgency, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_question_set_levels(asks_urgency, urgency_words, 2, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_call_open(client, asks_urgency, messages, 3, &call, &error);

while (rc == THINKTHEN_OK) {
    rc = thinkthen_call_next(call, &ranked, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_row_answer_rank(&ranked, &rank, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_row_original(&ranked, &message, &error);
    if (rc == THINKTHEN_OK)
        printf("%" PRIu64 " %.*s\n", rank, (int)message.len, message.data);
}
```

`thinkthen_row_answer_rank` writes the rank, starting at 1, and `thinkthen_row_original` gives your record. rank reads the whole collection before it gives anything back. `thinkthen_question_set_top(asks_urgency, 5, &error)` returns the five highest-ranked records after judging the collection. A failed record comes back after the ranked ones, with `status` set to `THINKTHEN_STATUS_FAILED`. `PRIu64` is from `<inttypes.h>`.

## find

find points to the one line that answers.

```c
const char *message = "Morgan works for Example Group.\nPlease refund my order.";
thinkthen_question *asks_action = NULL;
thinkthen_row action = THINKTHEN_ROW_INIT;
thinkthen_span span;

rc = thinkthen_question_new(THINKTHEN_FUNCTION_FIND,
                            "Which line states the requested action?",
                            THINKTHEN_NUL_TERMINATED, &asks_action, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_run_one(client, asks_action, message, THINKTHEN_NUL_TERMINATED,
                           THINKTHEN_RECORD_TEXT, &action, &error);

if (rc == THINKTHEN_OK) {
    if (action.status == THINKTHEN_STATUS_UNSURE) {
        puts("a person reads it");
    } else {
        rc = thinkthen_row_answer_span(&action, &span, &error);
        if (rc == THINKTHEN_OK && !span.present)
            puts("no line answers");
        else if (rc == THINKTHEN_OK)
            printf("%.*s\n", (int)span.text.len, span.text.data);
    }
}
```

The answer is a `thinkthen_span` with `text`, `start` and `end`, and with `present` set to 0 when no line answers. The offsets count bytes, as C strings do, so `message + span.start` is where the line starts and `span.end - span.start` is its length. `span.text` belongs to the row and is not NUL-terminated. Like choose, find can be not sure with no bar set.

## recognize

```c
thinkthen_question *finds_names = NULL;
thinkthen_row names = THINKTHEN_ROW_INIT;
const thinkthen_name *found = NULL;
size_t count = 0;

rc = thinkthen_question_new(THINKTHEN_FUNCTION_RECOGNIZE, NULL, 0, &finds_names, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_question_add_kind(finds_names, "person", THINKTHEN_NUL_TERMINATED, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_question_add_kind(finds_names, "organization",
                                     THINKTHEN_NUL_TERMINATED, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_run_one(client, finds_names, "Morgan works for Example Group.",
                           THINKTHEN_NUL_TERMINATED, THINKTHEN_RECORD_TEXT,
                           &names, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_row_answer_names(&names, &found, &count, &error);

for (size_t i = 0; rc == THINKTHEN_OK && i < count; i++) {
    const thinkthen_name *name = &found[i];
    printf("%.*s %.*s\n", (int)name->kind.len, name->kind.data,
           (int)name->text.len, name->text.data);
}
```

Each name also has `start`, `end` and `probability`. The answer can be empty, with `count` 0. recognize is never not sure. The names belong to the row: don't free them, and don't use them after you free the row.

## relate

```c
const char *message = "Morgan works for Example Group.\nPlease refund my order.";
const thinkthen_rule rule = {
    .source = {"person", THINKTHEN_NUL_TERMINATED},
    .target = {"organization", THINKTHEN_NUL_TERMINATED},
    .relation = {"works_for", THINKTHEN_NUL_TERMINATED},
};
thinkthen_question *finds_relations = NULL;
thinkthen_row relations = THINKTHEN_ROW_INIT;
const thinkthen_relation *found = NULL;
size_t count = 0;

rc = thinkthen_question_new(THINKTHEN_FUNCTION_RELATE, NULL, 0, &finds_relations, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_question_add_rule(finds_relations, &rule, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_run_one(client, finds_relations, message, THINKTHEN_NUL_TERMINATED,
                           THINKTHEN_RECORD_TEXT, &relations, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_row_answer_relations(&relations, &found, &count, &error);

for (size_t i = 0; rc == THINKTHEN_OK && i < count; i++) {
    const thinkthen_relation *relation = &found[i];
    printf("%.*s %.*s %.*s\n",
           (int)relation->source.text.len, relation->source.text.data,
           (int)relation->relation.len, relation->relation.data,
           (int)relation->target.text.len, relation->target.text.data);
}
```

A rule names the kind it starts from as `source`, the kind it points to as `target`, and the relation. Each relation has `source`, `target`, `relation`, `probability` and `either`. `thinkthen_row_detail_names(&relations, &every_name, &count, &error)` gives every name found, including names in no relation. The relations and names belong to the row.

## annotate

annotate asks several questions of each record and gives back one form per record.

```c
static const thinkthen_record_input messages[] = {
    THINKTHEN_TEXT_RECORD("Please refund my order."),
    THINKTHEN_TEXT_RECORD("Where is my parcel?"),
    THINKTHEN_TEXT_RECORD("I was charged twice for one invoice."),
};
thinkthen_question *asks_refund = NULL;
thinkthen_question *asks_topic = NULL;
thinkthen_question *asks_urgency = NULL;
thinkthen_question *questions = NULL;
thinkthen_call *forms = NULL;
thinkthen_row form = THINKTHEN_ROW_INIT;
thinkthen_row is_refund = THINKTHEN_ROW_INIT;
thinkthen_row topic = THINKTHEN_ROW_INIT;
thinkthen_row urgency = THINKTHEN_ROW_INIT;
thinkthen_record_view message;
bool yes = false;
int choice = 0;
double score = 0;

rc = thinkthen_question_new(THINKTHEN_FUNCTION_DECIDE,
                            "Does the customer request a refund?",
                            THINKTHEN_NUL_TERMINATED, &asks_refund, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_question_new(THINKTHEN_FUNCTION_CHOOSE, "What is the message about?",
                                THINKTHEN_NUL_TERMINATED, &asks_topic, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_question_set_choices(asks_topic, topic_words, 3, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_question_new(THINKTHEN_FUNCTION_SCORE, "How urgent is the request?",
                                THINKTHEN_NUL_TERMINATED, &asks_urgency, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_question_set_levels(asks_urgency, urgency_words, 2, &error);

if (rc == THINKTHEN_OK)
    rc = thinkthen_question_new(THINKTHEN_FUNCTION_ANNOTATE, NULL, 0, &questions, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_question_add_entry(questions, "refund", THINKTHEN_NUL_TERMINATED,
                                      asks_refund, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_question_add_entry(questions, "topic", THINKTHEN_NUL_TERMINATED,
                                      asks_topic, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_question_add_entry(questions, "urgency", THINKTHEN_NUL_TERMINATED,
                                      asks_urgency, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_call_open(client, questions, messages, 3, &forms, &error);

while (rc == THINKTHEN_OK) {
    rc = thinkthen_call_next(forms, &form, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_row_original(&form, &message, &error);
    if (rc != THINKTHEN_OK)
        break;
    if (form.status == THINKTHEN_STATUS_FAILED) {
        printf("try again later: %.*s\n", (int)message.len, message.data);
        continue;
    }
    rc = thinkthen_row_answer_entry(&form, "refund", THINKTHEN_NUL_TERMINATED,
                                    &is_refund, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_row_answer_entry(&form, "topic", THINKTHEN_NUL_TERMINATED,
                                        &topic, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_row_answer_entry(&form, "urgency", THINKTHEN_NUL_TERMINATED,
                                        &urgency, &error);
    if (rc != THINKTHEN_OK)
        break;
    if (is_refund.status == THINKTHEN_STATUS_FAILED ||
        topic.status == THINKTHEN_STATUS_FAILED ||
        urgency.status == THINKTHEN_STATUS_FAILED) {
        printf("try again later: %.*s\n", (int)message.len, message.data);
        continue;
    }
    if (is_refund.status == THINKTHEN_STATUS_UNSURE ||
        topic.status == THINKTHEN_STATUS_UNSURE ||
        urgency.status == THINKTHEN_STATUS_UNSURE) {
        printf("a person reads it: %.*s\n", (int)message.len, message.data);
        continue;
    }
    rc = thinkthen_row_answer_bool(&is_refund, &yes, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_row_answer_choice(&topic, &choice, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_row_answer_score(&urgency, &score, &error);
    if (rc == THINKTHEN_OK && yes)
        printf("refund queue: %s %.2f\n", topic_words[choice], score);
}
```

Each entry of a form is a result of its own function, with its own state. `thinkthen_row_answer_entry` is the form's answer read: it writes the named entry into a row of your own, which you read and free like any other result. The `"refund"` entry is a decide result, so `thinkthen_row_answer_bool` reads it. One entry can be not sure while the others answer. Asking for a name the form does not have is a mistake in the call and returns `-THINKTHEN_CODE_USAGE_NAME`. So is reading an entry with another function's read, which returns `-THINKTHEN_CODE_USAGE_FUNCTION`. Here `enum topic` is the three-choice enum from the choose section.

The annotate question copies each question you add, so you may free `asks_refund`, `asks_topic` and `asks_urgency` as soon as they are added.

A form can be answered while an entry failed. This example checks all three entries before reading their answers: failed entries go to retry, not-sure entries go to a person, and only answered entries reach the refund branch.

For one record, `thinkthen_run_one` fails for the parent form's own error, not for its children's states. An answered parent is returned with `THINKTHEN_OK` even if one, several or all entries failed. The call does not invoke the recursive failure helper or count failed children. Existing call-level errors still fail the call.

With the same questions, you can explicitly ask for any failed entry to stop the program:

```c
rc = thinkthen_run_one(client, questions, "Please refund my order.",
                       THINKTHEN_NUL_TERMINATED, THINKTHEN_RECORD_TEXT,
                       &form, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_row_err_if_failed(&form, &error);
```

The second step is optional. It returns the failed form's or first failed entry's negative code, with its code, message and HTTP status in `error`. Not-sure entries do not fail this check. The form is still yours to inspect and free, including after the explicit check fails. The same helper applies to forms from many-record calls; a successful parent alone never authorizes reading an unchecked entry's answer.

## Say what each answer means

When a short question is not enough, say what the answers mean.

```c
rc = thinkthen_question_set_yes_means(
    asks_refund, "the customer wants money back, including for a duplicate charge",
    THINKTHEN_NUL_TERMINATED, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_question_set_no_means(asks_refund, "the customer wants anything else",
                                         THINKTHEN_NUL_TERMINATED, &error);

if (rc == THINKTHEN_OK)
    rc = thinkthen_question_set_description(
        asks_topic, topic_words[TOPIC_BILLING], THINKTHEN_NUL_TERMINATED,
        "charges, invoices and refunds", THINKTHEN_NUL_TERMINATED, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_question_set_description(
        asks_topic, topic_words[TOPIC_SHIPPING], THINKTHEN_NUL_TERMINATED,
        "parcels and delivery", THINKTHEN_NUL_TERMINATED, &error);
```

decide and filter questions take `thinkthen_question_set_yes_means` and `thinkthen_question_set_no_means`. choose, tag, score and rank questions take `thinkthen_question_set_description`, keyed by the word of your choice, label or level.

## Records and fields

A record is UTF-8 text or one JSON value, given as bytes with their length and a kind: `THINKTHEN_RECORD_TEXT` or `THINKTHEN_RECORD_JSON`. Pass `THINKTHEN_NUL_TERMINATED` as the length of a C string. `thinkthen_question_set_field(asks_refund, "/text", THINKTHEN_NUL_TERMINATED, &error)` picks one field of a JSON object. Without it, ThinkThen judges the whole record.

Give JSON records an `id`. Each result carries it back, and `thinkthen_row_id(&is_refund, &id, &error)` reads it.

C has no conversion to set up for your own struct. Point each `thinkthen_record_input` at the text your struct holds, with `THINKTHEN_TEXT_RECORD` or `THINKTHEN_JSON_RECORD` for a C string, or by filling in `data` and `len` for counted bytes. `thinkthen_row_original` hands those bytes back.

`thinkthen_run_one` takes one record, so a JSON array given to it is judged as one record. Use `thinkthen_call_open` for many. A blank record (an empty string, only spaces, or `{}`) returns a negative status from `thinkthen_run_one` with code `record.empty`, before any service request. The same input through `thinkthen_call_open` remains a failed row even when the count is 1.

ThinkThen copies everything you pass in before the function returns. You never have to keep a question's text, a record or a word array alive for it.

## Saved questions

A question is a value. Build it once with its options, and many calls can share one definition.

```c
thinkthen_question *asks_refund = NULL;
thinkthen_row is_refund = THINKTHEN_ROW_INIT;

rc = thinkthen_question_new(THINKTHEN_FUNCTION_DECIDE,
                            "Does the customer request a refund?",
                            THINKTHEN_NUL_TERMINATED, &asks_refund, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_question_set_band(asks_refund, 0.2, 0.8, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_question_set_field(asks_refund, "/text", THINKTHEN_NUL_TERMINATED, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_run_one(client, asks_refund,
                           "{\"id\": \"a1\", \"text\": \"Please refund my order.\"}",
                           THINKTHEN_NUL_TERMINATED, THINKTHEN_RECORD_JSON,
                           &is_refund, &error);
```

There is one kind of question per function, named by its `THINKTHEN_FUNCTION_` constant. A toolbox keeps named questions in a JSON file:

```c
char json[65536];
FILE *file = fopen("toolbox.json", "rb");
size_t json_len = file ? fread(json, 1, sizeof json, file) : 0;
if (file)
    fclose(file);

thinkthen_toolbox *toolbox = NULL;
thinkthen_question *asks_topic = NULL;
thinkthen_row topic = THINKTHEN_ROW_INIT;

rc = thinkthen_toolbox_from_json(json, json_len, &toolbox, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_toolbox_select_as(toolbox, "topic", THINKTHEN_NUL_TERMINATED,
                                     THINKTHEN_FUNCTION_CHOOSE, &asks_topic, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_question_expect_choices(asks_topic, topic_words, 3, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_run_one(client, asks_topic,
                           "{\"id\": \"a1\", \"text\": \"Where is my parcel?\"}",
                           THINKTHEN_NUL_TERMINATED, THINKTHEN_RECORD_JSON, &topic, &error);

thinkthen_row_free(&topic);
thinkthen_question_free(asks_topic);
thinkthen_toolbox_free(toolbox);
```

`thinkthen_toolbox_select_as` returns `-THINKTHEN_CODE_USAGE_NAME` when the toolbox has no entry of that name, or the entry asks another function. `thinkthen_question_expect_choices` returns `-THINKTHEN_CODE_USAGE_QUESTION` when the saved choices are not exactly `topic_words`, in the same order. The question you select is your own copy: free it yourself, before or after the toolbox.

## Settings

`thinkthen_client_open` given NULL reads `THINKTHEN_API_KEY` and `THINKTHEN_BACKEND` from the environment. To choose settings yourself, build them and open the client from them, then make the same calls on it.

```c
thinkthen_settings *settings = NULL;
thinkthen_client *client = NULL;

rc = thinkthen_settings_new(&settings, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_settings_set_model(settings, "...", THINKTHEN_NUL_TERMINATED, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_settings_set_batch(settings, 16, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_settings_set_retry(settings, 5, 500, 120000, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_client_open(settings, &client, &error);
thinkthen_settings_free(settings);

if (rc == THINKTHEN_OK)
    rc = thinkthen_run_one(client, asks_refund, "Please refund my order.",
                           THINKTHEN_NUL_TERMINATED, THINKTHEN_RECORD_TEXT,
                           &is_refund, &error);
```

The setters are `thinkthen_settings_set_backend`, `_set_url`, `_set_model`, `_set_api_key`, `_set_batch` and `_set_retry`. `thinkthen_settings_set_retry` takes the number of retries, the first wait and the time allowed for one attempt, both in milliseconds. The client copies the settings, so free them as soon as it is open. A client opened from settings still reads the key from the environment when you set none. Keep the key in the environment, not in your code.

Every call waits for its answer. C has no async form, so a caller who must not wait runs the call on a thread of its own. One client is safe to share between threads.

## Usage

```c
thinkthen_usage usage = {.struct_size = sizeof usage};

rc = thinkthen_run_one(client, asks_refund, "Please refund my order.",
                       THINKTHEN_NUL_TERMINATED, THINKTHEN_RECORD_TEXT,
                       &is_refund, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_client_usage_values(client, &usage, &error);
if (rc == THINKTHEN_OK)
    printf("%" PRIu64 " %" PRIu64 " %" PRIu64 "\n", usage.attempts,
           usage.input_tokens.reported, usage.output_tokens.reported);
```

`thinkthen_client_usage_values` counts everything the client has sent so far: `attempts`, `responses`, retries, bytes sent, and tokens in and out as the service reported them. `usage` is a plain struct of numbers that you own, with nothing to free.

## Cancel

```c
thinkthen_cancel *stop = NULL;
thinkthen_settings *settings = NULL;
thinkthen_client *client = NULL;

rc = thinkthen_cancel_new(&stop, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_settings_new(&settings, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_settings_set_cancel(settings, stop, &error);
if (rc == THINKTHEN_OK)
    rc = thinkthen_client_open(settings, &client, &error);
thinkthen_settings_free(settings);

if (rc == THINKTHEN_OK)
    rc = thinkthen_run_one(client, asks_refund, "Please refund my order.",
                           THINKTHEN_NUL_TERMINATED, THINKTHEN_RECORD_TEXT,
                           &is_refund, &error);

thinkthen_client_free(client);
thinkthen_cancel_free(stop);
```

Calling `thinkthen_cancel_request(stop)` from another thread stops every call on that client, and each returns `-THINKTHEN_CODE_LOCAL_CANCELLED`. A many-record call also has `thinkthen_call_cancel(call)`, which stops that one call from any thread. Free `stop` with `thinkthen_cancel_free` once no thread will use it again. The client keeps its own hold on it, so the order of the two frees does not matter.

## Errors

| What happened | Where it shows | What you get |
| --- | --- | --- |
| A mistake in the call, such as a bad threshold | When you build the question or make the call | A negative status with a `usage.` code, such as `-THINKTHEN_CODE_USAGE_THRESHOLD`. Two bars given to filter, and a filter or rank question given to `thinkthen_run_one`, are refused this way |
| The client could not start, such as no API key | At `thinkthen_client_open` | `-THINKTHEN_CODE_USAGE_KEY_MISSING` |
| The call was cancelled | At the call | `-THINKTHEN_CODE_LOCAL_CANCELLED` |
| A one-record judgement failed, including blank input or a service failure on its row | At `thinkthen_run_one` | The unchanged negative code and `thinkthen_error` payload |
| A many-record row or form entry failed | At its answer read or explicit `thinkthen_row_err_if_failed` | The row's negative code, with its message in `error` |
| An annotate parent failed | At the scalar call; retained as a failed row in a many-record call | The parent's negative code from the scalar call; failed row in bulk |
| An answered annotate parent contains failed entries | The scalar call returns `THINKTHEN_OK`; a failed entry read or explicit form check reports failure | The entry's negative code and error payload |
| Not sure | The scalar call succeeds; the answer read reports it | `THINKTHEN_OK` from `thinkthen_run_one`, then `THINKTHEN_UNSURE` from the read |
| You print a result, store it, or read its `status`, `probability` or `error_code` | Never | |

For valid answer reads on shared many-record rows, form entries or saved rows, two tests let you handle by meaning:

- `rc != THINKTHEN_OK` is "there was no answer to read". It covers not sure and failed.
- `rc < 0` is "something went wrong". `THINKTHEN_UNSURE` is a positive number and never an error code, because not sure is a result and not an error. So a function that hands `rc` back to a caller who tests `rc < 0` must first say what not sure should do.

A mistake in the read itself, such as `thinkthen_row_answer_bool` on a choose result, is also a negative status, with the code `usage.function`.

A `thinkthen_error` carries `code` and `http_status`, and `thinkthen_error_string(&error)` gives the code's name and the message as one NUL-terminated string. `thinkthen_code_name(rc)` gives the name alone, and `thinkthen_code_family(rc)` groups the codes into `usage`, `record`, `backend`, `local` and `internal`. After a filter fails, `thinkthen_call_next_failed` reads the failed results. A not-sure result carries its `probability`.

Start every error as `THINKTHEN_ERROR_INIT`. You may pass NULL where you don't want the message.

## Upgrading from 0.1

The baseline is the [published 0.1.2 package/source](https://proxy.golang.org/github.com/botassembly/thinkthen/@v/v0.1.2.zip). Replace its call envelopes with the results shown above.

| 0.1.2 | Now |
| --- | --- |
| `thinkthen_engine_new`, `thinkthen_decide` and output `thinkthen_answer` | Typed client/question/row API |
| Generic `thinkthen_call` JSON envelope | Typed result and explicit answer readers. Failed scalar judgements return a negative status at `thinkthen_run_one`; bulk calls retain failed rows |
| Engine-local error reads | The diagnostic and result errors shown above |

## Ownership and freeing

Everything ThinkThen makes for you has one free function, and each free function accepts NULL or an empty value.

| Value | How you get it | Who frees it, and when |
| --- | --- | --- |
| `thinkthen_client *` | `thinkthen_client_open` | You, with `thinkthen_client_free`, after your last call on it. Many-record calls already open keep running |
| `thinkthen_settings *` | `thinkthen_settings_new` | You, with `thinkthen_settings_free`, as soon as the client is open |
| `thinkthen_question *` | `thinkthen_question_new`, `thinkthen_toolbox_select_as` | You, with `thinkthen_question_free`. Calls, forms and toolboxes keep their own copies |
| `thinkthen_toolbox *` | `thinkthen_toolbox_from_json` | You, with `thinkthen_toolbox_free` |
| `thinkthen_call *` | `thinkthen_call_open` | You, with `thinkthen_call_free`, which also stops it. Rows already read stay valid |
| `thinkthen_cancel *` | `thinkthen_cancel_new` | You, with `thinkthen_cancel_free` |
| `thinkthen_row` | You declare it as `THINKTHEN_ROW_INIT`. `thinkthen_run_one`, the `thinkthen_call_next` family and `thinkthen_row_answer_entry` fill it | You, with `thinkthen_row_free`, after its last use. Filling it again frees what it held |
| `thinkthen_error` | You declare it as `THINKTHEN_ERROR_INIT`. A failing function fills it | You, with `thinkthen_error_free`, after its last use. Passing it again frees what it held |
| `thinkthen_buffer` | `thinkthen_row_to_json` | You, with `thinkthen_buffer_free`, before you fill it again. It is not freed for you |
| The string from `thinkthen_row_string` | A row | The row. Never free it. It ends when the row is freed or filled again |
| The string from `thinkthen_error_string` | An error | The error. Never free it. It ends when the error is freed or filled again |
| `thinkthen_record_view`, `thinkthen_span`, `thinkthen_id`, and the arrays of `thinkthen_name` and `thinkthen_relation` | An answer read, `thinkthen_row_original`, `thinkthen_row_id`, `thinkthen_row_detail_names` | The row they came from. Never free them. They end when that row is freed or filled again |
| `bool`, `int`, `double`, `uint64_t` answers, the `bool` array of tag, and `thinkthen_usage` | An answer read, or `thinkthen_client_usage_values`, writes into your variable | Nobody. They are plain values of yours |
| The strings from `thinkthen_code_name` and `thinkthen_code_family` | Any code | Nobody. They last for the whole program |
| Text, records, word arrays and rules you pass in | You | You, whenever you like. ThinkThen copies them before the function returns |

A row or an error owns memory, so assigning one to another variable moves it and does not copy it. Free only one of the two.

A client may be shared by every thread. One thread at a time changes a question or reads a call. `thinkthen_cancel_request` and `thinkthen_call_cancel` may run on any thread. A row, with everything read from it, belongs to whichever thread holds it.

---
