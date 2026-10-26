package consumer

import (
	"os"
	"regexp"
	"strconv"
	"strings"
	"testing"

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
