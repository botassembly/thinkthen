//go:build cgo

package thinkthen

import "sort"

// Toolbox holds named saved questions.
type Toolbox struct{ entries map[string]question }

func NewToolbox(entries map[string]Question) Toolbox {
	out := make(map[string]question, len(entries))
	for k, v := range entries {
		out[k] = snapshot(v)
	}
	return Toolbox{out}
}
func sortedQuestions(q map[string]question) []string {
	names := make([]string, 0, len(q))
	for n := range q {
		names = append(names, n)
	}
	sort.Strings(names)
	return names
}
func (t Toolbox) selectQuestion(name string, function Function) (question, error) {
	q, ok := t.entries[name]
	if !ok {
		return question{}, &Error{code: CodeUsageName, message: "toolbox entry not found: " + name}
	}
	if q.function != function {
		return question{}, &Error{code: CodeUsageFunction, message: "toolbox entry has a different function: " + name}
	}
	return q, nil
}
func (t Toolbox) Decide(name string) (DecideQuestion, error) {
	q, err := t.selectQuestion(name, FunctionDecide)
	if err != nil {
		return DecideQuestion{}, err
	}
	v := DecideQuestion{}
	v.Question = q.text
	if q.field != nil {
		v.Field = *q.field
	}
	if q.name != nil {
		v.Name = *q.name
	}
	if q.yesMeans != nil {
		v.YesMeans = *q.yesMeans
	}
	if q.noMeans != nil {
		v.NoMeans = *q.noMeans
	}
	if q.threshold.kind == 1 {
		v.Threshold = Cutoff(q.threshold.low)
	} else if q.threshold.kind == 2 {
		v.Threshold = Band(q.threshold.low, q.threshold.high)
	}
	return v, nil
}
func (t Toolbox) Choose[C ~string](name string) (ChooseQuestion[C], error) {
	q, err := t.selectQuestion(name, FunctionChoose)
	if err != nil {
		return ChooseQuestion[C]{}, err
	}
	v := ChooseQuestion[C]{}
	v.Question = q.text
	if q.field != nil {
		v.Field = *q.field
	}
	if q.name != nil {
		v.Name = *q.name
	}
	if q.threshold.kind == 1 {
		v.Threshold = Cutoff(q.threshold.low)
	}
	v.Choices = make([]C, len(q.domain))
	for i, value := range q.domain {
		v.Choices[i] = C(value)
	}
	if q.descriptions != nil {
		v.Descriptions = make(map[C]string, len(q.descriptions))
		for key, value := range q.descriptions {
			v.Descriptions[C(key)] = value
		}
	}
	return v, nil
}
func (t Toolbox) Tag[C ~string](name string) (TagQuestion[C], error) {
	q, err := t.selectQuestion(name, FunctionTag)
	if err != nil {
		return TagQuestion[C]{}, err
	}
	v := TagQuestion[C]{}
	v.Question = q.text
	if q.field != nil {
		v.Field = *q.field
	}
	if q.name != nil {
		v.Name = *q.name
	}
	if q.threshold.kind == 1 {
		v.Threshold = Cutoff(q.threshold.low)
	}
	v.Labels = make([]C, len(q.domain))
	for i, value := range q.domain {
		v.Labels[i] = C(value)
	}
	if q.descriptions != nil {
		v.Descriptions = make(map[C]string, len(q.descriptions))
		for key, value := range q.descriptions {
			v.Descriptions[C(key)] = value
		}
	}
	return v, nil
}
func (t Toolbox) Score[C ~string](name string) (ScoreQuestion[C], error) {
	q, err := t.selectQuestion(name, FunctionScore)
	if err != nil {
		return ScoreQuestion[C]{}, err
	}
	v := ScoreQuestion[C]{}
	v.Question = q.text
	if q.field != nil {
		v.Field = *q.field
	}
	if q.name != nil {
		v.Name = *q.name
	}
	v.Levels = make([]C, len(q.domain))
	for i, value := range q.domain {
		v.Levels[i] = C(value)
	}
	if q.descriptions != nil {
		v.Descriptions = make(map[C]string, len(q.descriptions))
		for key, value := range q.descriptions {
			v.Descriptions[C(key)] = value
		}
	}
	return v, nil
}
func (t Toolbox) Filter(name string) (FilterQuestion, error) {
	q, err := t.selectQuestion(name, FunctionFilter)
	if err != nil {
		return FilterQuestion{}, err
	}
	v := FilterQuestion{}
	v.Question = q.text
	if q.field != nil {
		v.Field = *q.field
	}
	if q.name != nil {
		v.Name = *q.name
	}
	if q.yesMeans != nil {
		v.YesMeans = *q.yesMeans
	}
	if q.noMeans != nil {
		v.NoMeans = *q.noMeans
	}
	if q.threshold.kind == 1 {
		v.Threshold = Cutoff(q.threshold.low)
	}
	return v, nil
}
func (t Toolbox) Rank[C ~string](name string) (RankQuestion[C], error) {
	q, err := t.selectQuestion(name, FunctionRank)
	if err != nil {
		return RankQuestion[C]{}, err
	}
	v := RankQuestion[C]{}
	v.Question = q.text
	if q.field != nil {
		v.Field = *q.field
	}
	if q.name != nil {
		v.Name = *q.name
	}
	v.Levels = make([]C, len(q.domain))
	for i, value := range q.domain {
		v.Levels[i] = C(value)
	}
	if q.descriptions != nil {
		v.Descriptions = make(map[C]string, len(q.descriptions))
		for key, value := range q.descriptions {
			v.Descriptions[C(key)] = value
		}
	}
	if q.top != nil {
		v.Top = *q.top
	}
	return v, nil
}
func (t Toolbox) Find(name string) (FindQuestion, error) {
	q, err := t.selectQuestion(name, FunctionFind)
	if err != nil {
		return FindQuestion{}, err
	}
	v := FindQuestion{}
	v.Question = q.text
	if q.field != nil {
		v.Field = *q.field
	}
	if q.name != nil {
		v.Name = *q.name
	}
	if q.threshold.kind == 1 {
		v.Threshold = Cutoff(q.threshold.low)
	}
	return v, nil
}
func (t Toolbox) Recognize(name string) (RecognizeQuestion, error) {
	q, err := t.selectQuestion(name, FunctionRecognize)
	if err != nil {
		return RecognizeQuestion{}, err
	}
	v := RecognizeQuestion{}
	if q.field != nil {
		v.Field = *q.field
	}
	if q.name != nil {
		v.Name = *q.name
	}
	v.Kinds = make([]string, len(q.domain))
	for i, value := range q.domain {
		v.Kinds[i] = string(value)
	}
	if q.threshold.kind == 1 {
		v.Threshold = Cutoff(q.threshold.low)
	}
	return v, nil
}
func (t Toolbox) Relate(name string) (RelateQuestion, error) {
	q, err := t.selectQuestion(name, FunctionRelate)
	if err != nil {
		return RelateQuestion{}, err
	}
	v := RelateQuestion{}
	if q.field != nil {
		v.Field = *q.field
	}
	if q.name != nil {
		v.Name = *q.name
	}
	v.Rules = copyRules(q.rules)
	if q.threshold.kind == 1 {
		v.Threshold = Cutoff(q.threshold.low)
	}
	return v, nil
}

// Questions returns an independent map of toolbox definitions.
func (t Toolbox) Questions() map[string]Question {
	out := make(map[string]Question, len(t.entries))
	for name, q := range t.entries {
		switch q.function {
		case FunctionDecide:
			v, _ := t.Decide(name)
			out[name] = v
		case FunctionChoose:
			v, _ := t.Choose[string](name)
			out[name] = v
		case FunctionTag:
			v, _ := t.Tag[string](name)
			out[name] = v
		case FunctionScore:
			v, _ := t.Score[string](name)
			out[name] = v
		case FunctionRank:
			v, _ := t.Rank[string](name)
			out[name] = v
		case FunctionFilter:
			v, _ := t.Filter(name)
			out[name] = v
		case FunctionFind:
			v, _ := t.Find(name)
			out[name] = v
		case FunctionRecognize:
			v, _ := t.Recognize(name)
			out[name] = v
		case FunctionRelate:
			v, _ := t.Relate(name)
			out[name] = v
		}
	}
	return out
}

type annotationDefinition struct{ value question }

func (v annotationDefinition) annotationQuestion()  {}
func (v annotationDefinition) definition() question { return v.value }

// Annotate selects saved questions as one annotation form.
func (t Toolbox) Annotate(names ...string) (AnnotateQuestion, error) {
	if len(names) == 0 {
		names = sortedQuestions(t.entries)
	}
	children := make(map[string]AnnotationQuestion, len(names))
	for _, name := range names {
		q, ok := t.entries[name]
		if !ok {
			return AnnotateQuestion{}, &Error{code: CodeUsageName, message: "toolbox entry not found: " + name}
		}
		if q.function == FunctionRank || q.function == FunctionFilter {
			return AnnotateQuestion{}, &Error{code: CodeUsageQuestion, message: "collection question cannot be an annotation entry: " + name}
		}
		children[name] = annotationDefinition{q}
	}
	return AnnotateQuestion{Questions: children}, nil
}
