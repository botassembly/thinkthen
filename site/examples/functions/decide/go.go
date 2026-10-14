package main

import (
	"context"
	"log"

	tt "github.com/botassembly/thinkthen/libraries/go"
)

func main() {
	client, err := tt.NewClient(tt.EngineSettings{})
	if err != nil {
		log.Fatal(err)
	}
	defer client.Close()
	question := "Does the customer ask for a refund?"
	broken := "Please refund my order. It arrived broken."
	call, err := client.Decide(
		context.Background(),
		tt.TextQuestion(question),
		tt.TextInput(broken),
		nil,
	)
	if err != nil {
		log.Fatal(err)
	}
	refund, err := call.DecideAnswer()
	if err != nil {
		log.Fatal(err)
	}
	isRefund := refund != nil && *refund
	if !isRefund {
		log.Fatal("expected yes")
	}
}
