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

	topicQuestion := tt.NewChoose("What is the message about?",
		[]string{"billing", "shipping", "other"})
	inbox := tt.Texts(
		"Please refund my order.",
		"Where is my parcel?",
		"I was charged twice for one invoice.",
	)
	// ChooseAll sends every message in one call and yields rows in input order.
	for message, err := range client.ChooseAll(
		context.Background(), topicQuestion, inbox,
	) {
		if err != nil {
			log.Fatal(err) // the call failed, not one message
		}
		topic, err := message.Answer()
		switch {
		case errors.Is(err, tt.ErrUnsure):
			fmt.Println(message.Position(), "unsure: send to a person")
		case err != nil:
			// prints the status, code and reason
			fmt.Println(message.Position(), message)
		default:
			fmt.Println(message.Position(), topic)
		}
	}
}
