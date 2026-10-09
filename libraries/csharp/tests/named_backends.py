"""Public C# settings constructor selects the counted configured backend."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile
from toolchains import dotnet

ROOT = Path(__file__).resolve().parents[3]
PACKAGE = ROOT / "libraries/csharp"
sys.path.insert(0, str(ROOT / "conformance/children"))
from children import child_env
from backend_cases import ROWS, alias, configuration, paths

with tempfile.TemporaryDirectory(prefix="csharp-named-") as scratch:
    env = child_env(HOME=scratch, XDG_CONFIG_HOME=str(Path(scratch) / "config"), XDG_CACHE_HOME=str(Path(scratch) / "cache"), XDG_STATE_HOME=str(Path(scratch) / "state"),
                    DOTNET_CLI_HOME=scratch, DOTNET_CLI_TELEMETRY_OPTOUT="1", DOTNET_NOLOGO="1",
                    NUGET_PACKAGES=str(PACKAGE / "target/scratch/nuget"))
    subprocess.run([str(dotnet()), "build", str(PACKAGE / "tests/Consumer.csproj"), "--configuration", "Release",
                    "--source", str(PACKAGE / "target/scratch/nuget"), "-v", "quiet"], env=env, check=True, timeout=120)
    server = subprocess.Popen([ROOT / "target/debug/conformance-backend"],
                              env=child_env(THINKTHEN_TEST_MARKERS='{"local":"tt-named-loopback"}'),
                              stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True)
    try:
        port = int(server.stdout.readline())
        row = next(row for row in ROWS if row["name"] == "typesafe")
        env.update(THINKTHEN_API_KEY="tt-unnamed-loopback", TYPESAFE_API_KEY="tt-named-loopback",
                   THINKTHEN_BASE_URL=f"http://127.0.0.1:{port}/generic/v1")
        configuration(env, {"local":alias(row, f"http://127.0.0.1:{port}/arm/full/capture/v1")})
        run = subprocess.run([str(dotnet()), str(PACKAGE / "tests/bin/Release/net8.0/Consumer.dll"), "named"],
                             env=env, capture_output=True, text=True, timeout=60)
        assert run.returncode == 0 and "CSHARP_NAMED_BACKEND_PASS" in run.stdout, (run.stdout, run.stderr)
        for command, expected in (("count", 1), ("paths", paths(row["path"])),
                                  ("bearers", {"markers":{"local":1},"absent":0,"unknown":0,"overflow":False})):
            server.stdin.write(command + "\n"); server.stdin.flush()
            assert json.loads(server.stdout.readline()) == expected, command
        server.stdin.write("capture\n"); server.stdin.flush()
        bodies = json.loads(server.stdout.readline())["bodies"]
        assert [json.loads(body) for body in bodies] == [{"state":"Each question quotes the text it asks about.",
            "model":row["model"], "questions":{"q1":{"type":"noul","instructions":'The text is "named-csharp". Is it?'}}}]
        assert "tt-named-loopback" not in run.stdout + run.stderr + json.dumps(bodies)
        print("C# named backend: selected path, bearer, result and secrecy PASS")
    finally:
        server.stdin.close(); server.wait(timeout=10)
