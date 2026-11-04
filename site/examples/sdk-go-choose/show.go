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

	topicQuestion := tt.NewChoose(
		"What is the message about?",
		[]string{"billing", "shipping", "other"})
	message := tt.Text("Please refund my order.")
	topic, err := tt.Answer(
		client.Choose(ctx, topicQuestion, message))
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println(topic)
}
