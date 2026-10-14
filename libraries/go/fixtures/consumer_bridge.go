package main

import (
	tt "github.com/botassembly/thinkthen/libraries/go"
	"reflect"
	"regexp"
	"strings"
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

// Reconstruct conformance values through the installed typed public readers.
func fixtureValue(value any) any {
	if value == nil {
		return nil
	}
	v := reflect.ValueOf(value)
	if v.Kind() == reflect.Pointer {
		if v.IsNil() {
			return nil
		}
		return fixtureValue(v.Elem().Interface())
	}
	if v.Kind() == reflect.Slice {
		if v.IsNil() {
			return nil
		}
		items := make([]any, v.Len())
		for i := range items {
			items[i] = fixtureValue(v.Index(i).Interface())
		}
		return items
	}
	if v.Kind() == reflect.Map {
		if v.IsNil() {
			return nil
		}
		fields := map[string]any{}
		for _, key := range v.MapKeys() {
			fields[key.String()] = fixtureValue(v.MapIndex(key).Interface())
		}
		return fields
	}
	if v.Kind() != reflect.Struct {
		return value
	}
	object := map[string]any{}
	for i := 0; i < v.NumMethod(); i++ {
		method := v.Type().Method(i)
		reader := v.Method(i)
		if reader.Type().NumIn() != 0 {
			continue
		}
		if strings.HasPrefix(method.Name, "As") && reader.Type().NumOut() == 2 {
			result := reader.Call(nil)
			if result[1].IsNil() {
				return fixtureValue(result[0].Interface())
			}
		}
		if method.Name == "Value" && reader.Type().NumOut() == 2 {
			result := reader.Call(nil)
			if !result[1].IsNil() {
				panic(result[1].Interface())
			}
			return fixtureValue(result[0].Interface())
		}
		if reader.Type().NumOut() != 1 || !strings.HasPrefix(reader.Type().Out(0).Name(), "Presence[") {
			continue
		}
		if strings.HasSuffix(method.Name, "Answer") {
			continue
		}
		field := reader.Call(nil)[0]
		if err := field.FieldByName("Err"); !err.IsNil() {
			panic(err.Interface())
		}
		if !field.FieldByName("Present").Bool() {
			continue
		}
		key := strings.ToLower(fixtureFieldName.ReplaceAllString(method.Name, "${1}_${2}"))
		if field.FieldByName("Null").Bool() {
			object[key] = nil
		} else {
			object[key] = fixtureValue(field.FieldByName("Value").Interface())
		}
	}
	return object
}

var fixtureFieldName = regexp.MustCompile(`([a-z0-9])([A-Z])`)

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
