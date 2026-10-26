package thinkthen

/*
#include "loader.h"
*/
import "C"

import "errors"

type Code int32

const (
	CodeUsageThreshold    Code = C.THINKTHEN_CODE_USAGE_THRESHOLD
	CodeUsageQuestion     Code = C.THINKTHEN_CODE_USAGE_QUESTION
	CodeUsageName         Code = C.THINKTHEN_CODE_USAGE_NAME
	CodeUsageSettings     Code = C.THINKTHEN_CODE_USAGE_SETTINGS
	CodeUsageBackend      Code = C.THINKTHEN_CODE_USAGE_BACKEND
	CodeUsageURL          Code = C.THINKTHEN_CODE_USAGE_URL
	CodeUsageKeyMissing   Code = C.THINKTHEN_CODE_USAGE_KEY_MISSING
	CodeUsageBatch        Code = C.THINKTHEN_CODE_USAGE_BATCH
	CodeUsagePointer      Code = C.THINKTHEN_CODE_USAGE_POINTER
	CodeUsageSQLPolicy    Code = C.THINKTHEN_CODE_USAGE_SQL_POLICY
	CodeUsageSQLSelection Code = C.THINKTHEN_CODE_USAGE_SQL_SELECTION
	CodeUsageABI          Code = C.THINKTHEN_CODE_USAGE_ABI
	CodeRecordJSON        Code = C.THINKTHEN_CODE_RECORD_JSON
	CodeRecordMissing     Code = C.THINKTHEN_CODE_RECORD_MISSING
	CodeRecordTooLarge    Code = C.THINKTHEN_CODE_RECORD_TOO_LARGE
	CodeRecordInvalid     Code = C.THINKTHEN_CODE_RECORD_INVALID
	CodeBackendTransport  Code = C.THINKTHEN_CODE_BACKEND_TRANSPORT
	CodeBackendStatus     Code = C.THINKTHEN_CODE_BACKEND_STATUS
	CodeBackendReply      Code = C.THINKTHEN_CODE_BACKEND_REPLY
	CodeLocalCancelled    Code = C.THINKTHEN_CODE_LOCAL_CANCELLED
	CodeLocalIO           Code = C.THINKTHEN_CODE_LOCAL_IO
	CodeLocalToolbox      Code = C.THINKTHEN_CODE_LOCAL_TOOLBOX
	CodeInternalBug       Code = C.THINKTHEN_CODE_INTERNAL_BUG
)

var codeNames = map[Code]string{
	CodeUsageThreshold:    "usage.threshold",
	CodeUsageQuestion:     "usage.question",
	CodeUsageName:         "usage.name",
	CodeUsageSettings:     "usage.settings",
	CodeUsageBackend:      "usage.backend",
	CodeUsageURL:          "usage.url",
	CodeUsageKeyMissing:   "usage.key-missing",
	CodeUsageBatch:        "usage.batch",
	CodeUsagePointer:      "usage.pointer",
	CodeUsageSQLPolicy:    "usage.sql-policy",
	CodeUsageSQLSelection: "usage.sql-selection",
	CodeUsageABI:          "usage.abi",
	CodeRecordJSON:        "record.json",
	CodeRecordMissing:     "record.missing",
	CodeRecordTooLarge:    "record.too-large",
	CodeRecordInvalid:     "record.invalid",
	CodeBackendTransport:  "backend.transport",
	CodeBackendStatus:     "backend.status",
	CodeBackendReply:      "backend.reply",
	CodeLocalCancelled:    "local.cancelled",
	CodeLocalIO:           "local.io",
	CodeLocalToolbox:      "local.toolbox",
	CodeInternalBug:       "internal.bug",
}

func (c Code) String() string {
	if name, ok := codeNames[c]; ok {
		return name
	}
	return "unknown"
}
func (c Code) Error() string { return c.String() }

var ErrUsage = errors.New("thinkthen: usage")
var ErrTransport = errors.New("thinkthen: transport")
var ErrClosed = errors.New("thinkthen: closed")

type Error struct {
	code       Code
	message    string
	cause      error
	httpStatus int
}

func (e *Error) Code() Code { return e.code }
func (e *Error) Error() string {
	if e == nil {
		return ""
	}
	return e.message
}
func (e *Error) Unwrap() error           { return e.cause }
func (e *Error) HTTPStatus() (int, bool) { return e.httpStatus, e.httpStatus != 0 }
func (e *Error) Is(target error) bool {
	if code, ok := target.(Code); ok {
		return e.code == code
	}
	return target == ErrUsage && e.code >= CodeUsageThreshold && e.code <= CodeUsageABI || target == ErrTransport && e.code == CodeBackendTransport
}

type Backend uint32

const (
	SystemOne Backend = C.THINKTHEN_BACKEND_SYSTEMONE
	OpenAI    Backend = C.THINKTHEN_BACKEND_OPENAI
	Stub      Backend = C.THINKTHEN_BACKEND_STUB
)

func normalizedBackend(b Backend) Backend {
	if b == 0 {
		return SystemOne
	}
	return b
}
func (v Backend) String() string {
	switch v {
	case SystemOne:
		return "systemone"
	case OpenAI:
		return "openai"
	case Stub:
		return "stub"
	}
	return "unknown"
}

type Status uint32

const (
	Answered Status = C.THINKTHEN_STATUS_ANSWERED
	Unsure   Status = C.THINKTHEN_STATUS_UNSURE
	Failed   Status = C.THINKTHEN_STATUS_FAILED
)

func (v Status) String() string {
	switch v {
	case Answered:
		return "answered"
	case Unsure:
		return "unsure"
	case Failed:
		return "failed"
	}
	return "unknown"
}

type Function uint32

const (
	FunctionDecide    Function = C.THINKTHEN_FUNCTION_DECIDE
	FunctionChoose    Function = C.THINKTHEN_FUNCTION_CHOOSE
	FunctionTag       Function = C.THINKTHEN_FUNCTION_TAG
	FunctionScore     Function = C.THINKTHEN_FUNCTION_SCORE
	FunctionFilter    Function = C.THINKTHEN_FUNCTION_FILTER
	FunctionFind      Function = C.THINKTHEN_FUNCTION_FIND
	FunctionRecognize Function = C.THINKTHEN_FUNCTION_RECOGNIZE
	FunctionRelate    Function = C.THINKTHEN_FUNCTION_RELATE
	FunctionRank      Function = C.THINKTHEN_FUNCTION_RANK
	FunctionAnnotate  Function = C.THINKTHEN_FUNCTION_ANNOTATE
)

func (v Function) String() string {
	switch v {
	case FunctionDecide:
		return "decide"
	case FunctionChoose:
		return "choose"
	case FunctionTag:
		return "tag"
	case FunctionScore:
		return "score"
	case FunctionFilter:
		return "filter"
	case FunctionFind:
		return "find"
	case FunctionRecognize:
		return "recognize"
	case FunctionRelate:
		return "relate"
	case FunctionRank:
		return "rank"
	case FunctionAnnotate:
		return "annotate"
	}
	return "unknown"
}
func codeForName(name string) (Code, bool) {
	for code, value := range codeNames {
		if value == name {
			return code, true
		}
	}
	return 0, false
}

func abiError() error { return &Error{code: CodeUsageABI, message: C.THINKTHEN_ABI_ERROR_MESSAGE} }
