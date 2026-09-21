/*
 * thinkthen.h — the C door to the thinkthen engine, version 0.0.1.
 *
 * One archive per platform ships this header, the shared library, the
 * static library, and a `.pc` file. Every language that cannot bind Rust
 * directly binds this door: the JSON door carries any request, and the
 * typed `thinkthen_decide` and `thinkthen_decide_many` carry the hot path
 * without building JSON in the host.
 *
 * The prefix is `thinkthen_` on every name, and the public word for the
 * middle answer is UNSURE. An engine value carries its settings, holds no
 * thread between calls, and rebuilds its state after a fork. No call ever
 * blocks with the host's signal check unreachable: bulk waits run the poll
 * callback the host installed, on the calling thread, while they wait.
 *
 * The lifetime rules: an engine lives until `thinkthen_engine_free`; a
 * string from `thinkthen_call` lives until `thinkthen_free_string`; the
 * message from `thinkthen_error_message` lives until the next call on the
 * same engine. Nothing else is owned by the caller.
 */

#ifndef THINKTHEN_H
#define THINKTHEN_H

#ifdef __cplusplus
extern "C" {
#endif

/* Version 0.0.1. Version 0.1.0 is the first release. */
#define THINKTHEN_VERSION_MAJOR 0
#define THINKTHEN_VERSION_MINOR 0
#define THINKTHEN_VERSION_PATCH 1

/*
 * The return codes, one per error kind, and zero for success. A failure
 * never reads as an answer: on any nonzero code the out parameter holds
 * nothing and `thinkthen_error_message` names what failed.
 */
#define THINKTHEN_OK 0
#define THINKTHEN_EUSAGE 1      /* the request broke the grammar; nothing was sent */
#define THINKTHEN_EBACKEND 2   /* the wire failed or refused */
#define THINKTHEN_EDEADLINE 3  /* the caller's own budget ran out */
#define THINKTHEN_ELOCAL 4     /* a named local file or recording failed */
#define THINKTHEN_ECANCELLED 5 /* the token fired; sent requests finished */
#define THINKTHEN_EDEFECT 6    /* the engine broke its own contract */

/* The three answers a yes-or-no question gives. UNSURE is the public word;
 * the specification keeps "unresolved" in its own grammar. */
#define THINKTHEN_YES 1
#define THINKTHEN_NO 0
#define THINKTHEN_UNSURE 2

/* Opaque engine value, built from the environment and freed by the host. */
typedef struct thinkthen_engine thinkthen_engine;

/* One judgment: the answer under the question's rule and the probability
 * behind it. `outcome` is THINKTHEN_YES, THINKTHEN_NO, or THINKTHEN_UNSURE;
 * `probability` is the probability the backend gave the yes side. */
typedef struct thinkthen_answer {
    int outcome;
    double probability;
} thinkthen_answer;

/* Build an engine from the environment: THINKTHEN_BASE_URL for the
 * address, THINKTHEN_CACHE for a cache folder. Returns NULL only when the
 * process cannot hold an engine at all. */
thinkthen_engine *thinkthen_engine_new(void);

/* Free an engine. NULL is accepted and ignored. */
void thinkthen_engine_free(thinkthen_engine *engine);

/* The message for the last failure on this engine, valid until the next
 * call. A deadline's message names the limit and its value. Never NULL. */
const char *thinkthen_error_message(const thinkthen_engine *engine);

/* Whether a second try could help the last failure, in the caller's
 * fallback sense: busy and slow earn 1, refused and untrusted earn 0. */
int thinkthen_error_retryable(const thinkthen_engine *engine);

/* Ask one yes-or-no question of one text. `question_json` is one question
 * in the question-file grammar; `text` and `text_len` are the evidence.
 * The judgment lands in `out` on THINKTHEN_OK. */
int thinkthen_decide(const thinkthen_engine *engine, const char *question_json,
                     const char *text, unsigned long text_len,
                     thinkthen_answer *out);

/* Ask the same question of every text at once, at the engine's width, and
 * keep every judgment in input order. `texts` holds `count` pointers and
 * `lengths` holds their byte lengths; `out` holds room for `count`
 * answers. This is decide's bulk spelling, the one bulk entry point every
 * language loading this library uses. */
int thinkthen_decide_many(const thinkthen_engine *engine, const char *question_json,
                          const char *const *texts, const unsigned long *lengths,
                          unsigned long count, thinkthen_answer *out);

/* The JSON door: any request of the eight verbs as a JSON object, and the
 * answer as JSON text, which `thinkthen_free_string` frees. The request
 * carries the question file's own shape with the evidence beside it. This
 * is how a host reaches choose, score, tag, filter, rank, find, and
 * annotate before it grows a typed door, and how a test replays a
 * recording. Returns NULL on failure, with the code as the return of the
 * next `thinkthen_error_message` on the engine. */
char *thinkthen_call(const thinkthen_engine *engine, const char *request_json);

/* Find every name in one text, and the relations the rules allow. The
 * result has no fixed size, so it crosses as one JSON string written to
 * `*out` (freed with `thinkthen_free_string`) with its length in `*out_len`:
 * `{"entities": [...], "relations": [...]}` with `start` and `end`
 * counting code points of `text`. `spec_json` is the recognize section of
 * the question file (`kinds`, `relations`, `threshold`, `relation_threshold`);
 * a text the recordings do not hold is refused with the usage kind. Returns
 * 0 on success and -1 on failure, with `thinkthen_error_message` naming
 * what failed.
 *
 * The ends of a relation carry `source` and `target` on every surface,
 * this returned JSON and the question file included; `from` and `to` are
 * refused with the ruled spelling named. */
int thinkthen_recognize(const thinkthen_engine *engine, const char *spec_json,
                        const char *text, unsigned long text_len, char **out,
                        unsigned long *out_len);

/* Say how every record relates to the others, as one JSON object written
 * to `*out` (freed with `thinkthen_free_string`): `{"edges": [...]}`.
 * `texts` carries `count` records and `lengths` their lengths; `count` past
 * 255 is a usage refusal before anything else. `spec_json` is the relate
 * section (`relations` entries or bare names, `either`, optional
 * `kind_field`, `threshold`). Each edge carries `name`, `source`, `target`,
 * and `probability`. Returns 0 on success and -1 on failure, with
 * `thinkthen_error_message` naming what failed. */
int thinkthen_relate(const thinkthen_engine *engine, const char *spec_json,
                     const char *const *texts, const unsigned long *lengths,
                     unsigned long count, char **out, unsigned long *out_len);

/* Free a string `thinkthen_call`, `thinkthen_recognize`, or
 * `thinkthen_relate` returned. NULL is accepted and ignored. */
void thinkthen_free_string(char *text);

#ifdef __cplusplus
}
#endif

#endif /* THINKTHEN_H */
