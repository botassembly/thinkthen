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

	actionQuestion := tt.NewFind("Which line states the requested action?")
	action, err := tt.Answer(client.Find(context.Background(), actionQuestion,
		tt.Text(string(message))))
	if err != nil {
		log.Fatal(err)
	}
	// nil when no line answers the question.
	if action == nil {
		fmt.Println("no line answers")
	} else {
		fmt.Println(action.Text)
	}
}
