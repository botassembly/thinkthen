# The site's C samples predate the door's grammar

Filed 2026-09-24 by Claude during ticket 0094.

The eleven `site/src/data/examples/*__c.json` files hold the deck's drawn C samples, marked `"status": "drawn"`. They were written against the retired `surfaces` branch door, and several no longer run against the door that ticket 0094 landed at `libraries/c`. Examples:

- `rank__c.json` asks `{"decide": ..., "rank": true, "records": [...]}`. The door's only rank spelling is `{"rank": ..., "records": [...]}`, and its reply is the records in order, not their places.
- The relate sample passes bare sentences and the spec `{"either": ["contradicts"]}`. Each relate text is now one JSON record with `name` and `kind`, and the spec is a version-one relate file.
- The recognize sample passes `{"kinds": [...], "relations": {...}}`. The spec is now a version-one recognize file, and its entities carry `name`, `kind`, `start`, `end`, and `strength`.

`libraries/c/examples/functions.c` holds one compiled, tested call per function, and `libraries/c/DESIGN.md` gives the envelope grammar. A site ticket can draw its C tab from those files and mark each sample as run. This issue authorizes no work.
