---
thinkthen: questions/1
min_prob: 0.9
---

# Release notes checks

Four narrow questions about facts that are either printed in a set of release notes or absent from them. None of them asks whether the notes are good.

## names_upgrade_command

The notes have to tell an operator what to run before the new build starts.

```yaml
verb: if
ask: the notes name a command to run before starting the new build
```

## states_upload_limit

```yaml
verb: if
ask: the notes state a new maximum upload size
```

## lists_known_issues

```yaml
verb: if
ask: the notes list at least one known issue
```

## session_lifetime

Three spellings of the same visible fact, plus a fourth for silence. The list is short and the options exclude one another.

```yaml
verb: which
ask: what the notes say happens to the session cookie lifetime
options: [shorter, longer, unchanged, not_mentioned]
min_prob: 0.8
```
