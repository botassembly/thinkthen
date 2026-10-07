package thinkthen

// NewQuestion prepares typed fields; the native constructor validates grammar and readings.
func NewQuestion(kind Function, text Content) Question {
	return Question{Kind: kind, Text: text, Threshold: Rule{Kind: RuleKindDefault}, RelationThreshold: Rule{Kind: RuleKindDefault}}
}
func DecideQuestion(text Content) Question    { return NewQuestion(FunctionDecide, text) }
func ChooseQuestion(text Content) Question    { return NewQuestion(FunctionChoose, text) }
func TagQuestion(text Content) Question       { return NewQuestion(FunctionTag, text) }
func ScoreQuestion(text Content) Question     { return NewQuestion(FunctionScore, text) }
func FilterQuestion(text Content) Question    { return NewQuestion(FunctionFilter, text) }
func RankQuestion(text Content) Question      { return NewQuestion(FunctionRank, text) }
func FindQuestion(text Content) Question      { return NewQuestion(FunctionFind, text) }
func AnnotateQuestion(text Content) Question  { return NewQuestion(FunctionAnnotate, text) }
func RecognizeQuestion(text Content) Question { return NewQuestion(FunctionRecognize, text) }
func RelateQuestion(text Content) Question    { return NewQuestion(FunctionRelate, text) }
