package thinkthen

import (
	"context"
	"encoding/json"
	"errors"
	"io"
	"sync"
)

// Client admits settings once and can open independent concurrent calls.
type Client struct {
	mu     sync.Mutex
	native *nativeClient
}

// NewClient opens the matching package-owned native library and admits settings.
func NewClient(settings Settings) (*Client, error) {
	data, err := json.Marshal(settings)
	if err != nil {
		return nil, err
	}
	native, err := openClient(data)
	if err != nil {
		return nil, err
	}
	return &Client{native: &native}, nil
}

// Close releases this owner; existing calls remain usable.
func (c *Client) Close() error {
	c.mu.Lock()
	defer c.mu.Unlock()
	if c.native != nil {
		c.native.free()
		c.native = nil
	}
	return nil
}

// Rows pulls immutable results. Next, Row and Err have one caller; Close may
// interrupt Next from another goroutine. A terminal Next releases native state.
type Rows struct {
	operation sync.Mutex
	pointer   sync.Mutex
	native    nativeCall
	active    bool
	ctx       context.Context
	parent    context.Context
	cancel    context.CancelFunc
	watcher   chan struct{}
	records   []wireRecord
	producer  Producer
	index     int
	row       Row
	err       error
}

func (c *Client) call(ctx context.Context, function string, question Question, input Input, options Options) (*Rows, error) {
	if ctx == nil {
		return nil, errors.New("thinkthen: nil context")
	}
	data, err := json.Marshal(struct {
		Function string   `json:"function"`
		Question Question `json:"question"`
		Options  Options  `json:"options,omitempty"`
	}{function, question, options})
	if err != nil {
		return nil, err
	}
	records := make([]wireRecord, 0, len(input.records))
	for _, record := range input.records {
		serialized, e := serialize(record)
		if e != nil {
			return nil, e
		}
		records = append(records, serialized)
	}
	c.mu.Lock()
	if c.native == nil {
		c.mu.Unlock()
		return nil, io.ErrClosedPipe
	}
	native, err := c.native.open(data)
	c.mu.Unlock()
	if err != nil {
		return nil, err
	}
	derived, cancel := context.WithCancel(ctx)
	rows := &Rows{native: native, active: true, ctx: derived, parent: ctx, cancel: cancel, watcher: make(chan struct{}), records: records, producer: input.producer}
	go func() {
		defer close(rows.watcher)
		<-derived.Done()
		rows.pointer.Lock()
		if rows.active {
			rows.native.cancel()
		}
		rows.pointer.Unlock()
	}()
	return rows, nil
}

// Next advances to one row, or false at completion. Inspect Err afterward.
func (r *Rows) Next() bool {
	r.operation.Lock()
	defer r.operation.Unlock()
	if !r.active {
		return false
	}
	for {
		row, ok, needsInput, err := r.native.next()
		if err != nil {
			r.finish(err)
			return false
		}
		if ok {
			r.row = row
			return true
		}
		if !needsInput {
			r.finish(nil)
			return false
		}
		record, err := r.input()
		if r.ctx.Err() != nil {
			// Pull the shared cancellation error rather than substituting a host error.
			r.native.cancel()
			_, _, _, err = r.native.next()
			r.finish(err)
			return false
		}
		if errors.Is(err, io.EOF) {
			r.native.end()
			continue
		}
		if err != nil {
			r.finish(err)
			return false
		}
		if err = r.native.send(record); err != nil {
			r.finish(err)
			return false
		}
	}
}
func (r *Rows) input() (wireRecord, error) {
	if r.index < len(r.records) {
		value := r.records[r.index]
		r.index++
		return value, nil
	}
	if r.producer == nil {
		return wireRecord{}, io.EOF
	}
	type result struct {
		record wireRecord
		err    error
	}
	// Only one producer invocation is outstanding. A cancelled callback owns no
	// native pointer and can finish after this call has been released.
	producer, ctx := r.producer, r.ctx
	resultChannel := make(chan result)
	go func() {
		record, err := producer(ctx)
		var value wireRecord
		if err == nil {
			value, err = serialize(record)
		}
		select {
		case resultChannel <- result{value, err}:
		case <-ctx.Done():
		}
	}()
	select {
	case value := <-resultChannel:
		return value.record, value.err
	case <-r.ctx.Done():
		return wireRecord{}, r.ctx.Err()
	}
}
func (r *Rows) finish(err error) {
	if native, ok := err.(*Error); ok && native.code == "local.cancelled" {
		native.cause = r.parent.Err()
	}
	r.err = err
	r.cancel()
	<-r.watcher
	r.pointer.Lock()
	r.native.free()
	r.active = false
	r.pointer.Unlock()
	r.records = nil
}

// Row returns the last delivered immutable row, or a zero row before Next.
func (r *Rows) Row() Row { return r.row }

// Err returns the terminal error after Next returns false.
func (r *Rows) Err() error { return r.err }

// Close cancels early iteration and releases the native owner without waiting
// for a backend worker or a producer that ignores its cancellation context.
func (r *Rows) Close() error {
	if r.cancel == nil {
		return nil
	}
	r.cancel()
	r.operation.Lock()
	defer r.operation.Unlock()
	if r.active {
		r.native.cancel()
		_, _, _, err := r.native.next()
		r.finish(err)
	}
	return nil
}
