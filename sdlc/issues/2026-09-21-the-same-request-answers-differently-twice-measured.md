# The same request answers differently twice, measured

Status: Open

Item 5 of `2026-09-21-where-a-user-could-lose-trust-a-first-list.md` asked whether one request sent twice gets one answer. It does not. `experiments/212-thinkthen-repeat/` holds the probe, and Ian authorized probes on 2026-09-21. One capped job sent 100 SMS messages twice, minutes apart, with one bare question, no cache, and no retries. Fifty messages were picked because they sat near 0.5 the day before, and fifty were taken in sample order. Every answer came from `jev-1.13.0`.

| Compared | Probabilities that differ, of 100 | Largest difference | Flips at a 0.5 cut |
| --- | --- | --- | --- |
| Two runs minutes apart | 63 | 0.08 | 4 |
| Today and 2026-09-20 | 68 | 0.08 | 6 |

Most differences were 0.01. The fifty clear messages moved by 0.03 at most. The fifty borderline ones moved by up to 0.08. Every flip sat within 0.08 of the cut, and a band of 0.4:0.6 would have sent all four to a person in both runs. The job reported 59,646 input tokens under a 75,000 reservation.

## What follows

1. **The pages say it.** A probability is a reading with about 0.08 of play on a hard case, inside one model version. `specification/threshold.md` and the how-to on picking a threshold state the number and its test. No page says the same input always gives the same answer.
2. **The band is the answer, and the number sizes it.** A band narrower than about 0.1 on each side of a cut does not hold a flip out. The three-way gate page can say why its band is as wide as it is.
3. **`--cache` and `--replay` are what make a run repeatable.** That is a second reason to teach them, beside the bill. An eval that compares two question files reruns both live or replays both. One live and one replayed is not a fair pair.
4. **The comparison transform needs a floor.** A change of 0.08 between two runs is noise. `transforms/` reports every changed value today. The comparison how-to says which changes to ignore, or the transform takes a tolerance.
5. **Repeated trials earn their place.** The builder's plan already averages saved probabilities across trials of one case. This is the measurement behind it.

Limits: one question, one public set, one hundred messages, one day, half of them picked for being borderline.

## What Ian can overturn

All of it. Point 4 is the only one that may change code.

## Update, 2026-09-25: committed recordings show it, and a re-record stops on it

Found by experiment 259, a check of the ThinkThen talk's slide claims. The build under test was thinkthen 0.0.1 at e70bddab. Main was at 365fc938, and no command code changed between them. No key was used, and no request left the machine.

### What happens

The public Beatles Bench repository commits several recordings. Some request digests appear in more than one of them. Of 681 such digests, 178 hold different answers. Every one of those answers came from `jev-1.13.0`. The largest gap is 0.09. No yes or no answer crossed 0.5, but some crossed the 0.7 and 0.8 bars the examples use.

One case: Octopus's Garden under the Abbey Road question. The request bytes match, so the digest matches. The answers differ.

    $ BB=path/to/beatles-bench
    $ d=a35187df1644ea1ff2fe3563c374a4e96062a8a132a0b6a4d8e29d85aef8121c.json
    $ cmp <(jq -c .request $BB/examples/05-filter/recording/$d) \
          <(jq -c .request $BB/examples/11-audit/recording/$d) && echo same request
    same request
    $ for f in $BB/examples/{05-filter,11-audit}/recording/$d; do
        jq -c '[.response.model, .response.answers.q1.noul]' $f; done
    ["jev-1.13.0",0.66]
    ["jev-1.13.0",0.75]

The whole count comes from a short script. It lists every committed recording entry, groups the entries by file name, and compares the stored answers:

    $ cd $BB && python3 - <<'END'
    import collections, json, os, subprocess
    paths = [p for p in subprocess.run(["git", "ls-files"], capture_output=True, text=True).stdout.split()
             if "recording" in p and len(os.path.basename(p)) == 69 and p.endswith(".json")]
    by = collections.defaultdict(list)
    for p in paths:
        r = json.load(open(p))["response"]
        by[os.path.basename(p)].append((json.dumps(r.get("answers"), sort_keys=True), r.get("model")))
    multi = [v for v in by.values() if len(v) > 1]
    diff = [v for v in multi if len({a for a, _ in v}) > 1]
    print("recorded more than once", len(multi), "with different answers", len(diff),
          "models", collections.Counter(m for v in diff for _, m in v))
    END
    recorded more than once 681 with different answers 178 models Counter({'jev-1.13.0': 361})

Most of the 178 (165) sit between the bench's seed run and its final Jev run. The other 13 sit among the `01-decide`, `05-filter`, and `11-audit` example recordings.

### What a user hits

`--record` into a folder that already holds the entry asks the backend again. When the answer differs, the run stops at exit 5. Records before the conflict stay written. The offline loopback backend in `conformance/backend` shows this. The loopback answers every request with one fixed rule, so the test edits the stored answer to stand in for a changed live answer. The key value below is a placeholder the loopback accepts. It is not a real key.

    $ cargo build -p conformance-backend
    $ mkfifo lbin; target/debug/conformance-backend < lbin > lbo & exec 3> lbin
    $ U=http://127.0.0.1:$(head -1 lbo)/generic/v1
    $ x() { printf 'Come Together\n' | THINKTHEN_API_KEY=loopback-placeholder \
        thinkthen decide 'It appears on the album Abbey Road.' --lines --details --url $U "$@"; }
    $ x --record rec | jq -c '{p: .answer.probability, cached: .meta.cached}'
    {"p":0.9,"cached":false}
    $ sed -i -E 's/"noul":[0-9.]+/"noul":0.42/' rec/*.json
    $ x --record rec; echo "exit $?"
    thinkthen: the entry `c8c4a000…c047.json` already records a different response
    thinkthen: stopped at record 1; 0 records finished, 0 records from a recording
    exit 5

In the bench data, 26% of repeated requests (178 of 681) got a different answer. A user who re-records into a used folder should expect this stop often. No alias move is needed.

### What the spec says

- `specification/recording.md`, "An entry": a different stored response is a local failure at exit 5. The same page says a run that receives different responses for one digest stops at the conflict, and a repeated trial that wants another backend answer uses a fresh folder. The spec decides the stop.
- `specification/recording.md`, just before "What replay changes in a result": "The first live answers on 2026-09-19 returned the same probability for two identical requests. ADR 0010 holds the measurement."
- ADR 0010, "The first live answers": "The same request returns the same number, so a repeated trial means something only when the candidate's output changes."
- Item 42 of `2026-09-22-command-wording-and-help-fixes-for-0-1.md` already cuts any page line that implies a live call is repeatable. The spec sentence and the ADR fact were not revised with it.

### Why it matters

A reader of the spec or ADR 0010 learns that Jev repeats itself. Two recordings of one request then disagree, and a re-record stops at exit 5 with a message about a "different response". The user has no reason to expect a conflict, and the message does not say the backend changed its answer. The slides drawn from earlier live calls also differ from replay for this reason.

### Options

1. Add a dated amendment to ADR 0010. It keeps the 2026-09-19 measurement as history, states the later measurements (this issue's 100-message probe and the 681 bench digests), and withdraws "the same request returns the same number".
2. Replace the `recording.md` sentence with the measured play: about 0.08 to 0.09 on one model version, most differences 0.01.
3. Word the conflict message for the common cause. For example: the backend answered this request differently from the saved entry; record into a fresh folder, or use `--cache` to keep the saved answer.
4. Say in the `--record` help that a used folder can stop at a conflict, and point to a fresh folder or `--cache`.

Options 1 and 2 are documentation. Option 3 changes a fixed message. Option 4 changes help text. The stop itself stays, because the spec decides it.
