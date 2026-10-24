"""Caller type contracts checked by mypy with the installed API declarations."""

import pandas as pd
import polars as pl
from typing_extensions import assert_type

import thinkthen as tt
from thinkthen.pandas import ThinkThenAccessor
from thinkthen.polars import ThinkThenNamespace


def answer_types(
    client: tt.Client, text: str
) -> tuple[bool | None, str | None, float | None, str | None]:
    decision: tt.Row[bool] = client.decide("Ready?", text)
    choice: tt.Row[str] = client.choose("Which?", text, options=["a", "b"])
    score: tt.Row[float] = client.score("How much?", text, levels=["low", "high"])
    found: tt.Row[tt.Found] = client.find("Which line?", text)
    found_text: str | None = found.answer["text"] if found.answer is not None else None
    return decision.answer, choice.answer, score.answer, found_text


def filter_types(client: tt.Client, messages: list[str]) -> list[str]:
    with client.filter("Ready?", messages) as kept:
        return list(kept)


def named_dictionary_inputs(client: tt.Client) -> list[dict[str, str]]:
    messages: list[dict[str, str]] = [{"id": "a", "body": "Please review this order."}]
    client.decide("Ready?", messages[0], on="/body")
    with client.decide_stream("Ready?", messages, on="/body") as rows:
        decisions: list[tt.Row[bool]] = list(rows)
        if decisions:
            decision: bool | None = decisions[0].answer
            print(decision)
    with client.filter("Ready?", messages, on="/body") as kept:
        result: list[dict[str, str]] = list(kept)
        return result


def named_backend(backend: str) -> tt.Client:
    return tt.Client(adapter=backend)


def native_frame_types(pandas_series: pd.Series[str], polars_series: pl.Series) -> None:
    assert_type(ThinkThenAccessor(pandas_series).decide("Ready?"), pd.DataFrame)
    assert_type(ThinkThenNamespace(polars_series).decide("Ready?"), pl.DataFrame)


def saved_domain_types(client: tt.Client, question: tt.Question[tt.JSONValue]) -> None:
    assert_type(client.choose(question, "alpha"), tt.Row[str])
    assert_type(client.tag(question, "alpha"), tt.Row[list[str]])
    assert_type(client.score(question, "alpha"), tt.Row[float])


def relation_direction(relation: tt.Relation) -> bool:
    return relation.get("either", False)
