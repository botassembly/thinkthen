# ThinkThen for Go

ThinkThen answers typed questions about text. Ask for a yes or no, a label, a number, or records selected and ranked by meaning. Failed calls remain separate from answers.

## Install

```sh
go get github.com/botassembly/thinkthen/libraries/go@v0.2.0
```

## Example

Configure a model backend before running this example.

```go
package main

import (
    "context"
    "fmt"
    "log"
    tt "github.com/botassembly/thinkthen/libraries/go"
)

func main() {
    client, err := tt.NewClient(tt.EngineSettings{})
    if err != nil { log.Fatal(err) }
    defer client.Close()
    answer, err := client.DecideAnswer(context.Background(),
        tt.TextQuestion("Does this ask for a refund?"), "Please refund my order.", nil)
    if err != nil { log.Fatal(err) }
    fmt.Println(answer)
}
```

## Cost

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
