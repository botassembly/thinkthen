package main

import (
	"context"
	"log"

	thinkthen "github.com/botassembly/thinkthen/libraries/go"
)

func main() {
	tt, err := thinkthen.New()
	if err != nil {
		log.Fatal(err)
	}
	defer tt.Close()
	ctx := context.Background()

	question := "Does the customer ask for a refund?"
	brokenIsRefund, err := tt.Decide(
		ctx,
		question,
		"Please refund my order. It arrived broken.",
	)
	if err != nil {
		log.Fatal(err)
	}
	thanksIsRefund, err := tt.Decide(
		ctx,
		question,
		"Thanks for the quick help yesterday!",
	)
	if err != nil {
		log.Fatal(err)
	}
	if brokenIsRefund.Value.Outcome != thinkthen.Yes {
		log.Fatal("expected yes")
	}
	if thanksIsRefund.Value.Outcome != thinkthen.No {
		log.Fatal("expected no")
	}
}
