//go:build cgo

package thinkthen

import (
	"context"
	"errors"
	"iter"
	"sync"
)

// Client admits settings once and owns independent concurrent calls.
type Client struct {
	mu         sync.Mutex
	native     *nativeClient
	finalUsage *Usage
	usageError error
}

// New opens a client using cfg, with zero values selecting engine defaults.
func New(cfg Config) (*Client, error) {
	native, err := newNative(cfg)
	if err != nil {
		return nil, err
	}
	return &Client{native: &native}, nil
}

// Close releases the client and prevents new calls; existing streams remain independent.
func (c *Client) Close() error {
	if c == nil {
		return nil
	}
	c.mu.Lock()
	defer c.mu.Unlock()
	if c.native != nil {
		usage, err := c.native.usage()
		c.usageError = err
		if err == nil {
			c.finalUsage = &usage
		}
		c.native.free()
		c.native = nil
	}
	return c.usageError
}
func (c *Client) open(ctx context.Context, q question) (nativeCall, error) {
	if ctx == nil {
		return nativeCall{}, &Error{code: CodeUsageArgument, message: "context must not be nil"}
	}
	if c == nil {
		return nativeCall{}, &Error{code: CodeUsageClosed, message: "thinkthen: closed"}
	}
	c.mu.Lock()
	defer c.mu.Unlock()
	if c.native == nil {
		return nativeCall{}, &Error{code: CodeUsageClosed, message: "thinkthen: closed"}
	}
	return c.native.open(q)
}

// drive owns every native driving operation; the watcher only cancels.
func drive[R any](c *Client, ctx context.Context, q question, input inputValue, convert func(wireRow) (R, error)) iter.Seq2[R, error] {
	return func(yield func(R, error) bool) {
		var zero R
		if input.sequence == nil {
			yield(zero, &Error{code: CodeUsageArgument, message: "input must be an iterator"})
			return
		}
		native, err := c.open(ctx, q)
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
		next, stop := iter.Pull(input.sequence)
		defer stop()
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
				value, err := convert(row)
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
			record, more := next()
			if ctx.Err() != nil {
				native.cancel()
				continue
			}
			if !more {
				native.end()
				continue
			}
			if err = native.send(serialize(record)); err != nil {
				terminal(err)
				return
			}
		}
	}
}
func single[R interface{ Err() error }](seq iter.Seq2[R, error]) (R, error) {
	var result R
	var failure error
	for row, err := range seq {
		if err != nil {
			failure = err
		} else {
			result = row
		}
	}
	if failure == nil {
		failure = result.Err()
	}
	return result, failure
}
