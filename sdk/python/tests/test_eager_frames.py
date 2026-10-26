"""Own backend calls at eager frame boundaries and reuse materialized columns."""

from __future__ import annotations

from pathlib import Path
from typing import Any

import pandas as pd
import polars as pl
import pytest
from api_support import (
    backend,
    client,
    isolated_environment,  # noqa: F401 - shared pytest fixture.
    yes,
)

import thinkthen as tt
import thinkthen.pandas
import thinkthen.polars
from thinkthen.pandas import ThinkThenAccessor, ThinkThenFrameAccessor
from thinkthen.polars import ThinkThenFrameNamespace, ThinkThenNamespace

pytestmark = pytest.mark.usefixtures("isolated_environment")


def test_pandas_missing_rows_keep_duplicate_index_and_nullable_types(
    tmp_path: Path,
) -> None:
    index = pd.Index(["same", "same", "third", "fourth", "fifth"])
    source = pd.Series([None, pd.NA, float("nan"), pd.NaT, "body"], index=index)
    with backend(tmp_path, yes()) as server, client(server) as selected:
        result = ThinkThenAccessor(source).decide("Ready?", client=selected)
        assert result.index.equals(index) and len(result) == 5
        assert result["position"].tolist() == [1, 2, 3, 4, 5]
        assert result["status"].tolist() == ["failed"] * 4 + ["answered"]
        assert result["error_code"].iloc[:4].tolist() == ["record.missing"] * 4
        assert result["answer"].isna().tolist() == [True] * 4 + [False]
        assert str(result["answer"].dtype) == "boolean"
        assert str(result["probability"].dtype) == "Float64"
        assert str(result["status"].dtype) == "category"
        assert server.count() == 1
        assert selected.decide("Ready?", "still open").answer is True


def test_pandas_dataframe_evidence_and_filter_do_not_hide_invalid_records(
    tmp_path: Path,
) -> None:
    source = pd.DataFrame({"id": ["one", True], "body": ["text", "text"]})
    with backend(tmp_path, yes()) as server, client(server) as selected:
        result = ThinkThenFrameAccessor(source).filter(
            "Ready?", evidence="/body", client=selected
        )
        assert len(result) == 2 and result.index.equals(source.index)
        assert bool(result["kept"].iloc[0])
        assert pd.isna(result["kept"].iloc[1])
        assert result["error_code"].iloc[1] == "record.invalid"
        assert result["item"].iloc[0] == {"id": "one", "body": "text"}
        assert server.count() == 1


def test_polars_materialized_struct_results_never_call_back_during_lazy_queries(
    tmp_path: Path,
) -> None:
    frame = pl.DataFrame({"body": ["first", None, "third"]})
    before = frame.clone()
    with backend(tmp_path, yes()) as server, client(server) as selected:
        results = ThinkThenNamespace(frame.get_column("body")).decide(
            "Ready?", client=selected
        )
        assert isinstance(results.dtype, pl.Struct)
        assert results.len() == 3 and server.count() == 2
        assert results.struct.field("position").to_list() == [1, 2, 3]
        assert results.struct.field("error").struct.field("code").to_list() == [
            None,
            "record.missing",
            None,
        ]
        enriched = frame.with_columns(result=results)
        plan = enriched.lazy().select(pl.col("result").struct.field("answer"))
        assert plan.collect().get_column("answer").to_list() == [True, None, True]
        assert plan.collect().height == 3
        assert server.count() == 2 and frame.equals(before)
        assert not hasattr(pl.col("body"), "thinkthen")
        assert not hasattr(frame.lazy(), "thinkthen")
        selected.close()
        assert plan.collect().height == 3 and server.count() == 2


@pytest.mark.parametrize("identity", ["string", -(2**63), 2**63 - 1, 2**64 - 1])
def test_polars_dataframe_preserves_string_signed_and_unsigned_ids(
    tmp_path: Path, identity: str | int
) -> None:
    dtype = (
        pl.String
        if isinstance(identity, str)
        else (pl.UInt64 if identity >= 2**63 else pl.Int64)
    )
    source = pl.DataFrame(
        {"id": pl.Series("id", [identity], dtype=dtype), "body": ["text"]}
    )
    with backend(tmp_path, yes()) as server, client(server) as selected:
        result = ThinkThenFrameNamespace(source).decide(
            "Ready?", evidence="/body", client=selected
        )
        assert result.columns == ["result"] and result.height == 1
        ids = result.get_column("result").struct.field("id")
        kind = (
            "string"
            if isinstance(identity, str)
            else ("unsigned" if identity >= 2**63 else "signed")
        )
        assert ids.struct.field(kind).to_list() == [identity]
        assert ids.struct.field("kind").to_list() == [kind]
        assert server.count() == 1


@pytest.mark.parametrize("operation", ["decide", "rank"])
def test_polars_empty_results_have_zero_rows_and_the_declared_struct_dtype(
    tmp_path: Path, operation: str
) -> None:
    source = pl.Series("body", [], dtype=pl.String)
    options: dict[str, Any] = {"levels": ["low", "high"]} if operation == "rank" else {}
    with backend(tmp_path, yes()) as server, client(server) as selected:
        result = getattr(ThinkThenNamespace(source), operation)(
            "Ready?", client=selected, **options
        )
        assert result.len() == 0 and isinstance(result.dtype, pl.Struct)
        frame = getattr(ThinkThenFrameNamespace(source.to_frame()), operation)(
            "Ready?", client=selected, **options
        )
        assert frame.height == 0 and frame.columns == ["result"]
        assert frame.get_column("result").dtype == result.dtype
        assert server.count() == 0


def test_frame_rank_top_retains_failed_rows_and_original_positions() -> None:
    with tt.Client(backend="stub") as selected:
        series = pl.Series("body", ["first", None, "third"])
        result = ThinkThenNamespace(series).rank(
            "How much?", levels=["low", "high"], top=1, client=selected
        )
        assert result.len() == 2
        assert result.struct.field("answer").to_list() == [1, None]
        assert result.struct.field("status").to_list() == ["answered", "failed"]
        assert result.struct.field("position").to_list()[1] == 2
        assert result.struct.field("position").to_list()[0] in {1, 3}
        assert result.struct.field("error").struct.field("code").to_list() == [
            None,
            "record.missing",
        ]
        pandas_source = pd.Series(["first", None, "third"], index=["a", "b", "c"])
        pandas_result = ThinkThenAccessor(pandas_source).rank(
            "How much?", levels=["low", "high"], top=1, client=selected
        )
        assert len(pandas_result) == 2
        assert (
            pandas_result["position"].tolist()
            == result.struct.field("position").to_list()
        )
        assert pandas_result.index[-1] == "b"


def test_stream_converters_keep_empty_types_and_do_not_replay() -> None:
    with tt.Client(backend="stub") as selected:
        with selected.choose_stream("Which?", [], choices=["a", "b"]) as empty_rows:
            empty = empty_rows.to_polars()
        assert empty.height == 0
        assert empty["result"].struct.field("answer").dtype == pl.String
        with selected.decide_stream("Ready?", ["first", None, "third"]) as rows:
            assert next(rows).position == 1
            frame = rows.to_pandas()
        assert frame["position"].tolist() == [2, 3]
        assert frame["error_code"].iloc[0] == "record.missing"
        assert frame["answer"].iloc[0] is pd.NA
        assert rows.to_list() == []
