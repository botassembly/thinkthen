package main

import (
	"context"
	"fmt"
	"log"
	"os"
	"strings"

	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
)

func main() {
	data, err := os.ReadFile("messages.txt")
	if err != nil {
		log.Fatal(err)
	}
	messages := strings.Split(strings.TrimSuffix(string(data), "\n"), "\n")
	client, err := tt.New(tt.Config{})
	if err != nil {
		log.Fatal(err)
	}
	defer client.Close()

	// Only the messages that answer yes. A failed message arrives as an error.
	refundQuestion := tt.NewFilter("Does the customer request a refund?")
	for refund, err := range client.FilterAll(
		context.Background(), refundQuestion, tt.Texts(messages...)) {
		if err != nil {
			log.Fatal(err)
		}
		fmt.Println(refund.Item())
	}
}
