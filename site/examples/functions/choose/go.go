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

	teams := `{
		"billing": "Invoices, fees, and refunds.",
		"shipping": "Parcels and delivery.",
		"account": "Logins and passwords."
	}`
	request, err := json.Marshal(map[string]any{
		"choose":   "Which team owns this?",
		"options":  json.RawMessage(teams),
		"evidence": "Please refund the extra fee on my invoice.",
	})
	if err != nil {
		log.Fatal(err)
	}
	reply, err := tt.Call(ctx, string(request))
	if err != nil {
		log.Fatal(err)
	}
	var owner struct{ Value string }
	err = json.Unmarshal([]byte(reply), &owner)
	if err != nil {
		log.Fatal(err)
	}
	if owner.Value != "billing" {
		log.Fatalf("expected billing, got %q", owner.Value)
	}
}
