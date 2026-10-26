//go:build darwin || linux

package thinkthen

import (
	"fmt"
	"os"
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
