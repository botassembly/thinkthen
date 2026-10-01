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
	refund := `{"decide": "` + question + `", ` +
		`"threshold": "0.2:0.8"}`
	texts := []string{
		"Please refund my order. It arrived broken.",
		"Thanks for the quick help yesterday!",
		"I want to send this back.",
	}
	expected := []thinkthen.Outcome{
		thinkthen.Yes,
		thinkthen.No,
		thinkthen.Unsure,
	}
	for i, text := range texts {
		isRefund, err := tt.Decide(ctx, refund, text)
		if err != nil {
			log.Fatal(err)
		}
		if isRefund.Value.Outcome != expected[i] {
			log.Fatalf("expected %v for %q", expected[i], text)
		}
	}
}
