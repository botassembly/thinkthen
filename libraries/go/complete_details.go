package thinkthen

// Author metadata is supplied explicitly and validated by the native grammar.
type DeclarationKind uint32
type PropertyKind uint32

const (
	DeclarationAbsent DeclarationKind = iota
	DeclarationString
	DeclarationObject
)
const (
	PropertyString PropertyKind = 1 + iota
	PropertyNumber
	PropertyBoolean
	PropertyStringList
)

type InputProperty struct {
	Name string
	Kind PropertyKind
}
type InputDeclaration struct {
	Kind       DeclarationKind
	Properties []InputProperty
	Required   []string
}
type QuestionAuthor struct {
	Name           Optional[string]
	WordingVersion Optional[uint64]
	ItemSchema     InputDeclaration
	ContextSchema  InputDeclaration
}
type ReportedUsage struct {
	Present      bool
	InputTokens  Optional[uint64]
	OutputTokens Optional[uint64]
}
type SourceDetail struct {
	Origin     Origin
	AnsweredBy string
	BatchSize  Optional[uint64]
}
type InputView struct {
	Original Optional[Content]
	Position Optional[Location]
	Images   Optional[[]ImageView]
}
type Details struct {
	Question        Optional[Question]
	Threshold       Optional[Rule]
	RawPick         Optional[string]
	Usage           ReportedUsage
	QuestionSources []SourceDetail
	Observations    []ObservationIdentity
	Inputs          []InputView
}
type SourceEntity struct {
	Entity   Entity
	Position Optional[Location]
}
type SourceEntityEdge struct {
	Relation    string
	Source      SourceEntity
	Target      SourceEntity
	Probability float64
	Either      bool
}
type SourceRecognition struct {
	Present   bool
	Entities  []SourceEntity
	Relations Optional[[]SourceEntityEdge]
}
type SourceEndpoint struct {
	Ordinal  uint64
	Endpoint Endpoint
	Record   Content
	Position Optional[Location]
}
type SourceEdge struct {
	Relation    string
	Source      SourceEndpoint
	Target      SourceEndpoint
	Probability float64
	Either      bool
}
type SourceRelations struct {
	Present bool
	Edges   []SourceEdge
}
