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

[API reference](https://github.com/botassembly/thinkthen/tree/v0.2.0/sdk/go/thinkthen).
