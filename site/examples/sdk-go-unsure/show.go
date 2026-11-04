package main

import (
	"context"
	"errors"
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

	refundQuestion := tt.NewDecide(
		"Does the customer request a refund?",
	).WithThreshold(tt.Band(0.2, 0.8))
	message := tt.Text("I was charged twice for one invoice.")
	isRefund, err := tt.Answer(
		client.Decide(ctx, refundQuestion, message))
	switch {
	case errors.Is(err, tt.ErrUnsure):
		fmt.Println("Not sure. Ask a person.")
	case err != nil:
		log.Fatal(err)
	case isRefund:
		fmt.Println("Send to refunds.")
	default:
		fmt.Println("Leave in the inbox.")
	}
}
