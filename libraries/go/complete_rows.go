package thinkthen

type CommonRow struct {
	AnswerId  AnswerId
	Input     Optional[Content]
	Question  Optional[Question]
	Answer    Optional[AtomicAnswer]
	Threshold Optional[Rule]
	Position  Optional[Location]
	InputFile Optional[string]
	Meta      Meta
	Images    Optional[[]ImageView]
}

type DecideRow struct {
	Common CommonRow
	Value  DecideValue
}

type ChooseRow struct {
	Common CommonRow
	Value  Optional[string]
}

type TagRow struct {
	Common CommonRow
	Value  []string
}

type ScoreRow struct {
	Common CommonRow
	Value  float64
}

type FilterRow struct {
	Common CommonRow
	Value  bool
}

type RankRow struct {
	Common       CommonRow
	Value        Optional[uint64]
	QuestionName Optional[string]
}

type FindRow struct {
	Common CommonRow
	Value  Optional[Content]
	Index  Optional[uint64]
}

type AnnotateRow struct {
	Common  CommonRow
	Answers []AnnotationMember
}

type RecognizeRow struct {
	Common CommonRow
	Value  RecognizeValue
	Answer RecognizeAnswer
}

type RelateRow struct {
	Common    CommonRow
	Value     []Edge
	Questions []RelationAnswer
}

type RowValue struct {
	Function  Function
	Decide    Optional[DecideRow]
	Choose    Optional[ChooseRow]
	Tag       Optional[TagRow]
	Score     Optional[ScoreRow]
	Filter    Optional[FilterRow]
	Rank      Optional[RankRow]
	Find      Optional[FindRow]
	Annotate  Optional[AnnotateRow]
	Recognize Optional[RecognizeRow]
	Relate    Optional[RelateRow]
}

type ObservedProbabilities struct {
	Yes   Optional[float64]
	Named Optional[[]Probability]
}

type ObservationSuccess struct {
	AnswerId      AnswerId
	ObservationId ObservationId
	Value         MemberValue
	Probabilities ObservedProbabilities
	Confidence    Optional[float64]
}

type QuestionObservation struct {
	Index           uint64
	Member          Optional[string]
	Stage           Optional[Stage]
	Position        uint64
	QuestionSha256  Digest
	Model           string
	Url             string
	Requests        []Digest
	RequestsSent    uint64
	Cached          bool
	FailedQuestions uint64
	Usage           Optional[TokenUsage]
	QuestionSources []QuestionSource
	State           MemberState
	Success         Optional[ObservationSuccess]
	Failure         Optional[MemberFailure]
}

type RowObservation struct {
	Index uint64
	Value RowValue
}

type ObservationEvent struct {
	Kind     EventKind
	Question Optional[QuestionObservation]
	Row      Optional[RowObservation]
}
