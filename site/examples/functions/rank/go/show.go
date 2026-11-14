package main

import (
	"context"
	"fmt"
	"log"
	"os"
	"slices"
	"strings"

	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
)

func main() {
	data, err := os.ReadFile("messages.txt")
	if err != nil {
		log.Fatal(err)
	}
	messages := strings.Split(strings.TrimSuffix(string(data), "\n"), "\n")
	client, err := tt.New(tt.Config{})
	if err != nil {
		log.Fatal(err)
	}
	defer client.Close()

	// Every message, most urgent first. Its answer is its rank, starting at 1.
	urgencyQuestion := tt.RankQuestion[string]{Question: "How urgent is the request?", Levels: []string{"routine", "urgent"}}
	for ranked, err := range client.Rank(
		context.Background(), urgencyQuestion, slices.Values(messages)) {
		if err != nil {
			log.Fatal(err)
		}
		urgencyRank, err := ranked.Answer()
		if err != nil {
			log.Fatal(err)
		}
		fmt.Println(urgencyRank, ranked.Item())
	}
}
