package consumer

import (
	"encoding/json"
	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
	"os"
	"path/filepath"
	"testing"
)

// Feature: the installed public API retains shared deterministic stub outcomes.
func TestSharedStub(t *testing.T) {
	path := os.Getenv("THINKTHEN_STUB_CASES")
	if path == "" {
		path = filepath.Join(os.Getenv("THINKTHEN_CASES"), "../construction/stub.json")
	}
	raw, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	var cases []apiCase
	if err = json.Unmarshal(raw, &cases); err != nil {
		t.Fatal(err)
	}
	if len(cases) == 0 {
		t.Fatal("construction corpus is empty")
	}
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	for i, sample := range cases {
		t.Run(string(rune('A'+i)), func(t *testing.T) {
			if err := json.Unmarshal(sample.ExpectedData, &sample.Expected); err != nil {
				t.Fatal(err)
			}
			utf8Expected(t, &sample)
			runTypedCase(t, c, sample)
		})
	}
}
