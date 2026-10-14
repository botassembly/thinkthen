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
	ask := tt.RequestQuestionFile{Path: "kinds.json"}
	text := "Sam Patel says his Kindle arrived broken " +
		"in Denver."
	call, err := client.Recognize(
		context.Background(),
		ask,
		tt.TextInput(text),
		nil,
	)
	if err != nil {
		log.Fatal(err)
	}
	recognition := call.Value.(map[string]any)
	var names [][2]string
	for _, one := range recognition["entities"].([]any) {
		entity := one.(map[string]any)
		names = append(names, [2]string{entity["text"].(string), entity["kind"].(string)})
	}
	want := [][2]string{{"Sam Patel", "person"}, {"Kindle", "product"}, {"Denver", "place"}}
	if !slices.Equal(names, want) {
		log.Fatal("expected three names")
	}
	probabilities := call.Probability.(tt.ProbabilityList)
	wantProbabilities := []float64{0.9963502281449241, 0.9899999999980199, 0.9899999797949085}
	if len(probabilities) != len(wantProbabilities) {
		log.Fatal("expected aligned probabilities")
	}
	for i, probability := range probabilities {
		if probability == nil || *probability != wantProbabilities[i] {
			log.Fatal("unexpected recorded probability")
		}
	}
}
