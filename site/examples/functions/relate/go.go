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

	spec := `{"version": 1, "relate": {"relations": [
		{"name": "sings", "source": "singer", "target": "song"}
	]}}`
	names := []string{
		`{"name": "Paul McCartney", "kind": "singer"}`,
		`{"name": "Ringo Starr", "kind": "singer"}`,
		`{"name": "Yesterday", "kind": "song"}`,
		`{"name": "Octopus's Garden", "kind": "song"}`,
	}
	whoSings, err := tt.Relate(ctx, spec, names)
	if err != nil {
		log.Fatal(err)
	}
	type Named struct{ Name string }
	var graph struct {
		Edges []struct{ Source, Target Named }
	}
	err = json.Unmarshal(whoSings.Value, &graph)
	if err != nil {
		log.Fatal(err)
	}
	var sings [][2]string
	for _, edge := range graph.Edges {
		sings = append(sings, [2]string{
			edge.Source.Name, edge.Target.Name,
		})
	}
	expected := [][2]string{
		{"Paul McCartney", "Yesterday"},
		{"Ringo Starr", "Octopus's Garden"},
	}
	if !reflect.DeepEqual(sings, expected) {
		log.Fatalf("got %v", sings)
	}
}
