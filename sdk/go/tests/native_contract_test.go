package consumer

import (
	"context"
	"crypto/sha256"
	"encoding/json"
	"errors"
	"fmt"
	"io/fs"
	"os"
	"os/exec"
	"os/user"
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
		_, err = client.Decide(context.Background(), thinkthen.DecideQuestion{Question: "Ready?"}, any(nil))
		if !errors.Is(err, thinkthen.CodeRecordMissing) {
			t.Fatal("accepted engine did not forward the scalar failure")
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
		if _, ok := err.(*thinkthen.Error); !ok {
			t.Fatalf("loader error is not *thinkthen.Error: %T", err)
		}
		if err == nil || !errors.Is(err, thinkthen.CodeLocalIO) || errors.Is(err, thinkthen.CodeUsageABI) || !strings.Contains(err.Error(), path) {
			t.Fatalf("wrong load diagnostic: %v", err)
		}
		if strings.Count(err.Error(), "cannot load native engine") != 1 || (runtime.GOOS == "windows" && !strings.Contains(err.Error(), "system error")) {
			t.Fatalf("load diagnostic gives no reason: %v", err)
		}
		t.Log(err)
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
		command := exec.Command(os.Args[0], "-test.run=^TestNativeLoadFailure$", "-test.v")
		command.Env = append(os.Environ(), "THINKTHEN_MISSING_NATIVE_CHILD="+path, "THINKTHEN_LIBRARY="+path)
		output, err := command.CombinedOutput()
		if err != nil {
			t.Fatalf("%v\n%s", err, output)
		}
		t.Logf("%s", output)
	}
}

// Windows folders have no Unix mode, so these helpers set their access lists.
const everyone = "*S-1-1-0"

func icacls(t *testing.T, arguments ...string) {
	t.Helper()
	if output, err := exec.Command("icacls", arguments...).CombinedOutput(); err != nil {
		t.Fatalf("icacls %v: %v\n%s", arguments, err, output)
	}
}

func currentSID(t *testing.T) string {
	t.Helper()
	owner, err := user.Current()
	if err != nil {
		t.Fatal(err)
	}
	return owner.Uid
}

// A Windows root is short, because the fallback engine path must fit the loader's
// MAX_PATH, and private, because a folder inherits its parent's access list.
func cacheRoot(t *testing.T) string {
	t.Helper()
	if runtime.GOOS != "windows" {
		return t.TempDir()
	}
	root, err := os.MkdirTemp(os.Getenv("SystemDrive")+`\`, "tt-")
	if err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { os.RemoveAll(root) })
	icacls(t, root, "/inheritance:r", "/grant:r", "*"+currentSID(t)+":(OI)(CI)F", "*S-1-5-18:(OI)(CI)F", "*S-1-5-32-544:(OI)(CI)F")
	return root
}

func predictableCache(t *testing.T) string {
	t.Helper()
	if runtime.GOOS == "windows" {
		return fmt.Sprintf("thinkthen-%x", sha256.Sum256([]byte(currentSID(t))))
	}
	return fmt.Sprintf("thinkthen-%d", os.Geteuid())
}

func openCache(t *testing.T, path string, mode os.FileMode) {
	t.Helper()
	if runtime.GOOS == "windows" {
		icacls(t, path, "/grant", everyone+":(M)")
	} else if err := os.Chmod(path, mode); err != nil {
		t.Fatal(err)
	}
}

func closeCache(t *testing.T, path string) {
	t.Helper()
	if runtime.GOOS == "windows" {
		icacls(t, path, "/remove:g", everyone)
	} else if err := os.Chmod(path, 0700); err != nil {
		t.Fatal(err)
	}
}

// cacheStillWritable reports whether this token can publish a file under a denied
// cache anyway, as an elevated token with backup and restore privileges enabled can.
func cacheStillWritable(t *testing.T, path string) bool {
	t.Helper()
	probe := filepath.Join(path, "write-probe")
	defer os.RemoveAll(probe)
	if err := os.MkdirAll(probe, 0700); err != nil {
		return false
	}
	return os.WriteFile(filepath.Join(probe, "file"), nil, 0600) == nil
}

func denyCacheWrites(t *testing.T, path string) {
	t.Helper()
	if runtime.GOOS == "windows" {
		// Inherited, so a subfolder made by restore privilege is denied as well.
		icacls(t, path, "/deny", "*"+currentSID(t)+":(OI)(CI)(AD,WD)")
		return
	}
	if err := os.Chmod(path, 0500); err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { os.Chmod(path, 0700) })
}

// Feature: unavailable user caches use a private per-user temporary cache.
func TestNativeTemporaryCache(t *testing.T) {
	if os.Getenv("THINKTHEN_TEMP_CACHE_CHILD") == "1" {
		client, err := thinkthen.New(thinkthen.Config{Backend: thinkthen.Stub})
		if err != nil {
			t.Fatal(err)
		}
		defer client.Close()
		row, err := client.Decide(context.Background(), thinkthen.DecideQuestion{Question: "Ready?"}, "evidence")
		if err != nil || row.Status() != thinkthen.Answered {
			t.Fatalf("fallback did not answer: %v", err)
		}
		return
	}
	for _, mode := range []string{"unset", "readonly", "long", "occupied", "group-writable"} {
		t.Run(mode, func(t *testing.T) {
			root := cacheRoot(t)
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
			if mode == "group-writable" {
				openCache(t, cache, 0775)
			}
			if mode == "long" {
				cache = filepath.Join(root, strings.Repeat("x", 256))
				home = cache
			}
			writable := false
			if mode == "readonly" {
				if os.Geteuid() == 0 {
					t.Skip("root can write read-only folders")
				}
				denyCacheWrites(t, cache)
				writable = cacheStillWritable(t, cache)
			}
			if mode == "occupied" {
				if err := os.WriteFile(filepath.Join(root, predictableCache(t)), []byte("foreign occupant"), 0600); err != nil {
					t.Fatal(err)
				}
			}
			command := exec.Command(os.Args[0], "-test.run=^TestNativeTemporaryCache$")
			for _, entry := range os.Environ() {
				key, _, _ := strings.Cut(entry, "=")
				switch strings.ToUpper(key) {
				case "HOME", "XDG_CACHE_HOME", "TMPDIR", "THINKTHEN_LIBRARY", "LOCALAPPDATA", "TMP", "TEMP":
				default:
					command.Env = append(command.Env, entry)
				}
			}
			command.Env = append(command.Env, "THINKTHEN_TEMP_CACHE_CHILD=1", "TMPDIR="+root, "TMP="+root, "TEMP="+root)
			if mode != "unset" && mode != "occupied" {
				command.Env = append(command.Env, "XDG_CACHE_HOME="+cache, "LOCALAPPDATA="+cache)
				if runtime.GOOS == "darwin" {
					command.Env = append(command.Env, "HOME="+home)
				}
			}
			if output, err := command.CombinedOutput(); err != nil {
				t.Fatalf("%v: %s", err, output)
			}
			folders, err := filepath.Glob(filepath.Join(root, "thinkthen-*"))
			if writable {
				// This token can still write the user cache, so the engine belongs there and
				// no fallback is made. The other modes prove the fallback itself.
				library := map[bool]string{true: "library.dll", false: "library"}[runtime.GOOS == "windows"]
				assets, globErr := filepath.Glob(filepath.Join(cache, "thinkthen", "native", "*", library))
				if err != nil || globErr != nil || len(folders) != 0 || len(assets) != 1 {
					t.Fatalf("a writable cache was not used: temp %v %v, cache %v %v", folders, err, assets, globErr)
				}
				t.Log("this token writes past the deny entry; the engine was published in the user cache")
				return
			}
			if mode == "occupied" {
				var private []string
				for _, folder := range folders {
					if info, err := os.Lstat(folder); err == nil && info.IsDir() {
						private = append(private, folder)
					}
				}
				folders = private
			}
			if err != nil || len(folders) != 1 {
				// Name where the child put its engine, so a Windows token difference shows.
				for _, top := range []string{root, cache} {
					filepath.WalkDir(top, func(path string, entry fs.DirEntry, walkErr error) error {
						t.Logf("after child: %s %v", path, walkErr)
						return nil
					})
				}
				t.Fatalf("missing private fallback: %v %v", folders, err)
			}
			info, err := os.Lstat(folders[0])
			// Windows folders carry no mode; the check that refuses a broader
			// list is what the group-writable case and the retry below exercise.
			if err != nil || (runtime.GOOS != "windows" && info.Mode().Perm() != 0700) {
				t.Fatalf("fallback permissions: %v %v", info, err)
			}
			library := "library"
			if runtime.GOOS == "windows" {
				library = "library.dll"
			}
			assets, err := filepath.Glob(filepath.Join(folders[0], "thinkthen", "native", "*", library))
			if err != nil || len(assets) != 1 {
				t.Fatalf("fallback did not publish one engine: %v %v", assets, err)
			}
			// A predictable fallback with broader permissions is left untouched.
			openCache(t, folders[0], 0755)
			retry := exec.Command(command.Path, command.Args[1:]...)
			retry.Env = command.Env
			output, runErr := retry.CombinedOutput()
			closeCache(t, folders[0])
			if runErr != nil {
				t.Fatalf("private replacement failed: %v %s", runErr, output)
			}
			if all, err := filepath.Glob(filepath.Join(root, "thinkthen-*", "thinkthen", "native", "*", library)); err != nil || len(all) != 2 {
				t.Fatalf("private replacement did not publish its own engine: %v %v", all, err)
			}
		})
	}
}

// Feature: explicit native library overrides never use the library search path.
func TestNativeRelativeOverride(t *testing.T) {
	if os.Getenv("THINKTHEN_RELATIVE_CHILD") == "1" {
		_, err := thinkthen.New(thinkthen.Config{Backend: thinkthen.Stub})
		if !errors.Is(err, thinkthen.CodeLocalIO) || !strings.Contains(err.Error(), "must be an absolute path") {
			t.Fatalf("relative override accepted or misreported: %v", err)
		}
		return
	}
	for _, path := range []string{"library", "./library", "sub/library"} {
		command := exec.Command(os.Args[0], "-test.run=^TestNativeRelativeOverride$")
		command.Env = append(os.Environ(), "THINKTHEN_RELATIVE_CHILD=1", "THINKTHEN_LIBRARY="+path)
		if output, err := command.CombinedOutput(); err != nil {
			t.Fatalf("%v: %s", err, output)
		}
	}
}
