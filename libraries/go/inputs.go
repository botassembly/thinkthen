package thinkthen

import "encoding/base64"

// Ptr preserves explicit zero, false, empty collections and null authored values.
func Ptr[T any](value T) *T { return &value }

// TextQuestion treats its wording literally, including an initial @.
func TextQuestion(text string) RequestQuestion { return RequestQuestionText{Text: text} }

// DefinedQuestion carries a generated authored question or saved question set.
func DefinedQuestion(value RequestDefinition) RequestQuestion {
	return RequestQuestionDefinition{Value: value}
}

// TextItem keeps original text separate from per-item controls and attachments.
func TextItem(text string) RequestItem {
	return RequestItem{Original: Ptr[RequestOriginal](RequestOriginalText{Text: text})}
}

// JSONItem retains the original Go JSON value, including explicit null.
func JSONItem(value any) RequestItem {
	return RequestItem{Original: Ptr[RequestOriginal](RequestOriginalJson{Value: value})}
}

// BytesImage supplies an ordered attachment. Rust validates bytes and media.
func BytesImage(value []byte, media ImageMedia) RequestImage {
	return RequestImageBytes{Bytes: base64.StdEncoding.EncodeToString(value), Media: media}
}
