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
	question := "Is this a complaint?"
	reviews := []string{
		"Arrived a day early. Thank you!",
		"The zipper broke the first time I used it.",
		"Does this come in blue?",
		"The strap snapped on day two.",
	}
	call, err := client.Filter(
		context.Background(),
		tt.TextQuestion(question),
		reviews,
		nil,
	)
	if err != nil {
		log.Fatal(err)
	}
	kept, err := call.FilterAnswer()
	if err != nil {
		log.Fatal(err)
	}
	var complaints []string
	for _, text := range kept {
		complaints = append(complaints, text.(string))
	}
	want := []string{reviews[1], reviews[3]}
	if !slices.Equal(complaints, want) {
		log.Fatal("expected two complaints")
	}
}
