package thinkthen

/*
#cgo !linux pkg-config: thinkthen
#cgo linux,!amd64 pkg-config: thinkthen
#include "thinkthen.h"
*/
import "C"
import (
	"context"
	"fmt"
	"sync"
)

// CompleteBatch retains its engine until Close. A dedicated pinned goroutine
// owns start, next, facts and free on the C batch's creating thread. Callers
// may move between Go threads; returned rows contain only Go-owned memory.
type CompleteBatch[T any] struct {
	mu       sync.Mutex
	commands chan batchCommand[T]
	closed   bool
}
type batchReply[T any] struct {
	value *CompleteCall[T]
	err   error
}
type batchCommand[T any] struct {
	operation string
	reply     chan batchReply[T]
}

func (b *CompleteBatch[T]) send(operation string) (*CompleteCall[T], error) {
	b.mu.Lock()
	defer b.mu.Unlock()
	if b.closed {
		return nil, ErrClosed
	}
	reply := make(chan batchReply[T])
	b.commands <- batchCommand[T]{operation, reply}
	r := <-reply
	if operation == "close" {
		b.closed = true
	}
	return r.value, r.err
}

// Next returns nil at the end. An error retains the completed prefix already read.
func (b *CompleteBatch[T]) Next() (*CompleteCall[T], error)  { return b.send("next") }
func (b *CompleteBatch[T]) Facts() (*CompleteCall[T], error) { return b.send("facts") }
func (b *CompleteBatch[T]) Close() error {
	b.mu.Lock()
	if b.closed {
		b.mu.Unlock()
		return nil
	}
	b.mu.Unlock()
	_, err := b.send("close")
	return err
}
func startCompleteBatch[T any](e *Engine, ctx context.Context, q QuestionInput, source InputSource, controls CallControls, start func(*C.thinkthen_engine, *C.thinkthen_question, *C.thinkthen_source, *C.thinkthen_controls_v1, **C.thinkthen_batch) C.int, read func(*C.thinkthen_result, C.size_t) (T, error)) (*CompleteBatch[T], error) {
	b := &CompleteBatch[T]{commands: make(chan batchCommand[T])}
	ready := make(chan error)
	go func() {
		// Initialization failures are copied before leaving this native thread.
		raw, token, ms, leave, err := e.enter(ctx)
		if err != nil {
			ready <- err
			return
		}
		defer leave()
		a := nativeInputs{}
		defer a.free()
		var batch *C.thinkthen_batch
		err = func() (err error) {
			defer func() {
				if invalid := recover(); invalid != nil {
					err = &Error{Code: 1, Kind: KindUsage, Message: fmt.Sprint(invalid)}
				}
			}()
			question, err := a.asked(raw, q)
			if err != nil {
				return err
			}
			if err = a.input(raw, source); err != nil {
				return err
			}
			c := C.thinkthen_controls_v1{deadline_ms: ms, cancel: token, context: a.optionalContent(controls.Context), batch: C.thinkthen_optional_size_v1{present: flag(controls.Batch.Present), value: C.size_t(controls.Batch.Value)}, batch_max: flag(controls.BatchMax), attempts: flag(controls.Attempts), surface: a.string("go")}
			if rc := start(raw, question, a.source, &c, &batch); rc != 0 {
				return completeFailure(raw, rc)
			}
			return nil
		}()
		if err != nil {
			ready <- err
			return
		}
		defer func() { C.thinkthen_batch_free(batch) }()
		ready <- nil
		for command := range b.commands {
			if command.operation == "close" {
				C.thinkthen_batch_free(batch)
				batch = nil
				command.reply <- batchReply[T]{}
				return
			}
			var result *C.thinkthen_result
			var rc C.int
			if command.operation == "next" {
				rc = C.thinkthen_batch_next(batch, &result)
			} else {
				rc = C.thinkthen_batch_facts(batch, &result)
			}
			reply := batchReply[T]{}
			if rc != 0 {
				reply.err = completeFailure(raw, rc)
			} else if result != nil {
				value, err := copyComplete(result, read)
				C.thinkthen_result_free(result)
				reply.err = err
				if err == nil {
					reply.value = &value
				}
			}
			command.reply <- reply
		}
	}()
	if err := <-ready; err != nil {
		return nil, err
	}
	return b, nil
}
func (e *Engine) DecideBatch(ctx context.Context, q QuestionInput, source InputSource, controls CallControls) (*CompleteBatch[DecideRow], error) {
	return startCompleteBatch(e, ctx, q, source, controls, func(e *C.thinkthen_engine, q *C.thinkthen_question, s *C.thinkthen_source, c *C.thinkthen_controls_v1, b **C.thinkthen_batch) C.int {
		return C.thinkthen_decide_batch_start(e, q, s, c, b)
	}, readDecideRow)
}
func (e *Engine) ChooseBatch(ctx context.Context, q QuestionInput, source InputSource, controls CallControls) (*CompleteBatch[ChooseRow], error) {
	return startCompleteBatch(e, ctx, q, source, controls, func(e *C.thinkthen_engine, q *C.thinkthen_question, s *C.thinkthen_source, c *C.thinkthen_controls_v1, b **C.thinkthen_batch) C.int {
		return C.thinkthen_choose_batch_start(e, q, s, c, b)
	}, readChooseRow)
}
func (e *Engine) TagBatch(ctx context.Context, q QuestionInput, source InputSource, controls CallControls) (*CompleteBatch[TagRow], error) {
	return startCompleteBatch(e, ctx, q, source, controls, func(e *C.thinkthen_engine, q *C.thinkthen_question, s *C.thinkthen_source, c *C.thinkthen_controls_v1, b **C.thinkthen_batch) C.int {
		return C.thinkthen_tag_batch_start(e, q, s, c, b)
	}, readTagRow)
}
func (e *Engine) ScoreBatch(ctx context.Context, q QuestionInput, source InputSource, controls CallControls) (*CompleteBatch[ScoreRow], error) {
	return startCompleteBatch(e, ctx, q, source, controls, func(e *C.thinkthen_engine, q *C.thinkthen_question, s *C.thinkthen_source, c *C.thinkthen_controls_v1, b **C.thinkthen_batch) C.int {
		return C.thinkthen_score_batch_start(e, q, s, c, b)
	}, readScoreRow)
}
func (e *Engine) FilterBatch(ctx context.Context, q QuestionInput, source InputSource, controls CallControls) (*CompleteBatch[FilterRow], error) {
	return startCompleteBatch(e, ctx, q, source, controls, func(e *C.thinkthen_engine, q *C.thinkthen_question, s *C.thinkthen_source, c *C.thinkthen_controls_v1, b **C.thinkthen_batch) C.int {
		return C.thinkthen_filter_batch_start(e, q, s, c, b)
	}, readFilterRow)
}
func (e *Engine) AnnotateBatch(ctx context.Context, q QuestionInput, source InputSource, controls CallControls) (*CompleteBatch[AnnotateRow], error) {
	return startCompleteBatch(e, ctx, q, source, controls, func(e *C.thinkthen_engine, q *C.thinkthen_question, s *C.thinkthen_source, c *C.thinkthen_controls_v1, b **C.thinkthen_batch) C.int {
		return C.thinkthen_annotate_batch_start(e, q, s, c, b)
	}, readAnnotateRow)
}
