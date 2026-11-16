//go:build !thinkthen_system && !((linux && amd64) || (linux && arm64) || (darwin && arm64) || (windows && amd64))

package thinkthen

var library []byte
