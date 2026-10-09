"""Install managed nupkg and native tar in unrelated bwrap roots; count replies outside namespace."""
import collections,json,os,pathlib,re,shutil,sys,tarfile,zipfile,xml.etree.ElementTree as ET
from backend import Backend
from process_group import run
from toolchains import dotnet as resolve_dotnet
R=pathlib.Path(__file__).resolve().parent.parent
sys.path.insert(0,str(R.parents[1]/"conformance/children"))
from portable import one_portable_request
V=re.search(r'<Version>([^<]+)</Version>',(R/'ThinkThen.csproj').read_text())[1]
dotnet=resolve_dotnet()
mode=sys.argv[1];logs=pathlib.Path(sys.argv[2]);work=logs/('independent consumer '+mode)
release_package=os.environ.get('THINKTHEN_RELEASE_NUPKG')
package=os.environ.get('THINKTHEN_RELEASE_NUPKG')
release_c=os.environ.get('THINKTHEN_RELEASE_C_DIR')
if bool(release_package) != bool(release_c): raise AssertionError('installed release needs both package paths')
package=pathlib.Path(release_package) if release_package else R/f'target/scratch/managed/Botassembly.ThinkThen.{V}.nupkg'
if release_package:
 assert {item.name for item in package.parent.iterdir()} == {
  f'Botassembly.ThinkThen.{V}.nupkg','LICENSE','README.md','THINKTHEN-PACKAGE-INPUTS'},'C# wrapper inventory'
 for name in ('LICENSE','README.md'):
  assert (package.parent/name).read_bytes()==(R/name).read_bytes(),f'C# wrapper {name} differs'
 with zipfile.ZipFile(package) as bundle:
  names=set(bundle.namelist())
  assert {'Botassembly.ThinkThen.nuspec','lib/net8.0/ThinkThen.dll','README.md','LICENSE'} <= names,names
  assert 'runtimes/linux-x64/native/libthinkthen.so' in names,names
  assert not any(token in bundle.read(name) for name in names for token in (b'/home/', b'/Users/',b'thinkthen_panic_probe',b'tt-canary-290')), 'private nupkg byte'
  metadata=ET.fromstring(bundle.read('Botassembly.ThinkThen.nuspec'))
  fields={node.tag.rsplit('}',1)[-1]:(node.text or '').strip() for node in metadata.iter()}
  assert fields['id']=='Botassembly.ThinkThen' and fields['version']==V,fields
work.mkdir()
home=work/'home';home.mkdir();cache=work/'cache';cache.mkdir();nuget=work/'nuget';nuget.mkdir()
local=work/'feed';local.mkdir();shutil.copyfile(package,local/f'Botassembly.ThinkThen.{V}.nupkg')
# Only consumer source and package feed are visible; compiler and original source are absent.
shutil.copyfile(R/'tests/source/SessionChecks.cs',work/'SessionChecks.cs');shutil.copyfile(R/'tests/source/Installed.cs',work/'Installed.cs');shutil.copyfile(R/'tests/Installed.csproj',work/'Installed.csproj')
barrier=work/'barrier';barrier.mkdir();server=Backend(barrier)
cmd=['/usr/bin/bwrap','--unshare-all','--share-net','--die-with-parent','--dir','/usr','--dir','/usr/bin','--dir','/opt','--ro-bind',str(dotnet.parent),'/opt/dotnet','--ro-bind','/usr/lib','/usr/lib','--ro-bind','/usr/share','/usr/share','--ro-bind','/lib','/lib','--ro-bind','/lib64','/lib64','--ro-bind','/etc/passwd','/etc/passwd','--ro-bind','/etc/group','/etc/group','--proc','/proc','--dev','/dev','--tmpfs','/tmp','--bind',str(work),'/work','--chdir','/work','--','/opt/dotnet/dotnet','run','--project','/work/Installed.csproj','--configuration','Release','-p:RestoreSources=/work/feed','-p:RestoreIgnoreFailedSources=true']
env={'PATH':'/usr/bin:/bin','HOME':'/work/home','XDG_CONFIG_HOME':'/work/home','XDG_CACHE_HOME':'/work/home','DOTNET_CLI_HOME':'/work/home','NUGET_PACKAGES':'/work/nuget','DOTNET_CLI_TELEMETRY_OPTOUT':'1','DOTNET_SKIP_FIRST_TIME_EXPERIENCE':'1','DOTNET_NOLOGO':'1','DOTNET_MULTILEVEL_LOOKUP':'0','THINKTHEN_CACHE':'/work/cache','THINKTHEN_BASE_URL':f'http://127.0.0.1:{server.server_port}/generic/v1','THINKTHEN_API_KEY':'tt-canary-290'}
if mode=='portable': env['TT_PORTABLE_BATCH']='1'
if mode=='sessions': env['TT_SESSIONS']='1'
try:
 result=run(cmd,timeout=100,env=env)
 (work/'consumer.log').write_bytes(result.stdout+result.stderr)
 counted={'arrivals':server.arrivals,'attempts':server.attempts,'connections':server.connections,'pid':result.pid,'pgid':result.pgid,'exit':result.exit,'signals':result.signals}
 (work/'receipt.json').write_text(json.dumps(counted,indent=2)+'\n')
 assert result.exit==0,(result.exit,result.stdout[-1500:],result.stderr[-1500:])
 bodies=(barrier/'wire-requests.jsonl').read_bytes().splitlines()
 if mode=='sessions':
  assert b'INSTALLED_CSHARP_SESSION_PASS' in result.stdout,result.stdout
  required=collections.Counter(['session-owned','hold-session-task','hold-session-drain'])
  actual=collections.Counter(server.arrivals)
  admitted=collections.Counter((barrier/'admitted-feed').read_text().splitlines())
  assert required <= actual and actual <= required + admitted,counted
  assert server.attempts==server.connections==len(server.arrivals),counted
  print('installed C# owned sessions, bounded Full/Closed, held cancellation, explicit drain, ownership and presence PASS',flush=True)
 elif mode=='portable':
  fixture=R.parents[1]/'specification/fixtures/batching'
  corpus=json.loads((fixture/'portable-records.json').read_text())
  assert b'PORTABLE_BATCH_CSHARP_PASS' in result.stdout,result.stdout
  assert server.arrivals==['packed:'+','.join(corpus['texts'])],counted
  assert server.attempts==server.connections==1 and len(bodies)==1,counted
  one_portable_request(bodies)
  print('C# portable bulk: five typed rows, one exact packed body and one counted send PASS',flush=True)
 else:
  assert b'INSTALLED_TYPED_RESULT_PASS' in result.stdout and b'INSTALLED_CSHARP_CONSUMER_PASS' in result.stdout,result.stdout
  assert collections.Counter(server.arrivals)==collections.Counter(['consumer-csharp','consumer-json']) and server.attempts==server.connections==2,counted
  parsed=[json.loads(line) for line in bodies]
  expected=[{'model':'jev-1.13.0','questions':{'q1':{'type':'noul','instructions':f'The text is "{record}". Is it?'}},'state':'Each question quotes the text it asks about.'} for record in ('consumer-csharp','consumer-json')]
  assert collections.Counter(json.dumps(body,sort_keys=True) for body in parsed)==collections.Counter(json.dumps(body,sort_keys=True) for body in expected),(parsed,expected)
  print('C# observed bodies:',json.dumps(parsed,sort_keys=True),flush=True)
  print('isolated installed consumer',mode,'PASS 2 exact arrivals',flush=True)
finally:server.close()
