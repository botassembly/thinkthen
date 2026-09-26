# ADR 0054: Ask only what can matter, and speed wins inside the noise

- Status: Accepted 2026-09-26 by Ian's ruling. Ian can overturn it
- Date: 2026-09-26

This ADR states the rule that every function, planner and default follows when cost and accuracy pull apart. It came from a mistake in `recognize`, described below and in `sdlc/records/2026-09-26-recognize-asked-for-names-nobody-wanted.md`.

## Context

`recognize person` should return the people in a text. On main it returns every name in the text, songs, places and companies included, and labels each one `person`.

`recognize` works in two steps. The detection question asks the model which words start and continue a name. Its wording is fixed. It lists news categories: person, organization, place, nationality, event, product and creative work. The caller's kinds never appear in it. The kind question then asks which of the caller's kinds each name is. With one kind the tool skips that question and labels every detected name with the one kind (`core/recognize.rs`, `engine/facade/recognize.rs`). The design followed the letter of a 2026-09-21 ruling, "a caller who wants names without kinds gives one kind", and missed its point.

The first repair proposed kept the fixed detection question and added `none` to the kind question, so that a name of another kind could be dropped. That repair pays twice. The model finds names nobody asked for, and a second question throws them away. At one kind it roughly doubled the tokens a word. Nobody had measured the obvious alternative, which puts the caller's kinds into the detection question. Every recognize experiment before 2026-09-26 measured the fixed wording. Local experiment 274 now measures the alternative.

Ian's ruling of 2026-09-26: "Our choices need to be the most efficient thing. Filter at the earliest step. If something's not going to be viable, then don't ask for it in the first place. Speed beats accuracy when the accuracy is rounding error and noise. All these errors happen always. Batching creates noise changes."

## Decision

1. **Ask only what can matter.** A function asks the model only questions whose answers can reach the output. It rules out every candidate a cheaper step can rule out before it asks. Examples: detection asks only for the kinds the caller gave; a relation asks only about pairs whose kinds some relation type allows; no question goes out for a record that an earlier step already dropped.
2. **Filter at the earliest step.** When a later step exists only to discard what an earlier step produced, move the condition into the earlier step.
3. **Speed wins inside the noise.** Every answer from the model varies from run to run. Over the 306 Beatles titles, three runs of one request each disagreed on 3 to 5 titles (section 14 of `sdlc/records/2026-09-26-batching-and-recognize-evidence.md`). A slower or costlier method counts as more accurate only when its gain is larger than that run-to-run spread on the same inputs. Inside the spread, the faster and cheaper method is the default.
4. **A real loss is measured, stated and switchable.** When the fast method loses by more than the spread, on something users rely on, the loss is measured, stated plainly on the page, and a setting gives the slower method. Batching's `--batch 1` is the pattern. The slow method becomes the default only when the loss is beyond the noise and the function's purpose fails without it.
5. **Measure the obvious alternative before choosing.** A default is not settled until it has been measured against the simplest alternative that asks less. A ticket that sets or keeps a default names that comparison, or names it as a deferred gap.

## Consequences

- Batching stays the default under ADR 0048 and ADR 0053. Its answer shifts are the noise that every run has, and it sends the 306 titles in one request.
- Ticket 0147's item 4, `none` in the kind question, ships. Local experiment 274 measured labelled detection as the cheaper alternative, and its loss was beyond the noise on bare kinds. The record above gives the figures.
- Ticket 0147's item 5, a yes-or-no question for each pair, asks only about pairs a relation rule allows.
- An audit of all ten functions against this ADR files an issue for each place that asks more than can matter or keeps an unmeasured default.
- Review checks new tickets against items 1 to 5.
