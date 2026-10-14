package main

import (
	"context"
	"log"
	"slices"

	tt "github.com/botassembly/thinkthen/libraries/go"
)

func main() {
	client, err := tt.NewClient(tt.EngineSettings{})
	if err != nil {
		log.Fatal(err)
	}
	defer client.Close()
	names := []map[string]string{
		{"name": "Apple", "kind": "company"},
		{"name": "Amazon", "kind": "company"},
		{"name": "iPhone", "kind": "product"},
		{"name": "Kindle", "kind": "product"},
	}
	makes := tt.AuthoredRelation{
		Name:   "makes",
		Source: tt.Ptr[tt.AuthoredName]("company"),
		Target: tt.Ptr[tt.AuthoredName]("product"),
	}
	definition := tt.RequestDefinitionAuthoredRelate{
		Relate: tt.RelateQuestion{
			Relations: []tt.AuthoredRelation{makes},
		},
	}
	var records []tt.RequestItem
	for _, name := range names {
		records = append(records, tt.JSONItem(name))
	}
	ask := tt.DefinedQuestion(definition)
	call, err := client.Relate(
		context.Background(),
		ask,
		tt.RequestInputEntities{Items: records},
		nil,
	)
	if err != nil {
		log.Fatal(err)
	}
	related, err := call.RelateAnswer()
	if err != nil {
		log.Fatal(err)
	}
	var pairs [][2]string
	for _, edge := range related.Value().Value {
		source, _ := edge.Source().Value.
			AsRelationEndpointRelatedEntity()
		target, _ := edge.Target().Value.
			AsRelationEndpointRelatedEntity()
		pairs = append(pairs, [2]string{
			source.Name().Value, target.Name().Value,
		})
	}
	want := [][2]string{
		{"Apple", "iPhone"},
		{"Amazon", "Kindle"},
	}
	if !slices.Equal(pairs, want) {
		log.Fatal("expected two companies")
	}
}
