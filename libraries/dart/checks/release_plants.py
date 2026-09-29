"""Refuse missing Dart source and wrong pub identity before pub or a call."""
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

package = Path(sys.argv[1])
native = Path(sys.argv[2])
fixture = Path(__file__).with_name("portable_batch.py")
for name, marker in (("missing-door", "DART_ARCHIVE_MEMBERS"),
                     ("wrong-pub-name", "DART_PUBSPEC_IDENTITY")):
    with tempfile.TemporaryDirectory(prefix="thinkthen-dart-release-plant-") as scratch:
        planted = Path(scratch) / "package"
        shutil.copytree(package, planted)
        if name == "missing-door":
            (planted / "lib/thinkthen_dart.dart").unlink()
        else:
            manifest = planted / "pubspec.yaml"
            text = manifest.read_text()
            assert "name: thinkthen_dart\n" in text
            manifest.write_text(text.replace("name: thinkthen_dart\n", "name: wrong_dart\n"))
        env = os.environ.copy()
        env.update(THINKTHEN_RELEASE_DART_DIR=str(planted), THINKTHEN_RELEASE_C_DIR=str(native))
        result = subprocess.run([sys.executable, fixture], env=env, capture_output=True,
                                text=True, timeout=15)
        assert result.returncode != 0 and marker in result.stderr, (name, result.stdout, result.stderr)
        print(f"DART_RELEASE_PLANT_REJECTED {name} before backend start")
