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
 * call started; it is NULL for a pre-call refusal. Nothing partial is
 * delivered: a cancelled or deadline-expired
 * bulk call returns its code with no rows. With a null engine, the error accessors
 * name the calling thread's last failed `thinkthen_engine_new` until that
 * thread's next engine builds.
 *
 * Version 0.1.0 freezes the symbol names, the `thinkthen_answer` layout,
 * the argument types (exact-width `size_t` lengths and counts, `int64_t`
 * budgets), and the return codes. Every open-shaped result crosses as JSON
 * text; `thinkthen_answer` is the one struct that crosses the boundary,
 * and it never grows a field.
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
#define THINKTHEN_EUSAGE 1      /* the request broke the grammar or the argument rules; nothing was sent */
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

#ifdef __cplusplus
}
#endif

#endif /* THINKTHEN_H */
