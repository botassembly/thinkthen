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
		"person": null,
		"organization": null,
		"place": null
	}}}`
	text := "Maria Chen joined Northwind Freight, " +
		"a company in Chicago."
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
		{"Maria Chen", "person"},
		{"Northwind Freight", "organization"},
		{"Chicago", "place"},
	}
	if !reflect.DeepEqual(names, expected) {
		log.Fatalf("got %v", names)
	}
}
