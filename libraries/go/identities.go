package thinkthen

import (
	"encoding/json"
	"fmt"
)

// Each public wrapper is a distinct type. Validation and JSON ownership are shared.
type identifier struct{ value string }

func parseIdentifier(value string) (identifier, error) {
	if len(value) != 64 {
		return identifier{}, fmt.Errorf("identity must be 64 lowercase hexadecimal characters")
	}
	for _, c := range value {
		if !(c >= '0' && c <= '9' || c >= 'a' && c <= 'f') {
			return identifier{}, fmt.Errorf("identity must be 64 lowercase hexadecimal characters")
		}
	}
	return identifier{value}, nil
}
func (id identifier) String() string { return id.value }
func (id identifier) MarshalJSON() ([]byte, error) {
	if _, err := parseIdentifier(id.value); err != nil {
		return nil, err
	}
	return json.Marshal(id.value)
}
func (id *identifier) UnmarshalJSON(data []byte) error {
	var value string
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}
	parsed, err := parseIdentifier(value)
	if err == nil {
		*id = parsed
	}
	return err
}

type CallId struct{ identifier }

func NewCallId(value string) (CallId, error) {
	id, err := parseIdentifier(value)
	return CallId{id}, err
}

type SdkRequestId struct{ identifier }

func NewSdkRequestId(value string) (SdkRequestId, error) {
	id, err := parseIdentifier(value)
	return SdkRequestId{id}, err
}

type ObservationId struct{ identifier }

func NewObservationId(value string) (ObservationId, error) {
	id, err := parseIdentifier(value)
	return ObservationId{id}, err
}

type FailureId struct{ identifier }

func NewFailureId(value string) (FailureId, error) {
	id, err := parseIdentifier(value)
	return FailureId{id}, err
}

type AnswerId struct{ identifier }

func NewAnswerId(value string) (AnswerId, error) {
	id, err := parseIdentifier(value)
	return AnswerId{id}, err
}

type Digest struct{ identifier }

func NewDigest(value string) (Digest, error) {
	id, err := parseIdentifier(value)
	return Digest{id}, err
}
