#ifndef THINKTHEN_C_H
#define THINKTHEN_C_H

/* C11 caller API backed by the shared engine. */
#include <stdbool.h>
#include <stddef.h>
#include <jansson.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct tt_client tt_client;
typedef struct tt_call tt_call;
typedef struct tt_row tt_row;
typedef struct tt_error tt_error;

typedef struct tt_text {
    const char *data;
    size_t len;
} tt_text;

typedef struct tt_span {
    tt_text text;
    size_t start;
    size_t end;
} tt_span;

typedef enum tt_event { TT_ROW, TT_INPUT, TT_DONE, TT_ERROR } tt_event;
typedef enum tt_status { TT_ANSWERED, TT_UNSURE, TT_FAILED } tt_status;

/* Settings use schemas/client.schema.json. NULL means the default settings.
 * JSON inputs are borrowed through return, never retained or mutated.
 * Failed constructors return NULL and transfer an error to *error.
 * Every fallible operation requires an error slot and clears it on success.
 * A false send result transfers an independently owned error to *error.
 * Output slots must not already own a row or error.
 */
tt_client *tt_client_new(const json_t *settings, tt_error **error);
void tt_client_free(tt_client *client);

/* question uses the tagged question in schemas/request.schema.json.
 * options uses the selected function's options object; NULL omits it.
 * Named functions add only their function name and open a shared ABI call.
 * Records follow through send_text/send_json. C offsets count UTF-8 bytes.
 */
tt_call *tt_decide(const tt_client *, const json_t *question,
                   const json_t *options, tt_error **);
tt_call *tt_choose(const tt_client *, const json_t *question,
                   const json_t *options, tt_error **);
tt_call *tt_tag(const tt_client *, const json_t *question,
                const json_t *options, tt_error **);
tt_call *tt_score(const tt_client *, const json_t *question,
                  const json_t *options, tt_error **);
tt_call *tt_filter(const tt_client *, const json_t *question,
                   const json_t *options, tt_error **);
tt_call *tt_rank(const tt_client *, const json_t *question,
                 const json_t *options, tt_error **);
tt_call *tt_find(const tt_client *, const json_t *question,
                 const json_t *options, tt_error **);
tt_call *tt_annotate(const tt_client *, const json_t *question,
                     const json_t *options, tt_error **);
tt_call *tt_recognize(const tt_client *, const json_t *question,
                      const json_t *options, tt_error **);
tt_call *tt_relate(const tt_client *, const json_t *question,
                   const json_t *options, tt_error **);

/* next blocks for a row, input request or terminal result. TT_ROW transfers
 * one independently owned row to *row. TT_ERROR transfers one error to *error.
 * Other events leave both slots NULL. Release owners before reusing slots.
 * After TT_INPUT, send one record or end input before pulling again.
 */
tt_event tt_next(tt_call *, tt_row **row, tt_error **error);
bool tt_send_text(tt_call *, tt_text text, tt_error **error);
bool tt_send_json(tt_call *, const json_t *record, tt_error **error);
/* Counted JSON bypasses Jansson's signed integer range. Rust admits it. */
bool tt_send_raw_json(tt_call *, tt_text record, tt_error **error);
/* End input is idempotent and may precede the first pull for empty input. */
void tt_end_input(tt_call *);
void tt_cancel(tt_call *);
void tt_call_free(tt_call *);

/* Rows have no mutators. All returned values are borrowed until row_free.
 * Identifier, answer and detail are engine JSON bytes without a terminating
 * NUL. len == 0 means absent; otherwise data points to len readable bytes.
 * JSON null is the four-byte span "null". These values bypass Jansson so
 * its decoder cannot reject valid keys or narrow the engine's numbers.
 * Failed rows have no answer and expose their own borrowed error.
 */
tt_status tt_row_status(const tt_row *);
tt_text tt_row_id(const tt_row *);
tt_text tt_row_answer(const tt_row *);
/* True writes the probability to out; false leaves out unchanged. */
bool tt_row_probability(const tt_row *, double *out);
/* Typed answer accessors return false without changing out when the answer
 * is absent, null, another type, or cannot be decoded by Jansson. Raw JSON
 * remains available. Text borrows the row. Choose has one label at index 0;
 * tag has indexed labels. Find has one span at index 0; recognize has indexed
 * spans. A decoded answer is retained with the immutable row. */
bool tt_row_verdict(const tt_row *, bool *out);
bool tt_row_score(const tt_row *, double *out);
bool tt_row_label(const tt_row *, size_t index, tt_text *out);
bool tt_row_span(const tt_row *, size_t index, tt_span *out);
const tt_error *tt_row_error(const tt_row *);
tt_text tt_row_detail(const tt_row *);
/* Original input position, starting at one. */
size_t tt_row_position(const tt_row *);
bool tt_row_kept(const tt_row *);
void tt_row_free(tt_row *);

/* Code and message are counted UTF-8, borrowed until error_free or until the
 * owning row is freed. Native errors preserve the engine's code and message.
 * Only independently owned operation errors go to error_free.
 */
tt_text tt_error_code(const tt_error *);
tt_text tt_error_message(const tt_error *);
void tt_error_free(tt_error *);

/* Cleanup accepts NULL. Rows and operation errors survive calls and clients.
 * Calls retain their client state and survive client_free. Do not race
 * client_free with opening a call. A client may open independent calls
 * concurrently. Each call has one driver
 * for sends/end_input/next. cancel alone may run concurrently on a live call.
 * Cancel and let active host operations return before freeing a call.
 * Free never waits for a backend worker. The linked native library must remain
 * loaded for the process lifetime. No signal handler is installed.
 * All other pointer arguments must be live and non-NULL, except settings and
 * options as stated above. An empty tt_text may have data == NULL only when
 * len == 0. Output slots must be distinct and writable. Borrowed JSON trees
 * must not be mutated through nested values either. Caller-owned rows and
 * errors may move between threads; do not race their readers with cleanup.
 */
#ifdef __cplusplus
}
#endif
#endif
