package main

import (
	"context"
	"fmt"
	"log"

	thinkthen "github.com/botassembly/thinkthen/libraries/go"
)

func main() {
	engine, err := thinkthen.New()
	if err != nil {
		log.Fatal(err)
	}
	defer engine.Close()
	answer, err := engine.Decide(context.Background(), "Is it?", "café")
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("outcome=%d probability=%.1f\n", answer.Value.Outcome, answer.Value.Probability)
}
