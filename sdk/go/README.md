# Go SDK

The Go SDK asks questions about text from your Go code. Its first function, decide, answers a yes-or-no question with yes (`true`), no (`false`) or not sure.

Install it with Go 1.23 or later, cgo and a C compiler: `go get github.com/botassembly/thinkthen/sdk/go/thinkthen@v0.2.0`. The module carries its own engine. The client reads your key from `THINKTHEN_API_KEY`. [Configuration](https://thinkthen.dev/configuration/) shows how to get one.

## Build on Windows

cgo on Windows needs a 64-bit gcc, such as MinGW-w64, on `PATH`; it can't use the Visual Studio compiler. Save the program below as `main.go` in a new folder, then run this in PowerShell in that folder:

```powershell
$env:CGO_ENABLED = "1"
go mod init example
go get github.com/botassembly/thinkthen/sdk/go/thinkthen@v0.2.0
go build -o example.exe .
.\example.exe
```

The module carries its own engine, so there is no library to install or add to `PATH`.

## Ask a question

Ask whether a support message requests a refund:

```go
package main

import (
	"context"
	"errors"
	"fmt"
	"log"

	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
)

func main() {
	client, err := tt.New(tt.Config{})
	if err != nil {
		log.Fatal(err)
	}
	defer client.Close()

	refundQuestion := tt.NewDecide("Does the customer request a refund?")
	isRefund, err := tt.Answer(client.Decide(context.Background(),
		refundQuestion, tt.Text("Please refund my order.")))
	switch {
	case errors.Is(err, tt.ErrUnsure):
		fmt.Println("not sure: send to a person")
	case err != nil:
		log.Fatal(err)
	default:
		fmt.Println(isRefund)
	}
}
```

Output:

```text
true
```

`true` means yes: the message asks for a refund.

## Not sure and failures

`tt.Answer` gives the answer in one step. It returns `tt.ErrUnsure` when decide is not sure, and the error when ThinkThen could not judge the message, so neither one reads as `false`. decide is not sure only when you set a low and a high bar, as in `tt.NewDecide("Does the customer request a refund?").WithThreshold(tt.Band(0.2, 0.8))`. Between the bars, the program above prints its not-sure line, so a person can read the message.

## Next

The [Go SDK page](https://thinkthen.dev/sdks/go/) shows all ten functions. The [Go guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/go.md) and the [API reference](https://pkg.go.dev/github.com/botassembly/thinkthen/sdk/go/thinkthen@v0.2.0) cover every option.

## Link an installed C library instead

On Linux or macOS, extract the matching [C archive](https://github.com/botassembly/thinkthen/releases/tag/v0.2.0) into `thinkthen` and install pkg-config. Save the example as `main.go` in your Go module, then run:

```sh
export PKG_CONFIG_PATH="$PWD/thinkthen/lib/pkgconfig${PKG_CONFIG_PATH:+:$PKG_CONFIG_PATH}"
case "$(uname -s)" in
    Linux) export LD_LIBRARY_PATH="$PWD/thinkthen/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" ;;
    Darwin) export DYLD_LIBRARY_PATH="$PWD/thinkthen/lib${DYLD_LIBRARY_PATH:+:$DYLD_LIBRARY_PATH}" ;;
esac
go build -tags thinkthen_system -o example .
./example
```

The `thinkthen_system` tag links the installed library through pkg-config, and the program finds it on the library path when it runs. Windows does not support this tag.
