"""The Python surface of ThinkThen: ``import thinkthen as tt``.

The ten functions, ``None`` for "not sure", and a list or a Polars
container crossing into the engine once. A list of strings crosses as
strings; a Polars Series (or a frame column) crosses zero-copy through
the Arrow stream form, and ``annotate`` on a Polars frame returns the
frame with its new columns attached — the wheel never imports Polars and
never loops a row; the width and the vectorization live in Rust. The bulk
spelling is ``decide_many``. ``recognize`` reads one text or a frame's
column and returns names with their kinds; ``relate`` reads every record
at once and returns the edges. A relation's two ends are ``source`` and
``target`` on every surface. Errors raise this package's own exception
classes, each carrying ``kind`` and ``retryable``; a cancel during a bulk
wait raises ``Cancelled``, a subclass of both ``KeyboardInterrupt`` and
``ThinkThenError``.

Every verb takes ``deadline`` — seconds from the moment of the call —
and ``token``, a ``CancelToken`` any thread can set to stop the call.
No deadline is spelled ``deadline=None``; a negative number is refused
as a usage error, so a budget computed as ``end - now`` that lands below
zero can never quietly disable the deadline.
"""

from ._thinkthen import (
    Cancelled,
    CancelToken,
    DeadlineError,
    DefectError,
    Edge,
    Entity,
    LocalError,
    BackendError,
    Relation,
    Recognized,
    ThinkThenError,
    UsageError,
    _probe_frame_rebuild,
    annotate_rows,
    annotate_stream,
    choose,
    decide as _decide,
    decide_many,
    details,
    find as _find,
    filter,
    question,
    rank as _rank,
    recognize as _recognize_text,
    recognize_stream,
    relate as _relate_records,
    relate_stream,
    score as _score,
    tag,
    usage,
)

__all__ = [
    "annotate", "Cancelled", "CancelToken", "choose", "decide", "decide_many",
    "details", "Edge", "Entity", "find", "filter", "question", "rank",
    "recognize", "Relation",
    "relate", "Recognized", "score",
    "tag", "usage",
    "ThinkThenError", "UsageError", "BackendError", "DeadlineError",
    "LocalError", "DefectError",
]

# The sentences a frame host that cannot rebuild its own frame from the
# Arrow stream hears. The probe raises them before any request runs, and
# the fallback after a real construction keeps them the same words.
_ANNOTATE_REBUILD_REFUSAL = (
    "annotate with on= cannot rebuild a pandas frame from the "
    "Arrow stream it returns. Pass the column instead — "
    "tt.annotate(set, df[column]) — which returns a list of "
    "dictionaries, one per row, or convert once and back — "
    "tt.annotate(set, pl.from_pandas(df), on=column).to_pandas()"
)
_RECOGNIZE_REBUILD_REFUSAL = (
    "recognize with on= cannot rebuild a pandas frame from the "
    "Arrow stream it returns. Pass the column instead — "
    "tt.recognize(df[column], kinds=...) — or convert once and "
    "back — tt.recognize(pl.from_pandas(df), on=column)"
)
_RELATE_REBUILD_REFUSAL = (
    "relate with on= cannot rebuild a pandas frame from the "
    "Arrow stream it returns. Pass the column instead — "
    "tt.relate(df[column], relations=...) — or convert once and "
    "back — tt.relate(pl.from_pandas(df), on=column)"
)


def _refuse_if_not_rebuildable(records, sentence):
    """Refuse a frame host that cannot rebuild its own frame from the
    Arrow stream the frame doors return — before any request runs.

    The probe builds an empty frame of the same shape and asks the host's
    own constructor for it; pandas' ``DataFrame`` and pyarrow's ``Table``
    raise there, so the refusal happens before the input's stream is ever
    taken and before a paid batch could start.
    """
    try:
        _probe_frame_rebuild(records)
    except Exception as exc:
        raise UsageError(sentence) from exc


def decide(question, text, *, deadline=None, token=None):
    """Ask once. ``True``, ``False``, or ``None`` when the question's
    rule makes it "not sure".

    A Polars column in returns a column out: ``tt.decide(ask, df["body"])``
    crosses zero-copy through the Arrow stream form, the engine runs the
    column 32 wide in Rust, and the answers come back as a column whose
    nulls are "not sure" — the deck's ``with_columns(complaint=...)`` line
    runs as drawn. A pandas Series answers every row too, and comes back
    as the plain list (pandas cannot rebuild its own column from the
    Arrow capsule), so ``pd.Series(answers)`` is the one line back.
    """
    answer = _decide(question, text, deadline=deadline, token=token)
    return _column_or_value(text, answer)


def score(question, text, levels=None, *, deadline=None, token=None):
    """Place the text on the question's levels: the position 0 to K-1.

    A Polars column in returns a number column out, and the public call
    takes ``levels`` beside the text — ``tt.score(ask, df["body"], levels)``
    runs as the deck draws. A pandas Series comes back as the plain list
    of numbers, the same as ``decide``. A token is read between rows, so
    a controller thread stops the column before the next row is sent.
    """
    answer = _score(question, text, levels=levels, deadline=deadline, token=token)
    return _column_or_value(text, answer)


def _column_or_value(text, answer):
    """The host's own column when the host consumes the Arrow array
    capsule the engine returns, the bare answers otherwise.

    Polars rebuilds its own column from the capsule — ``type(series)``
    takes it — and that is the drawn ``with_columns`` path. A pandas
    Series handed the same object would silently wrap it as one object
    row, because pandas has no constructor over the Arrow PyCapsule
    interface; a host that is not the Polars family gets the answers as
    the plain list the list door returns, so the settled one-liner
    ``pd.Series(answers)`` stays the way back to a pandas column.
    """
    if not hasattr(answer, "__arrow_c_array__"):
        return answer
    if type(text).__module__.split(".", 1)[0] != "polars":
        return answer.to_list()
    try:
        return type(text)(answer)
    except Exception as exc:
        raise UsageError(
            "this host cannot rebuild its own column from the Arrow "
            "array the engine returns; pass a Polars Series, or call "
            "the verb once a text"
        ) from exc


def rank(question, records, *, top=None, deadline=None, token=None):
    """Order the records most likely yes first, ties in input order.

    The answer is the ruled pair per record — its place in the input and
    the probability — as ``{"index", "record", "probability"}`` small
    records, one shape with the other surfaces; ``top`` keeps the first
    entries.
    """
    ranked = _rank(question, records, deadline=deadline, token=token)
    ordered = [
        {"index": index, "record": records[index], "probability": probability}
        for index, probability in ranked
    ]
    return ordered if top is None else ordered[:top]


def find(question, units, *, deadline=None, token=None):
    """Pick the unit that best answers the question; ``None`` fits nothing.

    The answer is the ruled pair — the unit's place in the input and the
    probability — as ``{"index", "unit", "probability"}``, or ``None``
    when nothing fits.
    """
    found = _find(question, units, deadline=deadline, token=token)
    if found is None:
        return None
    index, probability = found
    return {"index": index, "unit": units[index], "probability": probability}


def annotate(set, records, *, on=None, deadline=None, token=None):
    """Ask every question in the set of every record.

    A list of records comes back as a list of dictionaries, one field a
    question in the set's name order. A Polars frame comes back as the
    frame with one new column a question: the column ``on`` names crosses
    zero-copy through the Arrow stream form, the engine runs the whole
    batch in Rust, and the frame is rebuilt through the same stream form,
    so the slide's ``tt.annotate("form.json", df, on="body")`` returns
    the frame with its new columns attached and no Python row ever moves.
    A pandas frame cannot be rebuilt from the Arrow stream; it is refused
    before any request runs, and the refusal names the two ways through —
    pass the column, or convert once and back.
    """
    if on is None:
        return annotate_rows(set, records, deadline=deadline, token=token)
    if hasattr(records, "__arrow_c_stream__"):
        _refuse_if_not_rebuildable(records, _ANNOTATE_REBUILD_REFUSAL)
        frame = annotate_stream(set, records, on, deadline=deadline, token=token)
        try:
            return type(records)(frame)
        except Exception as exc:
            raise UsageError(_ANNOTATE_REBUILD_REFUSAL) from exc
    raise UsageError(
        "annotate with on= takes a frame whose column crosses as Arrow "
        "(a Polars DataFrame); a plain list uses annotate with no on="
    )


def recognize(text, *, kinds=None, relations=None, threshold=None,
              relation_threshold=None, on=None, deadline=None, token=None):
    """Find every name in a text and say what kind it is.

    The deck's call, as drawn::

        found = tt.recognize(text, kinds=["person", "organization"],
                             relations={"works_for": ("person", "organization")})
        text[found.entities[0].start:found.entities[0].end]  # the name

    Offsets count Python string positions, so ``text[start:end]`` is the
    name. ``kinds`` is a list of the user's own kind words or a path to a
    question file; with none, person, organization, and place. A relation
    value is a ``(source, target)`` pair, each end a kind or the
    one-character
    string ``"*"``. The number on a name is ``entity.strength``, the settled
    field name for the value computed from several of the model's numbers.

    With ``on=`` the first argument is a frame's column: one row per name,
    with the source row's number counted from 1, as a long frame. Relation
    rules ride the text form.
    """
    if on is None:
        return _recognize_text(
            text, kinds=kinds, relations=relations, threshold=threshold,
            relation_threshold=relation_threshold, deadline=deadline, token=token,
        )
    if relations is not None:
        raise UsageError(
            "recognize with on= returns one row per name and takes no "
            "relation rules; ask them of the whole text — "
            "tt.recognize(text, relations={...})"
        )
    if hasattr(text, "__arrow_c_stream__"):
        _refuse_if_not_rebuildable(text, _RECOGNIZE_REBUILD_REFUSAL)
        frame = recognize_stream(text, on, kinds=kinds, threshold=threshold,
                                 relation_threshold=relation_threshold,
                                 deadline=deadline, token=token)
        try:
            return type(text)(frame)
        except Exception as exc:
            raise UsageError(_RECOGNIZE_REBUILD_REFUSAL) from exc
    raise UsageError(
        "recognize with on= takes a frame whose column crosses as Arrow "
        "(a Polars DataFrame); a plain text uses recognize with no on="
    )


def relate(records, *, relations=None, either=None, threshold=None, on=None,
           deadline=None, token=None):
    """Say how the records relate to each other.

    The deck's call, as drawn::

        edges = tt.relate(alerts, relations=["caused_by"],
                          either=["same_as"], threshold=0.9)
        edges[0].name, edges[0].source, edges[0].target  # "caused_by", 1, 4

    Every record crosses at once, and more than 255 records refuses with a
    usage error before anything happens. ``relations`` is a list of names
    (any kind to any kind) or ``{name, source, target}`` mappings, or a
    path to a question file; ``either`` names the rules that read the same
    both ways. ``source`` and ``target`` are record numbers counted from 1
    in input order.

    With ``on=`` the first argument is a frame's column and the answer is
    a frame of edges, one row per edge.
    """
    if on is None:
        return _relate_records(records, relations=relations, either=either,
                               threshold=threshold, deadline=deadline, token=token)
    if hasattr(records, "__arrow_c_stream__"):
        _refuse_if_not_rebuildable(records, _RELATE_REBUILD_REFUSAL)
        frame = relate_stream(records, on, relations=relations, either=either,
                              threshold=threshold, deadline=deadline, token=token)
        try:
            return type(records)(frame)
        except Exception as exc:
            raise UsageError(_RELATE_REBUILD_REFUSAL) from exc
    raise UsageError(
        "relate with on= takes a frame whose column crosses as Arrow "
        "(a Polars DataFrame); a plain list uses relate with no on="
    )
