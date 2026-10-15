from pathlib import Path
import importlib.util,shutil
root=Path(__file__).resolve().parents[2];source=root/'libraries/dart';pkg=source/'target/core-wave'
if pkg.exists():shutil.rmtree(pkg)
pkg.mkdir(parents=True)
spec=importlib.util.spec_from_file_location('assets',source/'checks/native_assets.py');assets=importlib.util.module_from_spec(spec);spec.loader.exec_module(assets)
for member in assets.package_members(source):
 target=pkg/member;target.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(source/member,target)
assets.configure(pkg,root/'target/debug/libthinkthen_c.so',[pkg])
shutil.copytree(source/'checks/session_consumer/bin',pkg/'bin')
shutil.copyfile(source/'example/main.dart',pkg/'bin/shipped_example.dart')
print('fresh Dart stage contains current documented example and exact-source native asset')

spec=importlib.util.spec_from_file_location("fixture_types",source/"checks/fixture_types.py");fixture=importlib.util.module_from_spec(spec);spec.loader.exec_module(fixture);fixture.write(pkg/"bin/fixture_values_generated.dart")
import base64
p=pkg/"bin/data_cases.dart";p.write_text(p.read_text().replace("__CANONICAL_IMAGE__",base64.b64encode((root/"specification/fixtures/images/red.png").read_bytes()).decode()))
