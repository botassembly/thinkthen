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

// Client is the canonical named-call API; Engine retains compatibility calls.
type Client struct {
	engine   *Engine
	mu       sync.Mutex
	closed   bool
	sessions map[*C.thinkthen_session]struct{}
}

func NewClient(settings map[string]any) (*Client, error) {
	data, err := json.Marshal(settings)
	if err != nil {
		return nil, err
	}
	engine, err := NewWith(string(data))
	if err != nil {
		return nil, err
	}
	return &Client{engine: engine, sessions: make(map[*C.thinkthen_session]struct{})}, nil
}
func (c *Client) Close() {
	c.mu.Lock()
	defer c.mu.Unlock()
	c.closed = true
	for session := range c.sessions {
		C.thinkthen_session_cancel(session)
	}
	c.engine.Close()
}

// FileRecords selects the native reader; its parsing and validation stay native.
type FileRecords struct {
	Paths   []string
	Reading map[string]any
}

// Item supplies per-record context/options separately from its original value.
type Item struct {
	Original any
	Fields   map[string]any
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
func descriptor(value any) map[string]any {
	fields := map[string]any{}
	if item, ok := value.(Item); ok {
		value = item.Original
		for k, v := range item.Fields {
			fields[k] = v
		}
	}
	original := map[string]any{"kind": "json", "value": value}
	if text, ok := value.(string); ok {
		original = map[string]any{"kind": "text", "text": text}
	}
	fields["original"] = original
	return fields
}
func (c *Client) call(ctx context.Context, verb string, question, input any, options map[string]any) (OwnedCall, error) {
	var result OwnedCall
	if ctx == nil {
		ctx = context.Background()
	}
	if err := ctx.Err(); err != nil {
		return result, err
	}
	asked := map[string]any{"kind": "definition", "value": question}
	if text, ok := question.(string); ok {
		asked = map[string]any{"kind": "text", "text": text}
	}
	kind := "records"
	if verb == "find" {
		kind = "units"
	}
	if verb == "relate" {
		kind = "entities"
	}
	source := map[string]any{"kind": kind}
	if files, ok := input.(FileRecords); ok {
		source = map[string]any{"kind": "source", "source": map[string]any{"paths": files.Paths, "reading": files.Reading, "media": "text"}}
	} else {
		values := []any{input}
		rv := reflect.ValueOf(input)
		if rv.IsValid() && (rv.Kind() == reflect.Slice || rv.Kind() == reflect.Array) {
			values = make([]any, rv.Len())
			for i := range values {
				values[i] = rv.Index(i).Interface()
			}
		}
		items := make([]map[string]any, len(values))
		for i, v := range values {
			items[i] = descriptor(v)
		}
		source["items"] = items
	}
	if options == nil {
		options = map[string]any{}
	}
	request := map[string]any{"schema": OwnedRequestVersion, "call": map[string]any{"function": verb, "question": asked, "input": source, "options": options}}
	data, err := json.Marshal(request)
	if err != nil {
		return result, err
	}
	bytes := C.CBytes(data)
	defer C.free(bytes)
	c.engine.mu.RLock()
	if c.engine.raw == nil {
		c.engine.mu.RUnlock()
		return result, ErrClosed
	}
	runtime.LockOSThread()
	var session *C.thinkthen_session
	err = sessionFailure(C.thinkthen_session_new(c.engine.raw, (*C.char)(bytes), C.size_t(len(data)), &session))
	runtime.UnlockOSThread()
	c.engine.mu.RUnlock()
	if err != nil {
		return result, err
	}
	c.mu.Lock()
	if c.closed {
		c.mu.Unlock()
		C.thinkthen_session_cancel(session)
		C.thinkthen_session_free(session)
		return result, ErrClosed
	}
	c.sessions[session] = struct{}{}
	c.mu.Unlock()
	defer func() { c.mu.Lock(); delete(c.sessions, session); C.thinkthen_session_free(session); c.mu.Unlock() }()
	runtime.LockOSThread()
	err = sessionFailure(C.thinkthen_session_finish(session, nil, 0))
	runtime.UnlockOSThread()
	if err != nil {
		return result, err
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
			select {
			case <-ctx.Done():
			case <-time.After(time.Millisecond):
			}
		}
	}
}
func (c *Client) Decide(ctx context.Context, question, input any, options map[string]any) (OwnedCall, error) {
	return c.call(ctx, "decide", question, input, options)
}
func (c *Client) Choose(ctx context.Context, question, input any, options map[string]any) (OwnedCall, error) {
	return c.call(ctx, "choose", question, input, options)
}
func (c *Client) Tag(ctx context.Context, question, input any, options map[string]any) (OwnedCall, error) {
	return c.call(ctx, "tag", question, input, options)
}
func (c *Client) Score(ctx context.Context, question, input any, options map[string]any) (OwnedCall, error) {
	return c.call(ctx, "score", question, input, options)
}
func (c *Client) Filter(ctx context.Context, question, input any, options map[string]any) (OwnedCall, error) {
	return c.call(ctx, "filter", question, input, options)
}
func (c *Client) Rank(ctx context.Context, question, input any, options map[string]any) (OwnedCall, error) {
	return c.call(ctx, "rank", question, input, options)
}
func (c *Client) Find(ctx context.Context, question, input any, options map[string]any) (OwnedCall, error) {
	return c.call(ctx, "find", question, input, options)
}
func (c *Client) Annotate(ctx context.Context, question, input any, options map[string]any) (OwnedCall, error) {
	return c.call(ctx, "annotate", question, input, options)
}
func (c *Client) Recognize(ctx context.Context, question, input any, options map[string]any) (OwnedCall, error) {
	return c.call(ctx, "recognize", question, input, options)
}
func (c *Client) Relate(ctx context.Context, question, input any, options map[string]any) (OwnedCall, error) {
	return c.call(ctx, "relate", question, input, options)
}
