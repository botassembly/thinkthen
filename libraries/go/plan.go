package thinkthen

/*
#include "thinkthen.h"
#include <stdlib.h>
*/
import "C"

import (
	"encoding/json"
	"runtime"
)

// Plan previews a canonical typed request without a key, cache reads or sends.
// Rust admits the supported questions and inputs. Unsupported or dynamic inputs
// retain native refusal errors. The result owns all of its bytes.
func (c *Client) Plan(request Request) (OwnedPlan, error) {
	var plan OwnedPlan
	data, err := json.Marshal(request)
	if err != nil {
		return plan, err
	}
	bytes := C.CBytes(data)
	defer C.free(bytes)
	c.engine.mu.RLock()
	defer c.engine.mu.RUnlock()
	if c.engine.raw == nil {
		return plan, ErrClosed
	}
	runtime.LockOSThread()
	defer runtime.UnlockOSThread()
	var result *C.char
	var length C.size_t
	err = sessionFailure(C.thinkthen_request_plan_json(c.engine.raw, (*C.char)(bytes), C.size_t(len(data)), &result, &length))
	if err != nil {
		return plan, err
	}
	defer C.thinkthen_free_string(result)
	raw, err := copyJSON(result, length)
	if err != nil {
		return plan, err
	}
	return ownedDecode[OwnedPlan](raw)
}
