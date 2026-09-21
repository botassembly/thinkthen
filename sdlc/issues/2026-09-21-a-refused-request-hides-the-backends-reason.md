# A refused request hides the backend's reason

Status: Open

Experiment 219 sent two requests over the vendor's size limits on 2026-09-21. Both came back as exit code 4 with one line:

    thinkthen: the backend answered with status 400

A user who piped in a long contract learns nothing from that line: not that the text was too large, not the limit, not what to do. `--record DIR` writes nothing for a refused exchange, so the vendor's own message cannot be read afterward either. The observed limits were 31,826 input tokens answered and about 33,150 refused for one text with one question, and 61,819 answered and about 66,000 refused for a whole request.

Two asks. The error carries the backend's own message, trimmed and with no header, so the reason reaches the user. A refused exchange is recorded like an answered one, body only. A profile that knows its backend's limits and refuses before sending is the larger fix, in `2026-09-21-size-cost-and-other-backends-what-the-manual-and-the-tests-must-carry.md`.

## A second, smaller finding from the same run

`--dry-run` accepted `--jobs 1` on a single document and printed a plan at exit 0. The live run refused the same command at exit 2: "--jobs bounds the requests in flight, and one document sends one request". A dry run should refuse what the real run refuses. The mistake cost a 25,000-token reservation and sent nothing.

Ian can overturn both asks.
