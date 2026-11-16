//go:build !thinkthen_system && linux && amd64

package thinkthen

import _ "embed"

//go:embed native/linux-amd64/libthinkthen_ffi.so
var library []byte
