package main

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"os"

	thinkthen "github.com/botassembly/thinkthen/libraries/go"
)

// With no argument, stdin is one JSON-door request and stdout its reply.
// "plan" reads one plan input and prints Engine.Plan's object. "fields"
// reads each annotate row member through ReadField and prints its state.
func main() {
	if len(os.Args) > 1 && os.Args[1] == "complete" {
		raw, err := io.ReadAll(os.Stdin)
		if err != nil {
			panic(err)
		}
		fmt.Println(completeCase(raw))
		return
	}

	e, err := thinkthen.New()
	if err != nil {
		panic(err)
	}
	defer e.Close()
	request, err := io.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}
	mode := ""
	if len(os.Args) > 1 {
		mode = os.Args[1]
	}
	var value string
	if mode == "plan" {
		var input struct {
			Verb     string          `json:"verb"`
			Question json.RawMessage `json:"question"`
			Input    []string        `json:"input"`
			Settings json.RawMessage `json:"settings"`
		}
		if err = json.Unmarshal(request, &input); err != nil {
			panic(err)
		}
		var question string // a question object passes as its own text
		if json.Unmarshal(input.Question, &question) != nil {
			question = string(input.Question)
		}
		var plan json.RawMessage
		plan, err = e.Plan(input.Verb, question, input.Input, input.Settings)
		value = string(plan)
	} else {
		value, err = e.Call(context.Background(), string(request))
	}
	if err != nil {
		var failure *thinkthen.Error
		if !errors.As(err, &failure) {
			panic(err)
		}
		row, err := json.Marshal(map[string]any{"failed": map[string]any{"kind": failure.Kind.String(), "code": failure.Code}})
		if err != nil {
			panic(err)
		}
		fmt.Println(string(row))
		return
	}
	if mode == "fields" {
		value = fields(value)
	}
	fmt.Println(value)
}

func fields(reply string) string {
	var call struct {
		Value []map[string]json.RawMessage `json:"value"`
	}
	if err := json.Unmarshal([]byte(reply), &call); err != nil {
		panic(err)
	}
	names := [...]string{"unresolved", "answered", "failed"}
	states := []map[string]string{}
	for _, row := range call.Value {
		state := map[string]string{}
		for name, member := range row {
			field, err := thinkthen.ReadField(member)
			if err != nil {
				panic(err)
			}
			state[name] = names[field.State]
			if field.State == thinkthen.Failed {
				state[name] += " " + field.Kind + " " + field.Cause
			}
		}
		states = append(states, state)
	}
	text, err := json.Marshal(states)
	if err != nil {
		panic(err)
	}
	return string(text)
}
