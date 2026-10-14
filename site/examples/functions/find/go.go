package main

import (
	"context"
	"log"

	tt "github.com/botassembly/thinkthen/libraries/go"
)

func main() {
	client, err := tt.NewClient(tt.EngineSettings{})
	if err != nil {
		log.Fatal(err)
	}
	defer client.Close()
	question := "Which line gives the refund deadline?"
	policy := []string{
		"Returns need the original receipt.",
		"Refunds are issued within 30 days of purchase.",
		"Shipping is free on orders over $50.",
		"Gift cards cannot be exchanged for cash.",
	}
	call, err := client.Find(
		context.Background(),
		tt.TextQuestion(question),
		tt.TextRecordsInput(policy...),
		nil,
	)
	if err != nil {
		log.Fatal(err)
	}
	refundDeadline, probability := call.Value.(string),
		float64(call.Probability.(tt.ProbabilityScalar))
	if refundDeadline != policy[1] || probability != 1 {
		log.Fatal("expected the refund line with probability 1")
	}
}
