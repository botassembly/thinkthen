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

	reviews := []string{
		"Arrived a day early. Thank you!",
		"The zipper broke the first time I used it.",
		"Does this come in blue?",
		"The strap snapped on day two.",
	}
	request, err := json.Marshal(map[string]any{
		"filter":  "Is this a complaint?",
		"records": reviews,
	})
	if err != nil {
		log.Fatal(err)
	}
	reply, err := tt.Call(ctx, string(request))
	if err != nil {
		log.Fatal(err)
	}
	var complaints struct{ Value []string }
	err = json.Unmarshal([]byte(reply), &complaints)
	if err != nil {
		log.Fatal(err)
	}
	expected := []string{reviews[1], reviews[3]}
	if !slices.Equal(complaints.Value, expected) {
		log.Fatalf("got %v", complaints.Value)
	}
}
