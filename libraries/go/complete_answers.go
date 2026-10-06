package thinkthen

type Probability struct {
	Name        string
	Probability float64
}

type DecideValue struct {
	Kind     ValueKind
	Boolean  bool
	Authored Optional[Content]
}

type AtomicAnswer struct {
	Kind          AtomicKind
	Probability   Optional[float64]
	Pick          Optional[string]
	Level         Optional[string]
	Probabilities []Probability
	Confidence    Optional[float64]
}

type Location struct {
	File      Optional[string]
	FirstLine Optional[uint64]
	LastLine  Optional[uint64]
}

type MemberValue struct {
	Function Function
	Decide   Optional[DecideValue]
	Choose   Optional[string]
	Tag      Optional[[]string]
	Score    Optional[float64]
}

type MemberFailure struct {
	FailureId FailureId
	Cause     MemberCause
}

type MemberSuccess struct {
	AnswerId  AnswerId
	Value     MemberValue
	Answer    AtomicAnswer
	Threshold Rule
}

type AnnotationMember struct {
	Name     string
	Request  Digest
	Question Question
	State    MemberState
	Success  Optional[MemberSuccess]
	Failure  Optional[MemberFailure]
}

type Entity struct {
	Text     string
	Start    uint64
	End      uint64
	Length   uint64
	Kind     string
	Strength float64
}

type EntityEdge struct {
	Relation    string
	Source      Entity
	Target      Entity
	Probability float64
	Either      bool
}

type Place struct {
	Start uint64
	End   uint64
}

type Piece struct {
	Start uint64
	End   uint64
	Tags  []Probability
}

type NameSpan struct {
	Start uint64
	End   uint64
	Kinds Optional[[]Probability]
	Edges Optional[[]Probability]
}

type PairSpan struct {
	Relation    string
	Source      Place
	Target      Place
	Probability float64
}

type RecognizeValue struct {
	Entities  []Entity
	Relations Optional[[]EntityEdge]
}

type RecognizeAnswer struct {
	Pieces []Piece
	Names  []NameSpan
	Pairs  []PairSpan
}

type Endpoint struct {
	Name string
	Kind string
}

type Edge struct {
	Relation    string
	Source      Endpoint
	Target      Endpoint
	Probability float64
	Either      bool
}

type RelationSuccess struct {
	AnswerId    AnswerId
	Probability float64
	Accepted    bool
}

type RelationAnswer struct {
	Relation  string
	Reads     string
	Method    RelationMethod
	Direction Direction
	Source    Endpoint
	Target    Optional[Endpoint]
	Request   Digest
	State     MemberState
	Success   Optional[RelationSuccess]
	Failure   Optional[MemberFailure]
}
