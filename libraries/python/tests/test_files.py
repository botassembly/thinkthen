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
