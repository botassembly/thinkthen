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
		"evidence": "My parcel went to the wrong address.",
	})
	if err != nil {
		log.Fatal(err)
	}
	reply, err := tt.Call(ctx, string(request))
	if err != nil {
		log.Fatal(err)
	}
	var team struct{ Value string }
	err = json.Unmarshal([]byte(reply), &team)
	if err != nil {
		log.Fatal(err)
	}
	if team.Value != "shipping" {
		log.Fatalf("expected shipping, got %q", team.Value)
	}
}
