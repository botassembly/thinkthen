//go:build cgo

package thinkthen

// Text returns the question text.
func (v Decide) Text() string { return v.q.text }

// Text returns the question text.
func (v Choose) Text() string { return v.q.text }

// Text returns the question text.
func (v Tag) Text() string { return v.q.text }

// Text returns the question text.
func (v Score) Text() string { return v.q.text }

// Text returns the question text.
func (v Filter) Text() string { return v.q.text }

// Text returns the question text.
func (v Rank) Text() string { return v.q.text }

// Text returns the question text.
func (v Find) Text() string { return v.q.text }

// Choices returns a copy of the ordered choices.
func (v Choose) Choices() []string { return append([]string(nil), v.q.domain...) }

// Labels returns a copy of the ordered labels.
func (v Tag) Labels() []string { return append([]string(nil), v.q.domain...) }

// Levels returns a copy of the ordered levels.
func (v Score) Levels() []string { return append([]string(nil), v.q.domain...) }

// Levels returns a copy of the ordered levels.
func (v Rank) Levels() []string { return append([]string(nil), v.q.domain...) }

// Kinds returns a copy of the entity kinds.
func (v Recognize) Kinds() []string { return append([]string(nil), v.q.domain...) }

// Rules returns an independent copy of the relation rules.
func (v Relate) Rules() []Rule { return copyRules(v.q.rules) }

// Field returns a copy of the optional field pointer.
func (v Decide) Field() *string { return copyString(v.q.field) }

// Name returns a copy of the optional source name.
func (v Decide) Name() *string { return copyString(v.q.name) }

// Field returns a copy of the optional field pointer.
func (v Choose) Field() *string { return copyString(v.q.field) }

// Name returns a copy of the optional source name.
func (v Choose) Name() *string { return copyString(v.q.name) }

// Field returns a copy of the optional field pointer.
func (v Tag) Field() *string { return copyString(v.q.field) }

// Name returns a copy of the optional source name.
func (v Tag) Name() *string { return copyString(v.q.name) }

// Field returns a copy of the optional field pointer.
func (v Score) Field() *string { return copyString(v.q.field) }

// Name returns a copy of the optional source name.
func (v Score) Name() *string { return copyString(v.q.name) }

// Field returns a copy of the optional field pointer.
func (v Filter) Field() *string { return copyString(v.q.field) }

// Name returns a copy of the optional source name.
func (v Filter) Name() *string { return copyString(v.q.name) }

// Field returns a copy of the optional field pointer.
func (v Rank) Field() *string { return copyString(v.q.field) }

// Name returns a copy of the optional source name.
func (v Rank) Name() *string { return copyString(v.q.name) }

// Field returns a copy of the optional field pointer.
func (v Find) Field() *string { return copyString(v.q.field) }

// Name returns a copy of the optional source name.
func (v Find) Name() *string { return copyString(v.q.name) }

// Field returns a copy of the optional field pointer.
func (v Recognize) Field() *string { return copyString(v.q.field) }

// Name returns a copy of the optional source name.
func (v Recognize) Name() *string { return copyString(v.q.name) }

// Field returns a copy of the optional field pointer.
func (v Relate) Field() *string { return copyString(v.q.field) }

// Name returns a copy of the optional source name.
func (v Relate) Name() *string { return copyString(v.q.name) }

// Field returns a copy of the optional field pointer.
func (v Annotate) Field() *string { return copyString(v.q.field) }

// Name returns a copy of the optional source name.
func (v Annotate) Name() *string { return copyString(v.q.name) }

// Top returns a copy of the optional rank limit.
func (v Rank) Top() *int {
	if v.q.top == nil {
		return nil
	}
	value := *v.q.top
	return &value
}
func copyString(value *string) *string {
	if value == nil {
		return nil
	}
	copied := *value
	return &copied
}
