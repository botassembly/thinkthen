"""Install managed nupkg and native tar in unrelated bwrap roots; count replies outside namespace."""
import collections,json,os,pathlib,re,shutil,sys,zipfile,xml.etree.ElementTree as ET
from process_group import run
from toolchains import dotnet as resolve_dotnet
R=pathlib.Path(__file__).resolve().parent.parent
sys.path.insert(0,str(R.parents[1]/"conformance/children"))
from backend import Backend
from portable import one_portable_request
from usage import run_usage
from native_guard import normalized_bytes
V=re.search(r'<Version>([^<]+)</Version>',(R/'ThinkThen.csproj').read_text())[1]
dotnet=resolve_dotnet()
mode=sys.argv[1];logs=pathlib.Path(sys.argv[2]);work=logs/('independent consumer '+mode)
release_package=os.environ.get('THINKTHEN_RELEASE_NUPKG')
release_c=os.environ.get('THINKTHEN_RELEASE_C_DIR')
if bool(release_package) != bool(release_c): raise AssertionError('installed release needs both package paths')
package=pathlib.Path(release_package) if release_package else R/f'target/scratch/managed/Botassembly.ThinkThen.{V}.nupkg'
if release_package:
 assert {item.name for item in package.parent.iterdir()} == {
  f'Botassembly.ThinkThen.{V}.nupkg','LICENSE','README.md','THINKTHEN-PACKAGE-INPUTS'},'C# wrapper inventory'
 for name in ('LICENSE','README.md'):
  source="README-PUBLIC.md" if name=="README.md" else name
  assert (package.parent/name).read_bytes()==(R/source).read_bytes(),f'C# wrapper {name} differs'
 with zipfile.ZipFile(package) as bundle:
  names=set(bundle.namelist())
  assert {'Botassembly.ThinkThen.nuspec','lib/net8.0/ThinkThen.dll','README.md','LICENSE'} <= names,names
  assert 'runtimes/linux-x64/native/libthinkthen.so' in names,names
  assert not any(token in normalized_bytes(bundle.read(name)) for name in names for token in (b'/home/', b'/Users/',b'thinkthen_panic_probe',b'tt-canary-290')), 'private nupkg byte'
  metadata=ET.fromstring(bundle.read('Botassembly.ThinkThen.nuspec'))
  fields={node.tag.rsplit('}',1)[-1]:(node.text or '').strip() for node in metadata.iter()}
  assert fields['id']=='Botassembly.ThinkThen' and fields['version']==V,fields
work.mkdir()
home=work/'home';home.mkdir();cache=work/'cache';cache.mkdir();nuget=work/'nuget';nuget.mkdir()
local=work/'feed';local.mkdir();shutil.copyfile(package,local/f'Botassembly.ThinkThen.{V}.nupkg')
# Only consumer source and package feed are visible; compiler and original source are absent.
if mode=='example-public':
 import re
 sample=re.search(r'```csharp\n(.*?)```', (R/'README-PUBLIC.md').read_text(), re.S).group(1)
 (work/'Installed.cs').write_text(sample)
elif mode.startswith('example'):
 sample = 'Program.cs' if mode == 'example' else mode.removeprefix('example-').capitalize()+'.cs'
 shutil.copyfile(R/'examples'/sample,work/'Installed.cs')
else:
 shutil.copyfile(R/'tests/source/SessionChecks.cs',work/'SessionChecks.cs')
 shutil.copyfile(R/'tests/source/FixtureQuestions.g.cs',work/'FixtureQuestions.g.cs')
 shutil.copyfile(R/'tests/source/Installed.cs',work/'Installed.cs')
shutil.copyfile(R/'tests/Installed.csproj',work/'Installed.csproj')
barrier=work/'barrier';barrier.mkdir();server=Backend(barrier, arrivals='state' if mode.startswith('example') else 'packed', packed_questions=None if mode.startswith('example') else {'Is it?', 'Is it relevant?'}, hold_first=True)
cmd=['/usr/bin/bwrap','--unshare-all','--share-net','--die-with-parent','--dir','/usr','--dir','/usr/bin','--dir','/opt','--ro-bind',str(dotnet.parent),'/opt/dotnet','--ro-bind','/usr/lib','/usr/lib','--ro-bind','/usr/share','/usr/share','--ro-bind','/lib','/lib','--ro-bind','/lib64','/lib64','--ro-bind','/etc/passwd','/etc/passwd','--ro-bind','/etc/group','/etc/group','--proc','/proc','--dev','/dev','--tmpfs','/tmp','--bind',str(work),'/work','--chdir','/work','--','/opt/dotnet/dotnet','run','--project','/work/Installed.csproj','--configuration','Release','-p:RestoreSources=/work/feed','-p:RestoreIgnoreFailedSources=true']
env={'PATH':'/usr/bin:/bin','HOME':'/work/home','XDG_CONFIG_HOME':'/work/home','XDG_CACHE_HOME':'/work/home','DOTNET_CLI_HOME':'/work/home','NUGET_PACKAGES':'/work/nuget','DOTNET_CLI_TELEMETRY_OPTOUT':'1','DOTNET_SKIP_FIRST_TIME_EXPERIENCE':'1','DOTNET_NOLOGO':'1','DOTNET_MULTILEVEL_LOOKUP':'0','THINKTHEN_CACHE':'/work/cache','THINKTHEN_BASE_URL':f'http://127.0.0.1:{server.server_port}/generic/v1','THINKTHEN_API_KEY':'tt-canary-290'}
if os.environ.get('LLVM_PROFILE_FILE'):
 observation=pathlib.Path(os.environ['LLVM_PROFILE_FILE']).parent.absolute()
 profiles=observation.resolve(strict=True)
 boundary=cmd.index('--')
 cmd[boundary:boundary]=['--bind',str(profiles),str(observation)]
 env['LLVM_PROFILE_FILE']=os.environ['LLVM_PROFILE_FILE']
if mode=='portable': env['TT_PORTABLE_BATCH']='1'
if mode=='sessions': env['TT_SESSIONS']='1'
if mode.startswith('usage-'):
 env['TT_USAGE']=mode.removeprefix('usage-')
 env['XDG_STATE_HOME']='/work/state'
 if mode=='usage-disabled': env.update(HOME='relative', XDG_STATE_HOME='')
try:
 result=(run_usage(cmd, state=work/'state/thinkthen', mode=mode.removeprefix('usage-'), env=env,
                   execute=lambda command, *, env: run(command, timeout=100, env=env))
         if mode.startswith('usage-') else run(cmd,timeout=100,env=env))
 (work/'consumer.log').write_bytes(result.stdout+result.stderr)
 counted={'arrivals':server.arrivals,'attempts':server.attempts,'connections':server.connections,'pid':result.pid,'pgid':result.pgid,'exit':result.exit,'signals':result.signals}
 (work/'receipt.json').write_text(json.dumps(counted,indent=2)+'\n')
 assert result.exit==0,(result.exit,result.stdout[-1500:],result.stderr[-1500:])
 wire=barrier/'wire-requests.jsonl'
 bodies=wire.read_bytes().splitlines() if wire.exists() else []
 if mode.startswith('usage-'):
  assert b'INSTALLED_CSHARP_USAGE_PASS' in result.stdout,result.stdout
  wanted=[] if mode=='usage-disabled' else ['consumer-csharp']
  assert server.arrivals==wanted and server.attempts==server.connections==len(wanted),counted
  print('installed C# live usage status, safe advice, retained answers/facts and exact sends PASS',flush=True)
 elif mode=='sessions':
  assert b'INSTALLED_CSHARP_SESSION_PASS' in result.stdout,result.stdout
  assert server.user_agents and set(server.user_agents)=={f'thinkthen/{V} (csharp)'},server.user_agents
  required=collections.Counter(['session-owned','hold-session-task','hold-session-drain','hold-session-dispose']+[f'worker-lifetime-{turn}' for turn in range(16)])
  actual=collections.Counter(server.arrivals)
  admitted=collections.Counter((barrier/'admitted-feed').read_text().splitlines())
  assert required <= actual and actual <= required + admitted,counted
  assert server.attempts==server.connections==len(server.arrivals),counted
  print('installed C# owned sessions, bounded Full/Closed, held cancellation, explicit drain, ownership and presence PASS',flush=True)
 elif mode.startswith('example'):
  function='decide' if mode in ('example','example-public') else mode.removeprefix('example-')
  outputs={'decide':'True','choose':'first','score':'0.1','tag':'first,second','filter':'1:first',
           'rank':'0,1','find':'first','annotate':'True','recognize':'John Smith','relate':'A -> B'}
  sends=6 if mode=='example-public' else 2 if function=='recognize' else 1
  if mode=='example-public': outputs['decide']='True\nTrue\nTrue\nbilling\nbilling\nbilling'
  assert result.stdout.strip()==outputs[function].encode(),result.stdout
  assert len(bodies)==sends and server.attempts==server.connections==sends,counted
  assert server.user_agents==[f'thinkthen/{V} (csharp)']*sends,server.user_agents
  parsed=[json.loads(body) for body in bodies]
  quoted={'model':'jev-1.13.0','state':'Each question quotes the text it asks about.'}
  question=lambda kind,instructions,**extra: dict(type=kind,instructions=instructions,**extra)
  if mode=='example-public':
   refund_texts=['Please refund my order. It arrived broken.','Thanks for the quick help yesterday!','I want to send this back.']
   team_texts=['Please refund the extra fee on my invoice.','My parcel went to the wrong address.','I cannot reset my password.']
   expected=[dict(quoted,questions={'q1':question('noul',f'The text is {json.dumps(text)}. Does the customer ask for a refund?')}) for text in refund_texts]
   criteria={'billing':'Invoices, fees, and refunds.','shipping':'Parcels and delivery.','account':'Logins and passwords.'}
   expected += [dict(quoted,questions={'q1':question('choice',f'The text is {json.dumps(text)}. Which team owns this?',criteria=criteria)}) for text in team_texts]
  elif function in ('decide','annotate'):
   expected=dict(quoted,questions={'q1':question('noul','The text is "Please refund my order.". Does this ask for a refund?' if mode=='example-public' else 'The text is "example". Is it?')})
  elif function=='choose':
   expected=dict(quoted,questions={'q1':question('choice','The text is "example". Which?',criteria={'first':None,'second':None})})
  elif function=='score':
   expected=dict(quoted,questions={'q1':question('score','The text is "example". How much?',criteria=['low','high'])})
  elif function=='tag':
   expected=dict(quoted,questions={f'q{i}':question('noul',f'The text is "example". Which labels?\n\nDetermine whether the label "{label}" applies to this item.') for i,label in enumerate(('first','second'),1)})
  elif function in ('filter','rank'):
   expected=dict(quoted,questions={f'q{i}':question('noul',f'The text is "{text}". Is it?') for i,text in enumerate(('first','second'),1)})
  elif function=='find':
   expected={'model':'jev-1.13.0','state':'[{"id":"u001","evidence":"first"},{"id":"u002","evidence":"second"}]',
             'questions':{'q1':question('choice','Is it?',criteria={'u001':None,'u002':None})}}
  elif function=='relate':
   expected={'model':'jev-1.13.0','state':{'entities':[{'id':'i1','name':'A','kind':'person'},{'id':'i2','name':'B','kind':'person'}]},
             'questions':{'q1':question('noul','Is it true that i1 linked i2?'),'q2':question('noul','Is it true that i2 linked i1?')}}
  else:
   assert server.arrivals==['John Smith','John Smith'],counted
   assert [list(body['questions']) for body in parsed]==[['q1','q2'],['q1']],parsed
   assert list(parsed[0]['questions']['q1']['criteria'])==['BEGIN','INSIDE','END','SINGLE','OUT'],parsed
   assert list(parsed[1]['questions']['q1']['criteria'])==['person','none of these'],parsed
   assert all(body['model']=='jev-1.13.0' and body['state']=='John Smith' for body in parsed),parsed
  if function!='recognize': assert parsed==(expected if mode=='example-public' else [expected]),parsed
  print(f'installed C# shipped {function} sample, exact answer and {sends} counted sends PASS',flush=True)
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
finally:
 server.close()
