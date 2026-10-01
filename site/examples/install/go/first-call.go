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
	isRefund, err := tt.Decide(
		ctx,
		question,
		"Please refund my order. It arrived broken.",
	)
	if err != nil {
		log.Fatal(err)
	}
	if isRefund.Value.Outcome != thinkthen.Yes {
		log.Fatal("expected yes")
	}

	refund := `{"decide": "` + question + `", ` +
		`"threshold": "0.2:0.8"}`
	isRefund, err = tt.Decide(
		ctx,
		refund,
		"I want to send this back.",
	)
	if err != nil {
		log.Fatal(err)
	}
	if isRefund.Value.Outcome != thinkthen.Unsure {
		log.Fatal("expected not sure")
	}
}
