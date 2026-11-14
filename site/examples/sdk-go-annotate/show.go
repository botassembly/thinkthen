package main

import (
	"context"
	"fmt"
	"log"
	"slices"

	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
)

func main() {
	client, err := tt.New(tt.Config{})
	if err != nil {
		log.Fatal(err)
	}
	defer client.Close()
	ctx := context.Background()

	form := tt.AnnotateQuestion{Questions: map[string]tt.AnnotationQuestion{
		"refund":  tt.DecideQuestion{Question: "Does the customer request a refund?"},
		"topic":   tt.ChooseQuestion[string]{Question: "What is the message about?", Choices: []string{"billing", "shipping", "other"}},
		"urgency": tt.ScoreQuestion[string]{Question: "How urgent is the request?", Levels: []string{"routine", "urgent"}},
	}}
	messages := slices.Values([]string{
		"Please refund my order.",
		"Where is my parcel?",
		"I was charged twice for one invoice.",
	})
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
		topic, err := tt.Answer(answers.Choose[string]("topic"))
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
