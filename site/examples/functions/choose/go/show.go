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

	topicQuestion := tt.NewChoose("What is the message about?",
		[]string{"billing", "shipping", "other"})
	topic, err := tt.Answer(client.Choose(context.Background(), topicQuestion,
		tt.Text(string(message))))
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println(topic)
}
