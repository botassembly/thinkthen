package thinkthen

import (
	"context"
	"encoding/json"
)

// Decide runs the shared decide function.
func (c *Client) Decide(ctx context.Context, question Question, input Input, options Options) (*Results[bool], error) {
	return typed[bool](c.call(ctx, "decide", question, input, options))
}

// Choose runs the shared choose function.
func (c *Client) Choose(ctx context.Context, question Question, input Input, options Options) (*Results[string], error) {
	return typed[string](c.call(ctx, "choose", question, input, options))
}

// Tag runs the shared tag function.
func (c *Client) Tag(ctx context.Context, question Question, input Input, options Options) (*Results[[]string], error) {
	return typed[[]string](c.call(ctx, "tag", question, input, options))
}

// Score runs the shared score function.
func (c *Client) Score(ctx context.Context, question Question, input Input, options Options) (*Results[float64], error) {
	return typed[float64](c.call(ctx, "score", question, input, options))
}

// Filter runs the shared filter function.
func (c *Client) Filter(ctx context.Context, question Question, input Input, options Options) (*Results[bool], error) {
	return typed[bool](c.call(ctx, "filter", question, input, options))
}

// Rank runs the shared rank function.
func (c *Client) Rank(ctx context.Context, question Question, input Input, options Options) (*Results[uint64], error) {
	return typed[uint64](c.call(ctx, "rank", question, input, options))
}

// Find runs the shared find function.
func (c *Client) Find(ctx context.Context, question Question, input Input, options Options) (*Results[*Span], error) {
	return typed[*Span](c.call(ctx, "find", question, input, options))
}

// Annotate runs the shared annotate function.
func (c *Client) Annotate(ctx context.Context, question Question, input Input, options Options) (*Results[map[string]json.RawMessage], error) {
	return typed[map[string]json.RawMessage](c.call(ctx, "annotate", question, input, options))
}

// Recognize runs the shared recognize function.
func (c *Client) Recognize(ctx context.Context, question Question, input Input, options Options) (*Results[[]Name], error) {
	return typed[[]Name](c.call(ctx, "recognize", question, input, options))
}

// Relate runs the shared relate function.
func (c *Client) Relate(ctx context.Context, question Question, input Input, options Options) (*Results[[]Relation], error) {
	return typed[[]Relation](c.call(ctx, "relate", question, input, options))
}
