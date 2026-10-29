"""Check shared admission cases through each native SDK's public boundaries."""
import json
from pathlib import Path
import subprocess
import sys


def check(consumer, root, surface, env):
    cases = json.loads((Path(root) / "conformance/public-contract/usage-admission.json").read_text())
    assert surface in ("c", "cpp", "swift", "zig"), surface
    for case in cases:
        assert case["kind"] == "admission", case["name"]
        boundary = case["boundary"]
        value = case["input"]
        message = case["message"]
        if surface == "c":
            mode = "admission-" + boundary
            coverage = "direct"
        elif boundary == "settings":
            reason = ("JSONDecoder rejects the string before library admission"
                      if surface == "swift" else
                      "typed settings cannot represent a string max_retries")
            declared = case.get("not_applicable", {}).get(surface)
            assert declared, (case["name"], surface, "unmarked admission omission")
            print(f"{surface} admission: {case['name']}: not applicable: {declared}")
            continue
        elif boundary == "toolbox":
            mode = "toolbox-invalid"
            coverage = "direct Toolbox saved JSON"
        else:
            mode = "toolbox-invalid"
            value = {"entries": {"admission": {"question": value}}}
            assert message.startswith("question."), message
            message = "toolbox.entries.[0]." + message
            coverage = "public saved-entry projection of serialized question"
            if surface == "swift":
                reason = {
                    "score": "ScoreQuestion JSONDecoder requires string levels before library admission",
                    "tag": "TagQuestion JSONDecoder requires labels before canonical unknown-field admission",
                    "relate": "RelateQuestion JSONDecoder requires string rule endpoints before library admission",
                }[case["input"]["function"]]
            else:
                reason = "typed questions cannot represent malformed serialized fields"
            print(f"{surface} admission: {case['name']}: direct question unsupported: {reason}")
        assert_error(consumer, mode, value, case, message, env)
        print(f"{surface} admission: {case['name']}: {coverage}")
        if surface == "c" and boundary == "question":
            # The C request entry point adds the library's request path prefix.
            assert_error(consumer, "admission-request", {"question": value, "records": []},
                         case, "request." + case["message"], env)
            print(f"{surface} admission: {case['name']}: public request projection")


def assert_error(consumer, mode, value, case, message, env):
    result = subprocess.run([consumer, mode, json.dumps(value)], env=env,
                            check=True, capture_output=True, text=True)
    assert "secret-placeholder" not in result.stdout + result.stderr, case["name"]
    actual = result.stdout.rstrip("\n").split("\n", 1)
    assert actual == [case["code"], message], (case["name"], mode, actual, message)


if __name__ == "__main__":
    consumer, root, surface = sys.argv[1:]
    sys.path.insert(0, str(Path(root) / "databases"))
    from testing import child_environment

    check(consumer, root, surface, child_environment())
