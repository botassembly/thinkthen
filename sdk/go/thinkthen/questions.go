//go:build cgo

package thinkthen

import "reflect"

// Threshold is a cutoff or not-sure band for a question.
type Threshold interface{ threshold() }

// Cut holds one probability cutoff.
type Cut struct {
	value float64
	set   bool
}

// BandValue holds the low and high bounds of a not-sure band.
type BandValue struct{ low, high float64 }

func (Cut) threshold()       {}
func (BandValue) threshold() {}

// Cutoff selects a single probability cutoff.
func Cutoff(p float64) Cut { return Cut{p, true} }

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

// RelationRule describes a named relationship between source and target kinds.
type RelationRule struct {
	// Source names the source entity kind.
	Source string
	// Target names the target entity kind.
	Target string
	// Relation names the relationship.
	Relation string `json:"relation"`
	// Reads describes the relationship in words.
	Reads string
	// Either allows the relationship in either direction.
	Either bool
}
type question struct {
	function          Function
	text              string
	domain            []string
	rules             []RelationRule
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
	if value == nil {
		return question{invalid: true}
	}
	v := reflect.ValueOf(value)
	if v.Kind() == reflect.Pointer && v.IsNil() {
		return question{invalid: true}
	}
	if q, ok := value.(interface{ definition() question }); ok {
		return q.definition()
	}
	return question{invalid: true}
}
func copyRules(rules []RelationRule) []RelationRule { return append([]RelationRule(nil), rules...) }
func copyChildren(values map[string]AnnotationQuestion) map[string]question {
	out := make(map[string]question, len(values))
	for name, value := range values {
		out[name] = snapshot(value)
	}
	return out
}
