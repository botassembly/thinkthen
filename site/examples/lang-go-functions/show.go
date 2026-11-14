package main

import (
	"context"
	"fmt"
	"log"
	"slices"

	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
)

func check(err error) {
	if err != nil {
		log.Fatal(err)
	}
}

func main() {
	ctx := context.Background()
	client, err := tt.New(tt.Config{})
	check(err)
	defer client.Close()

	refundMessage := "Please refund my order."
	inbox := slices.Values([]string{
		"Please refund my order.",
		"Where is my parcel?",
		"I was charged twice for one invoice.",
	})
	actionMessage := "Morgan works for Example Group.\nPlease refund my order."
	refundQuestion := "Does the customer request a refund?"
	topicQuestion := tt.ChooseQuestion[string]{Question: "What is the message about?", Choices: []string{"billing", "shipping", "other"}}
	urgencyLevels := []string{"routine", "urgent"}

	isRefund, err := tt.Answer(client.Decide(
		ctx, tt.DecideQuestion{Question: refundQuestion}, refundMessage,
	))
	check(err)
	fmt.Println("decide:", isRefund)

	for refundRequest, err := range client.Filter(
		ctx, tt.FilterQuestion{Question: refundQuestion}, inbox,
	) {
		check(err)
		fmt.Println("filter:", refundRequest)
	}

	topic, err := tt.Answer(client.Choose(ctx, topicQuestion, refundMessage))
	check(err)
	fmt.Println("choose:", topic)

	topics, err := tt.Answer(client.Tag(
		ctx,
		tt.TagQuestion[string]{Question: "Which topics appear?", Labels: []string{"billing", "shipping"}},
		"I was charged twice and my parcel is late again."))
	check(err)
	fmt.Println("tag:", topics)

	urgency, err := tt.Answer(client.Score(
		ctx, tt.ScoreQuestion[string]{Question: "How urgent is the request?", Levels: urgencyLevels},
		refundMessage,
	))
	check(err)
	fmt.Println("score:", urgency)

	for ranked, err := range client.Rank(
		ctx, tt.RankQuestion[string]{Question: "How urgent is the request?", Levels: urgencyLevels}, inbox,
	) {
		check(err)
		urgencyRank, err := ranked.Answer()
		check(err)
		fmt.Println("rank:", urgencyRank, ranked.Item())
	}

	action, err := tt.Answer(client.Find(
		ctx, tt.FindQuestion{Question: "Which line states the requested action?"},
		actionMessage,
	))
	check(err)
	if action == nil {
		fmt.Println("find: no line answers")
	} else {
		fmt.Println("find:", action.Text)
	}

	names, err := tt.Answer(client.Recognize(
		ctx, tt.RecognizeQuestion{Kinds: []string{"person", "organization"}},
		"Morgan works for Example Group."))
	check(err)
	for _, name := range names {
		fmt.Println("recognize:", name.Kind, name.Text)
	}

	worksFor := tt.RelateQuestion{Rules: []tt.RelationRule{
		{Relation: "works_for", Source: "person", Target: "organization"},
	}}
	relations, err := tt.Answer(client.Relate(ctx, worksFor, actionMessage))
	check(err)
	for _, relation := range relations {
		fmt.Println(
			"relate:", relation.Source.Text, relation.Relation, relation.Target.Text,
		)
	}

	triage, err := tt.Answer(client.Annotate(
		ctx, tt.AnnotateQuestion{Questions: map[string]tt.AnnotationQuestion{
			"refund":  tt.DecideQuestion{Question: refundQuestion},
			"topic":   topicQuestion,
			"urgency": tt.ScoreQuestion[string]{Question: "How urgent is the request?", Levels: urgencyLevels},
		}}, refundMessage,
	))
	check(err)
	formRefund, err := tt.Answer(triage.Decide("refund"))
	check(err)
	formTopic, err := tt.Answer(triage.Choose[string]("topic"))
	check(err)
	formUrgency, err := tt.Answer(triage.Score("urgency"))
	check(err)
	fmt.Println("annotate:", formRefund, formTopic, formUrgency)
}
