# ADR 0031: Show `--url` in short help

- Status: Accepted
- Date: 2026-09-21

## Context

ADR 0007 puts advanced backend options in long help. A first run uses the default hosted address unless `--url` or `THINKTHEN_BASE_URL` names another address. A user reading short help must be able to see the command-line control before sending evidence or a key to the default service.

## Decision

`--url` is the one advanced option shown in both short and long help. Every judging verb follows this rule, including `find`. `--model`, `--timeout`, `--max-retries`, `--record`, `--replay`, `--cache`, and `--jobs` remain long-help only.

This exception changes discovery alone. It does not change address precedence, defaults, request bytes, or key handling.

## Consequences

Short help grows by one option. It exposes the control that determines where a request goes while keeping tuning, recording, and concurrency controls on the long screen. A later change to any other advanced option needs its own decision.
