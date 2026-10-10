"""Extract independently installed Swift package/native archives and build in bwrap."""
import collections,fcntl,json,os,pathlib,shutil,sys,tarfile,zipfile,tempfile
from backend import Backend
from process_group import run
R=pathlib.Path(__file__).resolve().parents[2]
mode=sys.argv[1];L=pathlib.Path(sys.argv[2]);W=pathlib.Path(tempfile.mkdtemp(prefix='independent consumer '+mode+' ',dir=L))
if mode == 'owned':
    # A focused source consumer reuses the caller-selected matching native asset.
    # This does not claim an installed SwiftPM binary-package check.
    sys.path.insert(0, str(R.parents[1] / 'conformance/children'))
    from children import child_env
    native = pathlib.Path(sys.argv[3]).resolve()
    (W/'home').mkdir();(W/'barrier').mkdir();(L/'modules').mkdir(exist_ok=True)
    (W/'module.modulemap').write_text('module CThinkThen { header "' + str(R.parent/'c/include/thinkthen.h') + '" export * }\n')
    compiler = shutil.which('swiftc')
    assert compiler, 'Swift compiler unavailable'
    sources = [str(R/'Sources/ThinkThen'/name) for name in ('OwnedJSON.swift','InputsGenerated.swift','ResultsGenerated.swift','ABIGenerated.swift','OwnedInputs.swift','OwnedSession.swift','NativePackage.swift')]
    env = child_env(home=W/'home', TMPDIR=str(W), LANG='C.UTF-8', LC_ALL='C.UTF-8')
    build = run([compiler,'-swift-version','6','-warnings-as-errors','-j','2','-module-cache-path',str(L/'modules'),'-I',str(W),*sources,str(R/'Tests/fixtures/owned_consumer.swift'),'-L',str(native.parent),'-lthinkthen_c','-Xlinker','-rpath','-Xlinker','/native','-Xlinker','-rpath','-Xlinker','/swift/usr/lib/swift/linux','-o',str(W/'consumer')], timeout=180, env=env)
    (L/'owned-build.log').write_bytes(build.stdout + build.stderr)
    assert build.exit == 0, (build.exit, (build.stdout + build.stderr)[-3000:])
    toolchain = str(pathlib.Path(compiler).resolve().parents[2])
    server = Backend(W/'barrier')
    settings = json.dumps({'base_url':f'http://127.0.0.1:{server.server_port}/generic/v1','cache':False,'max_retries':0})
    namespace = ['/usr/bin/bwrap','--unshare-all','--share-net','--die-with-parent','--ro-bind','/usr','/usr','--symlink','usr/bin','/bin','--ro-bind','/lib','/lib','--ro-bind','/lib64','/lib64','--ro-bind',toolchain,'/swift','--proc','/proc','--dev','/dev','--tmpfs','/tmp','--dir','/native','--ro-bind',str(native),'/native/libthinkthen.so.0','--bind',str(W),'/work','--chdir','/work','--']
    env = child_env(home='/work/home', PATH='/swift/usr/bin:/usr/bin:/bin', LANG='C.UTF-8', LC_ALL='C.UTF-8', THINKTHEN_API_KEY='tt-canary-294')
    try:
        result = run(namespace + ['/work/consumer',settings,'/work/barrier'], timeout=30, env=env)
        (L/'owned-consumer.log').write_bytes(result.stdout + result.stderr)
        assert result.exit == 0, (result.exit, (result.stdout + result.stderr)[-3000:])
        assert b'OWNED_SWIFT_PASS' in result.stdout, result.stdout
        assert server.arrivals == ['hold-owned-swift','owned-swift','status-401'] and server.attempts == server.connections == 3, (server.arrivals,server.attempts,server.connections)
        print('Swift source consumer PASS: 3 counted loopback requests, held task cancellation and cleanup before release')
    finally:
        (W/'barrier/release-hold-owned-swift').touch()
        server.close()
        shutil.rmtree(W)
    sys.exit(0)
install=W/'installed package with spaces';install.mkdir()
with zipfile.ZipFile(R/'target/artifacts/thinkthen-swift-0.0.1.zip') as archive:
    for name in archive.namelist():
        dest=install/'package'/name.removeprefix('thinkthen-swift-0.0.1/')
        dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes(archive.read(name))
with tarfile.open(R/'target/artifacts/thinkthen-c-0.0.1-x86_64-linux-gnu.tar.gz') as archive:
    for member in archive:
        dest=install/'native'/member.name;dest.parent.mkdir(parents=True,exist_ok=True)
        if member.issym():dest.symlink_to(member.linkname)
        elif member.isfile():dest.write_bytes(archive.extractfile(member).read())
(W/'home').mkdir();(W/'cache').mkdir();(W/'barrier').mkdir();(W/'empty-tool').write_bytes(b'')
server=Backend(W/'barrier')
swift=os.environ.get('THINKTHEN_SWIFT') or shutil.which('swift')
assert swift and pathlib.Path(swift).is_file(), 'configured Swift executable unavailable'
SW=str(pathlib.Path(swift).resolve().parents[2])
assert pathlib.Path(SW+'/usr/bin/swift').is_file(), SW
# Runtime libraries are host dependencies; only the installed package/native archive
# and the Swift toolchain enter this mount. Cargo/rustc/source export are invisible.
base=['/usr/bin/bwrap','--unshare-all','--share-net','--die-with-parent',
 '--ro-bind','/usr','/usr','--symlink','usr/bin','/bin','--ro-bind','/lib','/lib','--ro-bind','/lib64','/lib64',
 '--ro-bind',SW,'/swift','--ro-bind',str(W/'empty-tool'),'/usr/bin/cargo','--ro-bind',str(W/'empty-tool'),'/usr/bin/rustc','--proc','/proc','--dev','/dev','--tmpfs','/tmp',
 '--bind',str(W),'/work','--chdir','/work','--']
env={'PATH':'/swift/usr/bin:/usr/bin:/bin','HOME':'/work/home','XDG_CACHE_HOME':'/work/cache','SWIFTPM_MODULECACHE_OVERRIDE':'/work/cache/modules','THINKTHEN_CACHE':'/work/cache/native',
 'THINKTHEN_API_KEY':'tt-canary-294','THINKTHEN_BASE_URL':f'http://127.0.0.1:{server.server_port}/generic/v1',
 'LD_LIBRARY_PATH':'/work/installed package with spaces/native/lib'}
def execute(label,args,timeout=120):
    result=run(base+args,timeout=timeout,env=env)
    (W/(label+'.log')).write_bytes(result.stdout+result.stderr)
    receipt={'case':label,'pid':result.pid,'pgid':result.pgid,'exit':result.exit,'signals':result.signals}
    with (W/'receipts.json').open('a') as f:f.write(json.dumps(receipt)+'\n')
    assert result.exit==0,(label,result.exit,(result.stdout+result.stderr)[-2000:])
    return result
usage_lock=None
if mode.startswith('usage-'):
    env['XDG_STATE_HOME']='/work/state'
    if mode=='usage-disabled': env.update(HOME='relative',XDG_STATE_HOME='')
    if mode=='usage-failed':
        state=W/'state/thinkthen';state.mkdir(parents=True,mode=0o700)
        usage_lock=(state/'.lock').open('w');os.chmod(state/'.lock',0o600);fcntl.flock(usage_lock,fcntl.LOCK_EX)
try:
    # Direct namespace checks, not host-path checks: the cargo binary and pinned
    # source must not be addressable from the consumer process.
    execute('namespace',['/bin/sh','-c','test ! -x /usr/bin/cargo && test ! -x /usr/bin/rustc && test ! -e /home && test ! -e /Users && echo NAMESPACE_PASS'],timeout=20)
    pkg='/work/installed package with spaces/package';native='/work/installed package with spaces/native/lib'
    if mode.startswith('usage-'):
        for name in ('native_views.swift','native_consumer.swift'): shutil.copyfile(R/'Tests/fixtures'/name,W/name)
        sources=[str(path).replace(str(W),'/work',1) for path in sorted((install/'package/Sources/ThinkThen').glob('*.swift'))]
        execute('native-build',['/swift/usr/bin/swiftc','-swift-version','6','-warnings-as-errors','-j','2','-module-cache-path','/work/cache/modules','-I',pkg+'/Sources/CThinkThen',*sources,'/work/native_views.swift','/work/native_consumer.swift','-L',native,'-lthinkthen','-Xlinker','-rpath','-Xlinker',native,'-o','/work/consumer'])
        settings=json.dumps({'base_url':env['THINKTHEN_BASE_URL'],'cache':False})
        result=execute('usage',['/work/consumer',mode,settings],timeout=15)
        assert b'USAGE_PASS' in result.stdout,result.stdout
        snapshots=[json.loads(line) for line in result.stdout.splitlines() if line.startswith(b'{')]
        wanted=[] if mode=='usage-disabled' else ['consumer-swift']
        assert server.arrivals==wanted and server.attempts==server.connections==len(wanted),(server.arrivals,server.attempts)
        assert not snapshots if not wanted else len(snapshots)==2 and snapshots[0]==snapshots[1],snapshots
        if wanted: assert snapshots[0]['rows'][0]['data']['decide']['value']['data']['boolean']==1,snapshots[0]
        print('installed Swift',mode,'PASS retained answer/facts; exact requests',len(wanted),flush=True)
        sys.exit(0)
    build=execute('swift-build',['/swift/usr/bin/swift','build','--package-path',pkg,'--scratch-path','/work/swift-build','--jobs','2'],timeout=180)
    # The packaged executable is the independent installed Swift consumer.
    result=execute('installed-example',['/work/swift-build/debug/ThinkThenExample'],timeout=50)
    assert b'INSTALLED_SWIFT_CONSUMER_PASS outcome=yes' in result.stdout,result.stdout
    assert collections.Counter(server.arrivals)==collections.Counter(['consumer-swift','consumer-json']) and server.attempts==2,(server.arrivals,server.attempts)
    bodies=[json.loads(line) for line in (W/'barrier/requests.jsonl').read_text().splitlines()]
    quoted='Each question quotes the text it asks about.'
    assert bodies==[{'state':quoted,'model':'jev-1.13.0','questions':{'q1':{'type':'noul','instructions':'The text is "consumer-swift". Is it?'}}}, {'state':quoted,'model':'jev-1.13.0','questions':{'q1':{'type':'choice','instructions':'The text is "consumer-json". Which?','criteria':{'first':None,'second':None}}}}], bodies
    print('isolated installed Swift consumer',mode,'PASS 2 exact arrivals; pid',result.pid,flush=True)
finally:
    (W/'counts.json').write_text(json.dumps({'arrivals':server.arrivals,'attempts':server.attempts},indent=2)+'\n')
    server.close()
    if usage_lock is not None: usage_lock.close()
    if mode.startswith('usage-'): shutil.rmtree(W)
