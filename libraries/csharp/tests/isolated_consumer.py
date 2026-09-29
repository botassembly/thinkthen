"""Install managed nupkg and native tar in unrelated bwrap roots; count replies outside namespace."""
import collections,json,pathlib,shutil,sys,tarfile,zipfile
from backend import Backend
from process_group import run
R=pathlib.Path(__file__).resolve().parent.parent
mode=sys.argv[1];logs=pathlib.Path(sys.argv[2]);work=logs/('independent consumer '+mode);work.mkdir()
install=work/'install with spaces';install.mkdir();home=work/'home';home.mkdir();cache=work/'cache';cache.mkdir();nuget=work/'nuget';nuget.mkdir()
with tarfile.open(R/'target/artifacts/thinkthen-c-0.0.1-x86_64-linux-gnu.tar.gz') as archive:
 for member in archive:
  dest=install/'native'/member.name;dest.parent.mkdir(parents=True,exist_ok=True)
  if member.issym():dest.symlink_to(member.linkname)
  elif member.isfile():dest.write_bytes(archive.extractfile(member).read())
local=work/'feed';local.mkdir();shutil.copyfile(R/'target/scratch/managed/Botassembly.ThinkThen.0.0.1.nupkg',local/'Botassembly.ThinkThen.0.0.1.nupkg')
# Only consumer source and package feed are visible; compiler and original source are absent.
shutil.copyfile(R/'tests/source/Installed.cs',work/'Installed.cs');shutil.copyfile(R/'tests/Installed.csproj',work/'Installed.csproj')
barrier=work/'barrier';barrier.mkdir();server=Backend(barrier)
cmd=['/usr/bin/bwrap','--unshare-all','--share-net','--die-with-parent','--dir','/usr','--dir','/usr/bin','--symlink','../lib/dotnet/dotnet','/usr/bin/dotnet','--ro-bind','/usr/lib','/usr/lib','--ro-bind','/usr/share','/usr/share','--ro-bind','/lib','/lib','--ro-bind','/lib64','/lib64','--ro-bind','/etc/passwd','/etc/passwd','--ro-bind','/etc/group','/etc/group','--proc','/proc','--dev','/dev','--tmpfs','/tmp','--bind',str(work),'/work','--chdir','/work','--','/usr/bin/dotnet','run','--project','/work/Installed.csproj','--configuration','Release','-p:RestoreSources=/work/feed','-p:RestoreIgnoreFailedSources=true']
env={'PATH':'/usr/bin:/bin','HOME':'/work/home','XDG_CONFIG_HOME':'/work/home','XDG_CACHE_HOME':'/work/home','DOTNET_CLI_HOME':'/work/home','NUGET_PACKAGES':'/work/nuget','DOTNET_CLI_TELEMETRY_OPTOUT':'1','DOTNET_SKIP_FIRST_TIME_EXPERIENCE':'1','DOTNET_NOLOGO':'1','DOTNET_MULTILEVEL_LOOKUP':'0','LD_LIBRARY_PATH':'/work/install with spaces/native/lib','THINKTHEN_CACHE':'/work/cache','THINKTHEN_BASE_URL':f'http://127.0.0.1:{server.server_port}/generic/v1','THINKTHEN_API_KEY':'tt-canary-290'}
try:
 result=run(cmd,timeout=100,env=env)
 (work/'consumer.log').write_bytes(result.stdout+result.stderr)
 counted={'arrivals':server.arrivals,'attempts':server.attempts,'connections':server.connections,'pid':result.pid,'pgid':result.pgid,'exit':result.exit,'signals':result.signals}
 (work/'receipt.json').write_text(json.dumps(counted,indent=2)+'\n')
 assert result.exit==0 and b'INSTALLED_CSHARP_CONSUMER_PASS' in result.stdout,(result.exit,result.stdout[-1500:],result.stderr[-1500:])
 assert collections.Counter(server.arrivals)==collections.Counter(['consumer-csharp','consumer-json']) and server.attempts==server.connections==2,counted
 print('isolated installed consumer',mode,'PASS 2 exact arrivals',flush=True)
finally:server.close()
