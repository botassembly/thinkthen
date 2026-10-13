"""Check result and question-file schema fixtures without executing a retired ABI."""

import json
from pathlib import Path

from jsonschema import Draft202012Validator


ROOT = Path(__file__).resolve().parents[3]
HERE = Path(__file__).resolve().parent
SCHEMA = ROOT / "specification/result.schema.json"
QUESTION_FILE = ROOT / "specification/question-file.schema.json"
CORPUS = HERE / "corpus.json"
def validators(schema):
    """One validator per result definition, plus `doorRequest` from the question-file schema."""
    requests = json.loads(QUESTION_FILE.read_text())
    Draft202012Validator.check_schema(schema)
    Draft202012Validator.check_schema(requests)
    checks = {
        name: Draft202012Validator({"$ref": f"#/$defs/{name}", "$defs": schema["$defs"]})
        for name in schema["$defs"]
    }
    checks["doorRequest"] = Draft202012Validator({"$ref": "#/$defs/doorRequest", "$defs": requests["$defs"]})
    return checks


def subset(actual, expected):
    if isinstance(expected, dict):
        return isinstance(actual, dict) and all(
            key in actual and subset(actual[key], value) for key, value in expected.items()
        )
    return actual == expected


def check_schema(cases, checks):
    for case in cases:
        name = case["name"]
        if "request" in case:
            got = checks["doorRequest"].is_valid(case["request"])
            assert got == case.get("request_valid", True), f"{name}: request schema verdict"
        if "response" in case:
            got = checks[case["definition"]].is_valid(case["response"])
            assert got == case.get("response_valid", True), f"{name}: result schema verdict"


def check_offsets(case, actual, conformance):
    source = conformance[case["case_id"]]
    assert source["text"] == "Le café 😀 Maria Chen arrived."
    span = case["offsets"]
    assert span == {"scalar": [10, 20], "utf16": [11, 21], "r_one_based_inclusive": [11, 20]}
    assert source["text"][10:20] == "Maria Chen"
    assert len(source["text"][:10].encode("utf-16-le")) // 2 == 11
    entity = actual["entities"][0]
    assert [entity["start"], entity["end"]] == span["scalar"]
    assert entity["length"] == 10


def main():
    corpus = json.loads(CORPUS.read_text())
    assert corpus["schema"] == "thinkthen.type-corpus/1"
    names = [case["name"] for case in corpus["cases"]]
    assert len(names) == len(set(names)), "duplicate type case"
    checks = validators(json.loads(SCHEMA.read_text()))
    check_schema(corpus["cases"], checks)
    conformance = {case["id"]: case for case in json.loads((ROOT / "conformance/cases.json").read_text())["cases"]}
    for case in corpus["cases"]:
        if "offsets" in case:
            check_offsets(case, case["response"], conformance)
    print(f"types: {len(names)} schema cases passed")


if __name__ == "__main__":
    main()
