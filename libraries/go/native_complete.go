package thinkthen

/*
#cgo pkg-config: thinkthen
#include "thinkthen.h"
*/
import "C"
import (
	"context"
	"fmt"
)

func completeFailure(e *C.thinkthen_engine, rc C.int) error {
	// Snapshot while the caller still holds its pinned native thread.
	old := failure(e, rc).(*Error)
	var result *C.thinkthen_result
	if C.thinkthen_error_complete(e, &result) == 0 && result != nil {
		defer C.thinkthen_result_free(result)
		var s C.thinkthen_summary_v1
		if C.thinkthen_result_summary(result, &s) == 0 && s.error.present != 0 {
			v := s.error.value
			old.Complete = Some(CompleteError{Code: uint64(v.code), Message: nativeString(v.message), Retryable: v.retryable != 0, Stopped: nativeOptional(v.stopped.present, func() Stopped { return nativeStopped(v.stopped.value) }), Facts: nativeOptional(s.facts.present, func() CallFacts { return nativeCallFacts(s.facts.value) }), Attempts: nativeOptional(s.attempts.present, func() []Attempt { return nativeSlice(s.attempts.value.data, s.attempts.value.len, nativeAttempt) })})
		}
	}
	return old
}
func copyComplete[T any](result *C.thinkthen_result, read func(*C.thinkthen_result, C.size_t) (T, error)) (out CompleteCall[T], err error) {
	defer func() {
		if defect := recover(); defect != nil {
			out = CompleteCall[T]{}
			err = &Error{Code: 6, Kind: KindDefect, Message: fmt.Sprint(defect)}
		}
	}()
	var s C.thinkthen_summary_v1
	if rc := C.thinkthen_result_summary(result, &s); rc != 0 {
		return out, fmt.Errorf("native summary failed: %d", rc)
	}
	if s.error.present != 0 {
		v := s.error.value
		out.Error = Some(CompleteError{Code: uint64(v.code), Message: nativeString(v.message), Retryable: v.retryable != 0, Stopped: nativeOptional(v.stopped.present, func() Stopped { return nativeStopped(v.stopped.value) }), Facts: nativeOptional(s.facts.present, func() CallFacts { return nativeCallFacts(s.facts.value) }), Attempts: nativeOptional(s.attempts.present, func() []Attempt { return nativeSlice(s.attempts.value.data, s.attempts.value.len, nativeAttempt) })})
	}
	out.Schema = nativeString(s.schema)
	if s.function.present != 0 {
		out.Function = Some(nativeFunction(uint32(s.function.value)))
	}
	out.AnswerId = nativeOptional(s.answer_id.present, func() AnswerId { return AnswerId{nativeIdentifier(s.answer_id.value)} })
	out.Meta = nativeOptional(s.meta.present, func() Meta { return nativeMeta(s.meta.value) })
	out.Facts = nativeOptional(s.facts.present, func() CallFacts { return nativeCallFacts(s.facts.value) })
	out.Attempts = nativeOptional(s.attempts.present, func() []Attempt { return nativeSlice(s.attempts.value.data, s.attempts.value.len, nativeAttempt) })
	out.Rows = make([]T, int(s.count))
	for i := range out.Rows {
		row, e := read(result, C.size_t(i))
		if e != nil {
			return CompleteCall[T]{}, e
		}
		out.Rows[i] = row
	}
	out.Observations = make([]ObservationEvent, int(s.observation_count))
	for i := range out.Observations {
		var v C.thinkthen_observation_v1
		if rc := C.thinkthen_result_observation(result, C.size_t(i), &v); rc != 0 {
			return CompleteCall[T]{}, fmt.Errorf("native observation failed: %d", rc)
		}
		out.Observations[i] = nativeObservationEvent(v)
		var d C.thinkthen_details_v1
		var author C.thinkthen_question_author_v1
		if rc := C.thinkthen_result_observation_details(result, C.size_t(i), &d); rc != 0 {
			return CompleteCall[T]{}, fmt.Errorf("native observation details failed: %d", rc)
		}
		if rc := C.thinkthen_result_observation_author(result, C.size_t(i), &author); rc != 0 {
			return CompleteCall[T]{}, fmt.Errorf("native observation author failed: %d", rc)
		}
		out.Observations[i].Details = nativeDetails(d)
		out.Observations[i].Author = nativeQuestionAuthor(author)
	}
	return out, nil
}
func executeComplete[T any](e *Engine, ctx context.Context, q QuestionInput, source InputSource, controls CallControls, call func(*C.thinkthen_engine, *C.thinkthen_question, *C.thinkthen_source, *C.thinkthen_controls_v1, **C.thinkthen_result) C.int, read func(*C.thinkthen_result, C.size_t) (T, error)) (out CompleteCall[T], err error) {
	raw, token, ms, leave, err := e.enter(ctx)
	if err != nil {
		return out, err
	}
	defer leave()
	a := nativeInputs{}
	defer a.free()
	defer func() {
		if invalid := recover(); invalid != nil {
			out = CompleteCall[T]{}
			err = &Error{Code: 1, Kind: KindUsage, Message: fmt.Sprint(invalid)}
		}
	}()
	question, err := a.asked(raw, q)
	if err != nil {
		return out, err
	}
	if err = a.input(raw, source); err != nil {
		return out, err
	}
	c := C.thinkthen_controls_v1{deadline_ms: ms, cancel: token, context: a.optionalContent(controls.Context), batch: C.thinkthen_optional_size_v1{present: flag(controls.Batch.Present), value: C.size_t(controls.Batch.Value)}, batch_max: flag(controls.BatchMax), attempts: flag(controls.Attempts), surface: a.string("go")}
	var result *C.thinkthen_result
	rc := call(raw, question, a.source, &c, &result)
	if rc != 0 {
		return out, completeFailure(raw, rc)
	}
	defer C.thinkthen_result_free(result)
	return copyComplete(result, read)
}

var _ CompleteEngine = (*Engine)(nil)
