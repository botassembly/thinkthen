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
		path, err := cachedLibrary()
		if err != nil {
			loadError = err
			return
		}
		name := C.CString(path)
		defer C.free(unsafe.Pointer(name))
		if message := C.tt_load(name); message != nil {
			loadError = errors.New(C.GoString(message))
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
func takeError(value *C.thinkthen_buffer) error {
	defer C.tt_buffer_free(value)
	var fields struct {
		Code    string `json:"code"`
		Message string `json:"message"`
	}
	if err := json.Unmarshal(copyBuffer(*value), &fields); err != nil {
		return err
	}
	return &Error{code: fields.Code, message: fields.Message}
}

type nativeClient struct{ ptr *C.thinkthen_client }
type nativeCall struct{ ptr *C.thinkthen_call }

func openClient(data []byte) (nativeClient, error) {
	if err := loadNative(); err != nil {
		return nativeClient{}, err
	}
	var ptr *C.thinkthen_client
	var e C.thinkthen_buffer
	if C.tt_client_open(bytesPointer(data), C.size_t(len(data)), &ptr, &e) != C.THINKTHEN_OK {
		return nativeClient{}, takeError(&e)
	}
	return nativeClient{ptr}, nil
}
func (c nativeClient) free() { C.tt_client_free(c.ptr) }
func (c nativeClient) open(data []byte) (nativeCall, error) {
	var ptr *C.thinkthen_call
	var e C.thinkthen_buffer
	if C.tt_call_open(c.ptr, bytesPointer(data), C.size_t(len(data)), C.THINKTHEN_UTF8, &ptr, &e) != C.THINKTHEN_OK {
		return nativeCall{}, takeError(&e)
	}
	return nativeCall{ptr}, nil
}
func (c nativeCall) send(record wireRecord) error {
	var e C.thinkthen_buffer
	var status C.int32_t
	if record.text {
		status = C.tt_send_text(c.ptr, bytesPointer(record.data), C.size_t(len(record.data)), &e)
	} else {
		status = C.tt_send_json(c.ptr, bytesPointer(record.data), C.size_t(len(record.data)), &e)
	}
	if status != C.THINKTHEN_OK {
		return takeError(&e)
	}
	return nil
}
func (c nativeCall) end()    { C.tt_end_input(c.ptr) }
func (c nativeCall) cancel() { C.tt_cancel(c.ptr) }
func (c nativeCall) free()   { C.tt_free(c.ptr) }
func (c nativeCall) next() (Row, bool, bool, error) {
	var row C.thinkthen_row
	var e C.thinkthen_buffer
	switch C.tt_next(c.ptr, &row, &e) {
	case C.THINKTHEN_NEEDS_INPUT:
		return Row{}, false, true, nil
	case C.THINKTHEN_END:
		return Row{}, false, false, nil
	case C.THINKTHEN_ERROR:
		return Row{}, false, false, takeError(&e)
	default:
		defer C.tt_row_free(&row)
		result, err := decodeRow(copyBuffer(row.fields), copyBuffer(row.detail), int(row.position), row.kept != 0)
		return result, err == nil, false, err
	}
}
