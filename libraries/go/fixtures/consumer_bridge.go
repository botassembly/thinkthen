package main

import (
	tt "github.com/botassembly/thinkthen/libraries/go"
	"reflect"
)

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
