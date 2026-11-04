package main

import (
	"context"
	"fmt"
	"log"

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

	refundMessage := tt.Text("Please refund my order.")
	inbox := tt.Texts(
		"Please refund my order.",
		"Where is my parcel?",
		"I was charged twice for one invoice.",
	)
	actionMessage := tt.Text(
		"Morgan works for Example Group.\nPlease refund my order.",
	)
	refundQuestion := "Does the customer request a refund?"
	topicQuestion := tt.NewChoose(
		"What is the message about?",
		[]string{"billing", "shipping", "other"},
	)
	urgencyLevels := []string{"routine", "urgent"}

	isRefund, err := tt.Answer(client.Decide(
		ctx, tt.NewDecide(refundQuestion), refundMessage,
	))
	check(err)
	fmt.Println("decide:", isRefund)

	for refundRequest, err := range client.FilterAll(
		ctx, tt.NewFilter(refundQuestion), inbox,
	) {
		check(err)
		fmt.Println("filter:", refundRequest.Item())
	}

	topic, err := tt.Answer(client.Choose(ctx, topicQuestion, refundMessage))
	check(err)
	fmt.Println("choose:", topic)

	topics, err := tt.Answer(client.Tag(
		ctx,
		tt.NewTag("Which topics appear?", []string{"billing", "shipping"}),
		tt.Text("I was charged twice and my parcel is late again.")))
	check(err)
	fmt.Println("tag:", topics)

	urgency, err := tt.Answer(client.Score(
		ctx, tt.NewScore("How urgent is the request?", urgencyLevels),
		refundMessage,
	))
	check(err)
	fmt.Println("score:", urgency)

	for ranked, err := range client.RankAll(
		ctx, tt.NewRank("How urgent is the request?", urgencyLevels), inbox,
	) {
		check(err)
		urgencyRank, err := ranked.Answer()
		check(err)
		fmt.Println("rank:", urgencyRank, ranked.Item())
	}

	action, err := tt.Answer(client.Find(
		ctx, tt.NewFind("Which line states the requested action?"),
		actionMessage,
	))
	check(err)
	if action == nil {
		fmt.Println("find: no line answers")
	} else {
		fmt.Println("find:", action.Text)
	}

	names, err := tt.Answer(client.Recognize(
		ctx, tt.NewRecognize([]string{"person", "organization"}),
		tt.Text("Morgan works for Example Group.")))
	check(err)
	for _, name := range names {
		fmt.Println("recognize:", name.Kind, name.Text)
	}

	worksFor := tt.NewRelate([]tt.Rule{
		{Name: "works_for", Source: "person", Target: "organization"},
	})
	relations, err := tt.Answer(client.Relate(ctx, worksFor, actionMessage))
	check(err)
	for _, relation := range relations {
		fmt.Println(
			"relate:", relation.Source.Text, relation.Label, relation.Target.Text,
		)
	}

	triage, err := tt.Answer(client.Annotate(
		ctx, tt.NewAnnotate(map[string]tt.AnnotationQuestion{
			"refund":  tt.NewDecide(refundQuestion),
			"topic":   topicQuestion,
			"urgency": tt.NewScore("How urgent is the request?", urgencyLevels),
		}), refundMessage,
	))
	check(err)
	formRefund, err := tt.Answer(triage.Decide("refund"))
	check(err)
	formTopic, err := tt.Answer(triage.Choose("topic"))
	check(err)
	formUrgency, err := tt.Answer(triage.Score("urgency"))
	check(err)
	fmt.Println("annotate:", formRefund, formTopic, formUrgency)
}
