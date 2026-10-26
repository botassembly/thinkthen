"""Exercise the installed C++ constructor against the shared native ABI cases."""
import json
import platform
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

prefix, fixture_manifest = map(Path, sys.argv[1:])
manifest = json.loads(fixture_manifest.read_text())
with tempfile.TemporaryDirectory() as directory:
    root = Path(directory)
    installed = root / "installed"
    shutil.copytree(prefix, installed, symlinks=True)
    source = root / "source"
    source.mkdir()
    shutil.copy2(Path(__file__).with_name("abi.cpp"), source / "main.cpp")
    (source / "CMakeLists.txt").write_text('''cmake_minimum_required(VERSION 3.20)
project(AbiConsumer LANGUAGES CXX)
find_package(ThinkThenCpp CONFIG REQUIRED)
add_executable(consumer main.cpp)
target_link_libraries(consumer PRIVATE ThinkThen::Cpp)
''')
    build = root / "build"
    subprocess.run(["cmake", "-S", str(source), "-B", str(build),
                    f"-DCMAKE_PREFIX_PATH={installed};{Path.home()}/.local/share/thinkthen-build-tools/nlohmann-json/3.11.3"], check=True)
    subprocess.run(["cmake", "--build", str(build)], check=True)
    engines = [p for p in (installed / "lib").glob("libthinkthen.*") if not p.is_symlink()]
    assert len(engines) == 1, engines
    subprocess.run([str(Path(__file__).resolve().parents[3] / "scripts/loader-failure-check"),
                    str(engines[0]), "--attempted", str(engines[0]) if platform.system() == "Darwin" else engines[0].name,
                    "--", str(build / "consumer"), "client"], check=True)
    for case in manifest["cases"]:
        shutil.copyfile(case["library"], engines[0])
        if "missing" in case:
            # Direct C++ consumers require these exports at their link boundary.
            (build / "consumer").unlink(missing_ok=True)
            result = subprocess.run(["cmake", "--build", str(build)], capture_output=True, text=True)
            assert result.returncode != 0 and case["missing"] in result.stderr, (case["name"], result)
        else:
            for first in ("client", "key", "question", "threshold"):
                result = subprocess.run([str(build / "consumer"), first], capture_output=True, text=True)
                expected = "accepted" if case["accept"] else "usage.abi"
                assert result.returncode == 0 and result.stdout.strip() == expected, (case["name"], first, result)
        print(f'installed C++ ABI: {case["name"]} passed')
