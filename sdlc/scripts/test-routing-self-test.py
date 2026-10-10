#!/usr/bin/env python3
"""Exercise suite routing through the real entrypoints without running build jobs."""
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest


class SuiteRouting(unittest.TestCase):
    def test_binding_builds_run_only_in_release_and_fail_the_release_on_error(self):
        source = Path(__file__).resolve().parent
        with tempfile.TemporaryDirectory(prefix="thinkthen-suite-routing-") as folder:
            root = Path(folder)
            scripts = root / "sdlc/scripts"
            scripts.mkdir(parents=True)
            for name in ("test", "test-full-cases", "scratch.sh"):
                shutil.copyfile(source / name, scripts / name)
            (scripts / "heavy-lock").write_text("")
            # Use the real scratch ownership/cleanup, with no host configuration reads.
            with (scripts / "scratch.sh").open("a") as stream:
                stream.write("\nusage_guard() { :; }\nconfig_home() { :; }\n")
            fake = root / "bin"
            fake.mkdir()
            log = root / "calls"
            command = '''#!/bin/sh
printf '%s %s\\n' "${0##*/}" "$*" >> "$ROUTING_LOG"
case ${0##*/} in
uname) echo Darwin ;;
sh) case $* in *sdlc/scripts/smoke*) exit "$ROUTING_SMOKE_CODE" ;; esac ;;
esac
'''
            for name in ("cargo", "python3", "sh", "uname"):
                path = fake / name
                path.write_text(command)
                path.chmod(0o755)
            demo = root / "demos/16-triage-pipeline/self-test"
            demo.parent.mkdir(parents=True)
            demo.write_text("#!/bin/sh\nexit 0\n")
            demo.chmod(0o755)
            env = {"PATH": f"{fake}:/usr/bin:/bin", "HOME": folder,
                   "ROUTING_LOG": str(log), "ROUTING_SMOKE_CODE": "0"}

            def run(entry, *args):
                log.write_text("")
                result = subprocess.run(["/bin/sh", str(scripts / entry), *args],
                                        cwd=root, env=env, capture_output=True,
                                        text=True, timeout=5)
                return result, log.read_text().splitlines()

            routine, calls = run("test")
            self.assertEqual(routine.returncode, 0, routine.stderr)
            self.assertFalse(any("sdlc/scripts/smoke" in call or "--ignored" in call
                                 or "sdlc/scripts/surfaces" in call for call in calls), calls)
            self.assertEqual(sum("cargo nextest run" in call for call in calls), 2)
            release, calls = run("test-full-cases", "--run")
            self.assertEqual(release.returncode, 0, release.stderr)
            self.assertEqual(sum("sdlc/scripts/smoke" in call for call in calls), 1)
            self.assertEqual(sum("--ignored" in call for call in calls), 2)
            self.assertEqual(sum("sdlc/scripts/surfaces --full-functional" in call
                                 for call in calls), 1)
            env["ROUTING_SMOKE_CODE"] = "9"
            failed, _ = run("test-full-cases", "--run")
            self.assertEqual(failed.returncode, 1, failed.stderr)
            self.assertIn("a binding smoke failed (exit 9)", failed.stderr)


if __name__ == "__main__":
    unittest.main()
