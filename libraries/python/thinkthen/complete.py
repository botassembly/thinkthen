"""Complete native results and explicit input carriers for the ten functions."""
from __future__ import annotations
from dataclasses import dataclass
import json
from . import _thinkthen
from . import _complete as c
from ._complete import *

@dataclass(frozen=True, repr=False, kw_only=True)
class QuestionSource:
    role: str
    body: c.Carrier | c.JsonValue | None = None
    path: str | None = None
    name: str | None = None
    reference: str | None = None
    none: bool = False
    raw: str | None = None

    def __repr__(self): return '<QuestionSource: content withheld>'
    def wire(self):
        out = {'role': self.role,'none':self.none}
        for key in ('body', 'path', 'name', 'reference', 'raw'):
            value = getattr(self, key)
            if value is not None: out[key] = c.to_json(value)
        return out

@dataclass(frozen=True, repr=False, kw_only=True)
class Image:
    media: str
    data: bytes
    def __repr__(self): return '<Image: bytes withheld>'

@dataclass(frozen=True, repr=False, kw_only=True)
class Item:
    value: c.JsonValue = None
    text: bool = False
    images: tuple[Image, ...] = ()
    context: c.JsonValue | c.Absent = c.ABSENT
    options: tuple[tuple[str, c.Description | c.Absent], ...] | c.Absent = c.ABSENT
    image_only: bool = False
    def __repr__(self): return '<Item: content withheld>'
    def wire(self):
        content = {'kind':'images'} if self.image_only else {'kind':'text' if self.text else 'json', 'value':c.to_json(self.value)}
        out = {'content':content,'images':[{'media':i.media,'bytes':list(i.data)} for i in self.images]}
        if self.context is not c.ABSENT:
            out['context'] = {'kind':'text' if isinstance(self.context, str) else 'json', 'value':c.to_json(self.context)}
        if self.options is not c.ABSENT:
            out['options'] = [{'name':name, **({} if d is c.ABSENT else {'description':c.to_json(d)})} for name,d in self.options]
        return out

@dataclass(frozen=True, repr=False)
class Records:
    items: tuple[Item, ...]
    def __repr__(self): return '<Records: content withheld>'
    def wire(self): return {'kind':'records','records':[item.wire() for item in self.items]}

@dataclass(frozen=True, repr=False)
class Completed:
    results: tuple[c.Result, ...]
    facts: c.Facts
    ordinals: tuple[int | None, ...]
    inputs: tuple[c.NativeInput, ...]
    def __repr__(self): return '<Completed: content withheld>'

@dataclass(frozen=True, repr=False, kw_only=True)
class Files:
    paths: tuple[str, ...]
    unit: str = 'line'
    window: int | c.Absent = c.ABSENT
    media: str = 'text'
    jsonl: bool = False
    def __repr__(self): return '<Files: content withheld>'

def _request(verb,question,source,attempts):
    if not isinstance(question,QuestionSource): raise _thinkthen.UsageError('explicit question source required')
    if isinstance(source,(Files,c.Files)):
        reading={'unit':source.unit}
        if source.window is not c.ABSENT: reading['window']=source.window
        wire={'kind':'files','paths':list(source.paths),'options':{'reading':reading,'media':'text' if source.media is c.ABSENT else source.media},'jsonl':getattr(source,'jsonl',False)}
    elif isinstance(source,Records): wire=source.wire()
    else: raise _thinkthen.UsageError('explicit records or files required')
    return json.dumps({'verb':verb,'question':question.wire(),'input':wire,'attempts':attempts},ensure_ascii=False,separators=(',',':'))

def _complete_error(error):
    if hasattr(error,'native_complete'): error.complete=c.decode('CallError',json.loads(error.native_complete))
    return error

@dataclass(frozen=True, repr=False)
class BatchRow:
    result: c.Result
    ordinal: int
    input: c.NativeInput
    def __repr__(self): return '<BatchRow: content withheld>'

class Batch:
    def __init__(self,native,kind): self._native=native;self._kind=kind;self.facts=None;self._ended=False
    def __repr__(self): return '<CompleteBatch>'
    def __iter__(self): return self
    def __next__(self):
        if self._ended: raise StopIteration
        event=json.loads(self._native._pull())
        if 'row' in event: return BatchRow(c.decode(self._kind,event['row']),event['ordinal'],c.decode('NativeInput',event['input']))
        self._ended=True
        self.close()
        if 'error' in event:
            error=event['error'];cls={'usage':_thinkthen.UsageError,'backend':_thinkthen.BackendError,'local':_thinkthen.LocalError,'cancelled':_thinkthen.Cancelled,'deadline':_thinkthen.DeadlineError,'defect':_thinkthen.DefectError}[error['kind']]
            raised=cls(error['message']);raised.complete=c.decode('CallError',error)
            if raised.complete.facts is not c.ABSENT: self.facts=raised.complete.facts
            raised.kind=error['kind'];raised.retryable=error['retryable'];raise raised
        self.facts=c.decode('Facts',event['facts']);raise StopIteration
    def close(self): self._ended=True;self._native.close()
    def cancel(self): self._native.cancel()
    def __enter__(self): return self
    def __exit__(self,*args): self.close()

class Engine:
    """The complete facade shares an existing engine or constructs one normally."""
    def __init__(self, *, _engine=None, **settings):
        try:
            self._engine = _engine if _engine is not None else _thinkthen._Engine(**settings)
        except TypeError as e:
            raise _thinkthen.UsageError('unsupported complete engine setting') from e
    def __repr__(self): return '<CompleteEngine>'
    def _call(self, verb, question: QuestionSource, source: Records | c.Files, *, token=None, deadline_ms=None, attempts=False, context=None):
        request=_request(verb,question,source,attempts)
        if context is not None:
            held=json.loads(request);held['context']=context;request=json.dumps(held,ensure_ascii=False,separators=(',',':'))
        try: call=self._engine._complete(request,deadline_ms,token)
        except _thinkthen.ThinkThenError as error: raise _complete_error(error)
        result=call.value
        rows=result['results'] if isinstance(result['results'],list) else [result['results']]
        kind=verb.title()+'Result'
        return Completed(tuple(c.decode(kind,row) for row in rows),c.decode('Facts',result['facts']), tuple(result['ordinals']), tuple(c.decode('NativeInput',v) for v in result['inputs']))
    def decide(self, question, source, **controls): return self._call('decide',question,source,**controls)
    def choose(self, question, source, **controls): return self._call('choose',question,source,**controls)
    def tag(self, question, source, **controls): return self._call('tag',question,source,**controls)
    def score(self, question, source, **controls): return self._call('score',question,source,**controls)
    def filter(self, question, source, **controls): return self._call('filter',question,source,**controls)
    def rank(self, question, source, **controls): return self._call('rank',question,source,**controls)
    def find(self, question, source, **controls): return self._call('find',question,source,**controls)
    def annotate(self, question, source, **controls): return self._call('annotate',question,source,**controls)
    def recognize(self, question, source, **controls): return self._call('recognize',question,source,**controls)
    def relate(self, question, source, **controls): return self._call('relate',question,source,**controls)

    def _batch(self,verb,question,source,*,token=None,deadline_ms=None,attempts=False,context=None):
        request=json.loads(_request(verb,question,source,attempts))
        if context is not None:request['context']=context
        return Batch(self._engine._complete_batch(json.dumps(request,ensure_ascii=False,separators=(',',':')),deadline_ms,token),verb.title()+'Result')
    def decide_batch(self,q,i,**c): return self._batch('decide',q,i,**c)
    def choose_batch(self,q,i,**c): return self._batch('choose',q,i,**c)
    def tag_batch(self,q,i,**c): return self._batch('tag',q,i,**c)
    def score_batch(self,q,i,**c): return self._batch('score',q,i,**c)
    def filter_batch(self,q,i,**c): return self._batch('filter',q,i,**c)
    def annotate_batch(self,q,i,**c): return self._batch('annotate',q,i,**c)
