# ThinkThen for Go

Enable cgo and install a C compiler. Run `go get github.com/botassembly/thinkthen/sdk/go/thinkthen@v0.2.0`.

The example uses the stub backend and needs no key or service.

```go
package main

import (
    "context"
    "fmt"
    "log"

    thinkthen "github.com/botassembly/thinkthen/sdk/go/thinkthen"
)

func main() {
    client, err := thinkthen.New(thinkthen.Config{Backend: thinkthen.Stub})
    if err != nil {
        log.Fatal(err)
    }
    defer client.Close()

    row, err := client.Decide(context.Background(),
        thinkthen.NewDecide("Does the message request a refund?"),
        thinkthen.Text("Please refund this order."))
    if err != nil {
        log.Fatal(err)
    }
    if refund, ok := row.Value(); ok {
        fmt.Println(refund)
    }
}
```

`Error.Code().Family()` returns the engine’s error family. `errors.Is(err, thinkthen.ErrUsage)` matches usage errors. Unknown code families return an empty string.

[API reference](https://github.com/botassembly/thinkthen/tree/v0.2.0/sdk/go/thinkthen).

For a system library on Linux or macOS, extract the matching [C archive](../c/README.md) into `thinkthen` and install pkg-config. Save the example as `main.go` in your Go module, then run:

```sh
export PKG_CONFIG_PATH="$PWD/thinkthen/lib/pkgconfig${PKG_CONFIG_PATH:+:$PKG_CONFIG_PATH}"
case "$(uname -s)" in
    Linux) export LD_LIBRARY_PATH="$PWD/thinkthen/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" ;;
    Darwin) export DYLD_LIBRARY_PATH="$PWD/thinkthen/lib${DYLD_LIBRARY_PATH:+:$DYLD_LIBRARY_PATH}" ;;
esac
go build -tags thinkthen_system -o example .
./example
```

The `thinkthen_system` tag uses pkg-config to link the installed library. The loader uses the platform's library path when the program runs.
