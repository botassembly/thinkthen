"""Caller overloads and concrete results checked against the installed package."""

from collections.abc import AsyncIterator
from typing import Any

import pandas as pd
import polars as pl
from typing_extensions import assert_type

import thinkthen as tt
import thinkthen.pandas
import thinkthen.polars
from thinkthen import aio
from thinkthen.pandas import ThinkThenAccessor, ThinkThenFrameAccessor
from thinkthen.polars import ThinkThenFrameNamespace, ThinkThenNamespace


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
    assert_type(toolbox.choose("route"), tt.ChooseQuestion[Any])
    assert_type(client.choose(toolbox.choose("route"), text), tt.ChooseRow[Any])
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
    assert_type(client.filter("Ready?", records), tt.KeptStream[str])
    assert_type(client.filter("Ready?", records).to_list(), list[str])
    ranked = client.rank("How much?", records, levels=["low", "high"])
    assert_type(ranked, tt.RowStream[tt.RankRow[str]])
    assert_type(next(ranked).item, str)
    client.decide("Ready?", "body", yes_means="ships today", no_means="waits")
    client.choose("Which?", "body", choices=["a", "b"], descriptions={"a": "first"})


async def typed_async(client: tt.AsyncClient, records: AsyncIterator[str]) -> None:
    assert_type(await client.decide("Ready?", "body"), tt.DecideRow)
    assert_type(await aio.decide("Ready?", "body", client=client), tt.DecideRow)
    assert_type(
        client.decide_stream("Ready?", records), tt.AsyncRowStream[tt.DecideRow]
    )
    assert_type(client.filter("Ready?", records), tt.AsyncKeptStream[str])
    async with client.choose_stream("Which?", records, choices=["a", "b"]) as rows:
        async for row in rows:
            assert_type(row, tt.ChooseRow)
            assert_type(row.answer, str)


def typed_optional_frames(
    pandas_series: pd.Series[str], polars_series: pl.Series
) -> None:
    assert_type(ThinkThenAccessor(pandas_series).decide("Ready?"), pd.Series[Any])
    assert_type(
        ThinkThenAccessor(pandas_series).decide("Ready?", details=True), pd.DataFrame
    )
    assert_type(ThinkThenAccessor(pandas_series).filter("Ready?"), pd.Series[Any])
    assert_type(
        ThinkThenFrameAccessor(pandas_series.to_frame()).filter("Ready?"), pd.DataFrame
    )
    assert_type(
        ThinkThenFrameAccessor(pandas_series.to_frame()).decide("Ready?"),
        pd.Series[Any],
    )
    assert_type(
        ThinkThenFrameNamespace(polars_series.to_frame()).decide("Ready?"), pl.Series
    )
    assert_type(
        ThinkThenFrameNamespace(polars_series.to_frame()).filter("Ready?"), pl.DataFrame
    )
    assert_type(ThinkThenNamespace(polars_series).decide("Ready?"), pl.Series)
    assert_type(ThinkThenAccessor(pandas_series).failures(), pd.DataFrame)
    assert_type(ThinkThenNamespace(polars_series).failures(), pl.DataFrame)


def typed_table_enum_domains(
    pandas_series: pd.Series[str], polars_series: pl.Series
) -> None:
    from enum import Enum

    class Topic(Enum):
        BILLING = "billing"
        SHIPPING = "shipping"

    class Urgency(Enum):
        ROUTINE = "routine"
        URGENT = "urgent"

    pandas_accessor = ThinkThenAccessor(pandas_series)
    polars_accessor = ThinkThenNamespace(polars_series)
    assert_type(pandas_accessor.choose("Which?", choices=Topic), pd.Series[Any])
    assert_type(polars_accessor.choose("Which?", choices=Topic), pl.Series)
    assert_type(pandas_accessor.tag("Which?", labels=Topic), pd.Series[Any])
    assert_type(polars_accessor.tag("Which?", labels=Topic), pl.Series)
    assert_type(pandas_accessor.score("How urgent?", levels=Urgency), pd.Series[Any])
    assert_type(polars_accessor.score("How urgent?", levels=Urgency), pl.Series)
    assert_type(pandas_accessor.rank("How urgent?", levels=Urgency), pd.Series[Any])
    assert_type(polars_accessor.rank("How urgent?", levels=Urgency), pl.Series)


def typed_choice_sets(client: tt.Client, text: str) -> None:
    from enum import Enum
    from typing import Literal

    class Team(Enum):
        BILLING = "billing"
        SHIPPING = "shipping"

    team = client.choose("Which team?", text, choices=Team)
    assert_type(team, tt.ChooseRow[Team])
    assert_type(team.answer, Team)
    question = tt.ChooseQuestion("Which team?", choices=Team)
    assert_type(client.choose(question, text), tt.ChooseRow[Team])
    fitting = client.tag("Which team?", text, labels=Team)
    assert_type(fitting, tt.TagRow[Team])
    assert_type(fitting.answer[0], Team)
    category = client.choose(
        "Which team?", text, choices=Literal["billing", "shipping"]
    )
    assert_type(category, tt.ChooseRow[Literal["billing", "shipping"]])
    literal_question = tt.ChooseQuestion(
        "Which team?", choices=Literal["billing", "shipping"]
    )
    assert_type(literal_question, tt.ChooseQuestion[Literal["billing", "shipping"]])
    assert_type(client.score("How much?", text, levels=Team), tt.ScoreRow)
