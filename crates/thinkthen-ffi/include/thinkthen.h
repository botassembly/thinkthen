/* Generated from Rust exports and contract registries. Do not edit. */
/*
 * ThinkThen C interface.
 *
 * Return values. A function that returns a status returns THINKTHEN_OK (0) on
 * success, or the negative of a thinkthen_code on failure, for example
 * -THINKTHEN_CODE_USAGE_QUESTION. Some also return THINKTHEN_UNSURE when the model is
 * not sure, or THINKTHEN_END and THINKTHEN_NEEDS_INPUT while reading a call; each
 * function lists the values it returns. Write rc != THINKTHEN_OK, not rc == -1.
 *
 * Errors. Every status-returning function takes a last thinkthen_error *error, which may be
 * NULL. When it is not NULL, initialize it with THINKTHEN_ERROR_INIT: a failure
 * fills error->code and error->message, and success leaves it empty. Free it with
 * thinkthen_error_free. Passing it again frees what it held, so reusing one error
 * variable never leaks. An error whose struct_size is not set returns
 * -THINKTHEN_CODE_USAGE_ABI and is left untouched.
 *
 * Text. Text is UTF-8, passed as a pointer and a byte length. Pass
 * THINKTHEN_NUL_TERMINATED as the length for a NUL-terminated string. The library
 * copies input before returning, except that thinkthen_find returns a span borrowing
 * the caller's text. Keep that text alive while using the span. An argument that is not UTF-8 is refused
 * with -THINKTHEN_CODE_USAGE_ABI; a record that is not UTF-8 fails its own row as
 * record.invalid. Text it returns (thinkthen_text) is not
 * NUL-terminated; print it with printf("%.*s", (int)text.len, text.data).
 *
 * Ownership. Each object has one free function: thinkthen_settings_free,
 * thinkthen_client_free, thinkthen_question_free, thinkthen_toolbox_free,
 * thinkthen_call_free, thinkthen_value_free, thinkthen_row_free,
 * thinkthen_error_free and thinkthen_buffer_free. Each accepts NULL. Initialize a row
 * with THINKTHEN_ROW_INIT; passing it again frees what it held. A thinkthen_buffer
 * output is overwritten, not freed, so free it before passing it again. A handle
 * output such as thinkthen_value ** is overwritten too. A view such as
 * thinkthen_text, thinkthen_span or thinkthen_record_view borrows from the value or
 * row it came from and lasts until that is freed; thinkthen_find spans instead borrow
 * the caller's text.
 *
 * Threads. A settings, question or toolbox object is used by one thread at a time
 * while it changes; once built, a question may be read by many threads. A client
 * may be shared by every thread. One thread drives a call at a time, and
 * thinkthen_call_cancel may run on any thread. Each function's Pointers and threads
 * note says what it needs.
 */
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
/** Marks a function whose returned status must be checked. */
#ifndef THINKTHEN_NODISCARD
#if defined(__GNUC__) || defined(__clang__)
#define THINKTHEN_NODISCARD __attribute__((warn_unused_result))
#elif defined(__cplusplus) && __cplusplus >= 201703L
#define THINKTHEN_NODISCARD [[nodiscard]]
#elif defined(_MSC_VER)
#include <sal.h>
#define THINKTHEN_NODISCARD _Check_return_
#else
#define THINKTHEN_NODISCARD
#endif
#endif
/** The library version this header describes, such as "0.2.0"; thinkthen_version() gives the loaded library's. */
#define THINKTHEN_VERSION "0.2.0"
/** The major part of THINKTHEN_VERSION. */
#define THINKTHEN_VERSION_MAJOR 0u
/** The minor part of THINKTHEN_VERSION. */
#define THINKTHEN_VERSION_MINOR 2u
/** The patch part of THINKTHEN_VERSION. */
#define THINKTHEN_VERSION_PATCH 0u
/** The binary interface this header needs: THINKTHEN_ABI_MAJOR in the high 16 bits, THINKTHEN_ABI_MINOR in the low 16. Pass it to thinkthen_abi_compatible with thinkthen_abi_version(). */
#define THINKTHEN_ABI_VERSION 131077u
/** The binary interface's major version; a library with another major is incompatible. */
#define THINKTHEN_ABI_MAJOR 2u
/** The binary interface's minor version; a library with this minor or a later one is compatible. */
#define THINKTHEN_ABI_MINOR 5u
/** The code name a binding reports when the loaded library's binary interface is incompatible. */
#define THINKTHEN_ABI_ERROR_CODE "usage.abi"
/** The message a binding reports when the loaded library's binary interface is incompatible. */
#define THINKTHEN_ABI_ERROR_MESSAGE "the native ABI is incompatible"
/** The environment variable that names the one native library a loading binding opens. */
#define THINKTHEN_LIBRARY_VARIABLE "THINKTHEN_LIBRARY"
/** The message a loading binding reports when that variable is not an absolute path. */
#define THINKTHEN_LIBRARY_PATH_MESSAGE "THINKTHEN_LIBRARY must be an absolute path"
/** Pass as a text length to read a NUL-terminated string instead of a counted one. */
#define THINKTHEN_NUL_TERMINATED SIZE_MAX
/**
 * Every error code, shared by every ThinkThen library. A failing function returns
 * the negative of one, such as -THINKTHEN_CODE_RECORD_EMPTY. thinkthen_code_name gives
 * its dotted name, such as "record.empty", and thinkthen_code_family its family.
 */
typedef enum thinkthen_code {
  /** usage.threshold: A cut or band is out of range or not allowed for this function. */
  THINKTHEN_CODE_USAGE_THRESHOLD = 1001,
  /** usage.question: The question is not valid: blank or missing a part it needs. */
  THINKTHEN_CODE_USAGE_QUESTION = 1002,
  /** usage.name: A name is blank, repeated, or not the name of an existing question. */
  THINKTHEN_CODE_USAGE_NAME = 1003,
  /** usage.settings: The settings are not valid. */
  THINKTHEN_CODE_USAGE_SETTINGS = 1004,
  /** usage.backend: The backend is unknown, or not allowed here. */
  THINKTHEN_CODE_USAGE_BACKEND = 1005,
  /** usage.url: The URL is not valid. */
  THINKTHEN_CODE_USAGE_URL = 1006,
  /** usage.key-missing: The backend needs an API key and none was given or found. */
  THINKTHEN_CODE_USAGE_KEY_MISSING = 1007,
  /** usage.batch: The batch size is not a positive count. */
  THINKTHEN_CODE_USAGE_BATCH = 1008,
  /** usage.pointer: A pointer, such as a field pointer, is not a valid JSON Pointer. */
  THINKTHEN_CODE_USAGE_POINTER = 1009,
  /** usage.sql-policy: The database administrator's SQL policy is not valid. */
  THINKTHEN_CODE_USAGE_SQL_POLICY = 1010,
  /** usage.sql-selection: The database administrator does not allow this backend and model. */
  THINKTHEN_CODE_USAGE_SQL_SELECTION = 1011,
  /** usage.abi: A struct, handle or slot is not one this library made or initialized, or the library's binary interface is incompatible. */
  THINKTHEN_CODE_USAGE_ABI = 1012,
  /** usage.closed: The client or call was used after it was closed. */
  THINKTHEN_CODE_USAGE_CLOSED = 1013,
  /** usage.function: This function does not fit the question's or value's function. */
  THINKTHEN_CODE_USAGE_FUNCTION = 1014,
  /** usage.record-kind: The record kind is unknown. */
  THINKTHEN_CODE_USAGE_RECORD_KIND = 1015,
  /** usage.offsets: The offset unit is unknown. */
  THINKTHEN_CODE_USAGE_OFFSETS = 1016,
  /** usage.argument: An argument is NULL, out of range, or not allowed here. */
  THINKTHEN_CODE_USAGE_ARGUMENT = 1017,
  /** usage.option: An option is not valid for this function. */
  THINKTHEN_CODE_USAGE_OPTION = 1018,
  /** usage.column: A table column, answer column or field pointer is not valid. */
  THINKTHEN_CODE_USAGE_COLUMN = 1019,
  /** usage.sheet: The table sheet selection is not valid. */
  THINKTHEN_CODE_USAGE_SHEET = 1020,
  /** usage.output: The table answers cannot go to the output asked for. */
  THINKTHEN_CODE_USAGE_OUTPUT = 1021,
  /** usage.table: The table input file cannot be used as it is. */
  THINKTHEN_CODE_USAGE_TABLE = 1022,
  /** record.json: The record is not valid JSON. A record error: that record's row fails and the call goes on. */
  THINKTHEN_CODE_RECORD_JSON = 2001,
  /** record.missing: The record is a missing value. A record error: that record's row fails and the call goes on. */
  THINKTHEN_CODE_RECORD_MISSING = 2002,
  /** record.too-large: The record is larger than the backend accepts. A record error: that record's row fails and the call goes on. */
  THINKTHEN_CODE_RECORD_TOO_LARGE = 2003,
  /** record.invalid: The record could not be read, for example text that is not UTF-8. A record error: that record's row fails and the call goes on. */
  THINKTHEN_CODE_RECORD_INVALID = 2004,
  /** record.empty: The record's text is blank, so it is never sent. A record error: that record's row fails and the call goes on. */
  THINKTHEN_CODE_RECORD_EMPTY = 2005,
  /** backend.transport: The backend could not be reached. A record error: that record's row fails and the call goes on. */
  THINKTHEN_CODE_BACKEND_TRANSPORT = 3001,
  /** backend.status: The backend refused the request. A record error: that record's row fails and the call goes on. */
  THINKTHEN_CODE_BACKEND_STATUS = 3002,
  /** backend.reply: The backend's reply could not be read. A record error: that record's row fails and the call goes on. */
  THINKTHEN_CODE_BACKEND_REPLY = 3003,
  /** local.cancelled: The call was cancelled. */
  THINKTHEN_CODE_LOCAL_CANCELLED = 4001,
  /** local.io: A local file could not be read or written. */
  THINKTHEN_CODE_LOCAL_IO = 4002,
  /** local.toolbox: A toolbox could not be read. */
  THINKTHEN_CODE_LOCAL_TOOLBOX = 4003,
  /** local.accounting: Client accounting exceeded its supported range before another request could be sent. */
  THINKTHEN_CODE_LOCAL_ACCOUNTING = 4004,
  /** local.request-limit: The client request allowance is exhausted; no further request was sent. */
  THINKTHEN_CODE_LOCAL_REQUEST_LIMIT = 4005,
  /** local.response-store:  */
  THINKTHEN_CODE_LOCAL_RESPONSE_STORE = 4006,
  /** local.store-corrupt:  */
  THINKTHEN_CODE_LOCAL_STORE_CORRUPT = 4007,
  /** local.store-format:  */
  THINKTHEN_CODE_LOCAL_STORE_FORMAT = 4008,
  /** local.store-answer:  */
  THINKTHEN_CODE_LOCAL_STORE_ANSWER = 4009,
  /** local.replay-miss:  */
  THINKTHEN_CODE_LOCAL_REPLAY_MISS = 4010,
  /** local.store-quota:  */
  THINKTHEN_CODE_LOCAL_STORE_QUOTA = 4011,
  /** internal.bug: A bug in this library; please report it. */
  THINKTHEN_CODE_INTERNAL_BUG = 5001,
} thinkthen_code;
/** The ten functions a question can ask. */
typedef enum thinkthen_function {
  /** decide: Answer yes or no. */
  THINKTHEN_FUNCTION_DECIDE = 1,
  /** choose: Pick one of the choices. */
  THINKTHEN_FUNCTION_CHOOSE = 2,
  /** tag: Pick every label that applies. */
  THINKTHEN_FUNCTION_TAG = 3,
  /** score: Place the text on ordered levels and give a number. */
  THINKTHEN_FUNCTION_SCORE = 4,
  /** filter: Keep the records the answer is yes for. */
  THINKTHEN_FUNCTION_FILTER = 5,
  /** find: Find the part of the text that answers the question. */
  THINKTHEN_FUNCTION_FIND = 7,
  /** recognize: Find names of the given kinds. */
  THINKTHEN_FUNCTION_RECOGNIZE = 8,
  /** relate: Find relations between recognized names. */
  THINKTHEN_FUNCTION_RELATE = 9,
  /** rank: Order the records by score. */
  THINKTHEN_FUNCTION_RANK = 6,
  /** annotate: Ask several named questions at once. */
  THINKTHEN_FUNCTION_ANNOTATE = 10,
} thinkthen_function;
/** The backends a client can use. The stub answers without a network or a key. */
typedef enum thinkthen_backend {
  /** systemone: The System One service; needs an API key. */
  THINKTHEN_BACKEND_SYSTEMONE = 1,
  /** openai: An OpenAI-compatible service; needs an API key. */
  THINKTHEN_BACKEND_OPENAI = 2,
  /** stub: A local stand-in that answers without a network or a key, for tests. */
  THINKTHEN_BACKEND_STUB = 3,
} thinkthen_backend;
/** A row's status. Read its answer only when it is THINKTHEN_STATUS_ANSWERED. */
typedef enum thinkthen_status {
  /** answered: The row has an answer. */
  THINKTHEN_STATUS_ANSWERED = 1,
  /** unsure: The model was not sure, so the row has no answer. */
  THINKTHEN_STATUS_UNSURE = 2,
  /** failed: The record failed; the row's error says why. */
  THINKTHEN_STATUS_FAILED = 3,
} thinkthen_status;
/** How a record's bytes are read: as text, as JSON, or as a missing or unreadable value. */
typedef enum thinkthen_record_kind {
  /** text: The bytes are UTF-8 text. */
  THINKTHEN_RECORD_TEXT = 0,
  /** json: The bytes are one JSON value. */
  THINKTHEN_RECORD_JSON = 1,
  /** missing: A missing value, such as SQL NULL; it fails as record.missing. */
  THINKTHEN_RECORD_MISSING = 2,
  /** invalid: A value the caller could not convert; it fails as record.invalid. */
  THINKTHEN_RECORD_INVALID = 3,
} thinkthen_record_kind;
/** The unit of span offsets. C uses THINKTHEN_OFFSET_UTF8, which counts bytes. */
typedef enum thinkthen_offset {
  /** utf8: Byte offsets into UTF-8 text. */
  THINKTHEN_OFFSET_UTF8 = 0,
  /** utf16: UTF-16 code unit offsets. */
  THINKTHEN_OFFSET_UTF16 = 1,
  /** code_points: Code point offsets, end exclusive. */
  THINKTHEN_OFFSET_CODE_POINTS = 2,
  /** code_points_inclusive: Code point offsets, end inclusive. */
  THINKTHEN_OFFSET_CODE_POINTS_INCLUSIVE = 3,
} thinkthen_offset;
/** The thinkthen_id kind for absent. */
#define THINKTHEN_ID_ABSENT 0u
/** The thinkthen_id kind for string. */
#define THINKTHEN_ID_STRING 1u
/** The thinkthen_id kind for signed. */
#define THINKTHEN_ID_SIGNED 2u
/** The thinkthen_id kind for unsigned. */
#define THINKTHEN_ID_UNSIGNED 3u
#endif /* THINKTHEN_REGISTRY_H */


#ifndef THINKTHEN_H
#define THINKTHEN_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Success: the function did what it says, or wrote the next row.
 */
#define THINKTHEN_OK 0

/**
 * A call has no more rows; failed records do not change this.
 */
#define THINKTHEN_END 1

/**
 * A streaming call waits for the next record or thinkthen_call_end_input.
 */
#define THINKTHEN_NEEDS_INPUT 2

/**
 * Not sure; no answer was written. This is not an engine error code.
 */
#define THINKTHEN_UNSURE 4

/**
 * Kept for compatibility. Failures return the negative of a thinkthen_code, never this value.
 */
#define THINKTHEN_ERROR -1

/**
 * Reuse eligible answers; save valid misses within configured bounds.
 */
#define THINKTHEN_RESPONSE_CACHE 0

/**
 * Append immutable live exchanges without evicting recordings.
 */
#define THINKTHEN_RESPONSE_RECORD 1

/**
 * Read a fixed recording snapshot without credentials or network.
 */
#define THINKTHEN_RESPONSE_REPLAY 2

/**
 * A call's wake source, from thinkthen_call_events; it may outlive the call. Free it with
 * thinkthen_events_free.
 */
typedef struct thinkthen_events thinkthen_events;

/**
 * An open call. One thread drives it at a time; thinkthen_call_cancel may run on any thread.
 * Free it with thinkthen_call_free.
 */
typedef struct thinkthen_call thinkthen_call;

/**
 * An open client. Any thread may use it, and calls opened from it outlive it. Free it with
 * thinkthen_client_free.
 */
typedef struct thinkthen_client thinkthen_client;

/**
 * A question. One thread at a time changes it; once complete, many threads may use it.
 * Free it with thinkthen_question_free.
 */
typedef struct thinkthen_question thinkthen_question;

/**
 * Settings for a client, copied when it opens. Free them with thinkthen_settings_free.
 */
typedef struct thinkthen_settings thinkthen_settings;

/**
 * A set of named questions, each copied in. Free it with thinkthen_toolbox_free.
 */
typedef struct thinkthen_toolbox thinkthen_toolbox;

/**
 * A copy of a row's answer, read with the thinkthen_value_* functions. Views it writes
 * borrow from it. Free it with thinkthen_value_free.
 */
typedef struct thinkthen_value thinkthen_value;

/**
 * Bytes the library allocated for the caller. Free them with thinkthen_buffer_free, or with
 * the free function of the error or row that holds the buffer.
 */
typedef struct thinkthen_buffer {
  /**
   * UTF-8 bytes, not NUL-terminated; NULL when empty.
   */
  uint8_t *data;
  /**
   * The byte length.
   */
  size_t len;
} thinkthen_buffer;

/**
 * Why a function failed. Initialize it with THINKTHEN_ERROR_INIT and free it with
 * thinkthen_error_free; passing it again frees what it held.
 */
typedef struct thinkthen_error {
  /**
   * The caller's allocation size, preserved by cleanup.
   */
  uint32_t struct_size;
  /**
   * The thinkthen_code, or 0 when there is no error.
   */
  int32_t code;
  /**
   * The backend's HTTP status, or 0 when there is none.
   */
  uint16_t http_status;
  /**
   * The message, owned by the error; it never holds a key.
   */
  struct thinkthen_buffer message;
} thinkthen_error;

/**
 * UTF-8 text as a pointer and a byte length, not NUL-terminated. Text passed in is copied;
 * text written out borrows from the value it came from.
 */
typedef struct thinkthen_text {
  /**
   * The bytes; NULL only when len is 0.
   */
  const char *data;
  /**
   * The byte length, or THINKTHEN_NUL_TERMINATED for text passed in.
   */
  size_t len;
} thinkthen_text;

/**
 * One relation rule for thinkthen_question_add_rule or thinkthen_relate; copied before return.
 */
typedef struct thinkthen_rule {
  /**
   * The relation's name, such as "works_for".
   */
  struct thinkthen_text relation;
  /**
   * The kind of name the relation starts from.
   */
  struct thinkthen_text source;
  /**
   * The kind of name it points to.
   */
  struct thinkthen_text target;
  /**
   * How the model should read the relation; data NULL to leave it out.
   */
  struct thinkthen_text reads;
  /**
   * 1 when the relation runs both ways, 0 when it runs from source to target.
   */
  uint8_t either;
} thinkthen_rule;

/**
 * A part of a text, as thinkthen_find and the find, piece and kind accessors write it.
 * Its text borrows from the value, or from the caller's own text for thinkthen_find.
 */
typedef struct thinkthen_span {
  /**
   * 1 when a part answers the question; 0 when the answer is that none does.
   */
  uint8_t present;
  /**
   * The part's text; not NUL-terminated.
   */
  struct thinkthen_text text;
  /**
   * Where the part starts, in the call's offset unit (bytes for C).
   */
  uint64_t start;
  /**
   * Where the part ends, exclusive, in the same unit.
   */
  uint64_t end;
} thinkthen_span;

/**
 * One record's result. Initialize it with THINKTHEN_ROW_INIT and free it with
 * thinkthen_row_free; passing it again frees what it held. Read the answer fields only when
 * status is THINKTHEN_STATUS_ANSWERED, or use thinkthen_row_values for typed access.
 */
typedef struct thinkthen_row {
  /**
   * The caller's allocation size, preserved by cleanup.
   */
  uint32_t struct_size;
  /**
   * The question's function.
   */
  thinkthen_function function;
  /**
   * Whether the record was answered, not sure, or failed.
   */
  thinkthen_status status;
  /**
   * The record's input position, 1 for the first.
   */
  uint64_t position;
  /**
   * The answer's probability, or NaN when its function has none.
   */
  double probability;
  /**
   * A failed record's thinkthen_code, or 0.
   */
  int32_t error_code;
  /**
   * A decide or filter answer: 1 for yes, 0 for no. 0 also when not answered.
   */
  uint8_t answer_bool;
  /**
   * A rank answer, 1 for the first.
   */
  uint64_t rank;
  /**
   * A score answer, from 0 to 1.
   */
  double number;
  /**
   * The record's id as JSON, or empty.
   */
  struct thinkthen_buffer id;
  /**
   * The answer as JSON; empty when not sure or failed.
   */
  struct thinkthen_buffer answer;
  /**
   * The answer's detail as JSON, or empty.
   */
  struct thinkthen_buffer detail;
  /**
   * Which question, backend and model gave the row, as JSON.
   */
  struct thinkthen_buffer source;
  /**
   * A failed record's error as JSON, or empty.
   */
  struct thinkthen_buffer error;
} thinkthen_row;

/**
 * One record for thinkthen_call_open_records_*; its bytes are copied before the call opens.
 */
typedef struct thinkthen_record_input {
  /**
   * sizeof(thinkthen_record_input), at most record_stride.
   */
  uint32_t struct_size;
  /**
   * How to read data, a thinkthen_record_kind; missing and invalid records need no data.
   */
  thinkthen_record_kind kind;
  /**
   * The record's bytes for text or JSON; NULL only when len is 0.
   */
  const char *data;
  /**
   * The byte length, or THINKTHEN_NUL_TERMINATED.
   */
  size_t len;
} thinkthen_record_input;

/**
 * A filter or rank record's input, as thinkthen_row_original writes it, with its kind and bytes as
 * sent, failed records included. Missing and invalid records have no bytes. The bytes may
 * not be UTF-8, so this is not a thinkthen_text.
 */
typedef struct thinkthen_record_view {
  /**
   * How the record was sent, a thinkthen_record_kind.
   */
  thinkthen_record_kind kind;
  /**
   * The record's bytes, borrowed from the row or value.
   */
  const uint8_t *data;
  /**
   * The byte length; not NUL-terminated.
   */
  size_t len;
} thinkthen_record_view;

/**
 * Borrowed typed store configuration. Set struct_size to sizeof(thinkthen_response_store_config).
 * Optional numeric pointers are NULL for the shared defaults; pointed zero is invalid.
 */
typedef struct thinkthen_response_store_config {
  /**
   * The initialized size of this configuration.
   */
  uint32_t struct_size;
  /**
   * Private local directory, copied before return.
   */
  struct thinkthen_text path;
  /**
   * One THINKTHEN_RESPONSE_* mode.
   */
  uint32_t mode;
  /**
   * Caller-owned nonsecret namespace, copied before return.
   */
  struct thinkthen_text namespace_name;
  /**
   * Explicit nonsecret model-generation label, copied before return.
   */
  struct thinkthen_text model_generation;
  /**
   * Optional logical retained payload bound in bytes.
   */
  const uint64_t *max_bytes;
  /**
   * Optional distinct eligible ask bound.
   */
  const uint64_t *max_entries;
  /**
   * Optional live-cache age in whole seconds; recordings do not expire.
   */
  const uint64_t *max_age;
  /**
   * Optional SQLite busy timeout in whole milliseconds.
   */
  const uint64_t *busy_timeout_ms;
} thinkthen_response_store_config;

/**
 * Observed tokens; an unknown reported total denotes arithmetic overflow.
 */
typedef struct thinkthen_token_usage {
  /**
   * Known token subtotal, meaningful only when reported_known is true.
   */
  uint64_t reported;
  /**
   * Finished attempts lacking a reported count for this dimension.
   */
  uint64_t unknown_attempts;
  /**
   * Whether the subtotal fits in reported; missing attempts are counted separately.
   */
  bool reported_known;
} thinkthen_token_usage;

/**
 * Independently copied cumulative usage. Initialize struct_size before a getter.
 */
typedef struct thinkthen_usage {
  /**
   * Set to sizeof(thinkthen_usage).
   */
  uint32_t struct_size;
  /**
   * Actual send attempts.
   */
  uint64_t attempts;
  /**
   * Fully received bounded response bodies.
   */
  uint64_t responses;
  /**
   * Attempts retrying an HTTP status.
   */
  uint64_t status_retries;
  /**
   * Attempts retrying a connection.
   */
  uint64_t connection_retries;
  /**
   * Encoded request bytes including retries.
   */
  uint64_t request_body_bytes;
  /**
   * Engine calls that have not settled, including paused input.
   */
  uint64_t active_calls;
  /**
   * Transport attempts that have not finished.
   */
  uint64_t active_attempts;
  /**
   * Observed input tokens.
   */
  struct thinkthen_token_usage input_tokens;
  /**
   * Observed output tokens.
   */
  struct thinkthen_token_usage output_tokens;
} thinkthen_usage;

/**
 * Full copied usage including local hit metrics. The embedded usage layout stays unchanged.
 */
typedef struct thinkthen_usage_v2 {
  /**
   * Initialize to sizeof(thinkthen_usage_v2).
   */
  uint32_t struct_size;
  /**
   * Original counters; the getter also fills this nested struct_size.
   */
  struct thinkthen_usage usage;
  /**
   * Eligible cached asks reused, not rows.
   */
  uint64_t cache_hits;
  /**
   * Recorded asks replayed, not rows.
   */
  uint64_t replay_hits;
} thinkthen_usage_v2;

/**
 * A recognized name, as thinkthen_value_name_at writes it; its texts borrow from the value.
 */
typedef struct thinkthen_name {
  /**
   * The name as it appears in the record.
   */
  struct thinkthen_text text;
  /**
   * Where the name starts, in the call's offset unit.
   */
  uint64_t start;
  /**
   * Where the name ends, exclusive.
   */
  uint64_t end;
  /**
   * The kind of name, one of the question's kinds.
   */
  struct thinkthen_text kind;
  /**
   * How likely the name is right, from 0 to 1.
   */
  double probability;
} thinkthen_name;

/**
 * One name a relation joins; its texts borrow from the value.
 */
typedef struct thinkthen_endpoint {
  /**
   * The name as it appears in the record.
   */
  struct thinkthen_text text;
  /**
   * Where the name starts, in the call's offset unit.
   */
  uint64_t start;
  /**
   * Where the name ends, exclusive.
   */
  uint64_t end;
  /**
   * The kind of name.
   */
  struct thinkthen_text kind;
} thinkthen_endpoint;

/**
 * A relation between two names, as thinkthen_value_relation_at and thinkthen_value_pair_at
 * write it; its texts borrow from the value.
 */
typedef struct thinkthen_relation {
  /**
   * The name the relation starts from.
   */
  struct thinkthen_endpoint source;
  /**
   * The name the relation points to.
   */
  struct thinkthen_endpoint target;
  /**
   * The relation's name, from the question's rule.
   */
  struct thinkthen_text relation;
  /**
   * How likely the relation holds, from 0 to 1.
   */
  double probability;
  /**
   * 1 when the rule runs both ways, 0 otherwise.
   */
  uint8_t either;
} thinkthen_relation;

/**
 * A label and its probability, as the detail accessors write it; label borrows from the value.
 */
typedef struct thinkthen_probability {
  /**
   * The label.
   */
  struct thinkthen_text label;
  /**
   * Its probability, from 0 to 1.
   */
  double probability;
} thinkthen_probability;

/**
 * Which question, backend and model gave a row, failed rows included; texts borrow from the value.
 */
typedef struct thinkthen_source {
  /**
   * A stable identity of the question.
   */
  struct thinkthen_text question_identity;
  /**
   * The backend, a thinkthen_backend value.
   */
  thinkthen_backend backend;
  /**
   * The model the backend used.
   */
  struct thinkthen_text model;
} thinkthen_source;

/**
 * A record's id, exactly as given: integers never pass through a double.
 */
typedef struct thinkthen_id {
  /**
   * Which field holds the id: THINKTHEN_ID_ABSENT, _STRING, _SIGNED or _UNSIGNED.
   */
  uint32_t kind;
  /**
   * The id, for THINKTHEN_ID_STRING; borrows from the value.
   */
  struct thinkthen_text text;
  /**
   * The id, for THINKTHEN_ID_SIGNED.
   */
  int64_t signed_value;
  /**
   * The id, for THINKTHEN_ID_UNSIGNED.
   */
  uint64_t unsigned_value;
} thinkthen_id;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Say what a yes answer means, for a decide or filter question.
 *
 * The meaning is sent with the question so the model reads it, and setting it again replaces
 * it. Returns THINKTHEN_OK, or a negative code with the question unchanged:
 * -THINKTHEN_CODE_USAGE_QUESTION when the question is not decide or filter, or
 * -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8.
 * Pointers and threads:
 * One thread at a time may call this on a question; other questions may be used on other
 * threads. question is a live question. data points to len readable bytes, or to a NUL-
 * terminated string when len is THINKTHEN_NUL_TERMINATED; it is copied before return. error is
 * NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
 * message; passing it again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_question_set_yes_means(struct thinkthen_question *question,
                                         const char *data,
                                         size_t len,
                                         struct thinkthen_error *error);

/**
 * Say what a no answer means, for a decide or filter question.
 *
 * Setting it again replaces it. Returns THINKTHEN_OK, or a negative code with the question
 * unchanged: -THINKTHEN_CODE_USAGE_QUESTION when the question is not decide or filter, or
 * -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8.
 * Pointers and threads:
 * One thread at a time may call this on a question; other questions may be used on other
 * threads. question is a live question. data points to len readable bytes, or to a NUL-
 * terminated string when len is THINKTHEN_NUL_TERMINATED; it is copied before return. error is
 * NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
 * message; passing it again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_question_set_no_means(struct thinkthen_question *question,
                                        const char *data,
                                        size_t len,
                                        struct thinkthen_error *error);

/**
 * Describe one choice, label or level by name, for choose, tag, score and rank.
 *
 * Add the option first; describing the same name again replaces its description. Returns
 * THINKTHEN_OK, or a negative code with the question unchanged: -THINKTHEN_CODE_USAGE_QUESTION
 * when name is not one of the question's options or the function takes no descriptions, or
 * -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8. A question still missing its options
 * is checked when it runs instead.
 * Pointers and threads:
 * One thread at a time may call this on a question; other questions may be used on other
 * threads. question is a live question. name points to name_len readable bytes, or to a NUL-
 * terminated string when name_len is THINKTHEN_NUL_TERMINATED; it is copied before return.
 * description points to len readable bytes, or to a NUL-terminated string when len is
 * THINKTHEN_NUL_TERMINATED; it is copied before return. error is NULL or initialized with
 * THINKTHEN_ERROR_INIT. On a negative return it holds the code and message; passing it again
 * frees what it held, so a reused error leaks nothing. Free it with thinkthen_error_free after
 * its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_question_set_description(struct thinkthen_question *question,
                                           const char *name,
                                           size_t name_len,
                                           const char *description,
                                           size_t len,
                                           struct thinkthen_error *error);

/**
 * Start a question that asks function about text.
 *
 * function is a thinkthen_function. recognize, relate and annotate take empty text; every
 * other function needs a question text. Returns THINKTHEN_OK with *out set to a new question,
 * or a negative code with *out set to NULL: -THINKTHEN_CODE_USAGE_FUNCTION for an unknown
 * function, -THINKTHEN_CODE_USAGE_QUESTION for blank text, or text given to a function that
 * takes none, or -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8. The caller owns *out
 * and frees it with thinkthen_question_free; an old handle in *out is not freed.
 * Pointers and threads:
 * Any thread may call this. data points to len readable bytes, or to a NUL-terminated string
 * when len is THINKTHEN_NUL_TERMINATED; it is copied before return. out is writable. error is
 * NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
 * message; passing it again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_question_new(thinkthen_function function,
                               const char *data,
                               size_t len,
                               struct thinkthen_question **out,
                               struct thinkthen_error *error);

/**
 * Add one choice to a choose question.
 *
 * Returns THINKTHEN_OK, or a negative code with the question unchanged:
 * -THINKTHEN_CODE_USAGE_QUESTION when the question is not choose, or the choice is blank or
 * repeated, or -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8.
 * Pointers and threads:
 * One thread at a time may call this on a question; other questions may be used on other
 * threads. question is a live question. data points to len readable bytes, or to a NUL-
 * terminated string when len is THINKTHEN_NUL_TERMINATED; it is copied before return. error is
 * NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
 * message; passing it again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_question_add_choice(struct thinkthen_question *question,
                                      const char *data,
                                      size_t len,
                                      struct thinkthen_error *error);

/**
 * Add one label to a tag question.
 *
 * Returns THINKTHEN_OK, or a negative code with the question unchanged:
 * -THINKTHEN_CODE_USAGE_QUESTION when the question is not tag, or the label is blank or
 * repeated, or -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8.
 * Pointers and threads:
 * One thread at a time may call this on a question; other questions may be used on other
 * threads. question is a live question. data points to len readable bytes, or to a NUL-
 * terminated string when len is THINKTHEN_NUL_TERMINATED; it is copied before return. error is
 * NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
 * message; passing it again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_question_add_label(struct thinkthen_question *question,
                                     const char *data,
                                     size_t len,
                                     struct thinkthen_error *error);

/**
 * Add one level, lowest first to a score or rank question.
 *
 * Returns THINKTHEN_OK, or a negative code with the question unchanged:
 * -THINKTHEN_CODE_USAGE_QUESTION when the question is not score or rank, or the level is blank
 * or repeated, or -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8.
 * Pointers and threads:
 * One thread at a time may call this on a question; other questions may be used on other
 * threads. question is a live question. data points to len readable bytes, or to a NUL-
 * terminated string when len is THINKTHEN_NUL_TERMINATED; it is copied before return. error is
 * NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
 * message; passing it again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_question_add_level(struct thinkthen_question *question,
                                     const char *data,
                                     size_t len,
                                     struct thinkthen_error *error);

/**
 * Add one kind of name to recognize to a recognize question.
 *
 * Returns THINKTHEN_OK, or a negative code with the question unchanged:
 * -THINKTHEN_CODE_USAGE_QUESTION when the question is not recognize, or the kind is blank or
 * repeated, or -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8.
 * Pointers and threads:
 * One thread at a time may call this on a question; other questions may be used on other
 * threads. question is a live question. data points to len readable bytes, or to a NUL-
 * terminated string when len is THINKTHEN_NUL_TERMINATED; it is copied before return. error is
 * NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
 * message; passing it again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_question_add_kind(struct thinkthen_question *question,
                                    const char *data,
                                    size_t len,
                                    struct thinkthen_error *error);

/**
 * Add one relation rule to a relate question.
 *
 * rule->relation names the relation, rule->source and rule->target the kinds it joins,
 * rule->reads (data NULL to leave it out) how to read it, and rule->either 1 when it runs both
 * ways. Returns THINKTHEN_OK, or a negative code with the question unchanged:
 * -THINKTHEN_CODE_USAGE_QUESTION when the question is not relate or the rule is blank or
 * repeated, -THINKTHEN_CODE_USAGE_ARGUMENT when either is not 0 or 1, or
 * -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8.
 * Pointers and threads:
 * One thread at a time may call this on a question; other questions may be used on other
 * threads. question is a live question. rule points to a readable rule whose text fields are
 * readable; they are copied. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a
 * negative return it holds the code and message; passing it again frees what it held, so a
 * reused error leaks nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_question_add_rule(struct thinkthen_question *question,
                                    const struct thinkthen_rule *rule,
                                    struct thinkthen_error *error);

/**
 * Set the cut a probability must reach for a yes or a choice.
 *
 * Returns THINKTHEN_OK, or a negative code with the question unchanged:
 * -THINKTHEN_CODE_USAGE_THRESHOLD when cut is outside 0 to 1 or the function takes no cut.
 * Pointers and threads:
 * One thread at a time may call this on a question; other questions may be used on other
 * threads. question is a live question. error is NULL or initialized with
 * THINKTHEN_ERROR_INIT. On a negative return it holds the code and message; passing it again
 * frees what it held, so a reused error leaks nothing. Free it with thinkthen_error_free after
 * its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_question_set_cut(struct thinkthen_question *question,
                                   double cut,
                                   struct thinkthen_error *error);

/**
 * Set a band of probabilities, from low to high, that gives THINKTHEN_UNSURE.
 *
 * Only decide and filter take a band. Returns THINKTHEN_OK, or a negative code with
 * the question unchanged: -THINKTHEN_CODE_USAGE_THRESHOLD when the band is out of order,
 * outside 0 to 1, or the function takes none.
 * Pointers and threads:
 * One thread at a time may call this on a question; other questions may be used on other
 * threads. question is a live question. error is NULL or initialized with
 * THINKTHEN_ERROR_INIT. On a negative return it holds the code and message; passing it again
 * frees what it held, so a reused error leaks nothing. Free it with thinkthen_error_free after
 * its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_question_set_band(struct thinkthen_question *question,
                                    double low,
                                    double high,
                                    struct thinkthen_error *error);

/**
 * Point the question at the part of a JSON record to read, as a JSON Pointer.
 *
 * Returns THINKTHEN_OK, or a negative code with the question unchanged:
 * -THINKTHEN_CODE_USAGE_POINTER when data is not a JSON Pointer, or -THINKTHEN_CODE_USAGE_ABI
 * when the text is not UTF-8.
 * Pointers and threads:
 * One thread at a time may call this on a question; other questions may be used on other
 * threads. question is a live question. data points to len readable bytes, or to a NUL-
 * terminated string when len is THINKTHEN_NUL_TERMINATED; it is copied before return. error is
 * NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
 * message; passing it again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_question_set_field(struct thinkthen_question *question,
                                     const char *data,
                                     size_t len,
                                     struct thinkthen_error *error);

/**
 * Keep only the top answered rows of a rank question; failed rows are always kept.
 *
 * Returns THINKTHEN_OK, or a negative code with the question unchanged:
 * -THINKTHEN_CODE_USAGE_QUESTION when the question is not rank or top is 0.
 * Pointers and threads:
 * One thread at a time may call this on a question; other questions may be used on other
 * threads. question is a live question. error is NULL or initialized with
 * THINKTHEN_ERROR_INIT. On a negative return it holds the code and message; passing it again
 * frees what it held, so a reused error leaks nothing. Free it with thinkthen_error_free after
 * its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_question_set_top(struct thinkthen_question *question,
                                   size_t top,
                                   struct thinkthen_error *error);

/**
 * Add a copy of child to an annotate question under name.
 *
 * Changing or freeing child afterwards does not change the annotation. Returns THINKTHEN_OK,
 * or a negative code with the question unchanged: -THINKTHEN_CODE_USAGE_QUESTION when the
 * question is not annotate, child is rank or annotate, or child is incomplete;
 * -THINKTHEN_CODE_USAGE_NAME when name is blank or repeated; or -THINKTHEN_CODE_USAGE_ABI when
 * the text is not UTF-8.
 * Pointers and threads:
 * One thread at a time may call this on a question; other questions may be used on other
 * threads. question is a live question. child is a live question no thread changes during the
 * call. name points to len readable bytes, or to a NUL-terminated string when len is
 * THINKTHEN_NUL_TERMINATED; it is copied before return. error is NULL or initialized with
 * THINKTHEN_ERROR_INIT. On a negative return it holds the code and message; passing it again
 * frees what it held, so a reused error leaks nothing. Free it with thinkthen_error_free after
 * its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_question_add_entry(struct thinkthen_question *question,
                                     const char *name,
                                     size_t len,
                                     const struct thinkthen_question *child,
                                     struct thinkthen_error *error);

/**
 * Free a question.
 *
 * Free a question made by any thinkthen_question_* function. NULL is ignored. Calls, toolboxes
 * and annotations made from it keep their own copies.
 * Pointers and threads:
 * Call this once, after every other use of question has returned.
 */
THINKTHEN_API void thinkthen_question_free(struct thinkthen_question *question);

/**
 * Copy a complete question into a new question that changes on its own.
 *
 * Returns THINKTHEN_OK with *out set to a new question, or a negative code with *out set to
 * NULL: -THINKTHEN_CODE_USAGE_QUESTION when source is incomplete. The caller owns *out and
 * frees it with thinkthen_question_free; an old handle in *out is not freed.
 * Pointers and threads:
 * Any thread may call this while no thread changes source. out is writable. error is NULL or
 * initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and message;
 * passing it again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_question_clone(const struct thinkthen_question *source,
                                 struct thinkthen_question **out,
                                 struct thinkthen_error *error);

/**
 * Read one question from its JSON definition.
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
int32_t thinkthen_question_from_json(const char *data,
                                     size_t len,
                                     struct thinkthen_question **out,
                                     struct thinkthen_error *error);

/**
 * Make settings with every default, without reading keys or the environment.
 *
 * Returns THINKTHEN_OK with *out set. The caller owns *out and frees it with
 * thinkthen_settings_free; an old handle in *out is not freed.
 * Pointers and threads:
 * Any thread may call this. out is writable. error is NULL or initialized with
 * THINKTHEN_ERROR_INIT. On a negative return it holds the code and message; passing it again
 * frees what it held, so a reused error leaks nothing. Free it with thinkthen_error_free after
 * its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_settings_new(struct thinkthen_settings **out,
                               struct thinkthen_error *error);

/**
 * Choose the backend, a thinkthen_backend.
 *
 * Returns THINKTHEN_OK, or -THINKTHEN_CODE_USAGE_BACKEND with the settings unchanged for an
 * unknown backend.
 * Pointers and threads:
 * One thread at a time may call this on a settings object; other settings objects may be used
 * on other threads. settings is live. error is NULL or initialized with THINKTHEN_ERROR_INIT.
 * On a negative return it holds the code and message; passing it again frees what it held, so
 * a reused error leaks nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_settings_set_backend(struct thinkthen_settings *settings,
                                       thinkthen_backend backend,
                                       struct thinkthen_error *error);

/**
 * Set the API key; the library never prints or logs it.
 *
 * Without one, the client reads the backend's key variable. Returns THINKTHEN_OK, or a
 * negative code with the settings unchanged: -THINKTHEN_CODE_USAGE_KEY_MISSING for a blank
 * key, or -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8.
 * Pointers and threads:
 * One thread at a time may call this on a settings object; other settings objects may be used
 * on other threads. settings is live. data points to len readable bytes, or to a NUL-
 * terminated string when len is THINKTHEN_NUL_TERMINATED; it is copied before return. error is
 * NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
 * message; passing it again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_settings_set_api_key(struct thinkthen_settings *settings,
                                       const char *data,
                                       size_t len,
                                       struct thinkthen_error *error);

/**
 * Set the model name the backend uses.
 *
 * Returns THINKTHEN_OK, or -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8, with the
 * settings unchanged.
 * Pointers and threads:
 * One thread at a time may call this on a settings object; other settings objects may be used
 * on other threads. settings is live. data points to len readable bytes, or to a NUL-
 * terminated string when len is THINKTHEN_NUL_TERMINATED; it is copied before return. error is
 * NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
 * message; passing it again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_settings_set_model(struct thinkthen_settings *settings,
                                     const char *data,
                                     size_t len,
                                     struct thinkthen_error *error);

/**
 * Set the backend's base URL.
 *
 * Returns THINKTHEN_OK, or a negative code with the settings unchanged:
 * -THINKTHEN_CODE_USAGE_URL for a URL that is not valid, or -THINKTHEN_CODE_USAGE_ABI when the
 * text is not UTF-8.
 * Pointers and threads:
 * One thread at a time may call this on a settings object; other settings objects may be used
 * on other threads. settings is live. data points to len readable bytes, or to a NUL-
 * terminated string when len is THINKTHEN_NUL_TERMINATED; it is copied before return. error is
 * NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
 * message; passing it again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_settings_set_url(struct thinkthen_settings *settings,
                                   const char *data,
                                   size_t len,
                                   struct thinkthen_error *error);

/**
 * Set how many records go to the backend in one request.
 *
 * Returns THINKTHEN_OK, or -THINKTHEN_CODE_USAGE_BATCH with the settings unchanged when batch
 * is 0 or too large.
 * Pointers and threads:
 * One thread at a time may call this on a settings object; other settings objects may be used
 * on other threads. settings is live. error is NULL or initialized with THINKTHEN_ERROR_INIT.
 * On a negative return it holds the code and message; passing it again frees what it held, so
 * a reused error leaks nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_settings_set_batch(struct thinkthen_settings *settings,
                                     uint64_t batch,
                                     struct thinkthen_error *error);

/**
 * Set how many times a failed request is retried, the first wait, and each attempt's limit, in
 * milliseconds.
 *
 * Returns THINKTHEN_OK, or -THINKTHEN_CODE_USAGE_SETTINGS with the settings unchanged for
 * values out of range.
 * Pointers and threads:
 * One thread at a time may call this on a settings object; other settings objects may be used
 * on other threads. settings is live. error is NULL or initialized with THINKTHEN_ERROR_INIT.
 * On a negative return it holds the code and message; passing it again frees what it held, so
 * a reused error leaks nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_settings_set_retry(struct thinkthen_settings *settings,
                                     uint32_t max_retries,
                                     uint64_t initial_backoff_ms,
                                     uint64_t attempt_timeout_ms,
                                     struct thinkthen_error *error);

/**
 * Free settings.
 *
 * Free settings. NULL is ignored. Clients opened from them keep their own copy.
 * Pointers and threads:
 * Call this once, after every other use of settings has returned.
 */
THINKTHEN_API void thinkthen_settings_free(struct thinkthen_settings *settings);

/**
 * Copy settings into a new settings object, without reading the environment.
 *
 * Returns THINKTHEN_OK with *out set. The caller owns *out and frees it with
 * thinkthen_settings_free; an old handle in *out is not freed.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees source. out is writable. error is
 * NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
 * message; passing it again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_settings_clone(const struct thinkthen_settings *source,
                                 struct thinkthen_settings **out,
                                 struct thinkthen_error *error);

/**
 * Copy only the API key from source into destination.
 *
 * Returns THINKTHEN_OK.
 * Pointers and threads:
 * One thread at a time may change destination, and no thread changes source during the call.
 * error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the
 * code and message; passing it again frees what it held, so a reused error leaks nothing. Free
 * it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_settings_copy_api_key(struct thinkthen_settings *destination,
                                        const struct thinkthen_settings *source,
                                        struct thinkthen_error *error);

/**
 * Read settings from JSON; fields left out take their defaults.
 *
 * Returns THINKTHEN_OK with *out set, or a negative code with *out set to NULL:
 * -THINKTHEN_CODE_USAGE_SETTINGS, or another usage code naming the field, for JSON that is not
 * valid settings, or -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8. The caller owns
 * *out and frees it with thinkthen_settings_free; an old handle in *out is not freed.
 * Pointers and threads:
 * Any thread may call this. data points to len readable bytes, or to a NUL-terminated string
 * when len is THINKTHEN_NUL_TERMINATED; it is copied before return. out is writable. error is
 * NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
 * message; passing it again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_settings_from_json(const char *data,
                                     size_t len,
                                     struct thinkthen_settings **out,
                                     struct thinkthen_error *error);

/**
 * Set the lifetime HTTP-attempt allowance. Zero prevents all remote sends.
 * Existing clients are unaffected; clones of a newly opened client share its allowance.
 * Pointers and threads:
 * settings is live and exclusively borrowed. error is NULL or initialized.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_settings_set_max_requests(struct thinkthen_settings *settings,
                                            uint64_t max_requests,
                                            struct thinkthen_error *error);

/**
 * Make an empty toolbox of named questions.
 *
 * Returns THINKTHEN_OK with *out set. The caller owns *out and frees it with
 * thinkthen_toolbox_free; an old handle in *out is not freed.
 * Pointers and threads:
 * Any thread may call this. out is writable. error is NULL or initialized with
 * THINKTHEN_ERROR_INIT. On a negative return it holds the code and message; passing it again
 * frees what it held, so a reused error leaks nothing. Free it with thinkthen_error_free after
 * its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_toolbox_new(struct thinkthen_toolbox **out,
                              struct thinkthen_error *error);

/**
 * Add a copy of question to the toolbox under name.
 *
 * Returns THINKTHEN_OK, or a negative code with the toolbox unchanged:
 * -THINKTHEN_CODE_USAGE_NAME when name is blank or already used,
 * -THINKTHEN_CODE_USAGE_QUESTION when question is incomplete or annotate, or
 * -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8.
 * Pointers and threads:
 * One thread at a time may call this on a toolbox; other toolboxs may be used on other
 * threads. question is live and no thread changes it during the call. name points to len
 * readable bytes, or to a NUL-terminated string when len is THINKTHEN_NUL_TERMINATED; it is
 * copied before return. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative
 * return it holds the code and message; passing it again frees what it held, so a reused error
 * leaks nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_toolbox_insert(struct thinkthen_toolbox *toolbox,
                                 const char *name,
                                 size_t len,
                                 const struct thinkthen_question *question,
                                 struct thinkthen_error *error);

/**
 * Copy the question named name out of the toolbox.
 *
 * Returns THINKTHEN_OK with *out set, or a negative code with *out set to NULL:
 * -THINKTHEN_CODE_USAGE_NAME when no question has that name, or -THINKTHEN_CODE_USAGE_ABI when
 * the text is not UTF-8. The caller owns *out and frees it with thinkthen_question_free; it
 * outlives the toolbox, and an old handle in *out is not freed.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees toolbox. name points to len
 * readable bytes, or to a NUL-terminated string when len is THINKTHEN_NUL_TERMINATED; it is
 * copied before return. out is writable. error is NULL or initialized with
 * THINKTHEN_ERROR_INIT. On a negative return it holds the code and message; passing it again
 * frees what it held, so a reused error leaks nothing. Free it with thinkthen_error_free after
 * its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_toolbox_select(const struct thinkthen_toolbox *toolbox,
                                 const char *name,
                                 size_t len,
                                 struct thinkthen_question **out,
                                 struct thinkthen_error *error);

/**
 * Free a toolbox.
 *
 * Free a toolbox. NULL is ignored. Questions selected from it and calls opened from them keep
 * their own copies.
 * Pointers and threads:
 * Call this once, after every other use of toolbox has returned.
 */
THINKTHEN_API void thinkthen_toolbox_free(struct thinkthen_toolbox *toolbox);

/**
 * Read a toolbox from the command's entries JSON, without reading files or running anything.
 *
 * Returns THINKTHEN_OK with *out set, or a negative code with *out set to NULL:
 * -THINKTHEN_CODE_LOCAL_TOOLBOX, -THINKTHEN_CODE_USAGE_NAME or -THINKTHEN_CODE_USAGE_QUESTION
 * for JSON that is not a valid toolbox, or -THINKTHEN_CODE_USAGE_ABI when the text is not
 * UTF-8. The caller owns *out and frees it with thinkthen_toolbox_free; an old handle in *out
 * is not freed.
 * Pointers and threads:
 * Any thread may call this. data points to len readable bytes, or to a NUL-terminated string
 * when len is THINKTHEN_NUL_TERMINATED; it is copied before return. out is writable. error is
 * NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
 * message; passing it again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_toolbox_from_json(const char *data,
                                    size_t len,
                                    struct thinkthen_toolbox **out,
                                    struct thinkthen_error *error);

/**
 * Copy the question named name out of the toolbox, checking that it asks function.
 *
 * Returns THINKTHEN_OK with *out set, or a negative code with *out set to NULL:
 * -THINKTHEN_CODE_USAGE_FUNCTION for an unknown function, -THINKTHEN_CODE_USAGE_NAME when no
 * question has that name or it asks another function, or -THINKTHEN_CODE_USAGE_ABI when the
 * text is not UTF-8. The caller owns *out and frees it with thinkthen_question_free; it
 * outlives the toolbox, and an old handle in *out is not freed.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees toolbox. name points to len
 * readable bytes, or to a NUL-terminated string when len is THINKTHEN_NUL_TERMINATED; it is
 * copied before return. out is writable. error is NULL or initialized with
 * THINKTHEN_ERROR_INIT. On a negative return it holds the code and message; passing it again
 * frees what it held, so a reused error leaks nothing. Free it with thinkthen_error_free after
 * its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_toolbox_select_as(const struct thinkthen_toolbox *toolbox,
                                    const char *name,
                                    size_t len,
                                    thinkthen_function function,
                                    struct thinkthen_question **out,
                                    struct thinkthen_error *error);

/**
 * Read a toolbox from flat question definitions, as thinkthen_toolbox_definitions_json writes
 * them.
 *
 * Returns THINKTHEN_OK with *out set, or a negative code with *out set to NULL: a usage code
 * for definitions that are not valid, or -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8.
 * The caller owns *out and frees it with thinkthen_toolbox_free; an old handle in *out is not
 * freed.
 * Pointers and threads:
 * Any thread may call this. data points to len readable bytes, or to a NUL-terminated string
 * when len is THINKTHEN_NUL_TERMINATED; it is copied before return. out is writable. error is
 * NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
 * message; passing it again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_toolbox_from_definitions_json(const char *data,
                                                size_t len,
                                                struct thinkthen_toolbox **out,
                                                struct thinkthen_error *error);

/**
 * Write the toolbox in the command's entries JSON to *out.
 *
 * Returns THINKTHEN_OK with *out written, or -THINKTHEN_CODE_INTERNAL_BUG with *out empty.
 * *out is a new buffer the caller owns; free it with thinkthen_buffer_free. The old contents
 * of *out are not freed, so free a buffer before passing it again.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees toolbox. out is writable. error is
 * NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
 * message; passing it again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_toolbox_to_json(const struct thinkthen_toolbox *toolbox,
                                  struct thinkthen_buffer *out,
                                  struct thinkthen_error *error);

/**
 * Write the toolbox as flat question definitions to *out.
 *
 * Returns THINKTHEN_OK with *out written, or -THINKTHEN_CODE_INTERNAL_BUG with *out empty.
 * *out is a new buffer the caller owns; free it with thinkthen_buffer_free. The old contents
 * of *out are not freed, so free a buffer before passing it again.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees toolbox. out is writable. error is
 * NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
 * message; passing it again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_toolbox_definitions_json(const struct thinkthen_toolbox *toolbox,
                                           struct thinkthen_buffer *out,
                                           struct thinkthen_error *error);

/**
 * Write a complete question's JSON definition to *out.
 *
 * Returns THINKTHEN_OK with *out written, or a negative code with *out empty:
 * -THINKTHEN_CODE_USAGE_QUESTION when the question is incomplete. *out is a new buffer the
 * caller owns; free it with thinkthen_buffer_free. The old contents of *out are not freed, so
 * free a buffer before passing it again.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees question. out is writable. error
 * is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
 * message; passing it again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_question_to_json(const struct thinkthen_question *question,
                                   struct thinkthen_buffer *out,
                                   struct thinkthen_error *error);

/**
 * Write the question's thinkthen_function to *out.
 *
 * Returns THINKTHEN_OK with *out written. It works on an incomplete question too.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees question. out is writable. error
 * is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
 * message; passing it again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_question_function(const struct thinkthen_question *question,
                                    thinkthen_function *out,
                                    struct thinkthen_error *error);

/**
 * Ask a yes-or-no question about one text, and write the answer to *answer.
 *
 * Returns THINKTHEN_OK with *answer written; THINKTHEN_UNSURE with *answer unchanged (the
 * default cut never gives it; set a low and a high bar with thinkthen_decide_band); or a
 * negative thinkthen_code with *answer unchanged:
 * -THINKTHEN_CODE_RECORD_EMPTY for empty or blank text (never sent to the backend), a
 * usage code for a bad argument, or a backend code.
 * A string argument that is not UTF-8 returns -THINKTHEN_CODE_USAGE_ABI.
 * Pointers and threads:
 * Any thread may call this, and threads may share the client. question is a NUL-terminated
 * string. text points to len readable bytes (or a NUL-terminated string with
 * THINKTHEN_NUL_TERMINATED). answer is writable. error is NULL or initialized with
 * THINKTHEN_ERROR_INIT. On a negative return it holds the code and message; passing it
 * again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_decide(const struct thinkthen_client *client,
                         const char *question,
                         const char *text,
                         size_t len,
                         bool *answer,
                         struct thinkthen_error *error);

/**
 * Ask a yes-or-no question about one text with a low and a high bar, and write the answer
 * to *answer.
 *
 * Returns THINKTHEN_OK with *answer written: false for a probability below low, true for one
 * at or above high. Returns THINKTHEN_UNSURE with *answer unchanged from low up to just under
 * high. Otherwise returns a negative thinkthen_code with *answer unchanged:
 * -THINKTHEN_CODE_USAGE_THRESHOLD unless 0 < low < high <= 1,
 * -THINKTHEN_CODE_RECORD_EMPTY for empty, blank or NULL text, -THINKTHEN_CODE_RECORD_INVALID
 * for text that is not UTF-8, or a usage code for a bad argument, none of which reaches the
 * backend; or a backend code.
 * A question that is not UTF-8 returns -THINKTHEN_CODE_USAGE_ABI.
 * Pointers and threads:
 * Any thread may call this, and threads may share the client. question is a NUL-terminated
 * string. text points to len readable bytes (or a NUL-terminated string with
 * THINKTHEN_NUL_TERMINATED). answer is writable. error is NULL or initialized with
 * THINKTHEN_ERROR_INIT. On a negative return it holds the code and message; passing it
 * again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_decide_band(const struct thinkthen_client *client,
                              const char *question,
                              double low,
                              double high,
                              const char *text,
                              size_t len,
                              bool *answer,
                              struct thinkthen_error *error);

/**
 * Choose one of count choices for one text, and write the chosen index to *choice.
 *
 * *choice is an index into choices, so the caller's own string names the answer. Returns
 * THINKTHEN_OK with *choice written; THINKTHEN_UNSURE with *choice unchanged when no
 * choice reaches the cut; or a negative thinkthen_code with *choice unchanged:
 * usage.question for fewer than two or repeated choices, -THINKTHEN_CODE_RECORD_EMPTY for
 * blank text, or a backend code.
 * A string argument that is not UTF-8 returns -THINKTHEN_CODE_USAGE_ABI.
 * Pointers and threads:
 * Any thread may call this, and threads may share the client. question and each of the
 * count choices are NUL-terminated strings. text points to len readable bytes (or a NUL-
 * terminated string with THINKTHEN_NUL_TERMINATED). choice is writable. error is NULL or
 * initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
 * message; passing it again frees what it held, so a reused error leaks nothing. Free it
 * with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_choose(const struct thinkthen_client *client,
                         const char *question,
                         const char *const *choices,
                         size_t count,
                         const char *text,
                         size_t len,
                         size_t *choice,
                         struct thinkthen_error *error);

/**
 * Tag one text with any of count labels, and write whether each applies to selected.
 *
 * selected has count entries, in the order of labels; an answer with no labels sets them
 * all to false. Returns THINKTHEN_OK with selected written, or a negative thinkthen_code
 * with selected unchanged: usage.question for no or repeated labels,
 * -THINKTHEN_CODE_RECORD_EMPTY for blank text, or a backend code.
 * A string argument that is not UTF-8 returns -THINKTHEN_CODE_USAGE_ABI.
 * Pointers and threads:
 * Any thread may call this, and threads may share the client. question and each of the
 * count labels are NUL-terminated strings. text points to len readable bytes (or a NUL-
 * terminated string with THINKTHEN_NUL_TERMINATED). selected points to count writable
 * bools. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_tag(const struct thinkthen_client *client,
                      const char *question,
                      const char *const *labels,
                      size_t count,
                      const char *text,
                      size_t len,
                      bool *selected,
                      struct thinkthen_error *error);

/**
 * Score one text on count ordered levels, and write the score to *score.
 *
 * The score runs from 0 for the first level to 1 for the last. Returns THINKTHEN_OK with
 * *score written, or a negative thinkthen_code with *score unchanged: usage.question for
 * fewer than two or repeated levels, -THINKTHEN_CODE_RECORD_EMPTY for blank text, or a
 * backend code.
 * A string argument that is not UTF-8 returns -THINKTHEN_CODE_USAGE_ABI.
 * Pointers and threads:
 * Any thread may call this, and threads may share the client. question and each of the
 * count levels are NUL-terminated strings. text points to len readable bytes (or a NUL-
 * terminated string with THINKTHEN_NUL_TERMINATED). score is writable. error is NULL or
 * initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
 * message; passing it again frees what it held, so a reused error leaks nothing. Free it
 * with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_score(const struct thinkthen_client *client,
                        const char *question,
                        const char *const *levels,
                        size_t count,
                        const char *text,
                        size_t len,
                        double *score,
                        struct thinkthen_error *error);

/**
 * Find the part of one text that answers a question, and write it to *found.
 *
 * found->present is 1 when a part answers it and 0 when the answer is that none does.
 * found->start and found->end are byte offsets into text, and found->text points into the
 * caller's own text, so it lives as long as text does. Returns THINKTHEN_OK with *found
 * written; THINKTHEN_UNSURE with *found unchanged; or a negative thinkthen_code with
 * *found unchanged: -THINKTHEN_CODE_RECORD_EMPTY for blank text, a usage code for a bad
 * argument, or a backend code.
 * A string argument that is not UTF-8 returns -THINKTHEN_CODE_USAGE_ABI.
 * Pointers and threads:
 * Any thread may call this, and threads may share the client. question is a NUL-terminated
 * string. text points to len readable bytes (or a NUL-terminated string with
 * THINKTHEN_NUL_TERMINATED). found is writable. error is NULL or initialized with
 * THINKTHEN_ERROR_INIT. On a negative return it holds the code and message; passing it
 * again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_find(const struct thinkthen_client *client,
                       const char *question,
                       const char *text,
                       size_t len,
                       struct thinkthen_span *found,
                       struct thinkthen_error *error);

/**
 * Recognize names of count kinds in one text, and write them to a new value.
 *
 * Read the names with thinkthen_value_name_count and thinkthen_value_name_at; their
 * offsets are bytes into text. Free the value with thinkthen_value_free. Returns
 * THINKTHEN_OK with *names set (an answer may hold no names), or a negative thinkthen_code
 * with *names unchanged: usage.question for no or repeated kinds,
 * -THINKTHEN_CODE_RECORD_EMPTY for blank text, or a backend code.
 * A string argument that is not UTF-8 returns -THINKTHEN_CODE_USAGE_ABI.
 * Pointers and threads:
 * Any thread may call this, and threads may share the client. Each of the count kinds is a
 * NUL-terminated string. text points to len readable bytes (or a NUL-terminated string
 * with THINKTHEN_NUL_TERMINATED). names is writable. error is NULL or initialized with
 * THINKTHEN_ERROR_INIT. On a negative return it holds the code and message; passing it
 * again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_recognize(const struct thinkthen_client *client,
                            const char *const *kinds,
                            size_t count,
                            const char *text,
                            size_t len,
                            struct thinkthen_value **names,
                            struct thinkthen_error *error);

/**
 * Find relations of count rules in one text, and write them to a new value.
 *
 * Read the relations with thinkthen_value_relation_count and thinkthen_value_relation_at,
 * and the names they join with thinkthen_value_name_count and thinkthen_value_name_at.
 * Free the value with thinkthen_value_free. Returns THINKTHEN_OK with *relations set (an
 * answer may hold no relations), or a negative thinkthen_code with *relations unchanged:
 * usage.question for no rules or a bad rule, -THINKTHEN_CODE_RECORD_EMPTY for blank text,
 * or a backend code.
 * A string argument that is not UTF-8 returns -THINKTHEN_CODE_USAGE_ABI.
 * Pointers and threads:
 * Any thread may call this, and threads may share the client. rules points to count
 * readable rules whose text fields are readable. text points to len readable bytes (or a
 * NUL-terminated string with THINKTHEN_NUL_TERMINATED). relations is writable. error is
 * NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code
 * and message; passing it again frees what it held, so a reused error leaks nothing. Free
 * it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_relate(const struct thinkthen_client *client,
                         const struct thinkthen_rule *rules,
                         size_t count,
                         const char *text,
                         size_t len,
                         struct thinkthen_value **relations,
                         struct thinkthen_error *error);

/**
 * Ask count named questions about one text, and write their answers to a new value.
 *
 * Read each answer by name with thinkthen_value_annotation_get, or by index with
 * thinkthen_value_annotation_at; each child follows its own function's rules. Free the
 * value with thinkthen_value_free. Returns THINKTHEN_OK with *annotation set, or a
 * negative thinkthen_code with *annotation unchanged: usage.name for repeated or bad
 * names, usage.question for a rank or annotate child, -THINKTHEN_CODE_RECORD_EMPTY for
 * blank text, or a backend code.
 * A string argument that is not UTF-8 returns -THINKTHEN_CODE_USAGE_ABI.
 * Pointers and threads:
 * Any thread may call this, and threads may share the client and the questions while no
 * thread changes them. names holds count NUL-terminated strings and questions count live
 * question handles. text points to len readable bytes (or a NUL-terminated string with
 * THINKTHEN_NUL_TERMINATED). annotation is writable. error is NULL or initialized with
 * THINKTHEN_ERROR_INIT. On a negative return it holds the code and message; passing it
 * again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_annotate(const struct thinkthen_client *client,
                           const char *const *names,
                           const struct thinkthen_question *const *questions,
                           size_t count,
                           const char *text,
                           size_t len,
                           struct thinkthen_value **annotation,
                           struct thinkthen_error *error);

/**
 * Get a wake source for a streaming call, for a host that waits on input and output at once.
 *
 * Read thinkthen_events_version before thinkthen_call_next. When it returns
 * THINKTHEN_NEEDS_INPUT, wait for your own input or thinkthen_events_wait(version), whichever
 * comes first. If input comes first, call thinkthen_events_wake, join the waiter, then send.
 * If an event comes first, call thinkthen_call_next again. Cancelling the call wakes the
 * source. Returns a new handle the caller owns; free it with thinkthen_events_free after every
 * wait and wake has returned. It may outlive the call.
 * Pointers and threads:
 * The thread driving call calls this. call is live.
 */
THINKTHEN_API struct thinkthen_events *thinkthen_call_events(const struct thinkthen_call *call);

/**
 * Return the wake source's current version.
 *
 * Read it before you check the call, then wait on it.
 * Pointers and threads:
 * Any thread may call this while events is live; event functions may run at the same time.
 */
THINKTHEN_API uint64_t thinkthen_events_version(const struct thinkthen_events *events);

/**
 * Wait until the version is no longer version; return at once if it already changed.
 *
 * A wake asks you to check the call again. It does not promise a row.
 * Pointers and threads:
 * Any thread may call this while events is live; event functions may run at the same time.
 */
THINKTHEN_API void thinkthen_events_wait(const struct thinkthen_events *events, uint64_t version);

/**
 * Wake every wait on an earlier version, including a wait not yet entered.
 *
 * It does not cancel the call or change its input.
 * Pointers and threads:
 * Any thread may call this while events is live; event functions may run at the same time.
 */
THINKTHEN_API void thinkthen_events_wake(const struct thinkthen_events *events);

/**
 * Free a wake source.
 *
 * NULL is ignored.
 * Pointers and threads:
 * Call this once, after every version, wait and wake call on events has returned.
 */
THINKTHEN_API void thinkthen_events_free(struct thinkthen_events *events);

/**
 * Fill error with the shared usage.settings error, for a binding that cannot convert its own
 * settings.
 *
 * Returns -THINKTHEN_CODE_USAGE_SETTINGS.
 * Pointers and threads:
 * Any thread may call this. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a
 * negative return it holds the code and message; passing it again frees what it held, so a
 * reused error leaks nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD int32_t thinkthen_settings_conversion_error(struct thinkthen_error *error);

/**
 * Fill error with the shared usage.question error, for a binding that cannot convert its own
 * question.
 *
 * Returns -THINKTHEN_CODE_USAGE_QUESTION.
 * Pointers and threads:
 * Any thread may call this. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a
 * negative return it holds the code and message; passing it again frees what it held, so a
 * reused error leaks nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD int32_t thinkthen_question_conversion_error(struct thinkthen_error *error);

/**
 * Fill error with the shared local.cancelled error, for a binding that stops a call itself.
 *
 * Returns -THINKTHEN_CODE_LOCAL_CANCELLED.
 * Pointers and threads:
 * Any thread may call this. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a
 * negative return it holds the code and message; passing it again frees what it held, so a
 * reused error leaks nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD int32_t thinkthen_cancelled_error(struct thinkthen_error *error);

/**
 * Return the loaded library's binary interface version.
 *
 * THINKTHEN_ABI_MAJOR is in the high 16 bits and THINKTHEN_ABI_MINOR in the low 16. Pass it
 * with THINKTHEN_ABI_VERSION to thinkthen_abi_compatible. Any thread may call this.
 */
THINKTHEN_API uint32_t thinkthen_abi_version(void);

/**
 * Return the loaded library's version, such as "0.2.0".
 *
 * The string is NUL-terminated and lasts for the whole process; never free it. Compare it with
 * THINKTHEN_VERSION. Any thread may call this.
 */
THINKTHEN_API const char *thinkthen_version(void);

/**
 * Return a code's dotted name, such as "record.empty", or NULL for an unknown code.
 *
 * value may be positive or negative. The string is NUL-terminated and lasts for the whole
 * process; never free it. Any thread may call this.
 */
THINKTHEN_API const char *thinkthen_code_name(int32_t value);

/**
 * Return a code's family, such as "record", or NULL for an unknown code or 0.
 *
 * The string is NUL-terminated and lasts for the whole process; never free it. Any thread may
 * call this.
 */
THINKTHEN_API const char *thinkthen_code_family(int32_t value);

/**
 * Open a client from settings, or from the environment when settings is NULL.
 *
 * With NULL settings, THINKTHEN_BACKEND picks the backend and the backend's own variables give
 * the key and URL. THINKTHEN_MAX_REQUESTS supplies the client allowance only on this
 * explicitly from-environment path. The client copies the settings, so they may be freed at once. Returns
 * THINKTHEN_OK with *out set, or a negative code with *out set to NULL:
 * -THINKTHEN_CODE_USAGE_BACKEND for an unknown THINKTHEN_BACKEND,
 * -THINKTHEN_CODE_USAGE_KEY_MISSING when the backend needs a key and none is set, or another
 * usage code for settings that are not valid. The caller owns *out and frees it with
 * thinkthen_client_free; an old handle in *out is not freed.
 * Pointers and threads:
 * Any thread may call this. settings is NULL or live, and no thread changes it during the
 * call. out is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative
 * return it holds the code and message; passing it again frees what it held, so a reused error
 * leaks nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_client_open(const struct thinkthen_settings *settings,
                              struct thinkthen_client **out,
                              struct thinkthen_error *error);

/**
 * Open a client on one backend with every other setting at its default.
 *
 * This is the shortcut for thinkthen_settings_new, thinkthen_settings_set_backend,
 * thinkthen_client_open and thinkthen_settings_free. The key comes from the backend's own
 * environment variable, and the stub backend needs none. Returns THINKTHEN_OK with *out set,
 * or a negative code with *out set to NULL: -THINKTHEN_CODE_USAGE_BACKEND for an unknown
 * backend, or -THINKTHEN_CODE_USAGE_KEY_MISSING when the backend needs a key and none is set.
 * The caller owns *out and frees it with thinkthen_client_free; an old handle in *out is not
 * freed.
 * Pointers and threads:
 * Any thread may call this. out is writable. error is NULL or initialized with
 * THINKTHEN_ERROR_INIT. On a negative return it holds the code and message; passing it again
 * frees what it held, so a reused error leaks nothing. Free it with thinkthen_error_free after
 * its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_client_open_backend(thinkthen_backend backend,
                                      struct thinkthen_client **out,
                                      struct thinkthen_error *error);

/**
 * Open a client from settings JSON.
 *
 * Returns THINKTHEN_OK with *out set, or a negative code with *out set to NULL: a usage code
 * for settings that are not valid, -THINKTHEN_CODE_USAGE_KEY_MISSING when the backend needs a
 * key and none is set, or -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8. The caller
 * owns *out and frees it with thinkthen_client_free; an old handle in *out is not freed.
 * Pointers and threads:
 * Any thread may call this. settings points to len readable bytes, or to a NUL-terminated
 * string when len is THINKTHEN_NUL_TERMINATED; it is copied before return. out is writable.
 * error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the
 * code and message; passing it again frees what it held, so a reused error leaks nothing. Free
 * it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_client_open_json(const char *settings,
                                   size_t len,
                                   struct thinkthen_client **out,
                                   struct thinkthen_error *error);

/**
 * Copy cumulative client usage into an independently owned JSON buffer.
 *
 * The snapshot remains valid after the client is freed. Active attempts are
 * unresolved; this is client-wide accounting, not SQL statement accounting.
 * Free the returned bytes with thinkthen_buffer_free.
 * Pointers and threads:
 * client is live, out is writable and empty, and error is NULL or initialized.
 * No thread may free client until this function returns.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_client_usage(const struct thinkthen_client *client,
                               struct thinkthen_buffer *out,
                               struct thinkthen_error *error);

/**
 * Wait for owned calls to settle and copy final cumulative client usage.
 *
 * First stop admitting calls and cancel/free every owned unfinished call.
 * Unlike thinkthen_client_usage, this waits for their engine workers to end.
 * The returned JSON has the same independently owned format.
 * Pointers and threads:
 * client is live; no thread opens new calls or frees it during this function.
 * out is writable and empty, and error is NULL or initialized.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_client_usage_settled(const struct thinkthen_client *client,
                                       struct thinkthen_buffer *out,
                                       struct thinkthen_error *error);

/**
 * Free a client.
 *
 * Free a client. NULL is ignored. Calls opened from it keep running on their own.
 * Pointers and threads:
 * Call this once, after every other use of client has returned.
 */
THINKTHEN_API void thinkthen_client_free(struct thinkthen_client *client);

/**
 * Open a streaming call that asks question about each record you send.
 *
 * offsets is a thinkthen_offset; C uses THINKTHEN_OFFSET_UTF8. name, when not NULL, names the
 * run in each row's source; a present empty name is refused. Read the call with
 * thinkthen_call_next. Returns THINKTHEN_OK with *out set, or a negative code with *out set to
 * NULL: -THINKTHEN_CODE_USAGE_QUESTION for an incomplete question, -THINKTHEN_CODE_USAGE_NAME
 * for a bad name, -THINKTHEN_CODE_USAGE_OFFSETS for an unknown offsets unit, or
 * -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8. The caller owns *out and frees it with
 * thinkthen_call_free; an old handle in *out is not freed.
 * Pointers and threads:
 * Any thread may call this while no thread changes question. client and question are live;
 * both may be freed after return. name is NULL or points to a readable thinkthen_text. out is
 * writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_call_open_question(const struct thinkthen_client *client,
                                     const struct thinkthen_question *question,
                                     const struct thinkthen_text *name,
                                     thinkthen_offset offsets,
                                     struct thinkthen_call **out,
                                     struct thinkthen_error *error);

/**
 * Open a call from request JSON, for a host that speaks JSON; question files are never read.
 *
 * Returns THINKTHEN_OK with *out set, or a negative code with *out set to NULL: a usage code
 * for a request that is not valid, -THINKTHEN_CODE_USAGE_OFFSETS for an unknown offsets unit,
 * or -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8. The caller owns *out and frees it
 * with thinkthen_call_free; an old handle in *out is not freed.
 * Pointers and threads:
 * Any thread may call this. client is live. request points to len readable bytes, or to a NUL-
 * terminated string when len is THINKTHEN_NUL_TERMINATED; it is copied before return. out is
 * writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_call_open_json(const struct thinkthen_client *client,
                                 const char *request,
                                 size_t len,
                                 thinkthen_offset offsets,
                                 struct thinkthen_call **out,
                                 struct thinkthen_error *error);

/**
 * Send one text record to a call.
 *
 * Send only after thinkthen_call_next returns THINKTHEN_NEEDS_INPUT. Returns THINKTHEN_OK, or
 * -THINKTHEN_CODE_USAGE_ABI when the call is not waiting for input. Text that is not UTF-8 is
 * sent as a record that fails as record.invalid, and blank text fails as record.empty; neither
 * reaches the backend.
 * Pointers and threads:
 * One thread at a time drives call; thinkthen_call_cancel may run on any thread. data points
 * to len readable bytes, or to a NUL-terminated string when len is THINKTHEN_NUL_TERMINATED;
 * it is copied before return. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a
 * negative return it holds the code and message; passing it again frees what it held, so a
 * reused error leaks nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_call_send_text(struct thinkthen_call *call,
                                 const char *data,
                                 size_t len,
                                 struct thinkthen_error *error);

/**
 * Send one JSON record to a call.
 *
 * Send only after thinkthen_call_next returns THINKTHEN_NEEDS_INPUT. Returns THINKTHEN_OK, or
 * -THINKTHEN_CODE_USAGE_ABI when the call is not waiting for input. JSON that is not valid is
 * sent as a record that fails as record.json.
 * Pointers and threads:
 * One thread at a time drives call; thinkthen_call_cancel may run on any thread. data points
 * to len readable bytes, or to a NUL-terminated string when len is THINKTHEN_NUL_TERMINATED;
 * it is copied before return. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a
 * negative return it holds the code and message; passing it again frees what it held, so a
 * reused error leaks nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_call_send_json(struct thinkthen_call *call,
                                 const char *data,
                                 size_t len,
                                 struct thinkthen_error *error);

/**
 * Send one UTF-16 text record, in the machine's byte order, to a call.
 *
 * Send only after thinkthen_call_next returns THINKTHEN_NEEDS_INPUT. Returns THINKTHEN_OK, or
 * -THINKTHEN_CODE_USAGE_ABI when the call is not waiting for input. Text with an unpaired
 * surrogate is sent as a record that fails as record.invalid.
 * Pointers and threads:
 * One thread at a time drives call; thinkthen_call_cancel may run on any thread. data points
 * to len readable, aligned uint16_t units; it may be NULL when len is 0. error is NULL or
 * initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and message;
 * passing it again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_call_send_utf16(struct thinkthen_call *call,
                                  const uint16_t *data,
                                  size_t len,
                                  struct thinkthen_error *error);

/**
 * Send a missing value, such as a SQL NULL, to a call.
 *
 * Send only after thinkthen_call_next returns THINKTHEN_NEEDS_INPUT. Returns THINKTHEN_OK, or
 * -THINKTHEN_CODE_USAGE_ABI when the call is not waiting for input. It fails as record.missing
 * without a backend request.
 * Pointers and threads:
 * One thread at a time drives call; thinkthen_call_cancel may run on any thread. error is NULL
 * or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
 * message; passing it again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_call_send_missing(struct thinkthen_call *call,
                                    struct thinkthen_error *error);

/**
 * Send a value the host could not convert to a call.
 *
 * Send only after thinkthen_call_next returns THINKTHEN_NEEDS_INPUT. Returns THINKTHEN_OK, or
 * -THINKTHEN_CODE_USAGE_ABI when the call is not waiting for input. It fails as record.invalid
 * without a backend request.
 * Pointers and threads:
 * One thread at a time drives call; thinkthen_call_cancel may run on any thread. error is NULL
 * or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
 * message; passing it again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_call_send_invalid(struct thinkthen_call *call,
                                    struct thinkthen_error *error);

/**
 * Say no more records are coming.
 *
 * Rows already sent still arrive; thinkthen_call_next returns THINKTHEN_END after the last. It
 * returns at once.
 * Pointers and threads:
 * One thread at a time drives call; thinkthen_call_cancel may run on any thread. call is live.
 */
THINKTHEN_API void thinkthen_call_end_input(struct thinkthen_call *call);

/**
 * Read the call's next event: a row, a request for input, or the end.
 *
 * Returns THINKTHEN_OK with the next row in *out; THINKTHEN_NEEDS_INPUT when a streaming call
 * waits for a send or thinkthen_call_end_input; THINKTHEN_END when every row is read; or a
 * negative code for the call as a whole, such as -THINKTHEN_CODE_LOCAL_CANCELLED. A record
 * that fails is still THINKTHEN_OK: its row has status THINKTHEN_STATUS_FAILED and its own
 * code, and reading goes on. *out is empty unless the return is THINKTHEN_OK. A filter row's
 * input comes from thinkthen_row_original. After a call-level error, every later read returns
 * that same error until the call is freed; a failed call never returns THINKTHEN_END.
 * Pointers and threads:
 * One thread at a time drives call; thinkthen_call_cancel may run on any thread. out is
 * initialized with THINKTHEN_ROW_INIT. Passing it again frees the row it held, so a reused row
 * leaks nothing; free it with thinkthen_row_free after its last use. error is NULL or
 * initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and message;
 * passing it again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_call_next(struct thinkthen_call *call,
                            struct thinkthen_row *out,
                            struct thinkthen_error *error);

/**
 * Read the next kept record of a filter call, skipping the records it drops.
 *
 * This is filter's usual view: a record the model answers no for, or is not sure about, is
 * skipped. thinkthen_call_next stays the view of every row. Returns THINKTHEN_OK with *row
 * holding a kept row (thinkthen_row_original gives its input); THINKTHEN_NEEDS_INPUT when a
 * streaming call waits for a send; THINKTHEN_END when every record is judged; or a negative
 * code. A failed record returns its own negative code in its place, with its row in *row and
 * its message in error, and you may keep reading the records after it.
 * -THINKTHEN_CODE_USAGE_FUNCTION means the call is not a filter call.
 * Pointers and threads:
 * One thread at a time drives call; thinkthen_call_cancel may run on any thread. row is
 * initialized with THINKTHEN_ROW_INIT. Passing it again frees the row it held, so a reused row
 * leaks nothing; free it with thinkthen_row_free after its last use. error is NULL or
 * initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and message;
 * passing it again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_call_next_kept(struct thinkthen_call *call,
                                 struct thinkthen_row *row,
                                 struct thinkthen_error *error);

/**
 * Ask a call to stop.
 *
 * It returns at once. Once the call stops, thinkthen_call_next returns
 * -THINKTHEN_CODE_LOCAL_CANCELLED, and a wait on the call's events wakes.
 * Pointers and threads:
 * Any thread may call this, also while another thread drives call. call is live until every
 * such call returns.
 */
THINKTHEN_API void thinkthen_call_cancel(struct thinkthen_call *call);

/**
 * Cancel and free a call.
 *
 * Free a call. NULL is ignored. It does not wait for a backend request in flight. Rows already
 * read stay valid.
 * Pointers and threads:
 * Call this once, after every other use of call, including thinkthen_call_cancel, has
 * returned.
 */
THINKTHEN_API void thinkthen_call_free(struct thinkthen_call *call);

/**
 * Free a buffer's contents and leave it empty.
 *
 * NULL and an empty buffer are ignored, so a freed buffer may be freed again.
 * Pointers and threads:
 * Call this after the last read of buffer.
 */
THINKTHEN_API void thinkthen_buffer_free(struct thinkthen_buffer *buffer);

/**
 * Free an error's message and leave it empty.
 *
 * NULL, an empty error and an error not initialized with THINKTHEN_ERROR_INIT are ignored, so
 * a freed error may be freed again or passed again.
 * Pointers and threads:
 * Call this after the last read of error.
 */
THINKTHEN_API void thinkthen_error_free(struct thinkthen_error *error);

/**
 * Free a row's contents and leave it empty.
 *
 * NULL, an empty row and a row not initialized with THINKTHEN_ROW_INIT are ignored, so a freed
 * row may be freed again or passed again. Views borrowed from the row end here.
 * Pointers and threads:
 * Call this after the last read of row and its views.
 */
THINKTHEN_API void thinkthen_row_free(struct thinkthen_row *row);

/**
 * Open a call over count records with a question from request JSON.
 *
 * The request must not carry its own records. Read the call with thinkthen_call_next; it never
 * returns THINKTHEN_NEEDS_INPUT. Returns THINKTHEN_OK with *out set, or a negative code with
 * *out set to NULL: a usage code for a request that is not valid or carries records,
 * -THINKTHEN_CODE_USAGE_RECORD_KIND for an unknown record kind, -THINKTHEN_CODE_USAGE_ABI for
 * a bad record size or stride, -THINKTHEN_CODE_USAGE_OFFSETS for an unknown offsets unit, or
 * -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8. The caller owns *out and frees it with
 * thinkthen_call_free; an old handle in *out is not freed.
 * Pointers and threads:
 * Any thread may call this. client is live. request points to len readable bytes, or to a NUL-
 * terminated string when len is THINKTHEN_NUL_TERMINATED; it is copied before return. records
 * points to count readable thinkthen_record values, each record_stride bytes apart and each
 * with struct_size set, whose bytes are copied before return. out is writable. error is NULL
 * or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
 * message; passing it again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_call_open_records_json(const struct thinkthen_client *client,
                                         const char *request,
                                         size_t len,
                                         const struct thinkthen_record_input *records,
                                         size_t count,
                                         size_t record_stride,
                                         thinkthen_offset offsets,
                                         struct thinkthen_call **out,
                                         struct thinkthen_error *error);

/**
 * Open a call that asks question about count records.
 *
 * name works as in thinkthen_call_open_question. Read the call with thinkthen_call_next; it
 * never returns THINKTHEN_NEEDS_INPUT. Returns THINKTHEN_OK with *out set, or a negative code
 * with *out set to NULL: -THINKTHEN_CODE_USAGE_QUESTION for an incomplete question,
 * -THINKTHEN_CODE_USAGE_NAME for a bad name, -THINKTHEN_CODE_USAGE_RECORD_KIND for an unknown
 * record kind, -THINKTHEN_CODE_USAGE_ABI for a bad record size or stride, or
 * -THINKTHEN_CODE_USAGE_OFFSETS for an unknown offsets unit. The caller owns *out and frees it
 * with thinkthen_call_free; an old handle in *out is not freed.
 * Pointers and threads:
 * Any thread may call this while no thread changes question. client and question are live.
 * records points to count readable thinkthen_record values, each record_stride bytes apart and
 * each with struct_size set, whose bytes are copied before return. name is NULL or points to a
 * readable thinkthen_text. out is writable. error is NULL or initialized with
 * THINKTHEN_ERROR_INIT. On a negative return it holds the code and message; passing it again
 * frees what it held, so a reused error leaks nothing. Free it with thinkthen_error_free after
 * its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_call_open_records_question(const struct thinkthen_client *client,
                                             const struct thinkthen_question *question,
                                             const struct thinkthen_text *name,
                                             const struct thinkthen_record_input *records,
                                             size_t count,
                                             size_t record_stride,
                                             thinkthen_offset offsets,
                                             struct thinkthen_call **out,
                                             struct thinkthen_error *error);

/**
 * Write the input of a filter or rank row to *out.
 *
 * *out borrows from the row and lasts until thinkthen_row_free(row), even after the call and
 * client are freed; never free it. Returns THINKTHEN_OK with *out written, or a negative code
 * with *out unchanged: -THINKTHEN_CODE_USAGE_FUNCTION when the row is not a filter or rank row, or
 * -THINKTHEN_CODE_USAGE_ABI when the row did not come from this library.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees row. out is writable. error is
 * NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
 * message; passing it again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_row_original(const struct thinkthen_row *row,
                               struct thinkthen_record_view *out,
                               struct thinkthen_error *error);

/**
 * Copy an opt-in response-store configuration into settings. NULL disables storage.
 * No store is opened and no credential is read until client construction.
 * Pointers and threads:
 * settings is live and exclusively borrowed. config is NULL or initialized with
 * its full struct_size; text and optional scalar pointers remain readable until
 * return. error is NULL or initialized. Callers may release all inputs on return.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_settings_set_response_store(struct thinkthen_settings *settings,
                                              const struct thinkthen_response_store_config *config,
                                              struct thinkthen_error *error);

/**
 * Answer one question about one record and write its whole row to *out.
 *
 * Returns THINKTHEN_OK when the record is answered, THINKTHEN_UNSURE when the model is not
 * sure, or a negative thinkthen_code: the record's own code when it fails (for example
 * -THINKTHEN_CODE_RECORD_EMPTY for blank text, which is never sent), a backend code when
 * the backend fails, or a usage code for a bad argument. Read row.answer_bool, row.number
 * or row.rank only after THINKTHEN_OK. The row is written in every case except a usage
 * error. Passing the same row again frees what it held, so a reused row leaks nothing;
 * free it with thinkthen_row_free after its last use.
 * Pointers and threads:
 * Any thread may call this, and threads may share the client and the question while no
 * thread changes the question. data points to len readable bytes (or a NUL-terminated
 * string with THINKTHEN_NUL_TERMINATED). out is initialized with THINKTHEN_ROW_INIT or
 * freed after its last use; error is NULL or initialized with THINKTHEN_ERROR_INIT. On a
 * negative return it holds the code and message; passing it again frees what it held, so a
 * reused error leaks nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_run_one(const struct thinkthen_client *client,
                          const struct thinkthen_question *question,
                          const char *data,
                          size_t len,
                          thinkthen_record_kind kind,
                          struct thinkthen_row *out,
                          struct thinkthen_error *error);

/**
 * Copy client usage into a sized typed output, with no allocation or waiting.
 * Pointers and threads:
 * client is live and cannot be freed concurrently. out is writable, initialized
 * with struct_size = sizeof(thinkthen_usage). error is NULL or initialized.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_client_usage_values(const struct thinkthen_client *client,
                                      struct thinkthen_usage *out,
                                      struct thinkthen_error *error);

/**
 * Wait for owned calls to settle and copy usage into a sized typed output.
 * First stop admitting calls and cancel/free owned unfinished calls.
 * Pointers and threads:
 * client is live; no thread opens new calls or frees it during this function.
 * out is writable with struct_size = sizeof(thinkthen_usage); error is NULL or initialized.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_client_usage_values_settled(const struct thinkthen_client *client,
                                              struct thinkthen_usage *out,
                                              struct thinkthen_error *error);

/**
 * Copy all usage and store metrics from one atomic account snapshot without waiting.
 * Pointers and threads:
 * client is live; out is writable with its top-level struct_size initialized;
 * error is NULL or initialized. No thread frees client until return.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_client_usage_v2_values(const struct thinkthen_client *client,
                                         struct thinkthen_usage_v2 *out,
                                         struct thinkthen_error *error);

/**
 * Wait for owned calls to settle, then copy all usage and store metrics atomically.
 * Pointers and threads:
 * Stop admission and cancel/free unfinished owned calls first. client remains live,
 * out is writable with top-level struct_size initialized; error is NULL or initialized.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_client_usage_v2_values_settled(const struct thinkthen_client *client,
                                                 struct thinkthen_usage_v2 *out,
                                                 struct thinkthen_error *error);

/**
 * Write a choose value's choice to *out.
 *
 * Returns THINKTHEN_OK with *out written; THINKTHEN_UNSURE with *out unchanged when the value
 * is not sure; or a negative code with *out unchanged: the record's own code when the value
 * failed, -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's. *out borrows
 * from value and lasts until thinkthen_value_free(value); never free it.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_choice(const struct thinkthen_value *value,
                               struct thinkthen_text *out,
                               struct thinkthen_error *error);

/**
 * Write a decide or filter value's answer to *out: 1 for yes and 0 for no.
 *
 * Returns THINKTHEN_OK with *out written; THINKTHEN_UNSURE with *out unchanged when the value
 * is not sure; or a negative code with *out unchanged: the record's own code when the value
 * failed, -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_bool(const struct thinkthen_value *value,
                             uint8_t *out,
                             struct thinkthen_error *error);

/**
 * Write a score value's score, from 0 to 1, to *out.
 *
 * Returns THINKTHEN_OK with *out written; THINKTHEN_UNSURE with *out unchanged when the value
 * is not sure; or a negative code with *out unchanged: the record's own code when the value
 * failed, -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_number(const struct thinkthen_value *value,
                               double *out,
                               struct thinkthen_error *error);

/**
 * Write a rank value's rank, 1 for the first, to *out.
 *
 * Returns THINKTHEN_OK with *out written; THINKTHEN_UNSURE with *out unchanged when the value
 * is not sure; or a negative code with *out unchanged: the record's own code when the value
 * failed, -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_rank(const struct thinkthen_value *value,
                             uint64_t *out,
                             struct thinkthen_error *error);

/**
 * Write how many labels a tag value applies to *out; 0 is a real answer.
 *
 * Returns THINKTHEN_OK with *out written; THINKTHEN_UNSURE with *out unchanged when the value
 * is not sure; or a negative code with *out unchanged: the record's own code when the value
 * failed, -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_tag_count(const struct thinkthen_value *value,
                                  size_t *out,
                                  struct thinkthen_error *error);

/**
 * Write a tag value's label at index to *out.
 *
 * Returns THINKTHEN_OK with *out written; THINKTHEN_UNSURE with *out unchanged when the value
 * is not sure; or a negative code with *out unchanged: the record's own code when the value
 * failed, -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's, or
 * -THINKTHEN_CODE_USAGE_ARGUMENT when index is out of range. *out borrows from value and lasts
 * until thinkthen_value_free(value); never free it.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_tag_at(const struct thinkthen_value *value,
                               size_t index,
                               struct thinkthen_text *out,
                               struct thinkthen_error *error);

/**
 * Write a find value's answer to *out; out->present is 0 when the answer is that no part
 * answers it.
 *
 * Returns THINKTHEN_OK with *out written; THINKTHEN_UNSURE with *out unchanged when the value
 * is not sure; or a negative code with *out unchanged: the record's own code when the value
 * failed, -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's. *out borrows
 * from value and lasts until thinkthen_value_free(value); never free it.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_found(const struct thinkthen_value *value,
                              struct thinkthen_span *out,
                              struct thinkthen_error *error);

/**
 * Write how many names a recognize value, or a relate value's detail, holds to *out.
 *
 * Returns THINKTHEN_OK with *out written; THINKTHEN_UNSURE with *out unchanged when the value
 * is not sure; or a negative code with *out unchanged: the record's own code when the value
 * failed, -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_name_count(const struct thinkthen_value *value,
                                   size_t *out,
                                   struct thinkthen_error *error);

/**
 * Write the name at index, with its kind and probability, to *out.
 *
 * Returns THINKTHEN_OK with *out written; THINKTHEN_UNSURE with *out unchanged when the value
 * is not sure; or a negative code with *out unchanged: the record's own code when the value
 * failed, -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's, or
 * -THINKTHEN_CODE_USAGE_ARGUMENT when index is out of range. *out borrows from value and lasts
 * until thinkthen_value_free(value); never free it.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_name_at(const struct thinkthen_value *value,
                                size_t index,
                                struct thinkthen_name *out,
                                struct thinkthen_error *error);

/**
 * Write how many relations a relate value holds to *out.
 *
 * Returns THINKTHEN_OK with *out written; THINKTHEN_UNSURE with *out unchanged when the value
 * is not sure; or a negative code with *out unchanged: the record's own code when the value
 * failed, -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_relation_count(const struct thinkthen_value *value,
                                       size_t *out,
                                       struct thinkthen_error *error);

/**
 * Write the relation at index, with both names it joins, to *out.
 *
 * Returns THINKTHEN_OK with *out written; THINKTHEN_UNSURE with *out unchanged when the value
 * is not sure; or a negative code with *out unchanged: the record's own code when the value
 * failed, -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's, or
 * -THINKTHEN_CODE_USAGE_ARGUMENT when index is out of range. *out borrows from value and lasts
 * until thinkthen_value_free(value); never free it.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_relation_at(const struct thinkthen_value *value,
                                    size_t index,
                                    struct thinkthen_relation *out,
                                    struct thinkthen_error *error);

/**
 * Write how many named answers an annotate value holds, failed ones included, to *out.
 *
 * Returns THINKTHEN_OK with *out written; THINKTHEN_UNSURE with *out unchanged when the value
 * is not sure; or a negative code with *out unchanged: the record's own code when the value
 * failed, -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_annotation_count(const struct thinkthen_value *value,
                                         size_t *out,
                                         struct thinkthen_error *error);

/**
 * Write the name of the answer at index to *name and a copy of the answer to *out.
 *
 * Returns THINKTHEN_OK with *name and *out written; THINKTHEN_UNSURE with *out unchanged when
 * the value is not sure; or a negative code with *out unchanged: the record's own code when
 * the value failed, -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's, or
 * -THINKTHEN_CODE_USAGE_ARGUMENT when index is out of range. *name borrows from value and
 * lasts until thinkthen_value_free(value). The caller owns *out, which follows its own
 * function's rules, and frees it with thinkthen_value_free; an old handle in *out is not
 * freed.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is live. name and out
 * are writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return
 * it holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_annotation_at(const struct thinkthen_value *value,
                                      size_t index,
                                      struct thinkthen_text *name,
                                      struct thinkthen_value **out,
                                      struct thinkthen_error *error);

/**
 * Look up one annotation answer by its question's name, and copy it into a new value.
 *
 * The new value follows its own function's rules, so read it with that function's accessor.
 * Returns THINKTHEN_OK with *out set; THINKTHEN_UNSURE with *out set to NULL when the
 * annotation itself is not sure; or a negative code with *out set to NULL: the record's own
 * code when it failed, -THINKTHEN_CODE_USAGE_FUNCTION when the value is not an annotation,
 * -THINKTHEN_CODE_USAGE_NAME when no question has that name, or -THINKTHEN_CODE_USAGE_ABI when
 * the text is not UTF-8. The caller owns *out and frees it with thinkthen_value_free; an old
 * handle in *out is not freed.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. name points to len readable
 * bytes, or to a NUL-terminated string when len is THINKTHEN_NUL_TERMINATED; it is copied
 * before return. out is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a
 * negative return it holds the code and message; passing it again frees what it held, so a
 * reused error leaks nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_annotation_get(const struct thinkthen_value *value,
                                       const char *name,
                                       size_t len,
                                       struct thinkthen_value **out,
                                       struct thinkthen_error *error);

/**
 * Write how many label probabilities a choose or tag value's detail holds to *out.
 *
 * Returns THINKTHEN_OK with *out written, or a negative code with *out unchanged:
 * -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's. A value without detail
 * writes 0.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_label_count(const struct thinkthen_value *value,
                                    size_t *out,
                                    struct thinkthen_error *error);

/**
 * Write the label probability at index, chosen or not, to *out.
 *
 * Returns THINKTHEN_OK with *out written, or a negative code with *out unchanged:
 * -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's, or
 * -THINKTHEN_CODE_USAGE_ARGUMENT when an index is out of range or the value has no detail.
 * *out borrows from value and lasts until thinkthen_value_free(value); never free it.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_label_at(const struct thinkthen_value *value,
                                 size_t index,
                                 struct thinkthen_probability *out,
                                 struct thinkthen_error *error);

/**
 * Write how many level probabilities a score or rank value's detail holds to *out.
 *
 * Returns THINKTHEN_OK with *out written, or a negative code with *out unchanged:
 * -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's. A value without detail
 * writes 0.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_level_count(const struct thinkthen_value *value,
                                    size_t *out,
                                    struct thinkthen_error *error);

/**
 * Write the probability of the level at index, in question order, to *out.
 *
 * Returns THINKTHEN_OK with *out written, or a negative code with *out unchanged:
 * -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's, or
 * -THINKTHEN_CODE_USAGE_ARGUMENT when an index is out of range or the value has no detail.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_level_at(const struct thinkthen_value *value,
                                 size_t index,
                                 double *out,
                                 struct thinkthen_error *error);

/**
 * Write how many pieces a recognize value's detail split the text into to *out.
 *
 * Returns THINKTHEN_OK with *out written, or a negative code with *out unchanged:
 * -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's. A value without detail
 * writes 0.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_piece_count(const struct thinkthen_value *value,
                                    size_t *out,
                                    struct thinkthen_error *error);

/**
 * Write the piece at index, with its span, to *out.
 *
 * Returns THINKTHEN_OK with *out written, or a negative code with *out unchanged:
 * -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's, or
 * -THINKTHEN_CODE_USAGE_ARGUMENT when an index is out of range or the value has no detail.
 * *out borrows from value and lasts until thinkthen_value_free(value); never free it.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_piece_at(const struct thinkthen_value *value,
                                 size_t index,
                                 struct thinkthen_span *out,
                                 struct thinkthen_error *error);

/**
 * Write how many boundary probabilities the piece at index holds to *out.
 *
 * Returns THINKTHEN_OK with *out written, or a negative code with *out unchanged:
 * -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's, or
 * -THINKTHEN_CODE_USAGE_ARGUMENT when an index is out of range or the value has no detail.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_piece_tag_count(const struct thinkthen_value *value,
                                        size_t index,
                                        size_t *out,
                                        struct thinkthen_error *error);

/**
 * Write boundary probability tag of the piece at index to *out.
 *
 * Returns THINKTHEN_OK with *out written, or a negative code with *out unchanged:
 * -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's, or
 * -THINKTHEN_CODE_USAGE_ARGUMENT when an index is out of range or the value has no detail.
 * *out borrows from value and lasts until thinkthen_value_free(value); never free it.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_piece_tag_at(const struct thinkthen_value *value,
                                     size_t index,
                                     size_t tag,
                                     struct thinkthen_probability *out,
                                     struct thinkthen_error *error);

/**
 * Write how many proposed names a recognize value's detail holds to *out.
 *
 * Returns THINKTHEN_OK with *out written, or a negative code with *out unchanged:
 * -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's. A value without detail
 * writes 0.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_kind_count(const struct thinkthen_value *value,
                                   size_t *out,
                                   struct thinkthen_error *error);

/**
 * Write the text and span of the proposed name at index to *out.
 *
 * Returns THINKTHEN_OK with *out written, or a negative code with *out unchanged:
 * -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's, or
 * -THINKTHEN_CODE_USAGE_ARGUMENT when an index is out of range or the value has no detail.
 * *out borrows from value and lasts until thinkthen_value_free(value); never free it.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_kind_span(const struct thinkthen_value *value,
                                  size_t index,
                                  struct thinkthen_span *out,
                                  struct thinkthen_error *error);

/**
 * Write how many kind probabilities the proposed name at index holds to *out.
 *
 * Returns THINKTHEN_OK with *out written, or a negative code with *out unchanged:
 * -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's, or
 * -THINKTHEN_CODE_USAGE_ARGUMENT when an index is out of range or the value has no detail.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_kind_probability_count(const struct thinkthen_value *value,
                                               size_t index,
                                               size_t *out,
                                               struct thinkthen_error *error);

/**
 * Write kind probability choice of the proposed name at index to *out.
 *
 * Returns THINKTHEN_OK with *out written, or a negative code with *out unchanged:
 * -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's, or
 * -THINKTHEN_CODE_USAGE_ARGUMENT when an index is out of range or the value has no detail.
 * *out borrows from value and lasts until thinkthen_value_free(value); never free it.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_kind_probability_at(const struct thinkthen_value *value,
                                            size_t index,
                                            size_t choice,
                                            struct thinkthen_probability *out,
                                            struct thinkthen_error *error);

/**
 * Write how many name pairs a relate value's detail judged, related or not, to *out.
 *
 * Returns THINKTHEN_OK with *out written, or a negative code with *out unchanged:
 * -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's. A value without detail
 * writes 0.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_pair_count(const struct thinkthen_value *value,
                                   size_t *out,
                                   struct thinkthen_error *error);

/**
 * Write the judged pair at index to *out.
 *
 * Returns THINKTHEN_OK with *out written, or a negative code with *out unchanged:
 * -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's, or
 * -THINKTHEN_CODE_USAGE_ARGUMENT when an index is out of range or the value has no detail.
 * *out borrows from value and lasts until thinkthen_value_free(value); never free it.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_pair_at(const struct thinkthen_value *value,
                                size_t index,
                                struct thinkthen_relation *out,
                                struct thinkthen_error *error);

/**
 * Return 1 when a library with binary interface actual serves a caller built for required, and
 * 0 when not.
 *
 * The major versions must match, and actual's minor must be at least required's. Any thread
 * may call this.
 */
THINKTHEN_API uint8_t thinkthen_abi_compatible(uint32_t required, uint32_t actual);

/**
 * Return how many codes thinkthen_code_at lists.
 *
 * Any thread may call this.
 */
THINKTHEN_API size_t thinkthen_code_count(void);

/**
 * Return the code at index, or 0 when index is out of range; 0 is never a code.
 *
 * Any thread may call this.
 */
THINKTHEN_API int32_t thinkthen_code_at(size_t index);

/**
 * Copy a row's answer into a new value, which the thinkthen_value_* functions read.
 *
 * Returns THINKTHEN_OK with *out set, or a negative code with *out set to NULL:
 * -THINKTHEN_CODE_USAGE_ABI when the row is empty or did not come from this library. The
 * caller owns *out, which outlives the row, and frees it with thinkthen_value_free; an old
 * handle in *out is not freed.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees row. out is writable. error is
 * NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
 * message; passing it again frees what it held, so a reused error leaks nothing. Free it with
 * thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_row_values(const struct thinkthen_row *row,
                             struct thinkthen_value **out,
                             struct thinkthen_error *error);

/**
 * Free a value.
 *
 * Free a value. NULL is ignored. Views borrowed from it end here.
 * Pointers and threads:
 * Call this once, after every other use of value and its views has returned.
 */
THINKTHEN_API void thinkthen_value_free(struct thinkthen_value *value);

/**
 * Write the value's thinkthen_function to *out.
 *
 * Returns THINKTHEN_OK with *out written.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_function(const struct thinkthen_value *value,
                                 thinkthen_function *out,
                                 struct thinkthen_error *error);

/**
 * Write the value's thinkthen_status to *out.
 *
 * Returns THINKTHEN_OK with *out written.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_status(const struct thinkthen_value *value,
                               thinkthen_status *out,
                               struct thinkthen_error *error);

/**
 * Write which question, backend and model gave the value to *out.
 *
 * Returns THINKTHEN_OK with *out written, or -THINKTHEN_CODE_USAGE_ABI with *out unchanged for
 * an annotation entry, which has no source. *out borrows from value and lasts until
 * thinkthen_value_free(value); never free it.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_source(const struct thinkthen_value *value,
                               struct thinkthen_source *out,
                               struct thinkthen_error *error);

/**
 * Write the record's id to *out, exactly: integers never pass through a double.
 *
 * out->kind is one of the THINKTHEN_ID_* kinds. Returns THINKTHEN_OK with *out written, or
 * -THINKTHEN_CODE_USAGE_ABI with *out unchanged for an annotation entry. *out borrows from
 * value and lasts until thinkthen_value_free(value); never free it.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_id(const struct thinkthen_value *value,
                           struct thinkthen_id *out,
                           struct thinkthen_error *error);

/**
 * Copy a failed value's code and message into *result; it stays empty for a value that did not
 * fail.
 *
 * result is initialized with THINKTHEN_ERROR_INIT. Passing it again frees what it held, so a
 * reused result leaks nothing; free it with thinkthen_error_free. Returns THINKTHEN_OK, or
 * -THINKTHEN_CODE_USAGE_ABI when result is not initialized.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is live. result and
 * error are different variables. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a
 * negative return it holds the code and message; passing it again frees what it held, so a
 * reused error leaks nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_error(const struct thinkthen_value *value,
                              struct thinkthen_error *result,
                              struct thinkthen_error *error);

/**
 * Write the value's probability to *out; NaN when its function has none.
 *
 * Returns THINKTHEN_OK with *out written.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_probability(const struct thinkthen_value *value,
                                    double *out,
                                    struct thinkthen_error *error);

/**
 * Write 1 to *out when the value carries detail, even an empty list, and 0 when it does not.
 *
 * Returns THINKTHEN_OK with *out written.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_has_detail(const struct thinkthen_value *value,
                                   uint8_t *out,
                                   struct thinkthen_error *error);

/**
 * Write the run's name to *out; out->data is NULL when the run has none.
 *
 * Returns THINKTHEN_OK with *out written, or -THINKTHEN_CODE_USAGE_ABI with *out unchanged for
 * an annotation entry. *out borrows from value and lasts until thinkthen_value_free(value);
 * never free it.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_source_name(const struct thinkthen_value *value,
                                    struct thinkthen_text *out,
                                    struct thinkthen_error *error);

/**
 * Write the input of a filter or rank value to *out.
 *
 * Returns THINKTHEN_OK with *out written, or -THINKTHEN_CODE_USAGE_ABI with *out unchanged
 * when the value has no input. *out borrows from value and lasts until
 * thinkthen_value_free(value); never free it.
 * Pointers and threads:
 * Any thread may call this while no thread changes or frees value. value is a live value. out
 * is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
 * holds the code and message; passing it again frees what it held, so a reused error leaks
 * nothing. Free it with thinkthen_error_free after its last use.
 */
THINKTHEN_API THINKTHEN_NODISCARD
int32_t thinkthen_value_original(const struct thinkthen_value *value,
                                 struct thinkthen_record_view *out,
                                 struct thinkthen_error *error);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus


/*
 * THINKTHEN_ERROR_INIT and THINKTHEN_ROW_INIT initialize an empty error or row:
 * thinkthen_error error = THINKTHEN_ERROR_INIT; thinkthen_row row = THINKTHEN_ROW_INIT;
 * They set struct_size, which every function checks before it writes the slot.
 */
#ifdef __cplusplus
#define THINKTHEN_ERROR_INIT {sizeof(thinkthen_error), {}, {}, {}}
#define THINKTHEN_ROW_INIT {sizeof(thinkthen_row), {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}}
#else
#define THINKTHEN_ERROR_INIT {.struct_size = sizeof(thinkthen_error)}
#define THINKTHEN_ROW_INIT {.struct_size = sizeof(thinkthen_row)}
#endif
#endif  /* THINKTHEN_H */
