"""Dataframe dispatch over the existing native Arrow and nullable readers.

Imports neither pandas nor Polars. Whole-set calls materialize the logical
Series once; they never pretend each chunk is a complete candidate set.
"""
from . import _thinkthen


def is_series(value):
    from . import _pandas
    if _pandas(value) == "Series":
        return True
    return type(value).__module__.partition(".")[0] == "polars" and \
        type(value).__name__ == "Series"


def source(value):
    from . import _pandas, _marked
    return _marked(value, "Series") if _pandas(value) == "Series" else value


def collection(engine, verb, asked, value, batch, context, deadline, token, tally=None):
    return _thinkthen._collection_column(engine._engine, verb, asked, source(value),
                                         batch, context, deadline, token, tally)


def filter_series(engine, asked, value, batch, context, deadline, token, tally=None):
    from . import _mapped, _pandas
    pandas = _pandas(value) == "Series"
    labels = value.index.copy() if pandas else None
    name = value.name if is_series(value) else None
    dtype = value.dtype if is_series(value) else None
    call = collection(engine, "filter", asked, value, batch, context, deadline, token, tally)
    def rebuild(rows):
        records = [row[1] for row in rows]
        if pandas:
            return type(value)(records, index=labels.take([row[0] for row in rows]),
                               name=name, dtype=dtype)
        return type(value)(name, records, dtype=dtype) if is_series(value) else records
    return _mapped(call, rebuild)


def annotate(engine, questions, value, batch, deadline, token):
    # Reuse the same named frame adapter, including its collision checks.
    frame = value.to_frame()
    return engine.annotate(questions, frame, on=frame.columns[0],
                           batch=batch, deadline_ms=deadline, token=token)


def recognize(engine, spec, value, deadline, token):
    from . import _mapped, _pandas
    pandas = _pandas(value) == "Series"
    labels, name = (value.index.copy() if pandas else None), value.name
    call = _thinkthen._recognize_series(engine._engine, spec, source(value), deadline, token)
    if pandas:
        return _mapped(call, lambda rows: type(value)(rows, index=labels,
                                                     name=name, dtype="object"))
    # Python Polars cannot store a native Recognized object in an Arrow struct;
    # its Object Series still retains the complete typed spans and relations.
    return _mapped(call, lambda rows: type(value)(name, rows, dtype=__import__("polars").Object))


def entities(value):
    # Relation endpoints are the native entities; order and duplicates survive.
    if hasattr(value, "isna"):
        return [item for item, missing in zip(value, value.isna()) if not missing]
    return [item for item in value.to_list() if item is not None]
