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
	fittingLabels, err := call.TagAnswer()
	if err != nil {
		log.Fatal(err)
	}
	want := []string{"praise", "bug", "billing"}
	if !slices.Equal(fittingLabels, want) {
		log.Fatal("expected all three labels")
	}
}
