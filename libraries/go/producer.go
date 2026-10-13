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
type producerReader struct {
	done chan struct{}
	err  error
}

func startProducer(ctx context.Context, producer Producer, session *C.thinkthen_session) *producerReader {
	reader := &producerReader{done: make(chan struct{})}
	go func() {
		defer close(reader.done)
		for ctx.Err() == nil {
			value, err := producer.Next(ctx)
			if ctx.Err() != nil {
				return
			}
			var data []byte
			failureKind := "io"
			if err == nil {
				data, err = json.Marshal(RequestSessionDescriptor{Item: descriptor(value)})
				failureKind = "invalid_input"
			}
			if err != nil {
				var failure []byte
				if !errors.Is(err, io.EOF) {
					failure = []byte(`{"kind":"` + failureKind + `"}`)
				}
				reader.err = finishSession(session, failure)
				if reader.err != nil {
					C.thinkthen_session_cancel(session)
				}
				return
			}
			accepted, err := pushDescriptor(session, data)
			if err != nil {
				reader.err = err
				C.thinkthen_session_cancel(session)
				return
			}
			if !accepted {
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

// Native backpressure retains this descriptor until accepted or closed.
func pushDescriptor(session *C.thinkthen_session, data []byte) (bool, error) {
	bytes := C.CBytes(data)
	defer C.free(bytes)
	runtime.LockOSThread()
	defer runtime.UnlockOSThread()
	var status C.uint32_t
	err := sessionFailure(C.thinkthen_session_push(session, (*C.char)(bytes), C.size_t(len(data)), &status))
	return status == C.THINKTHEN_SESSION_ACCEPTED_V1, err
}
