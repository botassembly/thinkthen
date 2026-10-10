import json
from contextlib import contextmanager
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from threading import Thread
from conftest import child_env, run

@contextmanager
def capturing_filter_listener(answer=None, usage=True, model="jev-latest"):
    """Keep wire bodies; a callback's None retains the normal decision answer."""
    bodies = []

    class Handler(BaseHTTPRequestHandler):
        # Debt 020: this HTTP/1.1 listener closes too, so policy.py needs no exemption;
        # sdlc/issues/closed/2026-09-30-ureq-reuses-a-connection-after-an-http-1-0-reply.md
        def end_headers(self):
            self.send_header("Connection", "close")
            super().end_headers()

        protocol_version = "HTTP/1.1"

        def do_POST(self):
            body = self.rfile.read(int(self.headers["Content-Length"]))
            bodies.append(body)
            request = json.loads(body)
            answers = {}
            for name, question in request["questions"].items():
                chosen = answer(question) if answer is not None else None
                if chosen is not None:
                    answers[name] = chosen
                    continue
                first = request["state"] == "one" or 'The text is "one"' in question["instructions"]
                answers[name] = {"type": "noul", "noul": 0.1 if first else 0.9}
            reply_fields = {"model": model, "answers": answers}
            if usage:
                reply_fields["usage"] = {"input_tokens": 6, "output_tokens": 3}
            reply = json.dumps(reply_fields,
                               separators=(",", ":")).encode()
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(reply)))
            self.end_headers()
            self.wfile.write(reply)

        def log_message(self, *_):
            pass

    listener = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    thread = Thread(target=listener.serve_forever, daemon=True)
    thread.start()
    try:
        yield f"http://127.0.0.1:{listener.server_port}/v1/systemone", bodies
    finally:
        listener.shutdown()
        listener.server_close()
        thread.join(timeout=5)


def test_process_cap_reserves_actual_attempts(backend, tmp_path):
    """The Python constructor forwards the active process cap. A refusal
    after one send cannot be simulated by a plan or by a per-call limit."""
    printed = run("""
    import thinkthen as tt
    low = tt.Engine(cache=False, max_requests_total=1)
    print(low.decide("Is it late?", "one").value)
    try:
        low.decide("Is it late?", "two").value
    except tt.UsageError as error:
        print(error.kind, error.retryable, low.usage()["requests_sent"])
    else:
        raise AssertionError("the second attempt crossed the cap")
    high = tt.Engine(cache=False, max_requests_total=2)
    print(high.decide("Is it late?", "three").value,
          high.usage()["requests_sent"])
    """, child_env(backend, tmp_path))
    assert printed.splitlines() == ["True", "usage False 1", "True 1"]
    assert backend.count() == 2


def test_token_cap_variable_refuses_before_any_send(backend, tmp_path):
    """The engine reads the token cap variable through from_env and sends nothing past it."""
    printed = run("""
    import polars as pl, thinkthen as tt
    engine = tt.Engine(cache=False)
    for text in ("one", pl.Series(["two", "three"])):
        try:
            engine.decide("Is it late?", text)
        except tt.UsageError as error:
            print(error.kind, error)
    """, child_env(backend, tmp_path, THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL="10"))
    refusal = ("usage max_estimated_input_tokens_total=10 (encoded-body-bytes-908-v1) "
               "would be exceeded before this call's first request")
    assert printed.splitlines() == [refusal, refusal]
    assert backend.count() == 0


