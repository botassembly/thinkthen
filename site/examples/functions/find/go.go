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
	question := "Which line gives the refund deadline?"
	policy := []string{
		"Returns need the original receipt.",
		"Refunds are issued within 30 days of purchase.",
		"Shipping is free on orders over $50.",
		"Gift cards cannot be exchanged for cash.",
	}
	call, err := client.Find(
		context.Background(),
		tt.TextQuestion(question),
		policy,
		nil,
	)
	if err != nil {
		log.Fatal(err)
	}
	var refundDeadline any
	for _, packet := range call.Packets {
		found, err := packet.AsSessionPacketFindAggregate()
		if err == nil {
			answer := found.Value().Value
			refundDeadline = answer.Value().Value
		}
	}
	if refundDeadline != policy[1] {
		log.Fatal("expected the refund line")
	}
}
