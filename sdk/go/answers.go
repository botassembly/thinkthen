package thinkthen

// Span identifies the selected text using UTF-8 byte offsets.
type Span struct {
	Text  string `json:"text"`
	Start int    `json:"start"`
	End   int    `json:"end"`
}

// Name is a recognized span with its kind and support probability.
type Name struct {
	Text  string  `json:"text"`
	Start int     `json:"start"`
	End   int     `json:"end"`
	Kind  string  `json:"kind"`
	P     float64 `json:"p"`
}

// RelationName is one endpoint of a relation.
type RelationName struct {
	Text  string `json:"text"`
	Start int    `json:"start"`
	End   int    `json:"end"`
	Kind  string `json:"kind"`
}

// Relation is a labeled edge with its probability and optional direction flag.
type Relation struct {
	Source   RelationName `json:"source"`
	Target   RelationName `json:"target"`
	Relation string       `json:"relation"`
	P        float64      `json:"p"`
	Either   *bool        `json:"either,omitempty"`
}

// Result retains the immutable row and exposes its function's answer type.
// DecodeAnswer remains available for application-specific decoding.
type Result[A any] struct{ Row }

// Answer decodes the answer into independently owned typed storage. JSON null
// returns A's zero value; Status distinguishes answered, unsure and failed rows.
func (r Result[A]) Answer() (A, error) {
	var value A
	if r.Status() == "failed" {
		return value, r.Error()
	}
	err := r.DecodeAnswer(&value)
	return value, err
}

// Results provides the function's typed rows and the ordinary Rows lifecycle.
type Results[A any] struct{ *Rows }

// Row returns the latest immutable result with its function's answer type.
func (r *Results[A]) Row() Result[A] {
	if r.Rows == nil {
		return Result[A]{}
	}
	return Result[A]{Row: r.Rows.Row()}
}

// Next advances the iterator; a zero Results value is already complete.
func (r *Results[A]) Next() bool { return r.Rows != nil && r.Rows.Next() }

// Err returns the terminal error, or nil for a zero Results value.
func (r *Results[A]) Err() error {
	if r.Rows == nil {
		return nil
	}
	return r.Rows.Err()
}

// Close releases the call, or does nothing for a zero Results value.
func (r *Results[A]) Close() error {
	if r.Rows == nil {
		return nil
	}
	return r.Rows.Close()
}

func typed[A any](rows *Rows, err error) (*Results[A], error) {
	if err != nil {
		return nil, err
	}
	return &Results[A]{Rows: rows}, nil
}
