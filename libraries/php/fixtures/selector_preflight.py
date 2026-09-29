"""Exercise the real PHP selector before either branch reaches a build."""
import os
from pathlib import Path
import subprocess
import tempfile

CHECK = Path(__file__).resolve().parents[1] / "check.sh"

with tempfile.TemporaryDirectory(prefix="php-selector-preflight-") as temporary:
    work = Path(temporary)
    php = work / "php"
    php.write_text("#!/bin/sh\nif [ \"$1\" = -v ]; then echo 'PHP 8.3.0 (cli)'; fi\n")
    php.chmod(0o755)
    python = work / "python"
    python.write_text("#!/bin/sh\nexit \"${FAKE_JSONSCHEMA_EXIT:-0}\"\n")
    python.chmod(0o755)
    base = os.environ.copy()
    base.update(THINKTHEN_PHP_BIN=str(php), THINKTHEN_PYTHON_BIN=str(python),
                THINKTHEN_FLOCK_BIN="/bin/true", THINKTHEN_BWRAP_BIN=str(work / "absent-bwrap"),
                THINKTHEN_GIT_BIN=str(work / "absent-git"), THINKTHEN_TEST_PROFILE="routine")

    def select(extra):
        return subprocess.run([str(CHECK)], env=base | extra, capture_output=True,
                              text=True, timeout=15)

    missing_schema = select({"FAKE_JSONSCHEMA_EXIT": "1", "THINKTHEN_BWRAP_BIN": "/bin/true",
                             "THINKTHEN_GIT_BIN": "/bin/true"})
    assert missing_schema.returncode == 77 and "Python jsonschema is unavailable" in missing_schema.stderr, missing_schema
    missing_bwrap = select({"FAKE_JSONSCHEMA_EXIT": "0"})
    assert missing_bwrap.returncode == 77 and "tool is unavailable: " + str(work / "absent-bwrap") in missing_bwrap.stderr, missing_bwrap
    installed = select({"THINKTHEN_ARTIFACT": str(work / "selected-php.tar.gz"),
                        "THINKTHEN_C_ARTIFACT": str(work / "absent-c.tar.gz"),
                        "FAKE_JSONSCHEMA_EXIT": "1"})
    assert installed.returncode == 1 and installed.stderr.strip() == "PHP installed: C archive missing", installed
    print("PHP selector preflight: source tools required, installed branch reaches C archive guard")
