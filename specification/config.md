# Configuration

Status: **Draft**. Every section below is Draft.

ADR 0010 proposes that the file, profiles, the `config` command, `--profile`, `--adapter`, `--key-env`, and `--config` leave version one. Under one wire shape and two variables a profile holds nothing that `THINKTHEN_API_KEY`, `THINKTHEN_BASE_URL`, and `--model` do not already say. This page waits on Ian's answer, and nothing here is built until he gives it. [backends.md](backends.md) holds the two variables, and they are Settled.

A configuration file holds profiles and run settings. The tool never writes it.

```text
thinkthen config path
thinkthen config show
thinkthen config check
```

## Where the file lives

The file is `$XDG_CONFIG_HOME/thinkthen/config.json`, or `~/.config/thinkthen/config.json` when that variable is unset or empty. `--config FILE` or `THINKTHEN_CONFIG` names another file. A flag beats the variable. A missing default file is fine, and every command runs without one. A file named by `--config` or `THINKTHEN_CONFIG` that is missing is a local failure at exit 5.

## The file

```json
{
  "version": 1,
  "profile": "local",
  "profiles": {
    "local": {"url": "http://127.0.0.1:8080/v1/systemone", "adapter": "systemone", "model": "some-local-model", "key_env": null}
  },
  "timeout_seconds": 30,
  "max_retries": 3,
  "jobs": 4
}
```

Every file the tool reads or writes is JSON, so one parser serves the configuration, the saved questions, and the results. `jq` edits all three.

| Key | Holds |
| --- | --- |
| `version` | `1` |
| `profile` | The name of the profile to use when no flag and no variable name one |
| `profiles` | A map from name to profile |
| `timeout_seconds` | The default for `--timeout` |
| `max_retries` | The default for `--max-retries` |
| `jobs` | How many requests are in flight at once in record mode. See [records.md](records.md) |

A profile is complete. `url`, `adapter`, and `model` are required, and `key_env` names the key's environment variable or is `null`.

**The file never holds a key.** It never holds a threshold, a context, an output path, or an output format. A policy belongs on the command line or in a saved question file.

An unknown key anywhere in the file is an error.

## Selecting a profile

`--profile NAME`, then `THINKTHEN_PROFILE`, then the file's `profile`, then the built-in `jev`. The built-in profile always exists, and a file profile named `jev` replaces it. [backends.md](backends.md) gives the profile rules and the ad-hoc flags.

The five `THINKTHEN_*` backend variables of ADR 0004 are gone. `THINKTHEN_PROFILE` and `THINKTHEN_CONFIG` are the two variables the tool reads.

## The three subcommands

| Subcommand | Prints |
| --- | --- |
| `config path` | The path of the file in use, whether or not it exists. It prints the file that `--config` or `THINKTHEN_CONFIG` names when either is given |
| `config show` | One JSON object with the file's own key names and every default filled in. The built-in `jev` profile appears among the profiles |
| `config check` | A verdict on the file |

None of the three sends a request, and none of them writes the file. An editor changes the configuration.

`config show` never prints a key. It prints each profile's `key_env` name, and it never reads the variable's value.

## Exit codes

| Code | Meaning |
| --- | --- |
| 0 | `path` and `show` finished. `check` found the file valid |
| 2 | Usage error: an unknown subcommand or a missing one |
| 5 | The file is missing where one was named, unreadable, or refused. `check` reports the first problem with its JSON Pointer |
| 70 | A defect in the tool |

## Examples

```sh
thinkthen config path
```

```sh
thinkthen config check && thinkthen config show | jq '.profiles | keys'
```

```sh
thinkthen decide 'The customer asks for a refund.' --profile local < message.txt
```

## Open point

- What does `jobs` default to? Draft: 4. [records.md](records.md) gives the reason.
