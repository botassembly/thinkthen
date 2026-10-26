package thinkthen

import (
	"context"
	"errors"
	"iter"
	"sync"
)

// Client admits settings once and owns independent concurrent calls.
type Client struct {
	mu     sync.Mutex
	native *nativeClient
}

func New(cfg Config) (*Client, error) {
	native, err := newNative(cfg)
	if err != nil {
		return nil, err
	}
	return &Client{native: &native}, nil
}
func (c *Client) Close() error {
	if c == nil {
		return nil
	}
	c.mu.Lock()
	defer c.mu.Unlock()
	if c.native != nil {
		c.native.free()
		c.native = nil
	}
	return nil
}
func (c *Client) open(ctx context.Context, q question, input Input) (nativeCall, error) {
	if ctx == nil {
		return nativeCall{}, conversionError(false)
	}
	if c == nil {
		return nativeCall{}, ErrClosed
	}
	c.mu.Lock()
	defer c.mu.Unlock()
	if c.native == nil {
		return nativeCall{}, ErrClosed
	}
	return c.native.open(q, input)
}

// drive owns every native driving operation; the watcher only cancels.
func drive[R any](c *Client, ctx context.Context, q question, input Input, convert func(wireRow, Record) (R, error)) iter.Seq2[R, error] {
	return func(yield func(R, error) bool) {
		var zero R
		if !input.finite && input.sequence == nil {
			yield(zero, conversionError(false))
			return
		}
		native, err := c.open(ctx, q, input)
		if err != nil {
			yield(zero, err)
			return
		}
		if ctx.Err() != nil {
			native.cancel()
		}
		done := make(chan struct{})
		joined := make(chan struct{})
		go func() {
			defer close(joined)
			select {
			case <-ctx.Done():
				native.cancel()
			case <-done:
			}
		}()
		defer func() { native.cancel(); close(done); <-joined; native.free() }()
		var next func() (Record, error, bool)
		if !input.finite {
			var stop func()
			next, stop = iter.Pull2(input.sequence)
			defer stop()
		}
		var position uint64
		var pending map[uint64]Record
		if q.function == FunctionFilter && !input.finite {
			pending = make(map[uint64]Record)
		}
		terminal := func(err error) {
			if native, ok := err.(*Error); ok && native.code == CodeLocalCancelled && ctx.Err() != nil {
				native.cause = ctx.Err()
				if cause := context.Cause(ctx); cause != ctx.Err() {
					native.cause = errors.Join(ctx.Err(), cause)
				}
			}
			yield(zero, err)
		}
		for {
			row, ok, needsInput, err := native.next()
			if err != nil {
				terminal(err)
				return
			}
			if ok {
				if row.function != uint32(q.function) {
					terminal(abiError())
					return
				}
				item := pending[row.position]
				delete(pending, row.position)
				if input.finite && q.function == FunctionFilter {
					if row.position == 0 || row.position > uint64(len(input.records)) {
						terminal(abiError())
						return
					}
					item = input.records[row.position-1]
				}
				value, err := convert(row, item)
				if err != nil {
					terminal(err)
					return
				}
				if !yield(value, nil) {
					return
				}
				continue
			}
			if !needsInput {
				return
			}
			if input.finite {
				terminal(abiError())
				return
			}
			record, inputError, more := next()
			if ctx.Err() != nil {
				native.cancel()
				continue
			}
			if inputError != nil {
				terminal(inputError)
				return
			}
			if !more {
				native.end()
				continue
			}
			position++
			if pending != nil {
				pending[position] = record
			}
			if err = native.send(serialize(record)); err != nil {
				terminal(err)
				return
			}
		}
	}
}
func single[R any](seq iter.Seq2[R, error]) (R, error) {
	var result R
	var failure error
	for row, err := range seq {
		if err != nil {
			failure = err
		} else {
			result = row
		}
	}
	return result, failure
}
