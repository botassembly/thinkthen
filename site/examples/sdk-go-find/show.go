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

	actionQuestion := tt.FindQuestion{Question: "Which line states the requested action?"}
	message := "Morgan works for Example Group.\n" +
		"Please refund my order."
	action, err := tt.Answer(
		client.Find(ctx, actionQuestion, message))
	if err != nil {
		log.Fatal(err)
	}
	if action == nil {
		fmt.Println("No line answers.")
	} else {
		fmt.Println(action.Text)
	}
}
