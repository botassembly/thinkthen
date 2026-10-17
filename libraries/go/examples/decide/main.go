package main

import (
	"context"
	"encoding/json"
	tt "github.com/botassembly/thinkthen/libraries/go"
	"log"
	"os"
)

func printAnswer(answer tt.Call, err error) {
	if err != nil {
		log.Fatal(err)
	}
	if err := json.NewEncoder(os.Stdout).Encode(answer.Value); err != nil {
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
	printAnswer(client.Decide(ctx, question, tt.TextInput("text"), nil))
	printAnswer(client.Choose(ctx, tt.DefinedQuestion(tt.RequestDefinitionAuthoredChoose{Choose: tt.AuthoredQuestionTextString("Which?"), Options: tt.Ptr[tt.AuthoredOptions](tt.AuthoredOptionsObject{{Key: "billing", Value: tt.AuthoredDescriptionNull{}}, {Key: "shipping", Value: tt.AuthoredDescriptionNull{}}, {Key: "account", Value: tt.AuthoredDescriptionNull{}}, {Key: "fraud", Value: tt.AuthoredDescriptionNull{}}, {Key: "returns", Value: tt.AuthoredDescriptionNull{}}})}), tt.TextInput("text"), nil))
	printAnswer(client.Tag(ctx, tt.DefinedQuestion(tt.RequestDefinitionAuthoredTag{Tag: tt.AuthoredQuestionTextString("Which?"), Labels: tt.Ptr[tt.AuthoredLabels](tt.AuthoredLabelsArray{"a", "b"})}), tt.TextInput("text"), nil))
	printAnswer(client.Score(ctx, tt.DefinedQuestion(tt.RequestDefinitionAuthoredScore{Score: tt.AuthoredQuestionTextString("How?"), Levels: tt.Ptr[tt.AuthoredLevels](tt.AuthoredLevelsArray{"low", "high"})}), tt.TextInput("text"), nil))
	printAnswer(client.Filter(ctx, question, tt.TextRecordsInput("alpha", "beta"), nil))
	printAnswer(client.Rank(ctx, question, tt.TextRecordsInput("alpha", "beta"), nil))
	printAnswer(client.Find(ctx, tt.TextQuestion("Which?"), tt.TextRecordsInput("alpha", "beta"), nil))
	printAnswer(client.Annotate(ctx, tt.DefinedQuestion(tt.RequestDefinitionAnnotateDefinition{Questions: tt.OrderedMap[tt.AnnotateQuestion]{{Key: "billing", Value: tt.AnnotateQuestionDecide{Decide: tt.AuthoredQuestionTextString("Is billing?")}}, {Key: "shipping", Value: tt.AnnotateQuestionDecide{Decide: tt.AuthoredQuestionTextString("Is shipping?")}}, {Key: "account", Value: tt.AnnotateQuestionDecide{Decide: tt.AuthoredQuestionTextString("Is account?")}}, {Key: "fraud", Value: tt.AnnotateQuestionDecide{Decide: tt.AuthoredQuestionTextString("Is fraud?")}}, {Key: "returns", Value: tt.AnnotateQuestionDecide{Decide: tt.AuthoredQuestionTextString("Is returns?")}}}}), tt.TextRecordsInput("text"), nil))
	printAnswer(client.Recognize(ctx, tt.DefinedQuestion(tt.RequestDefinitionRecognizeDefinition{Recognize: tt.RecognizeQuestion{Kinds: tt.Ptr(tt.OrderedMap[tt.AuthoredDescription]{{Key: "person", Value: tt.AuthoredDescriptionNull{}}, {Key: "organization", Value: tt.AuthoredDescriptionNull{}}, {Key: "location", Value: tt.AuthoredDescriptionNull{}}, {Key: "product", Value: tt.AuthoredDescriptionNull{}}, {Key: "event", Value: tt.AuthoredDescriptionNull{}}})}}), tt.TextInput("Ana Lima"), nil))
	relation, err := client.Relate(ctx, tt.DefinedQuestion(tt.RequestDefinitionAuthoredRelate{Relate: tt.RelateQuestion{Relations: []tt.AuthoredRelation{{Name: "knows", Source: tt.Ptr(tt.AuthoredName("person")), Target: tt.Ptr(tt.AuthoredName("person"))}}}}), tt.RecordsInput(tt.JSONItem(map[string]any{"name": "Ana", "kind": "person"}), tt.JSONItem(map[string]any{"name": "Bob", "kind": "person"})), nil)
	printAnswer(relation, err)
	if err == nil && len(relation.Results) > 0 {
		row, err := relation.Results[0].AsRelate()
		if err != nil {
			log.Fatal(err)
		}
		edges := row.Value().Value
		if len(edges) > 0 {
			json.NewEncoder(os.Stdout).Encode([]string{edges[0].SourceName().Value, edges[0].TargetName().Value})
		}
	}
}
