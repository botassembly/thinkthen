package main

import (
	"context"
	"fmt"
	"log"
	"os"

	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
)

func main() {
	message, err := os.ReadFile("message.txt")
	if err != nil {
		log.Fatal(err)
	}
	client, err := tt.New(tt.Config{})
	if err != nil {
		log.Fatal(err)
	}
	defer client.Close()

	worksFor := tt.RelateQuestion{Rules: []tt.RelationRule{
		{Relation: "works_for", Source: "person", Target: "organization"},
	}}
	relations, err := tt.Answer(client.Relate(context.Background(), worksFor,
		string(message)))
	if err != nil {
		log.Fatal(err)
	}
	for _, relation := range relations {
		fmt.Printf("%s --%s--> %s\n",
			relation.Source.Text, relation.Relation, relation.Target.Text)
	}
}
