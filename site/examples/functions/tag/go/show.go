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

	// Every label that fits, possibly none.
	topicsQuestion := tt.NewTag("Which topics appear?",
		[]string{"billing", "shipping"})
	topics, err := tt.Answer(client.Tag(context.Background(), topicsQuestion,
		tt.Text(string(message))))
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println(topics)
}
