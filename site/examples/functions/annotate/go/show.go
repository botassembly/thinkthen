package main

import (
	"context"
	"fmt"
	"log"
	"os"
	"slices"
	"strings"

	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
)

func main() {
	data, err := os.ReadFile("messages.txt")
	if err != nil {
		log.Fatal(err)
	}
	messages := strings.Split(strings.TrimSuffix(string(data), "\n"), "\n")
	client, err := tt.New(tt.Config{})
	if err != nil {
		log.Fatal(err)
	}
	defer client.Close()

	form := tt.AnnotateQuestion{Questions: map[string]tt.AnnotationQuestion{
		"refund":  tt.DecideQuestion{Question: "Does the customer request a refund?"},
		"topic":   tt.ChooseQuestion[string]{Question: "What is the message about?", Choices: []string{"billing", "shipping", "other"}},
		"urgency": tt.ScoreQuestion[string]{Question: "How urgent is the request?", Levels: []string{"routine", "urgent"}},
	}}
	for triage, err := range client.AnnotateAll(
		context.Background(), form, slices.Values(messages)) {
		if err != nil {
			log.Fatal(err)
		}
		answers, err := triage.Answer()
		if err != nil {
			log.Fatal(err)
		}
		// Each answer reads like a one-question call:
		// tt.ErrUnsure when not sure.
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
