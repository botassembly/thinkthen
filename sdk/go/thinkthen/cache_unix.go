//go:build !windows

package thinkthen

import (
	"fmt"
	"os"
	"path/filepath"
	"syscall"
)

func checkCachePath(path string, directory bool) error {
	info, err := os.Lstat(path)
	if err != nil {
		return err
	}
	owner, ok := info.Sys().(*syscall.Stat_t)
	if !ok || owner.Uid != uint32(os.Geteuid()) || info.Mode().Perm()&0022 != 0 ||
		(directory && !info.IsDir()) || (!directory && !info.Mode().IsRegular()) {
		return fmt.Errorf("unsafe ThinkThen native cache owner or permissions: %s", path)
	}
	return nil
}

func temporaryCache() (string, error) {
	path := filepath.Join(os.TempDir(), fmt.Sprintf("thinkthen-%d", os.Geteuid()))
	if err := os.Mkdir(path, 0700); err != nil && !os.IsExist(err) {
		return "", err
	}
	if err := checkCachePath(path, true); err == nil {
		info, err := os.Lstat(path)
		if err == nil && info.Mode().Perm() == 0700 {
			return path, nil
		}
	}
	return privateTemporaryCache()
}
