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
	triage, err := rows[0].Value().Value.Value()
	if err != nil {
		log.Fatal(err)
	}
	steps, stepsErr := triage["steps"].Value()
	area, areaErr := triage["area"].Value()
	impact, impactErr := triage["impact"].Value()
	if stepsErr != nil || areaErr != nil || impactErr != nil {
		log.Fatal("expected three annotation fields")
	}
	if steps != true || area != "login" ||
		impact != json.Number("1.98") {
		log.Fatal("expected steps, login and 1.98")
	}
}
