package thinkthen

/*
#include "thinkthen.h"
#include <stdlib.h>
*/
import "C"
import (
	"context"
	"encoding/json"
	"errors"
	"reflect"
	"runtime"
	"sync"
)

// Client owns one native engine and the ten named judgment calls.
type Client struct {
	engine   *nativeEngine
	mu       sync.Mutex
	closed   bool
	sessions map[*C.thinkthen_session]activeSession
}

func NewClient(settings EngineSettings) (*Client, error) {
	data, err := json.Marshal(settings)
	if err != nil {
		return nil, err
	}
	engine, err := newNativeWith(string(data))
	if err != nil {
		return nil, err
	}
	return &Client{engine: engine, sessions: make(map[*C.thinkthen_session]activeSession)}, nil
}
func (c *Client) Close() {
	c.mu.Lock()
	c.closed = true
	var readers []<-chan struct{}
	for session, active := range c.sessions {
		active.cancel()
		C.thinkthen_session_cancel(session)
		if active.done != nil {
			readers = append(readers, active.done)
		}
	}
	c.mu.Unlock()
	c.engine.Close()
	for _, done := range readers {
		<-done
	}
}

type OwnedCall struct {
	Packets  []OwnedSessionPacket
	Terminal *OwnedSessionPacketTerminal
}
type SessionError struct {
	Call   OwnedCall
	Detail OwnedError
}

func (e *SessionError) Error() string { return e.Detail.Message().Value }
func sessionFailure(code C.int) error {
	if code == 0 {
		return nil
	}
	var facts json.RawMessage
	if borrowed := C.thinkthen_session_error_facts_json(); borrowed != nil {
		facts = json.RawMessage(C.GoString(borrowed))
	}
	return &Error{Code: int(code), Kind: ErrorKind(code), Message: C.GoString(C.thinkthen_session_error_message()), Retryable: C.thinkthen_session_error_retryable() != 0, Facts: facts}
}
func descriptor(value any) RequestItem {
	if item, ok := value.(RequestItem); ok {
		return item
	}
	if text, ok := value.(string); ok {
		return TextItem(text)
	}
	return JSONItem(value)
}
func (c *Client) call(ctx context.Context, verb string, question RequestQuestion, input any, options *RequestOptions) (OwnedCall, error) {
	var result OwnedCall
	if ctx == nil {
		ctx = context.Background()
	}
	if err := ctx.Err(); err != nil {
		return result, err
	}
	var source RequestInput
	producer, feeding := input.(Producer)
	if feeding {
		source = RequestInputFeed{Name: "records"}
	} else if explicit, ok := input.(RequestInput); ok {
		source = explicit
	} else if files, ok := input.(RequestSource); ok {
		source = RequestInputSource{Source: files}
	} else {
		values := []any{input}
		rv := reflect.ValueOf(input)
		if rv.IsValid() && (rv.Kind() == reflect.Slice || rv.Kind() == reflect.Array) {
			values = make([]any, rv.Len())
			for i := range values {
				values[i] = rv.Index(i).Interface()
			}
		}
		items := make([]RequestItem, len(values))
		for i, v := range values {
			items[i] = descriptor(v)
		}
		source = RequestInputRecords{Items: items}
	}
	if options == nil {
		options = &RequestOptions{}
	}
	var request C.thinkthen_named_request_v1
	views := []*C.thinkthen_complete_utf8_v1{&request.function, &request.question_json, &request.input_json, &request.options_json}
	for i, value := range []any{verb, question, source, options} {
		var data []byte
		var err error
		if i == 0 {
			data = []byte(verb)
		} else {
			data, err = json.Marshal(value)
		}
		if err != nil {
			return result, err
		}
		bytes := C.CBytes(data)
		defer C.free(bytes)
		*views[i] = C.thinkthen_complete_utf8_v1{data: (*C.char)(bytes), len: C.size_t(len(data))}
	}
	const surfaceToken = "go"
	surfaceBytes := C.CBytes([]byte(surfaceToken))
	defer C.free(surfaceBytes)
	surface := C.thinkthen_complete_utf8_v1{data: (*C.char)(surfaceBytes), len: C.size_t(len(surfaceToken))}
	c.engine.mu.RLock()
	if c.engine.raw == nil {
		c.engine.mu.RUnlock()
		return result, ErrClosed
	}
	runtime.LockOSThread()
	var session *C.thinkthen_session
	err := sessionFailure(C.thinkthen_session_new_named_with_surface(c.engine.raw, &request, surface, &session))
	runtime.UnlockOSThread()
	c.engine.mu.RUnlock()
	if err != nil {
		return result, err
	}
	readerCtx, stopReader := context.WithCancel(ctx)
	defer stopReader()
	c.mu.Lock()
	if c.closed {
		c.mu.Unlock()
		C.thinkthen_session_cancel(session)
		C.thinkthen_session_free(session)
		return result, ErrClosed
	}
	var feed *producerReader
	if feeding {
		feed = startProducer(readerCtx, producer)
	}
	active := activeSession{cancel: stopReader}
	if feed != nil {
		active.done = feed.done
	}
	c.sessions[session] = active
	c.mu.Unlock()
	defer func() {
		stopReader()
		if feed != nil {
			<-feed.done
		}
		c.mu.Lock()
		delete(c.sessions, session)
		C.thinkthen_session_free(session)
		c.mu.Unlock()
	}()
	if feed == nil {
		err = finishSession(session, nil)
		if err != nil {
			return result, err
		}
	}
	for {
		c.mu.Lock()
		closed := c.closed
		c.mu.Unlock()
		if closed {
			return result, ErrClosed
		}
		if err = ctx.Err(); err != nil {
			C.thinkthen_session_cancel(session)
			return result, err
		}
		runtime.LockOSThread()
		var status C.uint32_t
		var packet *C.thinkthen_session_result
		err = sessionFailure(C.thinkthen_session_try_read(session, &status, &packet))
		if err != nil {
			runtime.UnlockOSThread()
			return result, err
		}
		if status == 0 {
			var out *C.char
			var length C.size_t
			err = sessionFailure(C.thinkthen_session_result_json(packet, &out, &length))
			var raw json.RawMessage
			if err == nil {
				raw, err = copyJSON(out, length)
			}
			C.thinkthen_session_result_free(packet)
			runtime.UnlockOSThread()
			if err != nil {
				return result, err
			}
			value, decodeErr := ownedDecode[OwnedSessionPacket](raw)
			if decodeErr != nil {
				return result, decodeErr
			}
			result.Packets = append(result.Packets, value)
			packetKind := ownedMember[string](raw, "kind")
			if packetKind.Err != nil {
				return result, packetKind.Err
			}
			if packetKind.Value == "terminal" {
				terminal, decodeErr := ownedDecode[OwnedSessionPacketTerminal](raw)
				result.Terminal = &terminal
				err = decodeErr
				if err != nil {
					return result, err
				}
				failed := result.Terminal.Failure()
				if failed.Err != nil {
					return result, failed.Err
				}
				if failed.Present && !failed.Null {
					detail := failed.Value.Error()
					if detail.Err != nil {
						return result, detail.Err
					}
					return result, &SessionError{result, detail.Value}
				}
				return result, nil
			}
		} else {
			runtime.UnlockOSThread()
			if status == 2 {
				return result, errors.New("native session ended before terminal facts")
			}
			if feed != nil {
				if err = feed.advance(session, stopReader); err != nil {
					return result, err
				}
			}
			var next <-chan producerValue
			if feed != nil && !feed.finished && feed.pending == nil {
				next = feed.values
			}
			interest := C.uint32_t(C.THINKTHEN_SESSION_WAIT_RESULT_V1)
			if feed != nil && feed.pending != nil {
				interest = C.THINKTHEN_SESSION_WAIT_INPUT_V1
			}
			waited := make(chan error, 1)
			go func() {
				runtime.LockOSThread()
				defer runtime.UnlockOSThread()
				var ready C.uint32_t
				waited <- sessionFailure(C.thinkthen_session_wait(session, interest, 50, &ready))
			}()
			select {
			case <-ctx.Done():
				C.thinkthen_session_cancel(session)
				err = <-waited
			case value := <-next:
				feed.pending = &value
				err = <-waited
			case err = <-waited:
			}
			// Every path joins its native wait before the deferred free.
			if err != nil {
				return result, err
			}
		}
	}
}

// UsageStatus owns one native observation and remains valid after Close.
type UsageStatus struct {
	state  UsagePersistenceState
	advice Presence[string]
}

func (s UsageStatus) State() UsagePersistenceState { return s.state }
func (s UsageStatus) Advice() Presence[string]     { return s.advice }
func (e *nativeEngine) usageStatus(finish bool) (UsageStatus, error) {
	e.mu.RLock()
	defer e.mu.RUnlock()
	if e.raw == nil {
		return UsageStatus{}, ErrClosed
	}
	runtime.LockOSThread()
	defer runtime.UnlockOSThread()
	var state C.thinkthen_complete_usage_persistence_v1
	var advice C.thinkthen_complete_utf8_v1
	var code C.int
	if finish {
		code = C.thinkthen_engine_finish_usage_status_v1(e.raw, &state, &advice)
	} else {
		code = C.thinkthen_engine_usage_persistence_v1(e.raw, &state, &advice)
	}
	if code != 0 {
		return UsageStatus{}, sessionFailure(code)
	}
	out := UsageStatus{state: UsagePersistenceState(state.kind)}
	if advice.data != nil {
		text, err := copyCountedResult(advice.data, advice.len)
		if err != nil {
			return UsageStatus{}, err
		}
		out.advice = Presence[string]{Present: true, Value: text}
	}
	return out, nil
}
