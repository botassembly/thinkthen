package main

import (
	"context"
	"fmt"
	"log"
	"slices"

	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
)

func main() {
	client, err := tt.New(tt.Config{})
	if err != nil {
		log.Fatal(err)
	}
	defer client.Close()
	ctx := context.Background()

	refundQuestion := tt.FilterQuestion{Question: "Does the customer request a refund?"}
	messages := slices.Values([]string{
		"Please refund my order.",
		"Where is my parcel?",
		"I was charged twice for one invoice.",
	})
	for kept, err := range client.Filter(
		ctx, refundQuestion, messages) {
		if err != nil {
			log.Fatal(err)
		}
		fmt.Println(kept)
	}
}
