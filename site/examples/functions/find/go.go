package main

import (
	"context"
	"encoding/json"
	"log"

	thinkthen "github.com/botassembly/thinkthen/libraries/go"
)

func main() {
	tt, err := thinkthen.New()
	if err != nil {
		log.Fatal(err)
	}
	defer tt.Close()
	ctx := context.Background()

	policy := []string{
		"Returns need the original receipt.",
		"Refunds are issued within 30 days of purchase.",
		"Shipping is free on orders over $50.",
		"Gift cards cannot be exchanged for cash.",
	}
	request, err := json.Marshal(map[string]any{
		"find":  "Which line gives the refund deadline?",
		"units": policy,
	})
	if err != nil {
		log.Fatal(err)
	}
	reply, err := tt.Call(ctx, string(request))
	if err != nil {
		log.Fatal(err)
	}
	var refundDeadline struct{ Value struct{ Unit string } }
	err = json.Unmarshal([]byte(reply), &refundDeadline)
	if err != nil {
		log.Fatal(err)
	}
	if refundDeadline.Value.Unit != policy[1] {
		log.Fatalf("got %q", refundDeadline.Value.Unit)
	}
}
