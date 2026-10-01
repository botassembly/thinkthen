package main

import (
	"context"
	"encoding/json"
	"log"
	"slices"

	thinkthen "github.com/botassembly/thinkthen/libraries/go"
)

func main() {
	tt, err := thinkthen.New()
	if err != nil {
		log.Fatal(err)
	}
	defer tt.Close()
	ctx := context.Background()

	inbox := []string{
		"Newsletter: our autumn catalog is here. " +
			"No reply needed.",
		"Our checkout page is down and customers " +
			"cannot pay",
		"Reminder: your invoice is due in 30 days",
		"Please send the signed quote by 5 pm today",
	}
	request, err := json.Marshal(map[string]any{
		"rank":    "Is this urgent?",
		"records": inbox,
	})
	if err != nil {
		log.Fatal(err)
	}
	reply, err := tt.Call(ctx, string(request))
	if err != nil {
		log.Fatal(err)
	}
	var byUrgency struct{ Value []struct{ Index int } }
	err = json.Unmarshal([]byte(reply), &byUrgency)
	if err != nil {
		log.Fatal(err)
	}
	var order []int
	for _, one := range byUrgency.Value {
		order = append(order, one.Index)
	}
	if !slices.Equal(order, []int{1, 3, 2, 0}) {
		log.Fatalf("got %v", order)
	}
}
