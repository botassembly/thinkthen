"""Resolve the .NET SDK executable for local and isolated consumer checks."""
import os
from pathlib import Path
import shutil


def dotnet():
    chosen = os.environ.get("THINKTHEN_DOTNET") or shutil.which("dotnet")
    if not chosen:
        raise RuntimeError("dotnet executable is missing; set THINKTHEN_DOTNET")
    binary = Path(chosen).expanduser().resolve()
    if not binary.is_file() or not os.access(binary, os.X_OK):
        raise RuntimeError(f"dotnet SDK executable is not runnable: {chosen}")
    return binary

if __name__ == "__main__":
    import tempfile
    original = dotnet()
    prior = os.environ.get("THINKTHEN_DOTNET")
    try:
        with tempfile.TemporaryDirectory(prefix="thinkthen-dotnet-path-") as folder:
            alias = Path(folder) / "dotnet"
            alias.symlink_to(original)
            os.environ["THINKTHEN_DOTNET"] = str(alias)
            assert dotnet() == original, "explicit SDK path was ignored"
            os.environ["THINKTHEN_DOTNET"] = str(Path(folder) / "missing")
            try:
                dotnet()
            except RuntimeError:
                pass
            else:
                raise AssertionError("missing SDK path was accepted")
    finally:
        if prior is None:
            os.environ.pop("THINKTHEN_DOTNET", None)
        else:
            os.environ["THINKTHEN_DOTNET"] = prior
    print("C# SDK path override and missing-path refusal PASS")
