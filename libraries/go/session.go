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
	"time"
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
	return &Error{Code: int(code), Kind: ErrorKind(code), Message: C.GoString(C.thinkthen_session_error_message())}
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
		switch verb {
		case "find":
			source = RequestInputUnits{Items: items}
		case "relate":
			source = RequestInputEntities{Items: items}
		default:
			source = RequestInputRecords{Items: items}
		}
	}
	var call RequestCall
	switch verb {
	case "decide":
		call = RequestCallDecide{Input: source, Options: options, Question: question}
	case "choose":
		call = RequestCallChoose{Input: source, Options: options, Question: question}
	case "tag":
		call = RequestCallTag{Input: source, Options: options, Question: question}
	case "score":
		call = RequestCallScore{Input: source, Options: options, Question: question}
	case "filter":
		call = RequestCallFilter{Input: source, Options: options, Question: question}
	case "rank":
		call = RequestCallRank{Input: source, Options: options, Question: question}
	case "find":
		call = RequestCallFind{Input: source, Options: options, Question: question}
	case "annotate":
		call = RequestCallAnnotate{Input: source, Options: options, Question: question}
	case "recognize":
		call = RequestCallRecognize{Input: source, Options: options, Question: question}
	case "relate":
		call = RequestCallRelate{Input: source, Options: options, Question: question}
	}
	request := Request{Call: call}
	data, err := json.Marshal(request)
	if err != nil {
		return result, err
	}
	bytes := C.CBytes(data)
	defer C.free(bytes)
	const surfaceToken = "go"
	surface := C.CBytes([]byte(surfaceToken))
	defer C.free(surface)
	c.engine.mu.RLock()
	if c.engine.raw == nil {
		c.engine.mu.RUnlock()
		return result, ErrClosed
	}
	runtime.LockOSThread()
	var session *C.thinkthen_session
	err = sessionFailure(C.thinkthen_session_new_with_surface(c.engine.raw, (*C.char)(bytes), C.size_t(len(data)), (*C.char)(surface), C.size_t(len(surfaceToken)), &session))
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
	ticker := time.NewTicker(time.Millisecond)
	defer ticker.Stop()
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
			select {
			case <-ctx.Done():
			case value := <-next:
				feed.pending = &value
			case <-ticker.C:
			}
		}
	}
}
func (c *Client) Decide(ctx context.Context, question RequestQuestion, input any, options *RequestOptions) (OwnedCall, error) {
	return c.call(ctx, "decide", question, input, options)
}
func (c *Client) Choose(ctx context.Context, question RequestQuestion, input any, options *RequestOptions) (OwnedCall, error) {
	return c.call(ctx, "choose", question, input, options)
}
func (c *Client) Tag(ctx context.Context, question RequestQuestion, input any, options *RequestOptions) (OwnedCall, error) {
	return c.call(ctx, "tag", question, input, options)
}
func (c *Client) Score(ctx context.Context, question RequestQuestion, input any, options *RequestOptions) (OwnedCall, error) {
	return c.call(ctx, "score", question, input, options)
}
func (c *Client) Filter(ctx context.Context, question RequestQuestion, input any, options *RequestOptions) (OwnedCall, error) {
	return c.call(ctx, "filter", question, input, options)
}
func (c *Client) Rank(ctx context.Context, question RequestQuestion, input any, options *RequestOptions) (OwnedCall, error) {
	return c.call(ctx, "rank", question, input, options)
}
func (c *Client) Find(ctx context.Context, question RequestQuestion, input any, options *RequestOptions) (OwnedCall, error) {
	return c.call(ctx, "find", question, input, options)
}
func (c *Client) Annotate(ctx context.Context, question RequestQuestion, input any, options *RequestOptions) (OwnedCall, error) {
	return c.call(ctx, "annotate", question, input, options)
}
func (c *Client) Recognize(ctx context.Context, question RequestQuestion, input any, options *RequestOptions) (OwnedCall, error) {
	return c.call(ctx, "recognize", question, input, options)
}
func (c *Client) Relate(ctx context.Context, question RequestQuestion, input any, options *RequestOptions) (OwnedCall, error) {
	return c.call(ctx, "relate", question, input, options)
}

// UsageStatus owns one native observation and remains valid after Close.
type UsageStatus struct {
	state  UsagePersistenceState
	advice Presence[string]
}

func (s UsageStatus) State() UsagePersistenceState              { return s.state }
func (s UsageStatus) Advice() Presence[string]                  { return s.advice }
func (c *Client) UsagePersistence() (UsageStatus, error)        { return c.engine.UsagePersistence() }
func (c *Client) FinishUsageStatus() (UsageStatus, error)       { return c.engine.FinishUsageStatus() }
func (e *nativeEngine) UsagePersistence() (UsageStatus, error)  { return e.usageStatus(false) }
func (e *nativeEngine) FinishUsageStatus() (UsageStatus, error) { return e.usageStatus(true) }
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
