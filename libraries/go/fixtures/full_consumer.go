// Installed named calls for the shared conformance inventory.
package main

import (
	"bufio"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	tt "github.com/botassembly/thinkthen/libraries/go"
	"io"
	"os"
)

func decode(data []byte, value any) {
	if err := json.Unmarshal(data, value); err != nil {
		panic(err)
	}
}
func object(data []byte) map[string]json.RawMessage {
	var value map[string]json.RawMessage
	decode(data, &value)
	return value
}
func scalar[T any](data []byte) T { var value T; decode(data, &value); return value }
func batch(data []byte) tt.RequestBatch {
	if len(data) > 0 && data[0] == '"' {
		return tt.RequestBatchString(scalar[string](data))
	}
	return tt.RequestBatchInteger(scalar[uint64](data))
}
func settings(data []byte) tt.EngineSettings {
	fields := object(data)
	cache, hasCache := fields["cache"]
	delete(fields, "cache")
	batches, hasBatch := fields["batch"]
	delete(fields, "batch")
	plain, err := json.Marshal(fields)
	if err != nil {
		panic(err)
	}
	var value tt.EngineSettings
	decode(plain, &value)
	if hasCache {
		if len(cache) > 0 && cache[0] == '"' {
			value.Cache = tt.Ptr[tt.CacheDocument](tt.CacheDocumentString(scalar[string](cache)))
		} else {
			value.Cache = tt.Ptr[tt.CacheDocument](tt.DisabledCache(scalar[bool](cache)))
		}
	}
	if hasBatch {
		value.Batch = tt.Ptr(batch(batches))
	}
	return value
}
func options(data []byte) tt.RequestOptions {
	fields := object(data)
	batches, hasBatch := fields["batch"]
	delete(fields, "batch")
	thresholds := map[string]json.RawMessage{}
	for _, key := range []string{"threshold", "relation_threshold"} {
		if raw, ok := fields[key]; ok {
			thresholds[key] = raw
			delete(fields, key)
		}
	}
	plain, err := json.Marshal(fields)
	if err != nil {
		panic(err)
	}
	var value tt.RequestOptions
	decode(plain, &value)
	if hasBatch {
		value.Batch = tt.Ptr(batch(batches))
	}
	for key, raw := range thresholds {
		var threshold tt.RequestThreshold
		if len(raw) > 0 && raw[0] == '"' {
			threshold = tt.RequestThresholdString(scalar[string](raw))
		} else {
			threshold = tt.RequestThresholdNumber(scalar[float64](raw))
		}
		if key == "threshold" {
			value.Threshold = tt.Ptr(threshold)
		} else {
			value.RelationThreshold = tt.Ptr(threshold)
		}
	}
	return value
}
func item(data []byte) tt.RequestItem {
	fields := object(data)
	original, hasOriginal := fields["original"]
	delete(fields, "original")
	images, hasImages := fields["images"]
	delete(fields, "images")
	ctx, hasContext := fields["context"]
	delete(fields, "context")
	plain, err := json.Marshal(fields)
	if err != nil {
		panic(err)
	}
	var value tt.RequestItem
	decode(plain, &value)
	if hasOriginal {
		raw := object(original)
		if scalar[string](raw["kind"]) == "text" {
			value.Original = tt.Ptr[tt.RequestOriginal](tt.RequestOriginalText{Text: scalar[string](raw["text"])})
		} else {
			value.Original = tt.Ptr[tt.RequestOriginal](tt.RequestOriginalJson{Value: scalar[any](raw["value"])})
		}
	}
	if hasContext {
		var context tt.ContextSchema
		if len(ctx) > 0 && ctx[0] == '"' {
			context = tt.ContextSchemaString(scalar[string](ctx))
		} else {
			context = tt.ContextSchemaObject(scalar[map[string]any](ctx))
		}
		value.Context = tt.Ptr(context)
	}
	if hasImages {
		attachments := []tt.RequestImage{}
		for _, raw := range scalar[[]json.RawMessage](images) {
			fields := object(raw)
			if scalar[string](fields["kind"]) == "file" {
				var image tt.RequestImageFile
				decode(raw, &image)
				attachments = append(attachments, image)
			} else {
				var image tt.RequestImageBytes
				decode(raw, &image)
				attachments = append(attachments, image)
			}
		}
		value.Images = tt.Ptr(attachments)
	}
	return value
}

type feed struct {
	items []tt.RequestItem
	at    int
}

func (f *feed) Next(ctx context.Context) (any, error) {
	if err := ctx.Err(); err != nil {
		return nil, err
	}
	if f.at == len(f.items) {
		return nil, io.EOF
	}
	value := f.items[f.at]
	f.at++
	return value, nil
}
func main() {
	fixture := object(mustRead(os.Args[1]))
	question := object(fixture["question"])
	var q tt.RequestQuestion
	switch scalar[string](question["kind"]) {
	case "file":
		q = tt.RequestQuestionFile{Path: scalar[string](question["path"])}
	case "name":
		q = tt.RequestQuestionName{Name: scalar[string](question["name"])}
	case "reference":
		q = tt.RequestQuestionReference{Reference: scalar[string](question["reference"])}
	default:
		if err := os.WriteFile("authored-question.json", question["value"], 0600); err != nil {
			panic(err)
		}
		q = tt.RequestQuestionFile{Path: "authored-question.json"}
	}
	input := object(fixture["input"])
	var source any
	if scalar[string](input["kind"]) == "source" {
		source = tt.RequestSource{}
		var value tt.RequestSource
		decode(input["source"], &value)
		source = value
	} else {
		items := []tt.RequestItem{}
		for _, raw := range scalar[[]json.RawMessage](input["items"]) {
			items = append(items, item(raw))
		}
		switch scalar[string](input["kind"]) {
		case "entities":
			source = tt.RequestInputEntities{Items: items}
		case "units":
			source = tt.RequestInputUnits{Items: items}
		default:
			source = tt.RequestInputRecords{Items: items}
			if raw := fixture["incremental"]; raw != nil && scalar[bool](raw) {
				source = &feed{items: items}
			}
		}
	}
	client, err := tt.NewClient(settings([]byte(os.Args[2])))
	if err != nil {
		emit(tt.OwnedCall{}, err)
		return
	}
	defer client.Close()
	calls := map[string]func(context.Context, tt.RequestQuestion, any, *tt.RequestOptions) (tt.OwnedCall, error){"decide": client.Decide, "choose": client.Choose, "tag": client.Tag, "score": client.Score, "filter": client.Filter, "rank": client.Rank, "find": client.Find, "annotate": client.Annotate, "recognize": client.Recognize, "relate": client.Relate}
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	if raw := fixture["cancel"]; raw != nil && scalar[bool](raw) {
		cancel()
	}
	if raw := fixture["held_cancel"]; raw != nil && scalar[bool](raw) {
		go func() {
			if char, err := bufio.NewReader(os.Stdin).ReadByte(); err == nil && char == '!' {
				fmt.Println("cancel-fired")
				cancel()
			}
		}()
	}
	controls := options(fixture["options"])
	call, err := calls[scalar[string](fixture["verb"])](ctx, q, source, &controls)
	emit(call, err)
}
func mustRead(path string) []byte {
	data, err := os.ReadFile(path)
	if err != nil {
		panic(err)
	}
	return data
}
func emit(call tt.OwnedCall, err error) {
	if errors.Is(err, context.Canceled) {
		json.NewEncoder(os.Stdout).Encode(map[string]any{"admission": map[string]any{"code": 5, "message": err.Error()}})
		return
	}
	var failure *tt.Error
	if errors.As(err, &failure) {
		json.NewEncoder(os.Stdout).Encode(map[string]any{"admission": map[string]any{"code": failure.Code, "message": failure.Message}})
		return
	}
	var started *tt.SessionError
	if err != nil && !errors.As(err, &started) {
		panic(err)
	}
	json.NewEncoder(os.Stdout).Encode(map[string]any{"packets": call.Packets})
}
