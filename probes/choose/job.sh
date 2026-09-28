#!/bin/sh
# Run only through sdlc/scripts/live after reviewed source and a named budget.
exec python3 probes/choose/measure.py "$@"
