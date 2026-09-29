// Package thinkthen is a Go 1.22 cgo wrapper for the ThinkThen C door.
// The native engine owns all
// judgment and scheduling rules; this package owns only argument and result memory.
package thinkthen

/*
#cgo pkg-config: thinkthen
#include <stdlib.h>
#include <pthread.h>
#include "thinkthen.h"
#if THINKTHEN_VERSION_MAJOR != 0 || THINKTHEN_VERSION_MINOR != 0 || THINKTHEN_VERSION_PATCH != 1
#error header does not match package ABI
#endif
static unsigned long native_thread(void) { return (unsigned long)pthread_self(); }
*/
import "C"

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"math"
	"runtime"
	"strings"
	"sync"
	"time"
	"unsafe"
)

var ErrEmbeddedNUL = errors.New("NUL in a C-string argument")
var ErrClosed = errors.New("ThinkThen engine is closed")

type Error struct {
	Code      int
	Kind      string
	Retryable bool
	Message   string
	Facts     json.RawMessage
}

func (e *Error) Error() string { return fmt.Sprintf("thinkthen code %d: %s", e.Code, e.Message) }

// Engine may be called concurrently. Close waits for in-flight calls and must
// not race with the caller's own use of a returned Answer, string, or Error.
// Those values contain only Go-owned data and remain valid after Close.
type Engine struct {
	mu  sync.RWMutex
	raw *C.thinkthen_engine
}

// Every failed call and all error reads must stay on one native thread.
// Go may reschedule a goroutine between cgo calls. Copy the borrowed message
// before unlocking that thread, even when constructing a new engine.
func failure(raw *C.thinkthen_engine, code C.int) error {
	kinds := [...]string{"", "usage", "backend", "deadline", "local", "cancelled", "defect"}
	kind := "defect"
	if code > 0 && int(code) < len(kinds) {
		kind = kinds[code]
	}
	borrowed := C.thinkthen_error_facts_json(raw)
	var facts json.RawMessage
	if borrowed != nil {
		facts = json.RawMessage([]byte(C.GoString(borrowed)))
	}
	return &Error{Code: int(code), Kind: kind, Retryable: C.thinkthen_error_retryable(raw) != 0,
		Message: C.GoString(C.thinkthen_error_message(raw)), Facts: facts}
}
func New() (*Engine, error) {
	return newEngine(nil)
}

// NewWith applies the same accepted settings JSON as the C constructor.
func NewWith(settingsJSON string) (*Engine, error) {
	settings, err := checkedCString(settingsJSON)
	if err != nil {
		return nil, err
	}
	defer C.free(unsafe.Pointer(settings))
	return newEngine(settings)
}
func newEngine(settings *C.char) (*Engine, error) {
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
	return &Engine{raw: raw}, nil
}
func (e *Engine) Close() {
	e.mu.Lock()
	defer e.mu.Unlock()
	if e.raw != nil {
		C.thinkthen_engine_free(e.raw)
		e.raw = nil
	}
}
func checkedCString(s string) (*C.char, error) {
	if strings.IndexByte(s, 0) >= 0 {
		return nil, ErrEmbeddedNUL
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
func buffer(s string) unsafe.Pointer {
	if len(s) == 0 {
		return nil
	}
	return C.CBytes([]byte(s)) // counted evidence, including NUL and invalid UTF-8
}
func budget(ctx context.Context) C.int64_t {
	end, ok := ctx.Deadline()
	if !ok {
		return C.int64_t(C.THINKTHEN_NO_DEADLINE)
	}
	left := time.Until(end)
	if left <= 0 {
		return 0
	}
	// C caps the budget at UINT32_MAX seconds. Round up to avoid spending
	// a positive submillisecond deadline before the native call starts.
	ms := float64(left) / float64(time.Millisecond)
	return C.int64_t(math.Min(math.Ceil(ms), 4294967295000))
}
func (e *Engine) enter(ctx context.Context) (*C.thinkthen_engine, *C.thinkthen_cancel_token, C.int64_t, func(), error) {
	if ctx == nil {
		ctx = context.Background()
	}
	e.mu.RLock()
	if e.raw == nil {
		e.mu.RUnlock()
		return nil, nil, 0, nil, ErrClosed
	}
	runtime.LockOSThread()
	token := C.thinkthen_cancel_token_new()
	if token == nil {
		runtime.UnlockOSThread()
		e.mu.RUnlock()
		return nil, nil, 0, nil, errors.New("cannot allocate cancellation token")
	}
	done := make(chan struct{})
	stopped := make(chan struct{})
	go func() {
		defer close(stopped)
		select {
		case <-ctx.Done():
			// Expiry spends deadline_ms and must report code 3. Only an
			// explicit cancellation fires the token (code 5). If cancel
			// wins at the deadline instant, the token wins by design.
			if ctx.Err() == context.Canceled {
				C.thinkthen_cancel(token)
			}
		case <-done:
		}
	}()
	leave := func() {
		close(done)
		<-stopped // never free token or engine while cancellation goroutine uses it
		C.thinkthen_cancel_token_free(token)
		runtime.UnlockOSThread()
		e.mu.RUnlock()
	}
	// A pre-expired deadline has budget zero, not a fired token.
	if ctx.Err() == context.Canceled {
		C.thinkthen_cancel(token)
	}
	return e.raw, token, budget(ctx), leave, nil
}

type Outcome int

const (
	No     Outcome = 0
	Yes    Outcome = 1
	Unsure Outcome = 2
)

type Answer struct {
	Outcome     Outcome
	Probability float64
}

func asAnswer(a C.thinkthen_answer) (Answer, error) {
	if a.outcome < 0 || a.outcome > 2 {
		return Answer{}, fmt.Errorf("invalid native outcome %d", a.outcome)
	}
	return Answer{Outcome: Outcome(a.outcome), Probability: float64(a.probability)}, nil
}

func (e *Engine) Decide(ctx context.Context, question, evidence string) (Answer, error) {
	q, err := checkedCString(question)
	if err != nil {
		return Answer{}, err
	}
	defer C.free(unsafe.Pointer(q))
	b := buffer(evidence)
	defer C.free(b)
	out := (*C.thinkthen_answer)(C.malloc(C.size_t(unsafe.Sizeof(C.thinkthen_answer{}))))
	if out == nil {
		return Answer{}, errors.New("cannot allocate answer")
	}
	defer C.free(unsafe.Pointer(out))
	raw, token, ms, leave, err := e.enter(ctx)
	if err != nil {
		return Answer{}, err
	}
	defer leave()
	code := C.thinkthen_decide_opts(raw, q, (*C.char)(b), C.size_t(len(evidence)), ms, token, out)
	if code != 0 {
		return Answer{}, failure(raw, code)
	}
	return asAnswer(*out)
}
func (e *Engine) DecideMany(ctx context.Context, question string, evidence []string) ([]Answer, error) {
	q, err := checkedCString(question)
	if err != nil {
		return nil, err
	}
	defer C.free(unsafe.Pointer(q))
	if len(evidence) == 0 {
		raw, token, ms, leave, err := e.enter(ctx)
		if err != nil {
			return nil, err
		}
		defer leave()
		code := C.thinkthen_decide_many_opts(raw, q, nil, nil, 0, ms, token, nil)
		if code != 0 {
			return nil, failure(raw, code)
		}
		return []Answer{}, nil
	}
	n := len(evidence)
	width := unsafe.Sizeof((*C.char)(nil))
	if uintptr(n) > ^uintptr(0)/width || uintptr(n) > ^uintptr(0)/unsafe.Sizeof(C.size_t(0)) || uintptr(n) > ^uintptr(0)/unsafe.Sizeof(C.thinkthen_answer{}) {
		return nil, errors.New("too many records")
	}
	ptrs := C.malloc(C.size_t(uintptr(n) * width))
	if ptrs == nil {
		return nil, errors.New("cannot allocate pointers")
	}
	defer C.free(ptrs)
	sizes := C.malloc(C.size_t(uintptr(n) * unsafe.Sizeof(C.size_t(0))))
	if sizes == nil {
		return nil, errors.New("cannot allocate lengths")
	}
	defer C.free(sizes)
	outs := C.malloc(C.size_t(uintptr(n) * unsafe.Sizeof(C.thinkthen_answer{})))
	if outs == nil {
		return nil, errors.New("cannot allocate answers")
	}
	defer C.free(outs)
	cptrs := unsafe.Slice((**C.char)(ptrs), n)
	lengths := unsafe.Slice((*C.size_t)(sizes), n)
	for i, s := range evidence {
		cptrs[i] = (*C.char)(buffer(s))
		lengths[i] = C.size_t(len(s))
	}
	defer func() {
		for _, p := range cptrs {
			C.free(unsafe.Pointer(p))
		}
	}()
	raw, token, ms, leave, err := e.enter(ctx)
	if err != nil {
		return nil, err
	}
	defer leave()
	code := C.thinkthen_decide_many_opts(raw, q, (**C.char)(ptrs), (*C.size_t)(sizes), C.size_t(n), ms, token, (*C.thinkthen_answer)(outs))
	if code != 0 {
		return nil, failure(raw, code)
	}
	values := make([]Answer, n)
	for i, a := range unsafe.Slice((*C.thinkthen_answer)(outs), n) {
		values[i], err = asAnswer(a)
		if err != nil {
			return nil, err
		}
	}
	return values, nil
}

// Call accepts the unchanged C JSON-door grammar. Successful JSON is a Go
// string; the borrowed C result is released before returning.
func (e *Engine) Call(ctx context.Context, request string) (string, error) {
	q, err := checkedCString(request)
	if err != nil {
		return "", err
	}
	defer C.free(unsafe.Pointer(q))
	raw, token, ms, leave, err := e.enter(ctx)
	if err != nil {
		return "", err
	}
	defer leave()
	result := C.thinkthen_call_opts(raw, q, ms, token)
	if result == nil {
		return "", failure(raw, C.thinkthen_error_code(raw))
	}
	defer C.thinkthen_free_string(result)
	return C.GoString(result), nil
}

// Recognize and Relate use their typed JSON-returning C entry points.
func (e *Engine) Recognize(ctx context.Context, spec, evidence string) (string, error) {
	q, err := checkedCString(spec)
	if err != nil {
		return "", err
	}
	defer C.free(unsafe.Pointer(q))
	b := buffer(evidence)
	defer C.free(b)
	out := (*unsafe.Pointer)(C.malloc(C.size_t(unsafe.Sizeof(uintptr(0)))))
	if out == nil {
		return "", errors.New("cannot allocate result pointer")
	}
	defer C.free(unsafe.Pointer(out))
	*out = nil
	length := (*C.size_t)(C.malloc(C.size_t(unsafe.Sizeof(C.size_t(0)))))
	if length == nil {
		return "", errors.New("cannot allocate result length")
	}
	defer C.free(unsafe.Pointer(length))
	raw, token, ms, leave, err := e.enter(ctx)
	if err != nil {
		return "", err
	}
	defer leave()
	code := C.thinkthen_recognize_opts(raw, q, (*C.char)(b), C.size_t(len(evidence)), ms, token, (**C.char)(unsafe.Pointer(out)), length)
	if code != 0 {
		return "", failure(raw, code)
	}
	result := (*C.char)(*out)
	defer C.thinkthen_free_string(result)
	return copyCountedResult(result, *length)
}
func (e *Engine) Relate(ctx context.Context, spec string, records []string) (string, error) {
	q, err := checkedCString(spec)
	if err != nil {
		return "", err
	}
	defer C.free(unsafe.Pointer(q))
	n := len(records)
	width := unsafe.Sizeof((*C.char)(nil))
	if uintptr(n) > ^uintptr(0)/width || uintptr(n) > ^uintptr(0)/unsafe.Sizeof(C.size_t(0)) {
		return "", errors.New("too many records")
	}
	ptrs := C.malloc(C.size_t(uintptr(n) * width))
	if ptrs == nil && n > 0 {
		return "", errors.New("cannot allocate pointers")
	}
	defer C.free(ptrs)
	sizes := C.malloc(C.size_t(uintptr(n) * unsafe.Sizeof(C.size_t(0))))
	if sizes == nil && n > 0 {
		return "", errors.New("cannot allocate lengths")
	}
	defer C.free(sizes)
	if n > 0 {
		cptrs := unsafe.Slice((**C.char)(ptrs), n)
		lengths := unsafe.Slice((*C.size_t)(sizes), n)
		for i, s := range records {
			cptrs[i] = (*C.char)(buffer(s))
			lengths[i] = C.size_t(len(s))
		}
		defer func() {
			for _, p := range cptrs {
				C.free(unsafe.Pointer(p))
			}
		}()
	}
	out := (*unsafe.Pointer)(C.malloc(C.size_t(unsafe.Sizeof(uintptr(0)))))
	if out == nil {
		return "", errors.New("cannot allocate result pointer")
	}
	defer C.free(unsafe.Pointer(out))
	*out = nil
	length := (*C.size_t)(C.malloc(C.size_t(unsafe.Sizeof(C.size_t(0)))))
	if length == nil {
		return "", errors.New("cannot allocate result length")
	}
	defer C.free(unsafe.Pointer(length))
	raw, token, ms, leave, err := e.enter(ctx)
	if err != nil {
		return "", err
	}
	defer leave()
	code := C.thinkthen_relate_opts(raw, q, (**C.char)(ptrs), (*C.size_t)(sizes), C.size_t(n), ms, token, (**C.char)(unsafe.Pointer(out)), length)
	if code != 0 {
		return "", failure(raw, code)
	}
	result := (*C.char)(*out)
	defer C.thinkthen_free_string(result)
	return copyCountedResult(result, *length)
}
