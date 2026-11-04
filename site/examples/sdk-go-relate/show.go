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

	worksFor := tt.NewRelate([]tt.Rule{{
		Name:   "works_for",
		Source: "person",
		Target: "organization",
	}})
	message := tt.Text("Morgan works for Example Group.\n" +
		"Please refund my order.")
	relations, err := tt.Answer(
		client.Relate(ctx, worksFor, message))
	if err != nil {
		log.Fatal(err)
	}
	for _, r := range relations {
		fmt.Println(r.Source.Text, r.Label, r.Target.Text)
	}
}
