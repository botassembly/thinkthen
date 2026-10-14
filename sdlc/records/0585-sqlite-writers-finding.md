# 0585: Measure concurrent SQLite answer writers

Ticket: 0585
Date: 2026-10-14

Review: accept. A fresh read-only reviewer independently recomputed percentiles, acknowledgment counts, sequence and payload checks, integrity, foreign keys, retries, WAL growth and reader results. The reviewer verified the deliberate stop exceptions and the partial-volume/shared-load limits. No findings required changes.

Source revision: 0e46dc2daf53bba7ad40165e67068948607f7bec. Beelink only. No product edits, paid calls, model jobs, publication, or changes to other teams' output.

## Method and evidence

Read the ticket, brief, current store implementation, marketing database PRD concurrency clause, prior experiment 253, and existing lock-test record before running. Current product uses journal_mode DELETE, synchronous FULL, secure_delete ON, and busy_timeout zero. The proposed WAL setting is an experimental change. Scripts reproduce the current explicit BEGIN IMMEDIATE transaction and busy retry schedule: 2, 4, 8, 16, 32, 64, then 100 milliseconds, capped at 30 seconds for each statement. A synthetic reply has one answer. Idle I/O priority and nice 15 protect concurrent release work; these are shared-host measurements, without a quiet baseline.

Python standard-library sqlite3 links SQLite 3.45.1 here. The product's bundled SQLite and extension host versions have not been qualified by this probe. We did not build or call the SQL extensions, because current extensions configure a cache folder rather than the proposed named answer database. Four additional independent processes are the ticket's permitted fallback.

run.py ran a small-table pilot with eight writers committing 1,000 rows each, followed by four writers committing 1,000 each. Two short readers issued indexed point lookups; a third reader retained the initial empty snapshot across every write. It wrote 12,000 of 12,000 rows, without writer or reader errors or timeouts. Separate verification visited every committed key, checked its exact synthetic payload, checked per-writer sequence counts/minimum/maximum/sum, and obtained integrity_check=ok. Commit logs and CSV each contain exactly 12,000 successes. Transaction p50 was 4.300ms, p99 96.573ms and maximum 10,254.813ms. Lock acquisition p50 was 0.013ms, p99 62.631ms and maximum 10,246.776ms. There were 3,429 busy retries, all eventually successful. Peak sampled WAL was 109,175,912 bytes while the held reader continued to see zero rows. Host one-minute load ranged from 9.86 to 12.38. This pilot passes the requested p99 bound at its small volume but does not establish the full result.

run_full.py uses the draft relational schema copied from experiment 480, baseline.sql and changes.sql, with JSON_TYPE rendered TEXT. One transaction creates evidence, call, observation, answer, answer_observation and call_answer records; foreign keys are enabled. Shared question, wording, actor and context exist before timing. Eight writers request 100,000 replies each, and four more request 100,000 each after two seconds. Two short readers and one long-snapshot reader continue through the run. Raw timings retain every successful reply and lock acquisition, worker JSON preserves error traces, host-wal.jsonl preserves host load and WAL growth. verify.py checks acknowledgments and payloads independently from worker loops. Detailed results are in summary.json.

## Boundaries

No quiet-host result, process crash or power-loss simulation, real network reply timing, mixed payload-size distribution, updater contention, or extension execution is claimed. CPU and I/O priority deliberately make this a conservative shared-host observation; it cannot distinguish SQLite capacity from this host's storage scheduling. The full target is bounded by the first timeout/error, 600 seconds, 14GiB WAL, or 50GiB free disk. The held snapshot is deliberate adverse behavior; this is not an unbounded workload.

All scripts, copied schema, per-write CSV, worker results, raw WAL/load samples, database files, manifests, and verification results remain in this owned folder while ticket 0585 needs its bytes. No evidence or scratch has been deleted.

## Final full-schema result and decision

The coordinator shortened the run after the observed p99 crossed the one-second criterion. The full-schema attempt ended at 206.281 seconds with 33,795 committed replies, representing 2.82% of the 1,200,000-reply target. This is a partial-volume performance failure under the measured shared load, not a completed full-volume test. Across successful replies, transaction p50 was 10.867ms, p99 1,344.688ms and maximum 6,141.599ms. Lock-acquisition p50 was 0.027ms, p99 1,333.613ms and maximum 6,133.697ms. The p99 is primarily waiting to acquire the write lock. Workers counted 57,777 successful busy retries. There were zero natural SQLite timeouts, zero unexpected storage failures, zero missing acknowledged keys, and zero wrong acknowledged payloads.

Independent verification found 33,795 rows in each of the six per-reply tables. All worker acknowledged counts and CSV rows total 33,795. Each writer's committed sequence is contiguous from zero, and per-writer counts/minimum/maximum/sums agree. integrity_check returned ok; foreign_key_check returned no violations. All twelve writer processes exited normally. Two short readers completed 19,409 and 19,423 queries without errors. The held reader completed 19,503 queries, retained its zero-row snapshot throughout, and rolled back normally. One-minute host load ranged from 8.38 to 15.96 on the sixteen-logical-CPU machine. Idle I/O scheduling and nice 15 remained in effect.

The largest sampled WAL was 2,366,144,872 bytes (about 2.20GiB), despite the final database being only 20,815,872 bytes. This is evidence that a long-lived snapshot can retain large per-transaction WAL traffic. All held readers and writer connections released their locks at shutdown. At the first post-run checkpoint observation, the WAL was already zero bytes, and PRAGMA wal_checkpoint(TRUNCATE) returned (0,0,0). This establishes successful release/checkpoint in this attempt; it does not time the exact automatic checkpoint. checkpoint.json records the observation.

### Stop-control detail

The launcher originally supported internal deadline/error/disk guards but omitted an external stop-file hook. To obey the coordinator's early-stop instruction without terminating processes or dropping buffered acknowledgments, a separate owned connection committed a BEFORE INSERT trigger that raises ABORT with the exact coordinator-stop reason. The next worker exception set the shared stop event. Eleven writers already entering or waiting for another reply reached this deliberate exception; one noticed the event before starting another reply. These eleven unacknowledged control-aborted replies are preserved in worker tracebacks. They are not SQLite busy timeouts or unexpected storage failures. No new successful timing samples were added for aborted replies. Natural stopping/censoring of outstanding attempts means tail statistics describe completed replies. Future reruns should add an explicit stop-file check and rollback on the stop event. The original scripts and control action remain unchanged as evidence.

### Recommendation

SQLite WAL remains feasible for 0.3 on the observed correctness evidence: independent processes committed six linked records per reply without losing acknowledged answers, and readers did not block those writes. The experiment does not prove the requested full-volume performance or justify a one-second p99 promise. Keep that promise unapproved until the actual product SQLite version, intended payloads, and a quieter host are measured. Consider grouping writes or managing write admission if twelve continuously ready writers are a real workload, but this run did not compare those designs and does not establish that grouping is required. Bound or close long notebook snapshots to prevent sustained WAL retention. No direct recommendation for the 0.2 release follows because its current store uses DELETE journaling and a different schema.

## Evidence retention

The experiment folder is `experiments/481-thinkthen-writers-0585`. The coordinator reviewed its raw results before recording this finding. After acceptance and landing, owned generated inputs, databases, packages and scratch can be removed. The observed results and stated limits remain in this record.

## Recorded measurements

```json
{
  "pilot": {
    "result": {
      "stop_reason": "complete",
      "elapsed_s": 71.99421968962997,
      "actual": 12000,
      "groups": [
        [
          0,
          1000,
          0,
          999,
          499500
        ],
        [
          1,
          1000,
          0,
          999,
          499500
        ],
        [
          2,
          1000,
          0,
          999,
          499500
        ],
        [
          3,
          1000,
          0,
          999,
          499500
        ],
        [
          4,
          1000,
          0,
          999,
          499500
        ],
        [
          5,
          1000,
          0,
          999,
          499500
        ],
        [
          6,
          1000,
          0,
          999,
          499500
        ],
        [
          7,
          1000,
          0,
          999,
          499500
        ],
        [
          8,
          1000,
          0,
          999,
          499500
        ],
        [
          9,
          1000,
          0,
          999,
          499500
        ],
        [
          10,
          1000,
          0,
          999,
          499500
        ],
        [
          11,
          1000,
          0,
          999,
          499500
        ]
      ],
      "bad_payloads": 0,
      "integrity": [
        [
          "ok"
        ]
      ],
      "load_end": [
        9.8623046875,
        10.62060546875,
        9.07421875
      ],
      "writer_exits": [
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0
      ]
    },
    "committed_from_worker_logs": 12000,
    "csv_rows": 12000,
    "missing_acknowledged_keys": 0,
    "bad_acknowledged_payloads": 0,
    "writer_errors": [],
    "busy_retries": 3429,
    "transaction": {
      "n": 12000,
      "p50_ms": 4.300439730286598,
      "p99_ms": 96.5732904151082,
      "max_ms": 10254.813305102289
    },
    "lock": {
      "n": 12000,
      "p50_ms": 0.0125151127576828,
      "p99_ms": 62.6310957595706,
      "max_ms": 10246.77597451955
    },
    "max_wal_bytes": 109175912,
    "load1_range": [
      9.8623046875,
      12.3837890625
    ],
    "readers": [
      {
        "reader": 0,
        "held": false,
        "queries": 7085,
        "errors": []
      },
      {
        "reader": 1,
        "held": false,
        "queries": 7089,
        "errors": []
      },
      {
        "reader": 2,
        "held": true,
        "queries": 7110,
        "errors": [],
        "snapshot_count": 0,
        "snapshot_count_at_end": 0
      }
    ]
  },
  "full": {
    "result": {
      "stop_reason": "worker error",
      "elapsed_s": 206.28096731938422,
      "actual": 33795,
      "table_counts": {
        "evidence": 33795,
        "call": 33795,
        "observation": 33795,
        "answer": 33795,
        "answer_observation": 33795,
        "call_answer": 33795
      },
      "foreign_key_check": [],
      "groups": [
        [
          "0",
          3410,
          0,
          3409,
          5812345
        ],
        [
          "1",
          2869,
          0,
          2868,
          4114146
        ],
        [
          "10",
          2717,
          0,
          2716,
          3689686
        ],
        [
          "11",
          2804,
          0,
          2803,
          3929806
        ],
        [
          "2",
          2839,
          0,
          2838,
          4028541
        ],
        [
          "3",
          2488,
          0,
          2487,
          3093828
        ],
        [
          "4",
          2647,
          0,
          2646,
          3501981
        ],
        [
          "5",
          2972,
          0,
          2971,
          4414906
        ],
        [
          "6",
          2601,
          0,
          2600,
          3381300
        ],
        [
          "7",
          3022,
          0,
          3021,
          4564731
        ],
        [
          "8",
          2976,
          0,
          2975,
          4426800
        ],
        [
          "9",
          2450,
          0,
          2449,
          3000025
        ]
      ],
      "bad_payloads": 0,
      "integrity": [
        [
          "ok"
        ]
      ],
      "load_end": [
        14.31103515625,
        12.18115234375,
        9.96142578125
      ],
      "writer_exits": [
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0
      ]
    },
    "committed_from_worker_logs": 33795,
    "csv_rows": 33795,
    "missing_acknowledged_keys": 0,
    "bad_acknowledged_payloads": 0,
    "writer_errors": [
      {
        "writer": 0,
        "requested": 100000,
        "committed": 3410,
        "retries": 4949,
        "error": "Traceback (most recent call last):\n  File \"/home/ian/workspace/experiments/481-thinkthen-writers-0585/run_full.py\", line 41, in writer\n    _,rr,ss=wait(c,sql,args);r2+=rr;s2+=ss\n            ^^^^^^^^^^^^^^^^\n  File \"/home/ian/workspace/experiments/481-thinkthen-writers-0585/run_full.py\", line 8, in wait\n    try:return c.execute(sql,args),retries,slept\n               ^^^^^^^^^^^^^^^^^^^\nsqlite3.IntegrityError: coordinator requested stop after measured p99 exceeded 1 second\n",
        "settings": {
          "journal_mode": "wal",
          "synchronous": 2,
          "secure_delete": 1,
          "busy_timeout": 0,
          "wal_autocheckpoint": 1000,
          "page_size": 4096
        }
      },
      {
        "writer": 1,
        "requested": 100000,
        "committed": 2869,
        "retries": 4725,
        "error": "Traceback (most recent call last):\n  File \"/home/ian/workspace/experiments/481-thinkthen-writers-0585/run_full.py\", line 41, in writer\n    _,rr,ss=wait(c,sql,args);r2+=rr;s2+=ss\n            ^^^^^^^^^^^^^^^^\n  File \"/home/ian/workspace/experiments/481-thinkthen-writers-0585/run_full.py\", line 8, in wait\n    try:return c.execute(sql,args),retries,slept\n               ^^^^^^^^^^^^^^^^^^^\nsqlite3.IntegrityError: coordinator requested stop after measured p99 exceeded 1 second\n",
        "settings": {
          "journal_mode": "wal",
          "synchronous": 2,
          "secure_delete": 1,
          "busy_timeout": 0,
          "wal_autocheckpoint": 1000,
          "page_size": 4096
        }
      },
      {
        "writer": 10,
        "requested": 100000,
        "committed": 2717,
        "retries": 4762,
        "error": "Traceback (most recent call last):\n  File \"/home/ian/workspace/experiments/481-thinkthen-writers-0585/run_full.py\", line 41, in writer\n    _,rr,ss=wait(c,sql,args);r2+=rr;s2+=ss\n            ^^^^^^^^^^^^^^^^\n  File \"/home/ian/workspace/experiments/481-thinkthen-writers-0585/run_full.py\", line 8, in wait\n    try:return c.execute(sql,args),retries,slept\n               ^^^^^^^^^^^^^^^^^^^\nsqlite3.IntegrityError: coordinator requested stop after measured p99 exceeded 1 second\n",
        "settings": {
          "journal_mode": "wal",
          "synchronous": 2,
          "secure_delete": 1,
          "busy_timeout": 0,
          "wal_autocheckpoint": 1000,
          "page_size": 4096
        }
      },
      {
        "writer": 11,
        "requested": 100000,
        "committed": 2804,
        "retries": 4992,
        "error": "Traceback (most recent call last):\n  File \"/home/ian/workspace/experiments/481-thinkthen-writers-0585/run_full.py\", line 41, in writer\n    _,rr,ss=wait(c,sql,args);r2+=rr;s2+=ss\n            ^^^^^^^^^^^^^^^^\n  File \"/home/ian/workspace/experiments/481-thinkthen-writers-0585/run_full.py\", line 8, in wait\n    try:return c.execute(sql,args),retries,slept\n               ^^^^^^^^^^^^^^^^^^^\nsqlite3.IntegrityError: coordinator requested stop after measured p99 exceeded 1 second\n",
        "settings": {
          "journal_mode": "wal",
          "synchronous": 2,
          "secure_delete": 1,
          "busy_timeout": 0,
          "wal_autocheckpoint": 1000,
          "page_size": 4096
        }
      },
      {
        "writer": 2,
        "requested": 100000,
        "committed": 2839,
        "retries": 4843,
        "error": "Traceback (most recent call last):\n  File \"/home/ian/workspace/experiments/481-thinkthen-writers-0585/run_full.py\", line 41, in writer\n    _,rr,ss=wait(c,sql,args);r2+=rr;s2+=ss\n            ^^^^^^^^^^^^^^^^\n  File \"/home/ian/workspace/experiments/481-thinkthen-writers-0585/run_full.py\", line 8, in wait\n    try:return c.execute(sql,args),retries,slept\n               ^^^^^^^^^^^^^^^^^^^\nsqlite3.IntegrityError: coordinator requested stop after measured p99 exceeded 1 second\n",
        "settings": {
          "journal_mode": "wal",
          "synchronous": 2,
          "secure_delete": 1,
          "busy_timeout": 0,
          "wal_autocheckpoint": 1000,
          "page_size": 4096
        }
      },
      {
        "writer": 3,
        "requested": 100000,
        "committed": 2488,
        "retries": 4601,
        "error": "Traceback (most recent call last):\n  File \"/home/ian/workspace/experiments/481-thinkthen-writers-0585/run_full.py\", line 41, in writer\n    _,rr,ss=wait(c,sql,args);r2+=rr;s2+=ss\n            ^^^^^^^^^^^^^^^^\n  File \"/home/ian/workspace/experiments/481-thinkthen-writers-0585/run_full.py\", line 8, in wait\n    try:return c.execute(sql,args),retries,slept\n               ^^^^^^^^^^^^^^^^^^^\nsqlite3.IntegrityError: coordinator requested stop after measured p99 exceeded 1 second\n",
        "settings": {
          "journal_mode": "wal",
          "synchronous": 2,
          "secure_delete": 1,
          "busy_timeout": 0,
          "wal_autocheckpoint": 1000,
          "page_size": 4096
        }
      },
      {
        "writer": 5,
        "requested": 100000,
        "committed": 2972,
        "retries": 5003,
        "error": "Traceback (most recent call last):\n  File \"/home/ian/workspace/experiments/481-thinkthen-writers-0585/run_full.py\", line 41, in writer\n    _,rr,ss=wait(c,sql,args);r2+=rr;s2+=ss\n            ^^^^^^^^^^^^^^^^\n  File \"/home/ian/workspace/experiments/481-thinkthen-writers-0585/run_full.py\", line 8, in wait\n    try:return c.execute(sql,args),retries,slept\n               ^^^^^^^^^^^^^^^^^^^\nsqlite3.IntegrityError: coordinator requested stop after measured p99 exceeded 1 second\n",
        "settings": {
          "journal_mode": "wal",
          "synchronous": 2,
          "secure_delete": 1,
          "busy_timeout": 0,
          "wal_autocheckpoint": 1000,
          "page_size": 4096
        }
      },
      {
        "writer": 6,
        "requested": 100000,
        "committed": 2601,
        "retries": 4871,
        "error": "Traceback (most recent call last):\n  File \"/home/ian/workspace/experiments/481-thinkthen-writers-0585/run_full.py\", line 41, in writer\n    _,rr,ss=wait(c,sql,args);r2+=rr;s2+=ss\n            ^^^^^^^^^^^^^^^^\n  File \"/home/ian/workspace/experiments/481-thinkthen-writers-0585/run_full.py\", line 8, in wait\n    try:return c.execute(sql,args),retries,slept\n               ^^^^^^^^^^^^^^^^^^^\nsqlite3.IntegrityError: coordinator requested stop after measured p99 exceeded 1 second\n",
        "settings": {
          "journal_mode": "wal",
          "synchronous": 2,
          "secure_delete": 1,
          "busy_timeout": 0,
          "wal_autocheckpoint": 1000,
          "page_size": 4096
        }
      },
      {
        "writer": 7,
        "requested": 100000,
        "committed": 3022,
        "retries": 4876,
        "error": "Traceback (most recent call last):\n  File \"/home/ian/workspace/experiments/481-thinkthen-writers-0585/run_full.py\", line 41, in writer\n    _,rr,ss=wait(c,sql,args);r2+=rr;s2+=ss\n            ^^^^^^^^^^^^^^^^\n  File \"/home/ian/workspace/experiments/481-thinkthen-writers-0585/run_full.py\", line 8, in wait\n    try:return c.execute(sql,args),retries,slept\n               ^^^^^^^^^^^^^^^^^^^\nsqlite3.IntegrityError: coordinator requested stop after measured p99 exceeded 1 second\n",
        "settings": {
          "journal_mode": "wal",
          "synchronous": 2,
          "secure_delete": 1,
          "busy_timeout": 0,
          "wal_autocheckpoint": 1000,
          "page_size": 4096
        }
      },
      {
        "writer": 8,
        "requested": 100000,
        "committed": 2976,
        "retries": 4698,
        "error": "Traceback (most recent call last):\n  File \"/home/ian/workspace/experiments/481-thinkthen-writers-0585/run_full.py\", line 41, in writer\n    _,rr,ss=wait(c,sql,args);r2+=rr;s2+=ss\n            ^^^^^^^^^^^^^^^^\n  File \"/home/ian/workspace/experiments/481-thinkthen-writers-0585/run_full.py\", line 8, in wait\n    try:return c.execute(sql,args),retries,slept\n               ^^^^^^^^^^^^^^^^^^^\nsqlite3.IntegrityError: coordinator requested stop after measured p99 exceeded 1 second\n",
        "settings": {
          "journal_mode": "wal",
          "synchronous": 2,
          "secure_delete": 1,
          "busy_timeout": 0,
          "wal_autocheckpoint": 1000,
          "page_size": 4096
        }
      },
      {
        "writer": 9,
        "requested": 100000,
        "committed": 2450,
        "retries": 4617,
        "error": "Traceback (most recent call last):\n  File \"/home/ian/workspace/experiments/481-thinkthen-writers-0585/run_full.py\", line 41, in writer\n    _,rr,ss=wait(c,sql,args);r2+=rr;s2+=ss\n            ^^^^^^^^^^^^^^^^\n  File \"/home/ian/workspace/experiments/481-thinkthen-writers-0585/run_full.py\", line 8, in wait\n    try:return c.execute(sql,args),retries,slept\n               ^^^^^^^^^^^^^^^^^^^\nsqlite3.IntegrityError: coordinator requested stop after measured p99 exceeded 1 second\n",
        "settings": {
          "journal_mode": "wal",
          "synchronous": 2,
          "secure_delete": 1,
          "busy_timeout": 0,
          "wal_autocheckpoint": 1000,
          "page_size": 4096
        }
      }
    ],
    "busy_retries": 57777,
    "transaction": {
      "n": 33795,
      "p50_ms": 10.866954922676086,
      "p99_ms": 1344.6875736117363,
      "max_ms": 6141.598805785179
    },
    "lock": {
      "n": 33795,
      "p50_ms": 0.026901252567768097,
      "p99_ms": 1333.612758666277,
      "max_ms": 6133.697173558176
    },
    "max_wal_bytes": 2366144872,
    "load1_range": [
      8.38427734375,
      15.95947265625
    ],
    "readers": [
      {
        "reader": 0,
        "held": false,
        "queries": 19409,
        "errors": []
      },
      {
        "reader": 1,
        "held": false,
        "queries": 19423,
        "errors": []
      },
      {
        "reader": 2,
        "held": true,
        "queries": 19503,
        "errors": [],
        "snapshot_count": 0,
        "snapshot_count_at_end": 0
      }
    ]
  }
}
```
