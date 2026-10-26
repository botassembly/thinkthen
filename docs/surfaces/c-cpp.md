# ThinkThen in C and C++

C uses run_one for one row and call_open/call_next for streams. C++ named methods return one row and `_each` methods return streams. A JSON array inside one record remains one record. C frees calls explicitly; C++ stream owners close calls, and `close()` ends early traversal.

Check `share/thinkthen-c/README.md` and `share/thinkthen-cpp/README.md` inside their archives for the supplied package's operating-system floor and runtime requirements before installation. Use the package for your operating system and architecture.

Extract the supplied C 0.2.0 archive into a `thinkthen` prefix. C++ also needs the supplied matching C++ archive extracted into that prefix. C requires a C11 compiler and pkg-config; C++ requires C++17. The C archive includes the native engine. Keep its headers and library together.

Compile C with `cc -std=c11 example.c -o example $(PKG_CONFIG_PATH="$PWD/thinkthen/lib/pkgconfig" pkg-config --cflags --libs thinkthen-c)`. Compile C++ with `c++ -std=c++17 -Ithinkthen/include example.cpp -Lthinkthen/lib -lthinkthen -Wl,-rpath,"$PWD/thinkthen/lib" -o example`. The supplied CMake configuration offers ThinkThen::Cpp.

Set `THINKTHEN_API_KEY` in the environment before creating a normal client. Keep keys out of source and reports. Omitted settings select the default backend. The stub needs no key or network and returns synthetic installation-check answers.

## C ownership and calls

Include `<thinkthen.h>`. Check `thinkthen_abi_compatible(THINKTHEN_ABI_VERSION, thinkthen_abi_version())` before using the library. Initialize every sized output with `.struct_size = sizeof output`. Settings and question setters copy counted input bytes; none require a terminating NUL.

Create settings with `thinkthen_settings_new`, select `THINKTHEN_BACKEND_STUB` with `thinkthen_settings_set_backend` for an installation check, then open a client with `thinkthen_client_open`. Construct questions with `thinkthen_question_new(function, bytes, length, &question, &error)`.

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
| ANNOTATE | thinkthen_question_add_entry | thinkthen_value_annotation_count / thinkthen_value_annotation_at |

All constants start `THINKTHEN_FUNCTION_`. Recognize, relate and annotate use no wording. Relation rules have source/target kinds, relation name and optional reads/either. Annotation entries are named per-record questions; rank and annotate cannot be children.

`thinkthen_run_one` runs one typed question over a text, JSON, missing or invalid record kind and returns a row. Its span offsets count Unicode code points from zero with an exclusive end. For a finite collection, open with `thinkthen_call_open_records_question` and a thinkthen_record_input array. Choose `THINKTHEN_OFFSET_UTF8` for byte offsets. For incremental input use `thinkthen_call_open_question`, pull with `thinkthen_call_next`, send a record on THINKTHEN_NEEDS_INPUT, and call `thinkthen_call_end_input` once the source ends. THINKTHEN_OK means a row; THINKTHEN_END means exhaustion. Other return codes are call errors. Free each output row before reusing its slot.

`thinkthen_row_values` creates an independently owned typed value snapshot. Accessors borrow its strings and collections until `thinkthen_value_free`. Check status before reading an answer. Scalar answers are Boolean, choice string, score double and rank uint64; tags are labels, find is a span with a present flag, recognize is names, relate is relations, and annotate is named typed outcomes. An answered find may have no match. The row also exposes serialized answer/detail/source buffers; treat buffers as counted bytes. Free rows with `thinkthen_row_free` and errors with `thinkthen_error_free`.

Use `thinkthen_value_id` for exact typed string/signed/unsigned IDs. Source identifies the question, backend and served model. Detail accessors expose label/level probabilities, recognition pieces and relation pairs. Filter rows retain original bytes through `thinkthen_row_original` or the snapshot's original accessor. Call `thinkthen_value_status` on each annotation child; an answered outer annotation can hold failed entries.

## C++ ownership and calls

A whole JSON null record or a record without the selected evidence field returns a failed `record.missing` row locally and sends no HTTP request for that record. A selected field containing JSON null, such as `{"text":null}` with `/text`, sends that null value to the backend. Invalid records return failed `record.*` rows. Pass every record to the collection call to retain false answers and failed rows.

Create `records.jsonl` with one JSON value per line. For example, put `{"id":"a1","text":"Please refund my order."}` and `{"id":"a2","text":"Where is my parcel?"}` on separate lines. Add `null`, `{"id":"a4"}` and `{"id":"a6","text":null}` on separate lines to compare the null cases. The example first judges `["first", "second"]` as one JSON array record, then reads the file as a collection of separate records. Both calls use the stub backend. Keep every collection outcome, including answered false and failed rows.

```cpp
#include <thinkthen/thinkthen.hpp>
#include <fstream>
#include <iostream>
#include <vector>

int main() {
    try {
        std::ifstream source("records.jsonl");
        if (!source) throw std::runtime_error("cannot open records.jsonl");
        std::vector<thinkthen::owned_record> records;
        for (std::string line; std::getline(source, line); ) {
            if (line.find_first_not_of(" \t\r") != std::string::npos)
                records.push_back(thinkthen::owned_json_text{line});
        }
        thinkthen::settings settings;
        settings.backend = thinkthen::backend::stub;
        thinkthen::client client(settings);
        thinkthen::decide question{"Does the customer request a refund?"};
        auto array = client.decide(question, thinkthen::json_text{R"(["first","second"])"});
        std::cout << "array " << (array.failure() ? "failed" : "completed") << '\n';
        question = question.evidence("/text");
        auto rows = client.decide_each(question, records);
        while (auto row = rows.next()) {
            std::cout << row->position() << ' ' << (row->failure() ? "failed" : "completed") << ' '
                << (row->value() ? (*row->value() ? "true" : "false") : "null") << ' '
                << (row->failure() ? row->failure()->what() : "") << '\n';
        }
    } catch (const std::exception& error) {
        std::cerr << error.what() << '\n';
        return 1;
    }
}
```

C++ offers typed question classes `decide`, `choose`, `tag`, `score`, `filter`, `rank`, `find`, `recognize`, `relate` and `annotate`. Call the matching client method with a typed question and one record, or its `_each` form with a vector of owned_record or a record_source. Required domains go into question constructors:

```cpp
thinkthen::choose topic{"Which topic?", {"billing", "shipping", "other"}};
thinkthen::score urgency{"How urgent is the request?", {"routine", "urgent"}};
thinkthen::recognize names{{"person", "organization"}};
thinkthen::relate links{std::vector<thinkthen::rule>{{"works_for", "person", "organization", std::nullopt, false}}};
thinkthen::annotate notes{thinkthen::annotation_questions{{"refund", thinkthen::decide{"Does the customer request a refund?"}}}};
```

Question `.evidence("/text")` returns a new question that selects a field in a JSON record. Retain the returned question when applying a modifier. Text uses text/owned_text; encoded JSON uses json_text/owned_json_text; missing_record marks absence. An owned_record copies its bytes. Use the right lifetime for borrowed records. C++ rows own immutable snapshots and expose `value()` for answered payloads, `failure()` for record errors, typed detail, source, position and `id_json()` for an exact encoded ID. Filter rows expose `item()` and `kept()`. Inspect each nested annotation outcome.

The ten answer shapes match C: bool, string, vector of strings, double score, uint64 rank, optional span, vector of names, vector of relations and named outcomes. Spans count UTF-8 bytes from zero with an exclusive end. Find selects one whole nonblank line; preserve line breaks.

A rank row's answer is its rank; its position is its input order. Rank reads all input, puts highest scores first and keeps failures when top limits successful ranks. Score/rank levels go lowest to highest. A filter returns every outcome, including false, unsure and failed.

## Batching, saved questions and files

Pass one finite collection or record source per call to let the engine batch. Client batch settings control packing; byte and backend limits also apply. One record can require several backend questions. Avoid loops of single calls when a stream fits.

C uses `thinkthen_toolbox_from_json` for saved named questions or toolbox insert/select functions for typed construction. C++ toolbox stores typed questions, loads saved entries with `toolbox::from_json`, saves with `to_json()` and retrieves typed snapshots with `select<Q>(name)`. Neither surface executes shell toolbox runs. Use host file/table libraries to read data and write CSV or spreadsheets. C++ users of nlohmann/json can include `<thinkthen/nlohmann.hpp>` for record adapters and typed row serialization. Preserve exact IDs and all statuses in saved results.

Decide, choose and filter accept numeric cuts or low/high bands. C uses question_set_cut/question_set_band; C++ uses `.cutoff(thinkthen::threshold::cutoff(value))` or `.cutoff(thinkthen::threshold::band(low, high))`. Tag, find, recognize and relate use cuts only. A decide/filter band yields false below low, unsure inside and true at or above high; choose needs its winner to reach the upper cut.

Call relate alone when you need names and relations from the same evidence. A C relate snapshot exposes all admitted names through `thinkthen_value_name_count` and `thinkthen_value_name_at`. C++ relation detail exposes `names` and `pairs`. Names keep the existing recognized-name shape and include names with no returned relation. Read accepted relations from the answer; do not run recognize first.

## Save and load questions

A saved toolbox uses named question entries. Save this as `toolbox.json`:

```json
{"entries":{"ready":{"question":{"function":"filter","question":"Does the customer request a refund?","evidence":"/text"}},"topic":{"question":{"function":"choose","question":"What is the message about?","choices":["billing","shipping","other"],"evidence":"/text"}}}}
```

Load the entries once, select a typed question and pass the whole collection to its matching call. Saving the toolbox preserves question definitions and their evidence selectors. SDK toolboxes contain questions; command shell steps and runs belong to the command's richer saved-run format.

```cpp
std::ifstream input("toolbox.json");
if (!input) throw std::runtime_error("cannot open toolbox.json");
const std::string json{std::istreambuf_iterator<char>(input), std::istreambuf_iterator<char>()};
auto box = thinkthen::toolbox::from_json(json);
std::ofstream output("toolbox-copy.json");
output << box.to_json();
if (!output) throw std::runtime_error("cannot write toolbox-copy.json");
auto topic = box.select<thinkthen::choose>("topic");
auto choices = client.choose_each(topic, records);
while (auto row = choices.next()) {
    if (row->failure()) throw *row->failure();
    if (const auto* answer = row->value()) std::cout << *answer << '\n';
}
```

This C++ code belongs inside the first example's `try` block with its client and records; add `<iterator>` for the file iterator. `box.select<thinkthen::filter>("ready")` selects the saved filter for `filter_each`. Typed selection reports a shared usage error for a missing name or wrong function.

C callers pass counted file bytes to `thinkthen_toolbox_from_json(data, len, &tools, &error)` and save with `thinkthen_toolbox_to_json(tools, &buffer, &error)`. Select a typed snapshot with `thinkthen_toolbox_select_as(tools, name, name_len, THINKTHEN_FUNCTION_FILTER, &question, &error)` or the CHOOSE function constant. Check every return code before use. Write the returned buffer's counted bytes with your application file IO. Free saved buffers with `thinkthen_buffer_free`, selected questions with `thinkthen_question_free` and the toolbox with `thinkthen_toolbox_free`.

## Failures and cancellation

A record/backend failure returns a failed row, including a single call. Inspect it before saving or chaining. Stop and close a stream if the task requires stopping after failure. Inspect nested annotation statuses independently. C call-level errors use the return code and thinkthen_error; C++ throws thinkthen::error with stable code and library message.

C cancels with `thinkthen_call_cancel`; C++ streams expose a cancel handle with `request_cancel()`. Close C++ streams on early exit and free C calls with `thinkthen_call_free`. Free questions, settings, clients and typed snapshots with their matching functions. No borrowed projection may outlive its owner.

Make questions unambiguous: define the evidence criterion, distinguish absence from explicit negatives and use distinct choices. The stub checks installation, not question quality.
