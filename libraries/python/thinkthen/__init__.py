"""The Python surface of ThinkThen: ``import thinkthen as tt``.

The eight verbs as functions, ``None`` for "not sure", and a list or a
Polars container crossing into the engine once. A list of strings crosses
as strings; a Polars Series (or a frame column) crosses zero-copy through
the Arrow stream form, and ``annotate`` on a Polars frame returns the
frame with its new columns attached — the wheel never imports Polars and
never loops a row; the width and the vectorization live in Rust. The bulk
spelling is ``decide_many``. Errors raise this package's own exception
classes, each carrying ``kind`` and ``retryable``; a cancel during a bulk
wait raises ``Cancelled``, a subclass of ``KeyboardInterrupt``.
"""

from ._thinkthen import (
    Cancelled,
    DeadlineError,
    DefectError,
    LocalError,
    BackendError,
    ThinkThenError,
    UsageError,
    annotate_rows,
    annotate_stream,
    choose,
    decide,
    decide_many,
    details,
    find,
    filter,
    question,
    rank,
    reset_usage,
    score,
    tag,
    usage,
)

__all__ = [
    "annotate", "Cancelled", "choose", "decide", "decide_many",
    "details", "find", "filter", "question", "rank", "reset_usage",
    "score", "tag", "usage",
    "ThinkThenError", "UsageError", "BackendError", "DeadlineError",
    "LocalError", "DefectError",
]


def annotate(set, records, *, on=None, deadline=None):
    """Ask every question in the set of every record.

    A list of records comes back as a list of dictionaries, one field a
    question in the set's name order. A Polars frame comes back as the
    frame with one new column a question: the column ``on`` names crosses
    zero-copy through the Arrow stream form, the engine runs the whole
    batch in Rust, and the frame is rebuilt through the same stream form,
    so the slide's ``tt.annotate("form.json", df, on="body")`` returns
    the frame with its new columns attached and no Python row ever moves.
    A pandas frame cannot be rebuilt from the Arrow stream; the refusal
    names the two ways through — pass the column, or convert once and back.
    """
    if on is None:
        return annotate_rows(set, records, deadline=deadline)
    if hasattr(records, "__arrow_c_stream__"):
        frame = annotate_stream(set, records, on, deadline=deadline)
        try:
            return type(records)(frame)
        except Exception as exc:
            raise UsageError(
                "annotate with on= cannot rebuild a pandas frame from the "
                "Arrow stream it returns. Pass the column instead — "
                "tt.annotate(set, df[column]) — which returns a list of "
                "dictionaries, one per row, or convert once and back — "
                "tt.annotate(set, pl.from_pandas(df), on=column).to_pandas()"
            ) from exc
    raise UsageError(
        "annotate with on= takes a frame whose column crosses as Arrow "
        "(a Polars DataFrame); a plain list uses annotate with no on="
    )
