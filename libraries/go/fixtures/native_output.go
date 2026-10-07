package main

import (
	"encoding/hex"
	"errors"
	thinkthen "github.com/botassembly/thinkthen/libraries/go"
)

func contentValue(v thinkthen.Content) any {
	if v.Kind == thinkthen.ContentKindText {
		return v.Text
	}
	if len(v.Json) == 0 {
		return nil
	}
	return v.Json
}
func optValue[T any](v thinkthen.Optional[T]) any {
	if v.Present {
		return v.Value
	}
	return nil
}
func decideValue(v thinkthen.DecideValue) any {
	switch v.Kind {
	case thinkthen.ValueKindBoolean:
		return v.Boolean
	case thinkthen.ValueKindAuthored:
		return contentValue(v.Authored.Value)
	default:
		return nil
	}
}
func memberValue(v thinkthen.MemberValue) any {
	switch v.Function {
	case thinkthen.FunctionDecide:
		return decideValue(v.Decide.Value)
	case thinkthen.FunctionChoose:
		return optValue(v.Choose)
	case thinkthen.FunctionTag:
		return v.Tag.Value
	case thinkthen.FunctionScore:
		return v.Score.Value
	}
	return nil
}
func authorValue(a thinkthen.QuestionAuthor) map[string]any {
	out := map[string]any{}
	if a.Name.Present {
		out["name"] = a.Name.Value
	}
	if a.WordingVersion.Present {
		out["wording_version"] = a.WordingVersion.Value
	}
	return out
}
func locationValue(out map[string]any, p thinkthen.Optional[thinkthen.Location]) {
	if !p.Present {
		return
	}
	if p.Value.File.Present {
		out["file"] = p.Value.File.Value
	}
	if p.Value.FirstLine.Present {
		out["first_line"] = p.Value.FirstLine.Value
	}
	if p.Value.LastLine.Present {
		out["last_line"] = p.Value.LastLine.Value
	}
}
func endpointValue(v thinkthen.Endpoint) any { return map[string]any{"name": v.Name, "kind": v.Kind} }
func edgesValue(v []thinkthen.Edge) []any {
	out := []any{}
	for _, e := range v {
		row := map[string]any{"relation": e.Relation, "source": endpointValue(e.Source), "target": endpointValue(e.Target), "probability": e.Probability}
		if e.Either {
			row["either"] = true
		}
		out = append(out, row)
	}
	return out
}
func entityValue(v thinkthen.Entity) any {
	return map[string]any{"text": v.Text, "start": v.Start, "end": v.End, "length": v.Length, "kind": v.Kind, "strength": v.Strength}
}
func recognizeValue(v thinkthen.RecognizeValue) any {
	entities := []any{}
	for _, e := range v.Entities {
		entities = append(entities, entityValue(e))
	}
	out := map[string]any{"entities": entities}
	if v.Relations.Present {
		edges := []any{}
		for _, e := range v.Relations.Value {
			row := map[string]any{"relation": e.Relation, "source": entityValue(e.Source), "target": entityValue(e.Target), "probability": e.Probability}
			if e.Either {
				row["either"] = true
			}
			edges = append(edges, row)
		}
		out["relations"] = edges
	}
	return out
}
func normalizedRow(value any, index int, events []thinkthen.ObservationEvent) map[string]any {
	var common thinkthen.CommonRow
	var bare any
	var selected any
	var members []thinkthen.AnnotationMember
	switch row := value.(type) {
	case thinkthen.DecideRow:
		common = row.Common
		bare = decideValue(row.Value)
	case thinkthen.ChooseRow:
		common = row.Common
		bare = optValue(row.Value)
	case thinkthen.TagRow:
		common = row.Common
		bare = row.Value
	case thinkthen.ScoreRow:
		common = row.Common
		bare = row.Value
	case thinkthen.FilterRow:
		common = row.Common
		bare = row.Value
	case thinkthen.RankRow:
		common = row.Common
		bare = optValue(row.Value)
	case thinkthen.FindRow:
		common = row.Common
		if row.Value.Present {
			bare = contentValue(row.Value.Value)
		}
		selected = optValue(row.Index)
	case thinkthen.AnnotateRow:
		common = row.Common
		members = row.Answers
		values := map[string]any{}
		for _, m := range members {
			if m.Success.Present {
				values[m.Name] = memberValue(m.Success.Value.Value)
			} else {
				values[m.Name] = map[string]any{"failed": map[string]any{"kind": "backend", "cause": string(m.Failure.Value.Cause)}}
			}
		}
		bare = values
	case thinkthen.RecognizeRow:
		common = row.Common
		bare = recognizeValue(row.Value)
	case thinkthen.RelateRow:
		common = row.Common
		bare = edgesValue(row.Value)
	}
	ordinal := uint64(0)
	for _, event := range events {
		if event.Row.Present {
			r := event.Row.Value
			var id thinkthen.AnswerId
			switch r.Value.Function {
			case thinkthen.FunctionDecide:
				id = r.Value.Decide.Value.Common.AnswerId
			case thinkthen.FunctionChoose:
				id = r.Value.Choose.Value.Common.AnswerId
			case thinkthen.FunctionTag:
				id = r.Value.Tag.Value.Common.AnswerId
			case thinkthen.FunctionScore:
				id = r.Value.Score.Value.Common.AnswerId
			case thinkthen.FunctionFilter:
				id = r.Value.Filter.Value.Common.AnswerId
			case thinkthen.FunctionRank:
				id = r.Value.Rank.Value.Common.AnswerId
			case thinkthen.FunctionAnnotate:
				id = r.Value.Annotate.Value.Common.AnswerId
			case thinkthen.FunctionRecognize:
				id = r.Value.Recognize.Value.Common.AnswerId
			}
			if id.String() == common.AnswerId.String() {
				ordinal = r.Index
				break
			}
		}
	}
	out := map[string]any{"value": bare, "index": ordinal, "answer_id": common.AnswerId.String(), "origin": nil, "answered_by": optValue(common.Meta.AnsweredBy), "observations": len(common.Meta.Observations), "sources": len(common.Meta.QuestionSources), "input": nil}
	if _, find := value.(thinkthen.FindRow); find {
		out["index"] = selected
	}
	if common.Input.Present {
		out["input"] = contentValue(common.Input.Value)
	}
	if common.Meta.Origin.Present {
		out["origin"] = map[thinkthen.Origin]int{thinkthen.OriginLive: 1, thinkthen.OriginCache: 2, thinkthen.OriginReplay: 3}[common.Meta.Origin.Value]
	}
	locationValue(out, common.Position)
	ids := []string{}
	for _, id := range common.Meta.Observations {
		if id.ObservationId.Present {
			ids = append(ids, id.ObservationId.Value.String())
		}
	}
	out["observation_ids"] = ids
	if common.Images.Present {
		images := []string{}
		for _, image := range common.Images.Value {
			images = append(images, hex.EncodeToString(image.Bytes))
		}
		out["images"] = images
		props := [][]uint64{}
		for _, i := range common.Images.Value {
			media := uint64(2)
			if i.Media == thinkthen.MediaJpeg {
				media = 1
			}
			props = append(props, []uint64{media, i.Width, i.Height})
		}
		out["image_properties"] = props
	}
	if common.Answer.Present {
		a := common.Answer.Value
		out["answer_kind"] = string(a.Kind)
		if a.Probability.Present {
			out["probability"] = a.Probability.Value
		} else {
			ps := map[string]float64{}
			for _, p := range a.Probabilities {
				ps[p.Name] = p.Probability
			}
			out["probabilities"] = ps
		}
	}
	inputs := []any{}
	for _, input := range common.Details.Inputs {
		row := map[string]any{"input": nil}
		if input.Original.Present {
			row["input"] = contentValue(input.Original.Value)
		}
		locationValue(row, input.Position)
		inputs = append(inputs, row)
	}
	out["detail_inputs"] = inputs
	for k, v := range authorValue(common.Author) {
		out[k] = v
	}
	if members != nil {
		authors := []any{}
		for _, m := range members {
			authors = append(authors, authorValue(m.Author))
		}
		out["member_authors"] = authors
	}
	return out
}
func normalizedError(err error) map[string]any {
	var e *thinkthen.Error
	if !errors.As(err, &e) {
		return map[string]any{"code": 6, "message": err.Error()}
	}
	out := map[string]any{"code": e.Code, "message": e.Message}
	if e.Complete.Present {
		c := e.Complete.Value
		if c.Facts.Present {
			out["requests_sent"] = c.Facts.Value.RequestsSent
			out["records"] = c.Facts.Value.Records
		}
		if c.Stopped.Present && c.Stopped.Value.At.Present {
			out["stopped_at"] = c.Stopped.Value.At.Value
		}
	}
	return out
}
func normalizedCall[T any](call thinkthen.CompleteCall[T], err error) map[string]any {
	if err != nil {
		return normalizedError(err)
	}
	rows := []any{}
	for i, row := range call.Rows {
		rows = append(rows, normalizedRow(row, i, call.Observations))
	}
	if call.Error.Present {
		c := call.Error.Value
		out := map[string]any{"code": c.Code, "message": c.Message}
		if call.Facts.Present {
			out["requests_sent"] = call.Facts.Value.RequestsSent
			out["records"] = call.Facts.Value.Records
		}
		if c.Stopped.Present && c.Stopped.Value.At.Present {
			out["stopped_at"] = c.Stopped.Value.At.Value
		}
		return out
	}
	out := map[string]any{"code": 0, "schema": call.Schema, "rows": rows, "observations": len(call.Observations)}
	if call.Facts.Present {
		out["call_id"] = call.Facts.Value.CallId.String()
		out["requests_sent"] = call.Facts.Value.RequestsSent
		out["cache_answers"] = call.Facts.Value.CacheAnswers
	}
	return out
}
func normalizedBatch[T any](b *thinkthen.CompleteBatch[T], err error) map[string]any {
	if err != nil {
		return normalizedError(err)
	}
	defer b.Close()
	rows := []any{}
	for {
		call, err := b.Next()
		if err != nil {
			out := normalizedError(err)
			out["completed"] = rows
			return out
		}
		if call == nil {
			break
		}
		part := normalizedCall(*call, nil)
		if part["code"] != 0 {
			part["completed"] = rows
			return part
		}
		rows = append(rows, part["rows"].([]any)...)
	}
	facts, err := b.Facts()
	if err != nil {
		out := normalizedError(err)
		out["completed"] = rows
		return out
	}
	if facts == nil {
		panic("batch has no final facts")
	}
	out := normalizedCall(*facts, nil)
	out["completed"] = rows
	out["rows"] = rows
	return out
}
