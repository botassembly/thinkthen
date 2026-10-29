"""Compare installed R projections without confusing integer and double storage."""
def typed_numbers(value):
    for entry in value.pop("numeric_types"):
        path = entry["path"]
        parent = value
        for part in path[:-1]:
            parent = parent[part]
        key = path[-1]
        if entry["type"] == "double" and type(parent[key]) is int:
            parent[key] = float(parent[key])
    return value


def assert_fields(actual, expected, context):
    if isinstance(expected, dict):
        assert isinstance(actual, dict), (context, actual, expected)
        assert actual.keys() == expected.keys(), (context, actual, expected)
        for key, value in expected.items():
            assert key in actual, (context, "missing", key, actual)
            assert_fields(actual[key], value, context)
    elif isinstance(expected, list):
        assert isinstance(actual, list) and len(actual) == len(expected), (context, actual, expected)
        for got, want in zip(actual, expected):
            assert_fields(got, want, context)
    else:
        assert type(actual) is type(expected) or type(expected) is int and type(actual) is float and actual.is_integer(), (context, actual, expected)
        assert actual == expected, (context, actual, expected)


def shared_rows(rows, backend, model):
    """Remove only the wire-dependent fields omitted by shared expected rows."""
    def project(row):
        row = dict(row)
        if "source" in row:
            source = dict(row.pop("source"))
            assert source.pop("backend") == backend, source
            assert source.pop("model") == model, source
            identity = source.pop("question_identity")
            assert len(identity) == 64 and all(c in "0123456789abcdef" for c in identity), source
            if source:
                row["source"] = source
        if "error" in row:
            row["error"] = dict(row["error"])
            row["error"].pop("message", None)
        if row["function"] == "annotate" and row.get("answer"):
            row["answer"] = {name: project(child) for name, child in row["answer"].items()}
        return row
    return [project(row) for row in rows]


if __name__ == "__main__":
    # Regression: a whole-valued probability still requires R double storage.
    for wrong in (1, True):
        try:
            assert_fields(wrong, 1.0, "seeded integer or boolean probability")
        except AssertionError:
            pass
        else:
            raise AssertionError("a non-double passed as a floating point value")
    restored = typed_numbers({"rows": [{"probability": 1}], "numeric_types": [
        {"path": ["rows", 0, "probability"], "type": "double"}]})
    assert_fields(restored, {"rows": [{"probability": 1.0}]}, "R double survives JSON")
    for actual, expected in (({"answer": True, "extra": 1}, {"answer": True}),
                             ({"source": {"name": "unexpected"}}, {})):
        try:
            assert_fields(actual, expected, "seeded extra field")
        except AssertionError:
            pass
        else:
            raise AssertionError("an unexpected field passed strict row comparison")
    print("R row comparison rejects integer and boolean probabilities")
