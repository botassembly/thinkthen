package consumer

import (
	"context"
	tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"
	"testing"
)

func TestUsageCopiesCountersAndSurvivesClose(t *testing.T) {
	client, err := tt.New(tt.Config{Backend: tt.Stub})
	if err != nil {
		t.Fatal(err)
	}
	defer client.Close()
	before, err := client.Usage()
	if err != nil {
		t.Fatal(err)
	}
	if before.Attempts != 0 || before.InputTokens.Reported == nil || *before.InputTokens.Reported != 0 {
		t.Fatalf("wrong initial usage: %+v", before)
	}
	row, err := client.Decide(context.Background(), tt.NewDecide("Ready?"), tt.Text("yes"))
	if err != nil || row.Status() != tt.Answered {
		t.Fatalf("stub row: %v %v", row, err)
	}
	if err := client.Close(); err != nil {
		t.Fatal(err)
	}
	after, err := client.Usage()
	if err != nil {
		t.Fatal(err)
	}
	if after.CacheHits != 0 || after.ReplayHits != 0 || after.Attempts != 0 || after.OutputTokens.Reported == nil || *after.OutputTokens.Reported != 0 {
		t.Fatalf("wrong closed usage: %+v", after)
	}
	*after.OutputTokens.Reported = 999
	again, err := client.Usage()
	if err != nil || *again.OutputTokens.Reported != 0 {
		t.Fatalf("snapshot mutation affected client: %+v %v", again, err)
	}
	if err := client.Close(); err != nil {
		t.Fatal(err)
	}
}

// Feature: an explicit pointer-to-zero reaches native settings instead of defaulting away.
func TestZeroRequestAllowanceProjection(t *testing.T) {
	server := startBackend(t, "--held", "headers")
	var zero uint64
	client, err := tt.New(tt.Config{Backend: tt.SystemOne, URL: server.url, APIKey: tt.APIKey("synthetic"), MaxRequests: &zero})
	if err != nil {
		t.Fatal(err)
	}
	defer client.Close()
	_, err = client.Decide(context.Background(), tt.NewDecide("Ready?"), tt.Text("evidence"))
	if err == nil {
		t.Fatal("request allowance disappeared")
	}
	usage, usageErr := client.Usage()
	if usageErr != nil || usage.Attempts != 0 {
		t.Fatalf("zero allowance sent work: %+v %v", usage, usageErr)
	}
}

func TestResponseStoreProjectsExplicitZero(t *testing.T) {
	var zero uint64
	_, err := tt.New(tt.Config{Backend: tt.OpenAI, ResponseStore: &tt.ResponseStoreConfig{
		Path: t.TempDir(), Mode: tt.ResponseCache, Namespace: "sdk-test", ModelGeneration: "generation", MaxEntries: &zero,
	}})
	if err == nil {
		t.Fatal("response-store zero limit was ignored")
	}
}
