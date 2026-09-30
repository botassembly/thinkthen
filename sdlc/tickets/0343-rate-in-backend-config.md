# 0343: The requests-per-minute rate is set per backend in the configuration file, with no default

Status: in progress. Lane claude-4. Branch `ticket/0343-rate-in-backend-config`. Plan: `sdlc/planning/cleanup-2026-09-30.md`, ruling 14, recorded with this ticket. Amends ticket 0308 and ADR 0114 section 2.

## Ruling

Ian, 2026-09-30: "Just make it configurable. There shouldn't be throttling at first. You set it or don't set it, and it can be done per backend in your configuration. There shouldn't be a default number for liquid. Everything should get the same configuration, and it should be in the settings file, not in the backend." Ian can overturn it. The cleanup plan records it as ruling 14.

## Outcome

No address is paced by default. The 1,000-a-minute default for an `https://` address from ticket 0308 is gone.

The configuration file (`thinkthen.config/1`) may set `requests_per_minute`, a whole number from 1 to 60,000, on any backend in its `backends` object. A built-in name (`typesafe`, `liquid`, `ollama`) may appear there holding only `requests_per_minute`. That paces the built-in and leaves its url, key variables and model as built. An added entry may carry `requests_per_minute` beside `url`, `key_env` and `model`. The built-in backend table in the code carries no rate.

```json
{
  "schema": "thinkthen.config/1",
  "backend": "liquid",
  "backends": {
    "liquid": {"requests_per_minute": 1000},
    "local-d1": {"url": "http://127.0.0.1:8080/v1", "key_env": "LOCAL_D1_KEY", "model": "d1:free", "requests_per_minute": 120}
  }
}
```

`THINKTHEN_REQUESTS_PER_MINUTE` stays and outranks the file, so the bench's current workaround keeps working. The rate applies when the backend it belongs to is selected by any tier. The pacer keeps its per-process, per-address behavior: the limit holds within one process, and separate processes each get the full rate. Every page that describes `requests_per_minute` says so plainly and names the future answers for a limit across processes: `sdlc/issues/2026-09-30-proxy-service-for-shared-limits-and-traces.md` and `sdlc/issues/2026-09-30-batch-command-runs-many-questions-in-one-process.md`.

Every surface builds from `EngineBuilder::from_env`, which reads the file. So the command, Rust, every library, the C door and the three SQL extensions pace at the file's rate for the backend that the file's `backend` field or `THINKTHEN_BACKEND` selects. The command and Rust also select a backend with `--backend` and `EngineBuilder::backend`. The other surfaces gain a per-engine backend setting in ADR 0114 build slice 2.

## Evidence

- Starts from: main `f0308dd9d`. Ticket 0308 built the pacer in `engine/backoff.rs` with a default of 1,000 a minute for `https://` addresses. ADR 0114 section 2 made `backends` a map of added entries with exactly three fields and refused a built-in name. `config/backends.rs` reads the entries. `core/backend/named.rs` finds a name among the configured entries first, then the built-ins. The public builder and the command both resolve the backend through `Choice::backend`, so a value the resolved `Backend` carries reaches every surface.
- Keeps: the pacer's mechanics from 0308: one next-slot time per posting address in each process, paced before the throttle place, cancel and deadline behavior, and replayed or cached answers never wait. `THINKTHEN_REQUESTS_PER_MINUTE` keeps its range and its refusal sentence, `THINKTHEN_REQUESTS_PER_MINUTE takes a whole number from 1 to 60000`. Every other configuration refusal stays word for word, and no refusal repeats a value. A file another user can write still may not hold `backends`; a rate-only entry counts, because it is part of `backends`. The tier rules of ADR 0114 section 3 are unchanged. The unnamed path keeps `THINKTHEN_API_KEY` and its model rules.
- Changes:
  - `engine/backoff.rs`: `DEFAULT_PER_MINUTE` goes. `interval` takes the rate alone and returns no spacing without one.
  - `core/backend.rs`: `Backend` carries an optional rate. `core/backend/named.rs`: `Named` carries it, and `Choice::backend` hands the selected backend's rate to the resolved `Backend`. With no backend named, a run whose posting URL equals a built-in's own base takes the rate the file gives that built-in. So a rate on `typesafe` paces a run that names nothing, and the README's unnamed Liquid setup takes the `liquid` rate. An added entry's rate applies only when a tier names it. The built-in table is untouched.
  - `engine/facade.rs`: the client's spacing is the variable's rate, else the backend's rate, else none. `fresh` reads the rate from the engine's own `Backend`. `with_model` rebuilds a `Backend` for a per-call model and carries the rate over, because a forked child may rebuild the shared client from that copy first.
  - `config/backends.rs`: an entry may hold `requests_per_minute`. A built-in name holding only `requests_per_minute` becomes that built-in with the rate. New refusals, each exit 5 and value-free:
    - `a configuration entry for a built-in backend holds `requests_per_minute` and nothing else` replaces `configuration backend names a built-in backend; choose another name`.
    - `configuration backend entries hold only `url`, `key_env`, `model`, and `requests_per_minute`` replaces the three-field sentence.
    - `configuration backend field `requests_per_minute` must be a whole number from 1 to 60000`.
  - Pages: `specification/settings.md` rows "Requests a minute" and "Named backends" and the configuration file paragraph; `specification/backends.md` "Named backends"; `specification/recording.md` field list; `specification/records.md` rate paragraph; `CHANGELOG.md`. Each says the limit holds within one process, that separate processes each get the full rate, and names the two issues above. ADR 0114 gains a status note that this ticket amends section 2.
  - `sdlc/scripts/settings` checks the settings table's "Configuration file" column both ways against the fields the configuration reader accepts, so a new file field needs a row. Its self-test plants a missing field.
- Proof:
  - A command test on loopback in a scratch `XDG_CONFIG_HOME`, each leg asserting only a lower bound on the spread of request starts so a loaded machine cannot fail it: with `requests_per_minute` 600 set only in the file for the `ollama` built-in, `--backend ollama --url LOOPBACK` spaces six requests at least 400 ms. With the file at 60000 and the variable at 600, the variable's rate wins and the same spread holds.
  - A public test builds from `EngineBuilder::from_env` with the file's `backend` naming an added entry that carries a rate, and shows the spacing.
  - An edge table in `config/backends.rs` pins each refusal sentence for a bad rate (0, 60001, a negative number, a fraction, a string, null), a built-in holding another field, an empty built-in entry, and an added entry with an unknown field, and asserts no sentence holds the marker value.
  - A `core::backend::named` edge table shows the unnamed path taking a built-in's rate at that built-in's base, in any host case, and at no other address, and never an added entry's rate.
  - The removed `https://` default cannot be shown on loopback, because a plain `http://` address was never paced. The unit test `the_rate_variable_takes_whole_numbers_from_1_to_60000` carries that proof: `interval` takes only a rate and gives no spacing without one, and `DEFAULT_PER_MINUTE` no longer exists to be applied. A `Choice::backend` unit test shows a built-in found through a rate-only entry keeps its base, key variables and model and carries the rate, and the same built-in unconfigured carries none.
  - Checks: `sdlc/scripts/test`, `spec`, `settings`, workspace clippy with `-D warnings`, `policy.py`, `tickets`, lint in a clean checkout, the C door tests.
- Defers: a limit shared across processes, as in 0308. Library and SQL setters for the rate; they read the variable and the file through `from_env`. `status` showing the rate. The `site/` reference sentence on requests a minute, which marketing owns (see What the build taught us).
