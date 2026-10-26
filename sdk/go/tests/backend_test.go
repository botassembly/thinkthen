package consumer

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"os/exec"
	"testing"
)

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
