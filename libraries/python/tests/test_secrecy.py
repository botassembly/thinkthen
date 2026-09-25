"""The key goes only to loopback, and no message or ``repr`` carries it
(amendment change 14)."""

from conftest import FAKE, child_env, run

SENTINEL = "sk-sentinel-parent-key-never-crosses"


def test_the_child_environment_holds_the_fake_key_beside_loopback(backend, tmp_path, monkeypatch):
    """Change 14: the helper removes the parent's key before it sets the
    fake one, so a developer's real key never reaches a test child."""
    monkeypatch.setenv("THINKTHEN_API_KEY", SENTINEL)
    env = child_env(backend, tmp_path)
    printed = run("""
        import os
        print(os.environ["THINKTHEN_API_KEY"], os.environ["THINKTHEN_BASE_URL"])
    """, env)
    assert printed.split() == [FAKE, backend.base()]
    assert backend.base().startswith("http://127.0.0.1:")
    assert SENTINEL not in printed


def test_the_fake_key_arrives_at_a_loopback_listener(backend, tmp_path):
    """Change 14, builder note 2: a standard-library listener records the
    ``Authorization`` header the engine sent to loopback."""
    printed = run("""
        import http.server, threading, thinkthen as tt
        seen = []
        class Listener(http.server.BaseHTTPRequestHandler):
            def do_POST(self):
                seen.append((self.path, self.headers["Authorization"]))
                self.send_response(400)
                self.end_headers()
            def log_message(self, *_):
                pass
        server = http.server.HTTPServer(("127.0.0.1", 0), Listener)
        threading.Thread(target=server.serve_forever, daemon=True).start()
        base = f"http://127.0.0.1:{server.server_port}/v1"
        try:
            tt.Engine(base_url=base, cache=False).decide(tt.question(decide="Late?"), "one")
        except tt.BackendError:
            pass
        print(seen)
    """, child_env(backend, tmp_path))
    assert printed.strip() == f"[('/v1/systemone', 'Bearer {FAKE}')]"


def test_no_message_or_repr_carries_the_key_or_address_credentials(backend, tmp_path):
    """Change 14: every failure path's message and ``repr``, and the reprs of
    the public values, name neither the fake key nor URL credentials."""
    printed = run(f"""
        import thinkthen as tt
        port = {backend.port}
        late = tt.question(decide="Is it late?")
        token = tt.CancelToken()
        token.cancel()
        shown = [repr(late), repr(token), repr(tt.Engine()), repr(tt.Entity("Ada", "person"))]
        calls = [
            lambda: tt.Engine(base_url=f"http://user:hidden-word@127.0.0.1:{{port}}/generic/v1"),
            lambda: tt.Engine(base_url=f"http://127.0.0.1:{{port}}/arm/refuse/v1", cache=False).decide(late, "one"),
            lambda: tt.Engine(base_url=f"http://127.0.0.1:{{port}}/arm/status/401/v1", cache=False).decide(late, "one"),
            lambda: tt.Engine(base_url=f"http://127.0.0.1:{{port}}/arm/503/v1", cache=False).decide(late, "two"),
            lambda: tt.decide(late, "   "),
            lambda: tt.decide(late, "one", token=token),
            lambda: tt.decide(late, "one", deadline=0),
            lambda: tt.question(file="/nonexistent/question.json"),
        ]
        for call in calls:
            try:
                call()
                shown.append("no error")
            except tt.ThinkThenError as error:
                shown += [str(error), repr(error)]
        shown.append(repr(tt.recognize("Ada is here", kinds=["person"])))
        shown.append(repr(tt.details(late, "one")))
        print("\\n".join(shown))
    """, child_env(backend, tmp_path))
    assert "no error" not in printed
    assert "a base address carries no user information" in printed
    assert FAKE not in printed and "hidden-word" not in printed
