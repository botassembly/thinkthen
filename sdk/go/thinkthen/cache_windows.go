package thinkthen

/*
#cgo windows LDFLAGS: -ladvapi32
#include <stdlib.h>
int tt_cache_secure(const char *path, int directory);
*/
import "C"

import (
	"crypto/sha256"
	"fmt"
	"os"
	"os/user"
	"path/filepath"
	"unsafe"
)

func checkCachePath(path string, directory bool) error {
	if _, err := os.Lstat(path); err != nil {
		return err
	}
	name := C.CString(path)
	defer C.free(unsafe.Pointer(name))
	kind := C.int(0)
	if directory {
		kind = 1
	}
	if C.tt_cache_secure(name, kind) == 0 {
		return fmt.Errorf("unsafe ThinkThen native cache owner or permissions: %s", path)
	}
	return nil
}

func temporaryCache() (string, error) {
	owner, err := user.Current()
	if err != nil {
		return "", err
	}
	path := filepath.Join(os.TempDir(), fmt.Sprintf("thinkthen-%x", sha256.Sum256([]byte(owner.Uid))))
	if err := os.Mkdir(path, 0700); err != nil && !os.IsExist(err) {
		return "", err
	}
	return path, checkCachePath(path, true)
}
