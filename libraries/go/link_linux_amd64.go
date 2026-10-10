//go:build linux && amd64

package thinkthen

/*
#cgo CFLAGS: -I${SRCDIR}/native/x86_64-unknown-linux-gnu/include
#cgo LDFLAGS: ${SRCDIR}/native/x86_64-unknown-linux-gnu/lib/libthinkthen.a -lgcc_s -lutil -lrt -lpthread -lm -ldl -lc
*/
import "C"
