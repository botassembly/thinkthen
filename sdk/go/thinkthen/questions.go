package thinkthen

import "sort"

type Threshold interface{ threshold() }
type Cut struct{ value float64 }
type BandValue struct{ low, high float64 }

func (Cut) threshold()                 {}
func (BandValue) threshold()           {}
func Cutoff(p float64) Cut             { return Cut{p} }
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

type Rule struct {
	Name           string `json:"relation"`
	Source, Target string
	Reads          *string
	Either         bool
}
type question struct {
	function       Function
	text           string
	domain         []string
	rules          []Rule
	children       map[string]question
	threshold      thresholdValue
	evidence, name *string
	top            *int
	invalid        bool
}
type Question interface{ question() }
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

type Decide struct{ q question }

func (Decide) question()                            {}
func (Decide) annotationQuestion()                  {}
func NewDecide(text string) Decide                  { return Decide{question{function: FunctionDecide, text: text}} }
func (v Decide) WithEvidence(pointer string) Decide { v.q.evidence = &pointer; return v }
func (v Decide) WithName(name string) Decide        { v.q.name = &name; return v }
func (v Decide) WithThreshold(t Threshold) Decide   { v.q.threshold = snapshotThreshold(t); return v }

type Choose struct{ q question }

func (Choose) question()           {}
func (Choose) annotationQuestion() {}
func NewChoose(text string, choices []string) Choose {
	return Choose{question{function: FunctionChoose, text: text, domain: append([]string(nil), choices...)}}
}
func (v Choose) WithEvidence(pointer string) Choose { v.q.evidence = &pointer; return v }
func (v Choose) WithName(name string) Choose        { v.q.name = &name; return v }
func (v Choose) WithThreshold(t Threshold) Choose   { v.q.threshold = snapshotThreshold(t); return v }

type Tag struct{ q question }

func (Tag) question()           {}
func (Tag) annotationQuestion() {}
func NewTag(text string, labels []string) Tag {
	return Tag{question{function: FunctionTag, text: text, domain: append([]string(nil), labels...)}}
}
func (v Tag) WithEvidence(pointer string) Tag { v.q.evidence = &pointer; return v }
func (v Tag) WithName(name string) Tag        { v.q.name = &name; return v }
func (v Tag) WithThreshold(t Cut) Tag         { v.q.threshold = snapshotThreshold(t); return v }

type Score struct{ q question }

func (Score) question()           {}
func (Score) annotationQuestion() {}
func NewScore(text string, levels []string) Score {
	return Score{question{function: FunctionScore, text: text, domain: append([]string(nil), levels...)}}
}
func (v Score) WithEvidence(pointer string) Score { v.q.evidence = &pointer; return v }
func (v Score) WithName(name string) Score        { v.q.name = &name; return v }

type Filter struct{ q question }

func (Filter) question()                            {}
func (Filter) annotationQuestion()                  {}
func NewFilter(text string) Filter                  { return Filter{question{function: FunctionFilter, text: text}} }
func (v Filter) WithEvidence(pointer string) Filter { v.q.evidence = &pointer; return v }
func (v Filter) WithName(name string) Filter        { v.q.name = &name; return v }
func (v Filter) WithThreshold(t Threshold) Filter   { v.q.threshold = snapshotThreshold(t); return v }

type Rank struct{ q question }

func (Rank) question() {}
func NewRank(text string, levels []string) Rank {
	return Rank{question{function: FunctionRank, text: text, domain: append([]string(nil), levels...)}}
}
func (v Rank) WithEvidence(pointer string) Rank { v.q.evidence = &pointer; return v }
func (v Rank) WithName(name string) Rank        { v.q.name = &name; return v }
func (v Rank) WithTop(top int) Rank             { v.q.top = &top; return v }

type Find struct{ q question }

func (Find) question()                          {}
func (Find) annotationQuestion()                {}
func NewFind(text string) Find                  { return Find{question{function: FunctionFind, text: text}} }
func (v Find) WithEvidence(pointer string) Find { v.q.evidence = &pointer; return v }
func (v Find) WithName(name string) Find        { v.q.name = &name; return v }
func (v Find) WithThreshold(t Cut) Find         { v.q.threshold = snapshotThreshold(t); return v }

type Recognize struct{ q question }

func (Recognize) question()           {}
func (Recognize) annotationQuestion() {}
func NewRecognize(kinds []string) Recognize {
	return Recognize{question{function: FunctionRecognize, domain: append([]string(nil), kinds...)}}
}
func (v Recognize) WithEvidence(pointer string) Recognize { v.q.evidence = &pointer; return v }
func (v Recognize) WithName(name string) Recognize        { v.q.name = &name; return v }
func (v Recognize) WithThreshold(t Cut) Recognize         { v.q.threshold = snapshotThreshold(t); return v }

type Relate struct{ q question }

func (Relate) question()           {}
func (Relate) annotationQuestion() {}
func NewRelate(rules []Rule) Relate {
	return Relate{question{function: FunctionRelate, rules: copyRules(rules)}}
}
func (v Relate) WithEvidence(pointer string) Relate { v.q.evidence = &pointer; return v }
func (v Relate) WithName(name string) Relate        { v.q.name = &name; return v }
func (v Relate) WithThreshold(t Cut) Relate         { v.q.threshold = snapshotThreshold(t); return v }

type Annotate struct{ q question }

func NewAnnotate(questions map[string]AnnotationQuestion) Annotate {
	return Annotate{question{function: FunctionAnnotate, children: copyChildren(questions)}}
}
func (v Annotate) WithEvidence(pointer string) Annotate { v.q.evidence = &pointer; return v }
func (v Annotate) WithName(name string) Annotate        { v.q.name = &name; return v }
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

type Toolbox struct{ entries map[string]question }

func NewToolbox(entries map[string]Question) Toolbox {
	out := make(map[string]question, len(entries))
	for name, value := range entries {
		out[name] = snapshot(value)
	}
	return Toolbox{out}
}
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
func (t Toolbox) Decide(name string) (Decide, bool) {
	q, ok := t.entries[name]
	if !ok || q.invalid || q.function != FunctionDecide {
		return Decide{}, false
	}
	return Decide{q}, true
}
func (t Toolbox) Choose(name string) (Choose, bool) {
	q, ok := t.entries[name]
	if !ok || q.invalid || q.function != FunctionChoose {
		return Choose{}, false
	}
	return Choose{q}, true
}
func (t Toolbox) Tag(name string) (Tag, bool) {
	q, ok := t.entries[name]
	if !ok || q.invalid || q.function != FunctionTag {
		return Tag{}, false
	}
	return Tag{q}, true
}
func (t Toolbox) Score(name string) (Score, bool) {
	q, ok := t.entries[name]
	if !ok || q.invalid || q.function != FunctionScore {
		return Score{}, false
	}
	return Score{q}, true
}
func (t Toolbox) Filter(name string) (Filter, bool) {
	q, ok := t.entries[name]
	if !ok || q.invalid || q.function != FunctionFilter {
		return Filter{}, false
	}
	return Filter{q}, true
}
func (t Toolbox) Rank(name string) (Rank, bool) {
	q, ok := t.entries[name]
	if !ok || q.invalid || q.function != FunctionRank {
		return Rank{}, false
	}
	return Rank{q}, true
}
func (t Toolbox) Find(name string) (Find, bool) {
	q, ok := t.entries[name]
	if !ok || q.invalid || q.function != FunctionFind {
		return Find{}, false
	}
	return Find{q}, true
}
func (t Toolbox) Recognize(name string) (Recognize, bool) {
	q, ok := t.entries[name]
	if !ok || q.invalid || q.function != FunctionRecognize {
		return Recognize{}, false
	}
	return Recognize{q}, true
}
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
