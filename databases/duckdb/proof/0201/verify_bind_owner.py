"""Bounded stock-host bind and statement-owner checks for the 0201 proof."""

import argparse
from pathlib import Path

import duckdb


def state(value: str) -> tuple[str, int, int, int, int, int]:
    phase, *numbers = value.split(":")
    assert phase in {"first", "begin"}, value
    return phase, *(int(number) for number in numbers)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--extension", type=Path, required=True)
    args = parser.parse_args()
    connection = duckdb.connect(config={"allow_unsigned_extensions": "true"})
    path = str(args.extension.resolve()).replace("'", "''")
    connection.execute(f"LOAD '{path}'")

    first = state(connection.execute("SELECT thinkthen_probe_owner('first')").fetchone()[0])
    assert first[:3] == ("first", 1, 1), first
    before = connection.execute("SELECT thinkthen_probe_count()").fetchone()[0]
    assert before == 0, before
    try:
        connection.execute("PREPARE bad AS SELECT thinkthen_probe_call('bad', 'x')")
    except duckdb.InvalidInputException as error:
        assert "thinkthen usage: bad constant question" in str(error), error
    else:
        raise AssertionError("ordinary bad constant reached execution")
    assert connection.execute("SELECT thinkthen_probe_count()").fetchone()[0] == 0

    connection.execute("PREPARE choices AS SELECT 0 AS i, thinkthen_probe_try('bad', 'x') AS value "
                       "UNION ALL SELECT 1, thinkthen_probe_try('ok', 'y') ORDER BY i")
    choices = connection.execute("EXECUTE choices").fetchall()
    assert choices[0] == (0, "failed:usage"), choices
    assert choices[1][0] == 1 and choices[1][1].startswith("answered:begin:"), choices
    assert choices[1][1].endswith(":y"), choices
    assert connection.execute("SELECT thinkthen_probe_count()").fetchone()[0] == 1

    both = connection.execute("SELECT thinkthen_probe_owner('a'), thinkthen_probe_owner('b')").fetchone()
    left, right = (state(value) for value in both)
    assert left[0] == right[0] == "begin" and left[1] == right[1], both
    assert {left[2], right[2]} == {1, 2}, both
    connection.execute("PREPARE owner AS SELECT thinkthen_probe_owner('prepared')")
    repeated = [state(connection.execute("EXECUTE owner").fetchone()[0]) for _ in range(2)]
    assert all(one[0] == "begin" and one[2] == 1 for one in repeated), repeated
    assert repeated[1][1] == repeated[0][1] + 1, repeated

    try:
        connection.execute("SELECT thinkthen_probe_call(q, 'x') FROM (VALUES ('bad')) t(q)")
    except duckdb.InvalidInputException as error:
        assert "thinkthen usage: bad question" in str(error), error
    else:
        raise AssertionError("bad row did not fail")
    recovered = state(connection.execute("SELECT thinkthen_probe_owner('after failure')").fetchone()[0])
    assert recovered[0] == "begin" and recovered[2] == 1, recovered
    assert recovered[4] == repeated[1][4] + 1, (repeated, recovered)

    before_signal = recovered[5]
    connection.execute("SELECT thinkthen_probe_signal(thinkthen_probe_owner('late'))").fetchone()
    after_signal = state(connection.execute("SELECT thinkthen_probe_owner('after signal')").fetchone()[0])
    assert after_signal[5] == before_signal + 1, (before_signal, after_signal)

    before_chunk = connection.execute("SELECT thinkthen_probe_count()").fetchone()[0]
    chunk = connection.execute(
        "SELECT thinkthen_probe_owner(i::VARCHAR) FROM range(2049) t(i)"
    ).fetchall()
    observed = [state(row[0]) for row in chunk]
    assert len(observed) == 2049 and len({row[1] for row in observed}) == 1
    assert sorted(row[2] for row in observed) == list(range(1, 2050))
    assert connection.execute("SELECT thinkthen_probe_count()").fetchone()[0] == before_chunk
    print("stock Python: bind zero-send, try row recovery, owner lifecycle, synthetic late signal, and two chunks passed")


if __name__ == "__main__":
    main()
