package thinkthen

/*
#include <stdlib.h>
#include "loader.h"
*/
import "C"
import "unsafe"

func openFinite(client *C.thinkthen_client, question *C.thinkthen_question, name *C.thinkthen_text, records []Record, out **C.thinkthen_call, e *C.thinkthen_error) C.int32_t {
	storage := C.calloc(C.size_t(len(records)), C.size_t(C.sizeof_thinkthen_record_input))
	defer C.free(storage)
	inputs := unsafe.Slice((*C.thinkthen_record_input)(storage), len(records))
	for i, record := range records {
		input := &inputs[i]
		input.struct_size = C.uint32_t(C.sizeof_thinkthen_record_input)
		switch record.kind {
		case 0:
			input.kind = C.THINKTHEN_RECORD_MISSING
		case 1:
			input.kind = C.THINKTHEN_RECORD_TEXT
		case 2:
			input.kind = C.THINKTHEN_RECORD_JSON
		}
		if record.kind != 0 {
			data := C.CBytes([]byte(record.data))
			defer C.free(data)
			input.data = (*C.uint8_t)(data)
			input.len = C.size_t(len(record.data))
		}
	}
	return C.tt_call_open_records_question(client, question, name, (*C.thinkthen_record_input)(storage), C.size_t(len(records)), C.size_t(C.sizeof_thinkthen_record_input), C.THINKTHEN_OFFSET_UTF8, out, e)
}
