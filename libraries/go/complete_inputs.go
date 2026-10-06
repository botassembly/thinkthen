package thinkthen

import "encoding/json"

type Content struct {
	Kind ContentKind
	Text string
	Json json.RawMessage
}

type Rule struct {
	Kind RuleKind
	Low  float64
	High float64
}

type Choice struct {
	Name        string
	Description Optional[Content]
	Weight      Optional[float64]
}

type Relation struct {
	Name   string
	Source string
	Target string
	Reads  Optional[string]
	Either bool
	Single bool
}

type QuestionMember struct {
	Name     string
	Question Question
}

type Question struct {
	Kind              Function
	Text              Content
	Yes               Optional[Content]
	No                Optional[Content]
	Choices           []Choice
	Threshold         Rule
	RelationThreshold Rule
	Model             Optional[string]
	Profile           Optional[string]
	Batch             Optional[uint64]
	BatchMax          bool
	None              bool
	On                []string
	Members           []QuestionMember
	Kinds             []Choice
	Relations         []Relation
	NamePointer       Optional[string]
	KindPointer       Optional[string]
}

type ImageInput struct {
	Media    Media
	Bytes    []byte
	Filename Optional[string]
}

type ImageView struct {
	Media    Media
	Bytes    []byte
	Width    uint64
	Height   uint64
	Filename Optional[string]
}

type RecordInput struct {
	Original Optional[Content]
	Context  Optional[Content]
	Options  []Choice
	Images   []ImageInput
}

type FileSource struct {
	Paths  []string
	Unit   SourceUnit
	Window uint64
}

type CallControls struct {
	Context  Optional[Content]
	Batch    Optional[uint64]
	BatchMax bool
	Attempts bool
}
