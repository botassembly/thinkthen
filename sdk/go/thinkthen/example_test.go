package thinkthen_test

import (
	"context"
	"fmt"
	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
)

func ExampleAnswer() {
	client, err := tt.New(tt.Config{})
	if err != nil {
		fmt.Println(err)
		return
	}
	defer client.Close()
	isRefund, err := tt.Answer(client.Decide(context.Background(),
		tt.DecideQuestion{Question: "Does the message request a refund?"},
		"Please refund this order."))
	if err != nil {
		fmt.Println(err)
		return
	}
	fmt.Println("refund requested:", isRefund)
}
