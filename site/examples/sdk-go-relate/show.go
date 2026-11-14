package main

import (
	"context"
	"fmt"
	"log"

	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
)

func main() {
	client, err := tt.New(tt.Config{})
	if err != nil {
		log.Fatal(err)
	}
	defer client.Close()
	ctx := context.Background()

	worksFor := tt.RelateQuestion{Rules: []tt.RelationRule{{
		Relation: "works_for",
		Source:   "person",
		Target:   "organization",
	}}}
	message := "Morgan works for Example Group.\n" +
		"Please refund my order."
	relations, err := tt.Answer(
		client.Relate(ctx, worksFor, message))
	if err != nil {
		log.Fatal(err)
	}
	for _, r := range relations {
		fmt.Println(r.Source.Text, r.Relation, r.Target.Text)
	}
}
