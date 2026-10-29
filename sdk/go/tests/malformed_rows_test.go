package consumer

import (
	"context"
	"errors"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"testing"
	"time"

	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
)

// Feature: malformed delivered rows fail with usage.abi through the public API.
func TestMalformedNativeRowsAreABIUsageErrors(t *testing.T) {
	if kind := os.Getenv("THINKTHEN_MALFORMED_ROW_CHILD"); kind != "" {
		client, err := tt.New(tt.Config{Backend: tt.Stub})
		if err != nil {
			t.Fatalf("current ABI fixture failed client admission: %v", err)
		}
		defer client.Close()
		record := tt.Text("evidence")
		if kind == "unanswered-answer" {
			record = tt.Missing()
		}
		row, err := client.Decide(context.Background(), tt.NewDecide("Ready?"), record)
		if !errors.Is(err, tt.CodeUsageABI) || !errors.Is(err, tt.ErrUsage) || row.Status() != 0 {
			t.Fatalf("malformed %s escaped: row %v error %v", kind, row, err)
		}
		return
	}
	directory := os.Getenv("THINKTHEN_MALFORMED_ROW_FIXTURES")
	if directory == "" {
		t.Fatal("THINKTHEN_MALFORMED_ROW_FIXTURES must identify the shared content fixtures")
	}
	basename := "libthinkthen_ffi.so"
	if runtime.GOOS == "darwin" {
		basename = "libthinkthen_ffi.dylib"
	}
	if runtime.GOOS == "windows" {
		basename = "thinkthen_ffi.dll"
	}
	for _, kind := range []string{"invalid-json", "unanswered-answer", "unknown-function"} {
		t.Run(kind, func(t *testing.T) {
			fixture := filepath.Join(directory, kind, basename)
			if _, err := os.Stat(fixture); err != nil {
				t.Fatal(err)
			}
			ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
			defer cancel()
			command := exec.CommandContext(ctx, os.Args[0], "-test.run=^TestMalformedNativeRowsAreABIUsageErrors$")
			command.Env = append(os.Environ(), "THINKTHEN_MALFORMED_ROW_CHILD="+kind, "THINKTHEN_LIBRARY="+fixture)
			if output, err := command.CombinedOutput(); err != nil {
				t.Fatalf("malformed row %s: %v\n%s", kind, err, output)
			}
		})
	}
}
