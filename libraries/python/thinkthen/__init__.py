"""The Python surface of ThinkThen: ``import thinkthen as tt``.

The eight verbs as functions, ``None`` for "not sure", and a list or a
data frame column crossing into the engine once. The bulk spelling is
``decide_many``. Errors raise this package's own exception classes, each
carrying ``kind`` and ``retryable``; a cancel during a bulk wait raises
``Cancelled``, a subclass of ``KeyboardInterrupt``.
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
    choose,
    decide,
    decide_many,
    details,
    find,
    filter,
    probe,
    question,
    rank,
    reset_usage,
    score,
    tag,
    usage,
)

__all__ = [
    "annotate", "Cancelled", "choose", "decide", "decide_many",
    "details", "find", "filter", "probe", "question", "rank", "reset_usage",
    "score", "tag", "usage",
    "ThinkThenError", "UsageError", "BackendError", "DeadlineError",
    "LocalError", "DefectError",
]


def annotate(set, records, *, on=None, deadline=None):
    """Ask every question in the set of every record.

    A list of records comes back as a list of dictionaries, one field a
    question in the set's name order. A data frame comes back with one
    new column a question, read from the column ``on`` names, so the
    slide's ``tt.annotate("form.json", df, on="body")`` returns the frame
    with its new columns attached.
    """
    if on is None:
        return annotate_rows(set, records, deadline=deadline)
    column = records[on]
    rows = annotate_rows(set, list(column), deadline=deadline)
    for name in rows[0]:
        records[name] = [row[name] for row in rows]
    return records
