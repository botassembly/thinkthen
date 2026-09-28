#!/bin/sh
# Run only through sdlc/scripts/live after reviewed source and a named reservation.
exec python3 probes/tag-score/measure.py "$@"
