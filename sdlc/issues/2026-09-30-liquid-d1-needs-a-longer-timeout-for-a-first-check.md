# Liquid d1 timed out on the default timeout for a first `check`

Status: open. Reported by the Beatles Bench team on 2026-09-30 from its live runs. Owner: ticket 0334's follow-up for built-in backend settings, or marketing's Liquid page if the docs route is chosen.

## The problem

The first `check` against Liquid's d1 timed out under the default timeout of 30 seconds (`specification/settings.md`, the Timeout row). The same check passed with a timeout of 90 seconds. A new user who follows the README's Liquid setup meets a timeout before any answer.

## Options

1. ADR 0114's `liquid` built-in carries a longer default timeout, such as 90 seconds, used when `--timeout` and the configuration name none. This is a per-backend setting. Ticket 0334 defers per-backend profiles and limits, so it needs a follow-up to 0334, not a change inside it.
2. The docs say so. The README's Liquid paragraph and the Liquid d1 page (`2026-09-29-docs-page-for-the-liquid-d1-backend.md`) tell the user to pass `--timeout 90` on a first run.

Recommendation: option 2 now, because it costs pages only; option 1 when a second backend needs its own default. Ian can overturn this.

## Evidence

The bench's live run notes of 2026-09-30: the first `check` timed out at 30 s and passed at 90 s. No thinkthen run repeated it; this issue ran no paid call.
