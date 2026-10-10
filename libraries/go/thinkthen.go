// Package thinkthen is a Go 1.22 cgo wrapper for the ThinkThen C door.
// The native engine owns all
// judgment and scheduling rules; this package owns only argument and result memory.
package thinkthen

/*
#cgo !linux pkg-config: thinkthen
#cgo linux,!amd64 pkg-config: thinkthen
#include <stdlib.h>
#include "thinkthen.h"
#if THINKTHEN_VERSION_MAJOR != 0 || THINKTHEN_VERSION_MINOR != 2 || THINKTHEN_VERSION_PATCH != 0
#error header does not match package ABI
#endif
*/
import "C"

import (
	"encoding/json"
	"errors"
	"fmt"
	"math"
	"runtime"
	"strings"
	"sync"
	"unsafe"
)

var errEmbeddedNUL = errors.New("NUL in a C-string argument")
var ErrClosed = errors.New("ThinkThen engine is closed")

// ErrorKind names a failure with the C door's codes 1 to 6.
type ErrorKind int

const (
	KindUsage ErrorKind = 1 + iota
	KindBackend
	KindDeadline
	KindLocal
	KindCancelled
	KindDefect
)

func (k ErrorKind) String() string {
	names := [...]string{"", "usage", "backend", "deadline", "local", "cancelled", "defect"}
	if k > 0 && int(k) < len(names) {
		return names[k]
	}
	return "defect"
}

type Error struct {
	Code      int
	Kind      ErrorKind
	Retryable bool
	Message   string
	Facts     json.RawMessage
}

func (e *Error) Error() string { return fmt.Sprintf("thinkthen code %d: %s", e.Code, e.Message) }

// nativeEngine is private; Client owns its lifetime and active sessions.
type nativeEngine struct {
	mu  sync.RWMutex
	raw *C.thinkthen_engine
}

// Every failed call and all error reads must stay on one native thread.
// Go may reschedule a goroutine between cgo calls. Copy the borrowed message
// before unlocking that thread, even when constructing a new engine.
func failure(raw *C.thinkthen_engine, code C.int) error {
	kind := KindDefect
	if code >= C.int(KindUsage) && code <= C.int(KindDefect) {
		kind = ErrorKind(code)
	}
	borrowed := C.thinkthen_error_facts_json(raw)
	var facts json.RawMessage
	if borrowed != nil {
		facts = json.RawMessage([]byte(C.GoString(borrowed)))
	}
	return &Error{Code: int(code), Kind: kind, Retryable: C.thinkthen_error_retryable(raw) != 0,
		Message: C.GoString(C.thinkthen_error_message(raw)), Facts: facts}
}

// The engine accepts only generated settings marshalled by NewClient.
func newNativeWith(settingsJSON string) (*nativeEngine, error) {
	settings, err := checkedCString(settingsJSON)
	if err != nil {
		return nil, err
	}
	defer C.free(unsafe.Pointer(settings))
	return newNativeEngine(settings)
}
func newNativeEngine(settings *C.char) (*nativeEngine, error) {
	runtime.LockOSThread()
	defer runtime.UnlockOSThread()
	var raw *C.thinkthen_engine
	if settings == nil {
		raw = C.thinkthen_engine_new()
	} else {
		raw = C.thinkthen_engine_new_with(settings)
	}
	if raw == nil {
		return nil, failure(nil, C.thinkthen_error_code(nil))
	}
	return &nativeEngine{raw: raw}, nil
}
func (e *nativeEngine) Close() {
	e.mu.Lock()
	defer e.mu.Unlock()
	if e.raw != nil {
		C.thinkthen_engine_free(e.raw)
		e.raw = nil
	}
}
func checkedCString(s string) (*C.char, error) {
	if strings.IndexByte(s, 0) >= 0 {
		return nil, errEmbeddedNUL
	}
	return C.CString(s), nil
}

// copyCountedResult refuses a C size_t that cannot fit GoStringN's C.int.
func copyCountedResult(result *C.char, length C.size_t) (string, error) {
	if uint64(length) > math.MaxInt32 {
		return "", errors.New("native result exceeds GoStringN length limit")
	}
	if result == nil && length != 0 {
		return "", errors.New("native result is nil with nonzero length")
	}
	return C.GoStringN(result, C.int(length)), nil
}
func copyJSON(result *C.char, length C.size_t) (json.RawMessage, error) {
	value, err := copyCountedResult(result, length)
	return json.RawMessage(value), err
}
