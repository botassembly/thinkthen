package main

import (
	"context"
	"fmt"
	tt "github.com/botassembly/thinkthen/libraries/go"
	"log"
	"os"
)

func main() {
	client, err := tt.NewClient(tt.EngineSettings{})
	if err != nil {
		log.Fatal(err)
	}
	defer client.Close()
	call, err := client.Decide(context.Background(), tt.TextQuestion(os.Getenv("THINKTHEN_TEST_SMOKE_QUESTION")), os.Getenv("THINKTHEN_TEST_SMOKE_TEXT"), nil)
	if err != nil {
		log.Fatal(err)
	}
	for _, packet := range call.Packets {
		row, err := packet.AsSessionPacketDecideRow()
		if err != nil {
			continue
		}
		value := row.Value().Value.Value()
		if value.Null {
			fmt.Println("smoke: null")
			return
		}
		answer, err := value.Value.Boolean()
		if err != nil {
			log.Fatal(err)
		}
		fmt.Printf("smoke: %t\n", answer)
		return
	}
	log.Fatal("missing decision row")
}
