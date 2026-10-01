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

	spec := `{"version": 1, "recognize": {"kinds": {
		"PER": "Part of a person's name.",
		"ORG": "Part of the name of an organization: ` +
		`a company, band, team, agency, government ` +
		`body, or media outlet.",
		"LOC": "Part of the name of a place: a country, ` +
		`region, city, or geographic feature.",
		"MISC": "Part of another named entity: a ` +
		`nationality, an event, a product, or the ` +
		`name of a creative work."
	}}}`
	text := "Maria Chen joined Northwind Freight " +
		"in Chicago last spring."
	facts, err := tt.Recognize(ctx, spec, text)
	if err != nil {
		log.Fatal(err)
	}
	var recognized struct {
		Entities []struct{ Text, Kind string }
	}
	err = json.Unmarshal(facts.Value, &recognized)
	if err != nil {
		log.Fatal(err)
	}
	var names [][2]string
	for _, one := range recognized.Entities {
		names = append(names, [2]string{one.Text, one.Kind})
	}
	expected := [][2]string{
		{"Maria Chen", "PER"},
		{"Northwind Freight", "ORG"},
		{"Chicago", "LOC"},
	}
	if !reflect.DeepEqual(names, expected) {
		log.Fatalf("got %v", names)
	}
}
