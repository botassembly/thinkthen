package consumer

import (
	"encoding/json"
	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
	"os"
	"regexp"
	"testing"
)

// Feature: public enum names and values follow the engine contract, never a binding list.
func TestEnumsMatchEngineContract(t *testing.T) {
	data, err := os.ReadFile(os.Getenv("THINKTHEN_CONTRACT"))
	if err != nil {
		t.Fatal(err)
	}
	// Other contract members are not enum arrays.
	var raw map[string]json.RawMessage
	if err := json.Unmarshal(data, &raw); err != nil {
		t.Fatal(err)
	}
	for key, name := range map[string]func(uint32) string{
		"functions": func(v uint32) string { return tt.Function(v).String() },
		"statuses":  func(v uint32) string { return tt.Status(v).String() },
		"backends":  func(v uint32) string { return tt.Backend(v).String() },
	} {
		var values []struct {
			Name  string
			Value uint32
		}
		if err := json.Unmarshal(raw[key], &values); err != nil {
			t.Fatal(err)
		}
		if len(values) == 0 {
			t.Fatalf("empty %s contract", key)
		}
		expected := make(map[uint32]string)
		for _, entry := range values {
			expected[entry.Value] = entry.Name
		}
		for value, expectedName := range expected {
			if actual := name(value); actual != expectedName {
				t.Fatalf("%s(%d): %q, want %q", key, value, actual, expectedName)
			}
		}
		for value := uint32(0); value <= uint32(len(values)+1); value++ {
			if _, found := expected[value]; !found && name(value) != "unknown" {
				t.Fatalf("extra %s value %d", key, value)
			}
		}
	}
}

// Feature: boundary tags follow the engine's TAGS until the schema declares those keys.
func TestBoundaryTagsMatchEngineSource(t *testing.T) {
	data, err := os.ReadFile(os.Getenv("THINKTHEN_BOUNDARY_SOURCE"))
	if err != nil {
		t.Fatal(err)
	}
	block := regexp.MustCompile(`(?s)const TAGS:.*?= \[(.*?)\n\];`).FindSubmatch(data)
	if len(block) != 2 {
		t.Fatal("engine TAGS declaration missing")
	}
	tags := regexp.MustCompile(`\(\s*"([A-Z]+)"\s*,`).FindAllSubmatch(block[1], -1)
	values := map[tt.BoundaryTag]bool{}
	for _, tag := range tags {
		var value tt.BoundaryTag
		if err := value.UnmarshalText(tag[1]); err != nil {
			t.Fatalf("missing boundary tag %s: %v", tag[1], err)
		}
		if value == 0 || values[value] {
			t.Fatal("duplicate boundary value")
		}
		values[value] = true
	}
	for _, value := range []tt.BoundaryTag{tt.Begin, tt.Inside, tt.End, tt.Single, tt.Out} {
		if !values[value] {
			t.Fatalf("extra public boundary value %d", value)
		}
	}
	if len(values) == 0 {
		t.Fatal("engine TAGS is empty")
	}
	var unknown tt.BoundaryTag
	if unknown.UnmarshalText([]byte("unknown")) == nil {
		t.Fatal("unknown boundary tag accepted")
	}
}
