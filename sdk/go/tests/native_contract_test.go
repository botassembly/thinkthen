package consumer

import (
	"context"
	"encoding/json"
	"errors"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
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
			if err := os.Unsetenv("THINKTHEN_LIBRARY"); err != nil {
				t.Fatal(err)
			}
			client, err := thinkthen.New(thinkthen.Config{Backend: thinkthen.Stub})
			if err != nil {
				t.Fatalf("ABI refusal prevented retry: %v", err)
			}
			client.Close()
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
		if err := os.Unsetenv("THINKTHEN_LIBRARY"); err != nil {
			t.Fatal(err)
		}
		client, err := thinkthen.New(thinkthen.Config{Backend: thinkthen.Stub})
		if err != nil {
			t.Fatalf("load failure prevented retry: %v", err)
		}
		client.Close()
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

// Feature: unavailable user caches use a private per-user temporary cache.
func TestNativeTemporaryCache(t *testing.T) {
	if runtime.GOOS == "windows" {
		t.Skip("Unix cache environment")
	}
	if os.Getenv("THINKTHEN_TEMP_CACHE_CHILD") == "1" {
		client, err := thinkthen.New(thinkthen.Config{Backend: thinkthen.Stub})
		if err != nil {
			t.Fatal(err)
		}
		defer client.Close()
		row, err := client.Decide(context.Background(), thinkthen.NewDecide("Ready?"), thinkthen.Text("evidence"))
		if err != nil || row.Status() != thinkthen.Answered {
			t.Fatalf("fallback did not answer: %v", err)
		}
		return
	}
	for _, mode := range []string{"unset", "readonly", "long"} {
		t.Run(mode, func(t *testing.T) {
			root := t.TempDir()
			cache := filepath.Join(root, "cache")
			home := filepath.Join(root, "home")
			if runtime.GOOS == "darwin" {
				cache = filepath.Join(home, "Library", "Caches")
				if err := os.MkdirAll(filepath.Dir(cache), 0700); err != nil {
					t.Fatal(err)
				}
			}
			if err := os.Mkdir(cache, 0700); err != nil {
				t.Fatal(err)
			}
			if mode == "long" {
				cache = filepath.Join(root, strings.Repeat("x", 256))
				home = cache
			}
			if mode == "readonly" {
				if os.Geteuid() == 0 {
					t.Skip("root can write read-only folders")
				}
				if err := os.Chmod(cache, 0500); err != nil {
					t.Fatal(err)
				}
				t.Cleanup(func() { os.Chmod(cache, 0700) })
			}
			command := exec.Command(os.Args[0], "-test.run=^TestNativeTemporaryCache$")
			for _, entry := range os.Environ() {
				key, _, _ := strings.Cut(entry, "=")
				if key != "HOME" && key != "XDG_CACHE_HOME" && key != "TMPDIR" && key != "THINKTHEN_LIBRARY" {
					command.Env = append(command.Env, entry)
				}
			}
			command.Env = append(command.Env, "THINKTHEN_TEMP_CACHE_CHILD=1", "TMPDIR="+root)
			if mode != "unset" {
				command.Env = append(command.Env, "XDG_CACHE_HOME="+cache)
				if runtime.GOOS == "darwin" {
					command.Env = append(command.Env, "HOME="+home)
				}
			}
			if output, err := command.CombinedOutput(); err != nil {
				t.Fatalf("%v: %s", err, output)
			}
			folders, err := filepath.Glob(filepath.Join(root, "thinkthen-*"))
			if err != nil || len(folders) != 1 {
				t.Fatalf("missing private fallback: %v %v", folders, err)
			}
			info, err := os.Lstat(folders[0])
			if err != nil || info.Mode().Perm() != 0700 {
				t.Fatalf("fallback permissions: %v %v", info, err)
			}
			assets, err := filepath.Glob(filepath.Join(folders[0], "thinkthen", "native", "*", "library"))
			if err != nil || len(assets) != 1 {
				t.Fatalf("fallback did not publish one engine: %v %v", assets, err)
			}
			// Private fallback roots reject permissions broader than 0700.
			if err := os.Chmod(folders[0], 0755); err != nil {
				t.Fatal(err)
			}
			retry := exec.Command(command.Path, command.Args[1:]...)
			retry.Env = command.Env
			output, runErr := retry.CombinedOutput()
			if err := os.Chmod(folders[0], 0700); err != nil {
				t.Fatal(err)
			}
			if runErr == nil || !strings.Contains(string(output), "cache owner or permissions") {
				t.Fatalf("insecure fallback accepted: %v %s", runErr, output)
			}
		})
	}
}
