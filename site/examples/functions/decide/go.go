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
		broken,
		nil,
	)
	if err != nil {
		log.Fatal(err)
	}
	var isRefund bool
	for _, packet := range call.Packets {
		row, err := packet.AsSessionPacketDecideRow()
		if err == nil {
			answer := row.Value().Value.Value().Value
			isRefund, _ = answer.Boolean()
		}
	}
	if !isRefund {
		log.Fatal("expected yes")
	}
}
