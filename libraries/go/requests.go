package thinkthen

import "encoding/json"

// QuestionInput selects a typed question or a user-named native question file.
type QuestionInput struct {
	Role      QuestionRole
	Saved     Optional[json.RawMessage]
	Named     Optional[string]
	Reference Optional[string]
	Question  Optional[Question]
	File      Optional[string]
}

func Asked(question Question) QuestionInput  { return QuestionInput{Question: Some(question)} }
func QuestionFile(path string) QuestionInput { return QuestionInput{File: Some(path)} }

type InputSource struct {
	Records Optional[[]RecordInput]
	Files   Optional[FileSource]
}

func Records(records []RecordInput) InputSource { return InputSource{Records: Some(records)} }
func SourceFiles(files FileSource) InputSource  { return InputSource{Files: Some(files)} }

// CompleteRequest is prepared host data, not a JSON compatibility request.
// Its execution awaits the complete C constructors/calls; no native handle is held.
type CompleteRequest struct {
	Function Function
	Question QuestionInput
	Source   InputSource
	Controls CallControls
}

func DecideRequest(question QuestionInput, source InputSource, controls CallControls) CompleteRequest {
	return CompleteRequest{FunctionDecide, question, source, controls}
}
func ChooseRequest(question QuestionInput, source InputSource, controls CallControls) CompleteRequest {
	return CompleteRequest{FunctionChoose, question, source, controls}
}
func TagRequest(question QuestionInput, source InputSource, controls CallControls) CompleteRequest {
	return CompleteRequest{FunctionTag, question, source, controls}
}
func ScoreRequest(question QuestionInput, source InputSource, controls CallControls) CompleteRequest {
	return CompleteRequest{FunctionScore, question, source, controls}
}
func FilterRequest(question QuestionInput, source InputSource, controls CallControls) CompleteRequest {
	return CompleteRequest{FunctionFilter, question, source, controls}
}
func RankRequest(question QuestionInput, source InputSource, controls CallControls) CompleteRequest {
	return CompleteRequest{FunctionRank, question, source, controls}
}
func FindRequest(question QuestionInput, source InputSource, controls CallControls) CompleteRequest {
	return CompleteRequest{FunctionFind, question, source, controls}
}
func AnnotateRequest(question QuestionInput, source InputSource, controls CallControls) CompleteRequest {
	return CompleteRequest{FunctionAnnotate, question, source, controls}
}
func RecognizeRequest(question QuestionInput, source InputSource, controls CallControls) CompleteRequest {
	return CompleteRequest{FunctionRecognize, question, source, controls}
}
func RelateRequest(question QuestionInput, source InputSource, controls CallControls) CompleteRequest {
	return CompleteRequest{FunctionRelate, question, source, controls}
}
