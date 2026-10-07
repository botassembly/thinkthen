"""Exercise the private Flutter app from separately unpacked Dart, Flutter and C files."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
from urllib.parse import unquote, urlparse

dart_root, flutter_stage, native_root = (Path(arg).resolve() for arg in sys.argv[1:])
flutter_root = flutter_stage / "flutter"
expected = {
    "README.md", "pubspec.yaml", "pubspec.lock", "lib/thinkthen_flutter.dart", "lib/thinkthen_complete_flutter.dart",
    "example/.metadata", "example/README.md", "example/pubspec.yaml",
    "example/pubspec.lock", "example/analysis_options.yaml", "example/lib/main.dart",
    "example/linux/CMakeLists.txt", "example/linux/flutter/CMakeLists.txt",
    "example/linux/flutter/generated_plugin_registrant.h",
    "example/linux/flutter/generated_plugin_registrant.cc",
    "example/linux/flutter/generated_plugins.cmake",
    "example/linux/runner/CMakeLists.txt", "example/linux/runner/main.cc",
    "example/linux/runner/my_application.cc", "example/linux/runner/my_application.h",
}
members = {str(path.relative_to(flutter_root)) for path in flutter_root.rglob("*") if path.is_file()}
assert members == expected and not any(path.is_symlink() for path in flutter_stage.rglob("*")), ("FLUTTER_ARCHIVE_MEMBERS", members)
flutter_manifest = flutter_stage / "THINKTHEN-PACKAGE-INPUTS"
dart_manifest = dart_root / "THINKTHEN-PACKAGE-INPUTS"
assert flutter_manifest.is_file() and dart_manifest.is_file(), "FLUTTER_ARCHIVE_MANIFESTS"
assert {str(path.relative_to(flutter_stage)) for path in flutter_stage.rglob("*") if path.is_file()} == {
    "THINKTHEN-PACKAGE-INPUTS", *("flutter/" + name for name in expected)
}, "FLUTTER_ARCHIVE_ROOT"
wrapper_spec = (flutter_root / "pubspec.yaml").read_text()
example_spec = (flutter_root / "example/pubspec.yaml").read_text()
assert "name: thinkthen_flutter\n" in wrapper_spec and "publish_to: none\n" in wrapper_spec and "path: ..\n" in wrapper_spec, "FLUTTER_PRIVATE_WRAPPER"
assert "name: thinkthen_flutter_example\n" in example_spec and "publish_to: none\n" in example_spec and "path: ..\n" in example_spec and "path: ../..\n" in example_spec, "FLUTTER_PRIVATE_EXAMPLE"
assert "name: thinkthen_dart\n" in (dart_root / "pubspec.yaml").read_text(), "FLUTTER_DART_IDENTITY"
dart_manifest_bytes = dart_manifest.read_bytes()
shutil.copytree(flutter_root, dart_root / "flutter")
assert dart_manifest.read_bytes() == dart_manifest_bytes and flutter_manifest.is_file(), "FLUTTER_MANIFEST_OVERWRITE"

work = dart_root.parent
for name in ("home", "cache", "config", "logs"):
    (work / name).mkdir(exist_ok=True)
env = os.environ.copy()
env.update(HOME=str(work / "home"), XDG_CACHE_HOME=str(work / "cache"),
           XDG_CONFIG_HOME=str(work / "config"),
           TT_FLUTTER_SOURCE=str(dart_root / "flutter"), TT_EMBEDDER_LOGS=str(work / "logs"),
           TT_NATIVE_LIBRARY=str(native_root / "lib/libthinkthen.so"))
flutter = env["TT_FLUTTER"]
for project in (dart_root / "flutter", dart_root / "flutter/example"):
    result = subprocess.run([flutter, "pub", "get", "--offline"], cwd=project, env=env,
                            capture_output=True, text=True, timeout=120)
    assert result.returncode == 0, ("FLUTTER_OFFLINE_PUB", project, result.stdout, result.stderr)

def package_root(config, name):
    entry = next(item for item in json.loads(config.read_text())["packages"] if item["name"] == name)
    uri = urlparse(entry["rootUri"])
    assert uri.scheme in ("", "file"), entry
    return (Path(unquote(uri.path)) if uri.scheme == "file" else config.parent / unquote(uri.path)).resolve()

wrapper_config = dart_root / "flutter/.dart_tool/package_config.json"
app_config = dart_root / "flutter/example/.dart_tool/package_config.json"
ffi = Path(env["PUB_CACHE"]).resolve() / "hosted/pub.dev/ffi-2.2.0"
for config in (wrapper_config, app_config):
    assert package_root(config, "thinkthen_dart") == dart_root, ("FLUTTER_DART_PATH", config)
    assert package_root(config, "ffi") == ffi, ("FLUTTER_FFI_PATH", config)
assert package_root(app_config, "thinkthen_flutter") == dart_root / "flutter", "FLUTTER_WRAPPER_PATH"
result = subprocess.run([sys.executable, str(Path(__file__).with_name("embedder.py"))],
                        env=env, capture_output=True, text=True, timeout=320)
assert result.returncode == 0 and "FLUTTER_EMBEDDER_PASS" in result.stdout, (result.stdout, result.stderr)
print(f"Flutter archive app: {dart_root / 'flutter/example/build/linux/x64/release/bundle/thinkthen_flutter_example'}")
print("Flutter installed: legacy and complete decoded bodies, two counted sends, cancellation and usage zero sends")
