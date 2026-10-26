package consumer

import (
	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

// Feature: annotation requests exclude collection rank, nested annotation, and general question maps.
func TestAnnotationTypesRejectIneligibleQuestions(t *testing.T) {
	module, err := os.ReadFile("go.mod")
	if err != nil {
		t.Fatal(err)
	}
	for name, expression := range map[string]string{
		"rank":        `var _ tt.AnnotationQuestion=tt.NewRank("How?",[]string{"low","high"})`,
		"nested":      `var _ tt.AnnotationQuestion=tt.NewAnnotate(nil)`,
		"general-map": `var _=tt.NewAnnotate(map[string]tt.Question{"ready":tt.NewDecide("Ready?")})`,
	} {
		t.Run(name, func(t *testing.T) {
			dir := t.TempDir()
			if err := os.WriteFile(filepath.Join(dir, "go.mod"), module, 0600); err != nil {
				t.Fatal(err)
			}
			source := "package consumer\nimport tt \"github.com/botassembly/thinkthen/sdk/go/thinkthen\"\n" + expression + "\n"
			if err := os.WriteFile(filepath.Join(dir, "consumer.go"), []byte(source), 0600); err != nil {
				t.Fatal(err)
			}
			command := exec.Command("go", "test", "-run=^$", ".")
			command.Dir = dir
			output, err := command.CombinedOutput()
			if err == nil || !(strings.Contains(string(output), "does not implement") || strings.Contains(string(output), "cannot use")) {
				t.Fatalf("consumer did not reject the ineligible annotation: %v\n%s", err, output)
			}
		})
	}
}

// A collection rank remains a concrete selectable toolbox question.
var (
	_ tt.Question                  = tt.NewRank("How?", []string{"low", "high"})
	_ func(string) (tt.Rank, bool) = tt.Toolbox{}.Rank
)
