# ADR 0037: Carry structured question descriptions

- Status: Accepted and implemented by verified ticket 0069; landing pending
- Date: 2026-09-22

This amends ADR 0013's text-only slots and the structured-description exclusion inherited from ADR 0010. Ian authorized the widening and settled its remaining public choices at `90a544f`. He can overturn the decisions below. Ticket 0069 supplies the implementation and proof; no library API is frozen here.

## Contract

Question text accepts a nonblank string, object, or array. Descriptions accept string, object, array, or null. A top-level number or boolean is invalid; either may occur inside an object or array. The existing ordered JSON parser rejects duplicate members and nonfinite numbers. Empty arrays and objects remain values. Command-line arguments remain strings, and existing replacement precedence remains intact.

`levels` takes a list of printable nonblank strings or an ordered map from printable nonblank level names to descriptions. There are still 2–10 unique levels. Map descriptions go to the model in order; names key detailed results and probability maps. Null map descriptions remain null, never a fallback to the name. Blank string map descriptions are refused, following the existing score description rule. Structured entries inside a list are refused with advice to use the map. String-list canonical form remains unchanged; map canonical form records both names and descriptions. Result `question.levels` remains the list of names and score arithmetic is unchanged.

A tag question with a string question and only string or absent descriptions retains today's exact requests. Otherwise each expanded instruction is `[QUESTION,{"label":NAME,"description":DESCRIPTION}]`; description is omitted when null/absent. The switch applies to every label in that logical question. The existing true-criterion mapping remains, with the same description value. The adapter never templates structured JSON into text.

An explicit null true/false boundary is serialized as a present null criterion and enters the canonical digest. An omitted boundary stays omitted. Null and blank option/tag descriptions keep their existing no-description meaning. Question text cannot be null. Existing string validation remains unchanged.

## Ownership and compatibility

One validated description representation reuses `core::json::Json`; the parser, resolver, serializer and adapter own the behavior in Rust. No dynamic `serde_json::Value`, wrapper-side canonicalizer, or policy exemption is added. Compact serialization preserves written object order and existing number normalization. Description whitespace outside strings does not affect identity; content and key order do. Old string-only request bytes, canonical digests, recordings and results remain unchanged.

Profile request limits count exact compact encoded body bytes, including nested keys, punctuation and escaping. No token estimate or new limit is introduced. Record-carried `--options` and `find` retain their existing string-only inputs. Recognition, relations, packing and typed library builders remain separate work.

## Structured state amendment

Selected record evidence keeps its JSON shape in the System One `state` field. More than one `--field` pointer produces an ordered object. One pointer resolving to an object or array produces that value. One pointer resolving to a string, number, boolean, or null remains compact text. An explicit root pointer follows the selected-value rule. No pointer means the existing whole record as text. Unpointed CSV/TSV rows, whole documents, `--lines`, and `find` remain strings; multiple selected CSV/TSV columns form an object.

Evidence is one validated text-or-JSON value in the core. Debug output withholds it. Empty selected objects and arrays are valid; blank selected strings remain invalid. The backend adapter emits the value without another parse. Evidence limits count compact state-value bytes, while request limits count the exact complete request. Existing text request bytes and identities stay unchanged. Structured states intentionally create new request identities. Historical recordings remain immutable; executable examples may add current mechanically derived entries without deleting or rewriting old evidence. The evidence-shape probe keeps its measured text arm by constructing the same compact object before an unpointed call, while its structured helper reads the now-native object state directly. This updates no measured answer or recorded exchange.

This implements ruling 7 of the founder issue received from main after the description half passed review. It changes no scheduler, cache, recorder, transport, provider call, or wrapper. Ian can overturn the narrow framing decisions above.

## Schema and verification

Publish an offline JSON Schema alongside the question-file specification. It describes structural forms and reusable question entries, including optional lists that a command-line replacement can supply. The production parser remains the authority for `--dry-run` and semantic rules. Document constraints a schema cannot express, including object order, duplicate JSON member names and contextual CLI resolution. Recommend `{what, not_for, examples}` without enforcing those names.

Use Python's existing `jsonschema` tool only in the gate to validate the artifact and shared structural examples, paired with production parser checks. The install rung checks its availability. The standard library has no JSON Schema validator; a test-only tool avoids a second runtime parser and a new Rust dependency. Gates load only local files and make no schema network requests.

## Order and evidence

This feature follows completed 0065 and 0068 and precedes private controls and recognition. Private controls still precede recognition integration and the public API freeze under ADR 0017. Local gates remain the verification authority while Actions is disabled. The small recorded structured-description measurement establishes feasibility, not a general accuracy claim. No fresh provider measurement or marketing claim is authorized here.
