Status: closed 2026-10-02 by the 0.1 milestone sweep. All three criteria are met. Commit `248bc6aa4` added the overhead sentence to `README.md`: "On 2026-10-01 a call to Jev took a median of 138 ms. ThinkThen's own work took about 2 ms of it. Each binding added a median of 3 ms or less, and the slowest single function added 6.7 ms." It links the site's overhead page, which names the measuring run: checkpoint 3, commit `bfc180a10`, site ticket 0050. The same sentence is on `release/0.1`. Owner: the queue owner.
Resolution: 248bc6aa4

Criteria 1 and 2 are met: the README names TypeSafe's site for a key, `THINKTHEN_API_KEY`, `THINKTHEN_BASE_URL`, `--model` and its default. Ticket 0316 rechecked them on 2026-09-30. At that recheck no overhead benchmark had run.

# README: where to get a key, how to change the backend, and the overhead line

## What Ian asked for

1. **Getting a key.** The README links to TypeSafe's website, where a user gets their own key. The key goes in `THINKTHEN_API_KEY`. Today the README names the variable at line 84 but never says where a key comes from.
2. **Another backend or model.** A user who points ThinkThen at another backend sets `THINKTHEN_BASE_URL` and gives that backend's key in `THINKTHEN_API_KEY`. Say this in one or two sentences next to the key line, with the model setting (`--model`, default `jev-1.13.0`).
3. **Overhead.** After the overhead benchmark runs (marketing's 2026-09-29 0.1 punch list item 8), the README gets one or two sentences on how much time ThinkThen adds to Jev's own time by language. The sentence names the run that measured it.

No launch coordination with TypeSafe is needed (Ian, 2026-09-29).

## Done when

The README says where to get a key, how to set the key and the address, and how much time ThinkThen adds, with the measuring run named.
