package main

import (
	"context"
	"encoding/json"
	"log"
	"os"

	thinkthen "github.com/botassembly/thinkthen/libraries/go"
)

func main() {
	tt, err := thinkthen.New()
	if err != nil {
		log.Fatal(err)
	}
	defer tt.Close()
	ctx := context.Background()

	form, err := os.ReadFile("form.json")
	if err != nil {
		log.Fatal(err)
	}
	request, err := json.Marshal(map[string]any{
		"annotate": json.RawMessage(form),
		"records": []string{
			"Steps: click Log in. Nobody gets in.",
		},
	})
	if err != nil {
		log.Fatal(err)
	}
	reply, err := tt.Call(ctx, string(request))
	if err != nil {
		log.Fatal(err)
	}
	type Triage struct {
		Steps  bool
		Area   string
		Impact float64
	}
	var triage struct{ Value []Triage }
	err = json.Unmarshal([]byte(reply), &triage)
	if err != nil {
		log.Fatal(err)
	}
	expected := Triage{
		Steps:  true,
		Area:   "login",
		Impact: 1.98,
	}
	if triage.Value[0] != expected {
		log.Fatalf("got %+v", triage.Value[0])
	}
}
