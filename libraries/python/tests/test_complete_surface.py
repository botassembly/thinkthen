"""The native Python door carries one explicit closed wrapper identity."""
import json
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import pytest
from thinkthen import _thinkthen as native


def test_complete_surface_is_closed_immediate_and_used_by_eager_and_lazy_calls(monkeypatch,tmp_path):
    for name in ('HOME','XDG_CONFIG_HOME','XDG_CACHE_HOME','XDG_STATE_HOME'):
        monkeypatch.setenv(name,str(tmp_path/name))
    monkeypatch.setenv('THINKTHEN_API_KEY','sk-surface-loopback')
    agents=[]
    class Handler(BaseHTTPRequestHandler):
        def do_POST(self):
            self.rfile.read(int(self.headers['Content-Length']))
            agents.append(self.headers['User-Agent'])
            body=b'{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}'
            self.send_response(200)
            self.send_header('Content-Length',str(len(body)))
            self.send_header("Connection", "close")
            self.end_headers()
            self.wfile.write(body)
        def log_message(self,*args): pass
    server=ThreadingHTTPServer(('127.0.0.1',0),Handler)
    worker=threading.Thread(target=server.serve_forever)
    worker.start()
    try:
        engine=native._Engine(base_url=f'http://127.0.0.1:{server.server_port}/v1',model='fixed',cache=False,max_retries=0)
        request=json.dumps({'verb':'decide','question':{'role':'atomic','body':{'decide':'Q'}},'input':{'kind':'records','records':[{'content':{'kind':'text','value':'a'},'images':[]}]}})
        for surface in ('typescript','Python',' python','unknown-secret',''):
            for method in (engine._complete,engine._complete_batch):
                with pytest.raises(native.ThinkThenError,match='surface must be python, pandas or python-polars') as error:
                    method('{bad',None,None,surface)
                assert surface not in str(error.value) or surface in ('',' python')
        assert agents==[]
        for surface in (None,'python','pandas','python-polars'):
            if surface is None:engine._complete(request,None,None)
            else:engine._complete(request,None,None,surface)
            batch=engine._complete_batch(request,None,None,surface)
            assert len(agents)%2==1
            event=json.loads(batch._pull())
            assert event['row']['value'] is True
            assert 'facts' in json.loads(batch._pull())
            batch.close()
        assert agents==[f'thinkthen/0.2.0 ({s})' for s in ('python','python','pandas','python-polars') for _ in range(2)]
    finally:
        server.shutdown();server.server_close();worker.join()


def test_complete_recognition_reads_proposals_and_preserves_old_documents():
    from thinkthen import complete as c
    answer = {'pieces': [], 'names': [], 'pairs': [], 'proposals': [
        {'start': 11, 'end': 14, 'span_probability': .4, 'selected': {'start': 11, 'end': 14},
         'kind': 'person', 'strength': .36, 'kept': False},
        {'start': 3, 'end': 6, 'span_probability': .9, 'kept': False}]}
    result = c.decode('RecognitionAnswer', answer)
    assert result.proposals[0].strength == .36
    assert result.proposals[0].kind == 'person'
    assert result.proposals[1].strength is c.ABSENT
    assert c.to_json(result) == answer
    old = {key: value for key, value in answer.items() if key != 'proposals'}
    assert c.decode('RecognitionAnswer', old).proposals is c.ABSENT
    with pytest.raises(ValueError):
        c.decode('RecognitionAnswer', {**answer, 'invented': True})
