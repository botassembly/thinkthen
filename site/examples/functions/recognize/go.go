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
	rows, err := call.RecognizeAnswer()
	if err != nil {
		log.Fatal(err)
	}
	var names [][2]string
	for _, row := range rows {
		facts, _ := row.Value().Value.AsRecognizeEntities()
		for _, one := range facts.Entities().Value {
			names = append(names, [2]string{
				one.Text().Value, one.Kind().Value,
			})
		}
	}
	want := [][2]string{
		{"Sam Patel", "person"},
		{"Kindle", "product"},
		{"Denver", "place"},
	}
	if !slices.Equal(names, want) {
		log.Fatal("expected three names")
	}
}
