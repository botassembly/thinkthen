---
flow: build
priority: 34
opens: specification/score.md sdlc/planning/handoff-to-the-architect-2026-09-21.md demos/13-pick-a-threshold/README.md site/src/pages/trust.astro
---

# 0087: Correct score, option-limit, and threshold documentation

Status: accepted

## Outcome and authority

Four documents tell the measured truth about score agreement, option limits, and threshold tuning. The threshold how-to covers all ten functions, including recognized names and relation edges, without changing a command, result, transform, or product limit. Ian explicitly authorized ticket 0087 and this exact bounded documentation scope in this session on 2026-09-23. The three issues supply evidence rather than authority.

## Current facts

- `specification/score.md` says the tool's weighted score and the vendor score agree. Experiment 235 found differences on 13 of 20 songs, up to 0.02. The tool correctly computes its number from the returned probabilities so the detailed row remains internally consistent; the vendor may use unrounded or differently normalized values.
- `sdlc/planning/handoff-to-the-architect-2026-09-21.md` says a question holds at most 100 options. Experiment 236 received answers with 101 and 255 options. The failed experiment-225 request was 165,154 bytes; exceeding the vendor's approximate request-size budget is the likely cause, not a proved diagnosis. The product's separate 255-option ceiling remains unchanged.
- `demos/13-pick-a-threshold/README.md` teaches a replay-only `jq` sweep for `decide`, `choose`, and `score`. `transforms/sweep/sweep.jq` also handles `tag` and mapped `annotate` answers, but neither page explains the signal to measure for all ten functions. `site/src/pages/trust.astro` gives only a four-step `decide` sketch and does not link to the executable how-to.
- A saved output can support a new cut only when it retains every candidate needed at that cut. `filter`, `recognize`, and `relate` omit rejected records, names, or edges. A lower cut cannot reconstruct them. Those functions require complete labeled candidate rows collected before filtering, or one honest rerun per tested cut from a committed recording. This ticket makes no live or paid call.
- The existing sweep accepts scalar `yes_no`, `choice`, and `score` rows, standalone `tag` rows with `--arg truth`, and mapped `annotate` answers of kind `yes_no`, `choice`, or `tag` with `--argjson truth`. It refuses a mapped annotate score. A named annotate score must first be projected into ordinary score rows, preserving its question, value, levels, input id, and trusted numeric label.

## Scope

Edit exactly the four files in `opens`.

1. Replace the false agreement sentence in `specification/score.md`. State that experiment 235 observed a difference up to 0.02, that the vendor field may use unrounded or differently normalized probabilities, and that callers trust the tool's computed value because it matches the probabilities in the same row. Make no stronger claim about the vendor's formula.
2. Replace the handoff's 100-option claim with the measured 101- and 255-option successes and the 165,154-byte failure. Call the size budget the likely cause. Keep the product's 255 ceiling distinct from the backend request-size budget.
3. Extend how-to 13 with the exact ten-row mapping below and a replay-only `jq` recipe over labeled rows. Existing `sweep.jq` remains the implementation for the shapes it supports. Compact inline `jq` may project a named annotate score or score explicit per-cut output, but it must not claim that an omitted candidate can be recovered.

| Function | Rows required | Signal and boundary |
| --- | --- | --- |
| `decide` | Complete detailed rows | `answer.probability`; existing cuts 0.05 through 0.95 |
| `choose` | Complete detailed rows | winning `answer.probabilities[answer.pick]`; existing cuts 0.05 through 0.95, with ties unresolved |
| `tag` | Complete detailed rows plus one human-label pointer | each label's probability independently; existing cuts 0.05 through 0.95 |
| `score` | Complete detailed rows with trusted numeric levels | `value`; integer boundaries 1 through K minus 1 |
| `filter` | Full labeled input joined to one recording-backed output per tested cut | output membership at each explicit tested cut; never infer records omitted at a lower cut |
| `rank` | Complete detailed ranked rows | `answer.probability`; cuts 0.05 through 0.95 as a downstream review policy, not a command threshold |
| `find` | One complete detailed result and trusted winner per labeled set | winning option probability; cuts 0.05 through 0.95 as a downstream coverage policy, not a command threshold |
| `annotate` | Complete detailed rows and mapped truth | existing mapped decide, choose, and tag support; project a named score to ordinary score rows before integer boundaries 1 through K minus 1 |
| `recognize` | Complete labeled name candidates, or one recording-backed rerun per tested cut | entity `strength`; explicit tested cuts only, never reconstruct omitted names |
| `relate` | Complete labeled candidate edges, or one recording-backed rerun per tested cut | edge `probability`; explicit tested cuts only, never reconstruct omitted edges |

4. Extend the trust page's existing sketch with a concise summary of the executable threshold how-to. Do not link to the private repository or invent an internal site route. Say that complete saved details can test another cut without a model request, while filtered outputs need complete candidate collection or recording-backed per-cut reruns.

Excluded: changing `transforms/sweep/sweep.jq` or any other transform; adding fixtures, recordings, commands, options, or result fields; implementing `recognize`, `relate`, request splitting, `--compare-threshold`, or automatic question rewriting; reconstructing lower-cut candidates from filtered output; changing the 255 product ceiling, backend size policy, score arithmetic, defaults, or any runtime behavior; editing the source issues or making a live or paid call.

## Acceptance

- The score page no longer says the two scores agree. It names experiment 235, 13 of 20 differing rows, the observed maximum difference of 0.02, and the tool-computed value as the internally consistent value to use.
- The handoff no longer states or implies a 100-option cap. It records successful 101- and 255-option probes, the 165,154-byte failed request, the approximate request-size limit as the likely cause, and the unchanged 255 product ceiling as separate facts.
- How-to 13 contains exactly the ten mapping rows and boundaries specified in Scope. Fixed-string checks pin each function, required row source, signal, boundary, and the three no-reconstruction statements for `filter`, `recognize`, and `relate`.
- The executable example uses only committed or inline synthetic local rows, asserts fixed output with `mustmatch`, and sends no request. A sweep uses complete labeled candidate rows with their unfiltered signals; a filtered-output example instead reruns each shown cut from a committed recording and joins the result to the full labeled input. It never derives a lower cut from an already filtered result.
- The annotate row states the existing mapped support for decide, choose, and tag. It states that mapped score is unsupported and demonstrates or describes projection of one named score into ordinary score rows before using integer boundaries 1 through K minus 1.
- The how-to never says every function accepts `--threshold`. It says a cut belongs to one question, model, output signal, labeled population, and collection boundary; a changed question, model, or incomplete candidate set requires another measured run.
- The trust page names the executable threshold how-to, makes the complete-details qualification, does not duplicate the full recipe, and exposes no private-repository link.
- The four owning files have an observed baseline of 272 nonblank lines: 54 in `specification/score.md`, 75 in the handoff, 69 in how-to 13, and 74 in the trust page. Their post-change total is at most 302 nonblank lines. How-to 13 remains at most 120 physical lines and 900 words. The implementer may consolidate or delete existing prose to meet either limit. No source ratchet changes.
- From the repository root, fixed-string checks prove the old score and 100-option claims are absent and the measured replacements are present. An exact expected-lines comparison checks the ten mapping rows rather than merely counting function names. With `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset, `mustmatch test demos/13-pick-a-threshold/README.md`, `npm --prefix site run build`, the full specification rung, and `git diff --check` pass.

## Dependencies

The three source issues are `sdlc/issues/closed/2026-09-23-score-spec-says-the-vendor-score-agrees-and-it-differs.md`, `sdlc/issues/closed/2026-09-23-relate-call-math-efficiency-and-real-jobs.md`, and `sdlc/issues/closed/2026-09-23-harvest-the-beatles-and-relate-runs-for-efficiency-thresholds-and-tuning.md`. The output meanings remain owned by `specification/result.md`, `sdlc/planning/recognize-design.md`, and `sdlc/planning/relate-design.md`. Issue `sdlc/issues/closed/2026-09-23-show-what-changes-when-the-cut-moves.md` remains separate and authorizes no work here.

## Complexity

- Contract: 1
- State and timing: 0
- Reach: 1
- Proof: 1
- Cost of error: 1
- Total: 4
- Minimum level floor: none
- Final level: 2
- Reasons: four public or durable documents change, but measured behavior, product limits, transforms, and code remain fixed. Executable local rows, exact text checks, the site build, and the specification rung prove the bounded result.
- Selected implementation model: Luna with high reasoning. Independent review checks factual fidelity, the no-behavior boundary, all-ten-function coverage, and the line budget.

Re-score and stop if the recipe needs transform code, a new output shape, a command option, a changed product ceiling, or a new claim about vendor internals.

## Review

The four opened documents now carry the experiment 235 score correction, the measured option and request-size facts, the exact ten-function threshold mapping, and the replay-versus-rerun boundary. Independent code review accepted the final change after the public trust page dropped a private-repository link and the ticket recorded the tested Node version. The coordinator ran `install`, `lint`, `test`, and `spec` in order with both backend environment variables unset. All passed. Under Node 22.22.3, the full Astro build produced 44 pages, wrote 44 Markdown twins and `llms.txt`, and passed the 44-page internal-link check. The four owning files total 268 nonblank lines. How-to 13 has 88 lines and 830 words. `git diff --check` passes.
