"""Bound SQL consumption and rank source retention through the installed host."""
import json
from pathlib import Path
import re
import subprocess
import sys

from migration_cases import child_environment, command, fixture


def rank_memory(package):
    # Feature: rank retains native source columns within a bounded resident heap.
    if sys.platform != 'win32':
        result = subprocess.run([str(package / 'sql'), str(package / 'thinkthen.duckdb_extension'), '--rank-memory'],
                                env=child_environment(), capture_output=True, text=True)
        assert result.returncode == 0, (result.stdout, result.stderr)
        observation = json.loads(result.stdout)
        assert observation['rows'] == 100000 and observation['misplaced'] == 0, observation
        print('DuckDB rank source retention:', observation, flush=True)
        assert observation['growth_bytes'] < 128 * 1024 * 1024, observation


def first_response(host_type, loopback):
    # Opt-in diagnostic: measure the pinned host's streaming LIMIT behavior.
    engine = Path(__file__).resolve().parents[3] / 'crates/thinkthen/src/engine.rs'
    workers = int(re.search(r'^const WORKERS: usize = (\d+);$', engine.read_text(), re.M)[1])
    marker = 'first-visible-source'
    with fixture(loopback, '--first-response', marker) as (server, url), host_type(url) as host:
        host.configure({'backend': 'openai', 'batch': 32})
        # Reduce the native caller buffer; the host pipeline can still need
        # another chunk before Fetch returns.
        host.query("SET streaming_buffer_size='1B'")
        rows = host.query({'first': "SELECT document_id,position,status FROM thinkthen_decide((SELECT range AS document_id, CASE WHEN range=0 THEN '" + marker + "' ELSE range::VARCHAR END AS record FROM range(8192)), 'Ready?') LIMIT 10"})
        assert len(rows) == 10 and all(row['status'] == 'answered' for row in rows), rows
        assert [int(row['document_id']) for row in rows] == list(range(10)), rows
        assert [int(row['position']) for row in rows] == list(range(1, 11)), rows
        stats = json.loads(command(server, 'stats'))
        print('DuckDB streaming LIMIT diagnostic:', stats, flush=True)
        assert 1 <= stats['requests'] <= workers + 1, stats
        assert len(stats['request_sizes']) == stats['requests'], stats
        # Other requests stay held until the caller closes them, including on
        # its own timeout. DuckDB 1.4.3 may wait for timeout-generated chunks.
        # These assertions prove rows and request counts, not response latency.


def streaming(host_type, package, loopback):
    rank_memory(package)
