package thinkthen

func (q Decide) Text() string         { return q.q.text }
func (q Choose) Text() string         { return q.q.text }
func (q Tag) Text() string            { return q.q.text }
func (q Score) Text() string          { return q.q.text }
func (q Filter) Text() string         { return q.q.text }
func (q Rank) Text() string           { return q.q.text }
func (q Find) Text() string           { return q.q.text }
func (q Choose) Choices() []string    { return append([]string(nil), q.q.domain...) }
func (q Tag) Labels() []string        { return append([]string(nil), q.q.domain...) }
func (q Score) Levels() []string      { return append([]string(nil), q.q.domain...) }
func (q Rank) Levels() []string       { return append([]string(nil), q.q.domain...) }
func (q Recognize) Kinds() []string   { return append([]string(nil), q.q.domain...) }
func (q Relate) Rules() []Rule        { return copyRules(q.q.rules) }
func (q Decide) Evidence() *string    { return copyString(q.q.evidence) }
func (q Decide) Name() *string        { return copyString(q.q.name) }
func (q Choose) Evidence() *string    { return copyString(q.q.evidence) }
func (q Choose) Name() *string        { return copyString(q.q.name) }
func (q Tag) Evidence() *string       { return copyString(q.q.evidence) }
func (q Tag) Name() *string           { return copyString(q.q.name) }
func (q Score) Evidence() *string     { return copyString(q.q.evidence) }
func (q Score) Name() *string         { return copyString(q.q.name) }
func (q Filter) Evidence() *string    { return copyString(q.q.evidence) }
func (q Filter) Name() *string        { return copyString(q.q.name) }
func (q Rank) Evidence() *string      { return copyString(q.q.evidence) }
func (q Rank) Name() *string          { return copyString(q.q.name) }
func (q Find) Evidence() *string      { return copyString(q.q.evidence) }
func (q Find) Name() *string          { return copyString(q.q.name) }
func (q Recognize) Evidence() *string { return copyString(q.q.evidence) }
func (q Recognize) Name() *string     { return copyString(q.q.name) }
func (q Relate) Evidence() *string    { return copyString(q.q.evidence) }
func (q Relate) Name() *string        { return copyString(q.q.name) }
func (q Annotate) Evidence() *string  { return copyString(q.q.evidence) }
func (q Annotate) Name() *string      { return copyString(q.q.name) }
func (q Rank) Top() *int {
	if q.q.top == nil {
		return nil
	}
	value := *q.q.top
	return &value
}
func copyString(value *string) *string {
	if value == nil {
		return nil
	}
	copied := *value
	return &copied
}
