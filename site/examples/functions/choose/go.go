package main

import (
	"context"
	"encoding/json"
	"log"
	"reflect"

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
	texts := []string{
		"Please refund the extra fee on my invoice.",
		"My parcel went to the wrong address.",
		"I cannot reset my password.",
		"My parcel never came, and now " +
			"I cannot log in to track it.",
	}
	var owners []any
	for _, text := range texts {
		request, err := json.Marshal(map[string]any{
			"choose":    "Which team owns this?",
			"options":   json.RawMessage(teams),
			"threshold": 0.9,
			"evidence":  text,
		})
		if err != nil {
			log.Fatal(err)
		}
		reply, err := tt.Call(ctx, string(request))
		if err != nil {
			log.Fatal(err)
		}
		var owner struct{ Value any }
		err = json.Unmarshal([]byte(reply), &owner)
		if err != nil {
			log.Fatal(err)
		}
		owners = append(owners, owner.Value)
	}
	expected := []any{"billing", "shipping", "account", nil}
	if !reflect.DeepEqual(owners, expected) {
		log.Fatalf("expected %v, got %v", expected, owners)
	}
}
