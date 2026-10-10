// This installed caller reads shared fixture inputs and invokes only named calls.
package main

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	tt "github.com/botassembly/thinkthen/libraries/go"
	"os"
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
	calls := map[string]func(context.Context, tt.RequestQuestion, any, *tt.RequestOptions) (tt.OwnedCall, error){
		"decide": client.Decide, "choose": client.Choose, "tag": client.Tag, "score": client.Score, "filter": client.Filter, "rank": client.Rank, "find": client.Find, "annotate": client.Annotate, "recognize": client.Recognize, "relate": client.Relate,
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
			json.NewEncoder(os.Stdout).Encode(map[string]any{"call": call, "failed": started.Detail})
			return
		}
		if errors.As(err, &refused) {
			json.NewEncoder(os.Stdout).Encode(map[string]any{"call": call, "code": refused.Code})
			return
		}
		panic(fmt.Sprintf("%s: %v", fixture.Verb, err))
	}
	if err = json.NewEncoder(os.Stdout).Encode(call); err != nil {
		panic(err)
	}
}
