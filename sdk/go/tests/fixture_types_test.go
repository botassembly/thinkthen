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
	Expected      []apiExpected   `json:"-"`
	Requests      int             `json:"requests"`
	ExpectedData  json.RawMessage `json:"expected"`
	ExpectedRows  []apiExpected   `json:"expected_rows"`
	ExpectedError string          `json:"-"`
	Cancelled     bool            `json:"cancelled"`
}
type apiQuestion struct {
	Function                       string `json:"function"`
	Question                       string `json:"question"`
	Choices, Labels, Levels, Kinds []string
	Rules                          []tt.Rule              `json:"rules"`
	Questions                      map[string]apiQuestion `json:"questions"`
	Threshold                      json.RawMessage        `json:"threshold"`
	Evidence                       *string                `json:"evidence"`
	Top                            *int                   `json:"top"`
}
type apiExpected struct {
	Function       string          `json:"function"`
	Position       uint64          `json:"position"`
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
		v := tt.NewDecide(q.Question)
		if len(q.Threshold) > 0 {
			v = v.WithThreshold(thresholdFor(t, q.Threshold))
		}
		if q.Evidence != nil {
			v = v.WithEvidence(*q.Evidence)
		}
		return v
	case "choose":
		v := tt.NewChoose(q.Question, q.Choices)
		if len(q.Threshold) > 0 {
			v = v.WithThreshold(thresholdFor(t, q.Threshold))
		}
		if q.Evidence != nil {
			v = v.WithEvidence(*q.Evidence)
		}
		return v
	case "filter":
		v := tt.NewFilter(q.Question)
		if len(q.Threshold) > 0 {
			v = v.WithThreshold(thresholdFor(t, q.Threshold))
		}
		if q.Evidence != nil {
			v = v.WithEvidence(*q.Evidence)
		}
		return v
	case "tag":
		v := tt.NewTag(q.Question, q.Labels)
		if len(q.Threshold) > 0 {
			v = v.WithThreshold(cutFor(t, q.Threshold))
		}
		if q.Evidence != nil {
			v = v.WithEvidence(*q.Evidence)
		}
		return v
	case "score":
		v := tt.NewScore(q.Question, q.Levels)
		if q.Evidence != nil {
			v = v.WithEvidence(*q.Evidence)
		}
		return v
	case "rank":
		v := tt.NewRank(q.Question, q.Levels)
		if q.Top != nil {
			v = v.WithTop(*q.Top)
		}
		if q.Evidence != nil {
			v = v.WithEvidence(*q.Evidence)
		}
		return v
	case "find":
		v := tt.NewFind(q.Question)
		if len(q.Threshold) > 0 {
			v = v.WithThreshold(cutFor(t, q.Threshold))
		}
		if q.Evidence != nil {
			v = v.WithEvidence(*q.Evidence)
		}
		return v
	case "recognize":
		v := tt.NewRecognize(q.Kinds)
		if len(q.Threshold) > 0 {
			v = v.WithThreshold(cutFor(t, q.Threshold))
		}
		if q.Evidence != nil {
			v = v.WithEvidence(*q.Evidence)
		}
		return v
	case "relate":
		v := tt.NewRelate(q.Rules)
		if len(q.Threshold) > 0 {
			v = v.WithThreshold(cutFor(t, q.Threshold))
		}
		if q.Evidence != nil {
			v = v.WithEvidence(*q.Evidence)
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
