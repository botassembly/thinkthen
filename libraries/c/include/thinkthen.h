/*
 * thinkthen.h — the C door to the thinkthen engine, version 0.2.0.
 *
 * One archive per platform ships this header, the shared library, and the
 * static library. Every language that cannot bind Rust
 * directly binds this door: the JSON door carries any request, and the
 * typed decide and decide-many calls carry the hot path without building
 * request JSON in the host. Prefer their `*_with_facts` forms for final
 * call facts; the older typed names remain bare ABI compatibility forms.
 *
 * The prefix is `thinkthen_` on every name, and the public word for the
 * middle answer is UNSURE. An engine value carries its settings, holds no
 * thread between calls, and rebuilds its state after a fork.
 *
 * Every call may carry two options beside its arguments: a budget in
 * `deadline_ms` and a cancel token. Every `_opts` spelling takes them,
 * and every plain spelling is exactly its `_opts` twin called with
 * THINKTHEN_NO_DEADLINE and a null token. A C host hears its own
 * interrupts by firing a token from another thread. The call reads the
 * token before every request and every retry, and its wait reads the
 * token and the budget on every tick. A spent budget ends the call
 * within one tick. A fired token starts no new request and no retry. A
 * request already sent runs to its end, within its attempt timeout and
 * the budget, and a complete answer it brings still reaches the cache
 * and the counters. Then the call returns THINKTHEN_ECANCELLED with no
 * results, whatever that reply held. The call reads the token a last
 * time after its last request ends; a token fired after that read does
 * not change the result. One engine serves any number of threads at
 * once, and each caller sees the answers it would get alone.
 *
 * The lifetime rules: an engine lives until `thinkthen_engine_free`; a
 * string from `thinkthen_call`, `thinkthen_question_file`, `thinkthen_plan_json`,
 * `thinkthen_recognize`, `thinkthen_relate`, or a typed `*_with_facts` form lives until
 * `thinkthen_free_string`; a cancel token
 * lives until `thinkthen_cancel_token_free`, and no call may carry a token
 * the host has freed; the message from `thinkthen_error_message` belongs
 * to the calling thread and lives until that thread records its next
 * failure, so no other thread's calls replace it.
 *
 * The failure rule: a nonzero return leaves every out parameter holding
 * what it held before the call, and `thinkthen_error_code`,
 * `thinkthen_error_message`, and `thinkthen_error_retryable` name the
 * failure. `thinkthen_error_facts_json` names final facts when the failed
 * call started; it is NULL for a pre-call refusal. The eager calls deliver no partial rows: a cancelled or deadline-expired
 * bulk call returns its code with no rows. The additive lazy batch calls below retain their completed prefix. With a null engine, the error accessors
 * name the calling thread's last failed `thinkthen_engine_new` until that
 * thread's next engine builds.
 *
 * Version 0.1.0 freezes the symbol names, the `thinkthen_answer` layout,
 * the argument types (exact-width `size_t` lengths and counts, `int64_t`
 * budgets), and the return codes. Its open-shaped results cross as JSON
 * text and its `thinkthen_answer` layout never grows a field. Additive
 * counted input handles and typed carrier layouts appear below.
 */

#ifndef THINKTHEN_H
#define THINKTHEN_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Version 0.2.0, the version of the library this header ships with.
 * Version 0.1.0 is the first release. */
#define THINKTHEN_VERSION_MAJOR 0
#define THINKTHEN_VERSION_MINOR 2
#define THINKTHEN_VERSION_PATCH 0

/*
 * The return codes, one per error kind, and zero for success. A failure
 * never reads as an answer: on any nonzero code every out parameter holds
 * what it held before the call, and `thinkthen_error_code`,
 * `thinkthen_error_message`, and `thinkthen_error_retryable` name what
 * failed.
 */
#define THINKTHEN_OK 0
#define THINKTHEN_EUSAGE 1      /* the arguments/input broke the grammar; the rejected stage was not sent */
#define THINKTHEN_EBACKEND 2   /* the wire failed or refused */
#define THINKTHEN_EDEADLINE 3  /* the caller's own budget ran out; no rows came */
#define THINKTHEN_ELOCAL 4     /* a named local file, cache, or recording failed */
#define THINKTHEN_ECANCELLED 5 /* the token fired; sent requests finished and no rows came */
#define THINKTHEN_EDEFECT 6    /* the engine broke its own contract */

/*
 * The budget an `_opts` call carries, in milliseconds, an `int64_t` so
 * every value and the sentinel are exact on every platform. A positive
 * value is the budget from the call; zero is a spent budget, so the call
 * refuses before anything is sent with THINKTHEN_EDEADLINE;
 * THINKTHEN_NO_DEADLINE sets none; and any other negative value is
 * refused with THINKTHEN_EUSAGE before anything is sent. The largest
 * budget is 4294967295000 ms (UINT32_MAX seconds); a larger value is
 * refused with THINKTHEN_EUSAGE before anything is sent. Clamp a computed
 * budget at zero (for example `end > now ? end - now : 0`). A passed
 * deadline is then spent and never reads as THINKTHEN_NO_DEADLINE.
 */
#define THINKTHEN_NO_DEADLINE INT64_C(-1)

/* The three answers a yes-or-no question gives. UNSURE is the machine word;
 * the specification says "not sure" in prose. */
#define THINKTHEN_YES 1
#define THINKTHEN_NO 0
#define THINKTHEN_UNSURE 2

/* Opaque engine value, built from the environment and freed by the host. */
typedef struct thinkthen_engine thinkthen_engine;

/* Opaque cancel token. Any number of calls on any engine may carry one
 * token; a null token means no token. Create it with
 * `thinkthen_cancel_token_new`, fire it with `thinkthen_cancel`, and free
 * it with `thinkthen_cancel_token_free` after every call that carried it
 * has returned. */
typedef struct thinkthen_cancel_token thinkthen_cancel_token;

/* One judgment: the answer under the question's rule and the probability
 * behind it. `outcome` is THINKTHEN_YES, THINKTHEN_NO, or THINKTHEN_UNSURE;
 * `probability` is the probability the backend gave the yes side. */
typedef struct thinkthen_answer {
    int outcome;
    double probability;
} thinkthen_answer;

/* Build an engine from the environment, as the command does:
 * THINKTHEN_BASE_URL for the address, THINKTHEN_API_KEY for the key,
 * THINKTHEN_CACHE for the cache folder, and otherwise the XDG cache home
 * and the XDG configuration file. Building sends nothing. Returns NULL
 * when the environment settings are invalid (THINKTHEN_EUSAGE) or the
 * cache folder or configuration file cannot be read (THINKTHEN_ELOCAL);
 * the error functions called with NULL then name that failure. */
thinkthen_engine *thinkthen_engine_new(void);

/* Build from the environment plus one UTF-8 JSON object. NULL or {} uses the
 * environment alone. Keys: backend, base_url, model, throttle, max_requests,
 * max_requests_total, max_estimated_input_tokens_total, max_request_bytes,
 * batch, cache, record, replay, timeout (whole seconds), max_retries,
 * profile. The estimated total charges each final encoded live request body
 * at ceil(bytes * 908 / 1000); it is not a provider token or billing cap.
 * cache takes false or a folder; max_requests and the two process totals may
 * be null. Unknown or repeated
 * keys and wrong types fail with THINKTHEN_EUSAGE in the calling thread's
 * null-engine error slot. backend selects a captured named key and default
 * model. Explicit base_url receives that backend key. No key is accepted.
 * Building sends nothing. */
thinkthen_engine *thinkthen_engine_new_with(const char *settings_json);

/* Free an engine. NULL is accepted and ignored. Free it only after every
 * call on it has returned. */
void thinkthen_engine_free(thinkthen_engine *engine);

/* The message for the last failure the calling thread recorded on this
 * engine, valid until that thread records its next failure; another
 * thread's calls never replace it. A deadline's message names the limit
 * and its value. Never NULL: before any failure it names that nothing
 * failed yet. With a null engine it is the calling thread's last failed
 * build's message, valid until that thread's next `thinkthen_engine_new`,
 * or else it names that no engine came. */
const char *thinkthen_error_message(const thinkthen_engine *engine);

/* Whether the same call could pass later: 1 for a backend status the
 * engine retries, such as busy or failing; 0 for a transport failure,
 * which may already have reached the backend, for a refused key, and for
 * every kind but the backend kind. Zero when nothing failed. With a null
 * engine it follows the calling thread's last failed build, else zero. */
int thinkthen_error_retryable(const thinkthen_engine *engine);

/* The code of the calling thread's last failure on this engine: the value
 * the failing call returned, THINKTHEN_OK when nothing failed yet. Success
 * does not clear it, so read it when a call fails. With a null engine it
 * is the calling thread's last failed build's code, else THINKTHEN_EUSAGE,
 * because no engine holds a failure. */
int thinkthen_error_code(const thinkthen_engine *engine);

/* Create a cancel token. */
thinkthen_cancel_token *thinkthen_cancel_token_new(void);

/* Fire a token: the calls carrying it start no new request or retry, let
 * the requests they sent finish, and return THINKTHEN_ECANCELLED with no
 * results, even when a sent request's reply arrives after the fire. A
 * token is one-shot: a fire leaves it fired, a second fire is ignored,
 * and no call re-arms it. Thread-safe from any thread, and it allocates
 * nothing; a null token is accepted and ignored. */
void thinkthen_cancel(thinkthen_cancel_token *token);

/* Free a token. NULL is accepted and ignored. */
void thinkthen_cancel_token_free(thinkthen_cancel_token *token);

/*
 * The argument rules, one row per pointer:
 *
 * - `engine` null: THINKTHEN_EUSAGE, or NULL from `thinkthen_call` and its
 *   `_opts` twin, with no message, because no engine holds one.
 * - `question_json`, `request_json`, and `spec_json` null or not UTF-8:
 *   THINKTHEN_EUSAGE. These strings are NUL-terminated; they carry no
 *   length.
 * - `text` null with `text_len` zero: the empty text, which the engine's
 *   own blank-evidence rule refuses with the usage kind. `text` null with
 *   a nonzero length: THINKTHEN_EUSAGE. A non-null `text` reads exactly
 *   `text_len` bytes; no terminator is read.
 * - `texts`, `lengths`, and the bulk `out` array: read or written for
 *   `count` entries. Null is THINKTHEN_EUSAGE when `count` is nonzero; a
 *   count of zero reads and writes nothing, so null is accepted.
 * - `out` and `out_len` of recognize and relate, and `out` of decide:
 *   always written on success, so null is THINKTHEN_EUSAGE.
 * - a cancel token in an `_opts` call: null means no token.
 *
 * The door checks these before it asks the engine, so a refusal sends
 * nothing.
 */

/* Read one named UTF-8 question file, at most 1 MiB, through the shared
 * single-question grammar. Success returns its validated source JSON in an
 * owned NUL-terminated string and writes its byte length. Free the string
 * once with `thinkthen_free_string`. Pass it to a typed question verb or use
 * it while constructing a JSON-door request. The loader itself sends nothing.
 * A null or invalid UTF-8 path, or null output pointer, is EUSAGE. An
 * unreadable, overlarge, invalid UTF-8 or malformed named file is ELOCAL,
 * non-retryable. No failure changes either output, and no diagnostic prints
 * the path or file content. */
int thinkthen_question_file(const thinkthen_engine *engine, const char *path,
                            char **out, size_t *out_len);

/* Preview a judgment call without sending it. `plan_json` is one closed
 * `thinkthen.plan-input/1` object: `verb` (`decide`, `choose`, `score`, or
 * `tag`), `question` (the bare question text, or one question object in the
 * question-file grammar asking that verb), `input` (one text or an array of
 * texts), and optional `settings` (the portable `thinkthen.settings/1`
 * object). Bare question text takes its question fields, such as `options`
 * or `threshold`, from the settings; a question object takes only `batch`,
 * `context`, and `deadline_ms` from them. Success writes the result schema's
 * `plan` object as owned NUL-terminated JSON text with its byte length:
 * `records`, `requests` (planned requests before cache answers, refusal
 * splits, and retries), `estimated_bytes`, `estimated_input_tokens` as
 * `{"lower", "upper"}`, `upper_bound`, and `first_body_utf8` (the first
 * request body, or null for no input). Free it once with
 * `thinkthen_free_string`. The preview reads no key and no cache and sends
 * nothing. An unknown or repeated member, a wrong verb or input shape, bad
 * settings, a settings field the question repeats, or a null pointer is
 * THINKTHEN_EUSAGE, and both outputs keep what they held. */
int thinkthen_plan_json(const thinkthen_engine *engine, const char *plan_json,
                        char **out, size_t *out_len);

/* Ask one yes-or-no question of one text: exactly `thinkthen_decide_opts`
 * with THINKTHEN_NO_DEADLINE and a null token. `question_json` is one
 * decide question in the question-file grammar, or the bare text of a
 * decide question at the cut of one half; `text` and `text_len` are the
 * evidence. The judgment lands in `out` on THINKTHEN_OK. */
int thinkthen_decide(const thinkthen_engine *engine, const char *question_json,
                     const char *text, size_t text_len,
                     thinkthen_answer *out);

/* The same call with the options beside it: `deadline_ms` is the budget
 * and `cancel` is the token, both described above. */
int thinkthen_decide_opts(const thinkthen_engine *engine, const char *question_json,
                          const char *text, size_t text_len,
                          int64_t deadline_ms, thinkthen_cancel_token *cancel,
                          thinkthen_answer *out);

/* Ask the same question of every text at once, at the engine's throttle,
 * and keep every judgment in input order: exactly
 * `thinkthen_decide_many_opts` with THINKTHEN_NO_DEADLINE and a null
 * token. `texts` holds `count` pointers and `lengths` holds their byte
 * lengths; `out` holds room for `count` answers. This is decide's bulk
 * spelling, the one bulk entry point every language loading this library
 * uses. */
int thinkthen_decide_many(const thinkthen_engine *engine, const char *question_json,
                          const char *const *texts, const size_t *lengths,
                          size_t count, thinkthen_answer *out);

/* The same bulk call with the options beside it. */
int thinkthen_decide_many_opts(const thinkthen_engine *engine, const char *question_json,
                               const char *const *texts, const size_t *lengths,
                               size_t count, int64_t deadline_ms,
                               thinkthen_cancel_token *cancel, thinkthen_answer *out);

/* The JSON door with no budget and no token: exactly
 * `thinkthen_call_opts` with THINKTHEN_NO_DEADLINE and a null token. */
char *thinkthen_call(const thinkthen_engine *engine, const char *request_json);

/* The JSON door: one request as a JSON object, and its answer as JSON
 * text, which `thinkthen_free_string` frees, with the options beside it.
 *
 * A request names one verb of the ten: `decide`, `choose`, `score`, `tag`,
 * `filter`, `rank`, `find`, `annotate`, `recognize`, or `relate`. Beside
 * the verb the door reads six keys of its own: `evidence` (one text, for
 * `decide`, `choose`, `score`, `tag`, and `recognize`), `records` (an
 * array for the four judgments, `filter`, `rank`, `annotate`, and `relate`),
 * `units` (an array for `find`), `details` (true or false for the four
 * judgments), `usage` (true, alone, for the counters), and `call` (a closed
 * object for eligible record arrays). `call.batch` is `"max"` or a positive
 * integer; `call.context` is nonblank shared text for supported text records.
 * Every other
 * key forms the question object, in the question file's own grammar, so
 * `{"choose": "Which team?", "options": ["billing", "other"],
 * "evidence": "..."}` asks one choose question. `filter` asks its text as
 * a decide question; `rank` takes its text alone; `find` takes its text
 * and `"none": true` to offer a none candidate, so it may answer `null`;
 * `annotate` carries the question set, and a member's `on` reads that
 * part of each record as JSON text; `recognize` and `relate` carry
 * `version` and their section beside the verb.
 *
 * A successful asking call returns `{"value":VALUE,"facts":FACTS}`.
 * VALUE is the old bare answer: `true`, `false`, or
 * `null` for decide; a label or `null` for choose; a number for score; an
 * array of labels for tag; an array of the kept records for filter; an
 * array of `{"index": N, "record": TEXT, "probability": P}` for every
 * record, most likely yes first, with a zero-based index, for rank;
 * `{"index": N, "unit": TEXT, "probability": P}` for the selected unit, with
 * a zero-based index, or `null` for find; an array of one object a record for annotate;
 * `{"entities": [...], "relations": [...]}` for recognize; and
 * `{"edges": [...]}` for relate. The four judgments also accept `records`,
 * whose VALUE is an ordered answer array. `"details": true` makes VALUE the
 * command's `--details` object or an array of full record-detail objects.
 * FACTS holds this call's records, sends, cache answers, seconds, and optional
 * provider tokens and model. `{"usage": true}` remains this engine's direct
 * counters as `{"requests_sent": N, "retries": N, "input_tokens": N, "output_tokens":
 * N, "cache_answers": N}` and takes no options.
 *
 * Returns NULL on failure, with no partial value; `thinkthen_error_code` and
 * `thinkthen_error_message` name what failed and
 * `thinkthen_error_retryable` says whether the same call could pass
 * later. `thinkthen_error_facts_json` reads final started-failure facts. */
char *thinkthen_call_opts(const thinkthen_engine *engine, const char *request_json,
                          int64_t deadline_ms, thinkthen_cancel_token *cancel);

/* Borrow the calling thread's last failed call's facts on this engine, or NULL
 * when no failure with started-call facts exists. The pointer has the same
 * lifetime as thinkthen_error_message and must not be freed. */
const char *thinkthen_error_facts_json(const thinkthen_engine *engine);

/* Find every name in one text, and the relations the rules allow: exactly
 * `thinkthen_recognize_opts` with THINKTHEN_NO_DEADLINE and a null
 * token. The result has no fixed size, so it crosses as one JSON string
 * written to `*out` (freed with `thinkthen_free_string`) with its length
 * in `*out_len`: `{"entities": [...], "relations": [...]}`. Each entity
 * carries `text`, `start`, `end`, `length`, `kind`, and `strength`, with
 * `start`, `end`, and `length` counting code points of `text`. With no
 * kinds, every name has the kind `ENTITY`. `spec_json` is a version-one
 * question file with its `recognize` section (`kinds`, `relations`) and
 * optional `threshold` and `relation_threshold`. The call follows the
 * engine's cache like every call. Returns THINKTHEN_OK on success and the
 * kind's code (1..6, THINKTHEN_EUSAGE through THINKTHEN_EDEFECT) on
 * failure, with `thinkthen_error_message` naming what failed.
 *
 * The ends of a relation carry `source` and `target` on every surface,
 * this returned JSON and the question file included. */
int thinkthen_recognize(const thinkthen_engine *engine, const char *spec_json,
                        const char *text, size_t text_len, char **out,
                        size_t *out_len);

/* The same call with the options beside it. */
int thinkthen_recognize_opts(const thinkthen_engine *engine, const char *spec_json,
                             const char *text, size_t text_len,
                             int64_t deadline_ms, thinkthen_cancel_token *cancel,
                             char **out, size_t *out_len);

/* Say how the given entities relate: exactly `thinkthen_relate_opts` with
 * THINKTHEN_NO_DEADLINE and a null token. The result is one JSON object
 * written to `*out` (freed with `thinkthen_free_string`):
 * `{"edges": [...]}`, where each edge is `{"relation": NAME, "source":
 * {"name", "kind"}, "target": {"name", "kind"}, "probability": P}` in
 * rule order, plus `"either": true` last on an edge of a both-ways rule,
 * whose ends are then in input order. `texts` carries `count` records and `lengths` their
 * lengths. Each record is one JSON object with a string `name` and a
 * string `kind`, as the command reads JSONL. `count` past 255 is a usage
 * refusal before anything else. `spec_json` is a version-one question
 * file with its `relate` section (`relations`) and optional `threshold`;
 * a `fields` pointer other than `/name` and `/kind` is refused with the
 * usage kind. Returns THINKTHEN_OK on success and the kind's code (1..6,
 * THINKTHEN_EUSAGE through THINKTHEN_EDEFECT) on failure, with
 * `thinkthen_error_message` naming what failed. */
int thinkthen_relate(const thinkthen_engine *engine, const char *spec_json,
                     const char *const *texts, const size_t *lengths,
                     size_t count, char **out, size_t *out_len);

/* The same call with the options beside it. */
int thinkthen_relate_opts(const thinkthen_engine *engine, const char *spec_json,
                          const char *const *texts, const size_t *lengths,
                          size_t count, int64_t deadline_ms,
                          thinkthen_cancel_token *cancel, char **out,
                          size_t *out_len);

/* Preferred typed forms: each successful call owns final facts JSON beside
 * its result. The facts object contains records, requests_sent, cache_answers,
 * seconds, and optional input_tokens, output_tokens, and model. Free each
 * returned JSON string with thinkthen_free_string. The old typed symbols
 * above remain ABI-compatible bare-result forms; they do not return facts.
 * A nonzero code changes no output slot. A started failure's facts remain
 * available from thinkthen_error_facts_json under its borrowed lifetime.
 * All output slots must be nonnull (except the zero-count answer array) and
 * must not share an address. Counts times pointer, size_t, and answer sizes,
 * and every text length, must fit PTRDIFF_MAX. The caller supplies live,
 * aligned, adequately sized, nonoverlapping input and output storage. */
int thinkthen_decide_with_facts(const thinkthen_engine *engine,
    const char *question_json, const char *text, size_t text_len,
    thinkthen_answer *out, char **facts_json, size_t *facts_len);
int thinkthen_decide_with_facts_opts(const thinkthen_engine *engine,
    const char *question_json, const char *text, size_t text_len,
    int64_t deadline_ms, thinkthen_cancel_token *cancel,
    thinkthen_answer *out, char **facts_json, size_t *facts_len);
int thinkthen_decide_many_with_facts(const thinkthen_engine *engine,
    const char *question_json, const char *const *texts, const size_t *lengths,
    size_t count, thinkthen_answer *out, char **facts_json, size_t *facts_len);
int thinkthen_decide_many_with_facts_opts(const thinkthen_engine *engine,
    const char *question_json, const char *const *texts, const size_t *lengths,
    size_t count, int64_t deadline_ms, thinkthen_cancel_token *cancel,
    thinkthen_answer *out, char **facts_json, size_t *facts_len);
int thinkthen_recognize_with_facts(const thinkthen_engine *engine,
    const char *spec_json, const char *text, size_t text_len,
    char **out, size_t *out_len, char **facts_json, size_t *facts_len);
int thinkthen_recognize_with_facts_opts(const thinkthen_engine *engine,
    const char *spec_json, const char *text, size_t text_len,
    int64_t deadline_ms, thinkthen_cancel_token *cancel,
    char **out, size_t *out_len, char **facts_json, size_t *facts_len);
int thinkthen_relate_with_facts(const thinkthen_engine *engine,
    const char *spec_json, const char *const *texts, const size_t *lengths,
    size_t count, char **out, size_t *out_len,
    char **facts_json, size_t *facts_len);
int thinkthen_relate_with_facts_opts(const thinkthen_engine *engine,
    const char *spec_json, const char *const *texts, const size_t *lengths,
    size_t count, int64_t deadline_ms, thinkthen_cancel_token *cancel,
    char **out, size_t *out_len, char **facts_json, size_t *facts_len);

/* Free a string `thinkthen_call`, `thinkthen_recognize`, or
 * `thinkthen_relate` returned, or their `_opts` twins. NULL is accepted
 * and ignored. */
void thinkthen_free_string(char *text);

typedef struct thinkthen_question thinkthen_question;
typedef struct thinkthen_source thinkthen_source;
typedef struct thinkthen_image thinkthen_image;
typedef struct thinkthen_result thinkthen_result;
typedef struct thinkthen_batch thinkthen_batch;

typedef struct thinkthen_string_v1 { const char *data; size_t len; } thinkthen_string_v1;
typedef struct thinkthen_strings_v1 { const thinkthen_string_v1 *data; size_t len; } thinkthen_strings_v1;
typedef struct thinkthen_optional_string_v1 { int present; thinkthen_string_v1 value; } thinkthen_optional_string_v1;
typedef struct thinkthen_optional_size_v1 { int present; size_t value; } thinkthen_optional_size_v1;
typedef struct thinkthen_optional_u64_v1 { int present; uint64_t value; } thinkthen_optional_u64_v1;
typedef struct thinkthen_optional_u16_v1 { int present; uint16_t value; } thinkthen_optional_u16_v1;
typedef struct thinkthen_optional_double_v1 { int present; double value; } thinkthen_optional_double_v1;
typedef struct thinkthen_optional_discriminator_v1 { int present; uint32_t value; } thinkthen_optional_discriminator_v1;

#define THINKTHEN_CONTENT_TEXT_V1 UINT32_C(1)
#define THINKTHEN_CONTENT_JSON_V1 UINT32_C(2)
typedef struct thinkthen_content_v1 { uint32_t kind; thinkthen_string_v1 data; } thinkthen_content_v1;
typedef struct thinkthen_optional_content_v1 { int present; thinkthen_content_v1 value; } thinkthen_optional_content_v1;

#define THINKTHEN_RULE_DEFAULT_V1 UINT32_C(0) /* input-only missing reading */
#define THINKTHEN_RULE_NULL_V1 UINT32_C(1)
#define THINKTHEN_RULE_CUT_V1 UINT32_C(2)
#define THINKTHEN_RULE_BAND_V1 UINT32_C(3)
/* CUT uses low; BAND uses low/high; NULL/DEFAULT use neither. */
typedef struct thinkthen_rule_v1 { uint32_t kind; double low; double high; } thinkthen_rule_v1;
typedef struct thinkthen_optional_rule_v1 { int present; thinkthen_rule_v1 value; } thinkthen_optional_rule_v1;

#define THINKTHEN_FUNCTION_DECIDE_V1 UINT32_C(1)
#define THINKTHEN_FUNCTION_CHOOSE_V1 UINT32_C(2)
#define THINKTHEN_FUNCTION_TAG_V1 UINT32_C(3)
#define THINKTHEN_FUNCTION_SCORE_V1 UINT32_C(4)
#define THINKTHEN_FUNCTION_FILTER_V1 UINT32_C(5)
#define THINKTHEN_FUNCTION_RANK_V1 UINT32_C(6)
#define THINKTHEN_FUNCTION_FIND_V1 UINT32_C(7)
#define THINKTHEN_FUNCTION_ANNOTATE_V1 UINT32_C(8)
#define THINKTHEN_FUNCTION_RECOGNIZE_V1 UINT32_C(9)
#define THINKTHEN_FUNCTION_RELATE_V1 UINT32_C(10)

typedef struct thinkthen_choice_v1 {
    thinkthen_string_v1 name;
    thinkthen_optional_content_v1 description;
    thinkthen_optional_double_v1 weight;
} thinkthen_choice_v1;
typedef struct thinkthen_choices_v1 { const thinkthen_choice_v1 *data; size_t len; } thinkthen_choices_v1;
typedef struct thinkthen_relation_v1 {
    thinkthen_string_v1 name;
    thinkthen_string_v1 source;
    thinkthen_string_v1 target;
    thinkthen_optional_string_v1 reads;
    int either;
    int single;
} thinkthen_relation_v1;
typedef struct thinkthen_relations_v1 { const thinkthen_relation_v1 *data; size_t len; } thinkthen_relations_v1;
typedef struct thinkthen_member_spec_v1 {
    thinkthen_string_v1 name;
    const thinkthen_question *question;
} thinkthen_member_spec_v1;
typedef struct thinkthen_member_specs_v1 { const thinkthen_member_spec_v1 *data; size_t len; } thinkthen_member_specs_v1;
typedef struct thinkthen_question_spec_v1 {
    uint32_t kind;
    thinkthen_content_v1 text;
    thinkthen_optional_content_v1 yes;
    thinkthen_optional_content_v1 no;
    thinkthen_choices_v1 choices;
    thinkthen_rule_v1 threshold;
    thinkthen_rule_v1 relation_threshold;
    thinkthen_optional_string_v1 model;
    thinkthen_optional_string_v1 profile;
    thinkthen_optional_size_v1 batch;
    int batch_max;
    int none;
    thinkthen_strings_v1 on;
    thinkthen_member_specs_v1 members;
    thinkthen_choices_v1 kinds;
    thinkthen_relations_v1 relations;
    thinkthen_optional_string_v1 name_pointer;
    thinkthen_optional_string_v1 kind_pointer;
} thinkthen_question_spec_v1;

/* Result questions expose nested questions directly, with no engine handle. */
typedef struct thinkthen_question_view_v1 thinkthen_question_view_v1;
typedef struct thinkthen_question_member_v1 {
    thinkthen_string_v1 name;
    const thinkthen_question_view_v1 *question;
} thinkthen_question_member_v1;
typedef struct thinkthen_question_members_v1 { const thinkthen_question_member_v1 *data; size_t len; } thinkthen_question_members_v1;
struct thinkthen_question_view_v1 {
    uint32_t kind;
    thinkthen_content_v1 text;
    thinkthen_optional_content_v1 yes;
    thinkthen_optional_content_v1 no;
    thinkthen_choices_v1 choices;
    thinkthen_rule_v1 threshold;
    thinkthen_rule_v1 relation_threshold;
    thinkthen_optional_string_v1 model;
    thinkthen_optional_string_v1 profile;
    thinkthen_optional_size_v1 batch;
    int batch_max;
    int none;
    thinkthen_strings_v1 on;
    thinkthen_question_members_v1 members;
    thinkthen_choices_v1 kinds;
    thinkthen_relations_v1 relations;
    thinkthen_optional_string_v1 name_pointer;
    thinkthen_optional_string_v1 kind_pointer;
};
typedef struct thinkthen_optional_question_v1 { int present; thinkthen_question_view_v1 value; } thinkthen_optional_question_v1;

typedef struct thinkthen_images_v1 { const thinkthen_image *const *data; size_t len; } thinkthen_images_v1;
#define THINKTHEN_IMAGE_JPEG_V1 UINT32_C(1)
#define THINKTHEN_IMAGE_PNG_V1 UINT32_C(2)
typedef struct thinkthen_image_view_v1 {
    uint32_t media;
    const uint8_t *bytes;
    size_t bytes_len;
    uint32_t width;
    uint32_t height;
    thinkthen_optional_string_v1 filename;
} thinkthen_image_view_v1;
typedef struct thinkthen_image_views_v1 { const thinkthen_image_view_v1 *data; size_t len; } thinkthen_image_views_v1;
typedef struct thinkthen_optional_image_views_v1 { int present; thinkthen_image_views_v1 value; } thinkthen_optional_image_views_v1;
typedef struct thinkthen_record_v1 {
    thinkthen_optional_content_v1 original;
    thinkthen_optional_content_v1 context;
    thinkthen_choices_v1 options;
    thinkthen_images_v1 images;
} thinkthen_record_v1;
#define THINKTHEN_SOURCE_LINE_V1 UINT32_C(1)
#define THINKTHEN_SOURCE_WINDOW_V1 UINT32_C(2)
#define THINKTHEN_SOURCE_FILE_V1 UINT32_C(3)
#define THINKTHEN_SOURCE_IMAGE_FILE_V1 UINT32_C(4)
#define THINKTHEN_SOURCE_JSONL_V1 UINT32_C(5)
typedef struct thinkthen_source_spec_v1 {
    thinkthen_strings_v1 paths;
    uint32_t unit;
    size_t window;
} thinkthen_source_spec_v1;
typedef struct thinkthen_controls_v1 {
    int64_t deadline_ms;
    thinkthen_cancel_token *cancel;
    thinkthen_optional_content_v1 context;
    thinkthen_optional_size_v1 batch;
    int batch_max;
    int attempts;
    thinkthen_string_v1 surface;
} thinkthen_controls_v1;

#define THINKTHEN_DECIDE_NULL_V1 UINT32_C(0)
#define THINKTHEN_DECIDE_BOOLEAN_V1 UINT32_C(1)
#define THINKTHEN_DECIDE_AUTHORED_V1 UINT32_C(2)
/* BOOLEAN is used only for ordinary un-authored true/false decisions.
 * AUTHORED preserves the chosen user meaning even if it spells a Boolean;
 * successful uncertainty always uses NULL. No JSON decode for that case. */
typedef struct thinkthen_decide_value_v1 {
    uint32_t kind;
    union {
        int boolean;
        thinkthen_content_v1 authored;
    } data;
} thinkthen_decide_value_v1;
typedef struct thinkthen_probability_v1 { thinkthen_string_v1 name; double probability; } thinkthen_probability_v1;
typedef struct thinkthen_probabilities_v1 { const thinkthen_probability_v1 *data; size_t len; } thinkthen_probabilities_v1;
typedef struct thinkthen_optional_probabilities_v1 { int present; thinkthen_probabilities_v1 value; } thinkthen_optional_probabilities_v1;
#define THINKTHEN_ANSWER_YES_NO_V1 UINT32_C(1)
#define THINKTHEN_ANSWER_CHOICE_V1 UINT32_C(2)
#define THINKTHEN_ANSWER_TAG_V1 UINT32_C(3)
#define THINKTHEN_ANSWER_SCORE_V1 UINT32_C(4)
#define THINKTHEN_ANSWER_FIND_V1 UINT32_C(5)
typedef struct thinkthen_named_answer_v1 {
    thinkthen_string_v1 pick;
    thinkthen_probabilities_v1 probabilities;
    thinkthen_optional_double_v1 confidence;
} thinkthen_named_answer_v1;
typedef struct thinkthen_score_answer_v1 {
    thinkthen_string_v1 level;
    thinkthen_probabilities_v1 probabilities;
    thinkthen_optional_double_v1 confidence;
} thinkthen_score_answer_v1;
typedef struct thinkthen_answer_v1 {
    uint32_t kind;
    union {
        double probability; /* YES_NO */
        thinkthen_named_answer_v1 choice;
        thinkthen_probabilities_v1 tag;
        thinkthen_score_answer_v1 score;
        thinkthen_named_answer_v1 find;
    } data;
} thinkthen_answer_v1;
typedef struct thinkthen_optional_answer_v1 { int present; thinkthen_answer_v1 value; } thinkthen_optional_answer_v1;
typedef struct thinkthen_location_v1 {
    thinkthen_optional_string_v1 file;
    thinkthen_optional_size_v1 first_line;
    thinkthen_optional_size_v1 last_line;
} thinkthen_location_v1;
typedef struct thinkthen_optional_location_v1 { int present; thinkthen_location_v1 value; } thinkthen_optional_location_v1;

/* Annotate admits these four kinds. The discriminator uses FUNCTION_*.
 * A null decide/choose is a successful typed value, not member failure. */
typedef struct thinkthen_member_value_v1 {
    uint32_t kind;
    union {
        thinkthen_decide_value_v1 decide;
        thinkthen_optional_string_v1 choose;
        thinkthen_strings_v1 tag;
        double score;
    } data;
} thinkthen_member_value_v1;
#define THINKTHEN_MEMBER_SUCCESS_V1 UINT32_C(1)
#define THINKTHEN_MEMBER_FAILURE_V1 UINT32_C(2)
#define THINKTHEN_MEMBER_MISSING_ANSWER_V1 UINT32_C(1)
#define THINKTHEN_MEMBER_WRONG_KIND_V1 UINT32_C(2)
#define THINKTHEN_MEMBER_MISSING_PROBABILITY_V1 UINT32_C(3)
#define THINKTHEN_MEMBER_INVALID_PROBABILITY_V1 UINT32_C(4)
#define THINKTHEN_MEMBER_INVALID_DISTRIBUTION_V1 UINT32_C(5)
#define THINKTHEN_MEMBER_UNEXPECTED_PROBABILITY_V1 UINT32_C(6)
typedef struct thinkthen_member_failure_v1 {
    thinkthen_string_v1 failure_id;
    uint32_t cause;
} thinkthen_member_failure_v1;
typedef struct thinkthen_member_success_v1 {
    thinkthen_string_v1 answer_id;
    thinkthen_member_value_v1 value;
    thinkthen_answer_v1 answer;
    thinkthen_rule_v1 threshold;
} thinkthen_member_success_v1;
typedef struct thinkthen_member_v1 {
    thinkthen_string_v1 name;
    thinkthen_string_v1 request;
    thinkthen_question_view_v1 question;
    uint32_t state;
    union {
        thinkthen_member_success_v1 success;
        thinkthen_member_failure_v1 failure;
    } data;
} thinkthen_member_v1;
typedef struct thinkthen_members_v1 { const thinkthen_member_v1 *data; size_t len; } thinkthen_members_v1;

typedef struct thinkthen_entity_v1 {
    thinkthen_string_v1 text;
    size_t start;
    size_t end;
    size_t length;
    thinkthen_string_v1 kind;
    double strength;
} thinkthen_entity_v1;
typedef struct thinkthen_entities_v1 { const thinkthen_entity_v1 *data; size_t len; } thinkthen_entities_v1;
typedef struct thinkthen_entity_edge_v1 {
    thinkthen_string_v1 relation;
    thinkthen_entity_v1 source;
    thinkthen_entity_v1 target;
    double probability;
    int either;
} thinkthen_entity_edge_v1;
typedef struct thinkthen_entity_edges_v1 { const thinkthen_entity_edge_v1 *data; size_t len; } thinkthen_entity_edges_v1;
typedef struct thinkthen_optional_entity_edges_v1 { int present; thinkthen_entity_edges_v1 value; } thinkthen_optional_entity_edges_v1;
typedef struct thinkthen_place_v1 { size_t start; size_t end; } thinkthen_place_v1;
typedef struct thinkthen_piece_v1 {
    size_t start;
    size_t end;
    thinkthen_probabilities_v1 tags;
} thinkthen_piece_v1;
typedef struct thinkthen_pieces_v1 { const thinkthen_piece_v1 *data; size_t len; } thinkthen_pieces_v1;
typedef struct thinkthen_name_v1 {
    size_t start;
    size_t end;
    thinkthen_optional_probabilities_v1 kinds;
    thinkthen_optional_probabilities_v1 edges;
} thinkthen_name_v1;
typedef struct thinkthen_names_v1 { const thinkthen_name_v1 *data; size_t len; } thinkthen_names_v1;
typedef struct thinkthen_pair_v1 {
    thinkthen_string_v1 relation;
    thinkthen_place_v1 source;
    thinkthen_place_v1 target;
    double probability;
} thinkthen_pair_v1;
typedef struct thinkthen_pairs_v1 { const thinkthen_pair_v1 *data; size_t len; } thinkthen_pairs_v1;
typedef struct thinkthen_recognize_value_v1 {
    thinkthen_entities_v1 entities;
    thinkthen_optional_entity_edges_v1 relations;
} thinkthen_recognize_value_v1;
typedef struct thinkthen_recognize_answer_v1 {
    thinkthen_pieces_v1 pieces;
    thinkthen_names_v1 names;
    thinkthen_pairs_v1 pairs;
} thinkthen_recognize_answer_v1;

typedef struct thinkthen_endpoint_v1 { thinkthen_string_v1 name; thinkthen_string_v1 kind; } thinkthen_endpoint_v1;
typedef struct thinkthen_optional_endpoint_v1 { int present; thinkthen_endpoint_v1 value; } thinkthen_optional_endpoint_v1;
typedef struct thinkthen_edge_v1 {
    thinkthen_string_v1 relation;
    thinkthen_endpoint_v1 source;
    thinkthen_endpoint_v1 target;
    double probability;
    int either;
} thinkthen_edge_v1;
typedef struct thinkthen_edges_v1 { const thinkthen_edge_v1 *data; size_t len; } thinkthen_edges_v1;
#define THINKTHEN_RELATION_YES_NO_V1 UINT32_C(1)
#define THINKTHEN_RELATION_CHOICE_V1 UINT32_C(2)
#define THINKTHEN_DIRECTION_SOURCE_TO_TARGET_V1 UINT32_C(1)
#define THINKTHEN_DIRECTION_EITHER_V1 UINT32_C(2)
typedef struct thinkthen_relation_success_v1 {
    thinkthen_string_v1 answer_id;
    double probability;
    int accepted;
} thinkthen_relation_success_v1;
typedef struct thinkthen_relation_answer_v1 {
    thinkthen_string_v1 relation;
    thinkthen_string_v1 reads;
    uint32_t method;
    uint32_t direction;
    thinkthen_endpoint_v1 source;
    thinkthen_optional_endpoint_v1 target;
    thinkthen_string_v1 request;
    uint32_t state; /* MEMBER_SUCCESS / MEMBER_FAILURE */
    union {
        thinkthen_relation_success_v1 success;
        thinkthen_member_failure_v1 failure;
    } data;
} thinkthen_relation_answer_v1;
typedef struct thinkthen_relation_answers_v1 { const thinkthen_relation_answer_v1 *data; size_t len; } thinkthen_relation_answers_v1;

typedef struct thinkthen_usage_v1 { uint64_t input_tokens; uint64_t output_tokens; } thinkthen_usage_v1;
typedef struct thinkthen_optional_usage_v1 { int present; thinkthen_usage_v1 value; } thinkthen_optional_usage_v1;
#define THINKTHEN_ORIGIN_LIVE_V1 UINT32_C(1)
#define THINKTHEN_ORIGIN_CACHE_V1 UINT32_C(2)
#define THINKTHEN_ORIGIN_REPLAY_V1 UINT32_C(3)
#define THINKTHEN_ORIGIN_PROXY_V1 UINT32_C(4) /* reserved, never emitted in 0.2 */
#define THINKTHEN_ORIGIN_MEMORY_V1 UINT32_C(5) /* reserved, never emitted in 0.2 */
typedef struct thinkthen_question_source_v1 { uint32_t origin; thinkthen_string_v1 answered_by; } thinkthen_question_source_v1;
typedef struct thinkthen_question_sources_v1 { const thinkthen_question_source_v1 *data; size_t len; } thinkthen_question_sources_v1;
#define THINKTHEN_ID_OBSERVATION_V1 UINT32_C(1)
#define THINKTHEN_ID_FAILURE_V1 UINT32_C(2)
typedef struct thinkthen_observation_identity_v1 {
    uint32_t kind;
    union {
        thinkthen_string_v1 observation_id;
        thinkthen_string_v1 failure_id;
    } data;
} thinkthen_observation_identity_v1;
typedef struct thinkthen_observation_identities_v1 { const thinkthen_observation_identity_v1 *data; size_t len; } thinkthen_observation_identities_v1;
typedef struct thinkthen_profile_warning_v1 { thinkthen_string_v1 tuned_for; thinkthen_string_v1 running; } thinkthen_profile_warning_v1;
typedef struct thinkthen_optional_profile_warning_v1 { int present; thinkthen_profile_warning_v1 value; } thinkthen_optional_profile_warning_v1;
#define THINKTHEN_BATCH_RECORDS_V1 UINT32_C(1)
#define THINKTHEN_BATCH_MAX_V1 UINT32_C(2)
typedef struct thinkthen_batch_v1 { uint32_t kind; size_t records; } thinkthen_batch_v1;
typedef struct thinkthen_optional_batch_v1 { int present; thinkthen_batch_v1 value; } thinkthen_optional_batch_v1;
typedef struct thinkthen_batch_warning_v1 { thinkthen_batch_v1 tuned_for; thinkthen_batch_v1 running; } thinkthen_batch_warning_v1;
typedef struct thinkthen_optional_batch_warning_v1 { int present; thinkthen_batch_warning_v1 value; } thinkthen_optional_batch_warning_v1;
#define THINKTHEN_ATTEMPT_OK_V1 UINT32_C(1)
#define THINKTHEN_ATTEMPT_STATUS_V1 UINT32_C(2)
#define THINKTHEN_ATTEMPT_TRANSPORT_V1 UINT32_C(3)
typedef struct thinkthen_attempt_v1 {
    uint64_t ordinal;
    thinkthen_string_v1 request_sha256;
    uint64_t wall_ms;
    uint32_t outcome;
    thinkthen_string_v1 sdk_request_id;
    thinkthen_optional_u16_v1 status;
    thinkthen_optional_u64_v1 server_ms;
    thinkthen_optional_string_v1 request_id;
} thinkthen_attempt_v1;
typedef struct thinkthen_attempts_v1 { const thinkthen_attempt_v1 *data; size_t len; } thinkthen_attempts_v1;
typedef struct thinkthen_optional_attempts_v1 { int present; thinkthen_attempts_v1 value; } thinkthen_optional_attempts_v1;
typedef struct thinkthen_meta_v1 {
    thinkthen_string_v1 tool;
    thinkthen_optional_string_v1 question_sha256;
    thinkthen_optional_string_v1 questions_sha256;
    thinkthen_string_v1 url;
    thinkthen_string_v1 model;
    thinkthen_optional_usage_v1 usage;
    uint64_t requests_sent;
    int cached;
    thinkthen_strings_v1 requests;
    size_t failed_questions;
    thinkthen_optional_profile_warning_v1 profile_warning;
    thinkthen_optional_batch_v1 batch_setting;
    thinkthen_optional_batch_warning_v1 batch_warning;
    thinkthen_optional_string_v1 context_sha256;
    thinkthen_optional_attempts_v1 attempts;
    thinkthen_optional_discriminator_v1 origin;
    thinkthen_question_sources_v1 question_sources;
    thinkthen_observation_identities_v1 observations;
    thinkthen_optional_string_v1 answered_by;
} thinkthen_meta_v1;
typedef struct thinkthen_optional_meta_v1 { int present; thinkthen_meta_v1 value; } thinkthen_optional_meta_v1;
/* origin.present=0 represents required JSON null, not omitted provenance.
 * requests, question_sources, observations have the same length/order.
 * Exactly one question digest is present: plural for annotate, singular else.
 * No proxy field exists: 0.2 cannot activate it or emit proxy metadata. */

typedef struct thinkthen_facts_v1 {
    thinkthen_string_v1 call_id;
    uint64_t cache_answers;
    thinkthen_optional_string_v1 estimated_cost_usd;
    thinkthen_optional_u64_v1 input_tokens;
    thinkthen_optional_string_v1 model;
    thinkthen_optional_u64_v1 output_tokens;
    uint64_t records;
    uint64_t requests_sent;
    double seconds;
    thinkthen_optional_u64_v1 command_ms;
} thinkthen_facts_v1;
typedef struct thinkthen_optional_facts_v1 { int present; thinkthen_facts_v1 value; } thinkthen_optional_facts_v1;
#define THINKTHEN_STOP_USAGE_V1 UINT32_C(1)
#define THINKTHEN_STOP_LOCAL_V1 UINT32_C(2)
#define THINKTHEN_STOP_NO_KEY_V1 UINT32_C(3)
#define THINKTHEN_STOP_TRANSPORT_V1 UINT32_C(4)
#define THINKTHEN_STOP_STATUS_V1 UINT32_C(5)
#define THINKTHEN_STOP_TOO_LARGE_V1 UINT32_C(6)
#define THINKTHEN_STOP_REPLY_V1 UINT32_C(7)
#define THINKTHEN_STOP_BACKEND_V1 UINT32_C(8)
#define THINKTHEN_STOP_CANCELLED_V1 UINT32_C(9)
#define THINKTHEN_STOP_DEFECT_V1 UINT32_C(10)
#define THINKTHEN_STOP_DEADLINE_V1 UINT32_C(11)
typedef struct thinkthen_stopped_v1 {
    thinkthen_optional_size_v1 at;
    uint32_t cause;
    thinkthen_optional_u16_v1 status;
    int retryable;
} thinkthen_stopped_v1;
typedef struct thinkthen_optional_stopped_v1 { int present; thinkthen_stopped_v1 value; } thinkthen_optional_stopped_v1;
typedef struct thinkthen_error_v1 {
    int code; /* the existing six THINKTHEN_E* return codes */
    thinkthen_string_v1 message;
    int retryable;
    thinkthen_optional_stopped_v1 stopped;
} thinkthen_error_v1;
typedef struct thinkthen_optional_error_v1 { int present; thinkthen_error_v1 value; } thinkthen_optional_error_v1;

typedef struct thinkthen_row_v1 {
    thinkthen_string_v1 answer_id;
    thinkthen_optional_content_v1 input;
    thinkthen_optional_question_v1 question;
    thinkthen_optional_answer_v1 answer;
    thinkthen_optional_rule_v1 threshold;
    thinkthen_optional_location_v1 position;
    thinkthen_optional_string_v1 input_file;
    thinkthen_meta_v1 meta;
    thinkthen_optional_image_views_v1 images;
} thinkthen_row_v1;
typedef struct thinkthen_decide_view_v1 { thinkthen_row_v1 common; thinkthen_decide_value_v1 value; } thinkthen_decide_view_v1;
typedef struct thinkthen_choose_view_v1 { thinkthen_row_v1 common; thinkthen_optional_string_v1 value; } thinkthen_choose_view_v1;
typedef struct thinkthen_tag_view_v1 { thinkthen_row_v1 common; thinkthen_strings_v1 value; } thinkthen_tag_view_v1;
typedef struct thinkthen_score_view_v1 { thinkthen_row_v1 common; double value; } thinkthen_score_view_v1;
typedef struct thinkthen_filter_view_v1 { thinkthen_row_v1 common; int value; } thinkthen_filter_view_v1;
typedef struct thinkthen_rank_view_v1 {
    thinkthen_row_v1 common;
    thinkthen_optional_size_v1 value;
    thinkthen_optional_string_v1 question_name;
} thinkthen_rank_view_v1;
typedef struct thinkthen_find_view_v1 {
    thinkthen_row_v1 common;
    thinkthen_optional_content_v1 value;
    thinkthen_optional_size_v1 index;
} thinkthen_find_view_v1;
typedef struct thinkthen_annotate_view_v1 { thinkthen_row_v1 common; thinkthen_members_v1 answers; } thinkthen_annotate_view_v1;
typedef struct thinkthen_recognize_view_v1 {
    thinkthen_row_v1 common;
    thinkthen_recognize_value_v1 value;
    thinkthen_recognize_answer_v1 answer;
} thinkthen_recognize_view_v1;
typedef struct thinkthen_relate_view_v1 {
    thinkthen_row_v1 common;
    thinkthen_edges_v1 value;
    thinkthen_relation_answers_v1 questions;
} thinkthen_relate_view_v1;

#define THINKTHEN_STAGE_BOUNDARY_V1 UINT32_C(1)
#define THINKTHEN_STAGE_KIND_V1 UINT32_C(2)
#define THINKTHEN_STAGE_EDGE_V1 UINT32_C(3)
#define THINKTHEN_STAGE_RELATION_V1 UINT32_C(4)
#define THINKTHEN_PROBABILITIES_YES_V1 UINT32_C(1)
#define THINKTHEN_PROBABILITIES_NAMED_V1 UINT32_C(2)
typedef struct thinkthen_observed_probabilities_v1 {
    uint32_t kind;
    union { double yes; thinkthen_probabilities_v1 named; } data;
} thinkthen_observed_probabilities_v1;
typedef struct thinkthen_observation_success_v1 {
    thinkthen_string_v1 answer_id;
    thinkthen_string_v1 observation_id;
    thinkthen_member_value_v1 value;
    thinkthen_observed_probabilities_v1 probabilities;
    thinkthen_optional_double_v1 confidence;
} thinkthen_observation_success_v1;
typedef struct thinkthen_question_observation_v1 {
    size_t index;
    thinkthen_optional_string_v1 member;
    thinkthen_optional_discriminator_v1 stage;
    size_t position;
    thinkthen_string_v1 question_sha256;
    thinkthen_string_v1 model;
    thinkthen_string_v1 url;
    thinkthen_strings_v1 requests;
    uint64_t requests_sent;
    int cached;
    size_t failed_questions;
    thinkthen_optional_usage_v1 usage;
    thinkthen_question_sources_v1 question_sources;
    uint32_t state;
    union {
        thinkthen_observation_success_v1 success;
        thinkthen_member_failure_v1 failure;
    } data;
} thinkthen_question_observation_v1;
typedef struct thinkthen_row_observation_v1 {
    size_t index;
    uint32_t function;
    union {
        thinkthen_decide_view_v1 decide;
        thinkthen_choose_view_v1 choose;
        thinkthen_tag_view_v1 tag;
        thinkthen_score_view_v1 score;
        thinkthen_filter_view_v1 filter;
        thinkthen_rank_view_v1 rank;
        thinkthen_find_view_v1 find;
        thinkthen_annotate_view_v1 annotate;
        thinkthen_recognize_view_v1 recognize;
        thinkthen_relate_view_v1 relate;
    } data;
} thinkthen_row_observation_v1;
#define THINKTHEN_EVENT_QUESTION_V1 UINT32_C(1)
#define THINKTHEN_EVENT_ROW_V1 UINT32_C(2)
typedef struct thinkthen_observation_v1 {
    uint32_t kind;
    union {
        thinkthen_question_observation_v1 question;
        thinkthen_row_observation_v1 row;
    } data;
} thinkthen_observation_v1;
#define THINKTHEN_RESULT_SUCCESS_V1 UINT32_C(1)
#define THINKTHEN_RESULT_FAILURE_V1 UINT32_C(2)
typedef struct thinkthen_summary_v1 {
    uint32_t state;
    thinkthen_string_v1 schema;
    thinkthen_optional_string_v1 answer_id;
    thinkthen_optional_discriminator_v1 function;
    size_t count;
    size_t observation_count;
    thinkthen_optional_meta_v1 meta;
    thinkthen_optional_facts_v1 facts;
    thinkthen_optional_attempts_v1 attempts;
    thinkthen_optional_error_v1 error;
} thinkthen_summary_v1;

/* Additive complete detail accessors; all pointers borrow result ownership.
 * The singular question-observation observation_id is NULL/zero-length for
 * aggregate questions; details.observations retains every actual identity. */
typedef struct thinkthen_reported_usage_v1 {
    int present;
    thinkthen_optional_u64_v1 input_tokens;
    thinkthen_optional_u64_v1 output_tokens;
} thinkthen_reported_usage_v1;
typedef struct thinkthen_source_detail_v1 {
    uint32_t origin;
    thinkthen_string_v1 answered_by;
    thinkthen_optional_size_v1 batch_size;
} thinkthen_source_detail_v1;
typedef struct thinkthen_source_details_v1 { const thinkthen_source_detail_v1 *data; size_t len; } thinkthen_source_details_v1;
typedef struct thinkthen_input_view_v1 {
    thinkthen_optional_content_v1 original;
    thinkthen_optional_location_v1 position;
    thinkthen_optional_image_views_v1 images;
} thinkthen_input_view_v1;
typedef struct thinkthen_input_views_v1 { const thinkthen_input_view_v1 *data; size_t len; } thinkthen_input_views_v1;
typedef struct thinkthen_details_v1 {
    thinkthen_optional_question_v1 question;
    thinkthen_optional_rule_v1 threshold;
    thinkthen_optional_string_v1 raw_pick;
    thinkthen_reported_usage_v1 usage;
    thinkthen_source_details_v1 question_sources;
    thinkthen_observation_identities_v1 observations;
    thinkthen_input_views_v1 inputs;
} thinkthen_details_v1;
/* Row ordinals match function-specific accessors. Observation ordinals match
 * result_observation. Wrong/out-of-range ordinals return EUSAGE unchanged.
 * usage.present retains a reported usage object even with an unknown dimension;
 * each token dimension has its own presence flag. Input views retain originals,
 * not model projections. Multirow summaries have no invented aggregate meta/ID. */

/* Additive located values come from native span/occurrence mapping. An
 * unlocated call returns present=0. Every nested view borrows result ownership. */
typedef struct thinkthen_source_entity_v1 { thinkthen_entity_v1 entity; thinkthen_optional_location_v1 position; } thinkthen_source_entity_v1;
typedef struct thinkthen_source_entities_v1 { const thinkthen_source_entity_v1 *data; size_t len; } thinkthen_source_entities_v1;
typedef struct thinkthen_source_entity_edge_v1 { thinkthen_string_v1 relation; thinkthen_source_entity_v1 source, target; double probability; int either; } thinkthen_source_entity_edge_v1;
typedef struct thinkthen_source_entity_edges_v1 { const thinkthen_source_entity_edge_v1 *data; size_t len; } thinkthen_source_entity_edges_v1;
typedef struct thinkthen_optional_source_entity_edges_v1 { int present; thinkthen_source_entity_edges_v1 value; } thinkthen_optional_source_entity_edges_v1;
typedef struct thinkthen_source_recognition_v1 { int present; thinkthen_source_entities_v1 entities; thinkthen_optional_source_entity_edges_v1 relations; } thinkthen_source_recognition_v1;
typedef struct thinkthen_source_endpoint_v1 { size_t ordinal; thinkthen_endpoint_v1 endpoint; thinkthen_content_v1 record; thinkthen_optional_location_v1 position; } thinkthen_source_endpoint_v1;
typedef struct thinkthen_source_edge_v1 { thinkthen_string_v1 relation; thinkthen_source_endpoint_v1 source, target; double probability; int either; } thinkthen_source_edge_v1;
typedef struct thinkthen_source_edges_v1 { const thinkthen_source_edge_v1 *data; size_t len; } thinkthen_source_edges_v1;
typedef struct thinkthen_source_relations_v1 { int present; thinkthen_source_edges_v1 edges; } thinkthen_source_relations_v1;
int thinkthen_result_source_recognition(const thinkthen_result *, size_t, thinkthen_source_recognition_v1 *);
int thinkthen_result_source_relations(const thinkthen_result *, size_t, thinkthen_source_relations_v1 *);
int thinkthen_result_details(const thinkthen_result *, size_t, thinkthen_details_v1 *);
int thinkthen_result_observation_details(const thinkthen_result *, size_t, thinkthen_details_v1 *);

/* Lazy record batches use the existing native scheduler for decide, choose,
 * tag, score, filter and annotate. Start clones question, source selection,
 * shared context and cancellation flag; callers may free question/source/token
 * after start. The engine must remain live through batch_free. Start, next,
 * facts and free must run on the creating thread with exclusive batch access.
 * Ordinary packing and real input pauses determine stages; there are no caller
 * stage markers. A source_records snapshot retains finite source bytes, while
 * source_files pulls the native reader lazily. Reader unit 5 explicitly reads
 * JSONL as typed JSON; ordinary text units never infer JSON from text.
 * next returns OK with one independently owned typed result, or OK with NULL
 * after exhaustion. Completed rows precede one terminal error; that error
 * leaves out unchanged, then next returns OK with NULL. Returned row results
 * outlive the batch, question, source and engine. Per-row summary facts are
 * absent while work continues; batch_facts returns an owned final summary
 * after exhaustion/error, and EUSAGE before termination without writing out.
 * batch_free stops and joins native workers before releasing owned backing.
 * The existing eager complete calls keep their all-before-send admission rule.
 * Rank, find, recognize and relate retain their native aggregate semantics
 * through the complete calls; they have no invented lazy batch interface.
 */
int thinkthen_decide_batch_start(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_batch **);
int thinkthen_choose_batch_start(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_batch **);
int thinkthen_tag_batch_start(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_batch **);
int thinkthen_score_batch_start(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_batch **);
int thinkthen_filter_batch_start(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_batch **);
int thinkthen_annotate_batch_start(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_batch **);
int thinkthen_batch_next(thinkthen_batch *, thinkthen_result **);
int thinkthen_batch_facts(const thinkthen_batch *, thinkthen_result **);
void thinkthen_batch_free(thinkthen_batch *);

int thinkthen_decide_complete(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
int thinkthen_choose_complete(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
int thinkthen_tag_complete(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
int thinkthen_score_complete(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
int thinkthen_filter_complete(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
int thinkthen_rank_complete(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
int thinkthen_find_complete(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
int thinkthen_annotate_complete(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
int thinkthen_recognize_complete(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
int thinkthen_relate_complete(const thinkthen_engine *, const thinkthen_question *, const thinkthen_source *, const thinkthen_controls_v1 *, thinkthen_result **);
void thinkthen_result_free(thinkthen_result *);
int thinkthen_result_summary(const thinkthen_result *, thinkthen_summary_v1 *);
int thinkthen_result_observation(const thinkthen_result *, size_t, thinkthen_observation_v1 *);
/* Borrow a final row by output position, retaining its original input index.
 * Nested views borrow until result_free. NULL owner/output or an out-of-range
 * position returns EUSAGE without writing output. */
int thinkthen_result_row(const thinkthen_result *, size_t, thinkthen_row_observation_v1 *);
int thinkthen_result_decide(const thinkthen_result *, size_t, thinkthen_decide_view_v1 *);
int thinkthen_result_choose(const thinkthen_result *, size_t, thinkthen_choose_view_v1 *);
int thinkthen_result_tag(const thinkthen_result *, size_t, thinkthen_tag_view_v1 *);
int thinkthen_result_score(const thinkthen_result *, size_t, thinkthen_score_view_v1 *);
int thinkthen_result_filter(const thinkthen_result *, size_t, thinkthen_filter_view_v1 *);
int thinkthen_result_rank(const thinkthen_result *, size_t, thinkthen_rank_view_v1 *);
/* Saved rank sets retain every ordered member. Simple rank has zero members.
 * Each member view has its own answer ID, complete probabilities and metadata.
 * Views borrow the result; invalid indices return EUSAGE without writing output. */
int thinkthen_result_rank_member_count(const thinkthen_result *, size_t, size_t *);
int thinkthen_result_rank_member(const thinkthen_result *, size_t, size_t, thinkthen_rank_view_v1 *);
int thinkthen_result_find(const thinkthen_result *, size_t, thinkthen_find_view_v1 *);
int thinkthen_result_annotate(const thinkthen_result *, size_t, thinkthen_annotate_view_v1 *);
int thinkthen_result_recognize(const thinkthen_result *, size_t, thinkthen_recognize_view_v1 *);
int thinkthen_result_relate(const thinkthen_result *, size_t, thinkthen_relate_view_v1 *);
/* Snapshot the calling thread's last failure for engine, or its failed-build
 * slot for engine=NULL. Never clears/replaces that slot.
 * With no saved failure: returns OK and writes *out=NULL (no allocation).
 * With a pre-start failure: returns OK and writes an owned FAILURE result;
 * facts/attempts are absent. With a started failure: returns OK and writes an
 * owned FAILURE result with final facts and opt-in attempts, even if []
 * because no send occurred. Both failure snapshots have absent schema,
 * answer_id and function, count=observation_count=0, and meta.present=0.
 * error is present; no origin/model/request/answer provenance is invented.
 * The return value describes snapshot creation, not the saved failure code.
 * out=NULL returns EUSAGE without changing the saved failure.
 */
int thinkthen_error_complete(const thinkthen_engine *, thinkthen_result **);

/* Unpublished 0426 integration carriers, pending whole-ticket qualification.
 * Complete calls use the existing native engine and immutable owned results.
 * Constructors clone caller buffers and referenced question/image values.
 * Image views borrow immutable image memory until image_free.
 * Frees accept NULL; nonnull handles must be live and freed exactly once
 * after all borrowers finish. Forged, stale and concurrently freed handles
 * violate this contract. Null required pointers and malformed descriptors
 * return EUSAGE. Every nonzero return leaves every output unchanged.
 */
int thinkthen_question_new(const thinkthen_engine *, const thinkthen_question_spec_v1 *, thinkthen_question **);
int thinkthen_question_load(const thinkthen_engine *, thinkthen_string_v1 path, thinkthen_question **);
/* Additive 0456 declarations. kind: ABSENT=0, STRING=1, OBJECT=2.
 * Object properties preserve authored order; property kinds STRING=1,
 * NUMBER=2, BOOLEAN=3, STRING_LIST=4. Required names preserve their order.
 * Non-object declarations require empty property/required arrays.
 * name/version absence is explicit; no inferred name or default version.
 */
enum thinkthen_input_declaration_kind_v1 {
    THINKTHEN_DECLARATION_ABSENT_V1=0, THINKTHEN_DECLARATION_STRING_V1=1,
    THINKTHEN_DECLARATION_OBJECT_V1=2
};
enum thinkthen_input_property_kind_v1 {
    THINKTHEN_PROPERTY_STRING_V1=1, THINKTHEN_PROPERTY_NUMBER_V1=2,
    THINKTHEN_PROPERTY_BOOLEAN_V1=3, THINKTHEN_PROPERTY_STRING_LIST_V1=4
};
enum thinkthen_question_loader_role_v1 {
    THINKTHEN_LOAD_ATOMIC_V1=1, THINKTHEN_LOAD_SET_V1=2,
    THINKTHEN_LOAD_DYNAMIC_CHOOSE_V1=3, THINKTHEN_LOAD_RECOGNIZE_V1=4,
    THINKTHEN_LOAD_RELATE_V1=5, THINKTHEN_LOAD_RANK_V1=6,
    THINKTHEN_LOAD_RANK_SET_V1=7, THINKTHEN_LOAD_FIND_V1=8
};
typedef struct thinkthen_input_property_v1 {
    thinkthen_string_v1 name; uint32_t kind;
} thinkthen_input_property_v1;
typedef struct thinkthen_input_properties_v1 {
    const thinkthen_input_property_v1 *data; size_t len;
} thinkthen_input_properties_v1;
typedef struct thinkthen_input_declaration_v1 {
    uint32_t kind;
    thinkthen_input_properties_v1 properties;
    thinkthen_strings_v1 required;
} thinkthen_input_declaration_v1;
typedef struct thinkthen_question_author_v1 {
    thinkthen_optional_string_v1 name;
    thinkthen_optional_u64_v1 wording_version;
    thinkthen_input_declaration_v1 item_schema, context_schema;
} thinkthen_question_author_v1;
/* Construct through the same native grammar, with separately counted author
 * metadata. author=NULL means no author metadata. All inputs are cloned.
 */
int thinkthen_question_new_authored(const thinkthen_engine *, const thinkthen_question_spec_v1 *, const thinkthen_question_author_v1 *, thinkthen_question **);
/* Borrow metadata owned by this immutable question until question_free. */
int thinkthen_question_author(const thinkthen_question *, thinkthen_question_author_v1 *);
/* Explicit native loader role: ATOMIC=1, SET=2, DYNAMIC_CHOOSE=3,
 * RECOGNIZE=4, RELATE=5, RANK=6, RANK_SET=7, FIND=8. No host search or fallback after file refusal.
 * These native-loaded handles execute directly. Embedding them as typed set
 * members awaits a native authored serialization/set composition API.
 */
/* Import saved question-file JSON through an explicit native grammar role.
 * This imports a question only; judgment calls and result fields remain typed.
 * Inline grammar failures return EUSAGE; no role guessing or parser fallback.
 * The immutable handle owns every byte independently of json's lifetime. */
int thinkthen_question_parse(const thinkthen_engine *, uint32_t role, thinkthen_string_v1 json, thinkthen_question **);
int thinkthen_question_load_named(const thinkthen_engine *, uint32_t role, thinkthen_string_v1 name, thinkthen_question **);
int thinkthen_question_load_reference(const thinkthen_engine *, uint32_t role, thinkthen_string_v1 reference, thinkthen_question **);
/* Borrow native author snapshots until result_free. Row/member ordinals
 * follow the existing named result accessors and annotation member order.
 * Set envelopes and generated internal questions carry absent author fields.
 * Invalid owner/output/ordinal or a member on a non-annotation row is Usage
 * and leaves output unchanged. Observation includes both question/row events.
 */
int thinkthen_result_question_author(const thinkthen_result *, size_t row, thinkthen_question_author_v1 *);
int thinkthen_result_member_author(const thinkthen_result *, size_t row, size_t member, thinkthen_question_author_v1 *);
int thinkthen_result_observation_author(const thinkthen_result *, size_t observation, thinkthen_question_author_v1 *);
void thinkthen_question_free(thinkthen_question *);
int thinkthen_image_clone(const thinkthen_engine *, const uint8_t *, size_t, uint32_t media, thinkthen_optional_string_v1 filename, thinkthen_image **);
int thinkthen_image_view(const thinkthen_image *, thinkthen_image_view_v1 *);
void thinkthen_image_free(thinkthen_image *);
int thinkthen_source_records(const thinkthen_engine *, const thinkthen_record_v1 *, size_t, thinkthen_source **);
int thinkthen_source_files(const thinkthen_engine *, const thinkthen_source_spec_v1 *, thinkthen_source **);
/* Explicit image media with physical unit line/window/file from the same
 * descriptor. Native admission rejects line/window before opening any path.
 * Existing source_files unit IMAGE_FILE remains a whole-image convenience. */
int thinkthen_source_image_files(const thinkthen_engine *, const thinkthen_source_spec_v1 *, thinkthen_source **);
void thinkthen_source_free(thinkthen_source *);

#ifdef __cplusplus
}
#endif
#endif
