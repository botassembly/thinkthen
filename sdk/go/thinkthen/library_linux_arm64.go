//go:build !thinkthen_system && linux && arm64

package thinkthen

import _ "embed"

//go:embed native/linux-arm64/libthinkthen_ffi.so
var library []byte
