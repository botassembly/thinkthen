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

// FeedInput supplies ordered values with native backpressure.
type FeedInput struct{ Producer Producer }

func (FeedInput) isRequestInput() {}

// TextInput supplies one text record.
func TextInput(text string) RequestInputText { return RequestInputText{Text: text} }

// RecordsInput keeps each original and its controls in order.
func RecordsInput(items ...RequestItem) RequestInputRecords { return RequestInputRecords{Items: items} }

// TextRecordsInput supplies ordered text records.
func TextRecordsInput(texts ...string) RequestInputRecords {
	items := make([]RequestItem, len(texts))
	for i, text := range texts {
		items[i] = TextItem(text)
	}
	return RecordsInput(items...)
}
