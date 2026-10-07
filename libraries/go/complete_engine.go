package thinkthen

import "context"

// CompleteCall owns a function-specific row collection plus full invocation data.
type CompleteCall[T any] struct {
	Error        Optional[CompleteError]
	Schema       string
	Function     Optional[Function]
	AnswerId     Optional[AnswerId]
	Meta         Optional[Meta]
	Facts        Optional[CallFacts]
	Attempts     Optional[[]Attempt]
	Rows         []T
	Observations []ObservationEvent
}

// CompleteEngine exposes typed native complete calls.
type CompleteEngine interface {
	DecideComplete(context.Context, QuestionInput, InputSource, CallControls) (CompleteCall[DecideRow], error)
	ChooseComplete(context.Context, QuestionInput, InputSource, CallControls) (CompleteCall[ChooseRow], error)
	TagComplete(context.Context, QuestionInput, InputSource, CallControls) (CompleteCall[TagRow], error)
	ScoreComplete(context.Context, QuestionInput, InputSource, CallControls) (CompleteCall[ScoreRow], error)
	FilterComplete(context.Context, QuestionInput, InputSource, CallControls) (CompleteCall[FilterRow], error)
	RankComplete(context.Context, QuestionInput, InputSource, CallControls) (CompleteCall[RankRow], error)
	FindComplete(context.Context, QuestionInput, InputSource, CallControls) (CompleteCall[FindRow], error)
	AnnotateComplete(context.Context, QuestionInput, InputSource, CallControls) (CompleteCall[AnnotateRow], error)
	RecognizeComplete(context.Context, QuestionInput, InputSource, CallControls) (CompleteCall[RecognizeRow], error)
	RelateComplete(context.Context, QuestionInput, InputSource, CallControls) (CompleteCall[RelateRow], error)
}
