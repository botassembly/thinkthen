"""Check packaged native ABI refusal through the public Swift startup error."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile


def check_abi(consumer, env):
    # Test the packaged lookup, not the override. Replace the installed resource in each
    # fresh process, so refused fixtures abort if layout access precedes the handshake.
    fixture_path = os.environ["THINKTHEN_NATIVE_FIXTURES"]
    cases = json.loads((Path(fixture_path) / "manifest.json").read_text())["cases"]
    filename = "libthinkthen_ffi.dylib" if sys.platform == "darwin" else "thinkthen_ffi.dll" if sys.platform == "win32" else "libthinkthen_ffi.so"
    resources = list(Path(consumer).parent.rglob(filename))
    assert len(resources) == 1, resources
    library = resources[0]


    def replace_library(source):
        # Give signed code a new inode before the next process loads it.
        with tempfile.TemporaryDirectory(dir=library.parent) as folder:
            staged = Path(folder) / library.name
            shutil.copy2(source, staged)
            os.replace(staged, library)


    with tempfile.TemporaryDirectory() as folder:
        original = Path(folder) / library.name
        shutil.copy2(library, original)
        try:
            for case in cases:
                replace_library(case["library"])
                result = subprocess.run([consumer, "abi"], env=env, capture_output=True, text=True, check=True)
                if case["accept"]:
                    assert result.stdout.strip() == "accepted", (case["name"], result.stdout)
                else:
                    code, message = result.stdout.rstrip("\n").split("\n", 1)
                    assert code == "usage.abi", (case["name"], code, message)
                    if case.get("missing"):
                        assert case["missing"] in message, (case["name"], message)
                    else:
                        assert "Native ABI" in message and "required ABI" in message, (case["name"], message)
        finally:
            replace_library(original)


if __name__ == '__main__':
    check_abi(sys.argv[1], dict(os.environ))
    print('Swift packaged ABI compatibility and exact missing-symbol diagnostics passed')
