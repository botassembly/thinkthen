package main

import (
	"context"
	"log"
	"slices"

	tt "github.com/botassembly/thinkthen/libraries/go"
)

func main() {
	client, err := tt.NewClient(tt.EngineSettings{})
	if err != nil {
		log.Fatal(err)
	}
	defer client.Close()
	question := "Is this urgent?"
	inbox := []string{
		"Newsletter: our autumn catalog is here. " +
			"No reply needed.",
		"Our checkout page is down " +
			"and customers cannot pay",
		"Reminder: your invoice is due in 30 days",
		"Please send the signed quote by 5 pm today",
	}
	call, err := client.Rank(
		context.Background(),
		tt.TextQuestion(question),
		inbox,
		nil,
	)
	if err != nil {
		log.Fatal(err)
	}
	var order []int64
	for _, packet := range call.Packets {
		ranked, err := packet.AsSessionPacketRankAggregate()
		if err == nil {
			for _, one := range ranked.Value().Value {
				index, _ := one.Index().Value.Int64()
				order = append(order, index)
			}
		}
	}
	if !slices.Equal(order, []int64{1, 3, 2, 0}) {
		log.Fatal("expected the outage first")
	}
}
