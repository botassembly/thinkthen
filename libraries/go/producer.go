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
	"io"
	"runtime"
	"unsafe"
)

// Producer supplies ordered values or Items without collecting the input.
// Next must return promptly when ctx is cancelled. Return io.EOF to finish.
// The client calls Next serially and joins its reader before the call returns.
type Producer interface {
	Next(ctx context.Context) (any, error)
}

type activeSession struct {
	cancel context.CancelFunc
	done   <-chan struct{}
}
type producerValue struct {
	data    []byte
	err     error
	failure string
}
type producerReader struct {
	values   chan producerValue
	demand   chan struct{}
	done     chan struct{}
	pending  *producerValue
	finished bool
}

func startProducer(ctx context.Context, producer Producer) *producerReader {
	reader := &producerReader{values: make(chan producerValue), demand: make(chan struct{}, 1), done: make(chan struct{})}
	reader.demand <- struct{}{}
	go func() {
		defer close(reader.done)
		for {
			select {
			case <-ctx.Done():
				return
			case <-reader.demand:
			}
			if ctx.Err() != nil {
				return
			}
			value, err := producer.Next(ctx)
			item := producerValue{err: err, failure: "io"}
			if err == nil {
				item.data, item.err = json.Marshal(map[string]any{"item": descriptor(value)})
				item.failure = "invalid_input"
			}
			select {
			case <-ctx.Done():
				return
			case reader.values <- item:
			}
			if item.err != nil {
				return
			}
		}
	}()
	return reader
}

func finishSession(session *C.thinkthen_session, failure []byte) error {
	var bytes *C.char
	if failure != nil {
		bytes = (*C.char)(C.CBytes(failure))
		defer C.free(unsafe.Pointer(bytes))
	}
	runtime.LockOSThread()
	defer runtime.UnlockOSThread()
	return sessionFailure(C.thinkthen_session_finish(session, bytes, C.size_t(len(failure))))
}

// advance retains exactly the same descriptor on FULL. The call loop drains
// output before retrying, including while Next is waiting for another value.
func (r *producerReader) advance(session *C.thinkthen_session, stop context.CancelFunc) error {
	if r.finished || r.pending == nil {
		return nil
	}
	item := r.pending
	if item.err != nil {
		var failure []byte
		if !errors.Is(item.err, io.EOF) {
			failure = []byte(`{"kind":"` + item.failure + `"}`)
		}
		r.finished = true
		r.pending = nil
		return finishSession(session, failure)
	}
	bytes := C.CBytes(item.data)
	defer C.free(bytes)
	runtime.LockOSThread()
	var status C.uint32_t
	err := sessionFailure(C.thinkthen_session_try_push(session, (*C.char)(bytes), C.size_t(len(item.data)), &status))
	runtime.UnlockOSThread()
	if err != nil {
		return err
	}
	switch status {
	case C.THINKTHEN_SESSION_ACCEPTED_V1:
		r.pending = nil
		r.demand <- struct{}{}
	case C.THINKTHEN_SESSION_CLOSED_V1:
		r.finished = true
		r.pending = nil
		stop()
	}
	return nil
}
