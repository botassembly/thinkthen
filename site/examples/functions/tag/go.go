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
	question := "Which labels fit this message?"
	labels := tt.AuthoredLabelsArray{
		"praise",
		"bug",
		"billing",
	}
	definition := tt.RequestDefinitionAuthoredTag{
		Tag:    tt.AuthoredQuestionTextString(question),
		Labels: tt.Ptr[tt.AuthoredLabels](labels),
	}
	labelQuestion := tt.DefinedQuestion(definition)
	message := "Love the new dashboard, " +
		"but export crashes the app,\n" +
		"and I was charged twice.\n"
	call, err := client.Tag(
		context.Background(),
		labelQuestion,
		tt.TextInput(message),
		nil,
	)
	if err != nil {
		log.Fatal(err)
	}
	labelsReturned := call.Value.([]any)
	var fittingLabels []string
	for _, label := range labelsReturned {
		fittingLabels = append(fittingLabels, label.(string))
	}
	want := []string{"praise", "bug", "billing"}
	if !slices.Equal(fittingLabels, want) || call.Probability != nil {
		log.Fatal("expected all three labels with no scalar probability")
	}
}
