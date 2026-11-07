//go:build cgo

package thinkthen

// Typed selectors make tt.Answer(answers.Decide("refund")) the same operation as
// tt.Answer(client.Decide(...)), including uncertainty and failed child outcomes.
func selectAnnotation[A Annotation](a Annotations, name string) (A, error) {
	var zero A
	child, present := a.entries[name]
	if !present {
		return zero, &Error{code: CodeUsageName, message: "annotation entry not found: " + name}
	}
	result, ok := child.(A)
	if !ok {
		return zero, &Error{code: CodeUsageFunction, message: "annotation entry has a different function: " + name}
	}
	return result, nil
}

// Decide selects a decide child for tt.Answer, preserving its uncertainty and failure.
func (a Annotations) Decide(name string) (DecideAnnotation, error) {
	return selectAnnotation[DecideAnnotation](a, name)
}

// Choose selects a choose child for tt.Answer.
func (a Annotations) Choose(name string) (ChooseAnnotation, error) {
	return selectAnnotation[ChooseAnnotation](a, name)
}

// Tag selects a tag child for tt.Answer.
func (a Annotations) Tag(name string) (TagAnnotation, error) {
	return selectAnnotation[TagAnnotation](a, name)
}

// Score selects a score child for tt.Answer.
func (a Annotations) Score(name string) (ScoreAnnotation, error) {
	return selectAnnotation[ScoreAnnotation](a, name)
}

// Filter selects a filter child for tt.Answer.
func (a Annotations) Filter(name string) (FilterAnnotation, error) {
	return selectAnnotation[FilterAnnotation](a, name)
}

// Find selects a find child for tt.Answer.
func (a Annotations) Find(name string) (FindAnnotation, error) {
	return selectAnnotation[FindAnnotation](a, name)
}

// Recognize selects a recognize child for tt.Answer.
func (a Annotations) Recognize(name string) (RecognizeAnnotation, error) {
	return selectAnnotation[RecognizeAnnotation](a, name)
}

// Relate selects a relate child for tt.Answer.
func (a Annotations) Relate(name string) (RelateAnnotation, error) {
	return selectAnnotation[RelateAnnotation](a, name)
}
