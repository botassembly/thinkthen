package thinkthen

import (
	"encoding/json"
	"testing"
)

// Unknown members are retained privately without exposing the transport packet.
func TestUnknownMemberRetention(t *testing.T) {
	var packet SessionPacket
	if err := json.Unmarshal([]byte(`{"kind":"terminal","extra":{"future":false},"failure":null}`), &packet); err != nil {
		t.Fatal(err)
	}
	value, err := packet.jsonValue()
	if err != nil {
		t.Fatal(err)
	}
	roundtrip, err := json.Marshal(value)
	if err != nil {
		t.Fatal(err)
	}
	if string(roundtrip) != `{"extra":{"future":false},"failure":null,"kind":"terminal"}` {
		t.Fatal("unknown field lost")
	}
}
