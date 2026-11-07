//go:build !cgo

package thinkthen

// The SDK reaches its engine through cgo. Every file that depends on the engine
// needs the cgo build tag, so this name is the only error a build without cgo reports.
var _ = thinkthen_needs_cgo_set_CGO_ENABLED_1_and_install_a_C_compiler
