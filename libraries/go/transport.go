package thinkthen

import (
	"encoding/json"
	"reflect"
	"strings"
)

// Private encoding applies generated wire tags without publishing JSON methods.
func transportMarshal(value any) ([]byte, error) {
	normalized, err := transportValue(reflect.ValueOf(value))
	if err != nil {
		return nil, err
	}
	return json.Marshal(normalized)
}
func transportValue(value reflect.Value) (any, error) {
	if !value.IsValid() {
		return nil, nil
	}
	if (value.Kind() == reflect.Pointer || value.Kind() == reflect.Interface) && value.IsNil() {
		return nil, nil
	}
	if value.CanInterface() {
		if encoder, ok := value.Interface().(interface{ marshalJSON() ([]byte, error) }); ok {
			raw, err := encoder.marshalJSON()
			return json.RawMessage(raw), err
		}
		if _, ok := value.Interface().(json.Marshaler); ok {
			return value.Interface(), nil
		}
	}
	switch value.Kind() {
	case reflect.Pointer, reflect.Interface:
		return transportValue(value.Elem())
	case reflect.Struct:
		fields := make(map[string]any)
		for i := 0; i < value.NumField(); i++ {
			field := value.Type().Field(i)
			if !field.IsExported() {
				continue
			}
			tag := strings.Split(field.Tag.Get("json"), ",")
			key := tag[0]
			if key == "-" {
				continue
			}
			if key == "" {
				key = field.Name
			}
			member := value.Field(i)
			if len(tag) > 1 && tag[1] == "omitempty" && member.IsZero() {
				continue
			}
			normalized, err := transportValue(member)
			if err != nil {
				return nil, err
			}
			fields[key] = normalized
		}
		return fields, nil
	case reflect.Slice, reflect.Array:
		if value.Kind() == reflect.Slice && value.IsNil() {
			return nil, nil
		}
		items := make([]any, value.Len())
		for i := range items {
			item, err := transportValue(value.Index(i))
			if err != nil {
				return nil, err
			}
			items[i] = item
		}
		return items, nil
	case reflect.Map:
		if value.IsNil() {
			return nil, nil
		}
		// Caller-authored arbitrary JSON maps retain the standard encoder's rules.
		iterator := value.MapRange()
		normalized := make(map[string]any)
		if value.Type().Key().Kind() != reflect.String {
			return value.Interface(), nil
		}
		for iterator.Next() {
			item, err := transportValue(iterator.Value())
			if err != nil {
				return nil, err
			}
			normalized[iterator.Key().String()] = item
		}
		return normalized, nil
	default:
		return value.Interface(), nil
	}
}
