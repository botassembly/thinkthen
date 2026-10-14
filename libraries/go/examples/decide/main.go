package main

import (
	"context"
	"encoding/json"
	tt "github.com/botassembly/thinkthen/libraries/go"
	"log"
	"os"
)

func printAnswer[T any](answer T, err error) {
	if err != nil {
		log.Fatal(err)
	}
	if err := json.NewEncoder(os.Stdout).Encode(answer); err != nil {
		log.Fatal(err)
	}
}

func main() {
	client, err := tt.NewClient(tt.EngineSettings{})
	if err != nil {
		log.Fatal(err)
	}
	defer client.Close()
	ctx := context.Background()
	question := tt.TextQuestion("Is it?")
	printAnswer(client.DecideAnswer(ctx, question, tt.TextInput("text"), nil))
	printAnswer(client.ChooseAnswer(ctx, tt.DefinedQuestion(tt.RequestDefinitionAuthoredChoose{Choose: tt.AuthoredQuestionTextString("Which?"), Options: tt.Ptr[tt.AuthoredOptions](tt.AuthoredOptionsObject{{Key: "billing", Value: tt.AuthoredDescriptionNull{}}, {Key: "shipping", Value: tt.AuthoredDescriptionNull{}}, {Key: "account", Value: tt.AuthoredDescriptionNull{}}, {Key: "fraud", Value: tt.AuthoredDescriptionNull{}}, {Key: "returns", Value: tt.AuthoredDescriptionNull{}}})}), tt.TextInput("text"), nil))
	printAnswer(client.TagAnswer(ctx, tt.DefinedQuestion(tt.RequestDefinitionAuthoredTag{Tag: tt.AuthoredQuestionTextString("Which?"), Labels: tt.Ptr[tt.AuthoredLabels](tt.AuthoredLabelsArray{"a", "b"})}), tt.TextInput("text"), nil))
	printAnswer(client.ScoreAnswer(ctx, tt.DefinedQuestion(tt.RequestDefinitionAuthoredScore{Score: tt.AuthoredQuestionTextString("How?"), Levels: tt.Ptr[tt.AuthoredLevels](tt.AuthoredLevelsArray{"low", "high"})}), tt.TextInput("text"), nil))
	printAnswer(client.FilterAnswer(ctx, question, tt.TextRecordsInput("alpha", "beta"), nil))
	printAnswer(client.RankAnswer(ctx, question, tt.TextRecordsInput("alpha", "beta"), nil))
	printAnswer(client.FindAnswer(ctx, tt.TextQuestion("Which?"), tt.TextRecordsInput("alpha", "beta"), nil))
	printAnswer(client.AnnotateAnswer(ctx, tt.DefinedQuestion(tt.RequestDefinitionAnnotateDefinition{Questions: tt.OrderedMap[tt.AnnotateQuestion]{{Key: "billing", Value: tt.AnnotateQuestionDecide{Decide: tt.AuthoredQuestionTextString("Is billing?")}}, {Key: "shipping", Value: tt.AnnotateQuestionDecide{Decide: tt.AuthoredQuestionTextString("Is shipping?")}}, {Key: "account", Value: tt.AnnotateQuestionDecide{Decide: tt.AuthoredQuestionTextString("Is account?")}}, {Key: "fraud", Value: tt.AnnotateQuestionDecide{Decide: tt.AuthoredQuestionTextString("Is fraud?")}}, {Key: "returns", Value: tt.AnnotateQuestionDecide{Decide: tt.AuthoredQuestionTextString("Is returns?")}}}}), tt.TextRecordsInput("text"), nil))
	printAnswer(client.RecognizeAnswer(ctx, tt.DefinedQuestion(tt.RequestDefinitionRecognizeDefinition{Recognize: tt.RecognizeQuestion{Kinds: tt.Ptr(tt.OrderedMap[tt.AuthoredDescription]{{Key: "person", Value: tt.AuthoredDescriptionNull{}}, {Key: "organization", Value: tt.AuthoredDescriptionNull{}}, {Key: "location", Value: tt.AuthoredDescriptionNull{}}, {Key: "product", Value: tt.AuthoredDescriptionNull{}}, {Key: "event", Value: tt.AuthoredDescriptionNull{}}})}}), tt.TextInput("Ana Lima"), nil))
	relation, err := client.RelateAnswer(ctx, tt.DefinedQuestion(tt.RequestDefinitionAuthoredRelate{Relate: tt.RelateQuestion{Relations: []tt.AuthoredRelation{{Name: "knows", Source: tt.Ptr(tt.AuthoredName("person")), Target: tt.Ptr(tt.AuthoredName("person"))}}}}), tt.RecordsInput(tt.JSONItem(map[string]any{"name": "Ana", "kind": "person"}), tt.JSONItem(map[string]any{"name": "Bob", "kind": "person"})), nil)
	printAnswer(relation, err)
	if err == nil {
		edges := relation.Value()
		if edges.Err != nil {
			log.Fatal(edges.Err)
		}
		if len(edges.Value) > 0 {
			printAnswer([]string{edges.Value[0].SourceName().Value, edges.Value[0].TargetName().Value}, nil)
		}
	}
}
