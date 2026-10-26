package thinkthen

type NoDetail struct{}
type LabelsDetail struct {
	Labels map[string]float64 `json:"labels"`
}
type LevelsDetail struct {
	Levels []float64 `json:"levels"`
}
type Span struct {
	Text       string `json:"text"`
	Start, End uint64
}
type Name struct {
	Text        string
	Start, End  uint64
	Kind        string
	Probability float64
}
type Endpoint struct {
	Text       string
	Start, End uint64
	Kind       string
}
type Relation struct {
	Source, Target Endpoint
	Label          string `json:"relation"`
	Probability    float64
	Either         *bool `json:"either,omitempty"`
}
type PairsDetail struct {
	Names []Name
	Pairs []Relation
}
type BoundaryTag uint32

const (
	Begin BoundaryTag = iota + 1
	Inside
	End
	Single
	Out
)

func (b *BoundaryTag) UnmarshalText(value []byte) error {
	switch string(value) {
	case "BEGIN":
		*b = Begin
	case "INSIDE":
		*b = Inside
	case "END":
		*b = End
	case "SINGLE":
		*b = Single
	case "OUT":
		*b = Out
	default:
		return abiError()
	}
	return nil
}

type PieceDetail struct {
	Text       string
	Start, End uint64
	Tags       map[BoundaryTag]float64
}
type KindDetail struct {
	Start, End uint64
	Kinds      map[string]float64
}
type RecognizeDetail struct {
	Pieces []PieceDetail
	Names  []KindDetail
}
type Source struct {
	QuestionIdentity string  `json:"question_identity"`
	Name             *string `json:"name,omitempty"`
	Backend          Backend
	Model            string
}
