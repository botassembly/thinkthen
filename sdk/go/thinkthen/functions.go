package thinkthen

import (
	"context"
	"encoding/json"
	"iter"
)

func typedRow[A, D any](r wireRow) (Row[A, D], error) {
	entry, err := typedEntry[A, D](r)
	return Row[A, D]{entry}, err
}
func typedEntry[A, D any](r wireRow) (Entry[A, D], error) {
	var a A
	var d D
	switch r.status {
	case "answered":
		if err := json.Unmarshal(r.answer, &a); err != nil {
			return Entry[A, D]{}, abiError()
		}
	case "unsure", "failed":
	default:
		return Entry[A, D]{}, abiError()
	}
	if len(r.detail) > 0 {
		if err := json.Unmarshal(r.detail, &d); err != nil {
			return Entry[A, D]{}, abiError()
		}
	}
	return Entry[A, D]{r}, nil
}

func (c *Client) Decide(ctx context.Context, q Decide, record Record) (DecideRow, error) {
	return single(c.DecideAll(ctx, q, Records(record)))
}
func (c *Client) DecideAll(ctx context.Context, q Decide, input Input) iter.Seq2[DecideRow, error] {
	return drive(c, ctx, q.q, input, func(r wireRow, item Record) (DecideRow, error) { return typedRow[bool, NoDetail](r) })
}
func (c *Client) Choose(ctx context.Context, q Choose, record Record) (ChooseRow, error) {
	return single(c.ChooseAll(ctx, q, Records(record)))
}
func (c *Client) ChooseAll(ctx context.Context, q Choose, input Input) iter.Seq2[ChooseRow, error] {
	return drive(c, ctx, q.q, input, func(r wireRow, item Record) (ChooseRow, error) { return typedRow[string, LabelsDetail](r) })
}
func (c *Client) Tag(ctx context.Context, q Tag, record Record) (TagRow, error) {
	return single(c.TagAll(ctx, q, Records(record)))
}
func (c *Client) TagAll(ctx context.Context, q Tag, input Input) iter.Seq2[TagRow, error] {
	return drive(c, ctx, q.q, input, func(r wireRow, item Record) (TagRow, error) { return typedRow[[]string, LabelsDetail](r) })
}
func (c *Client) Score(ctx context.Context, q Score, record Record) (ScoreRow, error) {
	return single(c.ScoreAll(ctx, q, Records(record)))
}
func (c *Client) ScoreAll(ctx context.Context, q Score, input Input) iter.Seq2[ScoreRow, error] {
	return drive(c, ctx, q.q, input, func(r wireRow, item Record) (ScoreRow, error) { return typedRow[float64, LevelsDetail](r) })
}
func (c *Client) Filter(ctx context.Context, q Filter, record Record) (FilterRow, error) {
	return single(c.FilterAll(ctx, q, Records(record)))
}
func (c *Client) FilterAll(ctx context.Context, q Filter, input Input) iter.Seq2[FilterRow, error] {
	return drive(c, ctx, q.q, input, func(r wireRow, item Record) (FilterRow, error) {
		v, err := typedRow[bool, NoDetail](r)
		return FilterRow{v, item}, err
	})
}
func (c *Client) Rank(ctx context.Context, q Rank, record Record) (RankRow, error) {
	return single(c.RankAll(ctx, q, Records(record)))
}
func (c *Client) RankAll(ctx context.Context, q Rank, input Input) iter.Seq2[RankRow, error] {
	return drive(c, ctx, q.q, input, func(r wireRow, item Record) (RankRow, error) { return typedRow[uint64, LevelsDetail](r) })
}
func (c *Client) Find(ctx context.Context, q Find, record Record) (FindRow, error) {
	return single(c.FindAll(ctx, q, Records(record)))
}
func (c *Client) FindAll(ctx context.Context, q Find, input Input) iter.Seq2[FindRow, error] {
	return drive(c, ctx, q.q, input, func(r wireRow, item Record) (FindRow, error) { return typedRow[*Span, NoDetail](r) })
}
func (c *Client) Recognize(ctx context.Context, q Recognize, record Record) (RecognizeRow, error) {
	return single(c.RecognizeAll(ctx, q, Records(record)))
}
func (c *Client) RecognizeAll(ctx context.Context, q Recognize, input Input) iter.Seq2[RecognizeRow, error] {
	return drive(c, ctx, q.q, input, func(r wireRow, item Record) (RecognizeRow, error) { return typedRow[[]Name, RecognizeDetail](r) })
}
func (c *Client) Relate(ctx context.Context, q Relate, record Record) (RelateRow, error) {
	return single(c.RelateAll(ctx, q, Records(record)))
}
func (c *Client) RelateAll(ctx context.Context, q Relate, input Input) iter.Seq2[RelateRow, error] {
	return drive(c, ctx, q.q, input, func(r wireRow, item Record) (RelateRow, error) { return typedRow[[]Relation, PairsDetail](r) })
}
func (c *Client) Annotate(ctx context.Context, q Annotate, record Record) (AnnotateRow, error) {
	return single(c.AnnotateAll(ctx, q, Records(record)))
}
func (c *Client) AnnotateAll(ctx context.Context, q Annotate, input Input) iter.Seq2[AnnotateRow, error] {
	return drive(c, ctx, q.q, input, func(r wireRow, item Record) (AnnotateRow, error) { return typedRow[Annotations, NoDetail](r) })
}
