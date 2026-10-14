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
	answer, err := client.Decide(context.Background(), tt.TextQuestion(os.Getenv("THINKTHEN_TEST_SMOKE_QUESTION")), tt.TextInput(os.Getenv("THINKTHEN_TEST_SMOKE_TEXT")), nil)
	if err != nil {
		log.Fatal(err)
	}
	if answer.Value == nil {
		fmt.Println("smoke: null")
		return
	}
	fmt.Printf("smoke: %t\n", answer.Value)
}
