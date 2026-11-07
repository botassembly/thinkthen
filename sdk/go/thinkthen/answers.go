//go:build cgo

package thinkthen

// NoDetail marks functions without a detail payload.
type NoDetail struct{}

// LabelsDetail holds probabilities by label.
type LabelsDetail struct {
	// Labels maps each label to its probability.
	Labels map[string]float64 `json:"labels"`
}

// LevelsDetail holds probabilities in the question level order.
type LevelsDetail struct {
	// Levels lists probabilities in the question level order.
	Levels []float64 `json:"levels"`
}

// Span is matched text with UTF-8 byte offsets; a nil span means no match.
type Span struct {
	// Text is the matched original text.
	Text string `json:"text"`
	// Start is the inclusive UTF-8 byte offset.
	Start int
	// End is the exclusive UTF-8 byte offset.
	End int
}

// Name is a recognized entity with UTF-8 byte offsets and its probability.
type Name struct {
	// Text is the matched original text.
	Text string
	// Start is the inclusive UTF-8 byte offset.
	Start int
	// End is the exclusive UTF-8 byte offset.
	End int
	// Kind names the recognized entity kind.
	Kind string
	// Probability is the probability of this entity or relation.
	Probability float64
}

// Endpoint is a relation endpoint with text, kind and UTF-8 byte offsets.
type Endpoint struct {
	// Text is the matched original text.
	Text string
	// Start is the inclusive UTF-8 byte offset.
	Start int
	// End is the exclusive UTF-8 byte offset.
	End int
	// Kind names the recognized entity kind.
	Kind string
}

// Relation is a labeled relationship between two endpoints.
type Relation struct {
	// Source is the relationship source endpoint.
	Source Endpoint
	// Target is the relationship target endpoint.
	Target Endpoint
	// Label names the relationship.
	Label string `json:"relation"`
	// Probability is the probability of this entity or relation.
	Probability float64
	// Either reports an undirected relation when present.
	Either *bool `json:"either,omitempty"`
}

// PairsDetail holds recognized names and candidate relations.
type PairsDetail struct {
	// Names lists recognized-name detail in original order.
	Names []Name
	// Pairs lists candidate relation detail.
	Pairs []Relation
}

// BoundaryTag identifies a recognition token boundary.
type BoundaryTag uint32

const (
	// Begin marks the first token of a multi-token name.
	Begin BoundaryTag = iota + 1
	// Inside marks an interior name token.
	Inside
	// End marks the last token of a multi-token name.
	End
	// Single marks a complete single-token name.
	Single
	// Out marks a token outside a name.
	Out
)

// UnmarshalText decodes the engine boundary tag.
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

// PieceDetail holds one token and its boundary probabilities.
type PieceDetail struct {
	// Text is the matched original text.
	Text string
	// Start is the inclusive UTF-8 byte offset.
	Start int
	// End is the exclusive UTF-8 byte offset.
	End int
	// Tags maps token boundary tags to probabilities.
	Tags map[BoundaryTag]float64
}

// KindDetail holds recognized-name offsets and probabilities by kind.
type KindDetail struct {
	// Text is the matched original text.
	Text string
	// Start is the inclusive UTF-8 byte offset.
	Start int
	// End is the exclusive UTF-8 byte offset.
	End int
	// Kinds maps entity kind names to probabilities.
	Kinds map[string]float64
}

// RecognizeDetail holds token boundaries and recognized-name probabilities.
type RecognizeDetail struct {
	// Pieces lists token boundary detail in original order.
	Pieces []PieceDetail
	// Names lists recognized-name detail in original order.
	Names []KindDetail
}

// Source identifies the admitted question and backend that produced a row.
type Source struct {
	// QuestionIdentity is the admitted question identity.
	QuestionIdentity string `json:"question_identity"`
	// Name is the optional source name.
	Name *string `json:"name,omitempty"`
	// Backend identifies the selected backend.
	Backend Backend
	// Model identifies the selected model.
	Model string
}
