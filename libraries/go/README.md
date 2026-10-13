# ThinkThen for Go

ThinkThen puts ten functions on top of any model: decide, choose, tag, score, filter, rank, find, annotate, recognize and relate. Your code branches on typed answers, gets not sure instead of a guess, packs many records into few requests, and caches each question so you pay once. Tests replay saved answers with no key or network. Use Jev, OpenAI or a local model from every language and database.

```sh
go get github.com/botassembly/thinkthen/libraries/go@v0.2.0
```

Set your hosted backend key: `export THINKTHEN_API_KEY=...`.

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

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
