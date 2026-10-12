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
	scale := tt.DefinedQuestion(tt.AuthoredScore{
		Score:  tt.AuthoredQuestionTextString(question),
		Levels: tt.Ptr[tt.AuthoredLevels](levels),
	})
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
	var urgency float64
	for _, packet := range call.Packets {
		row, err := packet.AsSessionPacketScoreRow()
		if err == nil {
			level := row.Value().Value.Value().Value
			urgency, _ = level.Float64()
		}
	}
	if urgency != 2 {
		log.Fatal("expected 2")
	}
}
