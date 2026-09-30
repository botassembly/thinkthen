// The replay smoke (ticket 0335): one Decide through New, which reads the
// environment, with the question and text sdlc/scripts/smoke names.
package main

import (
	"context"
	"fmt"
	"log"
	"os"

	thinkthen "github.com/botassembly/thinkthen/libraries/go"
)

func main() {
	engine, err := thinkthen.New()
	if err != nil {
		log.Fatal(err)
	}
	defer engine.Close()
	answer, err := engine.Decide(context.Background(), os.Getenv("THINKTHEN_TEST_SMOKE_QUESTION"), os.Getenv("THINKTHEN_TEST_SMOKE_TEXT"))
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("smoke: %s\n", map[thinkthen.Outcome]string{thinkthen.Yes: "true", thinkthen.No: "false", thinkthen.Unsure: "null"}[answer.Value.Outcome])
}
