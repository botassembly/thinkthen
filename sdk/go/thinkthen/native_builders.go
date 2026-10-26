package thinkthen

/*
#include <stdlib.h>
#include "loader.h"
*/
import "C"

import (
	"time"
)

func nativeError() C.thinkthen_error {
	return C.thinkthen_error{struct_size: C.uint32_t(C.sizeof_thinkthen_error)}
}
func nativeResult(status C.int32_t, e *C.thinkthen_error) error {
	if status != C.THINKTHEN_OK {
		return takeError(e)
	}
	return nil
}
func newNative(cfg Config) (nativeClient, error) {
	if err := loadNative(); err != nil {
		return nativeClient{}, err
	}
	var settings *C.thinkthen_settings
	e := nativeError()
	if err := nativeResult(C.tt_settings_new(&settings, &e), &e); err != nil {
		return nativeClient{}, err
	}
	defer C.tt_settings_free(settings)
	if err := nativeResult(C.tt_settings_set_backend(settings, C.uint32_t(normalizedBackend(cfg.Backend)), &e), &e); err != nil {
		return nativeClient{}, err
	}
	for _, v := range []struct {
		kind int
		text string
	}{{0, cfg.Model}, {1, cfg.URL}, {2, cfg.APIKey.secret}} {
		if v.text == "" && (v.kind != 2 || !cfg.APIKey.present) {
			continue
		}
		data := []byte(v.text)
		var status C.int32_t
		switch v.kind {
		case 0:
			status = C.tt_settings_set_model(settings, bytesPointer(data), C.size_t(len(data)), &e)
		case 1:
			status = C.tt_settings_set_url(settings, bytesPointer(data), C.size_t(len(data)), &e)
		case 2:
			status = C.tt_settings_set_api_key(settings, bytesPointer(data), C.size_t(len(data)), &e)
		}
		if err := nativeResult(status, &e); err != nil {
			return nativeClient{}, err
		}
	}
	if cfg.Batch != nil {
		if *cfg.Batch < 0 {
			return nativeClient{}, conversionError(true)
		}
		if err := nativeResult(C.tt_settings_set_batch(settings, C.uint64_t(*cfg.Batch), &e), &e); err != nil {
			return nativeClient{}, err
		}
	}
	if r := cfg.Retry; r != nil {
		if r.MaxRetries < 0 || uint64(r.MaxRetries) > uint64(^uint32(0)) || r.InitialBackoff < 0 || r.AttemptTimeout < 0 || r.InitialBackoff%time.Millisecond != 0 || r.AttemptTimeout%time.Millisecond != 0 {
			return nativeClient{}, conversionError(true)
		}
		if err := nativeResult(C.tt_settings_set_retry(settings, C.uint32_t(r.MaxRetries), C.uint64_t(r.InitialBackoff/time.Millisecond), C.uint64_t(r.AttemptTimeout/time.Millisecond), &e), &e); err != nil {
			return nativeClient{}, err
		}
	}
	var ptr *C.thinkthen_client
	if err := nativeResult(C.tt_client_open(settings, &ptr, &e), &e); err != nil {
		return nativeClient{}, err
	}
	return nativeClient{ptr}, nil
}
func buildQuestion(q question) (*C.thinkthen_question, error) {
	if q.invalid {
		return nil, conversionError(false)
	}
	data := []byte(q.text)
	e := nativeError()
	var ptr *C.thinkthen_question
	if err := nativeResult(C.tt_question_new(C.uint32_t(q.function), bytesPointer(data), C.size_t(len(data)), &ptr, &e), &e); err != nil {
		return nil, err
	}
	success := false
	defer func() {
		if !success {
			C.tt_question_free(ptr)
		}
	}()
	for _, s := range q.domain {
		data := []byte(s)
		var status C.int32_t
		switch q.function {
		case FunctionChoose:
			status = C.tt_question_add_choice(ptr, bytesPointer(data), C.size_t(len(data)), &e)
		case FunctionTag:
			status = C.tt_question_add_label(ptr, bytesPointer(data), C.size_t(len(data)), &e)
		case FunctionScore, FunctionRank:
			status = C.tt_question_add_level(ptr, bytesPointer(data), C.size_t(len(data)), &e)
		case FunctionRecognize:
			status = C.tt_question_add_kind(ptr, bytesPointer(data), C.size_t(len(data)), &e)
		}
		if err := nativeResult(status, &e); err != nil {
			return nil, err
		}
	}
	for _, rule := range q.rules {
		if err := addRule(ptr, rule); err != nil {
			return nil, err
		}
	}
	switch q.threshold.kind {
	case 1:
		if err := nativeResult(C.tt_question_set_cut(ptr, C.double(q.threshold.low), &e), &e); err != nil {
			return nil, err
		}
	case 2:
		if err := nativeResult(C.tt_question_set_band(ptr, C.double(q.threshold.low), C.double(q.threshold.high), &e), &e); err != nil {
			return nil, err
		}
	case 3:
		return nil, conversionError(false)
	}
	if q.evidence != nil {
		data := []byte(*q.evidence)
		if err := nativeResult(C.tt_question_set_evidence(ptr, bytesPointer(data), C.size_t(len(data)), &e), &e); err != nil {
			return nil, err
		}
	}
	if q.top != nil {
		if *q.top < 0 {
			return nil, conversionError(false)
		}
		if err := nativeResult(C.tt_question_set_top(ptr, C.size_t(*q.top), &e), &e); err != nil {
			return nil, err
		}
	}
	for _, name := range sortedQuestions(q.children) {
		child, err := buildQuestion(q.children[name])
		if err != nil {
			return nil, err
		}
		data := []byte(name)
		status := C.tt_question_add_entry(ptr, bytesPointer(data), C.size_t(len(data)), child, &e)
		C.tt_question_free(child)
		if err = nativeResult(status, &e); err != nil {
			return nil, err
		}
	}
	success = true
	return ptr, nil
}
func ownedText(s string) (C.thinkthen_text, func()) {
	data := C.CBytes(append([]byte(s), 0))
	return C.thinkthen_text{data: (*C.uint8_t)(data), len: C.size_t(len(s))}, func() { C.free(data) }
}
func addRule(q *C.thinkthen_question, rule Rule) error {
	name, f1 := ownedText(rule.Name)
	defer f1()
	source, f2 := ownedText(rule.Source)
	defer f2()
	target, f3 := ownedText(rule.Target)
	defer f3()
	r := C.thinkthen_rule{relation: name, source: source, target: target}
	if rule.Either {
		r.either = 1
	}
	if rule.Reads != nil {
		reads, f := ownedText(*rule.Reads)
		defer f()
		r.reads = reads
	}
	e := nativeError()
	return nativeResult(C.tt_question_add_rule(q, &r, &e), &e)
}
func (c nativeClient) open(q question, input Input) (nativeCall, error) {
	builder, err := buildQuestion(q)
	if err != nil {
		return nativeCall{}, err
	}
	defer C.tt_question_free(builder)
	var name *C.thinkthen_text
	if q.name != nil {
		value, f := ownedText(*q.name)
		defer f()
		name = &value
	}
	var ptr *C.thinkthen_call
	e := nativeError()
	var status C.int32_t
	if input.finite {
		status = openFinite(c.ptr, builder, name, input.records, &ptr, &e)
	} else {
		status = C.tt_call_open_question(c.ptr, builder, name, C.THINKTHEN_OFFSET_UTF8, &ptr, &e)
	}
	if err = nativeResult(status, &e); err != nil {
		return nativeCall{}, err
	}
	return nativeCall{ptr}, nil
}
