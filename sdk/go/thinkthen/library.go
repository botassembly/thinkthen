//go:build !thinkthen_system

package thinkthen

import _ "embed"

//go:embed native/library
var library []byte
