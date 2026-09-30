# About a third of the spec's "no calls" edges show only after a real send, and the spec lags the build in places

Status: open. Reported by the Beatles Bench team on 2026-09-30 from the private release QA suite's first edge pass. Owner: the queue owner, who feeds each finding back into `specification/`.

## The problem

The release QA suite tried to check each edge the specification marks as needing no calls. About a third of them can be seen only after a real send: the refusal or behavior appears on a reply, not before the request. In other places the specification describes behavior the build has since changed.

The recording page lag is one such case. It now sits in `2026-09-30-old-batching-files-still-have-live-callers.md`, which 0304 slice 5 pays.

## What should happen

Each edge the release QA suite reports is a release-QA finding. For each one, the queue owner either fixes the specification's "no calls" mark and wording, or files the build bug if the build is wrong. The specification stays the contract that both this repository's gate and the release QA suite read. This issue closes when the suite's current list is fed back.

## Evidence

The release QA suite's edge list, held privately by its owner. It is not copied here.
