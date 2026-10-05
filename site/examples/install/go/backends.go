package main

import (
	"context"
	"log"

	thinkthen "github.com/botassembly/thinkthen/libraries/go"
)

func main() {
	settings := []string{
		`{"backend":"typesafe"}`,
		`{"backend":"liquid"}`,
		`{"backend":"ollama","base_url":"http://localhost:11535/v1"}`,
	}
	for _, setting := range settings {
		tt, err := thinkthen.NewWith(setting)
		if err != nil {
			log.Fatal(err)
		}
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
		tt.Close()
	}
}
