#!/usr/bin/python3 -IS
"""Coordinator-only ambient named-key bridge for the reviewed manual checks."""
import os
from pathlib import Path
import sys
import tempfile

SELECTED = {"perplexity": "PERPLEXITY_API_KEY", "openrouter": "OPENROUTER_API_KEY"}
if len(sys.argv) != 2 or sys.argv[1] not in SELECTED:
    sys.exit("0399: name perplexity or openrouter")
name = sys.argv[1]
variable = SELECTED[name]
key = os.environ.get(variable, "")
if not key.strip():
    sys.exit("0399: selected named key variable is unset or blank")
repo = Path(__file__).resolve().parents[2]
# Fresh owner-only configuration prevents a pre-existing custom backend name.
# The coordinator retains this scratch directory with the run receipts.
scratch = Path(tempfile.mkdtemp(prefix=f"thinkthen-0399-{name}-"))
environment = {"PATH": "/usr/bin:/bin", "LC_ALL": "C", variable: key,
               "THINKTHEN_API_KEY": key, "HOME": str(scratch),
               "XDG_CONFIG_HOME": str(scratch / "config"),
               "XDG_CACHE_HOME": str(scratch / "cache"),
               "XDG_STATE_HOME": str(scratch / "state")}
wrapper = repo / "sdlc/scripts/live"
job = repo / "sdlc/manual/0399" / f"{name}.sh"
os.execve(wrapper, [str(wrapper), "--max-tokens", "10000", str(job)], environment)
