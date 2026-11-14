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

	urgencyQuestion := tt.RankQuestion[string]{Question: "How urgent is the request?", Levels: []string{"routine", "urgent"}}
	messages := slices.Values([]string{
		"Please refund my order.",
		"Where is my parcel?",
		"I was charged twice for one invoice.",
	})
	for ranked, err := range client.Rank(
		ctx, urgencyQuestion, messages) {
		if err != nil {
			log.Fatal(err)
		}
		rank, err := ranked.Answer()
		if err != nil {
			log.Fatal(err)
		}
		fmt.Println(rank, ranked.Item())
	}
}
