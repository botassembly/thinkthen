# Record the backend's own time for each call

Status: Open. Checked 2026-09-25: no backend time is recorded anywhere in `crates/` or `specification/`.

Found in Beatles Bench, 2026-09-23. A benchmark needs to split each call's time into the model's work, the network, and our command. Today a run keeps none of it. The bench times the whole `thinkthen` process from outside, and those times were taken on a heavily loaded machine.

## What the backends send

- **Jev.** The reply body has no timing field. The headers carry `x-envoy-upstream-service-time`, the server's time in milliseconds, and `x-typesafe-request-id`. Three calls on 2026-09-23 gave server times of 762, 997, and 65 ms, with about 140 ms of network on top of each. The full header set was date, content-type, content-length, server, x-typesafe-request-id, x-envoy-upstream-service-time, cf-cache-status, nel, report-to, and cf-ray.
- **Chat backends (Z.ai GLM).** The body carries `created`, the finish time in whole seconds. The request id begins with the arrival time. Their difference bounds the server time to the second.

## The ask

Ian's ruling, 2026-09-23: keep the times. Harvest only the fields we map, never the whole header set. Each adapter names its map:

| Adapter | Source | Kept as |
|---|---|---|
| systemone | header `x-envoy-upstream-service-time` | `server_ms` |
| systemone | header `x-typesafe-request-id` | `request_id` |
| chat | body `created`, and the time in the body `id` | `server_s`, whole seconds |
| every adapter | the command's own clock | `wall_ms` |
| every run | the command's own clock, from start to exit, minus the exchanges | `command_ms` |

A backend with no timing header keeps `wall_ms` and `command_ms` and leaves the rest out. The Laya shim is ours, so it can send `x-envoy-upstream-service-time` too. A new backend adds its row to the map. With all four, a user splits every answer into the model's time, the network's, and ThinkThen's own.

For every sent request, keep three facts: the wall time of the exchange as the command measured it, the backend's own time when a known header names it, and the backend's request id. Read only named headers from a fixed allowlist. Never keep any other header. Show them under `--details`.

## The constraint

`recording.md` says an entry holds bodies and never headers, and one exchange writes the same file every time. Times differ on every call. So the times belong beside the recording, in a separate file keyed by the entry's digest, and never inside the entry. A replay reads the entry and leaves the times alone.

## Why

The Beatles Bench paper compares Jev's median of 0.30 s with GLM's 8.4 s. A reader will ask how much is the model and how much is the network and the machine. With these fields, every run answers that for itself.
