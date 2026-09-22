# 0058: Make diagnostics actionable and safe

Date: 2026-09-21

Status: landed

## Result

Common HTTP and transport failures now carry fixed tool-owned guidance. The engine reduces structured HTTP and I/O errors to five safe kinds before the command formats them. It never parses or prints library, operating-system, address, credential, evidence, or backend-body text.

Question-file and question-set errors name the actual key or missing wrapper. Empty evidence, singular table fields, probability tolerance, recording paths that are files, stopped-run recording counts, and the `set -e` warning now use direct wording. A dry run applies the live one-document width rule before it prints a plan. Request bytes, retry policy, timeout duration, successful output, result shapes, exit codes, and dependencies did not change.

The source ratchet rose from 26,179 to 26,545 nonblank Rust lines. The structured boundary and focused behavior and secrecy tests account for the growth.

## Review and proof

Independent design review rejected the first draft for an inconsistent secrecy promise, a false zero-retry sentence, an underspecified transport mapping, and missing question-set precedence. The repaired ticket fixes complete public lines, maps structured error variants, safely escapes the one local key value, and makes the missing wrapper win. The same reviewer accepted it.

Independent code review rejected the first implementation because `find` omitted recording counts from a stopped framed run even when the user named a recording. The repair carries that state through preflight. Exact tests cover both named and unnamed recordings. The same reviewer accepted the final implementation with no remaining findings.

The final focused suites passed 177 library tests, 216 backend tests, 18 question-file tests, 18 decide edge tests, and 14 find edge tests. `install`, `lint`, `test`, and `spec` exited zero. The specification rung passed 26 command examples, seven transform examples, all replay checks, and 19 green how-tos. `git diff --check` passed. No paid or outside network call ran.
