//go:build !thinkthen_system && darwin && arm64

package thinkthen

import _ "embed"

//go:embed native/darwin-arm64/libthinkthen_ffi.dylib
var library []byte
