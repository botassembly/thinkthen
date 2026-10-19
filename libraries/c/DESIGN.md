# C interface

The generated [header](include/thinkthen.h) defines the current C interface. The [library guide](README.md) describes its installed use and links the complete session example.

Call input uses counted canonical JSON under the [C input decision](../../sdlc/decisions/2026-10-19-c-input-json.md). Transitional input functions remain for phase 3 callers.

Engine constructors use the shared native settings grammar. Constructors send nothing and record failures in the calling thread’s null-engine slot. A successful constructor clears that slot. Engine error accessors borrow diagnostics until the next failure on that engine or thread exit; engine destruction ends their lifetime.

Sessions own accepted inputs and retain their engine independently of the caller’s engine handle. Sessions deliver owned result packets. Every nested typed view borrows its packet owner; free the packet after its last field read. Session cancellation stops intake and does not block on a held provider. Free a session after all calls and borrowers finish.

Immediate session refusals use a separate calling-thread diagnostic slot. Successful session calls preserve its diagnostic and facts. Execution failures and final facts belong to terminal packets.

Parsed question owners retain authored metadata independently of caller buffers and engines. Their views borrow the question owner.

Canonical Request preview supports fixed atomic questions through native admission. It reads no key or cache and sends nothing. Successful preview JSON is owned and released with `thinkthen_free_string`. Refusals leave output slots unchanged.

Every exported entry catches panics before returning to the host. The header specifies pointer extents, nullability, output ownership and safe refusal rules beside each declaration.
