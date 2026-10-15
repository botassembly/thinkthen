from pathlib import Path
import importlib.util,os,shutil
root=Path(__file__).resolve().parents[2]; out=Path(checkpoint_env['GATE_OUTPUT']); scripts=Path(__file__).resolve().parent
env=checkpoint_env
def run(name,command,cwd=root,extra=None):
 return phase(name,command,cwd,extra)['exit']
native=root/'target/debug/libthinkthen_c.so'; static=root/'target/debug/libthinkthen_c.a'; header=root/'libraries/c/include/thinkthen.h'
run('native-c',['cargo','build','--locked','--offline','--manifest-path','libraries/c/Cargo.toml','--lib','-j2'])
run('c-fmt',['cargo','fmt','--manifest-path','libraries/c/Cargo.toml','--check'])
run('c-clippy',['cargo','clippy','--locked','--offline','--manifest-path','libraries/c/Cargo.toml','--all-targets','--','-D','warnings'])
run('native-c-abi',['python3','sdlc/scripts/check-c-exports.py',str(header),str(native)])
run('root-library',['cargo','build','--locked','--offline','--package','thinkthen','--lib','-j2'])
run('rust-library',['cargo','build','--locked','--offline','--manifest-path','libraries/rust/Cargo.toml','--examples','-j2'])
run('polars-feature',['cargo','build','--locked','--offline','--package','thinkthen','--features','polars','--lib','-j2'],extra={'CARGO_TARGET_DIR':str(root/'target/polars')})
run('polars-library',['cargo','build','--locked','--offline','--manifest-path','libraries/polars/consumer/Cargo.toml','-j2'],extra={'CARGO_TARGET_DIR':str(root/'target/polars')})
run('python',['cargo','build','--locked','--offline','--manifest-path','libraries/python/Cargo.toml','--lib','-j2'])
run('python-import',['python3','-c',"import importlib.util; p='target/debug/lib_thinkthen.so'; s=importlib.util.spec_from_file_location('_thinkthen',p); m=importlib.util.module_from_spec(s); s.loader.exec_module(m); print('native Python import passed')"])
run('ruby',['cargo','build','--locked','--offline','--manifest-path','libraries/ruby/Cargo.toml','--lib','-j2'])
run('typescript',['cargo','build','--locked','--offline','--manifest-path','libraries/typescript/Cargo.toml','--lib','-j2'])
if (root/'target/debug/libthinkthen_typescript.so').is_file():
 shutil.copyfile(root/'target/debug/libthinkthen_typescript.so',root/'libraries/typescript/target/thinkthen-core-checkpoint.node') if (root/'libraries/typescript/target').is_dir() else None
run('r-native',['cargo','build','--locked','--offline','--manifest-path','libraries/r/thinkthen/src/rust/Cargo.toml','--lib','-j2'])
if (root/'libraries/jvm/target/core-wave').exists(): shutil.rmtree(root/'libraries/jvm/target/core-wave')
run('jvm-java-kotlin-scala',['bash','libraries/jvm/build.sh','--out',str(root/'libraries/jvm/target/core-wave'),'--native',str(native),'--target','x86_64-unknown-linux-gnu'])
(root/'libraries/csharp/target/feed').mkdir(parents=True,exist_ok=True)
cpp=root/'libraries/cpp/target'; (cpp/'include/thinkthen').mkdir(parents=True,exist_ok=True); shutil.copyfile(header,cpp/'include/thinkthen/thinkthen.h')
run('cpp-configure',['cmake','-S','libraries/cpp','-B',str(cpp/'package-build'),'-DCMAKE_BUILD_TYPE=Release','-DCMAKE_INSTALL_PREFIX='+str(cpp/'install'),'-DTHINKTHEN_C_HEADER='+str(header),'-DTHINKTHEN_NATIVE_SHARED='+str(native),'-DTHINKTHEN_NATIVE_STATIC='+str(static)])
run('cpp',['cmake','--build',str(cpp/'package-build'),'--parallel','2'])
run('cpp-install',['cmake','--install',str(cpp/'package-build')])
ada=root/'libraries/ada/target'; ada.mkdir(parents=True,exist_ok=True)
run('ada',['gnatmake','-q','-gnat2022','-j2','-I'+str(root/'libraries/ada/src'),str(root/'libraries/ada/examples/smoke.adb'),'-D',str(ada),'-o',str(ada/'core-wave-consumer'),'-largs',str(native),'-Wl,-rpath,'+str(native.parent)])
co=root/'libraries/cobol/target'; (co/'include').mkdir(parents=True,exist_ok=True); shutil.copyfile(header,co/'include/thinkthen.h')
run('cobol',['cobc','-x','-free','-fstatic-call','-fno-gen-c-decl-static-call','-I',str(root/'libraries/cobol/copybooks'),'-A','-I'+str(co/'include')+' -include '+str(root/'libraries/cobol/src/tt_session.h')+' -Wno-incompatible-pointer-types','-o',str(co/'core-wave-consumer'),str(root/'libraries/cobol/examples/session.cob'),str(root/'libraries/cobol/src/tt_session.c'),str(root/'libraries/cobol/src/tt_requests_generated.c'),'-Q',str(native)+' -Wl,-rpath,'+str(native.parent)])
run('php',['/usr/bin/php8.3','-n','-d','extension=ffi','-d','ffi.enable=1','-r',"require 'libraries/php/autoload.php'; FFI::cdef(file_get_contents('libraries/php/src/session/ffi_generated.h'), 'target/debug/libthinkthen_c.so'); echo 'PHP native ABI import passed\n';"])

native=root/'target/debug/libthinkthen_c.so'; static=root/'target/debug/libthinkthen_c.a'; header=root/'libraries/c/include/thinkthen.h'
# Build-only owners share the lane's compiled native engine.
go=root/'libraries/go/target'; go.mkdir(parents=True,exist_ok=True); (go/'native/include').mkdir(parents=True,exist_ok=True); (go/'native/lib').mkdir(exist_ok=True)
shutil.copyfile(header,go/'native/include/thinkthen.h')
run('go-localize',['sh','libraries/c/localize.sh',str(static),str(go/'native/lib/libthinkthen.a')])
if (go/'core-wave').exists(): shutil.rmtree(go/'core-wave')
run('go-stage',['python3','libraries/go/fixtures/package_owned.py',str(root/'libraries/go'),str(go/'native'),str(go/'core-wave')])
package=go/'core-wave'
if package.is_dir():
 (package/'example').mkdir(exist_ok=True); shutil.copyfile(root/'libraries/go/examples/decide/main.go',package/'example/main.go')
 run('go',['go','build','-p','1','-buildvcs=false','-o',str(go/'core-wave-example'),'./example'],cwd=package)
zig=root/'libraries/zig/target'; (zig/'core-wave-include').mkdir(parents=True,exist_ok=True); shutil.copyfile(header,zig/'core-wave-include/thinkthen.h')
run('zig',['zig','build-exe','-j2','-fllvm','-flld',str(static),'--cache-dir',str(zig/'scratch/core-wave-cache'),'--global-cache-dir',str(zig/'cache'),'--dep','thinkthen','-Mroot='+str(root/'libraries/zig/examples/smoke.zig'),'-I',str(zig/'core-wave-include'),'-Mthinkthen='+str(root/'libraries/zig/src/thinkthen.zig'),'-lc','-lgcc_s','-lutil','-lrt','-lpthread','-lm','-ldl','-femit-bin='+str(zig/'core-wave-consumer')])
swift=root/'libraries/swift/target'; (swift/'CThinkThen/include').mkdir(parents=True,exist_ok=True)
shutil.copyfile(header,swift/'CThinkThen/include/thinkthen.h'); shutil.copyfile(root/'libraries/swift/Sources/CThinkThen/include/loader.h',swift/'CThinkThen/include/loader.h'); shutil.copyfile(root/'libraries/swift/Sources/CThinkThen/module.modulemap',swift/'CThinkThen/module.modulemap')
run('swift-linux',['swiftc','-swift-version','6','-j2','-emit-library','-I',str(swift/'CThinkThen'),*map(str,sorted((root/'libraries/swift/Sources/ThinkThen').glob('*.swift'))),str(native),'-o',str(swift/'libThinkThen-core-wave.so')])
typescript=root/'libraries/typescript/target'; typescript.mkdir(exist_ok=True)
if (root/'target/debug/libthinkthen_typescript.so').is_file():
 shutil.copyfile(root/'target/debug/libthinkthen_typescript.so',typescript/'thinkthen-core-checkpoint.node')
 run('typescript-import',['node','-e',"require('./libraries/typescript/target/thinkthen-core-checkpoint.node'); console.log('native Node addon import passed')"])
# Fresh stage follows installed_native_assets.py package, example and fixture preparation.
run('dart-stage',['python3',str(scripts/'release-stage-dart.py')])
pkg=root/'libraries/dart/target/core-wave';dart=root/'libraries/dart/target'
run('dart-dependencies',[env['TT_DART'],'pub','get','--offline'],cwd=pkg)
run('dart',[env['TT_DART'],'build','cli','-t','bin/main.dart','-o',str(dart/'core-wave-build')],cwd=pkg)
spec=importlib.util.spec_from_file_location('assets',root/'libraries/dart/checks/native_assets.py');assets=importlib.util.module_from_spec(spec);spec.loader.exec_module(assets)
flutter=dart/'core-wave-flutter'
if flutter.exists():shutil.rmtree(flutter)
shutil.copytree(root/'libraries/dart/flutter',flutter,ignore=shutil.ignore_patterns('build','.dart_tool','target'))
assets.configure(pkg,native,[flutter,flutter/'example'])
run('flutter-dependencies',[env['TT_FLUTTER'],'pub','get','--offline'],cwd=flutter/'example')
run('flutter-linux',[env['TT_FLUTTER'],'build','linux','--no-pub'],cwd=flutter/'example')

native=root/'target/debug/libthinkthen_c.so'; static=root/'target/debug/libthinkthen_c.a'
ruby=root/'libraries/ruby/target'; ruby.mkdir(exist_ok=True); shutil.copyfile(root/'target/debug/libthinkthen_ruby.so',ruby/'thinkthen.so')
run('ruby-import-correct-name',[env['RUBY'],'-e',"require File.expand_path('libraries/ruby/target/thinkthen.so'); puts 'native Ruby import passed'"])
run('csharp-offline-build',['dotnet','build','libraries/csharp/ThinkThen.csproj','-c','Release','-p:RestoreSources='+str(root/'libraries/csharp/target/feed'),'-o','libraries/csharp/target/core-wave','-v','quiet','--disable-build-servers','-p:UseSharedCompilation=false','-nodeReuse:false'])
ts=root/'libraries/typescript/target'; ts.mkdir(exist_ok=True); shutil.copyfile(root/'target/debug/libthinkthen_typescript.so',ts/'thinkthen-core-checkpoint.node')
# Copy the complete existing build consumer, including both local imports.
pkg=root/'libraries/dart/target/core-wave'; shutil.copytree(root/'libraries/dart/checks/session_consumer/bin',pkg/'bin',dirs_exist_ok=True)
python=root/'libraries/python/target/core-wave'; (python/'thinkthen').mkdir(parents=True,exist_ok=True); shutil.copytree(root/'libraries/python/thinkthen',python/'thinkthen',dirs_exist_ok=True); shutil.copyfile(root/'target/debug/lib_thinkthen.so',python/'thinkthen/_thinkthen.so')
run('python-pandas-polars-import',[str(root/'target/coverage-baseline/stage-python/venv/bin/python'),'-c',"import sys; sys.path.insert(0,'libraries/python/target/core-wave'); import thinkthen, thinkthen.pandas, thinkthen.polars; print('Python, pandas and Polars import passed')"])
r=root/'libraries/r/target'; r.mkdir(exist_ok=True); shutil.copyfile(root/'libraries/r/thinkthen/src/entrypoint.c',r/'entrypoint.c')
run('r-bridge',['R','CMD','SHLIB','entrypoint.c','-o','thinkthen.so'],cwd=r,extra={'PKG_LIBS':'-L'+str(root/'target/debug')+' -lthinkthen_r -lpthread -ldl -lm'})
run('r-bridge-import',['Rscript','--vanilla','-e',"dyn.load('libraries/r/target/thinkthen.so'); cat('native R bridge import passed\n')"])

native=root/'target/debug/libthinkthen_c.so'
c=root/'libraries/c/target'; c.mkdir(exist_ok=True)
run('c-consumer',['cc','-std=c11','-Wall','-Wextra','-Werror','-I'+str(root/'libraries/c/include'),str(root/'libraries/c/examples/session.c'),str(native),'-Wl,-rpath,'+str(native.parent),'-o',str(c/'core-wave-consumer')])
run('typescript-declarations',['tsc','--noEmit','--strict','--target','ES2022','--module','NodeNext','--moduleResolution','NodeNext','libraries/typescript/index.d.ts'])


run('objective-c-platform',['sh','libraries/objective-c/check.sh'])

# The installed SQLite diagnostic comparison needs the unchanged combined extension.
run('sqlite-baseline',['cargo','build','--locked','--offline','--manifest-path','databases/sqlite/Cargo.toml','--lib','-j2'])