package main

import (
	"context"
	"log"

	tt "github.com/botassembly/thinkthen/libraries/go"
)

func main() {
	client, err := tt.NewClient(tt.EngineSettings{})
	if err != nil {
		log.Fatal(err)
	}
	defer client.Close()
	teamQuestion := tt.RequestQuestionFile{Path: "teams.json"}
	parcel := "My parcel went to the wrong address."
	call, err := client.Choose(
		context.Background(),
		teamQuestion,
		parcel,
		nil,
	)
	if err != nil {
		log.Fatal(err)
	}
	selected, err := call.ChooseAnswer()
	if err != nil {
		log.Fatal(err)
	}
	team := ""
	if selected != nil {
		team = *selected
	}
	if team != "shipping" {
		log.Fatal("expected shipping")
	}
}
