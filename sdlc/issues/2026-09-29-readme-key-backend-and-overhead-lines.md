Status: Open. Filed 2026-09-29 by the marketing lead from Ian's launch review. The main builder owns `README.md`.

# README: where to get a key, how to change the backend, and the overhead line

## What Ian asked for

1. **Getting a key.** The README links to TypeSafe's website, where a user gets their own key. The key goes in `THINKTHEN_API_KEY`. Today the README names the variable at line 84 but never says where a key comes from.
2. **Another backend or model.** A user who points ThinkThen at another backend sets `THINKTHEN_BASE_URL` and gives that backend's key in `THINKTHEN_API_KEY`. Say this in one or two sentences next to the key line, with the model setting (`--model`, default `jev-1.13.0`).
3. **Overhead.** After the overhead benchmark runs (marketing punch list item 8, `mktg/sdlc/planning/2026-09-29-thinkthen-0-1-punch-list.md`), the README gets one or two sentences on how much time ThinkThen adds to Jev's own time by language. The sentence names the run that measured it.

No launch coordination with TypeSafe is needed (Ian, 2026-09-29).

## Done when

The README says where to get a key, how to set the key and the address, and how much time ThinkThen adds, with the measuring run named.
