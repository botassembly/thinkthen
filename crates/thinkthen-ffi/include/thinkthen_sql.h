/* Generated database declarations. Do not edit. */

#ifndef THINKTHEN_SQL_H
#define THINKTHEN_SQL_H

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <stddef.h>
#include <stdint.h>
#include "thinkthen.h"

/**
 * No event arrived during this bounded SQL wait.
 */
#define THINKTHEN_PENDING 3

/**
 * One statement's driver; no database handle enters the engine.
 */
typedef struct thinkthen_sql_call thinkthen_sql_call;

/**
 * Counted input copied before a database cursor advances.
 */
typedef struct thinkthen_sql_input {
  /**
   * Generated record kind; missing and invalid need no byte view.
   */
  uint32_t kind;
  /**
   * Readable bytes, or NULL at zero length.
   */
  const uint8_t *data;
  /**
   * Byte length without a terminator.
   */
  size_t len;
} thinkthen_sql_input;

/**
 * Read native interrupt state without throwing, unwinding or reentering.
 */
typedef uint8_t (*thinkthen_interrupt_probe)(void*);

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Admit SQL settings and optional administrator policy before resolving keys.
 * # Safety
 * Counted inputs are readable; out/error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_sql_client_open_with_policy(const uint8_t *settings,
                                              size_t len,
                                              const uint8_t *allowed,
                                              size_t allowed_len,
                                              thinkthen_client **out,
                                              thinkthen_error *error);

/**
 * Admit SQL settings without an administrator backend/model restriction.
 * # Safety
 * Counted settings are readable; out/error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_sql_client_open(const uint8_t *settings,
                                  size_t len,
                                  thinkthen_client **out,
                                  thinkthen_error *error);

/**
 * Open a SQL call from a copied typed question snapshot.
 * # Safety
 * Handles are live; output/error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_sql_call_open_question(const thinkthen_client *client,
                                         const thinkthen_question *question,
                                         const thinkthen_text *name,
                                         struct thinkthen_sql_call **out,
                                         thinkthen_error *error);

/**
 * Admit a structured SQL question with the shared per-function contract.
 * This consumes a question value, never a settings envelope or file reference.
 * # Safety
 * Counted input is readable; output/error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_sql_question_from_json(const uint8_t *data,
                                         size_t len,
                                         thinkthen_question **out,
                                         thinkthen_error *error);

/**
 * Admit SQL's optional cut or paired band without overriding conflicting inputs.
 * # Safety
 * Builder/error are exclusive; each non-NULL number pointer is readable.
 */
THINKTHEN_API
int32_t thinkthen_sql_question_set_threshold(thinkthen_question *question,
                                             const double *cut,
                                             const double *low,
                                             const double *high,
                                             thinkthen_error *error);

/**
 * Open an explicit canonical JSON request without an argument translation layer.
 * # Safety
 * Client/input remain live; output/error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_sql_call_open_json(const thinkthen_client *client,
                                     const uint8_t *data,
                                     size_t len,
                                     struct thinkthen_sql_call **out,
                                     thinkthen_error *error);

/**
 * Copy one current chunk after NEEDS_INPUT. Bad host text becomes a failed row.
 * # Safety
 * The sole driver owns call/error. The array lends count readable records.
 */
THINKTHEN_API
int32_t thinkthen_sql_call_send_chunk(struct thinkthen_sql_call *call,
                                      const struct thinkthen_sql_input *records,
                                      size_t count,
                                      thinkthen_error *error);

/**
 * Send PostgreSQL input with shared zero-byte JSON representation admission.
 * # Safety
 * The sole driver owns the call/error and borrows count readable input records.
 */
THINKTHEN_API
int32_t thinkthen_sql_call_send_chunk_postgres(struct thinkthen_sql_call *call,
                                               const struct thinkthen_sql_input *records,
                                               size_t count,
                                               thinkthen_error *error);

/**
 * End the source and flush its final partial batch.
 * # Safety
 * The sole driver owns a live call with no concurrent free.
 */
THINKTHEN_API void thinkthen_sql_call_end_input(struct thinkthen_sql_call *call);

/**
 * Poll on the database thread and probe interruption around the wait.
 * # Safety
 * The sole driver owns call, sized row/error and optional callback context.
 * The probe cannot throw, unwind, longjmp or reenter.
 */
THINKTHEN_API
int32_t thinkthen_sql_call_poll(struct thinkthen_sql_call *call,
                                uint32_t timeout_ms,
                                thinkthen_interrupt_probe interrupted,
                                void *context,
                                thinkthen_row *out,
                                thinkthen_error *error);

/**
 * Poll a PostgreSQL row after shared zero-byte representation admission.
 * # Safety
 * The sole driver owns call and supported row/error outputs and borrows the probe.
 */
THINKTHEN_API
int32_t thinkthen_sql_call_poll_postgres(struct thinkthen_sql_call *call,
                                         uint32_t timeout_ms,
                                         thinkthen_interrupt_probe interrupted,
                                         void *context,
                                         thinkthen_row *out,
                                         thinkthen_error *error);

/**
 * Copy a selected PostgreSQL id, or replace an unrepresentable id with a failed row.
 * A NULL view preserves the engine id; a present JSON null remains an explicit id.
 * # Safety
 * Row/error are live exclusive supported outputs; an optional id view is readable.
 */
THINKTHEN_API
int32_t thinkthen_sql_row_postgres_id(thinkthen_row *row,
                                      const thinkthen_text *id,
                                      thinkthen_error *error);

/**
 * Cancel and free without joining the backend worker.
 * # Safety
 * NULL or a unique live owner after driver operations finish.
 */
THINKTHEN_API void thinkthen_sql_call_free(struct thinkthen_sql_call *call);

/**
 * Request cancellation on the statement's owning database thread.
 * # Safety
 * The sole driver owns the live call with no concurrent operation or free.
 */
THINKTHEN_API void thinkthen_sql_call_cancel(struct thinkthen_sql_call *call);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* THINKTHEN_SQL_H */
