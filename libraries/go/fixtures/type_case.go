package main

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"os"

	thinkthen "github.com/botassembly/thinkthen/libraries/go"
)

func main() {
	e, err := thinkthen.New()
	if err != nil {
		panic(err)
	}
	defer e.Close()
	request, err := io.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}
	value, err := e.Call(context.Background(), string(request))
	if err != nil {
		var failure *thinkthen.Error
		if !errors.As(err, &failure) {
			panic(err)
		}
		row, err := json.Marshal(map[string]any{"failed": map[string]any{"kind": failure.Kind, "code": failure.Code}})
		if err != nil {
			panic(err)
		}
		fmt.Println(string(row))
		return
	}
	fmt.Println(value)
}
