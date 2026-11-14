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

	topicsQuestion := tt.TagQuestion[string]{Question: "Which topics appear?", Labels: []string{"billing", "shipping"}}
	message :=
		"I was charged twice and my parcel is late again."
	topics, err := tt.Answer(
		client.Tag(ctx, topicsQuestion, message))
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println(topics)
}
