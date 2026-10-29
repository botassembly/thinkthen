package consumer

import (
	"encoding/json"
	"errors"
	"fmt"
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

// Feature: installed typed constructors honor every shared option admission case.
func TestSharedConstructionOptions(t *testing.T) {
	raw, err := os.ReadFile(filepath.Join(os.Getenv("THINKTHEN_CASES"), "../construction/options.json"))
	if err != nil {
		t.Fatal(err)
	}
	var samples []struct {
		apiCase
		ExpectedError string `json:"expected_error"`
	}
	if err := json.Unmarshal(raw, &samples); err != nil {
		t.Fatal(err)
	}
	for index, fixture := range samples {
		t.Run(fmt.Sprint(index), func(t *testing.T) {
			sample := fixture.apiCase
			if reason := sample.NotApplicable["go"]; reason != "" {
				t.Skip("not applicable: " + reason)
			}
			// Public saved-entry admission checks shapes the typed constructors cannot express.
			var originals []struct {
				Request struct{ Question json.RawMessage }
			}
			if err := json.Unmarshal(raw, &originals); err != nil {
				t.Fatal(err)
			}
			payload, err := json.Marshal(map[string]any{"entries": map[string]any{"case": map[string]any{"question": originals[index].Request.Question}}})
			if err != nil {
				t.Fatal(err)
			}
			_, admission := tt.ParseToolbox(payload)
			if fixture.ExpectedError != "" {
				var failure *tt.Error
				if !errors.As(admission, &failure) || failure.Code().String() != fixture.ExpectedError {
					t.Fatalf("admission: %v", admission)
				}
				return
			}
			if admission != nil {
				t.Fatal(admission)
			}
			if err := json.Unmarshal(sample.ExpectedData, &sample.Expected); err != nil {
				t.Fatal(err)
			}
			cfg := tt.Config{Backend: tt.Stub, Batch: sample.Client.Batch}
			cfg.Retry = retryFor(sample.Client.Retry)
			runTypedCase(t, apiClientFor(t, cfg), sample)
		})
	}
}
