/* Generated database declarations. Do not edit. */

#ifndef THINKTHEN_SQL_H
#define THINKTHEN_SQL_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include "thinkthen.h"

/**
 * thinkthen_sql_call_poll's timeout passed with nothing to report; poll again.
 */
#define THINKTHEN_PENDING 3

/**
 * A database extension's call for one statement; free it with thinkthen_sql_call_free.
 */
typedef struct thinkthen_sql_call thinkthen_sql_call;

/**
 * One database row's input for thinkthen_sql_call_send_chunk; copied before return.
 */
typedef struct thinkthen_sql_input {
  /**
   * How to read data, a thinkthen_record_kind; missing and invalid rows need no data.
   */
  uint32_t kind;
  /**
   * The row's bytes; NULL only when len is 0.
   */
  const char *data;
  /**
   * The byte length; not NUL-terminated.
   */
  size_t len;
} thinkthen_sql_input;

/**
 * A database's interrupt check: returns nonzero to stop the call. It must not throw,
 * unwind, long-jump or call back into this library.
 */
typedef uint8_t (*thinkthen_interrupt_probe)(void*);

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Open a client for a database extension from settings JSON and the administrator's allowed
 * backends and models.
 *
 * allowed is JSON, or empty for no limit. Returns THINKTHEN_OK with *out set, or a negative
 * code with *out set to NULL: -THINKTHEN_CODE_USAGE_SQL_POLICY for a policy that is not valid,
 * -THINKTHEN_CODE_USAGE_SQL_SELECTION when it does not allow the chosen backend and model,
 * another usage code for settings that are not valid, or -THINKTHEN_CODE_USAGE_ABI when the
 * text is not UTF-8. The caller owns *out and frees it with thinkthen_client_free; an old
 * handle in *out is not freed.
 * Pointers and threads:
 * Any thread may call this. settings points to len readable bytes, or to a NUL-terminated
 * string when len is THINKTHEN_NUL_TERMINATED; it is copied before return. allowed points to
 * allowed_len readable bytes, or to a NUL-terminated string when allowed_len is
 * THINKTHEN_NUL_TERMINATED; it is copied before return. out is writable. error is NULL or
 * initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and message;
 * passing it again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_sql_client_open_with_policy(const char *settings,
                                              size_t len,
                                              const char *allowed,
                                              size_t allowed_len,
                                              thinkthen_client **out,
                                              thinkthen_error *error);

/**
 * Open a client for a database extension from settings JSON, with no administrator limit.
 *
 * Returns THINKTHEN_OK with *out set, or a negative code with *out set to NULL: a usage code
 * for settings that are not valid, or -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8.
 * The caller owns *out and frees it with thinkthen_client_free; an old handle in *out is not
 * freed.
 * Pointers and threads:
 * Any thread may call this. settings points to len readable bytes, or to a NUL-terminated
 * string when len is THINKTHEN_NUL_TERMINATED; it is copied before return. out is writable.
 * error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the
 * code and message; passing it again frees what it held, so a reused error leaks nothing. Free
 * it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_sql_client_open(const char *settings,
                                  size_t len,
                                  thinkthen_client **out,
                                  thinkthen_error *error);

/**
 * Open a database call that asks question about each chunk of rows.
 *
 * Returns THINKTHEN_OK with *out set, or a negative code with *out set to NULL:
 * -THINKTHEN_CODE_USAGE_QUESTION for an incomplete question, -THINKTHEN_CODE_USAGE_NAME for a
 * bad name, or -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8. The caller owns *out and
 * frees it with thinkthen_sql_call_free; an old handle in *out is not freed.
 * Pointers and threads:
 * Any thread may call this while no thread changes question. name is NULL or points to a
 * readable thinkthen_text. out is writable. error is NULL or initialized with
 * THINKTHEN_ERROR_INIT. On a negative return it holds the code and message; passing it again
 * frees what it held, so a reused error leaks nothing. Free it with thinkthen_error_free after
 * its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_sql_call_open_question(const thinkthen_client *client,
                                         const thinkthen_question *question,
                                         const thinkthen_text *name,
                                         struct thinkthen_sql_call **out,
                                         thinkthen_error *error);

/**
 * Read one question from a database function's JSON argument.
 *
 * Returns THINKTHEN_OK with *out set to a new question, or a negative code with *out set to
 * NULL: -THINKTHEN_CODE_USAGE_QUESTION when the JSON is not a valid question, or
 * -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8. The caller owns *out and frees it with
 * thinkthen_question_free; an old handle in *out is not freed.
 * Pointers and threads:
 * Any thread may call this. data points to len readable bytes, or to a NUL-terminated string
 * when len is THINKTHEN_NUL_TERMINATED; it is copied before return. out is writable. error is
 * NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
 * message; passing it again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_sql_question_from_json(const char *data,
                                         size_t len,
                                         thinkthen_question **out,
                                         thinkthen_error *error);

/**
 * Set a database function's optional cut, or its band from low to high.
 *
 * Each pointer may be NULL to leave that value out; low and high come together. Returns
 * THINKTHEN_OK, or -THINKTHEN_CODE_USAGE_THRESHOLD with the question unchanged for values that
 * are out of range, conflict, or do not fit the function.
 * Pointers and threads:
 * One thread at a time may call this on a question; other questions may be used on other
 * threads. question is a live question. Each non-NULL pointer points to a readable double.
 * error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the
 * code and message; passing it again frees what it held, so a reused error leaks nothing. Free
 * it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_sql_question_set_threshold(thinkthen_question *question,
                                             const double *cut,
                                             const double *low,
                                             const double *high,
                                             thinkthen_error *error);

/**
 * Open a database call from request JSON.
 *
 * Returns THINKTHEN_OK with *out set, or a negative code with *out set to NULL: a usage code
 * for a request that is not valid, or -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8.
 * The caller owns *out and frees it with thinkthen_sql_call_free; an old handle in *out is not
 * freed.
 * Pointers and threads:
 * Any thread may call this. data points to len readable bytes, or to a NUL-terminated string
 * when len is THINKTHEN_NUL_TERMINATED; it is copied before return. out is writable. error is
 * NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
 * message; passing it again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_sql_call_open_json(const thinkthen_client *client,
                                     const char *data,
                                     size_t len,
                                     struct thinkthen_sql_call **out,
                                     thinkthen_error *error);

/**
 * Send count database rows to a call.
 *
 * Send only after thinkthen_sql_call_poll returns THINKTHEN_NEEDS_INPUT. Returns THINKTHEN_OK,
 * or -THINKTHEN_CODE_USAGE_ABI when the call is not waiting for input or a record is
 * malformed. A value that is not valid text fails as its own row.
 * Pointers and threads:
 * One database thread drives call. records points to count readable records whose bytes are
 * copied before return. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative
 * return it holds the code and message; passing it again frees what it held, so a reused error
 * leaks nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_sql_call_send_chunk(struct thinkthen_sql_call *call,
                                      const struct thinkthen_sql_input *records,
                                      size_t count,
                                      thinkthen_error *error);

/**
 * Send count PostgreSQL rows to a call, refusing a zero byte in JSON as PostgreSQL does.
 *
 * Send only after thinkthen_sql_call_poll returns THINKTHEN_NEEDS_INPUT. Returns THINKTHEN_OK,
 * or -THINKTHEN_CODE_USAGE_ABI when the call is not waiting for input or a record is
 * malformed. A value that is not valid text fails as its own row.
 * Pointers and threads:
 * One database thread drives call. records points to count readable records whose bytes are
 * copied before return. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative
 * return it holds the code and message; passing it again frees what it held, so a reused error
 * leaks nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_sql_call_send_chunk_postgres(struct thinkthen_sql_call *call,
                                               const struct thinkthen_sql_input *records,
                                               size_t count,
                                               thinkthen_error *error);

/**
 * Say no more rows are coming, and send the last partial batch.
 *
 * It returns at once.
 * Pointers and threads:
 * One database thread drives call. call is live.
 */
THINKTHEN_API void thinkthen_sql_call_end_input(struct thinkthen_sql_call *call);

/**
 * Wait for the next row, request for input, or end, on the database thread.
 *
 * Waits at most timeout_ms, calling interrupted(context) around the wait; when it returns
 * nonzero the call stops with -THINKTHEN_CODE_LOCAL_CANCELLED. Returns THINKTHEN_OK with the
 * next row in *out, THINKTHEN_NEEDS_INPUT, THINKTHEN_END, or a negative code for the call as a
 * whole. A failed record is THINKTHEN_OK with a failed row. *out is overwritten, not freed, so
 * free it with thinkthen_row_free before polling again.
 * Pointers and threads:
 * One database thread drives call. interrupted is NULL or a function that returns without
 * throwing, unwinding, long-jumping or calling back into this library. out is initialized with
 * THINKTHEN_ROW_INIT. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative
 * return it holds the code and message; passing it again frees what it held, so a reused error
 * leaks nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_sql_call_poll(struct thinkthen_sql_call *call,
                                uint32_t timeout_ms,
                                thinkthen_interrupt_probe interrupted,
                                void *context,
                                thinkthen_row *out,
                                thinkthen_error *error);

/**
 * Wait for the next row as thinkthen_sql_call_poll does, refusing text PostgreSQL cannot
 * store.
 *
 * Waits at most timeout_ms, calling interrupted(context) around the wait; when it returns
 * nonzero the call stops with -THINKTHEN_CODE_LOCAL_CANCELLED. Returns THINKTHEN_OK with the
 * next row in *out, THINKTHEN_NEEDS_INPUT, THINKTHEN_END, or a negative code for the call as a
 * whole. A failed record is THINKTHEN_OK with a failed row. *out is overwritten, not freed, so
 * free it with thinkthen_row_free before polling again.
 * Pointers and threads:
 * One database thread drives call. interrupted is NULL or a function that returns without
 * throwing, unwinding, long-jumping or calling back into this library. out is initialized with
 * THINKTHEN_ROW_INIT. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative
 * return it holds the code and message; passing it again frees what it held, so a reused error
 * leaks nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_sql_call_poll_postgres(struct thinkthen_sql_call *call,
                                         uint32_t timeout_ms,
                                         thinkthen_interrupt_probe interrupted,
                                         void *context,
                                         thinkthen_row *out,
                                         thinkthen_error *error);

/**
 * Give a PostgreSQL row the id the query selected.
 *
 * id NULL keeps the row's id. An id PostgreSQL cannot store turns the row into a failed row.
 * Returns THINKTHEN_OK, or -THINKTHEN_CODE_USAGE_ABI when row did not come from this library.
 * The row is changed in place; free it with thinkthen_row_free.
 * Pointers and threads:
 * One database thread drives call. row is a row this library wrote. id is NULL or points to a
 * readable thinkthen_text. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a
 * negative return it holds the code and message; passing it again frees what it held, so a
 * reused error leaks nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_sql_row_postgres_id(thinkthen_row *row,
                                      const thinkthen_text *id,
                                      thinkthen_error *error);

/**
 * Cancel and free a database call.
 *
 * Free a database call. NULL is ignored. It does not wait for a backend request in flight.
 * Pointers and threads:
 * Call this once, after every other use of call has returned.
 */
THINKTHEN_API void thinkthen_sql_call_free(struct thinkthen_sql_call *call);

/**
 * Ask a database call to stop.
 *
 * It returns at once.
 * Pointers and threads:
 * One database thread drives call. call is live.
 */
THINKTHEN_API void thinkthen_sql_call_cancel(struct thinkthen_sql_call *call);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* THINKTHEN_SQL_H */
