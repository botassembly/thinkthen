package consumer

import (
	"context"
	"encoding/json"
	"errors"
	"testing"

	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
)

// Feature: row errors follow Go nil comparisons and retain native error identity.
func TestRowErrorsFollowGoConventions(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	row, err := c.Decide(context.Background(), tt.NewDecide("Ready?"), tt.Text("evidence"))
	if err != nil {
		t.Fatal(err)
	}
	if row.Err() != nil {
		t.Fatalf("successful row returned a non-nil error: %v", row.Err())
	}
	failed, err := c.Decide(context.Background(), tt.NewDecide("Ready?"), tt.Missing())
	if err != nil {
		t.Fatal(err)
	}
	var native *tt.Error
	if !errors.Is(failed.Err(), tt.CodeRecordMissing) || !errors.As(failed.Err(), &native) || native.Error() == "" {
		t.Fatal("failed row lost its native error")
	}
	var absent *tt.Error
	if absent.Error() != "" {
		t.Fatal("nil error pointer has a message")
	}
}

// Feature: rows export contract JSON with exact IDs and status-specific fields.
func TestRowsMarshalContractJSON(t *testing.T) {
	c := apiClientFor(t, tt.Config{Backend: tt.Stub})
	for _, id := range []string{"", `"case"`, `-9223372036854775808`, `18446744073709551615`} {
		for _, status := range []tt.Status{tt.Answered, tt.Unsure, tt.Failed} {
			q := tt.NewDecide("Ready?")
			body := `{"body":"evidence"}`
			if id != "" {
				body = `{"id":` + id + `,"body":"evidence"}`
			}
			record := tt.RawJSON([]byte(body))
			if status == tt.Unsure {
				q = q.WithThreshold(tt.Band(0.01, 1))
			}
			if status == tt.Failed {
				q = q.WithEvidence("/absent")
			}
			row, err := c.Decide(context.Background(), q, record)
			if err != nil {
				t.Fatal(err)
			}
			if row.Status() != status {
				t.Fatalf("fixture status: %v", row.Status())
			}
			encoded, err := json.Marshal(row)
			if err != nil {
				t.Fatal(err)
			}
			var fields map[string]json.RawMessage
			if err = json.Unmarshal(encoded, &fields); err != nil {
				t.Fatal(err)
			}
			if string(fields["id"]) != id || string(fields["function"]) != `"decide"` || string(fields["position"]) != "1" || string(fields["status"]) != `"`+status.String()+`"` {
				t.Fatalf("row metadata lost in JSON: %s", encoded)
			}
			var source struct {
				QuestionIdentity string `json:"question_identity"`
				Backend, Model   string
			}
			if err = json.Unmarshal(fields["source"], &source); err != nil {
				t.Fatal(err)
			}
			if source.QuestionIdentity != row.Source().QuestionIdentity || source.Backend != "stub" || source.Model != row.Source().Model {
				t.Fatalf("source lost in JSON: %s", encoded)
			}
			if status == tt.Failed {
				var failure struct{ Code, Message string }
				if err = json.Unmarshal(fields["error"], &failure); err != nil {
					t.Fatal(err)
				}
				if failure.Code != "record.missing" || failure.Message != row.Err().Error() {
					t.Fatalf("failure lost in JSON: %s", encoded)
				}
				wantFields := 6
				if id == "" {
					wantFields--
				}
				if len(fields) != wantFields {
					t.Fatalf("failed row has extra fields: %s", encoded)
				}
			} else {
				var probability float64
				if err = json.Unmarshal(fields["probability"], &probability); err != nil {
					t.Fatal(err)
				}
				want, _ := row.Probability()
				if probability != want {
					t.Fatal("probability lost precision")
				}
				if status == tt.Answered {
					var answer bool
					if err = json.Unmarshal(fields["answer"], &answer); err != nil {
						t.Fatal(err)
					}
					want, _ := row.Value()
					if answer != want {
						t.Fatal("answer lost in JSON")
					}
				} else if _, present := fields["answer"]; present {
					t.Fatalf("unsure row has an answer: %s", encoded)
				}
				if _, present := fields["error"]; present {
					t.Fatalf("successful row has an error: %s", encoded)
				}
			}
		}
	}
}
