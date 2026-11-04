package main

import (
	"context"
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

	form := tt.NewAnnotate(map[string]tt.AnnotationQuestion{
		"refund": tt.NewDecide(
			"Does the customer request a refund?"),
		"topic": tt.NewChoose("What is the message about?",
			[]string{"billing", "shipping", "other"}),
		"urgency": tt.NewScore("How urgent is the request?",
			[]string{"routine", "urgent"}),
	})
	messages := tt.Texts(
		"Please refund my order.",
		"Where is my parcel?",
		"I was charged twice for one invoice.",
	)
	for row, err := range client.AnnotateAll(
		ctx, form, messages) {
		if err != nil {
			log.Fatal(err)
		}
		answers, err := row.Answer()
		if err != nil {
			log.Fatal(err)
		}
		isRefund, err := tt.Answer(answers.Decide("refund"))
		if err != nil {
			log.Fatal(err)
		}
		topic, err := tt.Answer(answers.Choose("topic"))
		if err != nil {
			log.Fatal(err)
		}
		urgency, err := tt.Answer(answers.Score("urgency"))
		if err != nil {
			log.Fatal(err)
		}
		fmt.Println(isRefund, topic, urgency)
	}
}
