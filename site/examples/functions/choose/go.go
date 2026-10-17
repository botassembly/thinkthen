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
		tt.TextInput(parcel),
		nil,
	)
	if err != nil {
		log.Fatal(err)
	}
	team, probability := call.Value.(string),
		float64(call.Probability.(tt.ProbabilityScalar))
	if team != "shipping" || probability != 1 {
		log.Fatal("expected shipping with probability 1")
	}
}
