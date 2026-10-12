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
		{"name": "Paul McCartney", "kind": "singer"},
		{"name": "Ringo Starr", "kind": "singer"},
		{"name": "Yesterday", "kind": "song"},
		{"name": "Octopus's Garden", "kind": "song"},
	}
	sings := tt.AuthoredRelation{
		Name:   "sings",
		Source: tt.Ptr[tt.AuthoredName]("singer"),
		Target: tt.Ptr[tt.AuthoredName]("song"),
	}
	ask := tt.DefinedQuestion(tt.AuthoredRelate{
		Relate: tt.AuthoredRelateRelate{
			Relations: []tt.AuthoredRelation{sings},
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
		{"Paul McCartney", "Yesterday"},
		{"Ringo Starr", "Octopus's Garden"},
	}
	if !slices.Equal(pairs, want) {
		log.Fatal("expected two singers")
	}
}
