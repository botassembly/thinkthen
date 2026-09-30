package main

import (
	"context"
	"encoding/json"
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
	// Facts are JSON; read the members you need and ignore the rest.
	var facts struct {
		Records      int `json:"records"`
		RequestsSent int `json:"requests_sent"`
	}
	if err := json.Unmarshal(answer.Facts, &facts); err != nil || facts.Records != 1 || facts.RequestsSent != 1 {
		log.Fatalf("unexpected call facts: %s", answer.Facts)
	}
	fmt.Printf("outcome=%d probability=%.1f\n", answer.Value.Outcome, answer.Value.Probability)
}
