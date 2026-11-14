package main

import (
	"context"
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
	ctx := context.Background()

	kinds := tt.RecognizeQuestion{Kinds: []string{"person", "organization"}}
	message := "Morgan works for Example Group."
	names, err := tt.Answer(
		client.Recognize(ctx, kinds, message))
	if err != nil {
		log.Fatal(err)
	}
	for _, name := range names {
		fmt.Println(name.Kind, name.Text)
	}
}
