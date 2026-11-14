//go:build cgo

package thinkthen

// MarshalJSON saves the canonical engine row with character offsets.
func (r Entry[A, D]) MarshalJSON() ([]byte, error) {
	if len(r.row.canonical) == 0 {
		return nil, abiError()
	}
	return append([]byte(nil), r.row.canonical...), nil
}
