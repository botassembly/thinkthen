from pathlib import Path
from contextlib import ExitStack
import fcntl,importlib.util,json,os,runpy,shutil,signal,subprocess,sys,time
root=Path(__file__).resolve().parents[2]; out=Path(sys.argv[2]).resolve(); scripts=Path(__file__).resolve().parent
sha=sys.argv[1]
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=root,text=True).strip()==sha
assert os.getuid()!=0
os.chdir(root)
spec=importlib.util.spec_from_file_location('admission',root/'sdlc/scripts/build-admission.py'); admission=importlib.util.module_from_spec(spec); spec.loader.exec_module(admission)
for signum in (signal.SIGTERM,signal.SIGINT,signal.SIGHUP): signal.signal(signum,admission.interrupted)
sys.path.insert(0,str(root))
from conformance.parity import tool_environment
from conformance.children.children import child_env, CARGO
env=child_env(keep=(*CARGO,'LANG','LC_ALL','PUB_CACHE','UV_CACHE_DIR','UV_PYTHON_INSTALL_DIR','NPM_CONFIG_CACHE','RUBY','RBCONFIG','TT_DART','TT_FLUTTER','R_LIBS'))
env.update(tool_environment())
for key in ('THINKTHEN_SESSION_JDK_HOME','THINKTHEN_KOTLIN_HOME','THINKTHEN_SCALA_HOME','THINKTHEN_COMPOSER_BIN','THINKTHEN_PRIVATE_NAMES'):
 if key in os.environ: env[key]=os.environ[key]
env.update(GATE_OUTPUT=str(out),HOME=str(out/'home'))
env.setdefault('CARGO_HOME',str(Path.home()/'.cargo'))
env.setdefault('RUSTUP_HOME',str(Path.home()/'.rustup'))
env.setdefault('R_LIBS',env['R_LIBS_USER'])
env.setdefault('NPM_CONFIG_CACHE',env['npm_config_cache'])
for n in ('XDG_CONFIG_HOME','XDG_CACHE_HOME','XDG_STATE_HOME'):
 env[n]=str(out/'home'/n); Path(env[n]).mkdir(parents=True,exist_ok=True)
os.sched_setaffinity(0,sorted(os.sched_getaffinity(0))[:16])
adapter=out
shutil.copyfile(env['THINKTHEN_COMPOSER_BIN'],out/'composer'); (out/'composer').chmod(0o700)
env.update(CARGO_NET_OFFLINE='true',CARGO_BUILD_JOBS='2',RUSTUP_TOOLCHAIN='1.95.0',RUSTC_WRAPPER=str(scripts/'release-rustc-wrapper'),CARGO_TARGET_DIR=str(root/'target'),THINKTHEN_HEAVY_LOCK='/tmp/thinkthen-heavy.lock',DOTNET_CLI_DO_NOT_USE_MSBUILD_SERVER='1',MSBUILDDISABLENODEREUSE='1',GOPROXY='off',GOSUMDB='off',GOTOOLCHAIN='local',GOCACHE=str(root/'target/go/cache'),GOMODCACHE=str(root/'target/go/modcache'),CI='true',FLUTTER_SUPPRESS_ANALYTICS='true')
env['THINKTHEN_COMPOSER_BIN']=str(out/'composer')
env['PATH']=str(out)+os.pathsep+str(Path(env['TT_FLUTTER']).parent)+os.pathsep+env['PATH']
(out/'environment.json').write_text(json.dumps(env,indent=2)+'\n')
admission.load_allows_build=lambda:os.getloadavg()[0]<=16
(out/'pid').write_text(str(os.getpid())+'\n')
results={'revision':sha,'state':'running','gates':{}}
def atomic(name,value):
 temporary=out/(name+'.tmp'); temporary.write_text(json.dumps(value,indent=2)+'\n'); temporary.replace(out/name)
def save(): atomic('results-final-core.json',results)
def snapshot(name):
 value=Path('/proc/pressure/io').read_text(); (out/name).write_text(value)
 return {line.split()[0]:dict(part.split('=') for part in line.split()[1:]) for line in value.splitlines()}
def raw_phase(name,command,cwd=root,extra=None):
 assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=root,text=True).strip()==sha
 assert shutil.disk_usage(root).free >= 50*1024**3
 signal.pthread_sigmask(signal.SIG_SETMASK,set())
 before=snapshot(name+'.io-before'); start=time.monotonic()
 e=dict(env,THINKTHEN_HEAVY_LOCK_HELD='/tmp/thinkthen-heavy.lock',**(extra or {}))
 with (out/(name+'-final-core.log')).open('wb') as log:
  saved_out=os.dup(1); saved_err=os.dup(2); os.dup2(log.fileno(),1); os.dup2(log.fileno(),2)
  previous=os.getcwd(); os.chdir(cwd)
  try: code=admission.run_owned(command,e,None)
  except OSError as error: print(str(error),file=sys.stderr,flush=True); code=127
  finally:
   os.chdir(previous); os.dup2(saved_out,1); os.dup2(saved_err,2); os.close(saved_out); os.close(saved_err)
 after=snapshot(name+'.io-after')
 result={'revision':sha,'command':command,'cwd':str(cwd),'exit':code,'seconds':round(time.monotonic()-start,2),'remaining_children':sorted(admission.child_pids()),'io_before':before,'io_after':after}
 results['gates'][name]=result; atomic(name+'-final-core.exit',code); save()
 print(name+' exited '+str(code),flush=True)
 if code not in (0,77):
  print((out/(name+'-final-core.log')).read_text(errors='replace')[-8000:],flush=True)
 return result

lockpath='/tmp/thinkthen-heavy.lock';directory=Path(lockpath+'.priority');directory.mkdir(mode=0o700,exist_ok=True)
(out/'OWNER').write_text('Ticket: 0543\nRevision: '+sha+'\nRole: final combined required gate owner\nTarget: /tmp/thinkthen-0543-final-target\nPID: '+str(os.getpid())+'\n')
with ExitStack() as stack:
 lock=stack.enter_context(open(lockpath,'a+b'));resources=admission.Resources(lock.fileno(),directory,stack)
 def acquire(exclusive):
  signal.pthread_sigmask(signal.SIG_SETMASK,set())
  if not exclusive:
   admission.admit(lock.fileno(),resources,'normal',directory,None);return
  while True:
   if admission.available(lock.fileno(),lambda fd:fcntl.flock(fd,fcntl.LOCK_EX|fcntl.LOCK_NB)):
    if admission.load_allows_build() and all(resources.acquire(i) for i in range(admission.PERMITS)):return
    for i in range(admission.PERMITS):resources.release(i)
    fcntl.flock(lock.fileno(),fcntl.LOCK_UN)
   time.sleep(admission.POLL)
 def release():
  for i in range(admission.PERMITS+1):resources.release(i)
  fcntl.flock(lock.fileno(),fcntl.LOCK_UN)
 def phase(name,command,cwd=root,extra=None):
  acquire(name in ('c-official','c-complete','c-installed','polars-nextest'))
  try:return raw_phase(name,command,cwd,extra)
  finally:release()
 results['io_before']=snapshot('io-before-final-core');save()
 phase('cli-fresh',['cargo','build','--locked','--offline','--package','thinkthen','--bin','thinkthen','-j2'])
 shutil.copy2(root/'target/debug/thinkthen',out/'thinkthen')
 # Bound the driver helper to the freshly built command rather than a stale lane.
 env['PATH']=str(out)+os.pathsep+env['PATH']
 acquire(True)
 results['routine_hold']={'path':lockpath,'exclusive':True,'permits':list(range(admission.PERMITS)),'device':os.fstat(lock.fileno()).st_dev,'inode':os.fstat(lock.fileno()).st_ino};save()
 try:
  result=raw_phase('test',['bash','sdlc/scripts/test'])
  log=(out/'test-final-core.log').read_text(errors='replace')
  high=max(float(result[key][kind]['avg10']) for key in ('io_before','io_after') for kind in ('some','full'))>=1.0
  if result['exit'] and 'TIMEOUT' in log and high:raw_phase('test-retry',['bash','sdlc/scripts/test'])
 finally:release()
 phase('lint',['bash','sdlc/scripts/lint'])
 phase('polars-nextest',['bash','-c','. sdlc/scripts/scratch.sh; usage_home; export THINKTHEN_API_KEY=sk-polars-loopback THINKTHEN_BASE_URL=http://127.0.0.1:9/v1; cargo nextest run --profile default --locked --offline --package thinkthen --features polars --test polars'],extra={'CARGO_TARGET_DIR':str(root/'target/polars')})
 phase('polars-clippy',['cargo','clippy','--locked','--offline','--package','thinkthen','--features','polars','--lib','--bins','--test','polars','--','-D','warnings'],extra={'CARGO_TARGET_DIR':str(root/'target/polars')})
 phase('polars-clippy-nodefault',['cargo','clippy','--locked','--offline','--package','thinkthen','--features','polars','--no-default-features','--lib','--','-D','warnings'],extra={'CARGO_TARGET_DIR':str(root/'target/polars')})
 try:runpy.run_path(str(scripts/'release-gate-builds.py'),init_globals={'phase':phase,'checkpoint_env':env,'checkpoint_sha':sha})
 except Exception as error:results['build_setup_error']=repr(error);save()
 phase('c-official',['cargo','test','--locked','--offline','--manifest-path','libraries/c/Cargo.toml','-j2','--','--nocapture'])
 phase('c-complete',['python3','conformance/c_parity.py'],extra={'THINKTHEN_C_HEADER':str(root/'libraries/c/include/thinkthen.h'),'THINKTHEN_C_LIBRARY':str(root/'target/debug/libthinkthen_c.so')})
 phase('c-installed',['bash',str(scripts/'release-installed-c.sh')])
 import hashlib
 results['artifacts']={str(p):{'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'bytes':p.stat().st_size,'revision':sha} for p in [out/'thinkthen',root/'target/debug/libthinkthen_c.so',root/'target/debug/libthinkthen_c.a',root/'target/debug/libthinkthen.so',out/'native-current.tar.gz'] if p.exists()}
 results['io_after']=snapshot('io-after-final-core');results['remaining_children']=sorted(admission.child_pids());results['revision_after']=subprocess.check_output(['git','rev-parse','HEAD'],cwd=root,text=True).strip();results['source_clean']=not subprocess.check_output(['git','status','--porcelain'],cwd=root,text=True)
 effective_routine='test-retry' if 'test-retry' in results['gates'] else 'test'
 results['effective_routine']=effective_routine
 results['effective_failures']=[n for n,g in results['gates'].items() if g['exit'] != 0 and not (n=='objective-c-platform' and g['exit']==77) and not (n=='test' and effective_routine=='test-retry')]
 results['state']='failed' if results['effective_failures'] or 'build_setup_error' in results else 'passed-local-platform-pending';save()
print(json.dumps({'revision':sha,'state':results['state'],'remaining_children':results['remaining_children']}),flush=True)
sys.exit(0 if results['state']=='passed-local-platform-pending' else 1)
