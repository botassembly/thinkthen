"""Focused named Go calls through an external consumer of the staged module."""
import fcntl
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
plan_only = len(sys.argv) > 2 and sys.argv[2] == 'plan'
surface_only = len(sys.argv) > 2 and sys.argv[2] == 'surface'
usage_only = len(sys.argv) > 2 and sys.argv[2] == 'usage'
feed_only = len(sys.argv) > 2 and sys.argv[2] == 'feed'
with tempfile.TemporaryDirectory(prefix='thinkthen-go-owned-') as folder:
    home = Path(folder)
    consumer = home / 'consumer'
    consumer.mkdir()
    (consumer / 'go.mod').write_text('module example.org/owned-consumer\n\ngo 1.22\n\nrequire github.com/botassembly/thinkthen/libraries/go v0.0.0\nreplace github.com/botassembly/thinkthen/libraries/go => ' + str(module) + '\n')
    shutil.copy2(Path(__file__).with_name('owned_consumer.go'), consumer / 'main.go')
    env = child_env(home=str(home), GOPROXY='off', GOSUMDB='off', GOTOOLCHAIN='local', GOCACHE=str(Path(__file__).resolve().parents[3]/'target/go/cache'), GOMODCACHE=str(home/'modcache'), CGO_ENABLED='1')
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
        if plan_only:
            result=subprocess.run([str(home/'consumer-bin'),'plan'],env=env|{'THINKTHEN_BASE_URL':f'http://127.0.0.1:{server.server_port}/generic/v1'},cwd=consumer,text=True,capture_output=True,timeout=5)
            assert result.returncode==0,(result.stdout,result.stderr)
            import json
            corpus=json.loads((Path(__file__).resolve().parents[3]/'specification/fixtures/types/corpus.json').read_text())['cases']
            expected=next(c['response'] for c in corpus if c['name']=='plan-p1')
            assert json.loads(result.stdout)==expected,(result.stdout,expected)
            assert server.attempts==0 and not user_agents,server.attempts
            print('GO_PLAN_INSTALLED_PASS four-functions shared-plan absent-key retained-presence refusals zero-send')
            sys.exit(0)
        if usage_only:
            for mode in ('usage-written', 'usage-failed'):
                state = home / mode / 'state/thinkthen'; state.mkdir(mode=0o700, parents=True)
                before = server.attempts
                with (state / '.lock').open('w') as lock:
                    (state / '.lock').chmod(0o600)
                    if mode == 'usage-failed': fcntl.flock(lock, fcntl.LOCK_EX)
                    result = subprocess.run([str(home/'consumer-bin'), mode], env=child_env(home=str(home / mode), THINKTHEN_API_KEY='tt-canary-274', THINKTHEN_BASE_URL=f'http://127.0.0.1:{server.server_port}/generic/v1'), cwd=consumer, text=True, capture_output=True, timeout=5)
                    assert result.returncode == 0 and result.stdout.strip() == 'usage-status-pass requests=1', (mode, result.stdout, result.stderr)
                assert server.attempts == before + 1, server.attempts
                print('GO_USAGE_INSTALLED_PASS', mode, 'requests=1')
            sys.exit(0)
        result=subprocess.run([str(home/'consumer-bin'), *(['surface'] if surface_only else ['feed'] if feed_only else [])],env=run_env,cwd=consumer,text=True,capture_output=True,timeout=5 if surface_only else 20)
        assert result.returncode==0, (result.stdout,result.stderr)
        assert user_agents and all(agent == 'thinkthen/0.2.0 (go)' for agent in user_agents), user_agents
        if surface_only:
            assert result.stdout.strip() == 'surface-owned-results-pass requests=2', result.stdout
            assert server.attempts == len(user_agents) == 2, (server.attempts, user_agents)
            print('GO_SURFACE_INSTALLED_PASS user-agent retained-success-and-failure static-native requests=2')
            sys.exit(0)
        assert result.stdout.startswith('bounded-feed-results-pass requests=' if feed_only else 'ten-named-calls-owned-results-pass requests='),result.stdout
        # Successful calls and completed reader prefix account for every send.
        assert server.attempts==int(result.stdout.strip().split("=")[-1]), (server.attempts, result.stdout)
        baseline=server.attempts
        for iteration,mode in enumerate(('feed-cancel','feed-close','feed-full') if feed_only else ('cancel','close')):
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
                if feed_only:
                    ready = home/'reader-ready'
                    while not ready.exists() and time.monotonic()<deadline:
                        time.sleep(.005)
                    assert ready.exists(), 'producer did not reach bounded intake'
                    ready.unlink()
                cancel.touch()
                stdout,stderr=child.communicate(timeout=3)
                assert child.returncode==0 and stdout.strip()=='cancelled-before-release',(stdout,stderr)
                assert not (barrier/'release-hold-go').exists()
                assert server.attempts==baseline+iteration+1,server.attempts
            finally:
                (barrier/'release-hold-go').touch()
                if child.poll() is None:
                    child.terminate();child.wait(timeout=3)
        print('GO_FEED_INSTALLED_PASS bounded-intake typed-reader-failure zero-send cancellation close static-native' if feed_only else 'GO_OWNED_INSTALLED_PASS ten-functions presence retained-errors cancellation static-native')
    finally:
        server.close()
