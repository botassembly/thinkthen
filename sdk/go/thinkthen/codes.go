package thinkthen

/*
#include "loader.h"
*/
import "C"

import (
	"errors"
	"strings"
)

// Code is a stable engine error code and can be matched with errors.Is.
type Code int32

// String returns the engine code name.
func (c Code) String() string {
	if name, ok := codeNames[c]; ok {
		return name
	}
	return "unknown"
}

// Error returns the engine message.
func (c Code) Error() string { return c.String() }

// Family returns the engine contract family, or an empty string for an unknown code.
func (c Code) Family() string {
	family, _, ok := strings.Cut(c.String(), ".")
	if !ok {
		return ""
	}
	return family
}

// ErrUsage matches errors in the usage family through errors.Is.
var ErrUsage = errors.New("thinkthen: usage")

// ErrRecord matches errors in the record family through errors.Is.
var ErrRecord = errors.New("thinkthen: record")

// ErrBackend matches errors in the backend family through errors.Is.
var ErrBackend = errors.New("thinkthen: backend")

// ErrLocal matches errors in the local family through errors.Is.
var ErrLocal = errors.New("thinkthen: local")

// ErrInternal matches errors in the internal family through errors.Is.
var ErrInternal = errors.New("thinkthen: internal")

// ErrUnsure matches an outcome for which the engine is not sure.
var ErrUnsure = errors.New("thinkthen: unsure")

// ErrClosed matches a call on a closed client.
var ErrClosed = errors.New("thinkthen: closed")

// Error preserves an engine error code, message, cause and optional HTTP status.
type Error struct {
	code       Code
	message    string
	cause      error
	httpStatus int
	noAnswer   bool
}

// Code returns the stable engine error code.
func (e *Error) Code() Code { return e.code }

// Error returns the engine message.
func (e *Error) Error() string {
	if e == nil {
		return ""
	}
	return e.message
}

// Unwrap returns the underlying cause.
func (e *Error) Unwrap() error { return e.cause }

// HTTPStatus returns the backend HTTP status when present.
func (e *Error) HTTPStatus() (int, bool) { return e.httpStatus, e.httpStatus != 0 }

// Is matches an exact code or an error family.
func (e *Error) Is(target error) bool {
	if target == ErrNoAnswer {
		return e.noAnswer
	}
	if code, ok := target.(Code); ok {
		return e.code == code
	}
	if target == ErrClosed {
		return e.code == CodeUsageClosed
	}
	switch target {
	case ErrUsage:
		return e.code.Family() == "usage"
	case ErrRecord:
		return e.code.Family() == "record"
	case ErrBackend:
		return e.code.Family() == "backend"
	case ErrLocal:
		return e.code.Family() == "local"
	case ErrInternal:
		return e.code.Family() == "internal"
	}
	return false
}

// Backend selects a built-in engine backend.
type Backend uint32

const (
	// SystemOne selects the System One backend.
	SystemOne Backend = C.THINKTHEN_BACKEND_SYSTEMONE
	// OpenAI selects the OpenAI backend.
	OpenAI Backend = C.THINKTHEN_BACKEND_OPENAI
	// Stub selects deterministic offline test answers.
	Stub Backend = C.THINKTHEN_BACKEND_STUB
)

func normalizedBackend(b Backend) Backend {
	if b == 0 {
		return SystemOne
	}
	return b
}

// String returns a readable representation.
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

// Status identifies whether an outcome is answered, unsure, or failed.
type Status uint32

const (
	// Answered means the outcome has an answer.
	Answered Status = C.THINKTHEN_STATUS_ANSWERED
	// Unsure means the engine did not select a definite answer.
	Unsure Status = C.THINKTHEN_STATUS_UNSURE
	// Failed means the record carries an error.
	Failed Status = C.THINKTHEN_STATUS_FAILED
)

// String returns a readable representation.
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

// Function identifies one of the ten semantic functions.
type Function uint32

const (
	// FunctionDecide identifies the decide function.
	FunctionDecide Function = C.THINKTHEN_FUNCTION_DECIDE
	// FunctionChoose identifies the choose function.
	FunctionChoose Function = C.THINKTHEN_FUNCTION_CHOOSE
	// FunctionTag identifies the tag function.
	FunctionTag Function = C.THINKTHEN_FUNCTION_TAG
	// FunctionScore identifies the score function.
	FunctionScore Function = C.THINKTHEN_FUNCTION_SCORE
	// FunctionFilter identifies the filter function.
	FunctionFilter Function = C.THINKTHEN_FUNCTION_FILTER
	// FunctionFind identifies the find function.
	FunctionFind Function = C.THINKTHEN_FUNCTION_FIND
	// FunctionRecognize identifies the recognize function.
	FunctionRecognize Function = C.THINKTHEN_FUNCTION_RECOGNIZE
	// FunctionRelate identifies the relate function.
	FunctionRelate Function = C.THINKTHEN_FUNCTION_RELATE
	// FunctionRank identifies the rank function.
	FunctionRank Function = C.THINKTHEN_FUNCTION_RANK
	// FunctionAnnotate identifies the annotate function.
	FunctionAnnotate Function = C.THINKTHEN_FUNCTION_ANNOTATE
)

// String returns a readable representation.
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

// ErrNoAnswer matches uncertainty and failures read from results.
var ErrNoAnswer = errors.New("thinkthen: no answer")

// UnsureError reports an uncertain result's probability.
type UnsureError struct{ Probability float64 }

func (e *UnsureError) Error() string        { return "thinkthen: unsure" }
func (e *UnsureError) Is(target error) bool { return target == ErrUnsure || target == ErrNoAnswer }
