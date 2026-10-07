package thinkthen

/*
#cgo pkg-config: thinkthen
#include "thinkthen.h"
*/
import "C"
import (
	"context"
	"fmt"
)

func readDecideRow(result *C.thinkthen_result, i C.size_t) (DecideRow, error) {
	var v C.thinkthen_decide_view_v1
	if rc := C.thinkthen_result_decide(result, i, &v); rc != 0 {
		return DecideRow{}, fmt.Errorf("native decide row failed: %d", rc)
	}
	row := nativeDecideRow(v)
	if err := rowDetails(result, i, &row.Common); err != nil {
		return row, err
	}
	return row, nil
}
func (e *Engine) DecideComplete(ctx context.Context, q QuestionInput, source InputSource, controls CallControls) (CompleteCall[DecideRow], error) {
	return executeComplete(e, ctx, q, source, controls, func(e *C.thinkthen_engine, q *C.thinkthen_question, s *C.thinkthen_source, c *C.thinkthen_controls_v1, r **C.thinkthen_result) C.int {
		return C.thinkthen_decide_complete(e, q, s, c, r)
	}, readDecideRow)
}
func readChooseRow(result *C.thinkthen_result, i C.size_t) (ChooseRow, error) {
	var v C.thinkthen_choose_view_v1
	if rc := C.thinkthen_result_choose(result, i, &v); rc != 0 {
		return ChooseRow{}, fmt.Errorf("native choose row failed: %d", rc)
	}
	row := nativeChooseRow(v)
	if err := rowDetails(result, i, &row.Common); err != nil {
		return row, err
	}
	return row, nil
}
func (e *Engine) ChooseComplete(ctx context.Context, q QuestionInput, source InputSource, controls CallControls) (CompleteCall[ChooseRow], error) {
	return executeComplete(e, ctx, q, source, controls, func(e *C.thinkthen_engine, q *C.thinkthen_question, s *C.thinkthen_source, c *C.thinkthen_controls_v1, r **C.thinkthen_result) C.int {
		return C.thinkthen_choose_complete(e, q, s, c, r)
	}, readChooseRow)
}
func readTagRow(result *C.thinkthen_result, i C.size_t) (TagRow, error) {
	var v C.thinkthen_tag_view_v1
	if rc := C.thinkthen_result_tag(result, i, &v); rc != 0 {
		return TagRow{}, fmt.Errorf("native tag row failed: %d", rc)
	}
	row := nativeTagRow(v)
	if err := rowDetails(result, i, &row.Common); err != nil {
		return row, err
	}
	return row, nil
}
func (e *Engine) TagComplete(ctx context.Context, q QuestionInput, source InputSource, controls CallControls) (CompleteCall[TagRow], error) {
	return executeComplete(e, ctx, q, source, controls, func(e *C.thinkthen_engine, q *C.thinkthen_question, s *C.thinkthen_source, c *C.thinkthen_controls_v1, r **C.thinkthen_result) C.int {
		return C.thinkthen_tag_complete(e, q, s, c, r)
	}, readTagRow)
}
func readScoreRow(result *C.thinkthen_result, i C.size_t) (ScoreRow, error) {
	var v C.thinkthen_score_view_v1
	if rc := C.thinkthen_result_score(result, i, &v); rc != 0 {
		return ScoreRow{}, fmt.Errorf("native score row failed: %d", rc)
	}
	row := nativeScoreRow(v)
	if err := rowDetails(result, i, &row.Common); err != nil {
		return row, err
	}
	return row, nil
}
func (e *Engine) ScoreComplete(ctx context.Context, q QuestionInput, source InputSource, controls CallControls) (CompleteCall[ScoreRow], error) {
	return executeComplete(e, ctx, q, source, controls, func(e *C.thinkthen_engine, q *C.thinkthen_question, s *C.thinkthen_source, c *C.thinkthen_controls_v1, r **C.thinkthen_result) C.int {
		return C.thinkthen_score_complete(e, q, s, c, r)
	}, readScoreRow)
}
func readFilterRow(result *C.thinkthen_result, i C.size_t) (FilterRow, error) {
	var v C.thinkthen_filter_view_v1
	if rc := C.thinkthen_result_filter(result, i, &v); rc != 0 {
		return FilterRow{}, fmt.Errorf("native filter row failed: %d", rc)
	}
	row := nativeFilterRow(v)
	if err := rowDetails(result, i, &row.Common); err != nil {
		return row, err
	}
	return row, nil
}
func (e *Engine) FilterComplete(ctx context.Context, q QuestionInput, source InputSource, controls CallControls) (CompleteCall[FilterRow], error) {
	return executeComplete(e, ctx, q, source, controls, func(e *C.thinkthen_engine, q *C.thinkthen_question, s *C.thinkthen_source, c *C.thinkthen_controls_v1, r **C.thinkthen_result) C.int {
		return C.thinkthen_filter_complete(e, q, s, c, r)
	}, readFilterRow)
}
func readRankRow(result *C.thinkthen_result, i C.size_t) (RankRow, error) {
	var v C.thinkthen_rank_view_v1
	if rc := C.thinkthen_result_rank(result, i, &v); rc != 0 {
		return RankRow{}, fmt.Errorf("native rank row failed: %d", rc)
	}
	row := nativeRankRow(v)
	if err := rowDetails(result, i, &row.Common); err != nil {
		return row, err
	}
	var count C.size_t
	if rc := C.thinkthen_result_rank_member_count(result, i, &count); rc != 0 {
		return row, fmt.Errorf("native rank members failed: %d", rc)
	}
	row.Members = make([]RankRow, int(count))
	for j := range row.Members {
		var member C.thinkthen_rank_view_v1
		if rc := C.thinkthen_result_rank_member(result, i, C.size_t(j), &member); rc != 0 {
			return row, fmt.Errorf("native rank member failed: %d", rc)
		}
		row.Members[j] = nativeRankRow(member)
	}
	return row, nil
}
func (e *Engine) RankComplete(ctx context.Context, q QuestionInput, source InputSource, controls CallControls) (CompleteCall[RankRow], error) {
	return executeComplete(e, ctx, q, source, controls, func(e *C.thinkthen_engine, q *C.thinkthen_question, s *C.thinkthen_source, c *C.thinkthen_controls_v1, r **C.thinkthen_result) C.int {
		return C.thinkthen_rank_complete(e, q, s, c, r)
	}, readRankRow)
}
func readFindRow(result *C.thinkthen_result, i C.size_t) (FindRow, error) {
	var v C.thinkthen_find_view_v1
	if rc := C.thinkthen_result_find(result, i, &v); rc != 0 {
		return FindRow{}, fmt.Errorf("native find row failed: %d", rc)
	}
	row := nativeFindRow(v)
	if err := rowDetails(result, i, &row.Common); err != nil {
		return row, err
	}
	return row, nil
}
func (e *Engine) FindComplete(ctx context.Context, q QuestionInput, source InputSource, controls CallControls) (CompleteCall[FindRow], error) {
	return executeComplete(e, ctx, q, source, controls, func(e *C.thinkthen_engine, q *C.thinkthen_question, s *C.thinkthen_source, c *C.thinkthen_controls_v1, r **C.thinkthen_result) C.int {
		return C.thinkthen_find_complete(e, q, s, c, r)
	}, readFindRow)
}
func readAnnotateRow(result *C.thinkthen_result, i C.size_t) (AnnotateRow, error) {
	var v C.thinkthen_annotate_view_v1
	if rc := C.thinkthen_result_annotate(result, i, &v); rc != 0 {
		return AnnotateRow{}, fmt.Errorf("native annotate row failed: %d", rc)
	}
	row := nativeAnnotateRow(v)
	if err := rowDetails(result, i, &row.Common); err != nil {
		return row, err
	}
	for j := range row.Answers {
		var author C.thinkthen_question_author_v1
		if rc := C.thinkthen_result_member_author(result, i, C.size_t(j), &author); rc != 0 {
			return row, fmt.Errorf("native member author failed: %d", rc)
		}
		row.Answers[j].Author = nativeQuestionAuthor(author)
	}
	return row, nil
}
func (e *Engine) AnnotateComplete(ctx context.Context, q QuestionInput, source InputSource, controls CallControls) (CompleteCall[AnnotateRow], error) {
	return executeComplete(e, ctx, q, source, controls, func(e *C.thinkthen_engine, q *C.thinkthen_question, s *C.thinkthen_source, c *C.thinkthen_controls_v1, r **C.thinkthen_result) C.int {
		return C.thinkthen_annotate_complete(e, q, s, c, r)
	}, readAnnotateRow)
}
func readRecognizeRow(result *C.thinkthen_result, i C.size_t) (RecognizeRow, error) {
	var v C.thinkthen_recognize_view_v1
	if rc := C.thinkthen_result_recognize(result, i, &v); rc != 0 {
		return RecognizeRow{}, fmt.Errorf("native recognize row failed: %d", rc)
	}
	row := nativeRecognizeRow(v)
	if err := rowDetails(result, i, &row.Common); err != nil {
		return row, err
	}
	var located C.thinkthen_source_recognition_v1
	if rc := C.thinkthen_result_source_recognition(result, i, &located); rc != 0 {
		return row, fmt.Errorf("native source recognition failed: %d", rc)
	}
	row.Located = nativeSourceRecognition(located)
	return row, nil
}
func (e *Engine) RecognizeComplete(ctx context.Context, q QuestionInput, source InputSource, controls CallControls) (CompleteCall[RecognizeRow], error) {
	return executeComplete(e, ctx, q, source, controls, func(e *C.thinkthen_engine, q *C.thinkthen_question, s *C.thinkthen_source, c *C.thinkthen_controls_v1, r **C.thinkthen_result) C.int {
		return C.thinkthen_recognize_complete(e, q, s, c, r)
	}, readRecognizeRow)
}
func readRelateRow(result *C.thinkthen_result, i C.size_t) (RelateRow, error) {
	var v C.thinkthen_relate_view_v1
	if rc := C.thinkthen_result_relate(result, i, &v); rc != 0 {
		return RelateRow{}, fmt.Errorf("native relate row failed: %d", rc)
	}
	row := nativeRelateRow(v)
	if err := rowDetails(result, i, &row.Common); err != nil {
		return row, err
	}
	var located C.thinkthen_source_relations_v1
	if rc := C.thinkthen_result_source_relations(result, i, &located); rc != 0 {
		return row, fmt.Errorf("native source relations failed: %d", rc)
	}
	row.Located = nativeSourceRelations(located)
	return row, nil
}
func (e *Engine) RelateComplete(ctx context.Context, q QuestionInput, source InputSource, controls CallControls) (CompleteCall[RelateRow], error) {
	return executeComplete(e, ctx, q, source, controls, func(e *C.thinkthen_engine, q *C.thinkthen_question, s *C.thinkthen_source, c *C.thinkthen_controls_v1, r **C.thinkthen_result) C.int {
		return C.thinkthen_relate_complete(e, q, s, c, r)
	}, readRelateRow)
}
