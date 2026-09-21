#!/bin/sh
set -eu

jq -c -f transforms/triage/triage.jq < annotated-tickets.jsonl
