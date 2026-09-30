"""The key goes only to loopback, and no message or ``repr`` carries it
(amendment change 14)."""

from conftest import FAKE, child_env, run

SENTINEL = "sk-sentinel-parent-key-never-crosses"


def test_the_child_environment_holds_the_fake_key_beside_loopback(backend, tmp_path, monkeypatch):
    """Change 14 and ticket 0127: the helper builds the child's whole
    environment, so neither the parent's key nor any other parent variable
    reaches a test child. The child reports one named variable, never its
    environment."""
    monkeypatch.setenv("THINKTHEN_API_KEY", SENTINEL)
    monkeypatch.setenv("FAKE_SERVICE_API_KEY", SENTINEL)
    env = child_env(backend, tmp_path)
    printed = run("""
        import os
        print(os.environ["THINKTHEN_API_KEY"], os.environ["THINKTHEN_BASE_URL"],
              "FAKE_SERVICE_API_KEY" in os.environ)
    """, env)
    assert printed.split() == [FAKE, backend.base(), "False"]
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
            tt.Engine(base_url=base, cache=False).decide(tt.question(decide="Late?"), "one").value
        except tt.BackendError:
            pass
        print(seen)
    """, child_env(backend, tmp_path))
    assert printed.strip() == f"[('/v1/systemone', 'Bearer {FAKE}')]"


def test_no_message_or_repr_carries_the_key_or_address_credentials(backend, tmp_path):
    """Change 14: every verb's failure on the refuse, 401, and 503 arms, a
    pandas Series to each column verb and a pandas frame to ``annotate`` and
    ``recognize`` included (ticket 0122), every other failure path's message
    and ``repr``, and the reprs of the public values name neither the fake
    key nor URL credentials. Each message is pinned whole, so a sentence
    that grows a secret fails."""
    printed = run(f"""
        import pandas as pd, thinkthen as tt
        port = {backend.port}
        late = tt.question(decide="Is it late?")
        team = tt.question(choose="Which team?", options=["billing", "shipping"])
        urgent = tt.question(score="How urgent?", levels=["Routine.", "Soon."])
        kinds = tt.question(tag="Which kinds?", labels=["bill", "ship"])
        frame = pd.DataFrame({{"body": ["Ada is here"]}})
        form = {{"version": 1, "questions": {{"late": {{"decide": "Late?"}}}}}}
        token = tt.CancelToken()
        token.cancel()
        shown = [repr(late), repr(token), repr(tt.Engine()), repr(tt.Entity("Ada", "person"))]
        verbs = [
            lambda engine: engine.decide(late, "one").value,
            lambda engine: engine.decide(late, ["one", "two"]).value,
            lambda engine: engine.filter(late, ["one", "two"]).value,
            lambda engine: engine.rank("Which is late?", ["one", "two"]).value,
            lambda engine: engine.find("Which is late?", ["one", "two"]).value,
            lambda engine: engine.annotate(form, ["one"]).value,
            lambda engine: engine.recognize("Ada is here", kinds=["person"]).value,
            lambda engine: engine.relate([("Ada", "person"), ("Bob", "person")],
                                         relations={{"knows": ("person", "person")}}).value,
            lambda engine: engine.decide(late, pd.Series(["one"])).value,
            lambda engine: engine.decide(late, pd.Series(["one", "two"])).value,
            lambda engine: engine.choose(team, pd.Series(["one"])).value,
            lambda engine: engine.score(urgent, pd.Series(["one"])).value,
            lambda engine: engine.tag(kinds, pd.Series(["one"])).value,
            lambda engine: engine.annotate(form, frame, on="body").value,
            lambda engine: engine.recognize(frame, kinds=["person"], on="body").value,
        ]
        calls = [lambda: tt.Engine(base_url=f"http://user:hidden-word@127.0.0.1:{{port}}/generic/v1")]
        for arm in ("refuse", "status/401", "503"):
            engine = tt.Engine(base_url=f"http://127.0.0.1:{{port}}/arm/{{arm}}/v1", cache=False)
            calls += [lambda verb=verb, engine=engine: verb(engine) for verb in verbs]
        calls += [
            lambda: tt.decide(late, "   ").value,
            lambda: tt.decide(late, "one", token=token).value,
            lambda: tt.decide(late, "one", deadline_ms=0).value,
            lambda: tt.question(file="/nonexistent/question.json"),
        ]
        said = []
        for call in calls:
            try:
                call()
                said.append("no error")
            except tt.ThinkThenError as error:
                said.append(f"{{type(error).__name__}} {{error}}")
                shown.append(repr(error))
        shown.append(repr(tt.recognize("Ada is here", kinds=["person"]).value))
        shown.append(repr(tt.details(late, "one")))
        print("\\n".join(said + ["--"] + shown))
    """, child_env(backend, tmp_path))
    said = printed.split("\n--\n")[0].splitlines()
    assert said == [
        "UsageError a base address carries no user information",
        *[f"BackendError the backend answered with status {status}"
          for status in (422, 401, 503) for _ in range(15)],
        "UsageError evidence is text, not white space",
        "Cancelled the call was cancelled",
        "DeadlineError the deadline of 0 s passed before the call answered",
        "LocalError the question file could not be read",
    ]
    assert FAKE not in printed and "hidden-word" not in printed


def test_credentials_in_the_environment_address_are_refused_unshown(backend, tmp_path):
    """Change 14: a ``THINKTHEN_BASE_URL`` with user information stops
    ``tt.decide`` before any send, and the message names neither part."""
    base = backend.base().replace("http://", "http://user:hidden-word@")
    printed = run("""
        import thinkthen as tt
        try:
            tt.decide("Is it late?", "one").value
        except tt.ThinkThenError as error:
            print(type(error).__name__, error)
    """, {**child_env(backend, tmp_path), "THINKTHEN_BASE_URL": base})
    assert printed.strip() == ("UsageError THINKTHEN_BASE_URL: a base address carries "
                               "no user information")
    assert backend.count() == 0


def test_entity_and_edge_reprs_withhold_names_and_kinds(backend, tmp_path):
    """A name and a kind are the caller's text, so an ``Entity`` repr and an
    ``Edge`` repr count their bytes, as the Rust ``Entity``'s ``Debug`` does.
    The relation prints, as the Rust ``Edge``'s ``Debug`` prints it."""
    printed = run("""
        import thinkthen as tt
        pair = [("MARK-Ada", "MARK-kind"), ("MARK-Bo", "MARK-kind")]
        print(repr(tt.Entity("MARK-Ada", "MARK-kind")))
        print(repr(tt.relate(pair, relations={"knows": ("MARK-kind", "MARK-kind")}).value[0]))
    """, child_env(backend, tmp_path))
    assert printed.splitlines() == [
        "Entity(name=<8 bytes withheld>, kind=<9 bytes withheld>)",
        'Edge(relation="knows", source=Entity(name=<8 bytes withheld>, kind=<9 bytes withheld>), '
        "target=Entity(name=<7 bytes withheld>, kind=<9 bytes withheld>), probability=0.9, either=False)",
    ]
