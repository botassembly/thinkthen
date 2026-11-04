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
		"Does the customer request a refund?")
	_, err = tt.Answer(
		client.Decide(ctx, refundQuestion, tt.Text("")))
	var failure *tt.Error
	if errors.As(err, &failure) {
		fmt.Println(failure.Code())
	}
}
