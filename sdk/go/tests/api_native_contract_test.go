package consumer

import (
	"encoding/json"
	"errors"
	"os"
	"regexp"
	"strconv"
	"strings"
	"testing"
	"time"

	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
)

// Feature: Go's code numbers and names match the generated engine header.
func TestCodesMatchGeneratedHeader(t *testing.T) {
	path := os.Getenv("THINKTHEN_HEADER")
	if path == "" {
		t.Fatal("THINKTHEN_HEADER must identify the generated engine header")
	}
	data, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	// Read generated enum assignments or defines, including INT32_C literals.
	pattern := regexp.MustCompile(`(?m)THINKTHEN_CODE_([A-Z]+)_([A-Z_]+)\s*(?:=|\s)\s*(?:INT32_C\()?([0-9]+)`)
	entries := pattern.FindAllStringSubmatch(string(data), -1)
	if len(entries) == 0 {
		t.Fatal("generated header lacks numeric registry declarations")
	}
	goSource, err := os.ReadFile(os.Getenv("THINKTHEN_GO_CODES"))
	if err != nil {
		t.Fatal(err)
	}
	exports := regexp.MustCompile(`(?m)^\s*Code[A-Za-z]+\s+Code\s*=\s*C\.THINKTHEN_CODE_([A-Z_]+)`).FindAllStringSubmatch(string(goSource), -1)
	if len(exports) != len(entries) {
		t.Fatal("public code registry has omissions or extra exports")
	}
	exported := map[string]bool{}
	for _, entry := range exports {
		if exported[entry[1]] {
			t.Fatal("duplicate public code")
		}
		exported[entry[1]] = true
	}
	seen := map[tt.Code]bool{}
	for _, entry := range entries {
		if !exported[entry[1]+"_"+entry[2]] {
			t.Fatal("public code missing")
		}
		n, err := strconv.ParseInt(entry[3], 10, 32)
		if err != nil {
			t.Fatal(err)
		}
		code := tt.Code(n)
		if seen[code] {
			t.Fatal("duplicate generated code number")
		}
		seen[code] = true
		name := strings.ToLower(entry[1]) + "." + strings.ReplaceAll(strings.ToLower(entry[2]), "_", "-")
		if code.String() != name || code.Error() != name {
			t.Fatal("Go code differs from generated registry")
		}
	}
	if !seen[tt.CodeRecordMissing] {
		t.Fatal("public missing-record code does not use the engine value")
	}
}

// Feature: error classification follows the native contract family for every code.
func TestErrorFamiliesMatchEngineContract(t *testing.T) {
	data, err := os.ReadFile(os.Getenv("THINKTHEN_CONTRACT"))
	if err != nil {
		t.Fatal(err)
	}
	var registry struct {
		Codes []struct {
			Value  int32
			Family string
		}
	}
	if err := json.Unmarshal(data, &registry); err != nil {
		t.Fatal(err)
	}
	for _, code := range registry.Codes {
		if got := tt.Code(code.Value).Family(); got != code.Family {
			t.Fatalf("family %d: %q; want %q", code.Value, got, code.Family)
		}
	}
	if tt.Code(0).Family() != "" || tt.Code(2147483647).Family() != "" {
		t.Fatal("unknown code acquired a family")
	}
	_, err = tt.New(tt.Config{Backend: tt.Stub, Retry: &tt.Retry{MaxRetries: 11, AttemptTimeout: time.Minute}})
	if !errors.Is(err, tt.ErrUsage) || !errors.Is(err, tt.CodeUsageSettings) {
		t.Fatalf("retry cap: %v", err)
	}
}
