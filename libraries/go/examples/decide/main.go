package main

import (
	"context"
	"fmt"
	tt "github.com/botassembly/thinkthen/libraries/go"
	"log"
)

func main() {
	client, err := tt.NewClient(tt.EngineSettings{})
	if err != nil {
		log.Fatal(err)
	}
	defer client.Close()
	call, err := client.Decide(context.Background(), tt.TextQuestion("Is it?"), "café", nil)
	if err != nil {
		log.Fatal(err)
	}
	for _, packet := range call.Packets {
		row, err := packet.AsSessionPacketDecideRow()
		if err != nil {
			continue
		}
		value := row.Value().Value.Value()
		answer, err := value.Value.Boolean()
		if err != nil {
			log.Fatal(err)
		}
		fmt.Printf("answer=%t\n", answer)
	}
}
