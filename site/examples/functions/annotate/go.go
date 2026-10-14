package main

import (
	"context"
	"encoding/json"
	"log"

	tt "github.com/botassembly/thinkthen/libraries/go"
)

func main() {
	client, err := tt.NewClient(tt.EngineSettings{})
	if err != nil {
		log.Fatal(err)
	}
	defer client.Close()
	form := tt.RequestQuestionFile{Path: "form.json"}
	report := "Steps: click Log in. Nobody gets in."
	call, err := client.Annotate(
		context.Background(),
		form,
		tt.TextRecordsInput(report),
		nil,
	)
	if err != nil {
		log.Fatal(err)
	}
	rows, err := call.AnnotateAnswer()
	if err != nil {
		log.Fatal(err)
	}
	triage, err := json.Marshal(rows[0].Value().Value)
	if err != nil {
		log.Fatal(err)
	}
	want := `{"steps":true,"area":"login","impact":1.98}`
	if string(triage) != want {
		log.Fatal("expected steps, login and 1.98")
	}
}
