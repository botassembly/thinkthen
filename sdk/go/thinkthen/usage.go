//go:build cgo

package thinkthen

// TokenUsage retains reported tokens separately from unavailable finished attempts.
// Reported is nil after overflow; a known zero remains a non-nil zero.
type TokenUsage struct {
	Reported        *uint64 `json:"reported"`
	UnknownAttempts uint64  `json:"unknown_attempts"`
}

// Usage is a copied cumulative client snapshot, not an invoice. Active calls can
// perform more work even when no send is active at the instant of the snapshot.
type Usage struct {
	Attempts          uint64     `json:"attempts"`
	Responses         uint64     `json:"responses"`
	StatusRetries     uint64     `json:"status_retries"`
	ConnectionRetries uint64     `json:"connection_retries"`
	RequestBodyBytes  uint64     `json:"request_body_bytes"`
	ActiveAttempts    uint64     `json:"active_attempts"`
	ActiveCalls       uint64     `json:"active_calls"`
	InputTokens       TokenUsage `json:"input_tokens"`
	OutputTokens      TokenUsage `json:"output_tokens"`
}

func (u Usage) copy() Usage {
	if u.InputTokens.Reported != nil {
		value := *u.InputTokens.Reported
		u.InputTokens.Reported = &value
	}
	if u.OutputTokens.Reported != nil {
		value := *u.OutputTokens.Reported
		u.OutputTokens.Reported = &value
	}
	return u
}

// Usage returns an independent snapshot. After Close it returns the snapshot
// captured at close; independent streams can continue, so this may be unresolved.
func (c *Client) Usage() (Usage, error) {
	if c == nil {
		return Usage{}, &Error{code: CodeUsageClosed, message: "thinkthen: closed"}
	}
	c.mu.Lock()
	defer c.mu.Unlock()
	if c.native != nil {
		return c.native.usage()
	}
	if c.finalUsage != nil {
		return c.finalUsage.copy(), nil
	}
	if c.usageError != nil {
		return Usage{}, c.usageError
	}
	return Usage{}, &Error{code: CodeUsageClosed, message: "thinkthen: closed"}
}
