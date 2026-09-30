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

// Engine may be called concurrently. Close waits for in-flight calls and must
// not race with the caller's own use of a returned Result, string, or Error.
// Those values contain only Go-owned data and remain valid after Close.
type Engine struct {
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
func copyJSON(result *C.char, length C.size_t) (json.RawMessage, error) {
	value, err := copyCountedResult(result, length)
	return json.RawMessage(value), err
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

func (e *Engine) Decide(ctx context.Context, question, evidence string) (Result[Answer], error) {
	q, err := checkedCString(question)
	if err != nil {
		return Result[Answer]{}, err
	}
	defer C.free(unsafe.Pointer(q))
	b := buffer(evidence)
	defer C.free(b)
	out := (*C.thinkthen_answer)(C.malloc(C.size_t(unsafe.Sizeof(C.thinkthen_answer{}))))
	if out == nil {
		return Result[Answer]{}, errors.New("cannot allocate answer")
	}
	defer C.free(unsafe.Pointer(out))
	raw, token, ms, leave, err := e.enter(ctx)
	if err != nil {
		return Result[Answer]{}, err
	}
	defer leave()
	var facts *C.char
	var factsLen C.size_t
	code := C.thinkthen_decide_with_facts_opts(raw, q, (*C.char)(b), C.size_t(len(evidence)), ms, token, out, &facts, &factsLen)
	defer C.thinkthen_free_string(facts)
	if code != 0 {
		return Result[Answer]{}, failure(raw, code)
	}
	value, err := asAnswer(*out)
	if err != nil {
		return Result[Answer]{}, err
	}
	owned, err := copyJSON(facts, factsLen)
	if err != nil {
		return Result[Answer]{}, err
	}
	return Result[Answer]{Value: value, Facts: owned}, nil
}
func (e *Engine) DecideMany(ctx context.Context, question string, evidence []string) (Result[[]Answer], error) {
	q, err := checkedCString(question)
	if err != nil {
		return Result[[]Answer]{}, err
	}
	defer C.free(unsafe.Pointer(q))
	if len(evidence) == 0 {
		raw, token, ms, leave, err := e.enter(ctx)
		if err != nil {
			return Result[[]Answer]{}, err
		}
		defer leave()
		var facts *C.char
		var factsLen C.size_t
		code := C.thinkthen_decide_many_with_facts_opts(raw, q, nil, nil, 0, ms, token, nil, &facts, &factsLen)
		defer C.thinkthen_free_string(facts)
		if code != 0 {
			return Result[[]Answer]{}, failure(raw, code)
		}
		owned, err := copyJSON(facts, factsLen)
		if err != nil {
			return Result[[]Answer]{}, err
		}
		return Result[[]Answer]{Value: []Answer{}, Facts: owned}, nil
	}
	n := len(evidence)
	width := unsafe.Sizeof((*C.char)(nil))
	if uintptr(n) > ^uintptr(0)/width || uintptr(n) > ^uintptr(0)/unsafe.Sizeof(C.size_t(0)) || uintptr(n) > ^uintptr(0)/unsafe.Sizeof(C.thinkthen_answer{}) {
		return Result[[]Answer]{}, errors.New("too many records")
	}
	ptrs := C.malloc(C.size_t(uintptr(n) * width))
	if ptrs == nil {
		return Result[[]Answer]{}, errors.New("cannot allocate pointers")
	}
	defer C.free(ptrs)
	sizes := C.malloc(C.size_t(uintptr(n) * unsafe.Sizeof(C.size_t(0))))
	if sizes == nil {
		return Result[[]Answer]{}, errors.New("cannot allocate lengths")
	}
	defer C.free(sizes)
	outs := C.malloc(C.size_t(uintptr(n) * unsafe.Sizeof(C.thinkthen_answer{})))
	if outs == nil {
		return Result[[]Answer]{}, errors.New("cannot allocate answers")
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
		return Result[[]Answer]{}, err
	}
	defer leave()
	var facts *C.char
	var factsLen C.size_t
	code := C.thinkthen_decide_many_with_facts_opts(raw, q, (**C.char)(ptrs), (*C.size_t)(sizes), C.size_t(n), ms, token, (*C.thinkthen_answer)(outs), &facts, &factsLen)
	defer C.thinkthen_free_string(facts)
	if code != 0 {
		return Result[[]Answer]{}, failure(raw, code)
	}
	values := make([]Answer, n)
	for i, a := range unsafe.Slice((*C.thinkthen_answer)(outs), n) {
		values[i], err = asAnswer(a)
		if err != nil {
			return Result[[]Answer]{}, err
		}
	}
	owned, err := copyJSON(facts, factsLen)
	if err != nil {
		return Result[[]Answer]{}, err
	}
	return Result[[]Answer]{Value: values, Facts: owned}, nil
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

// Plan previews a judgment call through thinkthen_plan_json and returns the
// result schema's plan object. verb is decide, choose, score or tag. question
// is bare question text, or one question object when it starts with "{", as
// Decide reads it. settings is nil or a thinkthen.settings/1 object. The
// preview needs no key, reads no cache and sends nothing.
func (e *Engine) Plan(verb, question string, input []string, settings json.RawMessage) (json.RawMessage, error) {
	asked, _ := json.Marshal(question) // a Go string always encodes
	if trimmed := strings.TrimSpace(question); strings.HasPrefix(trimmed, "{") {
		asked = []byte(trimmed)
	}
	if input == nil {
		input = []string{}
	}
	request, err := json.Marshal(struct {
		Verb     string          `json:"verb"`
		Question json.RawMessage `json:"question"`
		Input    []string        `json:"input"`
		Settings json.RawMessage `json:"settings,omitempty"`
	}{verb, asked, input, settings})
	if err != nil { // a question object or settings that is not JSON
		return nil, &Error{Code: int(KindUsage), Kind: KindUsage, Message: err.Error()}
	}
	q := C.CString(string(request)) // encoding/json escapes every NUL
	defer C.free(unsafe.Pointer(q))
	e.mu.RLock()
	defer e.mu.RUnlock()
	if e.raw == nil {
		return nil, ErrClosed
	}
	runtime.LockOSThread()
	defer runtime.UnlockOSThread()
	var out *C.char
	var length C.size_t
	if code := C.thinkthen_plan_json(e.raw, q, &out, &length); code != 0 {
		return nil, failure(e.raw, code)
	}
	defer C.thinkthen_free_string(out)
	return copyJSON(out, length)
}

// Recognize and Relate use their typed JSON-returning C entry points.
func (e *Engine) Recognize(ctx context.Context, spec, evidence string) (Result[json.RawMessage], error) {
	q, err := checkedCString(spec)
	if err != nil {
		return Result[json.RawMessage]{}, err
	}
	defer C.free(unsafe.Pointer(q))
	b := buffer(evidence)
	defer C.free(b)
	out := (*unsafe.Pointer)(C.malloc(C.size_t(unsafe.Sizeof(uintptr(0)))))
	if out == nil {
		return Result[json.RawMessage]{}, errors.New("cannot allocate result pointer")
	}
	defer C.free(unsafe.Pointer(out))
	*out = nil
	length := (*C.size_t)(C.malloc(C.size_t(unsafe.Sizeof(C.size_t(0)))))
	if length == nil {
		return Result[json.RawMessage]{}, errors.New("cannot allocate result length")
	}
	defer C.free(unsafe.Pointer(length))
	raw, token, ms, leave, err := e.enter(ctx)
	if err != nil {
		return Result[json.RawMessage]{}, err
	}
	defer leave()
	var facts *C.char
	var factsLen C.size_t
	code := C.thinkthen_recognize_with_facts_opts(raw, q, (*C.char)(b), C.size_t(len(evidence)), ms, token, (**C.char)(unsafe.Pointer(out)), length, &facts, &factsLen)
	defer C.thinkthen_free_string(facts)
	defer C.thinkthen_free_string((*C.char)(*out))
	if code != 0 {
		return Result[json.RawMessage]{}, failure(raw, code)
	}
	value, err := copyJSON((*C.char)(*out), *length)
	if err != nil {
		return Result[json.RawMessage]{}, err
	}
	owned, err := copyJSON(facts, factsLen)
	if err != nil {
		return Result[json.RawMessage]{}, err
	}
	return Result[json.RawMessage]{Value: value, Facts: owned}, nil
}
func (e *Engine) Relate(ctx context.Context, spec string, records []string) (Result[json.RawMessage], error) {
	q, err := checkedCString(spec)
	if err != nil {
		return Result[json.RawMessage]{}, err
	}
	defer C.free(unsafe.Pointer(q))
	n := len(records)
	width := unsafe.Sizeof((*C.char)(nil))
	if uintptr(n) > ^uintptr(0)/width || uintptr(n) > ^uintptr(0)/unsafe.Sizeof(C.size_t(0)) {
		return Result[json.RawMessage]{}, errors.New("too many records")
	}
	ptrs := C.malloc(C.size_t(uintptr(n) * width))
	if ptrs == nil && n > 0 {
		return Result[json.RawMessage]{}, errors.New("cannot allocate pointers")
	}
	defer C.free(ptrs)
	sizes := C.malloc(C.size_t(uintptr(n) * unsafe.Sizeof(C.size_t(0))))
	if sizes == nil && n > 0 {
		return Result[json.RawMessage]{}, errors.New("cannot allocate lengths")
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
		return Result[json.RawMessage]{}, errors.New("cannot allocate result pointer")
	}
	defer C.free(unsafe.Pointer(out))
	*out = nil
	length := (*C.size_t)(C.malloc(C.size_t(unsafe.Sizeof(C.size_t(0)))))
	if length == nil {
		return Result[json.RawMessage]{}, errors.New("cannot allocate result length")
	}
	defer C.free(unsafe.Pointer(length))
	raw, token, ms, leave, err := e.enter(ctx)
	if err != nil {
		return Result[json.RawMessage]{}, err
	}
	defer leave()
	var facts *C.char
	var factsLen C.size_t
	code := C.thinkthen_relate_with_facts_opts(raw, q, (**C.char)(ptrs), (*C.size_t)(sizes), C.size_t(n), ms, token, (**C.char)(unsafe.Pointer(out)), length, &facts, &factsLen)
	defer C.thinkthen_free_string(facts)
	defer C.thinkthen_free_string((*C.char)(*out))
	if code != 0 {
		return Result[json.RawMessage]{}, failure(raw, code)
	}
	value, err := copyJSON((*C.char)(*out), *length)
	if err != nil {
		return Result[json.RawMessage]{}, err
	}
	owned, err := copyJSON(facts, factsLen)
	if err != nil {
		return Result[json.RawMessage]{}, err
	}
	return Result[json.RawMessage]{Value: value, Facts: owned}, nil
}
