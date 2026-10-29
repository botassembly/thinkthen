"""Keep eager column conversion compatible with canonical row fields."""

from __future__ import annotations

import importlib
import os
from pathlib import Path
from typing import Any

import pandas as pd
import polars as pl
import pytest
from conformance import Loopback

import thinkthen as tt


@pytest.mark.parametrize("library", ["pandas", "polars"])
def test_missing_column_rows_keep_engine_positions_and_probability(
    library: str,
) -> None:
    importlib.import_module("thinkthen." + library)
    source: Any = (
        pd.Series([None, "evidence"], index=["same", "same"], dtype=object)
        if library == "pandas"
        else pl.Series("body", [None, "evidence"])
    )
    path = Path(os.environ["THINKTHEN_CASES"]) / "typed/decide-precision.json"
    with Loopback(path) as server:
        client = tt.Client(url=server.url, api_key="synthetic")
        result = source.thinkthen.decide("Ready?", client=client)
        frame = result if library == "pandas" else result.struct.unnest()
        assert len(frame) == 2
        assert frame["position"].to_list() == [1, 2]
        assert frame["status"].to_list() == ["failed", "answered"]
        assert frame["probability"].to_list()[1] == 0.8123456789012345
        assert (
            frame["error_code"].to_list()[0]
            if library == "pandas"
            else frame["error"].to_list()[0]["code"]
        ) == "record.missing"
        assert server.count() == 1
        if library == "pandas":
            assert frame.index.equals(source.index)
        else:
            assert not hasattr(pl.col("body"), "thinkthen")
            assert not hasattr(source.to_frame().lazy(), "thinkthen")


@pytest.mark.parametrize("library", ["pandas", "polars"])
def test_empty_columns_preserve_declared_fields(library: str) -> None:
    module: Any = importlib.import_module(library)
    importlib.import_module("thinkthen." + library)
    source = (
        module.Series([], dtype=object)
        if library == "pandas"
        else module.Series("body", [], dtype=module.String)
    )
    result = source.thinkthen.decide("Ready?", client=tt.Client(backend="stub"))
    frame = result if library == "pandas" else result.struct.unnest()
    assert len(frame) == 0
    assert "probability" in frame.columns and "source" in frame.columns
    assert "p" not in frame.columns


def test_polars_relate_detail_keeps_recognized_names() -> None:
    import thinkthen.polars

    with tt.Client(backend="stub") as client:
        row = client.relate(
            "Ada knows Bob.", rules=[tt.RelationRule("person", "person", "knows")]
        )
        assert row.detail is not None and row.detail.names
        result = thinkthen.polars.series([row])
        names = result.struct.field("detail").struct.field("names").to_list()[0]
        assert names == row.to_dict()["detail"]["names"]
        empty = thinkthen.polars.series([], "relate")
        assert (
            empty.struct.field("detail").struct.field("names").dtype
            == result.struct.field("detail").struct.field("names").dtype
        )


@pytest.mark.parametrize("library", ["pandas", "polars"])
@pytest.mark.parametrize("backend", ["systemone", "openai"])
@pytest.mark.parametrize("surface", ["series", "stream"])
def test_filter_unsure_and_failed_are_not_kept(
    library: str, backend: str, surface: str
) -> None:
    import json

    from conformance import admitted

    importlib.import_module("thinkthen." + library)
    path = Path(os.environ["THINKTHEN_CASES"]) / "typed/filter-rows.json"
    case = json.loads(path.read_text())
    records = case["request"]["records"]
    question = admitted(case["request"]["question"])
    source: Any = (
        pd.Series(records, index=["same"] * len(records), dtype=object)
        if library == "pandas"
        else pl.Series("body", records)
    )
    with (
        Loopback(path) as server,
        tt.Client(
            backend=tt.Backend(backend), url=server.url, api_key="synthetic", batch=32
        ) as client,
    ):
        if surface == "stream":
            with client.filter(question, records) as rows:
                result = getattr(rows, "to_" + library)()
        else:
            result = source.thinkthen.filter(question, client=client)
        frame = (
            result
            if library == "pandas"
            else (
                result["result"] if isinstance(result, pl.DataFrame) else result
            ).struct.unnest()
        )
        assert len(frame) == len(case["expected"])
        for index, expected in enumerate(case["expected"]):
            assert frame["status"].to_list()[index] == expected["status"]
            kept = frame["kept"].to_list()[index]
            assert not pd.isna(kept)
            assert bool(kept) is (
                expected["status"] == "answered" and expected.get("answer") is True
            )
        assert server.count() == 1
