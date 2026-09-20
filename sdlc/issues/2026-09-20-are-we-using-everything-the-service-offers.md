# Are we using everything the service offers?

Status: Open

Ian asked on 2026-09-20 for a final check that the tool uses every input and every output of the hosted service. An agent read the vendor documents snapshot and `sdlc/planning/interface-audit.md`. Two of its answers were then checked against the binary with `--dry-run`, which sends nothing.

## The answer

The tool uses nearly all of it. One large capability is unused today, and the plan already aims at it.

## Used, checked by dry run or in the code

- The evidence, the model name, and all three question types.
- What yes and no mean. `--true` and `--false` land in `criteria.true` and `criteria.false`.
- A description for each option. `--option LABEL=DESCRIPTION` lands as the value under the label.
- The ordered levels of `score`.
- Every probability, the vendor's `confidence` on a pick and a score, the model name, and the token usage. All of it shows under `--details`.
- `Retry-After` and `Retry-After-Ms`. `crates/thinkthen/src/http.rs` reads them, the finer one first.

## Unused

1. **Several questions in one request.** Every request today carries exactly one question, named `q1`. The vendor measured thirteen questions in one call at 12.2 times cheaper and 10.0 times faster than thirteen calls, with the same answers. This is the largest gain the service offers and the tool leaves it on the table. `specification/annotate.md` plans to use it. The proposed `tag` question type rides on it. The cap on questions per request is still unmeasured.
2. **The models listing.** `GET /v1/models` returns a name, a description, and a release date. The roadmap holds it. The status issue filed the same day wants it for a `status` view.
3. **Evidence as a structured object.** The vendor accepts a string, an object, or a list as `state`. The tool always sends a string. A dry run with two `--field` pointers sent the evidence object written out as JSON text. Nobody has measured whether the model reads an object better than the same object written as text. One probe would answer it.
4. **The vendor's reason on a refused request.** A validation refusal carries a list of reasons. The tool prints one fixed sentence per status. This looks deliberate, because a reason can quote the evidence back. It stays as it is unless a stumble shows up in the register.

## The service offers none of these

Images or files as evidence, examples inside a request, streaming, bulk or background jobs, a request id, and an account usage or quota endpoint. The documents show none. The status issue already says usage has to come from a local ledger.

## The audit page has gone stale

`sdlc/planning/interface-audit.md` still says that the yes and no meanings and the option descriptions never reach the wire, and that `Retry-After` is ignored. The binary sends both, and the code reads the headers. The page needs a pass, or a line at its top that says which rows have landed. A stale audit cost this check a second round.
