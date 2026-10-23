#ifndef THINKTHEN_H
#define THINKTHEN_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Operation results; distinct from row statuses and dotted error codes. */
#define THINKTHEN_OK INT32_C(0)
#define THINKTHEN_END INT32_C(1)
#define THINKTHEN_NEEDS_INPUT INT32_C(2)
#define THINKTHEN_ERROR (-INT32_C(1))

/* Shared Rust string-index conventions. */
#define THINKTHEN_UTF8 UINT32_C(0)
#define THINKTHEN_UTF16 UINT32_C(1)
#define THINKTHEN_CODE_POINTS UINT32_C(2)

typedef struct thinkthen_client thinkthen_client;
typedef struct thinkthen_call thinkthen_call;

/* Owned UTF-8 JSON, without a terminating NUL. Never release with free(). */
typedef struct thinkthen_buffer {
    uint8_t *data;
    size_t len;
} thinkthen_buffer;

/* One owned row. fields contains the contract row excluding top-level detail.
 * detail contains unparsed JSON; {NULL, 0} means that detail was absent.
 * position is the original input position, starting at one.
 * kept is the shared engine's selection result, represented as 0 or 1.
 * Release with thinkthen_row_free. Each row survives its call and client.
 */
typedef struct thinkthen_row {
    thinkthen_buffer fields;
    thinkthen_buffer detail;
    size_t position;
    uint8_t kept;
} thinkthen_row;

/* Admit client-settings JSON once. Returns OK or ERROR.
 * Copies/admits bytes before returning; *out is NULL on failure.
 * Every failing operation below publishes its own owned contract error JSON
 * {"code": ..., "message": ...} in *error. Other results leave *error empty.
 * Errors survive the operation, originating thread, client and call.
 */
int32_t thinkthen_client_open(const uint8_t *settings, size_t len,
                             thinkthen_client **out, thinkthen_buffer *error);

/* Release this client owner. Existing calls retain their engine independently.
 * NULL is accepted. Do not race with call_open on this client.
 */
void thinkthen_client_free(thinkthen_client *client);

/* Admit call-request JSON, with initial records before later sent records.
 * offsets selects the shared Rust OffsetUnit above.
 * Returns OK or ERROR; *out is NULL on failure. Admission never waits for
 * output consumption. Client settings cannot appear in a call request.
 */
int32_t thinkthen_call_open(const thinkthen_client *client,
                           const uint8_t *request, size_t len,
                           uint32_t offsets,
                           thinkthen_call **out, thinkthen_buffer *error);

/* Supply one raw UTF-8 text record after next returns NEEDS_INPUT.
 * No JSON wrapping or escaping is required. Returns OK or ERROR.
 * Input bytes are copied before return; no foreign pointer is retained.
 */
int32_t thinkthen_send_text(thinkthen_call *call, const uint8_t *text, size_t len,
                           thinkthen_buffer *error);

/* Supply one raw native-endian UTF-16 text record after NEEDS_INPUT.
 * len counts uint16_t code units, without a terminating NUL. Rust's strict
 * decoder returns usage.record for an unpaired surrogate before transfer.
 * text is aligned and readable for len units, or NULL only when len is zero.
 * Returns OK or ERROR; units are copied before return, no pointer is retained.
 */
int32_t thinkthen_send_utf16(thinkthen_call *call, const uint16_t *text, size_t len,
                            thinkthen_buffer *error);

/* Supply one structured JSON record after next returns NEEDS_INPUT.
 * Rust owns parsing and evidence selection. Record errors may arrive in a
 * failed row. An OK send acknowledges transfer, not successful evaluation.
 * Returns OK or ERROR; input bytes are copied before return.
 */
int32_t thinkthen_send_json(thinkthen_call *call, const uint8_t *json, size_t len,
                           thinkthen_buffer *error);

/* Close input, then pull to drain results. Idempotent; never waits for queue
 * capacity, output consumption or backend completion. May close before the
 * first next call when the request already contains all input records.
 */
void thinkthen_end_input(thinkthen_call *call);

/* Pull a row, input request or terminal result. May wait for backend work,
 * but returns NEEDS_INPUT when the engine waits for a record. On that result,
 * send one record or end input before calling next again.
 * OK publishes one owned row; all other results leave *out empty.
 * END is successful completion; ERROR publishes the call's final failure.
 * Failed rows remain OK results. After all rows, ERROR reports the first
 * failure. A repeated terminal read repeats that outcome.
 */
int32_t thinkthen_next(thinkthen_call *call, thinkthen_row *out,
                      thinkthen_buffer *error);

/* Request cancellation from any thread while another operation is active.
 * Idempotent. Pulling after cancellation reports local.cancelled through
 * its returned error JSON. Already delivered rows remain independently owned.
 * Stops new requests and aborts in-flight backend requests.
 */
void thinkthen_cancel(thinkthen_call *call);

/* Cancel and release the host call immediately, without joining the worker
 * or waiting for an in-flight backend request. Unread output is discarded.
 * No host operation may be active or start during free. Cancel and finish
 * active host operations first. NULL is accepted.
 */
void thinkthen_free(thinkthen_call *call);

/* Release an owned error buffer and clear its fields. NULL/empty accepted.
 * No reader may still use the buffer. Ownership may move between threads.
 */
void thinkthen_buffer_free(thinkthen_buffer *buffer);

/* Release both row buffers and clear all fields. NULL/empty accepted. */
void thinkthen_row_free(thinkthen_row *row);

/* Project a live row into one owned JSON envelope without parsing payload keys.
 * status and p retain their ordinary JSON scalar values. Present id/answer/error
 * fields are JSON strings containing their original raw JSON bytes; absent
 * fields stay absent, distinct from a present answer containing JSON null.
 * Detail, position and kept remain on the original row. The original row is unchanged; out survives its release.
 * Returns OK or ERROR. Release out with thinkthen_buffer_free. Required out and
 * error slots are distinct and empty, as for every other owned-buffer operation.
 */
int32_t thinkthen_row_columns(const thinkthen_row *row,
                                 thinkthen_buffer *out, thinkthen_buffer *error);

/* Shared SQL host: one statement call across native source chunks.
 * All pointer, buffer, row and single-driver contracts above apply here.
 * The client may be freed after open; the admitted call retains its snapshot.
 */
typedef struct thinkthen_sql_call thinkthen_sql_call;
#define THINKTHEN_SQL_TEXT UINT32_C(0)
#define THINKTHEN_SQL_JSON UINT32_C(1)
#define THINKTHEN_WAIT INT32_C(3)
typedef struct thinkthen_sql_record {
    uint32_t kind;
    const uint8_t *data;
    size_t len;
} thinkthen_sql_record;
typedef uint8_t (*thinkthen_interrupt_probe)(void *context);

/* Database configuration MUST use this entry point, including defaults.
 * It rejects SQL url/key_source fields, even null, before constructing a client.
 * Backend address and credential-variable selection belong to the process owner.
 * Release the result with thinkthen_client_free.
 */
int32_t thinkthen_sql_client_open(const uint8_t *settings, size_t len,
                                 thinkthen_client **out, thinkthen_buffer *error);
/* As above, admitting an administrator's backend-to-model-list JSON policy
 * before resolving credentials. NULL/zero policy permits the default pair. */
int32_t thinkthen_sql_client_open_with_policy(const uint8_t *settings, size_t len,
    const uint8_t *allowed, size_t allowed_len,
    thinkthen_client **out, thinkthen_buffer *error);


/* Function, question and options are copied/admitted before return. A question
 * beginning with '{' after leading whitespace is tagged JSON; other text is
 * inline. NULL options with zero length use the shared defaults. Non-NULL
 * empty options text reaches ordinary JSON admission. No argument names a file.
 */
int32_t thinkthen_sql_open(const thinkthen_client *client,
                          const uint8_t *function, size_t function_len,
                          const uint8_t *question, size_t question_len,
                          const uint8_t *options, size_t options_len,
                          thinkthen_sql_call **out, thinkthen_buffer *error);

/* Submit one chunk after NEEDS_INPUT. Copies all bytes before return.
 * NULL records is allowed only at zero count. Empty chunks do not end input.
 */
int32_t thinkthen_sql_send_chunk(thinkthen_sql_call *call,
                                const thinkthen_sql_record *records, size_t count,
                                thinkthen_buffer *error);
void thinkthen_sql_end_input(thinkthen_sql_call *call);

/* OK publishes one owned row; NEEDS_INPUT requests another chunk; WAIT has no
 * event; END succeeds; ERROR publishes a shared error. All other row slots are
 * empty. Poll waits no longer than timeout_ms for an event. The optional probe
 * runs only on this calling thread, before/after waiting and between records;
 * it is not retained. Context is borrowed only until poll returns. The probe
 * only reads a native interrupt flag and MUST NOT throw, unwind, longjmp or
 * reenter the driver. A nonzero result requests shared cancellation. Raise a
 * native database interrupt only after poll returns and call cleanup completes.
 */
int32_t thinkthen_sql_poll(thinkthen_sql_call *call, uint32_t timeout_ms,
                          thinkthen_interrupt_probe interrupted, void *context,
                          thinkthen_row *out, thinkthen_buffer *error);
/* Cancel and drop without joining. NULL accepted; no concurrent operation. */
void thinkthen_sql_free(thinkthen_sql_call *call);

/* Use one driver for call_open/send/end_input/next on each call; do not run
 * these operations concurrently. cancel alone may run concurrently. Open
 * independent calls concurrently, including through the same live client.
 * Input byte pointers may be NULL only when len is zero. All other pointers
 * must be live, valid and aligned except NULL accepted by cleanup functions.
 * Output/error slots are required, distinct, writable, and hold no live owner.
 * Arbitrary pointers, use after free, double ownership and concurrent free
 * are caller errors with undefined behavior. Keep the native library loaded
 * for the process lifetime; freeing a call does not join its worker. No Rust
 * panic or foreign exception may unwind across this interface.
 */

#ifdef __cplusplus
}
#endif
#endif
