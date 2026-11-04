package main

import (
	"context"
	"fmt"
	"log"
	"os"

	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
)

func main() {
	message, err := os.ReadFile("message.txt")
	if err != nil {
		log.Fatal(err)
	}
	client, err := tt.New(tt.Config{})
	if err != nil {
		log.Fatal(err)
	}
	defer client.Close()

	// 0 means routine and 1 means urgent.
	urgencyQuestion := tt.NewScore("How urgent is the request?",
		[]string{"routine", "urgent"})
	urgency, err := tt.Answer(client.Score(context.Background(),
		urgencyQuestion, tt.Text(string(message))))
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println(urgency)
}
