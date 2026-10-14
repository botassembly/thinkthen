// This installed caller reads shared fixture inputs and invokes only named calls.
package main

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	tt "github.com/botassembly/thinkthen/libraries/go"
	"os"
	"reflect"
)

func main() {
	var fixture struct {
		CachePath    string            `json:"cache_path"`
		Images       []byte            `json:"images"`
		ImagePaths   []string          `json:"image_paths"`
		ImageMedia   tt.ImageMedia     `json:"image_media"`
		ImageOnly    bool              `json:"image_only"`
		Verb         string            `json:"verb"`
		Batch        uint64            `json:"batch"`
		QuestionFile string            `json:"question_file"`
		Items        any               `json:"items"`
		Settings     tt.EngineSettings `json:"settings"`
		Options      tt.RequestOptions `json:"options"`
	}
	if err := json.NewDecoder(os.Stdin).Decode(&fixture); err != nil {
		panic(err)
	}
	// Fixture settings contain only ordinary scalar fields. Cache union is supplied
	// as a Go value, exactly as a caller configures an uncached engine.
	if fixture.Batch != 0 {
		fixture.Options.Batch = tt.Ptr[tt.RequestBatch](tt.RequestBatchInteger(fixture.Batch))
	}
	fixture.Settings.Cache = tt.Ptr[tt.CacheDocument](tt.DisabledCache(false))
	if fixture.CachePath != "" {
		fixture.Settings.Cache = tt.Ptr[tt.CacheDocument](tt.CacheDocumentString(fixture.CachePath))
	}
	client, err := tt.NewClient(fixture.Settings)
	if err != nil {
		panic(err)
	}
	defer client.Close()
	calls := map[string]func(context.Context, tt.RequestQuestion, any, *tt.RequestOptions) (tt.Call, error){
		"decide": func(ctx context.Context, q tt.RequestQuestion, input any, o *tt.RequestOptions) (tt.Call, error) {
			return client.Decide(ctx, q, fixtureInput(input), o)
		}, "choose": func(ctx context.Context, q tt.RequestQuestion, input any, o *tt.RequestOptions) (tt.Call, error) {
			return client.Choose(ctx, q, fixtureInput(input), o)
		}, "tag": func(ctx context.Context, q tt.RequestQuestion, input any, o *tt.RequestOptions) (tt.Call, error) {
			return client.Tag(ctx, q, fixtureInput(input), o)
		}, "score": func(ctx context.Context, q tt.RequestQuestion, input any, o *tt.RequestOptions) (tt.Call, error) {
			return client.Score(ctx, q, fixtureInput(input), o)
		}, "filter": func(ctx context.Context, q tt.RequestQuestion, input any, o *tt.RequestOptions) (tt.Call, error) {
			return client.Filter(ctx, q, fixtureInput(input), o)
		}, "rank": func(ctx context.Context, q tt.RequestQuestion, input any, o *tt.RequestOptions) (tt.Call, error) {
			return client.Rank(ctx, q, fixtureInput(input), o)
		}, "find": func(ctx context.Context, q tt.RequestQuestion, input any, o *tt.RequestOptions) (tt.Call, error) {
			return client.Find(ctx, q, fixtureInput(input), o)
		}, "annotate": func(ctx context.Context, q tt.RequestQuestion, input any, o *tt.RequestOptions) (tt.Call, error) {
			return client.Annotate(ctx, q, fixtureInput(input), o)
		}, "recognize": func(ctx context.Context, q tt.RequestQuestion, input any, o *tt.RequestOptions) (tt.Call, error) {
			return client.Recognize(ctx, q, fixtureInput(input), o)
		}, "relate": func(ctx context.Context, q tt.RequestQuestion, input any, o *tt.RequestOptions) (tt.Call, error) {
			return client.Relate(ctx, q, fixtureInput(input), o)
		},
	}
	input := fixture.Items
	if fixture.Images != nil || fixture.ImagePaths != nil {
		item := tt.RequestItem{}
		if !fixture.ImageOnly {
			item = tt.TextItem(fixture.Items.(string))
		}
		attachments := []tt.RequestImage{}
		if fixture.Images != nil {
			attachments = append(attachments, tt.BytesImage(fixture.Images, fixture.ImageMedia))
		}
		for _, path := range fixture.ImagePaths {
			attachments = append(attachments, tt.RequestImageFile{Path: path, Media: tt.Ptr(fixture.ImageMedia)})
		}
		item.Images = tt.Ptr(attachments)
		input = item
	}
	call, err := calls[fixture.Verb](context.Background(), tt.RequestQuestionFile{Path: fixture.QuestionFile}, input, &fixture.Options)
	if err != nil {
		var started *tt.SessionError
		var refused *tt.Error
		if errors.As(err, &started) {
			json.NewEncoder(os.Stdout).Encode(map[string]any{"call": fixtureCall(call), "failed": fixtureValue(started.Detail)})
			return
		}
		if errors.As(err, &refused) {
			json.NewEncoder(os.Stdout).Encode(map[string]any{"call": fixtureCall(call), "code": refused.Code})
			return
		}
		panic(fmt.Sprintf("%s: %v", fixture.Verb, err))
	}
	if err = json.NewEncoder(os.Stdout).Encode(fixtureCall(call)); err != nil {
		panic(err)
	}
}

func fixtureInput(value any) tt.RequestInput {
	if input, ok := value.(tt.RequestInput); ok {
		return input
	}
	if producer, ok := value.(tt.Producer); ok {
		return tt.FeedInput{Producer: producer}
	}
	if source, ok := value.(tt.RequestSource); ok {
		return tt.RequestInputSource{Source: source}
	}
	items := []tt.RequestItem{}
	rv := reflect.ValueOf(value)
	values := []any{value}
	if rv.IsValid() && (rv.Kind() == reflect.Slice || rv.Kind() == reflect.Array) {
		values = make([]any, rv.Len())
		for i := range values {
			values[i] = rv.Index(i).Interface()
		}
	}
	for _, value := range values {
		if item, ok := value.(tt.RequestItem); ok {
			items = append(items, item)
		} else if text, ok := value.(string); ok {
			items = append(items, tt.TextItem(text))
		} else {
			items = append(items, tt.JSONItem(value))
		}
	}
	return tt.RequestInputRecords{Items: items}
}
func fixtureValue(value interface{ JSONValue() (any, error) }) any {
	result, err := value.JSONValue()
	if err != nil {
		panic(err)
	}
	return result
}
func fixtureCall(call tt.Call) any {
	packets := make([]any, len(call.Packets))
	for i, packet := range call.Packets {
		packets[i] = fixtureValue(packet)
	}
	var terminal any
	if call.Terminal != nil {
		terminal = fixtureValue(call.Terminal)
	}
	return map[string]any{"Packets": packets, "Terminal": terminal}
}
