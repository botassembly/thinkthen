"""Selected setup limits reach the Python consumer before any transport."""
import json
import sys
import subprocess
from conftest import child_env, run


def test_captured_setup_profile_refuses_and_explicit_profile_wins(backend, tmp_path):
    env = child_env(backend, tmp_path, THINKTHEN_BACKEND="small", LOCAL_SETUP_KEY="fake-0400")
    env["HOME"] = str(tmp_path)
    env["XDG_CONFIG_HOME"] = str(tmp_path / "config")
    env["APPDATA"] = str(tmp_path / "config")
    folder = tmp_path / ("Library/Application Support/thinkthen" if sys.platform == "darwin" else "config/thinkthen")
    folder.mkdir(parents=True)
    (folder / "config.json").write_text(json.dumps({
        "schema": "thinkthen.config/1", "backends": {"small": {
            "url": backend.base(), "key_env": "LOCAL_SETUP_KEY", "model": "m",
            "profile": {"schema": "thinkthen.backend-profile/1", "name": "small", "max_evidence_bytes": 3}}}}))
    if sys.platform == "win32":
        subprocess.run(["powershell.exe", "-NoProfile", "-NonInteractive", "-Command",
                        "$sid=[System.Security.Principal.WindowsIdentity]::GetCurrent().User.Value; "
                        "$a=Get-Acl -LiteralPath $env:FIXTURE_CONFIG; "
                        "$a.SetSecurityDescriptorSddlForm(\"O:${sid}D:P(A;;FA;;;$sid)(A;;FA;;;SY)\"); "
                        "Set-Acl -LiteralPath $env:FIXTURE_CONFIG -AclObject $a"],
                       env=env | {"FIXTURE_CONFIG": str(folder / "config.json")}, check=True)
    refused = run('''
        import thinkthen as tt
        try:
            tt.Engine(cache=False).tag("Which labels fit?", "alpha", labels=["one", "two"])
        except tt.ThinkThenError as error:
            print(type(error).__name__, error)
        else:
            raise AssertionError("the selected profile must refuse")
    ''', env)
    assert "profile small" in refused, refused
    assert "fake-0400" not in refused
    assert backend.count() == 0
    profile = tmp_path / "explicit.json"
    profile.write_text(json.dumps({"schema": "thinkthen.backend-profile/1", "name": "explicit", "max_evidence_bytes": 100}))
    answered = run(f'''
        import thinkthen as tt
        print(tt.Engine(cache=False, profile={str(profile)!r}).decide("a refund?", "alpha").value)
    ''', env)
    assert answered.strip() == "True"
    assert backend.count() == 1
