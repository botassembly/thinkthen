//go:build thinkthen_system && windows

package thinkthen

var library []byte

func installedABI() uint32 { return 0 }

// System linkage requires process symbol lookup, which the Windows loader lacks.
var _ = thinkthen_system_is_not_supported_on_windows
