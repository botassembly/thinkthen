#!/bin/sh
# Run only through sdlc/scripts/live after independent review of the offline plan.
exec python3 probes/annotate-batching/measure.py "$@"
