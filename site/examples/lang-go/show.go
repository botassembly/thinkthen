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

	refundQuestion := tt.DecideQuestion{Question: "Does the customer request a refund?"}
	isRefund, err := tt.Answer(client.Decide(context.Background(),
		refundQuestion, "Please refund my order."))
	switch {
	case errors.Is(err, tt.ErrUnsure):
		fmt.Println("not sure: send to a person")
	case err != nil:
		log.Fatal(err)
	default:
		fmt.Println(isRefund)
	}
}
