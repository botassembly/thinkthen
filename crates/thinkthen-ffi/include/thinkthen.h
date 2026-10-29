/* Generated from Rust exports and contract registries. Do not edit. */
#ifndef THINKTHEN_API
#if defined(_WIN32) && defined(THINKTHEN_BUILD)
#define THINKTHEN_API __declspec(dllexport)
#elif defined(_WIN32)
#define THINKTHEN_API __declspec(dllimport)
#elif defined(__GNUC__)
#define THINKTHEN_API __attribute__((visibility("default")))
#else
#define THINKTHEN_API
#endif
#endif
#ifndef THINKTHEN_REGISTRY_H
#define THINKTHEN_REGISTRY_H
#define THINKTHEN_ABI_VERSION 131074u
#define THINKTHEN_VERSION_MAJOR 0u
#define THINKTHEN_VERSION_MINOR 2u
#define THINKTHEN_VERSION_PATCH 0u
#define THINKTHEN_ABI_ERROR_CODE "usage.abi"
#define THINKTHEN_ABI_ERROR_MESSAGE "the native ABI is incompatible"
typedef enum thinkthen_code {
  THINKTHEN_CODE_USAGE_THRESHOLD = 1001,
  THINKTHEN_CODE_USAGE_QUESTION = 1002,
  THINKTHEN_CODE_USAGE_NAME = 1003,
  THINKTHEN_CODE_USAGE_SETTINGS = 1004,
  THINKTHEN_CODE_USAGE_BACKEND = 1005,
  THINKTHEN_CODE_USAGE_URL = 1006,
  THINKTHEN_CODE_USAGE_KEY_MISSING = 1007,
  THINKTHEN_CODE_USAGE_BATCH = 1008,
  THINKTHEN_CODE_USAGE_POINTER = 1009,
  THINKTHEN_CODE_USAGE_SQL_POLICY = 1010,
  THINKTHEN_CODE_USAGE_SQL_SELECTION = 1011,
  THINKTHEN_CODE_USAGE_ABI = 1012,
  THINKTHEN_CODE_RECORD_JSON = 2001,
  THINKTHEN_CODE_RECORD_MISSING = 2002,
  THINKTHEN_CODE_RECORD_TOO_LARGE = 2003,
  THINKTHEN_CODE_RECORD_INVALID = 2004,
  THINKTHEN_CODE_BACKEND_TRANSPORT = 3001,
  THINKTHEN_CODE_BACKEND_STATUS = 3002,
  THINKTHEN_CODE_BACKEND_REPLY = 3003,
  THINKTHEN_CODE_LOCAL_CANCELLED = 4001,
  THINKTHEN_CODE_LOCAL_IO = 4002,
  THINKTHEN_CODE_LOCAL_TOOLBOX = 4003,
  THINKTHEN_CODE_INTERNAL_BUG = 5001,
} thinkthen_code;
#define THINKTHEN_FUNCTION_DECIDE 1u
#define THINKTHEN_FUNCTION_CHOOSE 2u
#define THINKTHEN_FUNCTION_TAG 3u
#define THINKTHEN_FUNCTION_SCORE 4u
#define THINKTHEN_FUNCTION_FILTER 5u
#define THINKTHEN_FUNCTION_FIND 7u
#define THINKTHEN_FUNCTION_RECOGNIZE 8u
#define THINKTHEN_FUNCTION_RELATE 9u
#define THINKTHEN_FUNCTION_RANK 6u
#define THINKTHEN_FUNCTION_ANNOTATE 10u
#define THINKTHEN_BACKEND_SYSTEMONE 1u
#define THINKTHEN_BACKEND_OPENAI 2u
#define THINKTHEN_BACKEND_STUB 3u
#define THINKTHEN_STATUS_ANSWERED 1u
#define THINKTHEN_STATUS_UNSURE 2u
#define THINKTHEN_STATUS_FAILED 3u
#define THINKTHEN_RECORD_TEXT 0u
#define THINKTHEN_RECORD_JSON 1u
#define THINKTHEN_RECORD_MISSING 2u
#define THINKTHEN_RECORD_INVALID 3u
#define THINKTHEN_OFFSET_UTF8 0u
#define THINKTHEN_OFFSET_UTF16 1u
#define THINKTHEN_OFFSET_CODE_POINTS 2u
#define THINKTHEN_OFFSET_CODE_POINTS_INCLUSIVE 3u
#define THINKTHEN_ID_ABSENT 0u
#define THINKTHEN_ID_STRING 1u
#define THINKTHEN_ID_SIGNED 2u
#define THINKTHEN_ID_UNSIGNED 3u
#endif /* THINKTHEN_REGISTRY_H */


#ifndef THINKTHEN_H
#define THINKTHEN_H

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <stddef.h>
#include <stdint.h>

/**
 * A row or successful operation.
 */
#define THINKTHEN_OK 0

/**
 * Successful completion, including after record failures.
 */
#define THINKTHEN_END 1

/**
 * One input slot is reserved before the host sends its next record.
 */
#define THINKTHEN_NEEDS_INPUT 2

/**
 * A call-level error, separate from the row status and code registry.
 */
#define THINKTHEN_ERROR -1

/**
 * An independently retained event source, safe after the call is freed.
 */
typedef struct thinkthen_events thinkthen_events;

/**
 * One driving thread owns the cell; cancel accesses only its separate token.
 */
typedef struct thinkthen_call thinkthen_call;

/**
 * An independently retained admitted engine.
 */
typedef struct thinkthen_client thinkthen_client;

/**
 * Mutable typed question with no retained caller-owned pointers.
 */
typedef struct thinkthen_question thinkthen_question;

/**
 * Mutable settings copied when a client is opened.
 */
typedef struct thinkthen_settings thinkthen_settings;

/**
 * Named typed snapshots that survive their inserted children.
 */
typedef struct thinkthen_toolbox thinkthen_toolbox;

/**
 * An independent typed snapshot; every returned view borrows this owner.
 */
typedef struct thinkthen_value thinkthen_value;

/**
 * An engine allocation, released only by its matching cleanup operation.
 */
typedef struct thinkthen_buffer {
  /**
   * Counted UTF-8 bytes without a terminating NUL.
   */
  uint8_t *data;
  /**
   * Byte length.
   */
  size_t len;
} thinkthen_buffer;

/**
 * A sized owned error; code zero means no error and HTTP status zero means absent.
 */
typedef struct thinkthen_error {
  /**
   * The caller's allocation size, preserved by cleanup.
   */
  uint32_t struct_size;
  /**
   * A stable generated code, or zero.
   */
  int32_t code;
  /**
   * Optional HTTP status; zero means absent.
   */
  uint16_t http_status;
  /**
   * Owned plain UTF-8 message, independent of the call.
   */
  struct thinkthen_buffer message;
} thinkthen_error;

/**
 * Counted borrowed UTF-8. Setters copy input; projections borrow their value owner.
 */
typedef struct thinkthen_text {
  /**
   * Readable bytes, NULL only when length is zero.
   */
  const uint8_t *data;
  /**
   * Byte length without terminating NUL.
   */
  size_t len;
} thinkthen_text;

/**
 * A typed borrowed relation rule. A NULL reads pointer means omitted wording.
 */
typedef struct thinkthen_rule {
  /**
   * Relation name.
   */
  struct thinkthen_text relation;
  /**
   * Source kind.
   */
  struct thinkthen_text source;
  /**
   * Target kind.
   */
  struct thinkthen_text target;
  /**
   * Optional explicit wording.
   */
  struct thinkthen_text reads;
  /**
   * Zero for directed, one for unordered.
   */
  uint8_t either;
} thinkthen_rule;

/**
 * One independently owned typed row. Scalar fields are selected by function/status.
 */
typedef struct thinkthen_row {
  /**
   * The caller's allocation size, preserved by cleanup.
   */
  uint32_t struct_size;
  /**
   * Generated function discriminator.
   */
  uint32_t function;
  /**
   * Generated status discriminator.
   */
  uint32_t status;
  /**
   * Original one-based input position.
   */
  uint64_t position;
  /**
   * Full precision aggregate probability, or NaN when absent.
   */
  double probability;
  /**
   * Failed-record code, or zero.
   */
  int32_t error_code;
  /**
   * Answered decide/filter value as zero or one.
   */
  uint8_t answer_bool;
  /**
   * Answered rank, independent of position.
   */
  uint64_t rank;
  /**
   * Answered score.
   */
  double number;
  /**
   * Optional original id as exact contract JSON.
   */
  struct thinkthen_buffer id;
  /**
   * Typed answer as contract JSON, absent for unsure/failed.
   */
  struct thinkthen_buffer answer;
  /**
   * Optional typed function detail as contract JSON.
   */
  struct thinkthen_buffer detail;
  /**
   * Typed attempted-route source as contract JSON.
   */
  struct thinkthen_buffer source;
  /**
   * Failed record's typed contract error as JSON.
   */
  struct thinkthen_buffer error;
} thinkthen_row;

/**
 * One borrowed finite input; its byte view is copied before open returns.
 */
typedef struct thinkthen_record_input {
  /**
   * Allocated descriptor size, at least the current prefix and at most record_stride.
   */
  uint32_t struct_size;
  /**
   * Generated record kind; missing and invalid need no byte view.
   */
  uint32_t kind;
  /**
   * Readable bytes for text or JSON, or NULL at zero length.
   */
  const uint8_t *data;
  /**
   * Byte length without a terminator.
   */
  size_t len;
} thinkthen_record_input;

/**
 * The original filter input. Bytes retain text/JSON encoding even for failed rows.
 * Missing and explicit conversion-failure inputs have empty bytes. Invalid UTF-8
 * text or JSON still has its original kind and bytes, so this is not a text view.
 */
typedef struct thinkthen_record_view {
  /**
   * Generated record-kind discriminator.
   */
  uint32_t kind;
  /**
   * Borrowed bytes owned by the row or typed value snapshot.
   */
  const uint8_t *data;
  /**
   * Count without a terminating NUL.
   */
  size_t len;
} thinkthen_record_view;

/**
 * A typed borrowed text span; present distinguishes answered null from an empty span.
 */
typedef struct thinkthen_span {
  /**
   * Zero means find answered no match.
   */
  uint8_t present;
  /**
   * Decoded UTF-8 text owned by the value handle.
   */
  struct thinkthen_text text;
  /**
   * Start in the call's selected offset unit.
   */
  uint64_t start;
  /**
   * End in the selected offset unit.
   */
  uint64_t end;
} thinkthen_span;

/**
 * A recognized name and its support probability.
 */
typedef struct thinkthen_name {
  /**
   * Original decoded UTF-8 text.
   */
  struct thinkthen_text text;
  /**
   * Span start.
   */
  uint64_t start;
  /**
   * Span end.
   */
  uint64_t end;
  /**
   * Decoded kind name.
   */
  struct thinkthen_text kind;
  /**
   * Full precision support probability.
   */
  double probability;
} thinkthen_name;

/**
 * A relation endpoint has no invented aggregate probability.
 */
typedef struct thinkthen_endpoint {
  /**
   * Original endpoint text.
   */
  struct thinkthen_text text;
  /**
   * Span start.
   */
  uint64_t start;
  /**
   * Span end.
   */
  uint64_t end;
  /**
   * Endpoint kind.
   */
  struct thinkthen_text kind;
} thinkthen_endpoint;

/**
 * A typed relation, usable for answers and candidate-pair detail alike.
 */
typedef struct thinkthen_relation {
  /**
   * Typed source endpoint.
   */
  struct thinkthen_endpoint source;
  /**
   * Typed target endpoint.
   */
  struct thinkthen_endpoint target;
  /**
   * Decoded relation name.
   */
  struct thinkthen_text relation;
  /**
   * Full precision relation probability.
   */
  double probability;
  /**
   * One for an unordered rule, zero otherwise.
   */
  uint8_t either;
} thinkthen_relation;

/**
 * One caller-domain or boundary label and its probability.
 */
typedef struct thinkthen_probability {
  /**
   * Decoded label.
   */
  struct thinkthen_text label;
  /**
   * Full precision probability.
   */
  double probability;
} thinkthen_probability;

/**
 * The attempted route, including on failed rows.
 */
typedef struct thinkthen_source {
  /**
   * Stable question identity.
   */
  struct thinkthen_text question_identity;
  /**
   * Generated backend discriminator.
   */
  uint32_t backend;
  /**
   * Resolved model name.
   */
  struct thinkthen_text model;
} thinkthen_source;

/**
 * Exact identifier values never pass through a floating-point representation.
 */
typedef struct thinkthen_id {
  /**
   * Generated identifier-kind discriminator.
   */
  uint32_t kind;
  /**
   * String value, usable only for the string kind.
   */
  struct thinkthen_text text;
  /**
   * Integer value, usable only for the signed kind.
   */
  int64_t signed_value;
  /**
   * Integer value, usable only for the unsigned kind.
   */
  uint64_t unsigned_value;
} thinkthen_id;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Start one typed question; domain-less functions require empty text.
 * # Safety
 * Counted text is readable; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_question_new(uint32_t function,
                               const uint8_t *data,
                               size_t len,
                               struct thinkthen_question **out,
                               struct thinkthen_error *error);

/**
 * Add a copied choose choice, leaving the builder unchanged on rejection.
 * # Safety
 * One driver owns the builder and empty error slot; text is counted readable UTF-8.
 */
THINKTHEN_API
int32_t thinkthen_question_add_choice(struct thinkthen_question *question,
                                      const uint8_t *data,
                                      size_t len,
                                      struct thinkthen_error *error);

/**
 * Add a copied tag label.
 * # Safety
 * One driver owns the builder and error; text is counted readable UTF-8.
 */
THINKTHEN_API
int32_t thinkthen_question_add_label(struct thinkthen_question *question,
                                     const uint8_t *data,
                                     size_t len,
                                     struct thinkthen_error *error);

/**
 * Add a copied score or rank level.
 * # Safety
 * One driver owns the builder and error; text is counted readable UTF-8.
 */
THINKTHEN_API
int32_t thinkthen_question_add_level(struct thinkthen_question *question,
                                     const uint8_t *data,
                                     size_t len,
                                     struct thinkthen_error *error);

/**
 * Add a copied recognition kind.
 * # Safety
 * One driver owns the builder and error; text is counted readable UTF-8.
 */
THINKTHEN_API
int32_t thinkthen_question_add_kind(struct thinkthen_question *question,
                                    const uint8_t *data,
                                    size_t len,
                                    struct thinkthen_error *error);

/**
 * Add one fully admitted relation rule copied from borrowed fields.
 * # Safety
 * Builder and rule fields are live; error is empty exclusive storage.
 */
THINKTHEN_API
int32_t thinkthen_question_add_rule(struct thinkthen_question *question,
                                    const struct thinkthen_rule *rule,
                                    struct thinkthen_error *error);

/**
 * Set a validated numeric cut for a threshold-taking function.
 * # Safety
 * A sole driver owns the builder and empty error slot.
 */
THINKTHEN_API
int32_t thinkthen_question_set_cut(struct thinkthen_question *question,
                                   double cut,
                                   struct thinkthen_error *error);

/**
 * Set a band for decide, filter or choose; numeric-only functions reject it.
 * # Safety
 * A sole driver owns the builder and empty error slot.
 */
THINKTHEN_API
int32_t thinkthen_question_set_band(struct thinkthen_question *question,
                                    double low,
                                    double high,
                                    struct thinkthen_error *error);

/**
 * Copy an admitted evidence pointer.
 * # Safety
 * A sole driver owns the builder and empty error; text is counted readable UTF-8.
 */
THINKTHEN_API
int32_t thinkthen_question_set_evidence(struct thinkthen_question *question,
                                        const uint8_t *data,
                                        size_t len,
                                        struct thinkthen_error *error);

/**
 * Bound successful rank output, retaining failed rows.
 * # Safety
 * A sole driver owns the builder and empty error slot.
 */
THINKTHEN_API
int32_t thinkthen_question_set_top(struct thinkthen_question *question,
                                   size_t top,
                                   struct thinkthen_error *error);

/**
 * Snapshot one eligible child into annotation; rank and annotate are rejected.
 * # Safety
 * Both builders and counted name are live; error is exclusive and empty.
 */
THINKTHEN_API
int32_t thinkthen_question_add_entry(struct thinkthen_question *question,
                                     const uint8_t *name,
                                     size_t len,
                                     const struct thinkthen_question *child,
                                     struct thinkthen_error *error);

/**
 * Release a question; calls, toolboxes and annotations retain owned snapshots.
 * # Safety
 * NULL or the unique allocation after all readers finish.
 */
THINKTHEN_API void thinkthen_question_free(struct thinkthen_question *question);

/**
 * Copy a complete admitted question into an independent mutable builder.
 * # Safety
 * Source is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_question_clone(const struct thinkthen_question *source,
                                 struct thinkthen_question **out,
                                 struct thinkthen_error *error);

/**
 * Admit one canonical question through the shared grammar without executing it.
 * # Safety
 * Counted UTF-8 and distinct empty output slots remain live until return.
 */
THINKTHEN_API
int32_t thinkthen_question_from_json(const uint8_t *data,
                                     size_t len,
                                     struct thinkthen_question **out,
                                     struct thinkthen_error *error);

/**
 * Allocate default typed settings without reading keys or environment.
 * # Safety
 * Out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_settings_new(struct thinkthen_settings **out,
                               struct thinkthen_error *error);

/**
 * Select a typed backend before opening a client.
 * # Safety
 * Settings is live and exclusively driven; error is empty writable storage.
 */
THINKTHEN_API
int32_t thinkthen_settings_set_backend(struct thinkthen_settings *settings,
                                       uint32_t backend,
                                       struct thinkthen_error *error);

/**
 * Copy a validated direct API key; debug output never reveals it.
 * # Safety
 * Builder and counted bytes are live; error is exclusive and empty.
 */
THINKTHEN_API
int32_t thinkthen_settings_set_api_key(struct thinkthen_settings *settings,
                                       const uint8_t *data,
                                       size_t len,
                                       struct thinkthen_error *error);

/**
 * Copy a model name without resolving the backend or credentials.
 * # Safety
 * Builder and counted bytes are live; error is exclusive and empty.
 */
THINKTHEN_API
int32_t thinkthen_settings_set_model(struct thinkthen_settings *settings,
                                     const uint8_t *data,
                                     size_t len,
                                     struct thinkthen_error *error);

/**
 * Copy an admitted URL without reading credentials.
 * # Safety
 * Builder and counted bytes are live; error is exclusive and empty.
 */
THINKTHEN_API
int32_t thinkthen_settings_set_url(struct thinkthen_settings *settings,
                                   const uint8_t *data,
                                   size_t len,
                                   struct thinkthen_error *error);

/**
 * Select a validated positive batch count.
 * # Safety
 * Builder is live and exclusive; error is empty writable storage.
 */
THINKTHEN_API
int32_t thinkthen_settings_set_batch(struct thinkthen_settings *settings,
                                     uint64_t batch,
                                     struct thinkthen_error *error);

/**
 * Copy a validated retry policy expressed in whole milliseconds.
 * # Safety
 * Builder is live and exclusive; error is empty writable storage.
 */
THINKTHEN_API
int32_t thinkthen_settings_set_retry(struct thinkthen_settings *settings,
                                     uint32_t max_retries,
                                     uint64_t initial_backoff_ms,
                                     uint64_t attempt_timeout_ms,
                                     struct thinkthen_error *error);

/**
 * Release a settings builder; already opened clients own their snapshots.
 * # Safety
 * NULL or the unique live builder after all readers finish.
 */
THINKTHEN_API void thinkthen_settings_free(struct thinkthen_settings *settings);

/**
 * Copy an independent settings snapshot without resolving environment variables.
 * # Safety
 * Source is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_settings_clone(const struct thinkthen_settings *source,
                                 struct thinkthen_settings **out,
                                 struct thinkthen_error *error);

/**
 * Replace only the explicit key from an opaque settings snapshot.
 * # Safety
 * Both handles are live; destination is exclusive; error is empty writable storage.
 */
THINKTHEN_API
int32_t thinkthen_settings_copy_api_key(struct thinkthen_settings *destination,
                                        const struct thinkthen_settings *source,
                                        struct thinkthen_error *error);

/**
 * Admit canonical settings with omitted fields resolved by the engine defaults.
 * # Safety
 * Counted bytes are live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_settings_from_json(const uint8_t *data,
                                     size_t len,
                                     struct thinkthen_settings **out,
                                     struct thinkthen_error *error);

/**
 * Allocate an initially empty collection of named typed questions.
 * # Safety
 * Out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_toolbox_new(struct thinkthen_toolbox **out,
                              struct thinkthen_error *error);

/**
 * Insert a copied non-annotate question; duplicate names leave the toolbox unchanged.
 * # Safety
 * Live handles and counted name remain borrowed until return; error is empty.
 */
THINKTHEN_API
int32_t thinkthen_toolbox_insert(struct thinkthen_toolbox *toolbox,
                                 const uint8_t *name,
                                 size_t len,
                                 const struct thinkthen_question *question,
                                 struct thinkthen_error *error);

/**
 * Select an independent owned question that survives toolbox destruction.
 * # Safety
 * Handles and counted name are live; outputs are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_toolbox_select(const struct thinkthen_toolbox *toolbox,
                                 const uint8_t *name,
                                 size_t len,
                                 struct thinkthen_question **out,
                                 struct thinkthen_error *error);

/**
 * Release a toolbox; selected questions and opened calls remain independent.
 * # Safety
 * NULL or the unique live collection after all readers finish.
 */
THINKTHEN_API void thinkthen_toolbox_free(struct thinkthen_toolbox *toolbox);

/**
 * Admit canonical toolbox JSON without resolving files or executing entries.
 * # Safety
 * Counted UTF-8 and distinct empty output slots remain live until return.
 */
THINKTHEN_API
int32_t thinkthen_toolbox_from_json(const uint8_t *data,
                                    size_t len,
                                    struct thinkthen_toolbox **out,
                                    struct thinkthen_error *error);

/**
 * Select an owned snapshot of the requested function, with shared mismatch errors.
 * # Safety
 * Handles, counted name and distinct empty output slots are live until return.
 */
THINKTHEN_API
int32_t thinkthen_toolbox_select_as(const struct thinkthen_toolbox *toolbox,
                                    const uint8_t *name,
                                    size_t len,
                                    uint32_t function,
                                    struct thinkthen_question **out,
                                    struct thinkthen_error *error);

/**
 * Admit flat typed definitions from a native host constructor.
 * # Safety
 * Counted UTF-8 and distinct empty output slots remain live until return.
 */
THINKTHEN_API
int32_t thinkthen_toolbox_from_definitions_json(const uint8_t *data,
                                                size_t len,
                                                struct thinkthen_toolbox **out,
                                                struct thinkthen_error *error);

/**
 * Save the admitted named questions in the command's entries format.
 * # Safety
 * The handle is borrowed; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_toolbox_to_json(const struct thinkthen_toolbox *toolbox,
                                  struct thinkthen_buffer *out,
                                  struct thinkthen_error *error);

/**
 * Export admitted flat definitions for host hydration, without host validation.
 * # Safety
 * The handle is borrowed; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_toolbox_definitions_json(const struct thinkthen_toolbox *toolbox,
                                           struct thinkthen_buffer *out,
                                           struct thinkthen_error *error);

/**
 * Export one admitted question's canonical definition for host hydration.
 * # Safety
 * The handle is borrowed; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_question_to_json(const struct thinkthen_question *question,
                                   struct thinkthen_buffer *out,
                                   struct thinkthen_error *error);

/**
 * Return the question's function discriminator without completing its builder.
 * # Safety
 * The question is live; out and error are distinct exclusive writable slots.
 */
THINKTHEN_API
int32_t thinkthen_question_function(const struct thinkthen_question *question,
                                    uint32_t *out,
                                    struct thinkthen_error *error);

/**
 * Retain the call's wake source without changing its input reservation.
 *
 * Host sequence: retain once; read events_version BEFORE call_next; on
 * NEEDS_INPUT select caller input against events_wait(version). If input wins,
 * call events_wake and join the waiter before sending through the sole driver.
 * If an event wins, recheck call_next without issuing another input demand.
 * Output may become ready while the reserved input remains outstanding.
 *
 * Cancellation wakes this source through the call's queues. A host also wakes
 * and joins its waiters on close; it frees the event handle only after all
 * version/wait/wake operations return. The event handle can outlive call_free.
 * # Safety
 * The sole driver owns a live call during this retain operation.
 */
THINKTHEN_API struct thinkthen_events *thinkthen_call_events(const struct thinkthen_call *call);

/**
 * Read the version BEFORE the sole driver checks call readiness.
 * # Safety
 * Events stays live until this call returns. Concurrent event operations are safe.
 */
THINKTHEN_API uint64_t thinkthen_events_version(const struct thinkthen_events *events);

/**
 * Wait for a change from version, returning immediately if it already changed.
 * A wake requests a recheck; it does not promise a row or consume input capacity.
 * # Safety
 * Events stays live until this wait returns. Concurrent waits and wakes are safe.
 */
THINKTHEN_API void thinkthen_events_wait(const struct thinkthen_events *events, uint64_t version);

/**
 * Wake waits on earlier versions, including waits not yet entered.
 * This does not cancel the call or alter its reserved input.
 * # Safety
 * Events stays live until this call returns. Concurrent event operations are safe.
 */
THINKTHEN_API void thinkthen_events_wake(const struct thinkthen_events *events);

/**
 * Free an event source after all its operations have returned.
 * # Safety
 * Events is uniquely owned and no version, wait or wake operation remains active.
 */
THINKTHEN_API void thinkthen_events_free(struct thinkthen_events *events);

/**
 * Return the shared call-level error for host settings conversion failure.
 * # Safety
 * The error is an exclusive, empty writable slot with a supported struct_size.
 */
THINKTHEN_API int32_t thinkthen_settings_conversion_error(struct thinkthen_error *error);

/**
 * Return the shared call-level error for host question conversion failure.
 * # Safety
 * The error is an exclusive, empty writable slot with a supported struct_size.
 */
THINKTHEN_API int32_t thinkthen_question_conversion_error(struct thinkthen_error *error);

/**
 * Return the shared cancellation error after a host interrupts a completed pull.
 * # Safety
 * The error is an exclusive, empty writable slot with a supported struct_size.
 */
THINKTHEN_API int32_t thinkthen_cancelled_error(struct thinkthen_error *error);

/**
 * Native ABI major/minor, independent of package version.
 */
THINKTHEN_API uint32_t thinkthen_abi_version(void);

/**
 * Process-lifetime NUL-terminated package version; never free this pointer.
 */
THINKTHEN_API const char *thinkthen_version(void);

/**
 * Process-lifetime dotted code name, or NULL for an unknown integer.
 */
THINKTHEN_API const char *thinkthen_code_name(int32_t value);

/**
 * Process-lifetime NUL-terminated family, or NULL for an unknown code, including zero.
 * The family comes from the engine registry. Never free the returned pointer.
 */
THINKTHEN_API const char *thinkthen_code_family(int32_t value);

/**
 * Open a client from a copied typed settings snapshot.
 * # Safety
 * Handles are live; out/error are distinct, empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_client_open(const struct thinkthen_settings *settings,
                              struct thinkthen_client **out,
                              struct thinkthen_error *error);

/**
 * Admit canonical client JSON for a JSON-native host before resolving keys.
 * # Safety
 * Counted bytes and distinct, empty outputs remain live until return.
 */
THINKTHEN_API
int32_t thinkthen_client_open_json(const uint8_t *settings,
                                   size_t len,
                                   struct thinkthen_client **out,
                                   struct thinkthen_error *error);

/**
 * Free this client; its calls retain the engine independently.
 * # Safety
 * NULL or one live owner after all active operations finish.
 */
THINKTHEN_API void thinkthen_client_free(struct thinkthen_client *client);

/**
 * Open a copied typed question with its lists, options and optional execution name.
 * NULL name is absent; a present empty name fails the shared name validation.
 * Toolbox selection never supplies this execution name.
 * # Safety
 * Handles, optional name view and distinct empty outputs remain live until return.
 */
THINKTHEN_API
int32_t thinkthen_call_open_question(const struct thinkthen_client *client,
                                     const struct thinkthen_question *question,
                                     const struct thinkthen_text *name,
                                     uint32_t offsets,
                                     struct thinkthen_call **out,
                                     struct thinkthen_error *error);

/**
 * Admit a canonical request for a JSON-native host; never load a question file.
 * # Safety
 * Client and counted bytes are live; outputs are distinct empty slots.
 */
THINKTHEN_API
int32_t thinkthen_call_open_json(const struct thinkthen_client *client,
                                 const uint8_t *request,
                                 size_t len,
                                 uint32_t offsets,
                                 struct thinkthen_call **out,
                                 struct thinkthen_error *error);

/**
 * Send copied text after NEEDS_INPUT; invalid UTF-8 becomes one record.invalid row.
 * # Safety
 * One driver owns a live call and borrows counted readable bytes and an empty error.
 */
THINKTHEN_API
int32_t thinkthen_call_send_text(struct thinkthen_call *call,
                                 const uint8_t *data,
                                 size_t len,
                                 struct thinkthen_error *error);

/**
 * Send copied structured JSON; admission failures become failed records.
 * # Safety
 * One driver owns the call; counted input and error slots remain live.
 */
THINKTHEN_API
int32_t thinkthen_call_send_json(struct thinkthen_call *call,
                                 const uint8_t *data,
                                 size_t len,
                                 struct thinkthen_error *error);

/**
 * Send native-endian UTF-16; unpaired surrogates become one failed record.
 * # Safety
 * One driver owns the call; data lends len aligned units or NULL at zero.
 */
THINKTHEN_API
int32_t thinkthen_call_send_utf16(struct thinkthen_call *call,
                                  const uint16_t *data,
                                  size_t len,
                                  struct thinkthen_error *error);

/**
 * Send an explicit host missing value without a backend call.
 * # Safety
 * One driver owns the live call and empty exclusive error slot.
 */
THINKTHEN_API
int32_t thinkthen_call_send_missing(struct thinkthen_call *call,
                                    struct thinkthen_error *error);

/**
 * Send an explicit host conversion failure without fabricating a JSON id.
 * # Safety
 * One driver owns the live call and empty exclusive error slot.
 */
THINKTHEN_API
int32_t thinkthen_call_send_invalid(struct thinkthen_call *call,
                                    struct thinkthen_error *error);

/**
 * Close input without waiting for output capacity or backend work.
 * # Safety
 * A sole driver owns a live call; free cannot run concurrently.
 */
THINKTHEN_API void thinkthen_call_end_input(struct thinkthen_call *call);

/**
 * Pull one row, reserved input slot or completion. Failed records return OK.
 * # Safety
 * A sole driver owns the call; row/error are sized, distinct, empty output slots.
 */
THINKTHEN_API
int32_t thinkthen_call_next(struct thinkthen_call *call,
                            struct thinkthen_row *out,
                            struct thinkthen_error *error);

/**
 * Request cancellation concurrently without touching driver state.
 * # Safety
 * The handle remains live until every concurrent operation returns.
 */
THINKTHEN_API void thinkthen_call_cancel(struct thinkthen_call *call);

/**
 * Cancel and release a call without joining a backend worker.
 * # Safety
 * NULL or the unique owner after all active operations finish.
 */
THINKTHEN_API void thinkthen_call_free(struct thinkthen_call *call);

/**
 * Free one owned counted buffer and clear it; empty/NULL is harmless.
 * # Safety
 * The slot uniquely owns an engine allocation and no reader remains.
 */
THINKTHEN_API void thinkthen_buffer_free(struct thinkthen_buffer *buffer);

/**
 * Free a supported error prefix while preserving caller size and trailing bytes.
 * # Safety
 * NULL or a sized unique error after readers finish; unsupported prefixes are untouched.
 */
THINKTHEN_API void thinkthen_error_free(struct thinkthen_error *error);

/**
 * Free a supported row prefix while preserving caller size and trailing bytes.
 * # Safety
 * NULL or a sized unique row after readers finish; unsupported prefixes are untouched.
 */
THINKTHEN_API void thinkthen_row_free(struct thinkthen_row *row);

/**
 * Admit a question and copy all finite descriptors before starting the shared worker.
 * Initial records in request JSON are rejected as competing collections.
 * # Safety
 * Client and counted request bytes are live; records points to count readable slots
 * of record_stride bytes each, with readable byte views for text and JSON kinds.
 * Each slot has a readable size field; out/error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_call_open_records_json(const struct thinkthen_client *client,
                                         const uint8_t *request,
                                         size_t len,
                                         const struct thinkthen_record_input *records,
                                         size_t count,
                                         size_t record_stride,
                                         uint32_t offsets,
                                         struct thinkthen_call **out,
                                         struct thinkthen_error *error);

/**
 * Open a typed named question over a finite collection copied before return.
 * The exact count preserves finite batching and model-bound admission.
 * # Safety
 * Client, question, optional name and counted descriptors are live until return.
 * Each descriptor has record_stride readable bytes and a supported size prefix.
 * Out and error are distinct empty writable slots; one thread drives the call.
 */
THINKTHEN_API
int32_t thinkthen_call_open_records_question(const struct thinkthen_client *client,
                                             const struct thinkthen_question *question,
                                             const struct thinkthen_text *name,
                                             const struct thinkthen_record_input *records,
                                             size_t count,
                                             size_t record_stride,
                                             uint32_t offsets,
                                             struct thinkthen_call **out,
                                             struct thinkthen_error *error);

/**
 * Borrow the original filter input after call/client destruction, until row cleanup.
 * # Safety
 * Row is an engine-produced supported prefix with a live source allocation;
 * out/error are distinct empty writable slots. Immutable readers may run concurrently.
 */
THINKTHEN_API
int32_t thinkthen_row_original(const struct thinkthen_row *row,
                               struct thinkthen_record_view *out,
                               struct thinkthen_error *error);

/**
 * Evaluate one record; a failed record is an OK row, never an operation error.
 * # Safety
 * Handles and counted input are live; sized row/error outputs are distinct and empty.
 */
THINKTHEN_API
int32_t thinkthen_run_one(const struct thinkthen_client *client,
                          const struct thinkthen_question *question,
                          const uint8_t *data,
                          size_t len,
                          uint32_t kind,
                          struct thinkthen_row *out,
                          struct thinkthen_error *error);

/**
 * Borrow an answered choose label until value cleanup.
 * # Safety
 * Value is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_choice(const struct thinkthen_value *value,
                               struct thinkthen_text *out,
                               struct thinkthen_error *error);

/**
 * Read an answered decide or filter boolean as zero or one.
 * # Safety
 * Value is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_bool(const struct thinkthen_value *value,
                             uint8_t *out,
                             struct thinkthen_error *error);

/**
 * Read an answered score without an aggregate probability.
 * # Safety
 * Value is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_number(const struct thinkthen_value *value,
                               double *out,
                               struct thinkthen_error *error);

/**
 * Read an answered rank, independent of its original input position.
 * # Safety
 * Value is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_rank(const struct thinkthen_value *value,
                             uint64_t *out,
                             struct thinkthen_error *error);

/**
 * Count the answered tag labels, including a legitimate empty answer.
 * # Safety
 * Value is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_tag_count(const struct thinkthen_value *value,
                                  size_t *out,
                                  struct thinkthen_error *error);

/**
 * Borrow one answered tag label until value cleanup.
 * # Safety
 * Value is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_tag_at(const struct thinkthen_value *value,
                               size_t index,
                               struct thinkthen_text *out,
                               struct thinkthen_error *error);

/**
 * Borrow a find answer; present zero means answered null, distinct from unsure.
 * # Safety
 * Value is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_found(const struct thinkthen_value *value,
                              struct thinkthen_span *out,
                              struct thinkthen_error *error);

/**
 * Count recognized names from a recognize answer or relate detail.
 * # Safety
 * Value is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_name_count(const struct thinkthen_value *value,
                                   size_t *out,
                                   struct thinkthen_error *error);

/**
 * Borrow one recognized name and its typed support probability.
 * # Safety
 * Value is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_name_at(const struct thinkthen_value *value,
                                size_t index,
                                struct thinkthen_name *out,
                                struct thinkthen_error *error);

/**
 * Count answered relations.
 * # Safety
 * Value is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_relation_count(const struct thinkthen_value *value,
                                       size_t *out,
                                       struct thinkthen_error *error);

/**
 * Borrow one answered relation and both typed endpoints.
 * # Safety
 * Value is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_relation_at(const struct thinkthen_value *value,
                                    size_t index,
                                    struct thinkthen_relation *out,
                                    struct thinkthen_error *error);

/**
 * Count named annotation outcomes, including failed children.
 * # Safety
 * Value is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_annotation_count(const struct thinkthen_value *value,
                                         size_t *out,
                                         struct thinkthen_error *error);

/**
 * Borrow an entry name and copy its typed outcome into an independent owner.
 * # Safety
 * Value is live; name/out/error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_annotation_at(const struct thinkthen_value *value,
                                      size_t index,
                                      struct thinkthen_text *name,
                                      struct thinkthen_value **out,
                                      struct thinkthen_error *error);

/**
 * Count optional choose/tag label probabilities.
 * # Safety
 * Value is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_label_count(const struct thinkthen_value *value,
                                    size_t *out,
                                    struct thinkthen_error *error);

/**
 * Borrow one choose/tag label probability, including unselected labels.
 * # Safety
 * Value is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_label_at(const struct thinkthen_value *value,
                                 size_t index,
                                 struct thinkthen_probability *out,
                                 struct thinkthen_error *error);

/**
 * Count optional score/rank level probabilities in question order.
 * # Safety
 * Value is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_level_count(const struct thinkthen_value *value,
                                    size_t *out,
                                    struct thinkthen_error *error);

/**
 * Read one score/rank level probability in question order.
 * # Safety
 * Value is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_level_at(const struct thinkthen_value *value,
                                 size_t index,
                                 double *out,
                                 struct thinkthen_error *error);

/**
 * Count recognition's original pieces, or zero when detail is absent.
 * # Safety
 * Value is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_piece_count(const struct thinkthen_value *value,
                                    size_t *out,
                                    struct thinkthen_error *error);

/**
 * Borrow an original piece and its translated span.
 * # Safety
 * Value is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_piece_at(const struct thinkthen_value *value,
                                 size_t index,
                                 struct thinkthen_span *out,
                                 struct thinkthen_error *error);

/**
 * Count one piece's typed boundary probabilities.
 * # Safety
 * Value is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_piece_tag_count(const struct thinkthen_value *value,
                                        size_t index,
                                        size_t *out,
                                        struct thinkthen_error *error);

/**
 * Borrow one original piece's boundary label and probability.
 * # Safety
 * Value is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_piece_tag_at(const struct thinkthen_value *value,
                                     size_t index,
                                     size_t tag,
                                     struct thinkthen_probability *out,
                                     struct thinkthen_error *error);

/**
 * Count proposed-name detail entries for recognition.
 * # Safety
 * Value is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_kind_count(const struct thinkthen_value *value,
                                   size_t *out,
                                   struct thinkthen_error *error);

/**
 * Read one proposed name's translated span; this detail has no text field.
 * # Safety
 * Value is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_kind_span(const struct thinkthen_value *value,
                                  size_t index,
                                  struct thinkthen_span *out,
                                  struct thinkthen_error *error);

/**
 * Count one proposed name's caller-domain kind probabilities.
 * # Safety
 * Value is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_kind_probability_count(const struct thinkthen_value *value,
                                               size_t index,
                                               size_t *out,
                                               struct thinkthen_error *error);

/**
 * Borrow one proposed name's kind label and probability.
 * # Safety
 * Value is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_kind_probability_at(const struct thinkthen_value *value,
                                            size_t index,
                                            size_t choice,
                                            struct thinkthen_probability *out,
                                            struct thinkthen_error *error);

/**
 * Count evaluated relation candidate pairs, including pairs outside the answer.
 * # Safety
 * Value is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_pair_count(const struct thinkthen_value *value,
                                   size_t *out,
                                   struct thinkthen_error *error);

/**
 * Borrow one evaluated candidate pair using the same typed relation shape.
 * # Safety
 * Value is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_pair_at(const struct thinkthen_value *value,
                                size_t index,
                                struct thinkthen_relation *out,
                                struct thinkthen_error *error);

/**
 * Compare required and engine ABI versions using the shared acceptance rule.
 */
THINKTHEN_API uint8_t thinkthen_abi_compatible(uint32_t required, uint32_t actual);

/**
 * Number of codes in the authoritative engine registry.
 */
THINKTHEN_API size_t thinkthen_code_count(void);

/**
 * Registry code at index, or zero outside the registry. Zero is not an error code.
 */
THINKTHEN_API int32_t thinkthen_code_at(size_t index);

/**
 * Copy typed row values into an independent owner; every view borrows that owner.
 * # Safety
 * Row is a live sized prefix; out/error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_row_values(const struct thinkthen_row *row,
                             struct thinkthen_value **out,
                             struct thinkthen_error *error);

/**
 * Release a typed snapshot after all of its borrowed field views are finished.
 * # Safety
 * NULL or the unique live value owner with no active readers.
 */
THINKTHEN_API void thinkthen_value_free(struct thinkthen_value *value);

/**
 * Read a typed function discriminator without interpreting JSON.
 * # Safety
 * Value is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_function(const struct thinkthen_value *value,
                                 uint32_t *out,
                                 struct thinkthen_error *error);

/**
 * Read a typed status discriminator.
 * # Safety
 * Value is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_status(const struct thinkthen_value *value,
                               uint32_t *out,
                               struct thinkthen_error *error);

/**
 * Borrow source fields until value cleanup. Annotation entries have no source.
 * # Safety
 * Value is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_source(const struct thinkthen_value *value,
                               struct thinkthen_source *out,
                               struct thinkthen_error *error);

/**
 * Borrow an exact identifier; integer variants never use a double.
 * # Safety
 * Value is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_id(const struct thinkthen_value *value,
                           struct thinkthen_id *out,
                           struct thinkthen_error *error);

/**
 * Copy a failed-record error into a sized owner; an ordinary result gives no code.
 * # Safety
 * Value is live; result/error are sized, distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_error(const struct thinkthen_value *value,
                              struct thinkthen_error *result,
                              struct thinkthen_error *error);

/**
 * Read an aggregate probability; NaN means the function has no defined aggregate.
 * # Safety
 * Value is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_probability(const struct thinkthen_value *value,
                                    double *out,
                                    struct thinkthen_error *error);

/**
 * Distinguish absent detail from a present empty typed detail collection.
 * # Safety
 * Value is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_has_detail(const struct thinkthen_value *value,
                                   uint8_t *out,
                                   struct thinkthen_error *error);

/**
 * Borrow the actual execution name; a null text view means unnamed execution.
 * # Safety
 * Value is live; out and error are distinct empty writable slots.
 */
THINKTHEN_API
int32_t thinkthen_value_source_name(const struct thinkthen_value *value,
                                    struct thinkthen_text *out,
                                    struct thinkthen_error *error);

/**
 * Borrow original filter bytes owned by this independent typed snapshot.
 * # Safety
 * Value is live; out/error are distinct empty writable slots; readers may run concurrently.
 */
THINKTHEN_API
int32_t thinkthen_value_original(const struct thinkthen_value *value,
                                 struct thinkthen_record_view *out,
                                 struct thinkthen_error *error);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* THINKTHEN_H */
