"""Caller overloads and concrete results checked against the installed package."""

from collections.abc import AsyncIterator, Iterator

import pandas as pd
import polars as pl
from typing_extensions import assert_type

import thinkthen as tt
import thinkthen.pandas
import thinkthen.polars
from thinkthen import aio
from thinkthen.pandas import ThinkThenAccessor
from thinkthen.polars import ThinkThenNamespace


def typed_rows(client: tt.Client, text: str) -> None:
    assert_type(client.decide("Ready?", text), tt.DecideRow)
    assert_type(client.choose("Which?", text, choices=["a", "b"]), tt.ChooseRow)
    assert_type(client.tag("Which?", text, labels=("a", "b")), tt.TagRow)
    assert_type(client.score("How much?", text, levels=["low", "high"]), tt.ScoreRow)
    assert_type(client.find("Which line?", text), tt.FindRow)
    assert_type(client.recognize(text, kinds=["person"]), tt.RecognizeRow)
    rule = tt.RelationRule(source="person", target="person", relation="knows")
    assert_type(client.relate(text, rules=[rule]), tt.RelateRow)
    question = tt.ChooseQuestion("Which?", choices=["a", "b"])
    toolbox = tt.Toolbox({"route": question})
    assert_type(toolbox.choose("route"), tt.ChooseQuestion)
    assert_type(client.choose(toolbox.choose("route"), text), tt.ChooseRow)
    assert_type(tt.choose(question, text, client=client), tt.ChooseRow)
    assert_type(
        client.annotate(
            tt.AnnotationQuestions({"ready": tt.DecideQuestion("Ready?")}), text
        ),
        tt.AnnotateRow,
    )
    assert_type(client.decide("Ready?", text).probability, float | None)
    assert_type(client.decide("Ready?", text).status, tt.Status)
    assert_type(client.decide("Ready?", text).id, str | int | None)
    client.decide("Ready?", text, threshold=tt.Band(0.2, 0.8))
    client.decide("Ready?", text, threshold=(0.2, 0.8))


def typed_iterators(client: tt.Client, records: list[str]) -> None:
    assert_type(client.decide_stream("Ready?", records), tt.RowStream[tt.DecideRow])
    with client.filter("Ready?", records) as rows:
        row = next(rows)
        assert_type(row, tt.FilterRow[str])
        assert_type(row.item, str)
        assert_type(row.kept, bool)
        assert_type(rows.kept_items(), Iterator[str])
    assert_type(
        client.rank("How much?", records, levels=["low", "high"]),
        tt.RowStream[tt.RankRow],
    )


async def typed_async(client: tt.AsyncClient, records: AsyncIterator[str]) -> None:
    assert_type(await client.decide("Ready?", "body"), tt.DecideRow)
    assert_type(await aio.decide("Ready?", "body", client=client), tt.DecideRow)
    assert_type(
        client.decide_stream("Ready?", records), tt.AsyncRowStream[tt.DecideRow]
    )
    assert_type(client.filter("Ready?", records).kept_items(), AsyncIterator[str])
    async with client.choose_stream("Which?", records, choices=["a", "b"]) as rows:
        async for row in rows:
            assert_type(row, tt.ChooseRow)
            assert_type(row.answer, str | None)


def typed_optional_frames(
    pandas_series: pd.Series[str], polars_series: pl.Series
) -> None:
    assert_type(ThinkThenAccessor(pandas_series).decide("Ready?"), pd.DataFrame)
    assert_type(ThinkThenNamespace(polars_series).decide("Ready?"), pl.Series)
