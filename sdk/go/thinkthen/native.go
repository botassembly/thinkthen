package thinkthen

/*
#cgo linux LDFLAGS: -ldl
#include <stdlib.h>
#include "loader.h"
*/
import "C"

import (
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"math"
	"os"
	"path/filepath"
	"runtime"
	"sync"
	"unsafe"
)

var loadOnce sync.Once
var loadError error

func loadNative() error {
	loadOnce.Do(func() {
		path := os.Getenv("THINKTHEN_LIBRARY")
		if path == "" && installedABI() != 0 {
			if message := C.tt_load(nil); message != nil {
				loadError = &Error{code: CodeUsageABI, message: C.GoString(message)}
			}
			return
		}
		if path == "" {
			var err error
			path, err = cachedLibrary()
			if err != nil {
				loadError = err
				return
			}
		}
		name := C.CString(path)
		defer C.free(unsafe.Pointer(name))
		if message := C.tt_load(name); message != nil {
			loadError = &Error{code: Code(C.tt_load_code()), message: C.GoString(message)}
			if C.tt_load_code() == C.THINKTHEN_CODE_LOCAL_IO {
				loadError = fmt.Errorf("cannot load native engine %s: %w", path, loadError)
			}
		}
	})
	return loadError
}

// Embedding leaves no package file at runtime, so retain one verified cache copy.
func cachedLibrary() (string, error) {
	base, err := os.UserCacheDir()
	if err != nil {
		return "", err
	}
	digest := sha256.Sum256(library)
	dir := filepath.Join(base, "thinkthen", "native", hex.EncodeToString(digest[:]))
	if err := os.MkdirAll(dir, 0700); err != nil {
		return "", err
	}
	for _, directory := range []string{base, filepath.Join(base, "thinkthen"), filepath.Join(base, "thinkthen", "native"), dir} {
		if err := checkCachePath(directory, true); err != nil {
			return "", err
		}
	}
	name := "library"
	if runtime.GOOS == "windows" {
		name += ".dll"
	}
	path := filepath.Join(dir, name)
	if err := checkCachePath(path, false); err != nil && !errors.Is(err, os.ErrNotExist) {
		return "", err
	}
	if data, err := os.ReadFile(path); err == nil {
		if sha256.Sum256(data) != digest {
			return "", errors.New("cached ThinkThen native engine checksum mismatch")
		}
		return path, nil
	} else if !errors.Is(err, os.ErrNotExist) {
		return "", err
	}
	file, err := os.CreateTemp(dir, ".library-")
	if err != nil {
		return "", err
	}
	defer os.Remove(file.Name())
	_, writeErr := file.Write(library)
	closeErr := file.Close()
	if writeErr != nil {
		return "", writeErr
	}
	if closeErr != nil {
		return "", closeErr
	}
	// Another process may have published the same immutable library first.
	if err := os.Rename(file.Name(), path); err != nil {
		data, readErr := os.ReadFile(path)
		if readErr != nil || sha256.Sum256(data) != digest {
			return "", err
		}
	}
	if err := checkCachePath(path, false); err != nil {
		return "", err
	}
	return path, nil
}

func bytesPointer(value []byte) *C.uint8_t {
	return (*C.uint8_t)(unsafe.Pointer(unsafe.SliceData(value)))
}
func copyBuffer(value C.thinkthen_buffer) []byte {
	return append([]byte(nil), unsafe.Slice((*byte)(unsafe.Pointer(value.data)), int(value.len))...)
}
func takeError(value *C.thinkthen_error) error {
	defer C.tt_error_free(value)
	name := C.tt_code_name(value.code)
	if name == nil {
		return abiError()
	}
	code, known := codeForName(C.GoString(name))
	if !known || code != Code(value.code) {
		return abiError()
	}
	return &Error{code: code, message: string(copyBuffer(value.message)), httpStatus: int(value.http_status)}
}

type nativeClient struct{ ptr *C.thinkthen_client }
type nativeCall struct{ ptr *C.thinkthen_call }

func (c nativeClient) free() { C.tt_client_free(c.ptr) }
func (c nativeCall) send(record wireRecord) error {
	e := C.thinkthen_error{struct_size: C.uint32_t(C.sizeof_thinkthen_error)}
	var status C.int32_t
	if record.invalid {
		status = C.tt_call_send_invalid(c.ptr, &e)
	} else if record.text {
		status = C.tt_call_send_text(c.ptr, bytesPointer(record.data), C.size_t(len(record.data)), &e)
	} else {
		status = C.tt_call_send_json(c.ptr, bytesPointer(record.data), C.size_t(len(record.data)), &e)
	}
	if status != C.THINKTHEN_OK {
		return takeError(&e)
	}
	return nil
}
func (c nativeCall) end()    { C.tt_call_end_input(c.ptr) }
func (c nativeCall) cancel() { C.tt_call_cancel(c.ptr) }
func (c nativeCall) free()   { C.tt_call_free(c.ptr) }
func (c nativeCall) next() (wireRow, bool, bool, error) {
	row := C.thinkthen_row{struct_size: C.uint32_t(C.sizeof_thinkthen_row)}
	e := C.thinkthen_error{struct_size: C.uint32_t(C.sizeof_thinkthen_error)}
	switch C.tt_call_next(c.ptr, &row, &e) {
	case C.THINKTHEN_NEEDS_INPUT:
		return wireRow{}, false, true, nil
	case C.THINKTHEN_END:
		return wireRow{}, false, false, nil
	case C.THINKTHEN_ERROR:
		return wireRow{}, false, false, takeError(&e)
	case C.THINKTHEN_OK:
		defer C.tt_row_free(&row)
		status := ""
		switch row.status {
		case C.THINKTHEN_STATUS_ANSWERED:
			status = "answered"
		case C.THINKTHEN_STATUS_UNSURE:
			status = "unsure"
		case C.THINKTHEN_STATUS_FAILED:
			status = "failed"
		default:
			return wireRow{}, false, false, &Error{code: CodeUsageABI, message: C.THINKTHEN_ABI_ERROR_MESSAGE}
		}
		switch row.function {
		case C.THINKTHEN_FUNCTION_DECIDE, C.THINKTHEN_FUNCTION_CHOOSE, C.THINKTHEN_FUNCTION_TAG, C.THINKTHEN_FUNCTION_SCORE, C.THINKTHEN_FUNCTION_FILTER, C.THINKTHEN_FUNCTION_RANK, C.THINKTHEN_FUNCTION_FIND, C.THINKTHEN_FUNCTION_RECOGNIZE, C.THINKTHEN_FUNCTION_RELATE, C.THINKTHEN_FUNCTION_ANNOTATE:
		default:
			return wireRow{}, false, false, &Error{code: CodeUsageABI, message: C.THINKTHEN_ABI_ERROR_MESSAGE}
		}
		result := wireRow{id: copyBuffer(row.id), answer: copyBuffer(row.answer), detail: copyBuffer(row.detail), source: copyBuffer(row.source), status: status, position: uint64(row.position), function: uint32(row.function), kept: row.function == C.THINKTHEN_FUNCTION_FILTER && row.status == C.THINKTHEN_STATUS_ANSWERED && row.answer_bool != 0}
		if !math.IsNaN(float64(row.probability)) {
			p := float64(row.probability)
			result.probability = &p
		}
		if row.error_code != 0 {
			var fields struct {
				Code       string `json:"code"`
				Message    string `json:"message"`
				HTTPStatus int    `json:"http_status"`
			}
			if err := json.Unmarshal(copyBuffer(row.error), &fields); err != nil {
				return wireRow{}, false, false, abiError()
			}
			name := C.tt_code_name(row.error_code)
			if name == nil || C.GoString(name) != fields.Code {
				return wireRow{}, false, false, &Error{code: CodeUsageABI, message: C.THINKTHEN_ABI_ERROR_MESSAGE}
			}
			code, known := codeForName(fields.Code)
			if !known || code != Code(row.error_code) {
				return wireRow{}, false, false, abiError()
			}
			result.failure = &Error{code: code, message: fields.Message, httpStatus: fields.HTTPStatus}
		}
		return result, true, false, nil
	default:
		return wireRow{}, false, false, &Error{code: CodeUsageABI, message: C.THINKTHEN_ABI_ERROR_MESSAGE}
	}
}

// conversionError asks the engine to own the host encoding failure.
func conversionError(settings bool) error {
	if err := loadNative(); err != nil {
		return err
	}
	e := C.thinkthen_error{struct_size: C.uint32_t(C.sizeof_thinkthen_error)}
	if settings {
		C.tt_settings_conversion_error(&e)
	} else {
		C.tt_question_conversion_error(&e)
	}
	return takeError(&e)
}
