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

	labels := []string{"praise", "bug", "billing"}
	request, err := json.Marshal(map[string]any{
		"tag":    "Which labels fit this message?",
		"labels": labels,
		"evidence": "Love the new dashboard, " +
			"but export crashes the app,\n" +
			"and I was charged twice.\n",
	})
	if err != nil {
		log.Fatal(err)
	}
	reply, err := tt.Call(ctx, string(request))
	if err != nil {
		log.Fatal(err)
	}
	var fittingLabels struct{ Value []string }
	err = json.Unmarshal([]byte(reply), &fittingLabels)
	if err != nil {
		log.Fatal(err)
	}
	if !slices.Equal(fittingLabels.Value, labels) {
		log.Fatalf("got %v", fittingLabels.Value)
	}
}
