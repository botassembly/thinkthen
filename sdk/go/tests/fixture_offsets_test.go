package consumer

import (
	"encoding/json"
	"strings"
	"testing"
)

func tBackend(t *testing.T) string { parts := strings.Split(t.Name(), "/"); return parts[len(parts)-1] }
func utf8Expected(t *testing.T, s *apiCase) {
	t.Helper()
	for i := range s.Expected {
		row := &s.Expected[i]
		position := int(row.Position) - 1
		if position < 0 || position >= len(s.Request.Records) {
			continue
		}
		var text string
		if json.Unmarshal(s.Request.Records[position], &text) != nil {
			var record map[string]json.RawMessage
			_ = json.Unmarshal(s.Request.Records[position], &record)
			if evidence := s.Request.Question.Evidence; evidence != nil && strings.HasPrefix(*evidence, "/") {
				_ = json.Unmarshal(record[strings.TrimPrefix(*evidence, "/")], &text)
			}
		}
		if text == "" {
			continue
		}
		for _, raw := range []*json.RawMessage{&row.Answer, &row.Detail} {
			if len(*raw) == 0 {
				continue
			}
			var value any
			decoder := json.NewDecoder(strings.NewReader(string(*raw)))
			decoder.UseNumber()
			if err := decoder.Decode(&value); err != nil {
				t.Fatal(err)
			}
			byteOffsets(value, text)
			data, err := json.Marshal(value)
			if err != nil {
				t.Fatal(err)
			}
			*raw = data
		}
	}
}
func byteOffsets(value any, text string) {
	switch v := value.(type) {
	case map[string]any:
		start, s := v["start"].(json.Number)
		end, e := v["end"].(json.Number)
		if s && e {
			a, _ := start.Int64()
			b, _ := end.Int64()
			points := []rune(text)
			v["start"] = len(string(points[:a]))
			v["end"] = len(string(points[:b]))
		}
		for _, child := range v {
			byteOffsets(child, text)
		}
	case []any:
		for _, child := range v {
			byteOffsets(child, text)
		}
	}
}
