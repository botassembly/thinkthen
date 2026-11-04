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

	// tt.Answer returns tt.ErrUnsure when not sure,
	// and the error when the message failed.
	isRefund, err := tt.Answer(client.Decide(context.Background(),
		tt.NewDecide("Does the customer request a refund?"),
		tt.Text(string(message))))
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println(isRefund)
}
