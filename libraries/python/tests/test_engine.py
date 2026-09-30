"""``tt.Engine`` and its ADR 0017 section 5 settings (amendment changes 11
to 13). The throttle is one per process, so each engine lives in its own
child with its own backend and cache folder."""

import time

from conftest import Backend, child_env, run, start

SETTINGS = """
    import os, thinkthen as tt
    late = tt.question(decide="Is it late?")
    def said(verb="decide", **settings):
        try:
            asked = late if verb == "decide" else "Which is late?"
            getattr(tt.Engine(**settings), verb)(asked, ["one", "two", "three"])
        except tt.ThinkThenError as error:
            print(type(error).__name__, error)
"""


def test_held_request_does_not_pull_ahead(backend, tmp_path):
    """A held reply keeps the planner from pulling the next request;
    release still lets all 20 records finish."""
    child = start("""
        import thinkthen as tt
        tt.Engine(throttle=8, batch=1, cache=False).decide(
            tt.question(decide="Is it late?"), [f"note {n}" for n in range(20)]).value
    """, child_env(backend, tmp_path, "arm/held"))
    assert backend.wait(1) == 1
    time.sleep(0.3)
    assert backend.count() == 1
    backend.release()
    assert child.wait(timeout=10) == 0, child.stderr.read()
    assert backend.count() == 20


def test_bad_settings_are_usage_errors_that_send_nothing(backend, tmp_path):
    """Change 13: the binding checks the throttle itself, so 300 is a
    ``UsageError`` and not an ``OverflowError``. A bool is refused. A
    request limit below the records refuses ``rank`` before any send; a
    streaming call sends the records under the limit first, as
    ``EngineBuilder::max_requests`` says, so this test uses ``rank``."""
    printed = run(SETTINGS + """
    for throttle in (0, 33, 300, -1, True, 8.0):
        said(throttle=throttle)
    said("rank", max_requests=2)
    said(max_requests=0)
    said(max_request_bytes=0)
    said(timeout=0)
    said(timeout=True)
    said(max_retries=-1)
    said(cache=7)
    """, child_env(backend, tmp_path))
    throttle = "UsageError a throttle is a whole number from 1 through 32"
    assert printed.splitlines() == 6 * [throttle] + [
        "UsageError this engine answers at most 2 records in one call",
        "UsageError a request limit is a whole number of 1 or more",
        "UsageError max_request_bytes is a whole number of at least 1",
        "UsageError a timeout is a time above zero",
        "UsageError a timeout is a whole number of seconds above zero",
        "UsageError max_retries is a whole number",
        "UsageError cache is a folder path, False for no cache, or True for the default folder",
    ]
    assert backend.count() == 0


def test_a_second_different_throttle_names_the_one_in_force(backend, tmp_path):
    """0077: the first explicit throttle sets the process cap."""
    printed = run(SETTINGS + """
    tt.Engine(throttle=8)
    tt.Engine(throttle=8)
    said(throttle=4)
    """, child_env(backend, tmp_path))
    assert printed.strip() == ("UsageError throttle 8 is already active for this process; "
                               "use throttle 8 or drop the throttle argument")


def test_the_cache_keyword_names_the_only_folder_written(backend, tmp_path):
    """Change 11: ``cache=`` writes only in its named folder."""
    printed = run(SETTINGS + """
    named = tt.Engine(cache=os.environ["NAMED"])
    named.decide(late, "one").value, named.decide(late, "one").value
    print(named.usage()["cache_answers"])
    """, child_env(backend, tmp_path, NAMED=str(tmp_path / "named")))
    assert printed.split() == ["1"]
    assert any((tmp_path / "named").rglob("*"))
    assert not (tmp_path / "cache").exists()
    assert backend.count() == 1


def test_replay_accepts_a_pathlike_folder(backend, tmp_path):
    printed = run(SETTINGS + """
    from pathlib import Path
    folder = Path(os.environ["NAMED"])
    assert tt.Engine(cache=folder).decide(late, "saved").value is True
    print(tt.Engine(replay=folder).decide(late, "saved").value)
    """, child_env(backend, tmp_path, NAMED=str(tmp_path / "named")))
    assert printed.strip() == "True"
    assert backend.count() == 1


def test_the_base_url_keyword_wins_over_the_environment(backend, tmp_path):
    """ADR 0017: the engine value's address comes before
    ``THINKTHEN_BASE_URL``. Both addresses are loopback."""
    other = Backend()
    try:
        run(SETTINGS + f"""
    tt.Engine(base_url="{other.base()}", cache=False).decide(late, "one").value
        """, child_env(backend, tmp_path))
        assert (other.count(), backend.count()) == (1, 0)
    finally:
        other.close()


def test_the_environment_seeds_every_unset_setting(backend, tmp_path):
    """Change 13: an engine built with keywords still reads
    ``THINKTHEN_CACHE``. The second call is a cache answer, and the entry
    lands in that folder and not under ``HOME``. ADR 0113: the scratch
    home holds only the count-only usage totals."""
    scratch = tmp_path / "home"
    run(SETTINGS + f"""
    engine = tt.Engine(throttle=8, base_url="{backend.base()}")
    engine.decide(late, "one").value, engine.decide(late, "one").value
    """, child_env(backend, tmp_path, HOME=str(scratch), XDG_CACHE_HOME=str(scratch)))
    assert backend.count() == 1
    assert any((tmp_path / "cache").rglob("*"))
    written = [path for path in scratch.rglob("*") if path.is_file()]
    assert written and all("thinkthen-usage" in path.parts for path in written)
    assert not any(b"late" in path.read_bytes() for path in written)
