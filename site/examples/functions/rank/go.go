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
	order, err := call.RankAnswer()
	if err != nil {
		log.Fatal(err)
	}
	if !slices.Equal(order, []uint64{1, 3, 2, 0}) {
		log.Fatal("expected the outage first")
	}
}
