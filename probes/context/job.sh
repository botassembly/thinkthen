#!/bin/sh
# Run only after the ticket's capped live call is authorized.
# sdlc/scripts/live --max-tokens 150000 probes/context/job.sh BENCH NAME
exec python3 probes/context/measure.py "$@"
