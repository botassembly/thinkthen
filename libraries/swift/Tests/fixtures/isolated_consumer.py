"""Extract independently installed Swift package/native archives and build in bwrap."""
import collections,json,os,pathlib,shutil,sys,tarfile,zipfile,tempfile
from backend import Backend
from process_group import run
R=pathlib.Path(__file__).resolve().parents[2]
mode=sys.argv[1];L=pathlib.Path(sys.argv[2]);W=pathlib.Path(tempfile.mkdtemp(prefix='independent consumer '+mode+' ',dir=L))
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
try:
    # Direct namespace checks, not host-path checks: the cargo binary and pinned
    # source must not be addressable from the consumer process.
    execute('namespace',['/bin/sh','-c','test ! -x /usr/bin/cargo && test ! -x /usr/bin/rustc && test ! -e /home/ian/workspace/repos/thinkthen && test ! -e /home/ian/workspace/experiments && echo NAMESPACE_PASS'],timeout=20)
    pkg='/work/installed package with spaces/package';native='/work/installed package with spaces/native/lib'
    build=execute('swift-build',['/swift/usr/bin/swift','build','--package-path',pkg,'--scratch-path','/work/swift-build','--jobs','2','-Xlinker','-L','-Xlinker',native,'-Xlinker','-rpath','-Xlinker',native],timeout=180)
    # The packaged executable is the independent installed Swift consumer.
    result=execute('installed-example',['/work/swift-build/debug/ThinkThenExample'],timeout=50)
    assert b'INSTALLED_SWIFT_CONSUMER_PASS outcome=yes' in result.stdout,result.stdout
    assert collections.Counter(server.arrivals)==collections.Counter(['consumer-swift','consumer-json']) and server.attempts==2,(server.arrivals,server.attempts)
    bodies=[json.loads(line) for line in (W/'barrier/requests.jsonl').read_text().splitlines()]
    assert bodies==[{'state':'consumer-swift','model':'jev-1.13.0','questions':{'q1':{'type':'noul','instructions':'Is it?'}}}, {'state':'consumer-json','model':'jev-1.13.0','questions':{'q1':{'type':'choice','instructions':'Which?','criteria':{'first':None,'second':None}}}}], bodies
    print('isolated installed Swift consumer',mode,'PASS 2 exact arrivals; pid',result.pid,flush=True)
finally:
    (W/'counts.json').write_text(json.dumps({'arrivals':server.arrivals,'attempts':server.attempts},indent=2)+'\n')
    server.close()
