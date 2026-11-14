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

	urgencyQuestion := tt.ScoreQuestion[string]{Question: "How urgent is the request?", Levels: []string{"routine", "urgent"}}
	message := "Please refund my order."
	urgency, err := tt.Answer(
		client.Score(ctx, urgencyQuestion, message))
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println(urgency)
}
