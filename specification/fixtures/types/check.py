"""Check the generated result schema and the door request schema against the public JSON door, offline."""

import ctypes
import json
import os
from pathlib import Path
import select
import subprocess
import sys
import tempfile

from jsonschema import Draft202012Validator


ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "conformance/children"))
from children import CARGO, child_env

HERE = Path(__file__).resolve().parent
SCHEMA = ROOT / "specification/result.schema.json"
QUESTION_FILE = ROOT / "specification/question-file.schema.json"
CORPUS = HERE / "corpus.json"
BACKEND = ROOT / "target/debug/conformance-backend"
LIBRARY_NAME = "libthinkthen_c.dylib" if sys.platform == "darwin" else "libthinkthen_c.so"
C_LIBRARY = ROOT / "libraries/c/target/debug" / LIBRARY_NAME


def checked(command):
    subprocess.run(command, cwd=ROOT, env=child_env(keep=CARGO), check=True, timeout=600)


def build():
    checked(["cargo", "build", "--locked", "--offline", "--quiet", "--manifest-path", "libraries/c/Cargo.toml", "--lib"])


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


def load_door():
    door = ctypes.CDLL(str(C_LIBRARY))
    door.thinkthen_engine_new.restype = ctypes.c_void_p
    door.thinkthen_engine_free.argtypes = [ctypes.c_void_p]
    door.thinkthen_call.argtypes = [ctypes.c_void_p, ctypes.c_char_p]
    door.thinkthen_call.restype = ctypes.c_void_p
    door.thinkthen_free_string.argtypes = [ctypes.c_void_p]
    door.thinkthen_error_code.argtypes = [ctypes.c_void_p]
    door.thinkthen_error_code.restype = ctypes.c_int
    door.thinkthen_plan_json.argtypes = [ctypes.c_void_p, ctypes.c_char_p,
                                         ctypes.POINTER(ctypes.c_void_p), ctypes.POINTER(ctypes.c_size_t)]
    door.thinkthen_plan_json.restype = ctypes.c_int
    return door


def start_backend():
    checked(["cargo", "build", "--locked", "--offline", "--quiet", "--package", "conformance-backend",
             "--target-dir", str(ROOT / "target")])
    process = subprocess.Popen(
        [str(BACKEND)], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
        stderr=subprocess.PIPE, env={"PATH": os.environ.get("PATH", "/usr/bin:/bin")},
        text=True,
    )
    try:
        ready, _, _ = select.select([process.stdout], [], [], 5)
        assert ready, "the offline backend never announced its port"
        return process, int(process.stdout.readline().strip())
    except BaseException:
        process.kill()
        process.wait()
        process.stdin.close()
        process.stdout.close()
        process.stderr.close()
        raise


def stop_backend(process):
    try:
        process.stdin.close()
        try:
            process.wait(timeout=5)
        except subprocess.TimeoutExpired:
            process.kill()
            process.wait()
    finally:
        process.stdout.close()
        process.stderr.close()


def call(door, port, case, cache):
    route = case.get("case_id", "generic")
    os.environ["THINKTHEN_BASE_URL"] = f"http://127.0.0.1:{port}/{ 'generic' if route == 'generic' else 'case/' + route }/v1"
    os.environ["THINKTHEN_API_KEY"] = "sk-type-contract-loopback"
    os.environ["THINKTHEN_CACHE"] = str(cache)
    engine = door.thinkthen_engine_new()
    assert engine, f"{case['name']}: C engine build"
    try:
        request = json.dumps(case["request"], ensure_ascii=False, separators=(",", ":")).encode()
        pointer = door.thinkthen_call(engine, request)
        if "expected_error" in case:
            assert not pointer, f"{case['name']}: invalid request answered"
            assert door.thinkthen_error_code(engine) == 1, f"{case['name']}: not a usage error"
            return None
        assert pointer, f"{case['name']}: C door returned error {door.thinkthen_error_code(engine)}"
        try:
            return json.loads(ctypes.string_at(pointer))
        finally:
            door.thinkthen_free_string(pointer)
    finally:
        door.thinkthen_engine_free(engine)


def plan(door, port, case, cache):
    """A plan case runs with no key, and the backend it could reach answers nothing."""
    os.environ["THINKTHEN_BASE_URL"] = f"http://127.0.0.1:{port}/generic/v1"
    os.environ.pop("THINKTHEN_API_KEY", None)
    os.environ["THINKTHEN_CACHE"] = str(cache)
    engine = door.thinkthen_engine_new()
    assert engine, f"{case['name']}: C engine build"
    try:
        request = json.dumps(case["plan_input"], ensure_ascii=False, separators=(",", ":")).encode()
        out, length = ctypes.c_void_p(), ctypes.c_size_t()
        code = door.thinkthen_plan_json(engine, request, ctypes.byref(out), ctypes.byref(length))
        assert code == 0, f"{case['name']}: C plan returned error {code}"
        try:
            return json.loads(ctypes.string_at(out, length.value))
        finally:
            door.thinkthen_free_string(out)
    finally:
        door.thinkthen_engine_free(engine)


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


def check_runtime(cases, checks, conformance):
    build()
    door = load_door()
    backend, port = start_backend()
    try:
        with tempfile.TemporaryDirectory(prefix="thinkthen-types-") as folder:
            for index, case in enumerate(cases):
                if "plan_input" in case:
                    actual = plan(door, port, case, Path(folder) / str(index))
                    assert checks["plan"].is_valid(actual), f"{case['name']}: actual plan schema"
                    assert actual == case["response"], f"{case['name']}: wrong C plan: {actual!r}"
                    continue
                if "request" not in case or case.get("schema_only", False):
                    continue
                if "case_id" in case and case["case_id"] != "generic":
                    assert case["case_id"] in conformance, f"{case['name']}: missing source case"
                actual = call(door, port, case, Path(folder) / str(index))
                if "expected_error" in case:
                    continue
                if case["definition"] != "usage":
                    assert checks["callSuccess"].is_valid(actual), f"{case['name']}: call envelope schema"
                    actual = actual["value"]
                if case["definition"] != "doorRequest":
                    assert checks[case["definition"]].is_valid(actual), f"{case['name']}: actual result schema"
                if "response" in case:
                    assert actual == case["response"], f"{case['name']}: wrong C result: {actual!r}"
                if "expect_fields" in case:
                    assert subset(actual, case["expect_fields"]), f"{case['name']}: wrong detail fields"
                if "expect_keys" in case:
                    assert set(actual) == set(case["expect_keys"]), f"{case['name']}: wrong result keys"
                if "offsets" in case:
                    check_offsets(case, actual, conformance)
    finally:
        stop_backend(backend)


def main():
    os.environ.pop("THINKTHEN_API_KEY", None)
    corpus = json.loads(CORPUS.read_text())
    assert corpus["schema"] == "thinkthen.type-corpus/1"
    names = [case["name"] for case in corpus["cases"]]
    assert len(names) == len(set(names)), "duplicate type case"
    checks = validators(json.loads(SCHEMA.read_text()))
    check_schema(corpus["cases"], checks)
    conformance = {case["id"]: case for case in json.loads((ROOT / "conformance/cases.json").read_text())["cases"]}
    check_runtime(corpus["cases"], checks, conformance)
    print(f"types: {len(names)} schema cases and real C door checks passed")


if __name__ == "__main__":
    main()
