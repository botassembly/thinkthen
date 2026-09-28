# Libraries cannot replay a recording strictly

Status: Open. Filed 2026-09-26 by the marketing session while building smoke tests for the website's code examples.

## What happens

The command has `--replay DIR`. A request with no saved reply fails with a replay miss and sends nothing. The website's command examples use it, so every command example runs as an offline test with no key.

The public engine has no such setting. `EngineBuilder` offers `cache_at`, `no_cache`, and `cache_bytes` (`crates/thinkthen/src/public/settings.rs:191` to `:213`). `cache_at` sets the same folder as both the recording and the replay folder (`settings.rs`, `Storage { record: Some(folder), replay: Some(folder), .. }`). A cache miss then sends a live request. `EngineError::ReplayMiss` exists (`public/error.rs:186`), but no public setter reaches a replay-only mode. None of the Python, TypeScript, Ruby, Rust, C, Polars, pandas, or R libraries has a replay option.

## Why it matters

The website shows every function in every language. The language examples cannot run as offline tests. A test that points a library at a recording folder would send a paid request on any miss, and a missing key would fail it for the wrong reason. The site keeps those examples on a skip list today, so they can drift from the library without anyone noticing.

## What is asked

A replay-only setting on the public engine that matches `--replay`: answers come only from the named folder, and a miss is an error that sends nothing. Each library and database extension exposes it through its own settings, in the same shape it exposes the cache. The command's recording folder format stays the one format, so one recording serves the command and every library.

The design belongs to this repository. Ian can overturn the request.

## Partial resolution, 2026-09-27

Ticket 0148 settles strict replay for Rust, Python, TypeScript, Ruby, R, C and their frame adapters. SQL replay remains for0149; this issue stays open. See `sdlc/records/0148-engine-settings-everywhere.md`.
