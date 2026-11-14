package consumer

import (
	"encoding/json"
	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
	"testing"
	"time"
)

// apiCase consumes the regenerated shared fixture grammar. Removed request
// envelopes are rejected, so stale fixtures cannot silently claim parity.
type apiCase struct {
	NotApplicable map[string]string `json:"not_applicable"`
	Client        struct {
		Backend string    `json:"backend"`
		Model   string    `json:"model"`
		Batch   *int      `json:"batch"`
		Retry   *apiRetry `json:"retry"`
	} `json:"client"`
	Request struct {
		Question apiQuestion       `json:"question"`
		Records  []json.RawMessage `json:"records"`
		Name     string            `json:"name"`
	} `json:"request"`
	Expected           []apiExpected   `json:"-"`
	Requests           int             `json:"requests"`
	ExpectedData       json.RawMessage `json:"expected"`
	ExpectedRows       []apiExpected   `json:"expected_rows"`
	ExpectedError      string          `json:"-"`
	ExpectedHTTPStatus int             `json:"-"`
	Cancelled          bool            `json:"cancelled"`
}
type apiQuestion struct {
	Function  string                 `json:"function"`
	Question  string                 `json:"question,omitempty"`
	Choices   []string               `json:"choices,omitempty"`
	Labels    []string               `json:"labels,omitempty"`
	Levels    []string               `json:"levels,omitempty"`
	Kinds     []string               `json:"kinds,omitempty"`
	Rules     []tt.RelationRule      `json:"rules,omitempty"`
	Questions map[string]apiQuestion `json:"questions,omitempty"`
	Threshold json.RawMessage        `json:"threshold,omitempty"`
	Field     *string                `json:"field,omitempty"`
	Top       *int                   `json:"top,omitempty"`
}

type apiExpected struct {
	Function       string          `json:"function"`
	Position       int             `json:"position"`
	ID             json.RawMessage `json:"id"`
	Status         string          `json:"status"`
	Answer, Detail json.RawMessage
	Probability    *float64 `json:"probability"`
	Source         struct {
		QuestionIdentity string  `json:"question_identity"`
		Name             *string `json:"name"`
		Backend          string  `json:"backend"`
		Model            string  `json:"model"`
	} `json:"source"`
	Error *struct {
		Code, Message string
		HTTPStatus    *int `json:"http_status"`
	} `json:"error"`
}

func thresholdFor(t *testing.T, raw json.RawMessage) tt.Threshold {
	t.Helper()
	if len(raw) == 0 {
		return nil
	}
	var cut float64
	if json.Unmarshal(raw, &cut) == nil {
		return tt.Cutoff(cut)
	}
	var band struct{ Low, High float64 }
	if err := json.Unmarshal(raw, &band); err != nil {
		t.Fatal(err)
	}
	return tt.Band(band.Low, band.High)
}
func cutFor(t *testing.T, raw json.RawMessage) tt.Cut {
	t.Helper()
	var n float64
	if err := json.Unmarshal(raw, &n); err != nil {
		t.Fatal("numeric-only fixture supplied a band")
	}
	return tt.Cutoff(n)
}
func questionFor(t *testing.T, q apiQuestion) tt.Question {
	t.Helper()
	switch q.Function {
	case "decide":
		v := tt.DecideQuestion{Question: q.Question}
		if len(q.Threshold) > 0 {
			v.Threshold = thresholdFor(t, q.Threshold)
		}
		if q.Field != nil {
			v.Field = *q.Field
		}
		return v
	case "choose":
		v := tt.ChooseQuestion[string]{Question: q.Question, Choices: q.Choices}
		if len(q.Threshold) > 0 {
			v.Threshold = cutFor(t, q.Threshold)
		}
		if q.Field != nil {
			v.Field = *q.Field
		}
		return v
	case "filter":
		v := tt.FilterQuestion{Question: q.Question}
		if len(q.Threshold) > 0 {
			v.Threshold = cutFor(t, q.Threshold)
		}
		if q.Field != nil {
			v.Field = *q.Field
		}
		return v
	case "tag":
		v := tt.TagQuestion[string]{Question: q.Question, Labels: q.Labels}
		if len(q.Threshold) > 0 {
			v.Threshold = cutFor(t, q.Threshold)
		}
		if q.Field != nil {
			v.Field = *q.Field
		}
		return v
	case "score":
		v := tt.ScoreQuestion[string]{Question: q.Question, Levels: q.Levels}
		if q.Field != nil {
			v.Field = *q.Field
		}
		return v
	case "rank":
		v := tt.RankQuestion[string]{Question: q.Question, Levels: q.Levels}
		if q.Top != nil {
			v.Top = *q.Top
		}
		if q.Field != nil {
			v.Field = *q.Field
		}
		return v
	case "find":
		v := tt.FindQuestion{Question: q.Question}
		if len(q.Threshold) > 0 {
			v.Threshold = cutFor(t, q.Threshold)
		}
		if q.Field != nil {
			v.Field = *q.Field
		}
		return v
	case "recognize":
		v := tt.RecognizeQuestion{Kinds: q.Kinds}
		if len(q.Threshold) > 0 {
			v.Threshold = cutFor(t, q.Threshold)
		}
		if q.Field != nil {
			v.Field = *q.Field
		}
		return v
	case "relate":
		v := tt.RelateQuestion{Rules: q.Rules}
		if len(q.Threshold) > 0 {
			v.Threshold = cutFor(t, q.Threshold)
		}
		if q.Field != nil {
			v.Field = *q.Field
		}
		return v
	default:
		t.Fatalf("unsupported fixture question %q", q.Function)
		return nil
	}
}

type apiRetry struct {
	MaxRetries     int    `json:"max_retries"`
	InitialBackoff *int64 `json:"initial_backoff_ms"`
	AttemptTimeout *int64 `json:"attempt_timeout_ms"`
}

func retryFor(r *apiRetry) *tt.Retry {
	if r == nil {
		return nil
	}
	backoff, timeout := int64(1), int64(10000)
	if r.InitialBackoff != nil {
		backoff = *r.InitialBackoff
	}
	if r.AttemptTimeout != nil {
		timeout = *r.AttemptTimeout
	}
	return &tt.Retry{MaxRetries: r.MaxRetries, InitialBackoff: time.Duration(backoff) * time.Millisecond, AttemptTimeout: time.Duration(timeout) * time.Millisecond}
}
