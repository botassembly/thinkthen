//go:build thinkthen_system

package thinkthen

/*
#cgo pkg-config: thinkthen-c
#include "thinkthen.h"
*/
import "C"

var library []byte

func installedABI() uint32 { return uint32(C.thinkthen_abi_version()) }
