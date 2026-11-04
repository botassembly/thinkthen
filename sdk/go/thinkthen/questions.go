package thinkthen

import "sort"

// Threshold is a cutoff or not-sure band for a question.
type Threshold interface{ threshold() }

// Cut holds one probability cutoff.
type Cut struct{ value float64 }

// BandValue holds the low and high bounds of a not-sure band.
type BandValue struct{ low, high float64 }

func (Cut) threshold()       {}
func (BandValue) threshold() {}

// Cutoff selects a single probability cutoff.
func Cutoff(p float64) Cut { return Cut{p} }

// Band selects the low and high bounds of a not-sure band.
func Band(low, high float64) BandValue { return BandValue{low, high} }

type thresholdValue struct {
	kind      uint8
	low, high float64
}

func snapshotThreshold(t Threshold) thresholdValue {
	switch v := t.(type) {
	case nil:
		return thresholdValue{}
	case Cut:
		return thresholdValue{1, v.value, 0}
	case BandValue:
		return thresholdValue{2, v.low, v.high}
	case *Cut:
		if v != nil {
			return snapshotThreshold(*v)
		}
	case *BandValue:
		if v != nil {
			return snapshotThreshold(*v)
		}
	}
	return thresholdValue{kind: 3}
}

// Rule describes a named relationship between source and target kinds.
type Rule struct {
	// Name names the relationship.
	Name string `json:"relation"`
	// Source names the source entity kind.
	Source string
	// Target names the target entity kind.
	Target string
	// Reads supplies an optional description of the relationship.
	Reads *string
	// Either allows the relationship in either direction.
	Either bool
}
type question struct {
	function          Function
	text              string
	domain            []string
	rules             []Rule
	children          map[string]question
	threshold         thresholdValue
	field, name       *string
	top               *int
	invalid           bool
	yesMeans, noMeans *string
	descriptions      map[string]string
}

// Question is a sealed typed toolbox question.
type Question interface{ question() }

// AnnotationQuestion is a sealed question that can be used inside an annotation.
type AnnotationQuestion interface{ annotationQuestion() }

func snapshot(value any) question {
	switch q := value.(type) {
	case Decide:
		return q.q
	case *Decide:
		if q != nil {
			return q.q
		}
	case Choose:
		return q.q
	case *Choose:
		if q != nil {
			return q.q
		}
	case Tag:
		return q.q
	case *Tag:
		if q != nil {
			return q.q
		}
	case Score:
		return q.q
	case *Score:
		if q != nil {
			return q.q
		}
	case Filter:
		return q.q
	case *Filter:
		if q != nil {
			return q.q
		}
	case Rank:
		return q.q
	case *Rank:
		if q != nil {
			return q.q
		}
	case Find:
		return q.q
	case *Find:
		if q != nil {
			return q.q
		}
	case Recognize:
		return q.q
	case *Recognize:
		if q != nil {
			return q.q
		}
	case Relate:
		return q.q
	case *Relate:
		if q != nil {
			return q.q
		}
	case Annotate:
		return q.q
	case *Annotate:
		if q != nil {
			return q.q
		}
	}
	return question{invalid: true}
}

// Decide holds an immutable decide question.
type Decide struct{ q question }

func (Decide) question()           {}
func (Decide) annotationQuestion() {}

// NewDecide creates a decide question with an independent copy of its arguments.
func NewDecide(text string) Decide { return Decide{question{function: FunctionDecide, text: text}} }

// WithField returns a copy selecting field at the JSON pointer.
func (v Decide) WithField(pointer string) Decide { v.q.field = &pointer; return v }

// WithName returns a copy with the given source name.
func (v Decide) WithName(name string) Decide { v.q.name = &name; return v }

// WithThreshold returns a copy using the given probability threshold.
func (v Decide) WithThreshold(t Threshold) Decide { v.q.threshold = snapshotThreshold(t); return v }

// Choose holds an immutable choose question.
type Choose struct{ q question }

func (Choose) question()           {}
func (Choose) annotationQuestion() {}

// NewChoose creates a choose question with an independent copy of its arguments.
func NewChoose(text string, choices []string) Choose {
	return Choose{question{function: FunctionChoose, text: text, domain: append([]string(nil), choices...)}}
}

// WithField returns a copy selecting field at the JSON pointer.
func (v Choose) WithField(pointer string) Choose { v.q.field = &pointer; return v }

// WithName returns a copy with the given source name.
func (v Choose) WithName(name string) Choose { v.q.name = &name; return v }

// WithThreshold returns a copy using the given probability threshold.
func (v Choose) WithThreshold(t Threshold) Choose { v.q.threshold = snapshotThreshold(t); return v }

// Tag holds an immutable tag question.
type Tag struct{ q question }

func (Tag) question()           {}
func (Tag) annotationQuestion() {}

// NewTag creates a tag question with an independent copy of its arguments.
func NewTag(text string, labels []string) Tag {
	return Tag{question{function: FunctionTag, text: text, domain: append([]string(nil), labels...)}}
}

// WithField returns a copy selecting field at the JSON pointer.
func (v Tag) WithField(pointer string) Tag { v.q.field = &pointer; return v }

// WithName returns a copy with the given source name.
func (v Tag) WithName(name string) Tag { v.q.name = &name; return v }

// WithThreshold returns a copy using the given probability threshold.
func (v Tag) WithThreshold(t Cut) Tag { v.q.threshold = snapshotThreshold(t); return v }

// Score holds an immutable score question.
type Score struct{ q question }

func (Score) question()           {}
func (Score) annotationQuestion() {}

// NewScore creates a score question with an independent copy of its arguments.
func NewScore(text string, levels []string) Score {
	return Score{question{function: FunctionScore, text: text, domain: append([]string(nil), levels...)}}
}

// WithField returns a copy selecting field at the JSON pointer.
func (v Score) WithField(pointer string) Score { v.q.field = &pointer; return v }

// WithName returns a copy with the given source name.
func (v Score) WithName(name string) Score { v.q.name = &name; return v }

// Filter holds an immutable filter question.
type Filter struct{ q question }

func (Filter) question()           {}
func (Filter) annotationQuestion() {}

// NewFilter creates a filter question with an independent copy of its arguments.
func NewFilter(text string) Filter { return Filter{question{function: FunctionFilter, text: text}} }

// WithField returns a copy selecting field at the JSON pointer.
func (v Filter) WithField(pointer string) Filter { v.q.field = &pointer; return v }

// WithName returns a copy with the given source name.
func (v Filter) WithName(name string) Filter { v.q.name = &name; return v }

// WithThreshold returns a copy using the given probability threshold.
func (v Filter) WithThreshold(t Threshold) Filter { v.q.threshold = snapshotThreshold(t); return v }

// Rank holds an immutable rank question.
type Rank struct{ q question }

func (Rank) question() {}

// NewRank creates a rank question with an independent copy of its arguments.
func NewRank(text string, levels []string) Rank {
	return Rank{question{function: FunctionRank, text: text, domain: append([]string(nil), levels...)}}
}

// WithField returns a copy selecting field at the JSON pointer.
func (v Rank) WithField(pointer string) Rank { v.q.field = &pointer; return v }

// WithName returns a copy with the given source name.
func (v Rank) WithName(name string) Rank { v.q.name = &name; return v }

// WithTop returns a copy limiting the number of ranked results.
func (v Rank) WithTop(top int) Rank { v.q.top = &top; return v }

// Find holds an immutable find question.
type Find struct{ q question }

func (Find) question()           {}
func (Find) annotationQuestion() {}

// NewFind creates a find question with an independent copy of its arguments.
func NewFind(text string) Find { return Find{question{function: FunctionFind, text: text}} }

// WithField returns a copy selecting field at the JSON pointer.
func (v Find) WithField(pointer string) Find { v.q.field = &pointer; return v }

// WithName returns a copy with the given source name.
func (v Find) WithName(name string) Find { v.q.name = &name; return v }

// WithThreshold returns a copy using the given probability threshold.
func (v Find) WithThreshold(t Cut) Find { v.q.threshold = snapshotThreshold(t); return v }

// Recognize holds an immutable recognize question.
type Recognize struct{ q question }

func (Recognize) question()           {}
func (Recognize) annotationQuestion() {}

// NewRecognize creates a recognize question with an independent copy of its arguments.
func NewRecognize(kinds []string) Recognize {
	return Recognize{question{function: FunctionRecognize, domain: append([]string(nil), kinds...)}}
}

// WithField returns a copy selecting field at the JSON pointer.
func (v Recognize) WithField(pointer string) Recognize { v.q.field = &pointer; return v }

// WithName returns a copy with the given source name.
func (v Recognize) WithName(name string) Recognize { v.q.name = &name; return v }

// WithThreshold returns a copy using the given probability threshold.
func (v Recognize) WithThreshold(t Cut) Recognize { v.q.threshold = snapshotThreshold(t); return v }

// Relate holds an immutable relate question.
type Relate struct{ q question }

func (Relate) question()           {}
func (Relate) annotationQuestion() {}

// NewRelate creates a relate question with an independent copy of its arguments.
func NewRelate(rules []Rule) Relate {
	return Relate{question{function: FunctionRelate, rules: copyRules(rules)}}
}

// WithField returns a copy selecting field at the JSON pointer.
func (v Relate) WithField(pointer string) Relate { v.q.field = &pointer; return v }

// WithName returns a copy with the given source name.
func (v Relate) WithName(name string) Relate { v.q.name = &name; return v }

// WithThreshold returns a copy using the given probability threshold.
func (v Relate) WithThreshold(t Cut) Relate { v.q.threshold = snapshotThreshold(t); return v }

// Annotate holds an immutable annotate question.
type Annotate struct{ q question }

// NewAnnotate creates a annotate question with an independent copy of its arguments.
func NewAnnotate(questions map[string]AnnotationQuestion) Annotate {
	return Annotate{question{function: FunctionAnnotate, children: copyChildren(questions)}}
}

// WithField returns a copy selecting field at the JSON pointer.
func (v Annotate) WithField(pointer string) Annotate { v.q.field = &pointer; return v }

// WithName returns a copy with the given source name.
func (v Annotate) WithName(name string) Annotate { v.q.name = &name; return v }
func copyRules(rules []Rule) []Rule {
	out := append([]Rule(nil), rules...)
	for i := range out {
		if out[i].Reads != nil {
			v := *out[i].Reads
			out[i].Reads = &v
		}
	}
	return out
}
func copyChildren(values map[string]AnnotationQuestion) map[string]question {
	out := make(map[string]question, len(values))
	for name, value := range values {
		out[name] = snapshot(value)
	}
	return out
}

// Toolbox owns named questions that can be saved and selected by type.
type Toolbox struct{ entries map[string]question }

// NewToolbox creates a toolbox question with an independent copy of its arguments.
func NewToolbox(entries map[string]Question) Toolbox {
	out := make(map[string]question, len(entries))
	for name, value := range entries {
		out[name] = snapshot(value)
	}
	return Toolbox{out}
}

// Questions returns an independent map of typed toolbox questions.
func (t Toolbox) Questions() map[string]Question {
	out := make(map[string]Question, len(t.entries))
	for name, q := range t.entries {
		switch q.function {
		case FunctionDecide:
			out[name] = Decide{q}
		case FunctionChoose:
			out[name] = Choose{q}
		case FunctionTag:
			out[name] = Tag{q}
		case FunctionScore:
			out[name] = Score{q}
		case FunctionFilter:
			out[name] = Filter{q}
		case FunctionRank:
			out[name] = Rank{q}
		case FunctionFind:
			out[name] = Find{q}
		case FunctionRecognize:
			out[name] = Recognize{q}
		case FunctionRelate:
			out[name] = Relate{q}
		}
	}
	return out
}

// Decide selects the named decide question, if present with the correct type.
func (t Toolbox) Decide(name string) (Decide, bool) {
	q, ok := t.entries[name]
	if !ok || q.invalid || q.function != FunctionDecide {
		return Decide{}, false
	}
	return Decide{q}, true
}

// Choose selects the named choose question, if present with the correct type.
func (t Toolbox) Choose(name string) (Choose, bool) {
	q, ok := t.entries[name]
	if !ok || q.invalid || q.function != FunctionChoose {
		return Choose{}, false
	}
	return Choose{q}, true
}

// Tag selects the named tag question, if present with the correct type.
func (t Toolbox) Tag(name string) (Tag, bool) {
	q, ok := t.entries[name]
	if !ok || q.invalid || q.function != FunctionTag {
		return Tag{}, false
	}
	return Tag{q}, true
}

// Score selects the named score question, if present with the correct type.
func (t Toolbox) Score(name string) (Score, bool) {
	q, ok := t.entries[name]
	if !ok || q.invalid || q.function != FunctionScore {
		return Score{}, false
	}
	return Score{q}, true
}

// Filter selects the named filter question, if present with the correct type.
func (t Toolbox) Filter(name string) (Filter, bool) {
	q, ok := t.entries[name]
	if !ok || q.invalid || q.function != FunctionFilter {
		return Filter{}, false
	}
	return Filter{q}, true
}

// Rank selects the named rank question, if present with the correct type.
func (t Toolbox) Rank(name string) (Rank, bool) {
	q, ok := t.entries[name]
	if !ok || q.invalid || q.function != FunctionRank {
		return Rank{}, false
	}
	return Rank{q}, true
}

// Find selects the named find question, if present with the correct type.
func (t Toolbox) Find(name string) (Find, bool) {
	q, ok := t.entries[name]
	if !ok || q.invalid || q.function != FunctionFind {
		return Find{}, false
	}
	return Find{q}, true
}

// Recognize selects the named recognize question, if present with the correct type.
func (t Toolbox) Recognize(name string) (Recognize, bool) {
	q, ok := t.entries[name]
	if !ok || q.invalid || q.function != FunctionRecognize {
		return Recognize{}, false
	}
	return Recognize{q}, true
}

// Relate selects the named relate question, if present with the correct type.
func (t Toolbox) Relate(name string) (Relate, bool) {
	q, ok := t.entries[name]
	if !ok || q.invalid || q.function != FunctionRelate {
		return Relate{}, false
	}
	return Relate{q}, true
}
func sortedQuestions(q map[string]question) []string {
	names := make([]string, 0, len(q))
	for name := range q {
		names = append(names, name)
	}
	sort.Strings(names)
	return names
}

// Annotate selects named toolbox questions, or all entries when names is empty.
// Rank questions cannot be annotation children and return a usage error.
func (t Toolbox) Annotate(names ...string) (Annotate, error) {
	questions := t.Questions()
	if len(names) == 0 {
		names = sortedQuestions(t.entries)
	}
	entries := make(map[string]AnnotationQuestion, len(names))
	for _, name := range names {
		q, ok := questions[name]
		if !ok {
			return Annotate{}, &Error{code: CodeLocalToolbox, message: "toolbox entry not found: " + name}
		}
		child, ok := q.(AnnotationQuestion)
		if !ok {
			return Annotate{}, &Error{code: CodeUsageQuestion, message: "annotation entry cannot be a rank question: " + name}
		}
		entries[name] = child
	}
	return NewAnnotate(entries), nil
}
