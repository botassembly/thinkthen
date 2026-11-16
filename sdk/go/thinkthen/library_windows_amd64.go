//go:build !thinkthen_system && windows && amd64

package thinkthen

import _ "embed"

//go:embed native/windows-amd64/thinkthen_ffi.dll
var library []byte
