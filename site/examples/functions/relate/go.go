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
	ask := tt.DefinedQuestion(tt.AuthoredRelate{
		Relate: tt.AuthoredRelateRelate{
			Relations: []tt.AuthoredRelation{makes},
		},
	})
	call, err := client.Relate(
		context.Background(),
		ask,
		names,
		nil,
	)
	if err != nil {
		log.Fatal(err)
	}
	var edges []tt.OwnedRelatedEntityEdge
	for _, packet := range call.Packets {
		related, err := packet.AsSessionPacketRelateAggregate()
		if err == nil {
			edges = related.Value().Value.Value().Value
		}
	}
	var pairs [][2]string
	for _, edge := range edges {
		source, _ := edge.Source().Value.
			AsRelatedEntityEdgePropertiesSourceFieldsKindName()
		target, _ := edge.Target().Value.
			AsRelatedEntityEdgePropertiesSourceFieldsKindName()
		pairs = append(pairs, [2]string{
			source.Name().Value,
			target.Name().Value,
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
