# The shipped help breaks the approved fixed words

Status: Closed on 2026-09-22. Merged into 2026-09-22-command-wording-and-help-fixes-for-0-1.md.

The approved vocabulary (`products/thinkthen/vocabulary.md` in the marketing repository, approved 2026-09-21) fixes one word per concept. The shipped help, the README, and two how-tos break five of those fixed words. Every fix is mechanical.

| Word used | Fixed word | Where |
| --- | --- | --- |
| label (for what `choose` picks from) | option | main help `choose` line, `choose --help` lines 1 and 7, `score --help` line 9 |
| document | text | `--field` and `--jobs` option paragraphs in seven of eight function helps |
| row (for a record) | record | `filter --help` line 3 and the shared record paragraphs on all record verbs |
| judgment | question or answer | `annotate --help` line 11, README line 15, demos 13, 16, 17 |
| failed | broken | `decide --help` line 13, `choose --help` line 7 |
| rating | level, place on a scale | README line 3, demo 17 line 78 |

## Reproduction

    $ grep -c document scratch/help/decide.txt
    3
    $ grep -n "CSV and TSV rows" scratch/help/filter.txt
    3:... CSV and TSV rows become compact JSON objects ...
    $ grep -n judgment scratch/help/annotate.txt
    11:          A JSON question set containing the named judgments to apply

The vocabulary's own option row also says "One of the labels `choose` picks from," so the vocabulary table needs the same fix on the marketing side. The vocabulary keeps "judgment" in the library pages only, and the first-week copy of the command, the README, and the how-tos carries it. "Header row" for a CSV header is standard and stays.

## Expected

One spelling per concept, per the vocabulary's fixed-words table.

Added 2026-09-21 after "The words for numbers": the word "failed" now has a sanctioned use, a question the backend could not answer inside an otherwise good request. The two help rows above still want "broken", because those sentences name the outcome of a run that could not get an answer, not a refused question. The fixer reads both rules: "failed" names a refused question; "broken" names the broken outcome.

## How bad it is for a user

Minor. The words confuse a reader moving between the deck, the help, and the manual, and nothing breaks.

Found by experiment 218, wave 1, area 12.
