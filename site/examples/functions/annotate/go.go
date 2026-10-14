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
	rows := call.Value.([]any)
	triage := rows[0].(map[string]any)
	if len(rows) != 1 || len(triage) != 3 ||
		triage["steps"] != true ||
		triage["area"] != "login" ||
		triage["impact"] != json.Number("1.98") ||
		call.Probability != nil {
		log.Fatal("expected steps, login and 1.98 " +
			"with no scalar probability")
	}
}
