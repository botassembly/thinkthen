"""Refuse missing PHP product source and wrong Composer identity before a call."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

package = Path(sys.argv[1])
native = Path(sys.argv[2])
fixture = Path(__file__).with_name("portable_batch.py")
for name, marker in (("missing-autoload", "PHP_ARCHIVE_MEMBERS"),
                     ("wrong-composer-name", "PHP_COMPOSER_IDENTITY")):
    with tempfile.TemporaryDirectory(prefix="thinkthen-php-release-plant-") as scratch:
        planted = Path(scratch) / "package"
        shutil.copytree(package, planted)
        if name == "missing-autoload":
            (planted / "autoload.php").unlink()
        else:
            metadata = json.loads((planted / "composer.json").read_text())
            metadata["name"] = "wrong/thinkthen"
            (planted / "composer.json").write_text(json.dumps(metadata))
        env = os.environ.copy()
        env.update(THINKTHEN_RELEASE_PHP_DIR=str(planted), THINKTHEN_RELEASE_C_DIR=str(native))
        result = subprocess.run([sys.executable, fixture], env=env, capture_output=True,
                                text=True, timeout=15)
        assert result.returncode != 0 and marker in result.stderr, (name, result.stdout, result.stderr)
        print(f"PHP_RELEASE_PLANT_REJECTED {name} before backend start")
