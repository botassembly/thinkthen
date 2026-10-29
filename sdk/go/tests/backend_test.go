package consumer

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"os/exec"
	"testing"
	"time"
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
		done := make(chan error, 1)
		go func() { done <- child.Wait() }()
		if err := receiveBefore(t, done, time.Now().Add(30*time.Second)); err != nil {
			t.Error(err)
		}
	})
	b.url = "http://127.0.0.1:" + b.lineBefore(t, time.Now().Add(30*time.Second)) + "/v1"
	return b
}
func receiveBefore[T any](t *testing.T, result <-chan T, deadline time.Time) T {
	t.Helper()
	timer := time.NewTimer(time.Until(deadline))
	defer timer.Stop()
	select {
	case value := <-result:
		return value
	case <-timer.C:
		t.Fatal("blocking test operation exceeded its deadline")
	}
	var zero T
	return zero
}
func (b *backend) lineBefore(t *testing.T, deadline time.Time) string {
	t.Helper()
	done := make(chan string, 1)
	go func() {
		if b.output.Scan() {
			done <- b.output.Text()
		} else {
			done <- ""
		}
	}()
	timer := time.NewTimer(time.Until(deadline))
	defer timer.Stop()
	select {
	case value := <-done:
		if value == "" {
			t.Fatal("loopback ended before its response")
		}
		return value
	case <-timer.C:
		_ = b.child.Process.Kill()
		t.Fatal("loopback response exceeded its deadline")
	}
	return ""
}
func (b *backend) commandBefore(t *testing.T, command string, deadline time.Time) string {
	t.Helper()
	if _, err := fmt.Fprintln(b.input, command); err != nil {
		t.Fatal(err)
	}
	return b.lineBefore(t, deadline)
}
func (b *backend) command(t *testing.T, command string) string {
	return b.commandBefore(t, command, time.Now().Add(30*time.Second))
}
