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

	kinds := tt.RecognizeQuestion{Kinds: []string{"person", "organization"}}
	names, err := tt.Answer(client.Recognize(context.Background(), kinds,
		string(message)))
	if err != nil {
		log.Fatal(err)
	}
	for _, name := range names {
		fmt.Println(name.Kind+":", name.Text)
	}
}
