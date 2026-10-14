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
	question := "How urgent is this?"
	levels := tt.AuthoredLevelsArray{
		"Routine.",
		"Soon.",
		"Immediate.",
	}
	definition := tt.RequestDefinitionAuthoredScore{
		Score:  tt.AuthoredQuestionTextString(question),
		Levels: tt.Ptr[tt.AuthoredLevels](levels),
	}
	scale := tt.DefinedQuestion(definition)
	outage := "Our checkout page is down " +
		"and customers cannot pay.\n"
	call, err := client.Score(
		context.Background(),
		scale,
		outage,
		nil,
	)
	if err != nil {
		log.Fatal(err)
	}
	level, err := call.ScoreAnswer()
	if err != nil {
		log.Fatal(err)
	}
	urgency, _ := level.Float64()
	if urgency != 2 {
		log.Fatal("expected 2")
	}
}
