package thinkthen

import "encoding/json"

// MarshalJSON exports the engine's contract row, including its original ID.
func (r Row[A, D]) MarshalJSON() ([]byte, error) {
	type failure struct {
		Code       string `json:"code"`
		Message    string `json:"message"`
		HTTPStatus int    `json:"http_status,omitempty"`
	}
	var failed *failure
	if e := r.row.failure; e != nil {
		failed = &failure{e.code.String(), e.message, e.httpStatus}
	}
	return json.Marshal(struct {
		Function    string          `json:"function"`
		Position    uint64          `json:"position"`
		ID          json.RawMessage `json:"id,omitempty"`
		Source      json.RawMessage `json:"source"`
		Status      string          `json:"status"`
		Answer      json.RawMessage `json:"answer,omitempty"`
		Probability *float64        `json:"probability,omitempty"`
		Detail      json.RawMessage `json:"detail,omitempty"`
		Error       *failure        `json:"error,omitempty"`
	}{Function(r.row.function).String(), r.row.position, r.row.id, r.row.source,
		r.row.status, r.row.answer, r.row.probability, r.row.detail, failed})
}
