# Relate entities

Status: **Settled** for version one by accepted ticket 0088 and `sdlc/planning/relate-design.md`.

`relate` reads one complete entity set and prints the relationships that reach one cut. It uses the shared relation planner from `recognize`, but its input and output contain names and kinds rather than text offsets.

```sh
thinkthen relate works_for=person:organization < entities.json
thinkthen relate @entities.json --jsonl < entities.jsonl
```

## Relations

The command takes one or more ordered inline rules. `NAME=SOURCE_KIND:TARGET_KIND` names a directed rule. Bare `NAME` means `NAME=*:*`. `--either` makes every inline rule unordered. A rule name, source kind, and target kind are nonblank text without control characters. A line with another `=` or `:` is malformed.

The alternative form takes exactly one `@FILE`. The closed version-one file is:

```json
{"version":1,"relate":{"fields":{"name":"/name","kind":"/kind"},"relations":[{"name":"works_for","source":"person","target":"organization","reads":"works for","either":false}]},"threshold":0.5,"model":"jev-latest","profile":"measured"}
```

`relations` is required, ordered, nonempty, and has distinct names. `reads` defaults to the relation name with underscores replaced by spaces. `either` defaults to false. `fields` defaults to `{"name":"/name","kind":"/kind"}`. `threshold` defaults to `0.5`. `model` and saved calibration `profile` are optional. Inline rules and `@FILE` never mix. An `@FILE` beside any other rule exits 2 with zero sends.

`--field`, `--kind-field`, `--threshold`, and `--model` independently replace file values. Framing and `--input` are command-only. The command sends its requests in order and refuses `--jobs` at exit 2. Saved `profile` has no command-line replacement. `--profile FILE` selects a runtime backend profile and does not replace saved calibration identity.

## Entities

Without a framing flag, input is one JSON array. Under `--jsonl`, `--csv`, or `--tsv`, every record is one entity in the same complete set. The name and kind pointers resolve independently against each original object. Each selected value must be a nonblank string. CSV and TSV headers form the addressed object.

`--lines` uses each complete nonempty line as a name and assigns kind `*`. It takes neither pointer and accepts only bare or `*:*` rules.

The command validates the complete set before any request. It refuses a malformed rule, invalid or missing pointer value, blank name or kind, duplicate name-and-kind pair, absent concrete kind, incompatible line rule, or more than 255 entities at exit 2 with zero sends.

Empty line and JSONL streams succeed with no output and no request. A blank line is invalid in either stream. Empty CSV and TSV input is exit 2; a header with no records succeeds empty. Empty document input is exit 2. Normal and dry runs have the same outcomes.

## Output

Bare output writes one compact edge per line in relation, concrete expansion, question, and candidate order. Endpoints contain only `name` and `kind`.

```json
{"relation":"works_for","source":{"name":"Ada","kind":"person"},"target":{"name":"Acme","kind":"organization"},"probability":0.84}
```

Directed output keeps the declared direction even when the target side asks. Unordered output normalizes endpoints to input order. No edge appears twice. A probability equal to the cut is accepted.

Different-kind relations use a choice from the larger side to the smaller side plus `none`. Equal counts ask from the source side. Same-kind relations use one yes/no question per pair. `either` asks one unordered pair; a directed same-kind rule asks both directions. Wildcards expand by first-seen kind order. The shared planner and selected backend profile decide per concrete relation when a choice falls back to yes/no.

## Dry run

`--dry-run` sends nothing, reads no key, and prints one compact `thinkthen.relate-plan/1` object. Keys appear in this order: `schema`, `url`, `model`, `key_env`, `backend_profile`, `framing`, `fields`, optional `from`, `entity_count`, `relations`, `logical_questions`, `request_count`, `requests`.

Each relation carries `name`, `source`, `target`, `reads`, `either`, `method`, nullable `fallback`, `logical_questions`, and `request_count`. Each request carries its recording `digest`, UTF-8 `bytes`, and exact `body_utf8`. The plan makes no token or price claim.

A plan at the built-in address splits each relation under the built-in ceiling of `specification/backends.md`, so `request_count` can exceed one with no profile. `backend_profile` is the resolved runtime profile name or null. `fields` is null for lines. Inline rules omit `from`. A file-backed run writes `question`, `threshold`, `model`, `field`, `kind_field`, and optional saved `profile`, each as `file`, `command line`, or `default`.

## Detailed result

`--details` prints one `thinkthen.result/1` object with keys `schema`, `value`, `question`, `answer`, `meta`. `value` is the accepted edge list. `question` carries `verb`, nullable `fields`, ordered resolved `relations`, `threshold`, and optional saved `profile`.

`answer.questions` preserves every logical question. A successful choice entry carries relation identity, `method:"choice"`, public direction, asker role and entity, all entity candidates plus `none`, probabilities, inclusive `accepted` markers, the pre-threshold `pick`, and request digest. A successful H entry carries the same identity, `method:"yes_no"`, source, target, probability, accepted marker, and request digest. Failed entries preserve those identities and carry `failure`, but omit probability, accepted, and pick.

`meta.failed_questions` is always present. One or more valid logical answers beside one or more recoverable failed answers prints the complete buffered result and exits 6. Bare output prints only successful edges and exits 6. If no valid logical answer remains, the command prints nothing and exits 4. Transport, status, reply size, replay, local, output, cancellation, and defect failures never become partial success.

## Question identity

The canonical question has keys `verb`, `fields`, `relations`, `threshold`, then optional saved `profile`. Each relation has `name`, `source`, `target`, `reads`, `either`. Compact JSON bytes are hashed with SHA-256. Entities, model, address, framing, provenance, and runtime backend profile are absent. A saved calibration profile changes the digest; `--profile FILE` does not.

## Record and replay

Every exact request uses the ordinary recording digest and folder rules. Replay opens no connection and reads no key. A recorded partial reply reproduces the same successful edges, failed entries, failure count, and exit 6. Recordings and caches contain the request evidence; protect them as the input itself.
