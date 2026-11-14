//go:build cgo

package thinkthen

import (
	"context"
	"encoding/json"
	"iter"
	"reflect"
)

func typedEntry[A, D any](r wireRow) (Entry[A, D], error) {
	var a A
	var d D
	switch r.status {
	case "answered":
		if err := decodeAnswer(r.answer, &a); err != nil {
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
func recordInput[T any](value T) (recordValue, error) {
	if any(value) == nil {
		return missingRecord(), nil
	}
	if raw, ok := any(value).(json.RawMessage); ok {
		if raw == nil {
			return rawJSONRecord([]byte("null")), nil
		}
		return rawJSONRecord(raw), nil
	}
	if text, ok := any(value).(string); ok {
		return textRecord(text), nil
	}
	return jsonRecord(value)
}
func streamRows[T, A, D any](c *Client, ctx context.Context, q question, input iter.Seq[T]) iter.Seq2[Row[T, A, D], error] {
	return func(yield func(Row[T, A, D], error) bool) {
		if input == nil {
			var zero Row[T, A, D]
			yield(zero, &Error{code: CodeUsageArgument, message: "input must be an iterator"})
			return
		}
		type pending struct {
			item  T
			cause error
		}
		originals := make(map[uint64]pending)
		var position uint64
		source := fromSeqInput(func(send func(recordValue) bool) {
			for item := range input {
				position++
				record, err := recordInput(item)
				originals[position] = pending{item, err}
				if err != nil {
					record = recordValue{kind: 3}
				}
				if !send(record) {
					return
				}
			}
		})
		for row, err := range drive(c, ctx, q, source, func(r wireRow) (Row[T, A, D], error) {
			record, ok := originals[r.position]
			if !ok {
				return Row[T, A, D]{}, abiError()
			}
			delete(originals, r.position)
			if record.cause != nil && r.failure != nil {
				failure := *r.failure
				failure.cause = record.cause
				r.failure = &failure
			}
			entry, err := typedEntry[A, D](r)
			return Row[T, A, D]{entry, record.item}, err
		}) {
			if !yield(row, err) {
				return
			}
		}
	}
}
func (c *Client) DecideAll[T any](ctx context.Context, q DecideQuestion, input iter.Seq[T]) iter.Seq2[DecideRow[T], error] {
	return streamRows[T, bool, NoDetail](c, ctx, q.definition(), input)
}
func (c *Client) Decide[T any](ctx context.Context, q DecideQuestion, item T) (DecideRow[T], error) {
	if err := singleRecord(item, "Decide"); err != nil {
		return DecideRow[T]{}, err
	}
	return single(c.DecideAll(ctx, q, func(yield func(T) bool) { yield(item) }))
}
func (c *Client) ChooseAll[T any, C ~string](ctx context.Context, q ChooseQuestion[C], input iter.Seq[T]) iter.Seq2[ChooseRow[C, T], error] {
	return streamRows[T, C, LabelsDetail](c, ctx, q.definition(), input)
}
func (c *Client) Choose[T any, C ~string](ctx context.Context, q ChooseQuestion[C], item T) (ChooseRow[C, T], error) {
	if err := singleRecord(item, "Choose"); err != nil {
		return ChooseRow[C, T]{}, err
	}
	return single(c.ChooseAll(ctx, q, func(yield func(T) bool) { yield(item) }))
}
func (c *Client) TagAll[T any, C ~string](ctx context.Context, q TagQuestion[C], input iter.Seq[T]) iter.Seq2[TagRow[C, T], error] {
	return streamRows[T, []C, LabelsDetail](c, ctx, q.definition(), input)
}
func (c *Client) Tag[T any, C ~string](ctx context.Context, q TagQuestion[C], item T) (TagRow[C, T], error) {
	if err := singleRecord(item, "Tag"); err != nil {
		return TagRow[C, T]{}, err
	}
	return single(c.TagAll(ctx, q, func(yield func(T) bool) { yield(item) }))
}
func (c *Client) ScoreAll[T any, C ~string](ctx context.Context, q ScoreQuestion[C], input iter.Seq[T]) iter.Seq2[ScoreRow[T], error] {
	return streamRows[T, float64, LevelsDetail](c, ctx, q.definition(), input)
}
func (c *Client) Score[T any, C ~string](ctx context.Context, q ScoreQuestion[C], item T) (ScoreRow[T], error) {
	if err := singleRecord(item, "Score"); err != nil {
		return ScoreRow[T]{}, err
	}
	return single(c.ScoreAll(ctx, q, func(yield func(T) bool) { yield(item) }))
}
func (c *Client) Rank[T any, C ~string](ctx context.Context, q RankQuestion[C], input iter.Seq[T]) iter.Seq2[RankRow[T], error] {
	return streamRows[T, int, LevelsDetail](c, ctx, q.definition(), input)
}
func (c *Client) FindAll[T any](ctx context.Context, q FindQuestion, input iter.Seq[T]) iter.Seq2[FindRow[T], error] {
	return streamRows[T, *Span, NoDetail](c, ctx, q.definition(), input)
}
func (c *Client) Find[T any](ctx context.Context, q FindQuestion, item T) (FindRow[T], error) {
	if err := singleRecord(item, "Find"); err != nil {
		return FindRow[T]{}, err
	}
	return single(c.FindAll(ctx, q, func(yield func(T) bool) { yield(item) }))
}
func (c *Client) RecognizeAll[T any](ctx context.Context, q RecognizeQuestion, input iter.Seq[T]) iter.Seq2[RecognizeRow[T], error] {
	return streamRows[T, []Name, RecognizeDetail](c, ctx, q.definition(), input)
}
func (c *Client) Recognize[T any](ctx context.Context, q RecognizeQuestion, item T) (RecognizeRow[T], error) {
	if err := singleRecord(item, "Recognize"); err != nil {
		return RecognizeRow[T]{}, err
	}
	return single(c.RecognizeAll(ctx, q, func(yield func(T) bool) { yield(item) }))
}
func (c *Client) RelateAll[T any](ctx context.Context, q RelateQuestion, input iter.Seq[T]) iter.Seq2[RelateRow[T], error] {
	return streamRows[T, []Relation, PairsDetail](c, ctx, q.definition(), input)
}
func (c *Client) Relate[T any](ctx context.Context, q RelateQuestion, item T) (RelateRow[T], error) {
	if err := singleRecord(item, "Relate"); err != nil {
		return RelateRow[T]{}, err
	}
	return single(c.RelateAll(ctx, q, func(yield func(T) bool) { yield(item) }))
}
func (c *Client) AnnotateAll[T any](ctx context.Context, q AnnotateQuestion, input iter.Seq[T]) iter.Seq2[AnnotateRow[T], error] {
	return streamRows[T, Annotations, NoDetail](c, ctx, q.definition(), input)
}
func (c *Client) Annotate[T any](ctx context.Context, q AnnotateQuestion, item T) (AnnotateRow[T], error) {
	if err := singleRecord(item, "Annotate"); err != nil {
		return AnnotateRow[T]{}, err
	}
	return single(c.AnnotateAll(ctx, q, func(yield func(T) bool) { yield(item) }))
}

// FilterError contains every failed record, after all kept records were yielded.
type FilterError[T any] struct{ Rows []DecideRow[T] }

func (e *FilterError[T]) Error() string { return "filter: records failed; inspect Rows" }
func (e *FilterError[T]) Unwrap() []error {
	result := make([]error, len(e.Rows))
	for i, row := range e.Rows {
		result[i] = row.Err()
	}
	return result
}
func (c *Client) Filter[T any](ctx context.Context, q FilterQuestion, input iter.Seq[T]) iter.Seq2[T, error] {
	return func(yield func(T, error) bool) {
		var failures []DecideRow[T]
		var zero T
		for row, err := range streamRows[T, bool, NoDetail](c, ctx, q.definition(), input) {
			if err != nil {
				yield(zero, err)
				return
			}
			if row.IsFailed() {
				failures = append(failures, row)
			} else {
				yes, err := row.Answer()
				if err != nil {
					yield(zero, err)
					return
				}
				if yes && !yield(row.Item(), nil) {
					return
				}
			}
		}
		if len(failures) > 0 {
			yield(zero, &FilterError[T]{Rows: failures})
		}
	}
}

func singleRecord[T any](value T, function string) error {
	if any(value) == nil {
		return nil
	}
	if _, ok := any(value).(json.RawMessage); ok {
		return nil
	}
	kind := reflect.TypeOf(value).Kind()
	if kind == reflect.Slice || kind == reflect.Array {
		return &Error{code: CodeUsageArgument, message: "a collection needs client." + function + "All"}
	}
	return nil
}
