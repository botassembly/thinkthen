# ADR 0114: Named backends each name their key variable

- Status: **Proposed**, 2026-09-30. Ian can overturn each item.
- Date: 2026-09-30

Ian asked for several providers with separate API keys in one environment. One machine holds a TypeSafe key and a Liquid key at once. A shell script, a program, and the cache must all use both without mixing the keys. `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` stay the primary story. This ADR partly reverses ADR 0010's configuration section and amends ADR 0033 and `specification/backends.md`. Ticket 0334 builds it after ADR 0111 slice 3.

## Context

Today one key variable serves every address. `THINKTHEN_API_KEY` holds the key. `--url`, then `THINKTHEN_BASE_URL`, then the configuration file's `url`, then `https://api.typesafe.ai/v1` names the address. `specification/backends.md` says "No option names another variable." ADR 0010 removed named profiles because two variables and `--model` said everything a profile held. ADR 0032 later gave `--profile` limits and a calibration name only. ADR 0033 made the configuration file `thinkthen.config/1`, read only, with `url`, `model`, `cache`, `cache_bytes`, and later ADR 0108's two price fields.

With one key variable, a user who works with two providers must swap two variables in front of each command. A user whose `THINKTHEN_API_KEY` holds a TypeSafe key and who points `THINKTHEN_BASE_URL` at Liquid sends the TypeSafe key to Liquid. Ian's machine holds exactly that shape: `THINKTHEN_API_KEY` currently equals the TypeSafe key, and `LIQUIDAI_API_KEY` holds the Liquid key.

ADR 0010 held that the key goes to the address the user named, because naming the address is the user's own act. This ADR keeps that rule. It adds names that pair an address with its own key variable, so naming the backend names both.

## Decision

### 1. Two built-in backends

| Name | Base | Key variables, first nonblank wins | Model |
| --- | --- | --- | --- |
| `typesafe` | `https://api.typesafe.ai/v1` | `TYPESAFE_API_KEY` | `jev-1.13.0` |
| `liquid` | `https://api.liquid.ai/decisions/v1` | `LIQUIDAI_API_KEY`, then `LIQUID_API_KEY` | `d1:free` |

The `typesafe` base is today's default address. The `liquid` base and model are the ones the README gives for Liquid's d1. A backend name uses 1 to 32 lowercase ASCII letters, digits, and hyphens.

The built-in table lives under `crates/thinkthen/src/core/adapters/`, because `typesafe` is a vendor word and `policy.py` holds vendor words behind the adapter.

### 2. The configuration file names more backends and never holds a key

The file keeps the schema `thinkthen.config/1` and gains two optional fields. ADR 0108 added fields to this schema without a new version, and this ADR follows it.

```json
{
  "schema": "thinkthen.config/1",
  "backend": "liquid",
  "backends": {
    "local-d1": {
      "url": "http://127.0.0.1:8080/v1",
      "key_env": "LOCAL_D1_KEY",
      "model": "d1:free"
    }
  }
}
```

- `backend` names the backend used when nothing outranks it (section 3). It names a built-in or an entry of `backends`.
- `backends` maps a name to an object with exactly three required fields. `url` is a base that passes today's address rules. `key_env` is the name of an environment variable, matching `[A-Z_][A-Z0-9_]*`. `model` is a nonblank model name. No other field is accepted.
- An entry may not reuse a built-in name. The refusal is `configuration backend names a built-in backend; choose another name`.
- The file holds variable names and never a key. The `key_env` pattern refuses most pasted keys, because keys hold lowercase letters or hyphens. The tool still never echoes a `key_env` value it refuses.
- Each refusal exits 2, names the field, and never repeats a value. The existing unknown-field sentence becomes `the configuration file holds a field other than `schema`, `url`, `model`, `cache`, `cache_bytes`, `backend`, and `backends``.
- The tool still reads the file and never creates or edits it. The warning for a file another user can write stays word for word. Its reason now covers the key variable as well as the address.

A keyless local server takes an entry whose `key_env` names a variable left unset. Today's loopback rule then sends no `Authorization` header.

### 3. Selection order

A setting's tier decides, as `specification/settings.md` states for every setting. The typed tier outranks the environment tier, which outranks the configuration tier.

| Tier | Names a backend | Names an address |
| --- | --- | --- |
| Typed | `--backend NAME` | `--url BASE` |
| Environment | `EngineBuilder::backend` and each library's `backend` engine setting, then `THINKTHEN_BACKEND` | `EngineBuilder::base_url`, then `THINKTHEN_BASE_URL` |
| Configuration | `backend` | `url` |

The rules:

1. Walk the tiers from the top. The first tier that names a backend or an address decides.
2. That tier names a backend and no address: the backend's base, key variables, and model apply.
3. That tier names an address and no backend: the unnamed path applies. It is today's behavior exactly. The key comes from `THINKTHEN_API_KEY`, and the model follows today's order.
4. That tier names both: the backend's key variables and model apply at that tier's address. `--backend liquid --url http://127.0.0.1:8080/v1` sends the Liquid key to the loopback server.
5. A lower tier's address never replaces a higher tier's backend. `--backend liquid` ignores `THINKTHEN_BASE_URL`, as `--url` ignores it today.
6. A higher tier's address outranks a lower tier's backend. `THINKTHEN_BASE_URL` set in the shell outranks `backend` in the configuration file and takes the unnamed path.
7. No tier names either: the unnamed path at the built-in address, as today.

The recommendation placed the library engine setting and the SQL setting last, after the configuration file. `specification/settings.md` puts every engine-level library setting and SQL session setting in the environment tier. So this ADR puts `backend` there too, above the configuration file, as `base_url` and `batch` sit today. An explicit setter outranks what `EngineBuilder::from_env` captured, as it does for `base_url`.

The model on the named path is `--model`, then the question file's `model`, then the engine's `model`, then the backend's model. The configuration's top-level `model` and `jev-1.13.0` apply only on the unnamed path.

### 4. The key on the named path

A named backend reads only its own key variables, in the table's order. The first nonblank value wins. `THINKTHEN_API_KEY` is not read on the named path, unless a configuration entry names it as its `key_env`. On a bare Rust builder, an explicit `api_key` setter still supplies the key; section 5's refusal applies to it.

The recommendation kept `THINKTHEN_API_KEY` as an override on every path. The code and Ian's own environment show a conflict. If `THINKTHEN_API_KEY` outranked the backend's variable, then `--backend liquid` on Ian's machine would send the TypeSafe key to Liquid on every call. So `THINKTHEN_API_KEY` stays the key of the unnamed path and overrides no named backend. `THINKTHEN_BASE_URL` still overrides a backend named in the configuration file, by rule 6. Ian can overturn this: the alternative lets `THINKTHEN_API_KEY` outrank every backend's variable, and a user whose key belongs to one provider must then unset it before naming another.

Every existing key rule applies to the selected key unchanged. A key holding a control character is refused. A key that occurs in the posting URL is refused with today's sentence. An unset or blank key exits 4 before any connection, and a loopback address then sends no `Authorization` header. The exit-4 sentence keeps its form and names the backend's first key variable: `the environment variable `LIQUIDAI_API_KEY` is unset or blank, so no key is sent`. A replay and a dry run read no key, as today.

### 5. A built-in's key never goes to another built-in's host

The rule: when the selected backend reads a variable that a built-in backend lists, and the host of the final posting URL equals the host of a different built-in backend's base, the call is refused. The comparison is exact ASCII host equality after today's host lower-casing. It ignores port and path. `api.typesafe.ai` and `api.liquid.ai` are the two known hosts today. A subdomain does not match.

The refusal exits 2 before any key is read, any cache or recording is opened, or any connection is made. It prints no address and no key. Its whole standard error line reads:

`thinkthen: backend `NAME` reads `VARIABLE`, the key of backend `OWNER`, which never goes to the address of backend `OTHER``

`VARIABLE` is the first built-in variable the backend reads. For example, `--backend typesafe --url https://api.liquid.ai/decisions/v1` prints `thinkthen: backend `typesafe` reads `TYPESAFE_API_KEY`, the key of backend `typesafe`, which never goes to the address of backend `liquid``. A configuration entry that names `TYPESAFE_API_KEY` with a Liquid base is refused the same way. A second TypeSafe account on `api.typesafe.ai` under its own variable passes, because its variable belongs to no built-in.

The recommendation aimed this refusal at `THINKTHEN_API_KEY`. Two facts move it. First, the README documents the unnamed Liquid path: `THINKTHEN_BASE_URL=https://api.liquid.ai/decisions/v1` with a Liquid key in `THINKTHEN_API_KEY`. The tool cannot know which provider issued `THINKTHEN_API_KEY`, so a refusal there would break that documented path. Second, section 4 keeps `THINKTHEN_API_KEY` off the named path, so it can no longer reach a provider through a backend name. The refusal therefore guards the one pairing the tool can know: a built-in's own variable and another built-in's host.

An unknown host passes. A user who types `--backend typesafe --url https://gateway.example/v1` named that address, and ADR 0010's rule sends the key there. Ian can overturn the scope in either direction. A stricter rule sends a built-in's key only to its own host or to loopback, and it blocks gateways and proxies with their own host names. A looser rule drops the refusal.

### 6. Other sentences and surfaces

- An invalid backend name exits 2 with `a backend name uses 1 to 32 lowercase letters, digits, and hyphens` and repeats nothing.
- An unknown valid name exits 2 with `unknown backend `NAME`; the built-in backends are `liquid` and `typesafe`, and the configuration file may name more`. The builder writes the built-in list from the table.
- `thinkthen check` treats a named backend as a named address. Its refusal becomes `check needs an address you name: give --url or --backend, set THINKTHEN_BASE_URL or THINKTHEN_BACKEND, or set url or backend in the configuration file`.
- `thinkthen status` prints `backend NAME` or `backend none`, and `key_variable NAME` for the variable it would read. `api_key_set` keeps its meaning for that variable. No line holds a key.
- `--help` for `--backend` names no provider, so the vendor word stays behind the adapter.
- A program holds one engine per backend at once. Each engine keeps its own backend, address, and key. One command run uses one backend. A shell script mixes backends by naming one on each command.

### 7. The cache needs no change

ADR 0111's question key hashes the adapter name, the posting URL, the model, the state, and one question. It never includes the key. So answers from two providers never mix, because their URLs differ. Two backends with one URL and one model share answers, because the same service answered the same question. A key change never invalidates or reveals a stored answer. The store, the fixtures, the usage totals, and recordings hold no backend name and no key.

## What this amends

| Source | Change |
| --- | --- |
| ADR 0010, configuration section | The configuration file again pairs an address with a key variable name. Named backends replace the profile's old address and key role. `--profile` still carries limits and a calibration name only |
| ADR 0010, ruling 2 | The two variables stay the unnamed path. Built-in backends add their own key variables |
| ADR 0033 | `thinkthen.config/1` gains `backend` and `backends` |
| `specification/backends.md` | "No option names another variable" gives way to sections 1 to 5 |
| `specification/settings.md` | New rows for the backend, `THINKTHEN_BACKEND`, and each built-in key variable |

## Build order

Build after ADR 0111 slice 3, which rewrites the public builder, the facade, and the SQL hosts that this change touches.

1. **Command, configuration, and the Rust builder.** Ticket 0334.
2. **Bindings and SQL.** Python, TypeScript, Ruby, R, C, and the three SQL extensions gain a `backend` engine or session setting over the Rust builder. A later ticket.

## What Ian can overturn

The recommendation Ian received, as built:

1. The two built-ins and their variables, `LIQUID_API_KEY` included.
2. The configuration shape: `backend`, `backends`, three required fields, no reuse of a built-in name.
3. The cache stays unchanged.

The coordinator's adjustments where the code conflicted, each stated above:

4. The library and SQL setting sits in the environment tier, above the configuration file (section 3).
5. `THINKTHEN_API_KEY` is not read on the named path (section 4).
6. The refusal guards a built-in's own variable at another built-in's host, and unknown hosts pass (section 5).

## Open questions for Ian

1. Which other providers should become built-ins, and under which variable names?
2. Is the refusal in section 5 too strict or too loose? The options are the current rule, own host or loopback only, or no refusal.
