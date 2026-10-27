package consumer

import (
	"context"
	"encoding/json"
	"errors"
	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
	"math"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

type failedEncoding struct{ message string }

func (v failedEncoding) MarshalJSON() ([]byte, error) { return nil, errors.New(v.message) }

// Feature: host conversion errors use redacted engine messages and cannot open a call.
func TestHostConversionErrorsAreRedacted(t *testing.T) {
	secret := "synthetic-encoding-secret"
	_, err := tt.JSON(failedEncoding{secret})
	if !errors.Is(err, tt.CodeRecordJSON) || errors.Is(err, tt.ErrUsage) || strings.Contains(err.Error(), secret) {
		t.Fatal("host conversion exposed its message")
	}
	var cause *json.MarshalerError
	if !errors.As(err, &cause) || cause.Err.Error() != secret {
		t.Fatal("record conversion lost the JSON cause")
	}
	for _, retry := range []tt.Retry{
		{MaxRetries: -1}, {InitialBackoff: time.Nanosecond}, {AttemptTimeout: -time.Millisecond},
	} {
		_, err := tt.New(tt.Config{Backend: tt.Stub, APIKey: tt.APIKey(secret), Retry: &retry})
		if !errors.Is(err, tt.CodeUsageSettings) || strings.Contains(err.Error(), secret) {
			t.Fatal("retry conversion did not preserve redacted engine error")
		}
	}
}

// Feature: encoding scalar JSON integers never rounds their evidence through float64.
func TestScalarJSONIntegersKeepExactValues(t *testing.T) {
	fixture := filepath.Join(t.TempDir(), "numbers.json")
	data := `{"replies":[{"contains":"9007199254740993","probability":0.9},{"contains":"-9223372036854775808","probability":0.9},{"contains":"18446744073709551615","probability":0.9},{"probability":0.1}]}`
	if err := os.WriteFile(fixture, []byte(data), 0600); err != nil {
		t.Fatal(err)
	}
	b := startBackend(t, fixture)
	batch := 1
	c := apiClientFor(t, tt.Config{URL: b.url, APIKey: tt.APIKey("loopback-only"), Batch: &batch})
	for _, value := range []any{json.Number("9007199254740993"), int64(math.MinInt64), uint64(math.MaxUint64)} {
		record, err := tt.JSON(value)
		if err != nil {
			t.Fatal(err)
		}
		row, err := c.Decide(context.Background(), tt.NewDecide("Ready?"), record)
		if err != nil {
			t.Fatal(err)
		}
		if probability, ok := row.Probability(); !ok || probability != 0.9 {
			t.Fatal("scalar JSON integer evidence changed")
		}
	}
	if got := b.command(t, "count"); got != "3" {
		t.Fatal("scalar records did not each reach the backend")
	}
}
