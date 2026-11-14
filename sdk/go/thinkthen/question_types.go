//go:build cgo

package thinkthen

// DecideQuestion describes a decide call.
type DecideQuestion struct {
	Question  string
	Field     string
	Name      string
	YesMeans  string
	NoMeans   string
	Threshold Threshold
}

func (v DecideQuestion) question()           {}
func (v DecideQuestion) annotationQuestion() {}
func (v DecideQuestion) definition() question {
	q := question{function: FunctionDecide}
	q.text = v.Question
	if v.Field != "" {
		q.field = &v.Field
	}
	if v.Name != "" {
		q.name = &v.Name
	}
	if v.YesMeans != "" {
		q.yesMeans = &v.YesMeans
	}
	if v.NoMeans != "" {
		q.noMeans = &v.NoMeans
	}
	q.threshold = snapshotThreshold(v.Threshold)
	return q
}

// ChooseQuestion describes a choose call.
type ChooseQuestion[C ~string] struct {
	Question     string
	Field        string
	Name         string
	Threshold    Cut
	Choices      []C
	Descriptions map[C]string
}

func (v ChooseQuestion[C]) question()           {}
func (v ChooseQuestion[C]) annotationQuestion() {}
func (v ChooseQuestion[C]) definition() question {
	q := question{function: FunctionChoose}
	q.text = v.Question
	if v.Field != "" {
		q.field = &v.Field
	}
	if v.Name != "" {
		q.name = &v.Name
	}
	if v.Threshold.set {
		q.threshold = snapshotThreshold(v.Threshold)
	}
	q.domain = make([]string, len(v.Choices))
	for i, value := range v.Choices {
		q.domain[i] = string(value)
	}
	if v.Descriptions != nil {
		q.descriptions = make(map[string]string, len(v.Descriptions))
		for key, value := range v.Descriptions {
			q.descriptions[string(key)] = value
		}
	}
	return q
}

// TagQuestion describes a tag call.
type TagQuestion[C ~string] struct {
	Question     string
	Field        string
	Name         string
	Threshold    Cut
	Labels       []C
	Descriptions map[C]string
}

func (v TagQuestion[C]) question()           {}
func (v TagQuestion[C]) annotationQuestion() {}
func (v TagQuestion[C]) definition() question {
	q := question{function: FunctionTag}
	q.text = v.Question
	if v.Field != "" {
		q.field = &v.Field
	}
	if v.Name != "" {
		q.name = &v.Name
	}
	if v.Threshold.set {
		q.threshold = snapshotThreshold(v.Threshold)
	}
	q.domain = make([]string, len(v.Labels))
	for i, value := range v.Labels {
		q.domain[i] = string(value)
	}
	if v.Descriptions != nil {
		q.descriptions = make(map[string]string, len(v.Descriptions))
		for key, value := range v.Descriptions {
			q.descriptions[string(key)] = value
		}
	}
	return q
}

// ScoreQuestion describes a score call.
type ScoreQuestion[C ~string] struct {
	Question     string
	Field        string
	Name         string
	Levels       []C
	Descriptions map[C]string
}

func (v ScoreQuestion[C]) question()           {}
func (v ScoreQuestion[C]) annotationQuestion() {}
func (v ScoreQuestion[C]) definition() question {
	q := question{function: FunctionScore}
	q.text = v.Question
	if v.Field != "" {
		q.field = &v.Field
	}
	if v.Name != "" {
		q.name = &v.Name
	}
	q.domain = make([]string, len(v.Levels))
	for i, value := range v.Levels {
		q.domain[i] = string(value)
	}
	if v.Descriptions != nil {
		q.descriptions = make(map[string]string, len(v.Descriptions))
		for key, value := range v.Descriptions {
			q.descriptions[string(key)] = value
		}
	}
	return q
}

// FilterQuestion describes a filter call.
type FilterQuestion struct {
	Question  string
	Field     string
	Name      string
	YesMeans  string
	NoMeans   string
	Threshold Cut
}

func (v FilterQuestion) question() {}
func (v FilterQuestion) definition() question {
	q := question{function: FunctionFilter}
	q.text = v.Question
	if v.Field != "" {
		q.field = &v.Field
	}
	if v.Name != "" {
		q.name = &v.Name
	}
	if v.YesMeans != "" {
		q.yesMeans = &v.YesMeans
	}
	if v.NoMeans != "" {
		q.noMeans = &v.NoMeans
	}
	if v.Threshold.set {
		q.threshold = snapshotThreshold(v.Threshold)
	}
	return q
}

// RankQuestion describes a rank call.
type RankQuestion[C ~string] struct {
	Question     string
	Field        string
	Name         string
	Levels       []C
	Descriptions map[C]string
	Top          int
}

func (v RankQuestion[C]) question() {}
func (v RankQuestion[C]) definition() question {
	q := question{function: FunctionRank}
	q.text = v.Question
	if v.Field != "" {
		q.field = &v.Field
	}
	if v.Name != "" {
		q.name = &v.Name
	}
	q.domain = make([]string, len(v.Levels))
	for i, value := range v.Levels {
		q.domain[i] = string(value)
	}
	if v.Descriptions != nil {
		q.descriptions = make(map[string]string, len(v.Descriptions))
		for key, value := range v.Descriptions {
			q.descriptions[string(key)] = value
		}
	}
	if v.Top != 0 {
		q.top = &v.Top
	}
	return q
}

// FindQuestion describes a find call.
type FindQuestion struct {
	Question  string
	Field     string
	Name      string
	Threshold Cut
}

func (v FindQuestion) question()           {}
func (v FindQuestion) annotationQuestion() {}
func (v FindQuestion) definition() question {
	q := question{function: FunctionFind}
	q.text = v.Question
	if v.Field != "" {
		q.field = &v.Field
	}
	if v.Name != "" {
		q.name = &v.Name
	}
	if v.Threshold.set {
		q.threshold = snapshotThreshold(v.Threshold)
	}
	return q
}

// RecognizeQuestion describes a recognize call.
type RecognizeQuestion struct {
	Threshold Cut
	Field     string
	Name      string
	Kinds     []string
}

func (v RecognizeQuestion) question()           {}
func (v RecognizeQuestion) annotationQuestion() {}
func (v RecognizeQuestion) definition() question {
	q := question{function: FunctionRecognize}
	if v.Threshold.set {
		q.threshold = snapshotThreshold(v.Threshold)
	}
	if v.Field != "" {
		q.field = &v.Field
	}
	if v.Name != "" {
		q.name = &v.Name
	}
	q.domain = make([]string, len(v.Kinds))
	for i, value := range v.Kinds {
		q.domain[i] = string(value)
	}
	return q
}

// RelateQuestion describes a relate call.
type RelateQuestion struct {
	Threshold Cut
	Field     string
	Name      string
	Rules     []RelationRule
}

func (v RelateQuestion) question()           {}
func (v RelateQuestion) annotationQuestion() {}
func (v RelateQuestion) definition() question {
	q := question{function: FunctionRelate}
	if v.Threshold.set {
		q.threshold = snapshotThreshold(v.Threshold)
	}
	if v.Field != "" {
		q.field = &v.Field
	}
	if v.Name != "" {
		q.name = &v.Name
	}
	q.rules = copyRules(v.Rules)
	return q
}

// AnnotateQuestion describes a annotate call.
type AnnotateQuestion struct {
	Field     string
	Name      string
	Questions map[string]AnnotationQuestion
}

func (v AnnotateQuestion) definition() question {
	q := question{function: FunctionAnnotate}
	if v.Field != "" {
		q.field = &v.Field
	}
	if v.Name != "" {
		q.name = &v.Name
	}
	q.children = copyChildren(v.Questions)
	return q
}
