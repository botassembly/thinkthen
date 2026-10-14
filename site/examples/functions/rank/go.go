package main

import (
	"context"
	"encoding/json"
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
		tt.TextRecordsInput(inbox...),
		nil,
	)
	if err != nil {
		log.Fatal(err)
	}
	order := call.Value.([]any)
	want := []any{json.Number("1"), json.Number("3"),
		json.Number("2"), json.Number("0")}
	if !slices.Equal(order, want) {
		log.Fatal("expected the outage first")
	}
	probabilities := call.Probability.(tt.ProbabilityList)
	wantProbabilities := []float64{0.97, 0.92, 0.19, 0.03}
	if len(probabilities) != len(wantProbabilities) {
		log.Fatal("expected aligned probabilities")
	}
	for i, probability := range probabilities {
		if probability == nil ||
			*probability != wantProbabilities[i] {
			log.Fatal("unexpected recorded probability")
		}
	}
}
