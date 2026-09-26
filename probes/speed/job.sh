#!/bin/sh
# The speed test's live job (ticket 0145). Run it only with Ian's authorization:
#   sdlc/scripts/live --max-tokens 2000000 probes/speed/job.sh BENCH NAME
exec python3 probes/speed/measure.py live "$@"
