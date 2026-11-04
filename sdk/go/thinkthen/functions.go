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
		if len(r.answer) != 0 {
			return Entry[A, D]{}, abiError()
		}
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

// Decide returns a typed decide outcome for one record.
func (c *Client) Decide(ctx context.Context, q Decide, record Record) (DecideRow, error) {
	return single(c.DecideAll(ctx, q, Records(record)))
}

// DecideAll streams typed decide outcomes; break stops the stream and releases its resources.
func (c *Client) DecideAll(ctx context.Context, q Decide, input Input) iter.Seq2[DecideRow, error] {
	return drive(c, ctx, q.q, input, func(r wireRow, item Record) (DecideRow, error) { return typedRow[bool, NoDetail](r) })
}

// Choose returns a typed choose outcome for one record.
func (c *Client) Choose(ctx context.Context, q Choose, record Record) (ChooseRow, error) {
	return single(c.ChooseAll(ctx, q, Records(record)))
}

// ChooseAll streams typed choose outcomes; break stops the stream and releases its resources.
func (c *Client) ChooseAll(ctx context.Context, q Choose, input Input) iter.Seq2[ChooseRow, error] {
	return drive(c, ctx, q.q, input, func(r wireRow, item Record) (ChooseRow, error) { return typedRow[string, LabelsDetail](r) })
}

// Tag returns a typed tag outcome for one record.
func (c *Client) Tag(ctx context.Context, q Tag, record Record) (TagRow, error) {
	return single(c.TagAll(ctx, q, Records(record)))
}

// TagAll streams typed tag outcomes; break stops the stream and releases its resources.
func (c *Client) TagAll(ctx context.Context, q Tag, input Input) iter.Seq2[TagRow, error] {
	return drive(c, ctx, q.q, input, func(r wireRow, item Record) (TagRow, error) { return typedRow[[]string, LabelsDetail](r) })
}

// Score returns a typed score outcome for one record.
func (c *Client) Score(ctx context.Context, q Score, record Record) (ScoreRow, error) {
	return single(c.ScoreAll(ctx, q, Records(record)))
}

// ScoreAll streams typed score outcomes; break stops the stream and releases its resources.
func (c *Client) ScoreAll(ctx context.Context, q Score, input Input) iter.Seq2[ScoreRow, error] {
	return drive(c, ctx, q.q, input, func(r wireRow, item Record) (ScoreRow, error) { return typedRow[float64, LevelsDetail](r) })
}

// FilterAll yields kept records and failed records in their original places.
// A failed record has a nonzero Position and its Err matches the yielded error;
// callers may continue. A call or source error has Position zero and ends iteration.
func (c *Client) FilterAll(ctx context.Context, q Filter, input Input) iter.Seq2[FilterRow, error] {
	return func(yield func(FilterRow, error) bool) {
		for row, err := range c.FilterRowsAll(ctx, q, input) {
			if err != nil {
				yield(row, err)
				return
			}
			if row.Err() != nil {
				if !yield(row, row.Err()) {
					return
				}
			} else if row.Kept() {
				if !yield(row, nil) {
					return
				}
			}
		}
	}
}

// FilterRowsAll yields every input row, including dropped, unsure and failed rows.
func (c *Client) FilterRowsAll(ctx context.Context, q Filter, input Input) iter.Seq2[FilterRow, error] {
	return drive(c, ctx, q.q, input, func(r wireRow, item Record) (FilterRow, error) {
		v, err := typedRow[bool, NoDetail](r)
		return FilterRow{v, item}, err
	})
}

// Rank returns a typed rank outcome for one record.
func (c *Client) Rank(ctx context.Context, q Rank, record Record) (RankRow, error) {
	return single(c.RankAll(ctx, q, Records(record)))
}

// RankAll streams typed rank outcomes; break stops the stream and releases its resources.
func (c *Client) RankAll(ctx context.Context, q Rank, input Input) iter.Seq2[RankRow, error] {
	return drive(c, ctx, q.q, input, func(r wireRow, item Record) (RankRow, error) {
		row, err := typedRow[int, LevelsDetail](r)
		return RankRow{row}, err
	})
}

// Find returns a typed find outcome for one record.
func (c *Client) Find(ctx context.Context, q Find, record Record) (FindRow, error) {
	return single(c.FindAll(ctx, q, Records(record)))
}

// FindAll streams typed find outcomes; break stops the stream and releases its resources.
func (c *Client) FindAll(ctx context.Context, q Find, input Input) iter.Seq2[FindRow, error] {
	return drive(c, ctx, q.q, input, func(r wireRow, item Record) (FindRow, error) { return typedRow[*Span, NoDetail](r) })
}

// Recognize returns a typed recognize outcome for one record.
func (c *Client) Recognize(ctx context.Context, q Recognize, record Record) (RecognizeRow, error) {
	return single(c.RecognizeAll(ctx, q, Records(record)))
}

// RecognizeAll streams typed recognize outcomes; break stops the stream and releases its resources.
func (c *Client) RecognizeAll(ctx context.Context, q Recognize, input Input) iter.Seq2[RecognizeRow, error] {
	return drive(c, ctx, q.q, input, func(r wireRow, item Record) (RecognizeRow, error) { return typedRow[[]Name, RecognizeDetail](r) })
}

// Relate returns a typed relate outcome for one record.
func (c *Client) Relate(ctx context.Context, q Relate, record Record) (RelateRow, error) {
	return single(c.RelateAll(ctx, q, Records(record)))
}

// RelateAll streams typed relate outcomes; break stops the stream and releases its resources.
func (c *Client) RelateAll(ctx context.Context, q Relate, input Input) iter.Seq2[RelateRow, error] {
	return drive(c, ctx, q.q, input, func(r wireRow, item Record) (RelateRow, error) { return typedRow[[]Relation, PairsDetail](r) })
}

// Annotate returns a typed annotate outcome for one record.
func (c *Client) Annotate(ctx context.Context, q Annotate, record Record) (AnnotateRow, error) {
	return single(c.AnnotateAll(ctx, q, Records(record)))
}

// AnnotateAll streams typed annotate outcomes; break stops the stream and releases its resources.
func (c *Client) AnnotateAll(ctx context.Context, q Annotate, input Input) iter.Seq2[AnnotateRow, error] {
	return drive(c, ctx, q.q, input, func(r wireRow, item Record) (AnnotateRow, error) { return typedRow[Annotations, NoDetail](r) })
}
