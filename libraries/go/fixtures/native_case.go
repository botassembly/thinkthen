package main

import (
	"context"
	"encoding/json"
	"fmt"
	thinkthen "github.com/botassembly/thinkthen/libraries/go"
	"os"
)

type nativeDocument struct {
	Verb           string            `json:"verb"`
	Question       json.RawMessage   `json:"question"`
	Raw            *string           `json:"raw"`
	Items          []json.RawMessage `json:"items"`
	Text           bool              `json:"text"`
	Paths          []string          `json:"paths"`
	Unit           uint32            `json:"source_unit"`
	Window         uint64            `json:"window"`
	ImagePaths     []string          `json:"image_paths"`
	ImageOnly      bool              `json:"image_only"`
	ImageReader    bool              `json:"image_reader"`
	Context        json.RawMessage   `json:"context"`
	ContextPresent bool              `json:"context_present"`
	SharedContext  json.RawMessage   `json:"shared_context"`
	Loader         string            `json:"loader"`
	Reference      string            `json:"reference"`
	QuestionForm   string            `json:"question_form"`
	Metadata       struct {
		Name           string `json:"name"`
		WordingVersion uint64 `json:"wording_version"`
	} `json:"metadata"`
	Incremental bool `json:"incremental"`
	Operation   struct {
		Injection string `json:"injection"`
	} `json:"operation"`
	Settings string `json:"engine_settings"`
}

func caseContent(v json.RawMessage, text bool) thinkthen.Content {
	if text && len(v) > 0 && v[0] == '"' {
		var s string
		if json.Unmarshal(v, &s) == nil {
			return thinkthen.Content{Kind: thinkthen.ContentKindText, Text: s}
		}
	}
	return thinkthen.Content{Kind: thinkthen.ContentKindJson, Json: v}
}
func completeCase(raw []byte) string {
	var d nativeDocument
	if err := json.Unmarshal(raw, &d); err != nil {
		panic(err)
	}
	e, err := thinkthen.NewWith(d.Settings)
	if err != nil {
		out, _ := json.Marshal(normalizedError(err))
		return string(out)
	}
	defer e.Close()
	role := thinkthen.LoadAtomic
	var question map[string]json.RawMessage
	_ = json.Unmarshal(d.Question, &question)
	switch d.Verb {
	case "annotate":
		role = thinkthen.LoadSet
	case "rank":
		role = thinkthen.LoadRank
		if _, ok := question["questions"]; ok {
			role = thinkthen.LoadRankSet
		}
	case "find":
		role = thinkthen.LoadFind
	case "recognize":
		role = thinkthen.LoadRecognize
	case "relate":
		role = thinkthen.LoadRelate
	case "choose":
		if _, ok := question["options"]; !ok {
			role = thinkthen.LoadDynamicChoose
		}
	}
	q := thinkthen.SavedQuestion(role, d.Question)
	if d.Verb == "find" && string(question["none"]) == "true" {
		spec := thinkthen.FindQuestion(caseContent(question["find"], true))
		spec.None = true
		if d.Metadata.Name != "" {
			spec.Author = thinkthen.Some(thinkthen.QuestionAuthor{Name: thinkthen.Some(d.Metadata.Name), WordingVersion: thinkthen.Some(d.Metadata.WordingVersion)})
		}
		q = thinkthen.Asked(spec)
	}
	if d.Raw != nil {
		q = thinkthen.SavedQuestion(role, json.RawMessage(*d.Raw))
	}
	switch d.Loader {
	case "load_named", "named":
		q = thinkthen.NamedQuestion(role, d.Reference)
	case "load_reference", "reference":
		q = thinkthen.QuestionReference(role, d.Reference)
	case "load", "file":
		q = thinkthen.QuestionFile(d.Reference)
	}
	if d.QuestionForm == "file" {
		q = thinkthen.QuestionFile("fixture-question.json")
	}
	source := thinkthen.Records(nil)
	if len(d.Paths) > 0 {
		units := []thinkthen.SourceUnit{"", thinkthen.SourceUnitLine, thinkthen.SourceUnitWindow, thinkthen.SourceUnitFile, thinkthen.SourceUnitImageFile, thinkthen.SourceUnitJSONL}
		source = thinkthen.SourceFiles(thinkthen.FileSource{Paths: d.Paths, Unit: units[d.Unit], Window: d.Window, ImageReader: d.ImageReader})
	} else {
		records := []thinkthen.RecordInput{}
		for _, item := range d.Items {
			r := thinkthen.RecordInput{}
			if !d.ImageOnly {
				r.Original = thinkthen.Some(caseContent(item, d.Text))
			}
			if d.ContextPresent {
				r.Context = thinkthen.Some(caseContent(d.Context, true))
			}
			for _, path := range d.ImagePaths {
				b, err := os.ReadFile(path)
				if err != nil {
					panic(err)
				}
				r.Images = append(r.Images, thinkthen.ImageInput{Media: thinkthen.MediaPng, Bytes: b})
			}
			records = append(records, r)
		}
		source = thinkthen.Records(records)
	}
	ctx := context.Background()
	if d.Operation.Injection == "cancel_token" {
		var cancel context.CancelFunc
		ctx, cancel = context.WithCancel(ctx)
		cancel()
	}
	if d.Operation.Injection == "expired_deadline" {
		var cancel context.CancelFunc
		ctx, cancel = context.WithTimeout(ctx, 0)
		defer cancel()
	}
	controls := thinkthen.CallControls{Attempts: true}
	if len(d.SharedContext) != 0 && string(d.SharedContext) != "null" {
		controls.Context = thinkthen.Some(caseContent(d.SharedContext, true))
	}
	var out map[string]any
	if d.Incremental {
		switch d.Verb {
		case "decide":
			out = normalizedBatch(e.DecideBatch(ctx, q, source, controls))
		case "choose":
			out = normalizedBatch(e.ChooseBatch(ctx, q, source, controls))
		case "tag":
			out = normalizedBatch(e.TagBatch(ctx, q, source, controls))
		case "score":
			out = normalizedBatch(e.ScoreBatch(ctx, q, source, controls))
		case "filter":
			out = normalizedBatch(e.FilterBatch(ctx, q, source, controls))
		case "annotate":
			out = normalizedBatch(e.AnnotateBatch(ctx, q, source, controls))
		default:
			panic("function has no batch entry")
		}
	} else {
		switch d.Verb {
		case "decide":
			call, err := e.DecideComplete(ctx, q, source, controls)
			out = normalizedCall(call, err)
		case "choose":
			call, err := e.ChooseComplete(ctx, q, source, controls)
			out = normalizedCall(call, err)
		case "tag":
			call, err := e.TagComplete(ctx, q, source, controls)
			out = normalizedCall(call, err)
		case "score":
			call, err := e.ScoreComplete(ctx, q, source, controls)
			out = normalizedCall(call, err)
		case "filter":
			call, err := e.FilterComplete(ctx, q, source, controls)
			out = normalizedCall(call, err)
		case "rank":
			call, err := e.RankComplete(ctx, q, source, controls)
			out = normalizedCall(call, err)
		case "find":
			call, err := e.FindComplete(ctx, q, source, controls)
			out = normalizedCall(call, err)
		case "annotate":
			call, err := e.AnnotateComplete(ctx, q, source, controls)
			out = normalizedCall(call, err)
		case "recognize":
			call, err := e.RecognizeComplete(ctx, q, source, controls)
			out = normalizedCall(call, err)
		case "relate":
			call, err := e.RelateComplete(ctx, q, source, controls)
			out = normalizedCall(call, err)
		default:
			panic("unknown named function")
		}
	}
	text, err := json.Marshal(out)
	if err != nil {
		panic(fmt.Sprint(err))
	}
	return string(text)
}
