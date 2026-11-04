package main

import (
	"context"
	"fmt"
	"log"
	"os"
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

	form := tt.NewAnnotate(map[string]tt.AnnotationQuestion{
		"refund": tt.NewDecide("Does the customer request a refund?"),
		"topic": tt.NewChoose("What is the message about?",
			[]string{"billing", "shipping", "other"}),
		"urgency": tt.NewScore("How urgent is the request?",
			[]string{"routine", "urgent"}),
	})
	for triage, err := range client.AnnotateAll(
		context.Background(), form, tt.Texts(messages...)) {
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
