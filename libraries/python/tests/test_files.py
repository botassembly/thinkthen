"""Explicit source selections keep old inputs and provenance across all ten verbs."""
from pathlib import Path
from conftest import child_env, run

DOCUMENTS = Path(__file__).resolve().parents[3] / "specification/fixtures/files/documents"
QUESTIONS = DOCUMENTS.parent / "questions.json"


def test_all_functions_keep_the_shared_documents_and_source_coordinates(backend, tmp_path):
    output = run(f'''
        import pathlib, thinkthen as tt
        engine = tt.Engine(cache=False)
        selection = tt.read_files({str(DOCUMENTS)!r}, unit="file")
        original = [path.read_text() for path in sorted(pathlib.Path({str(DOCUMENTS)!r}).iterdir())]
        rows = list(selection)
        assert [row.record for row in rows] == original
        assert [(row.first_line, row.last_line) for row in rows] == [(1,4),(1,4)]
        calls = [engine.decide("Does this document need attention?", selection),
            engine.choose("What is this document?", selection, options=["policy","contract"]),
            engine.tag("Which topics appear?", selection, labels=["refund","support"]),
            engine.score("How urgent is this document?", selection, levels=["low","high"]),
            engine.filter("Does this document contain a support contract?", selection),
            engine.rank("Which document needs attention?", selection),
            engine.find("Which document contains a refund policy?", selection),
            engine.annotate({str(QUESTIONS)!r}, selection),
            engine.recognize(selection, kinds=["person"]),
            engine.relate(selection, relations={{"supports":("*","*")}})]
        for at, call in enumerate(calls):
            assert call.facts["requests_sent"] > 0
            if at == 9:
                assert call.value
                result = [edge[end] for edge in call.value for end in ("source","target")]
            elif at == 6: result = [call.value]
            else: result = call.value
            assert result
            for row in result:
                assert isinstance(row, tt.Located)
                assert row.source.record in original
                assert (row.source.first_line,row.source.last_line) == (1,4)
        assert engine.decide("Q?", "original evidence").value is True
        assert engine.decide("Q?", ["one","two"]).value == [True,True]
        assert engine.filter("Q?", ["one","two"]).value == ["one","two"]
        print("all ten and old calls")
    ''', child_env(backend, tmp_path, "arm/full"))
    assert output.strip() == "all ten and old calls"


def test_reader_refusals_send_nothing_and_do_not_infer_paths(backend, tmp_path):
    output = run(f'''
        import thinkthen as tt
        engine = tt.Engine(cache=False)
        for unit, window in [("file",2),("window",0),("window",None),("unknown",None),("line",True)]:
            try: tt.read_files({str(DOCUMENTS)!r}, unit=unit, window=window)
            except tt.UsageError: pass
            else: raise AssertionError("reader option accepted")
        try: engine.decide("Q?", tt.read_files({str(tmp_path / "missing")!r}))
        except tt.LocalError: pass
        else: raise AssertionError("missing operand accepted")
        try: engine.recognize(tt.read_files({str(DOCUMENTS)!r}), on="field")
        except tt.UsageError: pass
        else: raise AssertionError("field source map invented")
        print("refused")
    ''', child_env(backend, tmp_path))
    assert output.strip() == "refused"
    assert backend.count() == 0


def test_iteration_reports_a_later_content_failure_after_the_first_record(tmp_path):
    import thinkthen as tt
    folder = tmp_path / "sources"
    folder.mkdir()
    (folder / "01-good.txt").write_text("original\n")
    (folder / "02-bad.txt").write_bytes(b"\xff\n")
    selection = tt.read_files(folder)
    rows = iter(selection)
    assert next(rows).record == "original"
    try:
        next(rows)
    except tt.UsageError as error:
        assert str(error) == "the record is not valid UTF-8"
    else:
        raise AssertionError("invalid UTF-8 admitted")


def test_published_ten_function_script_replays_original_sources_without_requests(backend, tmp_path):
    import json, os
    from conftest import REPO
    script = REPO / "site/examples/learn/python/files.py"
    recording = REPO / "site/recordings"
    replacement = f'tt.Engine(model="local-1", base_url="https://api.typesafe.ai/v1", replay={str(recording)!r})'
    output = run(f'''
        import os, pathlib, thinkthen
        os.chdir({str(DOCUMENTS.parent)!r})
        script = pathlib.Path({str(script)!r})
        code = script.read_text().replace(
            'tt.Engine(model="local-1")',
            {replacement!r})
        exec(compile(code, str(script), "exec"))
    ''', child_env(backend, tmp_path))
    expected = script.with_suffix(".py.out").read_text()
    if os.name == "nt":
        # Folder entries use native separators; explicit window paths keep their spelling.
        folder, windows = expected.split("\nwindows\n")
        for filename in ("01-policy.txt", "02-contract.txt"):
            folder = folder.replace(f'"file": "documents/{filename}"',
                                    f'"file": {json.dumps(str(Path("documents") / filename))}')
        expected = folder + "\nwindows\n" + windows
    assert output == expected
    assert backend.count() == 0


def test_source_plan_bounds_admission_before_an_invalid_unread_tail(backend, tmp_path):
    folder = tmp_path / "planned"
    folder.mkdir()
    paths = [folder / f"{at}.txt" for at in range(3)]
    paths[0].write_text("Ada")
    paths[1].write_text("Bea")
    paths[2].write_bytes(b"\xff")
    output = run(f'''
        import thinkthen as tt
        engine = tt.Engine(cache=False, max_requests=1)
        try: engine.plan(engine.decide("Q?"), tt.read_files({str(folder)!r}, unit="file"))
        except tt.UsageError as error:
            assert str(error) == "this engine answers at most 1 records in one call", str(error)
        else: raise AssertionError("excess source record admitted")
        print("bounded")
    ''', child_env(backend, tmp_path))
    assert output.strip() == "bounded"
    assert backend.count() == 0


def test_source_plan_bounds_total_evidence_before_an_invalid_unread_tail(backend, tmp_path):
    folder = tmp_path / "planned"
    folder.mkdir()
    for at in range(2):
        (folder / f"{at}.txt").write_text("x" * (8 * 1024 * 1024 + 1))
    (folder / "2.txt").write_bytes(b"\xff")
    output = run(f'''
        import thinkthen as tt
        engine = tt.Engine(cache=False)
        try: engine.plan(engine.decide("Q?"), tt.read_files({str(folder)!r}, unit="file"))
        except tt.UsageError as error:
            assert str(error) == "source plan input exceeds 16 MiB", str(error)
        else: raise AssertionError("excess source evidence admitted")
        print("bounded")
    ''', child_env(backend, tmp_path))
    assert output.strip() == "bounded"
    assert backend.count() == 0


def test_source_plan_matches_original_evidence_without_metadata_or_sends(backend, tmp_path):
    path = tmp_path / "notes.txt"
    path.write_bytes("Refund βeta\r\n".encode())
    output = run(f'''
        import thinkthen as tt
        engine = tt.Engine(cache=False)
        judge = engine.decide("Q?")
        source = engine.plan(judge, tt.read_files({str(path)!r}))
        plain = engine.plan(judge, ["Refund βeta"])
        assert source == plain, (source, plain)
        assert source["records"] == 1
        assert source["requests"] == 1
        print("identical")
    ''', child_env(backend, tmp_path))
    assert output.strip() == "identical"
    assert backend.count() == 0
