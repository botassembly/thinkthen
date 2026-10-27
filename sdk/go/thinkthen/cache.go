package thinkthen

import (
	"crypto/sha256"
	"encoding/hex"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"runtime"
)

// Embedding leaves no package file at runtime, so retain one verified cache copy.
func cachedLibrary() (string, error) {
	if len(library) == 0 {
		return "", fmt.Errorf("no bundled ThinkThen native engine for %s/%s", runtime.GOOS, runtime.GOARCH)
	}
	base, err := os.UserCacheDir()
	if err == nil {
		path, cacheErr := cacheLibrary(base)
		// Only direct filesystem errors permit fallback. Cache integrity failures
		// remain fatal even when an error wraps a filesystem failure.
		_, unavailable := cacheErr.(*os.PathError)
		if cacheErr == nil || !unavailable {
			return path, cacheErr
		}
	}
	base, err = temporaryCache()
	if err != nil {
		return "", err
	}
	return cacheLibrary(base)
}

func cacheLibrary(base string) (string, error) {
	digest := sha256.Sum256(library)
	dir := filepath.Join(base, "thinkthen", "native", hex.EncodeToString(digest[:]))
	directories := []string{base, filepath.Join(base, "thinkthen"), filepath.Join(base, "thinkthen", "native"), dir}
	// Reject unsafe existing paths before a failed creation can trigger fallback.
	for _, directory := range directories {
		if err := checkCachePath(directory, true); err != nil && !errors.Is(err, os.ErrNotExist) {
			return "", err
		}
	}
	if err := os.MkdirAll(dir, 0700); err != nil {
		return "", err
	}
	for _, directory := range directories {
		if err := checkCachePath(directory, true); err != nil {
			return "", err
		}
	}
	name := "library"
	if runtime.GOOS == "windows" {
		name += ".dll"
	}
	path := filepath.Join(dir, name)
	if err := checkCachePath(path, false); err != nil && !errors.Is(err, os.ErrNotExist) {
		return "", err
	}
	if data, err := os.ReadFile(path); err == nil {
		if sha256.Sum256(data) != digest {
			return "", errors.New("cached ThinkThen native engine checksum mismatch")
		}
		return path, nil
	} else if !errors.Is(err, os.ErrNotExist) {
		return "", err
	}
	file, err := os.CreateTemp(dir, ".library-")
	if err != nil {
		return "", err
	}
	defer os.Remove(file.Name())
	_, writeErr := file.Write(library)
	closeErr := file.Close()
	if writeErr != nil {
		return "", writeErr
	}
	if closeErr != nil {
		return "", closeErr
	}
	// Another process may have published the same immutable library first.
	if err := os.Rename(file.Name(), path); err != nil {
		data, readErr := os.ReadFile(path)
		if readErr != nil || sha256.Sum256(data) != digest {
			return "", err
		}
	}
	if err := checkCachePath(path, false); err != nil {
		return "", err
	}
	return path, nil
}
