package consumer

import (
	"bufio"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	thinkthen "github.com/botassembly/thinkthen/sdk/go"
	"io"
	"os"
	"os/exec"
	"path/filepath"
	"reflect"
	"testing"
)

type request struct {
	Function string `json:"function"`
	Question struct {
		Type  string `json:"type"`
		Value any    `json:"value"`
		Name  string `json:"name"`
	} `json:"question"`
	Options thinkthen.Options `json:"options"`
	Records []any             `json:"records"`
}
type construction struct {
	Client       thinkthen.Settings `json:"client"`
	Request      request            `json:"request"`
	Expected     any                `json:"expected"`
	ExpectedRows []any              `json:"expected_rows"`
	Requests     int                `json:"requests"`
	Cancelled    bool               `json:"cancelled"`
}
type caseRows struct {
	*thinkthen.Rows
	answer func() (any, error)
}

type namedCall func(context.Context, thinkthen.Question, thinkthen.Input, thinkthen.Options) (*caseRows, error)

func typedCall[A any](call func(context.Context, thinkthen.Question, thinkthen.Input, thinkthen.Options) (*thinkthen.Results[A], error)) namedCall {
	return func(ctx context.Context, question thinkthen.Question, input thinkthen.Input, options thinkthen.Options) (*caseRows, error) {
		rows, err := call(ctx, question, input, options)
		if err != nil {
			return nil, err
		}
		return &caseRows{Rows: rows.Rows, answer: func() (any, error) { return rows.Row().Answer() }}, nil
	}
}

func invoke(client *thinkthen.Client, ctx context.Context, request request, input thinkthen.Input) (*caseRows, error) {
	calls := map[string]namedCall{"decide": typedCall(client.Decide), "choose": typedCall(client.Choose), "tag": typedCall(client.Tag), "score": typedCall(client.Score), "filter": typedCall(client.Filter), "rank": typedCall(client.Rank), "find": typedCall(client.Find), "annotate": typedCall(client.Annotate), "recognize": typedCall(client.Recognize), "relate": typedCall(client.Relate)}
	question := thinkthen.Inline(request.Question.Value)
	switch request.Question.Type {
	case "toolbox":
		question = thinkthen.Toolbox(request.Question.Value.(map[string]any), request.Question.Name)
	case "reference":
		question = thinkthen.Reference(request.Question.Name)
	}
	return calls[request.Function](ctx, question, input, request.Options)
}
func rowJSON(t *testing.T, row thinkthen.Row) map[string]any {
	t.Helper()
	value := map[string]any{"status": row.Status()}
	if id := row.ID(); id != nil {
		var decoded any
		if err := json.Unmarshal(id, &decoded); err != nil {
			t.Fatal(err)
		}
		value["id"] = decoded
	}
	if failure := row.Error(); failure != nil {
		if failure.Error() == "" {
			t.Fatal("failed row lost its native message")
		}
		value["error"] = map[string]any{"code": failure.Code()}
	} else {
		var answer any
		if err := row.DecodeAnswer(&answer); err != nil {
			t.Fatal(err)
		}
		value["answer"] = answer
	}
	if p, present := row.Probability(); present {
		value["p"] = p
	}
	if detail := row.Detail(); detail != nil {
		var decoded any
		if err := json.Unmarshal(detail, &decoded); err != nil {
			t.Fatal(err)
		}
		value["detail"] = decoded
	}
	stripMessages(value["answer"])
	return value
}
func stripMessages(answer any) {
	if entries, ok := answer.(map[string]any); ok {
		for _, value := range entries {
			if entry, ok := value.(map[string]any); ok {
				if failure, ok := entry["error"].(map[string]any); ok {
					delete(failure, "message")
				}
			}
		}
	}
}

// Shared fixtures use code points; this Go surface promises byte offsets.
func byteOffsets(value any, text string) {
	switch value := value.(type) {
	case map[string]any:
		start, hasStart := value["start"].(float64)
		end, hasEnd := value["end"].(float64)
		if hasStart && hasEnd {
			points := []rune(text)
			value["start"] = float64(len(string(points[:int(start)])))
			value["end"] = float64(len(string(points[:int(end)])))
		}
		for _, child := range value {
			byteOffsets(child, text)
		}
	case []any:
		for _, child := range value {
			byteOffsets(child, text)
		}
	}
}
func code(t *testing.T, err error) string {
	t.Helper()
	var native *thinkthen.Error
	if !errors.As(err, &native) {
		t.Fatalf("expected coded native failure, got %v", err)
	}
	if native.Error() == "" {
		t.Fatal("native error lost its message")
	}
	return native.Code()
}
func runConstruction(t *testing.T, c construction) any {
	t.Helper()
	result := []any{}
	defer func() {
		if c.ExpectedRows != nil && !reflect.DeepEqual(result, c.ExpectedRows) {
			t.Fatalf("rows before terminal error %#v; expected %#v", result, c.ExpectedRows)
		}
	}()
	client, err := thinkthen.NewClient(c.Client)
	if err != nil {
		return map[string]any{"error": code(t, err)}
	}
	defer client.Close()
	records := make([]thinkthen.Record, 0, len(c.Request.Records))
	for _, value := range c.Request.Records {
		if text, ok := value.(string); ok {
			records = append(records, thinkthen.TextRecord(text))
		} else {
			records = append(records, thinkthen.JSONRecord(value))
		}
	}
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	if c.Cancelled {
		cancel()
	}
	rows, err := invoke(client, ctx, c.Request, thinkthen.Records(records...))
	if err != nil {
		return map[string]any{"error": code(t, err)}
	}
	defer rows.Close()
	for rows.Next() {
		row := rows.Row()
		if row.Position() < 1 {
			t.Fatal("missing original position")
		}
		if c.Request.Function == "filter" {
			expectedKept, _ := c.Expected.([]any)[len(result)].(map[string]any)["answer"].(bool)
			if row.Kept() != expectedKept {
				t.Fatal("filter selection metadata changed")
			}
		}
		value := rowJSON(t, row)
		if row.Status() == "failed" {
			_, err := rows.answer()
			if code(t, err) != row.Error().Code() || err.Error() != row.Error().Error() {
				t.Fatal("typed answer lost its engine error")
			}
		}
		if row.Status() == "answered" {
			answer, err := rows.answer()
			if err != nil {
				t.Fatal(err)
			}
			encoded, err := json.Marshal(answer)
			if err != nil {
				t.Fatal(err)
			}
			var decoded any
			if err = json.Unmarshal(encoded, &decoded); err != nil {
				t.Fatal(err)
			}
			value["answer"] = decoded
			stripMessages(value["answer"])
		}
		result = append(result, value)
	}
	if err = rows.Err(); err != nil {
		if c.Cancelled {
			if code(t, err) != "local.cancelled" || !errors.Is(err, context.Canceled) {
				t.Fatalf("wrong cancellation: %v", err)
			}
		}
		return map[string]any{"error": code(t, err)}
	}
	if rows.Next() {
		t.Fatal("terminal iterator restarted")
	}
	return result
}
func TestSharedStub(t *testing.T) {
	data, err := os.ReadFile(os.Getenv("THINKTHEN_STUB_CASES"))
	if err != nil {
		t.Fatal(err)
	}
	var cases []construction
	if err = json.Unmarshal(data, &cases); err != nil {
		t.Fatal(err)
	}
	if len(cases) != 10 {
		t.Fatal("construction file must exercise each named function")
	}
	for _, c := range cases {
		t.Run(c.Request.Function, func(t *testing.T) {
			actual := runConstruction(t, c)
			if !reflect.DeepEqual(actual, c.Expected) {
				t.Fatalf("actual %#v; expected %#v", actual, c.Expected)
			}
		})
	}
}
func TestSharedConformance(t *testing.T) {
	paths, err := filepath.Glob(filepath.Join(os.Getenv("THINKTHEN_CASES"), "*.json"))
	if err != nil {
		t.Fatal(err)
	}
	if len(paths) == 0 {
		t.Fatal("no conformance cases")
	}
	for _, path := range paths {
		t.Run(filepath.Base(path), func(t *testing.T) {
			data, err := os.ReadFile(path)
			if err != nil {
				t.Fatal(err)
			}
			var c construction
			if err = json.Unmarshal(data, &c); err != nil {
				t.Fatal(err)
			}
			backend := startBackend(t, path)
			c.Client.URL = backend.url

			if expected, ok := c.Expected.([]any); ok && (c.Request.Function == "find" || c.Request.Function == "recognize" || c.Request.Function == "relate") {
				for i, expected := range expected {
					if text, ok := c.Request.Records[i].(string); ok {
						byteOffsets(expected.(map[string]any)["answer"], text)
						byteOffsets(expected.(map[string]any)["detail"], text)
					}
				}
			}
			actual := runConstruction(t, c)
			if !reflect.DeepEqual(actual, c.Expected) {
				t.Fatalf("actual %#v; expected %#v", actual, c.Expected)
			}
			reply := backend.command(t, "count")
			var count int
			if _, err = fmt.Sscan(reply, &count); err != nil {
				t.Fatal(err)
			}
			if count != c.Requests {
				t.Fatalf("requests %d; expected %d", count, c.Requests)
			}
		})
	}
}

type backend struct {
	child  *exec.Cmd
	input  io.WriteCloser
	output *bufio.Scanner
	url    string
}

func startBackend(t *testing.T, args ...string) *backend {
	t.Helper()
	child := exec.Command(os.Getenv("THINKTHEN_LOOPBACK"), args...)
	child.Env = []string{"PATH=" + os.Getenv("PATH")}
	input, err := child.StdinPipe()
	if err != nil {
		t.Fatal(err)
	}
	output, err := child.StdoutPipe()
	if err != nil {
		t.Fatal(err)
	}
	child.Stderr = os.Stderr
	if err = child.Start(); err != nil {
		t.Fatal(err)
	}
	b := &backend{child: child, input: input, output: bufio.NewScanner(output)}
	t.Cleanup(func() {
		fmt.Fprintln(input, "shutdown")
		input.Close()
		if err := child.Wait(); err != nil {
			t.Error(err)
		}
	})
	if !b.output.Scan() {
		t.Fatal("loopback did not publish its port")
	}
	b.url = "http://127.0.0.1:" + b.output.Text() + "/v1"
	return b
}
func (b *backend) command(t *testing.T, command string) string {
	t.Helper()
	if _, err := fmt.Fprintln(b.input, command); err != nil {
		t.Fatal(err)
	}
	if !b.output.Scan() {
		t.Fatal("loopback did not answer " + command)
	}
	return b.output.Text()
}
