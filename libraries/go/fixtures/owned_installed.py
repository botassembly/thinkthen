"""Focused named Go calls through an external consumer of the staged module."""
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import time
from backend import Backend, Handler
sys.path.insert(0, str(Path(__file__).resolve().parents[3] / 'conformance/children'))
from children import child_env

module = Path(sys.argv[1]).resolve()
surface_only = len(sys.argv) > 2 and sys.argv[2] == 'surface'
with tempfile.TemporaryDirectory(prefix='thinkthen-go-owned-') as folder:
    home = Path(folder)
    consumer = home / 'consumer'
    consumer.mkdir()
    (consumer / 'go.mod').write_text('module example.org/owned-consumer\n\ngo 1.22\n\nrequire github.com/botassembly/thinkthen/libraries/go v0.0.0\nreplace github.com/botassembly/thinkthen/libraries/go => ' + str(module) + '\n')
    shutil.copy2(Path(__file__).with_name('owned_consumer.go'), consumer / 'main.go')
    env = child_env(HOME=str(home), XDG_CONFIG_HOME=str(home/'config'), XDG_STATE_HOME=str(home/'state'), XDG_CACHE_HOME=str(home/'cache'), GOPROXY='off', GOSUMDB='off', GOTOOLCHAIN='local', GOCACHE=str(module.parent/'go-cache'), GOMODCACHE=str(home/'modcache'), CGO_ENABLED='1')
    subprocess.run(['go','build','-p','1','-buildvcs=false','-o',str(home/'consumer-bin'),'.'],cwd=consumer,env=env,check=True)
    linked = subprocess.check_output(['ldd', str(home/'consumer-bin')], text=True, env=env)
    assert 'libthinkthen' not in linked, linked
    barrier = home/'barrier'
    barrier.mkdir()
    server = Backend(barrier)
    user_agents = []
    class AttributedHandler(Handler):
        def do_POST(self):
            user_agents.append(self.headers.get('User-Agent'))
            super().do_POST()
    server.RequestHandlerClass = AttributedHandler
    try:
        run_env=env|{'THINKTHEN_API_KEY':'tt-canary-274','THINKTHEN_BASE_URL':f'http://127.0.0.1:{server.server_port}/generic/v1'}
        result=subprocess.run([str(home/'consumer-bin'), *(['surface'] if surface_only else [])],env=run_env,cwd=consumer,text=True,capture_output=True,timeout=5 if surface_only else 20)
        assert result.returncode==0, (result.stdout,result.stderr)
        assert user_agents and all(agent == 'thinkthen/0.2.0 (go)' for agent in user_agents), user_agents
        if surface_only:
            assert result.stdout.strip() == 'surface-owned-results-pass requests=2', result.stdout
            assert server.attempts == len(user_agents) == 2, (server.attempts, user_agents)
            print('GO_SURFACE_INSTALLED_PASS user-agent retained-success-and-failure static-native requests=2')
            sys.exit(0)
        assert result.stdout.startswith('ten-named-calls-owned-results-pass requests='),result.stdout
        # Ten named calls plus one false-decision call; invalid/precancel/deadline add none.
        assert server.attempts==int(result.stdout.strip().split("=")[-1]), (server.attempts, result.stdout)
        baseline=server.attempts
        for iteration,mode in enumerate(('cancel','close')):
            for marker in ('arrived-hold-go','release-hold-go'):
                (barrier/marker).unlink(missing_ok=True)
            (home/'cancel').unlink(missing_ok=True)
            cancel=home/'cancel'
            child=subprocess.Popen([str(home/'consumer-bin'),mode],env=run_env|{'TT_CANCEL_FILE':str(cancel)},cwd=consumer,text=True,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
            try:
                deadline=time.monotonic()+5
                while not (barrier/'arrived-hold-go').exists() and time.monotonic()<deadline:
                    time.sleep(.005)
                assert (barrier/'arrived-hold-go').exists(),'held request never arrived'
                cancel.touch()
                stdout,stderr=child.communicate(timeout=3)
                assert child.returncode==0 and stdout.strip()=='cancelled-before-release',(stdout,stderr)
                assert not (barrier/'release-hold-go').exists()
                assert server.attempts==baseline+iteration+1,server.attempts
            finally:
                (barrier/'release-hold-go').touch()
                if child.poll() is None:
                    child.terminate();child.wait(timeout=3)
        print('GO_OWNED_INSTALLED_PASS ten-functions presence retained-errors cancellation static-native')
    finally:
        server.close()
