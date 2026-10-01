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

	request, err := json.Marshal(map[string]any{
		"score":  "How urgent is this?",
		"levels": []string{"Routine.", "Soon.", "Immediate."},
		"evidence": "Our checkout page is down " +
			"and customers cannot pay.\n",
	})
	if err != nil {
		log.Fatal(err)
	}
	reply, err := tt.Call(ctx, string(request))
	if err != nil {
		log.Fatal(err)
	}
	var urgency struct{ Value float64 }
	err = json.Unmarshal([]byte(reply), &urgency)
	if err != nil {
		log.Fatal(err)
	}
	if urgency.Value != 2.0 {
		log.Fatalf("expected 2.0, got %v", urgency.Value)
	}
}
