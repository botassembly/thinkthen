# ThinkThen in C and C++

[Where record text goes](service.md) describes keys, service selection, connection settings and [relation rule fields](service.md#relation-rule-fields).

C answers one text with one call, such as `thinkthen_decide`, and reads many records from a call with `thinkthen_call_next`, or `thinkthen_call_next_kept` for a filter. C++ named methods return one row and `_each` methods return streams. A JSON array inside one record remains one record. C frees calls explicitly; C++ stream owners close calls, and `close()` ends early traversal.

Check `share/thinkthen-c/README.md` and `share/thinkthen-cpp/README.md` inside their archives for the release package's operating-system floor and runtime requirements before installation. Use the package for your operating system and architecture.

Extract the release C 0.2.0 archive into a `thinkthen` prefix. C++ also needs the release matching C++ archive extracted into that prefix. C requires a C11 compiler and pkg-config; C++ requires C++17. The C archive includes the native engine. Keep its headers and library together.

Compile C with `cc -std=c11 -Wall -Wextra -Werror example.c -o example $(PKG_CONFIG_PATH="$PWD/thinkthen/lib/pkgconfig" pkg-config --cflags --libs thinkthen-c) -Wl,-rpath,"$PWD/thinkthen/lib"`. Compile C++ with `c++ -std=c++17 -Ithinkthen/include example.cpp -Lthinkthen/lib -lthinkthen -Wl,-rpath,"$PWD/thinkthen/lib" -o example`. The release CMake configuration offers ThinkThen::Cpp.

Set `THINKTHEN_API_KEY` in the environment before opening a client. Keep keys out of source and reports. Omitted settings select the default backend, `systemone`. The C++ examples below use default settings; to choose `openai` instead, set `THINKTHEN_BACKEND=openai` and `OPENAI_API_KEY`.

## C: one call per answer

Include `<thinkthen.h>`. This program asks one yes-or-no question about one text:

```c
#include <thinkthen.h>
#include <stdio.h>
#include <stdlib.h>

static void report(const thinkthen_error *error) {
    fprintf(stderr, "%s: ", thinkthen_code_name(error->code));
    fwrite(error->message.data, 1, error->message.len, stderr);
    fputc('\n', stderr);
}

int main(void) {
    thinkthen_client *client = NULL;
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    bool is_refund = false;
    int rc = thinkthen_client_open(NULL, &client, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_decide(client, "Does the customer request a refund?",
                              "Please refund my order.",
                              THINKTHEN_NUL_TERMINATED, &is_refund, &error);
    if (rc == THINKTHEN_OK)
        puts(is_refund ? "true" : "false");
    else
        report(&error);
    thinkthen_error_free(&error);
    thinkthen_client_free(client);
    return rc == THINKTHEN_OK ? EXIT_SUCCESS : EXIT_FAILURE;
}
```

`thinkthen_decide` returns `THINKTHEN_OK` and writes `is_refund` only when there is an answer. When the call fails it returns the negative of a `thinkthen_code` and leaves `is_refund` alone, so a failure can never read as "no". Blank text returns `-THINKTHEN_CODE_RECORD_EMPTY` and is never sent to the backend. The error holds the code and a message, and `thinkthen_code_name` turns the code into a name such as `record.empty`.

`thinkthen_client_open` with NULL settings reads the environment: `THINKTHEN_BACKEND` picks the backend, and the backend's own variables give the key and URL. `systemone`, the default, reads `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL`; `openai` reads `OPENAI_API_KEY`. An unknown `THINKTHEN_BACKEND` returns `-THINKTHEN_CODE_USAGE_BACKEND`, and a missing key returns `-THINKTHEN_CODE_USAGE_KEY_MISSING`. To fix the backend in code, call `thinkthen_client_open_backend(THINKTHEN_BACKEND_OPENAI, &client, &error)`. To set the key, model, URL, batch size or retries in code, build settings with `thinkthen_settings_new` and its setters, then pass them to `thinkthen_client_open`; the client copies them, so free them at once.

Each one-call function takes the client, the question as a NUL-terminated string, the text with its length, and a place for the answer. It returns `THINKTHEN_OK` with the answer written, or a negative code with the answer left alone.

| Function | Takes | Writes |
| --- | --- | --- |
| `thinkthen_decide` | question | `bool`, yes or no |
| `thinkthen_decide_band` | question, low bar and high bar | `bool`, yes or no; returns `THINKTHEN_UNSURE` between the bars |
| `thinkthen_choose` | question, choices and count | the index of the chosen string; returns `THINKTHEN_UNSURE` when no choice reaches the cut |
| `thinkthen_tag` | question, labels and count | one `bool` per label, in label order |
| `thinkthen_score` | question, levels lowest first, and count | a `double` from 0 for the first level to 1 for the last |
| `thinkthen_find` | question | a `thinkthen_span` that points into your text, with byte offsets; `present` is 0 when no part answers; returns `THINKTHEN_UNSURE` when not sure |
| `thinkthen_recognize` | kinds and count | a new `thinkthen_value`; read it with `thinkthen_value_name_count` and `thinkthen_value_name_at` |
| `thinkthen_relate` | `thinkthen_rule` array and count | a new `thinkthen_value`; read it with `thinkthen_value_relation_count` and `thinkthen_value_relation_at` |
| `thinkthen_annotate` | names, questions and count | a new `thinkthen_value`; read one answer with `thinkthen_value_annotation_get(value, "name", THINKTHEN_NUL_TERMINATED, &answer, &error)` |

Free a written `thinkthen_value` with `thinkthen_value_free`. Filter and rank judge many records, so they have no one-call function; use a call, as below.

## C: not sure

`thinkthen_decide` uses the default cut, which always answers. To let the model say it is not sure, call `thinkthen_decide_band(client, question, 0.3, 0.7, text, len, &is_refund, &error)`. Below the low bar is no, at or above the high bar is yes, and between them it returns `THINKTHEN_UNSURE` with `is_refund` unchanged. It refuses bars unless 0 < low < high <= 1, and sends nothing.

To also say what yes and no mean, build the question, set a band, and ask it with `thinkthen_run_one`:

```c
#include <thinkthen.h>
#include <stdio.h>
#include <stdlib.h>

static void report(const thinkthen_error *error) {
    fprintf(stderr, "%s: ", thinkthen_code_name(error->code));
    fwrite(error->message.data, 1, error->message.len, stderr);
    fputc('\n', stderr);
}

int main(void) {
    thinkthen_client *client = NULL;
    thinkthen_question *refund = NULL;
    thinkthen_row row = THINKTHEN_ROW_INIT;
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    int rc = thinkthen_client_open(NULL, &client, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_new(THINKTHEN_FUNCTION_DECIDE,
                                    "Does the customer ask for a refund?",
                                    THINKTHEN_NUL_TERMINATED, &refund, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_set_yes_means(
            refund, "the customer wants their money back",
            THINKTHEN_NUL_TERMINATED, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_set_no_means(
            refund, "the customer wants something else",
            THINKTHEN_NUL_TERMINATED, &error);
    /* Below 0.3 is no, 0.7 and above is yes, and between is not sure. */
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_set_band(refund, 0.3, 0.7, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_run_one(client, refund, "I may send it back.",
                               THINKTHEN_NUL_TERMINATED, THINKTHEN_RECORD_TEXT,
                               &row, &error);
    if (rc == THINKTHEN_OK) {
        bool is_refund = row.answer_bool;
        puts(is_refund ? "refund" : "no refund");
    } else if (rc == THINKTHEN_UNSURE) {
        puts("not sure");
    } else {
        report(&error);
    }
    thinkthen_row_free(&row);
    thinkthen_error_free(&error);
    thinkthen_question_free(refund);
    thinkthen_client_free(client);
    return rc < 0 ? EXIT_FAILURE : EXIT_SUCCESS;
}
```

`thinkthen_run_one` returns `THINKTHEN_OK` with the answer in the row, `THINKTHEN_UNSURE` when the model is not sure, or the record's own negative code when it fails. Read `row.answer_bool`, `row.number` or `row.rank` only after `THINKTHEN_OK`; on any other return `answer_bool` is 0, which is not an answer.

`thinkthen_question_set_yes_means` and `thinkthen_question_set_no_means` tell the model what yes and no mean for a decide or filter question. For choose, tag, score and rank, add the option first, then describe it with `thinkthen_question_set_description(question, "billing", THINKTHEN_NUL_TERMINATED, "charges and refunds", THINKTHEN_NUL_TERMINATED, &error)`. Decide and filter take a band; `thinkthen_question_set_cut` sets a single cut instead.

## C: keep only the matching records

A filter call judges every record. `thinkthen_call_next_kept` returns only the records it keeps:

```c
#include <thinkthen.h>
#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>

static void report(const thinkthen_error *error) {
    fprintf(stderr, "%s: ", thinkthen_code_name(error->code));
    fwrite(error->message.data, 1, error->message.len, stderr);
    fputc('\n', stderr);
}

static void print_message(const thinkthen_row *row) {
    thinkthen_record_view message;
    if (thinkthen_row_original(row, &message, NULL) == THINKTHEN_OK) {
        fwrite(message.data, 1, message.len, stdout);
        putchar('\n');
    }
}

static const thinkthen_record_input messages[] = {
    {.struct_size = sizeof(thinkthen_record_input),
     .kind = THINKTHEN_RECORD_TEXT, .data = "Please refund my order.",
     .len = THINKTHEN_NUL_TERMINATED},
    {.struct_size = sizeof(thinkthen_record_input),
     .kind = THINKTHEN_RECORD_TEXT, .data = "Where is my parcel?",
     .len = THINKTHEN_NUL_TERMINATED},
    {.struct_size = sizeof(thinkthen_record_input),
     .kind = THINKTHEN_RECORD_TEXT, .data = "I want my money back.",
     .len = THINKTHEN_NUL_TERMINATED},
};

int main(void) {
    thinkthen_client *client = NULL;
    thinkthen_question *asks_refund = NULL;
    thinkthen_call *call = NULL;
    thinkthen_row refund_request = THINKTHEN_ROW_INIT;
    thinkthen_error error = THINKTHEN_ERROR_INIT;
    size_t count = sizeof messages / sizeof messages[0];
    int rc = thinkthen_client_open(NULL, &client, &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_question_new(THINKTHEN_FUNCTION_FILTER,
                                    "Does the customer ask for a refund?",
                                    THINKTHEN_NUL_TERMINATED, &asks_refund,
                                    &error);
    if (rc == THINKTHEN_OK)
        rc = thinkthen_call_open_records_question(
            client, asks_refund, NULL, messages, count, sizeof messages[0],
            THINKTHEN_OFFSET_UTF8, &call, &error);
    /* Each pass frees the row it held, so one variable serves the loop. */
    while (rc == THINKTHEN_OK
           || refund_request.status == THINKTHEN_STATUS_FAILED) {
        rc = thinkthen_call_next_kept(call, &refund_request, &error);
        if (rc == THINKTHEN_OK) {
            print_message(&refund_request);
        } else if (refund_request.status == THINKTHEN_STATUS_FAILED) {
            fprintf(stderr, "message %" PRIu64 " failed: ",
                    refund_request.position);
            report(&error);
        }
    }
    if (rc != THINKTHEN_END)
        report(&error);
    thinkthen_row_free(&refund_request);
    thinkthen_error_free(&error);
    thinkthen_call_free(call);
    thinkthen_question_free(asks_refund);
    thinkthen_client_free(client);
    return rc == THINKTHEN_END ? EXIT_SUCCESS : EXIT_FAILURE;
}
```

`thinkthen_call_open_records_question` takes an array of `thinkthen_record_input`, its count and `sizeof` one element. A record is `THINKTHEN_RECORD_TEXT`, `THINKTHEN_RECORD_JSON`, `THINKTHEN_RECORD_MISSING` or `THINKTHEN_RECORD_INVALID`; missing and invalid records need no bytes. The call copies the records before it returns.

`thinkthen_call_next_kept` returns `THINKTHEN_OK` with a kept row, and skips the records the model says no to or is not sure about. `thinkthen_row_original` gives a kept row's input. A failed record returns its own negative code, with its row (status `THINKTHEN_STATUS_FAILED`, with its `position`) and its message in the error, and the records after it can still be read. `THINKTHEN_END` means every record is judged. Any other negative return, such as `-THINKTHEN_CODE_LOCAL_CANCELLED`, ends the call. `thinkthen_call_next` is the view of every row, including no, not sure and failed rows.

For records that arrive over time, open with `thinkthen_call_open_question`, send each record with `thinkthen_call_send_text`, `thinkthen_call_send_json`, `thinkthen_call_send_missing` or `thinkthen_call_send_invalid`, and call `thinkthen_call_end_input` once the source ends. A read returns `THINKTHEN_NEEDS_INPUT` when the call waits for the next record.

## C ownership and calls

Functions return `THINKTHEN_OK` (0) on success or the negative of a `thinkthen_code` on failure. Some also return `THINKTHEN_UNSURE`, or `THINKTHEN_END` and `THINKTHEN_NEEDS_INPUT` while reading a call; the header lists each function's returns. Write `rc != THINKTHEN_OK`, never `rc == -1`. Each function that returns a status is marked `THINKTHEN_NODISCARD`, so the compiler warns when a status is ignored.

The last argument is a `thinkthen_error *`, which may be NULL. Otherwise start it with `THINKTHEN_ERROR_INIT`; a failure fills its `code` and `message`. Start each row with `THINKTHEN_ROW_INIT`. Passing an error or row again frees what it held, so reusing one variable never leaks; free it with `thinkthen_error_free` or `thinkthen_row_free` after its last use. A `thinkthen_buffer` output is overwritten, not freed, so free it with `thinkthen_buffer_free` before passing it again. A handle output, such as `thinkthen_value **`, is overwritten too.

Text is UTF-8, passed as a pointer and a byte length. Pass `THINKTHEN_NUL_TERMINATED` as the length for a NUL-terminated string. The library copies every input before it returns, except that the `thinkthen_span` from `thinkthen_find` points into your text, so keep that text until you are done with the span. Text it returns (`thinkthen_text`) is not NUL-terminated; print it with its length.

Settings, clients, questions, toolboxes, calls and values each have one free function: `thinkthen_settings_free`, `thinkthen_client_free`, `thinkthen_question_free`, `thinkthen_toolbox_free`, `thinkthen_call_free` and `thinkthen_value_free`. Each accepts NULL. A view such as `thinkthen_text`, `thinkthen_span` or `thinkthen_record_view` borrows from the value or row it came from and lasts until that is freed.

A client may be shared by every thread. A question is changed by one thread at a time; once built, many threads may read it. One thread drives a call at a time, and `thinkthen_call_cancel` may run on any thread.

`THINKTHEN_VERSION` is the header's version, such as `"0.2.0"`, and `thinkthen_version()` the loaded library's. `THINKTHEN_ABI_MAJOR` and `THINKTHEN_ABI_MINOR` name the binary interface the header needs. Check `thinkthen_abi_compatible(THINKTHEN_ABI_VERSION, thinkthen_abi_version())` at startup to refuse a library with another major version or an older minor one.

Construct questions with `thinkthen_question_new(function, text, length, &question, &error)`.

| Function constant suffix | Required question additions | Answer access on typed value |
| --- | --- | --- |
| DECIDE, FILTER | Wording | thinkthen_value_bool |
| CHOOSE | Wording, thinkthen_question_add_choice | thinkthen_value_choice |
| TAG | Wording, thinkthen_question_add_label | thinkthen_value_tag_count / thinkthen_value_tag_at |
| SCORE | Wording, thinkthen_question_add_level | thinkthen_value_number |
| RANK | Wording, thinkthen_question_add_level | thinkthen_value_rank |
| FIND | Wording | thinkthen_value_found |
| RECOGNIZE | thinkthen_question_add_kind | thinkthen_value_name_count / thinkthen_value_name_at |
| RELATE | thinkthen_question_add_rule | thinkthen_value_relation_count / thinkthen_value_relation_at |
| ANNOTATE | thinkthen_question_add_entry | thinkthen_value_annotation_get, or thinkthen_value_annotation_count / thinkthen_value_annotation_at |

All constants start `THINKTHEN_FUNCTION_`. Recognize, relate and annotate use no wording. Relation rules have source/target kinds, relation name and optional reads/either. Annotation entries are named per-record questions; rank and annotate cannot be children.

`thinkthen_run_one` runs one question over a text, JSON, missing or invalid record and writes its row. Its span offsets count UTF-8 bytes from zero with an exclusive end. A call's offsets follow the unit you open it with; `THINKTHEN_OFFSET_UTF8` counts bytes.

`thinkthen_row_values` copies a row's answer into a `thinkthen_value` that outlives the row. Accessors borrow its strings and collections until `thinkthen_value_free`. Scalar answers are Boolean, choice string, score double and rank uint64; tags are labels, find is a span with a present flag, recognize is names, relate is relations, and annotate is named answers. An answered find may have no match. Each answer accessor returns `THINKTHEN_UNSURE` on a not-sure value and the record's own negative code on a failed one, and writes nothing in either case. The row also exposes serialized answer/detail/source buffers; treat buffers as counted bytes.

Use `thinkthen_value_id` for exact typed string/signed/unsigned IDs. Source identifies the question, backend and served model. Detail accessors expose label/level probabilities, recognition pieces and relation pairs. Filter rows retain original bytes through `thinkthen_row_original` or the snapshot's original accessor. Call `thinkthen_value_status` on each annotation answer, or check its accessor's return; an answered annotation can hold failed or not-sure answers.

## C++: one call per answer

Include `<thinkthen/thinkthen.hpp>`. This program asks one yes-or-no question about one text:

```cpp
#include <thinkthen/thinkthen.hpp>
#include <iostream>

namespace tt = thinkthen;

int main() {
    try {
        tt::client client;
        const auto is_refund = client.decide(
            tt::decide{"Does the customer request a refund?"},
            "Please refund my order.");
        std::cout << std::boolalpha << is_refund.value() << '\n';
    } catch (const std::exception& error) {
        std::cerr << error.what() << '\n';
        return 1;
    }
}
```

`client.decide` takes a question and a string, copies the string, and returns one row; name it for what its answer means. `value()` returns a reference to the answer, here a `bool`. It throws `tt::unsure` when the model is not sure and `tt::error` when the record failed, so a failure never reads as no. A decide or filter row also tests as its answer: `if (is_refund)` takes the yes branch only on a yes and throws as `value()` does. `answer()` returns a pointer to the answer that is null when there is none, and `unsure()` and `failed()` test the status without throwing.

A client with default settings reads the environment: `THINKTHEN_BACKEND` picks the backend, and the backend's own variables give the key and URL. `systemone`, the default, reads `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL`; `openai` reads `OPENAI_API_KEY`. To fix them in code, fill a `tt::settings` (`backend`, `model`, `url`, `api_key`, `batch`, `retry`) and pass it to the client. An unknown backend name or a missing key throws `tt::error` from the client's constructor.

`tt::error` derives from `std::system_error`. `code()` is a `std::error_code` whose `message()` is the code's name, such as `record.empty`, and which compares with `tt::errc` values. `what()` is the engine's message, unchanged, and `http_status()` holds the backend's HTTP status when a reply gave one. Blank text throws `record.empty` from the call and is never sent.

## C++: not sure

A band lets the model say it is not sure. Below its low end is no, at or above its high end is yes, and between is not sure:

```cpp
#include <thinkthen/thinkthen.hpp>
#include <iostream>

namespace tt = thinkthen;

int main() {
    try {
        tt::client client;
        const auto refund =
            tt::decide{"Does the customer request a refund?"}
                .yes_means("the customer wants their money back")
                .no_means("the customer wants something else")
                .threshold(tt::threshold::band(0.3, 0.7));
        const auto is_refund = client.decide(refund, "I may send it back.");
        try {
            std::cout << (is_refund ? "refund" : "no refund") << '\n';
        } catch (const tt::unsure& not_sure) {
            std::cout << "not sure";
            if (const auto probability = not_sure.probability())
                std::cout << ", probability " << *probability;
            std::cout << '\n';
        }
    } catch (const std::exception& error) {
        std::cerr << error.what() << '\n';
        return 1;
    }
}
```

`tt::unsure` derives from `std::exception`, not from `tt::error`, so a handler for engine errors never takes a not-sure answer. Its `probability()` is the model's probability when it gave one.

`yes_means` and `no_means` tell the model what yes and no mean for a decide or filter question. Choose, tag, score and rank take `descriptions`, a `std::map` from each choice, label or level to its meaning. `threshold(0.7)` sets a single cut instead of a band. Decide and filter take a cut or a band; choose, tag, find, recognize and relate take a cut. Each of these methods returns a new question and leaves the one it was called on unchanged.

## C++: keep only the matching records

`filter_each` judges every record and yields only the ones the model keeps:

```cpp
#include <thinkthen/thinkthen.hpp>
#include <iostream>
#include <string>
#include <variant>
#include <vector>

namespace tt = thinkthen;

int main() {
    try {
        tt::client client;
        const tt::filter asks_refund{"Does the customer request a refund?"};
        const std::vector<std::string> messages{"Please refund my order.",
                                                "Where is my parcel?",
                                                "I want my money back."};
        for (const auto& request :
             client.filter_each(asks_refund, messages))
            std::cout << std::get<tt::owned_text>(request).value << '\n';
    } catch (const tt::error& error) {
        std::cerr << error.code().message() << ": " << error.what() << '\n';
        for (const auto& failed : error.failed_rows())
            std::cerr << "message " << failed.position << ": "
                      << failed.message << '\n';
        return 1;
    }
}
```

Each kept record comes back as the `tt::owned_record` you passed in: a string comes back as `tt::owned_text`, and JSON as `tt::owned_json_text` with its bytes unchanged. No and not-sure records are left out. A failed record does not stop the stream: it reads every record and yields every kept one, then throws one `tt::error` with the first failure's code and message. Its `failed_rows()` lists every failed record with its `position`, `id_json`, `code`, `message`, `http_status` and original `record`.

`filter_rows_each` is the view of every row, in input order, including no, not sure and failed. `kept()` is true only for an answered yes, and `item()` is the original record.

## C++ ownership and calls

Every function has a one-record method, such as `decide` or `choose`, and an `_each` method that streams one row per record. The `_each` methods take a `std::vector` of strings, an iterator range of strings or owned records, a `std::vector<tt::owned_record>`, or a `tt::record_source`, a function that returns the next record or `std::nullopt` at the end. A stream reads once, closes its call when it is destroyed, and `close()` stops it early. `cancel_handle()` gives a handle whose `request_cancel()` may run on another thread.

A whole JSON null record, a record without the selected field, and a selected field holding JSON null, such as `{"text":null}` with `/text`, each return a failed `record.missing` row locally and send no HTTP request for that record. A JSON array has no `/text` field, so a one-record call such as `client.decide` on an array returns a failed `record.missing` row whose message says to use the stream form; pass the array's values to an `_each` method to judge each one. Invalid records return failed `record.*` rows. Pass every record to the collection call to retain false answers and failed rows.

Create `records.jsonl` with one JSON value per line. For example, put `{"id":"a1","text":"Please refund my order."}` and `{"id":"a2","text":"Where is my parcel?"}` on separate lines. Add `null`, `{"id":"a4"}` and `{"id":"a6","text":null}` on separate lines to compare the null cases. The example first shows a one-record call refusing `["first", "second"]`, then reads the file as a collection of separate records. Keep every collection outcome, including answered false and failed rows.

```cpp
#include <thinkthen/thinkthen.hpp>
#include <fstream>
#include <iostream>
#include <stdexcept>
#include <string>
#include <vector>

namespace tt = thinkthen;

int main() {
    try {
        std::ifstream source("records.jsonl");
        if (!source) throw std::runtime_error("cannot open records.jsonl");
        std::vector<tt::owned_record> records;
        for (std::string line; std::getline(source, line);) {
            if (line.find_first_not_of(" \t\r") != std::string::npos)
                records.push_back(tt::owned_json_text{line});
        }
        tt::client client;
        const auto refund_in_text =
            tt::decide{"Does the customer request a refund?"}.field("/text");
        const auto whole_array = client.decide(
            refund_in_text, tt::json_text{R"(["first","second"])"});
        std::cout << "array " << whole_array.status();
        if (const tt::error* refused = whole_array.failure())
            std::cout << ' ' << refused->what();
        std::cout << '\n';
        for (const auto& is_refund :
             client.decide_each(refund_in_text, records)) {
            std::cout << is_refund.position() << ' ' << is_refund.status();
            if (const bool* answer = is_refund.answer())
                std::cout << ' ' << std::boolalpha << *answer;
            if (const tt::error* failure = is_refund.failure())
                std::cout << ' ' << failure->what();
            std::cout << '\n';
        }
    } catch (const std::exception& error) {
        std::cerr << error.what() << '\n';
        return 1;
    }
}
```

C++ offers typed question classes `decide`, `choose`, `tag`, `score`, `filter`, `rank`, `find`, `recognize`, `relate` and `annotate`. Required domains go into question constructors:

```cpp
tt::choose topic{"Which topic?", {"billing", "shipping", "other"}};
tt::score urgency{"How urgent is the request?", {"routine", "urgent"}};
tt::recognize names{{"person", "organization"}};
tt::relate links{std::vector<tt::rule>{
    {"works_for", "person", "organization"}}};
tt::annotate notes{tt::annotation_questions{
    {"refund", tt::decide{"Does the customer request a refund?"}}}};
```

Question `.field("/text")` returns a new question that selects a field in a JSON record. A one-record method takes a `std::string_view`, which it copies, or a `tt::record`: `tt::text` for text, `tt::json_text` for encoded JSON, or `tt::missing_record` for an absent record. `tt::owned_record` holds `tt::owned_text`, `tt::owned_json_text` or `tt::missing_record` and owns its bytes. Rows own immutable copies and expose `value()`, `answer()`, `status()`, `probability()`, `failure()` for a record error, typed `detail()`, `source()`, `position()` and `id_json()` for an exact encoded ID. Filter and rank rows expose `item()`, the original record. `tt::get<tt::decide>(answers, "refund")` reads one annotation answer as the question type it was built with; check each one's status.

The ten answer shapes match C: bool, string, vector of strings, double score, uint64 rank, optional span, vector of names, vector of relations and named outcomes. Spans count UTF-8 bytes from zero with an exclusive end. Find selects one whole nonblank line; preserve line breaks.

A rank row's answer is its rank, starting at 1 for the highest score; its position is its input order, and rows can come back in rank order. Rank reads all input and keeps failures when top limits successful ranks. Score/rank levels go lowest to highest. `filter_rows_each` returns every outcome, including false, unsure and failed.

## Batching, saved questions and files

Pass one finite collection or record source per call to let the engine batch. Client batch settings control packing; byte and backend limits also apply. One record can require several backend questions. Avoid loops of single calls when a stream fits.

C uses `thinkthen_toolbox_from_json` for saved named questions or toolbox insert/select functions for typed construction. C++ toolbox stores typed questions, loads saved entries with `toolbox::from_json`, saves with `to_json()` and retrieves typed snapshots with `select<Q>(name)`. Neither surface executes shell toolbox runs. Use host file/table libraries to read data and write CSV or spreadsheets. C++ users of nlohmann/json can include `<thinkthen/nlohmann.hpp>` for record adapters and typed row serialization. Preserve exact IDs and all statuses in saved results.

Decide and filter accept numeric cuts or low/high bands. C uses question_set_cut/question_set_band, or `thinkthen_decide_band` for a decide band in one call; C++ uses `.threshold(value)` or `.threshold(thinkthen::threshold::band(low, high))`. Choose, tag, find, recognize and relate use cuts only. A decide/filter band yields false below low, unsure inside and true at or above high; choose needs its winner to reach the cut.

Call relate alone when you need names and relations from the same field. A C relate snapshot exposes all admitted names through `thinkthen_value_name_count` and `thinkthen_value_name_at`. C++ relation detail exposes `names` and `pairs`. Names keep the existing recognized-name shape and include names with no returned relation. Read accepted relations from the answer; do not run recognize first.

## Save and load questions

A saved toolbox uses named question entries. Save this as `toolbox.json`:

```json
{"entries":{"ready":{"question":{"function":"filter","question":"Does the customer request a refund?","field":"/text"}},"topic":{"question":{"function":"choose","question":"What is the message about?","choices":["billing","shipping","other"],"field":"/text"}}}}
```

Load the entries once, select a typed question and pass the whole collection to its matching call. Saving the toolbox preserves question definitions and their field selectors. SDK toolboxes contain questions; command shell steps and runs belong to the command's richer saved-run format.

```cpp
std::ifstream input("toolbox.json");
if (!input) throw std::runtime_error("cannot open toolbox.json");
const std::string json{std::istreambuf_iterator<char>(input),
                       std::istreambuf_iterator<char>()};
const auto box = tt::toolbox::from_json(json);
std::ofstream output("toolbox-copy.json");
output << box.to_json();
if (!output) throw std::runtime_error("cannot write toolbox-copy.json");
const auto topic = box.select<tt::choose>("topic");
for (const auto& message_topic : client.choose_each(topic, records)) {
    if (const tt::error* failure = message_topic.failure()) {
        std::cerr << message_topic.position() << ' ' << message_topic.id_json()
                  << ' ' << failure->code().message() << ": "
                  << failure->what() << '\n';
        continue;
    }
    std::cout << message_topic.position() << ' ' << message_topic << '\n';
}
```

This C++ code belongs inside the `records.jsonl` example's `try` block, after its client and records; add `<iterator>` for the file iterator. `box.select<tt::filter>("ready")` selects the saved filter for `filter_each`. Typed selection reports a shared usage error for a missing name or wrong function.

C callers pass counted file bytes to `thinkthen_toolbox_from_json(data, len, &tools, &error)` and save with `thinkthen_toolbox_to_json(tools, &buffer, &error)`. Select a typed snapshot with `thinkthen_toolbox_select_as(tools, name, name_len, THINKTHEN_FUNCTION_FILTER, &question, &error)` or the CHOOSE function constant. Check every return code before use. Write the returned buffer's counted bytes with your application file IO. Free saved buffers with `thinkthen_buffer_free`, selected questions with `thinkthen_question_free` and the toolbox with `thinkthen_toolbox_free`.

## Failures and cancellation

A record/backend failure returns a failed row in a stream. A C single call returns that record's own negative code instead, and a C++ single call returns the failed row. Inspect it before saving or chaining. Stop and close a stream if the task requires stopping after failure. Inspect nested annotation statuses independently. C call-level errors use the return code and thinkthen_error; C++ throws thinkthen::error with stable code and library message.

C cancels with `thinkthen_call_cancel`; C++ streams expose a cancel handle with `request_cancel()`. Close C++ streams on early exit and free C calls with `thinkthen_call_free`. Free questions, settings, clients and typed snapshots with their matching functions. No borrowed projection may outlive its owner.

Make questions unambiguous: define the field criterion, distinguish absence from explicit negatives and use distinct choices.
