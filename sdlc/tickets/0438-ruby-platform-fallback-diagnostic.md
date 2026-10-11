# 0438: Replace Ruby’s unsupported-platform placeholder route

Status: COMPLETE.

Opened as: 2026-10-11. Matching-version diagnostic fallback and release packaging reviewed and checked.

Milestone: 0.2

Owner: builder.
Ticket review: accepted 2026-10-06; blocking findings corrected.

## Outcome

Supported Ruby consumers receive a real native package. Unsupported Ruby/platform consumers receive an explicit diagnostic instead of silently installing the old 0.0.1 placeholder.

## Evidence

- Starts from: Issue 2026-10-03-rubygems-ruby-platform-gem-is-the-0-0-1-placeholder.md; 0394 Darwin correction. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 8.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Plan a matching-version plain ruby fallback package with a clear install/load diagnostic. Preserve supported platform gems, Ruby version bounds and macOS floor. Set the fallback metadata so unsupported runtimes can receive the diagnostic rather than falling back to the historical placeholder.
- Proof: Use local gem selection/install consumers for supported native platforms and unsupported Ruby/platform combinations. Verify no path reports the placeholder as a working current package. Keep native import and package contents checks.
- Defers: Proxy service/screens, images, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

0394’s Darwin-platform correction is retained evidence, not this fallback fix. No registry yanking, publication or credential/settings change belongs to implementation without existing release authorization.
