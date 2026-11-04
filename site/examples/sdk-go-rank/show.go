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

	urgencyQuestion := tt.NewRank(
		"How urgent is the request?",
		[]string{"routine", "urgent"})
	messages := tt.Texts(
		"Please refund my order.",
		"Where is my parcel?",
		"I was charged twice for one invoice.",
	)
	for ranked, err := range client.RankAll(
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
