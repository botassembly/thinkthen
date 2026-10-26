package consumer

import (
	"context"
	"encoding/json"
	"errors"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"

	thinkthen "github.com/botassembly/thinkthen/sdk/go/thinkthen"
)

func TestNativeABIManifest(t *testing.T) {
	if os.Getenv("THINKTHEN_NATIVE_CHILD") == "1" {
		client, err := thinkthen.New(thinkthen.Config{Backend: thinkthen.Stub})
		if os.Getenv("THINKTHEN_NATIVE_ACCEPT") == "false" {
			if err == nil {
				client.Close()
				t.Fatal("incompatible engine accepted")
			}
			if !errors.Is(err, thinkthen.CodeUsageABI) {
				t.Fatal("loader lost ABI error")
			}
			return
		}
		if err != nil {
			t.Fatal(err)
		}
		defer client.Close()
		row, err := client.Decide(context.Background(), thinkthen.NewDecide("Ready?"), thinkthen.Missing())
		if err != nil || !errors.Is(row.Error(), thinkthen.CodeRecordMissing) {
			t.Fatal("accepted engine did not forward failed row")
		}
		return
	}
	path := os.Getenv("THINKTHEN_NATIVE_FIXTURES")
	raw, err := os.ReadFile(filepath.Join(path, "manifest.json"))
	if err != nil {
		t.Fatal(err)
	}
	var manifest struct {
		Cases []struct {
			Name, Library string
			Accept        bool
		}
	}
	if err = json.Unmarshal(raw, &manifest); err != nil {
		t.Fatal(err)
	}
	if len(manifest.Cases) == 0 {
		t.Fatal("native fixture manifest is empty")
	}
	for _, sample := range manifest.Cases {
		t.Run(sample.Name, func(t *testing.T) {
			command := exec.Command(os.Args[0], "-test.run=^TestNativeABIManifest$")
			command.Env = append(os.Environ(), "THINKTHEN_NATIVE_CHILD=1", "THINKTHEN_NATIVE_ACCEPT="+map[bool]string{true: "true", false: "false"}[sample.Accept], "THINKTHEN_LIBRARY="+sample.Library)
			if output, err := command.CombinedOutput(); err != nil {
				t.Fatalf("native fixture: %v\n%s", err, output)
			}
		})
	}
}

// Feature: a missing or broken engine reports a load failure and its attempted path.
func TestNativeLoadFailure(t *testing.T) {
	if path := os.Getenv("THINKTHEN_MISSING_NATIVE_CHILD"); path != "" {
		_, err := thinkthen.New(thinkthen.Config{Backend: thinkthen.Stub})
		if err == nil || !errors.Is(err, thinkthen.CodeLocalIO) || errors.Is(err, thinkthen.CodeUsageABI) || !strings.Contains(err.Error(), path) {
			t.Fatalf("wrong load diagnostic: %v", err)
		}
		return
	}
	path := filepath.Join(t.TempDir(), "missing-engine")
	for _, broken := range []bool{false, true} {
		if broken {
			if err := os.WriteFile(path, []byte("not a shared library"), 0600); err != nil {
				t.Fatal(err)
			}
		}
		command := exec.Command(os.Args[0], "-test.run=^TestNativeLoadFailure$")
		command.Env = append(os.Environ(), "THINKTHEN_MISSING_NATIVE_CHILD="+path, "THINKTHEN_LIBRARY="+path)
		if output, err := command.CombinedOutput(); err != nil {
			t.Fatalf("%v\n%s", err, output)
		}
	}
}
