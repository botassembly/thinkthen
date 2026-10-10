"""Explicit host file rows and ten judgments over the shared folder fixture."""
import errno
import json
import os
import sys
import tempfile
from pathlib import Path

from harness import Backend, REPO_ROOT, case, expect, main, rows, run, said

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / "sqlite" / "tests"))
from conditional_backend import ConditionalBackend

FIXTURE = REPO_ROOT / 'specification/fixtures/files/documents'
EXAMPLES = REPO_ROOT / 'databases/duckdb/examples/files.sql'


def literal(value):
    return "'" + value.replace("'", "''") + "'"


@case
def ten_folder_examples_keep_original_documents_and_mapped_endpoints():
    # These are the published statements; only fixture paths are made absolute.
    source = EXAMPLES.read_text().replace('specification/fixtures/files/documents', str(FIXTURE))
    source = source.replace('specification/fixtures/files/questions.json', str(FIXTURE.parent / 'questions.json'))
    source = '\n'.join(line for line in source.splitlines() if not line.startswith('--'))
    statements = [statement.strip() for statement in source.split(';') if statement.strip()]
    expect(len(statements), 12, 'two reader units plus all ten examples')
    expected = [[i, p.read_text(), str(p), 1, 4] for i,p in enumerate(sorted(FIXTURE.iterdir()), 1)]
    lines = [[i, line, str(path), number, number]
             for i,(path,number,line) in enumerate(
                 ((p,n,line) for p in sorted(FIXTURE.iterdir())
                  for n,line in enumerate(p.read_text().splitlines(),1)),1)]
    with tempfile.TemporaryDirectory(prefix='thinkthen-files-cache-') as cache, Backend() as backend, ConditionalBackend(backend.base()) as proxy:
        extra = {'THINKTHEN_CACHE': cache}
        def reply(answers):
            proxy.reply = json.dumps({'model':'jev-1.13.0','answers':answers}).encode()
        # Seed real judgment cache entries with controlled yes/no replies. The
        # published WHERE runs over every source row and must reject five lines.
        for row in lines:
            probability = 0.9 if 'refund' in row[1].lower() else 0.1
            reply({'q1':{'type':'noul','noul':probability}})
            result = run(["SELECT thinkthen_decide('Does this line describe a refund?', " + literal(row[1]) + ")"], proxy.base, extra=extra)
            expect(rows(result[0]), [[probability >= 0.5]], 'controlled accepted and rejected line')
        got = []
        for index,statement in enumerate(statements[2:]):
            proxy.reply = None
            if index == 5:
                reply({'q1':{'type':'noul','noul':0.2},'q2':{'type':'noul','noul':0.8}})
            elif index == 6:
                reply({'q1':{'type':'choice','probabilities':{
                    f'u{n:03}': 0.9 if n == 2 else 0.1 / 7 for n in range(1,9)}}})
            result = run([*statements[:2],statement], proxy.base, extra=extra)
            got.append(result[-1])
        for index, values in ((0,[True,True]),(1,['policy','policy']),
                              (2,[['refund','support','billing']]*2),(3,[0.1,0.1])):
            expect(rows(got[index]), [r+[v] for r,v in zip(expected,values)], 'row judgment original bytes/locations')
        expect(rows(got[4]), lines[:3], 'filter accepts refund lines and rejects other source rows')
        expect(rows(got[5]), [expected[1]+[1,0.8],expected[0]+[2,0.2]], 'rank orders whole files by controlled relevance')
        expect(rows(got[6]), [lines[1]+[0.9]], 'find returns original refund policy line and file')
        expect([r[:5] for r in rows(got[7])], expected, 'annotation metadata stays outside original record')
        expect([json.loads(r[5]) for r in rows(got[7])], [{'contract':True,'urgent':True}]*2, 'named annotations')
        recognized = rows(got[8])
        expect(len(recognized), 4, 'recognition rows for two requested kinds')
        for row in recognized:
            original = expected[row[0]-1]
            ordinal,record,file,name,start,end,kind,first,last = row
            expect([ordinal,record,file], original[:3], 'recognition source')
            expect(record[start:end],name,'native Unicode offsets')
            expect([first,last], [1+record[:start].count('\n'),1+record[:end-1].count('\n')], 'recognized physical lines from actual offsets')
        endpoints = rows(got[9])
        expect(len(endpoints), 2, 'both directed document edges')
        for edge in endpoints:
            expect(edge[1:6], expected[edge[1]-1], 'actual source endpoint row')
            expect(edge[6:11], expected[edge[6]-1], 'actual target endpoint row')
            expect(edge[0], 'supports', 'relation')
            expect(edge[11], 0.9, 'edge probability')
        expect(proxy.count(), 20, 'eight seeded lines and twelve remaining uncached judgment requests')
        expect(backend.count(), 10, 'ten controlled replies stay on the existing proxy')


@case
def reader_units_order_hidden_files_links_and_unicode_spans():
    with tempfile.TemporaryDirectory(prefix='thinkthen-reader-') as tmp, Backend() as backend:
        folder = Path(tmp)
        (folder/'.hidden').write_bytes('café 😀\r\n\r\nAda\nlast'.encode())
        (folder/'nested').mkdir(); (folder/'nested/z').write_text('tail\n')
        (folder/'linked').symlink_to(folder/'.hidden')
        (folder/'cycle').symlink_to(folder,target_is_directory=True)
        path = literal(tmp)
        got = run([
            f'SELECT * FROM thinkthen_read_files({path})',
            f'SELECT * FROM thinkthen_read_files({path},\'{{"unit":"window","window":3}}\')',
            f'SELECT * FROM thinkthen_read_files({path},\'{{"unit":"file"}}\')',
            f'SELECT record FROM thinkthen_read_files({literal(tmp+"/linked")})',
            f'SELECT file FROM thinkthen_read_files([{literal(tmp+"/nested/z")},{literal(tmp+"/.hidden")},{literal(tmp+"/nested/z")}],\'{{"unit":"file"}}\')',
            "SELECT thinkthen_span_lines('café 😀\r\n\r\nAda\nlast',7,10,13)",
        ], backend.base())
        hidden,tail = str(folder/'.hidden'),str(folder/'nested/z')
        expect(rows(got[0]),[[1,'café 😀',hidden,1,1],[2,'Ada',hidden,3,3],[3,'last',hidden,4,4],[4,'tail',tail,1,1]],'physical lines')
        expect(rows(got[1]),[[1,'café 😀\r\n\r\nAda',hidden,1,3],[2,'last',hidden,4,4],[3,'tail',tail,1,1]],'window bytes')
        expect(rows(got[2]),[[1,'café 😀\r\n\r\nAda\nlast',hidden,1,4],[2,'tail\n',tail,1,1]],'whole file bytes')
        expect(rows(got[3]),[['café 😀'],['Ada'],['last']],'explicit symlink file')
        expect(rows(got[4]),[[tail],[hidden],[tail]],'operand order and duplicate occurrences')
        expect(rows(got[5]),[[{'first_line':9,'last_line':9}]],'native Unicode/CRLF span mapper')
        expect(backend.count(),0,'reader sends nothing')


@case
def invalid_reader_inputs_and_unread_tail_send_nothing():
    with tempfile.TemporaryDirectory(prefix='thinkthen-reader-') as tmp, Backend() as backend:
        folder=Path(tmp)
        (folder/'good').write_text('good\n'); (folder/'invalid').write_bytes(b'\xffPRIVATE_INPUT_MARKER')
        (folder/'big').write_bytes(b'x'*(16*1024*1024+1))
        import os
        os.mkfifo(folder/'fifo')
        path=literal(tmp+'/good')
        invalid=[
            f"SELECT thinkthen_decide('Is it good?',record) FROM thinkthen_read_files([{path},{literal(tmp+'/missing')}])",
            f"SELECT * FROM thinkthen_read_files({literal(tmp+'/fifo')})",
            f"SELECT * FROM thinkthen_read_files({path},'{{\"unit\":\"window\",\"window\":0}}')",
            f"SELECT * FROM thinkthen_read_files({path},'{{\"unit\":\"line\",\"window\":2}}')",
            f"SELECT * FROM thinkthen_read_files({path},'{{\"extra\":\"PRIVATE_INPUT_MARKER\"}}')",
            'SELECT * FROM thinkthen_read_files([]::VARCHAR[])',
            "SELECT thinkthen_span_lines('a\nb',9223372036854775807,2,3)",
            "SELECT thinkthen_span_lines('abc',1,2,2)",
            f"SELECT * FROM thinkthen_read_files({literal(tmp+'/invalid')})",
            f"SELECT * FROM thinkthen_read_files({literal(tmp+'/big')})",
        ]
        (folder/'blocked').mkdir(); (folder/'blocked/x').write_text('blocked\n')
        (folder/'blocked').chmod(0)
        invalid += [f'SELECT * FROM thinkthen_read_files({literal(tmp)})',
                    f'SELECT * FROM thinkthen_read_files({literal(tmp+"/blocked")})']
        got=run([*invalid,f'SELECT record FROM thinkthen_read_files([{path},{literal(tmp+"/invalid")}]) LIMIT 1'],backend.base())
        for result in got[:-1]:
            expect('error' in result,True,'reader refusal')
            expect('PRIVATE_INPUT_MARKER' in said(result),False,'content secrecy')
        (folder/'blocked').chmod(0o700)
        expect(rows(got[-1]),[['good']],'unread next file not opened')
        expect(backend.count(),0,'all refusals send nothing')


@case
def host_authority_is_rechecked_on_prepared_execution_and_open():
    with tempfile.TemporaryDirectory(prefix='thinkthen-reader-') as tmp, Backend() as backend:
        folder=Path(tmp); (folder/'good').write_text('good\n')
        path=literal(tmp+'/good')
        query=f"SELECT thinkthen_decide('Is it good?',record) FROM thinkthen_read_files({path})"
        got=run([f'PREPARE reader AS {query}','SET enable_external_access=false','EXECUTE reader'],backend.base())
        expect('error' in got[-1],True,'prepared caller denied')
        expect(backend.count(),0,'prepared refusal sends nothing')
        denied=run(["SET disabled_filesystems='LocalFileSystem'",query],backend.base())
        expect('error' in denied[-1],True,'disabled local filesystem denied')
        expect(backend.count(),0,'disabled filesystem sends nothing')
        allowed=run([f'SET allowed_paths=[{path}]','SET enable_external_access=false',query,
                     f'SELECT * FROM thinkthen_read_files({literal(tmp)})'],backend.base())
        expect(rows(allowed[2]),[[True]],'explicit allowed path reads')
        expect('error' in allowed[3],True,'allowed file does not authorize folder')
        expect(backend.count(),1,'only explicitly allowed file judgment sent')
        directories=run([f'SET allowed_directories=[{literal(tmp)}]','SET enable_external_access=false',query],backend.base())
        expect(rows(directories[2]),[[True]],'allowed directory reads descendant')
        expect(backend.count(),2,'second isolated allowed judgment sent')


@case
def relocated_documents_reuse_answers_and_never_send_source_metadata():
    with tempfile.TemporaryDirectory(prefix='thinkthen-relocated-') as tmp, Backend() as backend:
        import shutil
        shutil.copytree(FIXTURE, Path(tmp)/'moved')
        def query(path):
            return f"SELECT thinkthen_decide('Does this document contain a support contract?',record) FROM thinkthen_read_files({literal(str(path))}, '{{\"unit\":\"file\"}}')"
        got=run([query(FIXTURE),query(Path(tmp)/'moved')],backend.base('arm/full/capture'))
        expect([rows(result) for result in got],[[[True],[True]]]*2,'relocated answer equality')
        expect(backend.count(),2,'relocated evidence reused identical cache keys')
        bodies=backend.capture()
        expect(len(bodies),2,'exact two original evidence requests')
        for body in bodies:
            expect(str(FIXTURE) in body or tmp in body,False,'file metadata excluded')
            expect('first_line' in body or 'last_line' in body,False,'line metadata excluded')
        texts = [p.read_text() for p in sorted(FIXTURE.iterdir())]
        expected = [f'The text is {json.dumps(text,ensure_ascii=False)}. Does this document contain a support contract?' for text in texts]
        expect([json.loads(body)['questions']['q1']['instructions'] for body in bodies],expected,'exact original evidence instructions')


@case
def invalid_utf8_filename_is_refused_by_reader_or_host_before_judgment():
    with tempfile.TemporaryDirectory(prefix='thinkthen-names-') as tmp, Backend() as backend:
        Path(tmp,'a').write_text('valid\n')
        try:
            descriptor=os.open(os.fsencode(tmp)+b'/z-\xff',os.O_CREAT|os.O_WRONLY,0o600)
        except OSError as error:
            if sys.platform != 'darwin' or error.errno != errno.EILSEQ:
                raise
            expect(backend.count(),0,'host filename refusal sends nothing')
            print('not run: invalid UTF-8 filename reader refusal; macOS rejected fixture creation with EILSEQ')
            return
        os.close(descriptor)
        got=run([f"SELECT thinkthen_decide('Is it valid?',record) FROM thinkthen_read_files({literal(tmp)})"],backend.base())
        expect(said(got[0]),'thinkthen local: source filename must be UTF-8 (retryable: no)','exact filename refusal')
        expect(backend.count(),0,'complete manifest refusal sends nothing')


@case
def oversized_sorted_manifest_refuses_before_admitting_content():
    with tempfile.TemporaryDirectory(prefix='thinkthen-manifest-') as tmp, Backend() as backend:
        folder=Path(tmp)
        # Stay below macOS's path limit while exceeding the same manifest cap.
        for _ in range(2):
            folder=folder/('d'*200)
            folder.mkdir()
        filename_bytes=200
        needed=(16*1024*1024)//(len(os.fsencode(folder))+1+filename_bytes+1)+2
        for ordinal in range(needed):
            (folder/(f'{ordinal:06d}-'+'f'*(filename_bytes-7))).touch()
        got=run([f"SELECT thinkthen_decide('Is it valid?',record) FROM thinkthen_read_files({literal(tmp)})"],backend.base())
        expect(said(got[0]),'thinkthen local: source manifest exceeds 16 MiB (retryable: no)','exact manifest refusal')
        expect(backend.count(),0,'oversized manifest admitted no content')


@case
def complete_files_reject_unknown_formats_before_filesystem_work():
    with tempfile.TemporaryDirectory(prefix='thinkthen-format-') as tmp, Backend() as backend:
        folder=Path(tmp)
        os.mkfifo(folder/'fifo')
        ordinary=folder/'ordinary.txt'
        ordinary.write_text('Refund please.')
        paths=[str(folder/'fifo'),'/dev/zero',str(folder/'missing'),str(ordinary)]
        statements=[]
        for path in paths:
            for format in ('text',42,None,False,{},[]):
                payload=json.dumps({'files':{'paths':[path],'format':format}})
                statements.append("SELECT thinkthen_decide_complete('{\"decide\":\"Refund?\"}',"+literal(payload)+")")
        trace=folder/'format-io.trace'
        wrap=['strace','-f','-e','trace=openat,newfstatat,statx,access,readlink','-o',str(trace)] if sys.platform=='linux' else None
        got=run(statements,backend.base(),timeout=5,wrap=wrap)
        if wrap:
            calls=trace.read_text()
            assert calls and all('"'+path+'"' not in calls for path in paths),calls
        for result in got:
            value=json.loads(rows(result)[0][0])
            expect(value['native']['error']['kind'],'usage','format refusal precedes source access')
            expect(value['native']['error']['message'],'file format is jsonl, csv or tsv','exact format diagnostic')
            assert 'facts' not in value['native'] and value['observations']==[],value
        expect(backend.count(),0,'unknown formats send nothing')
        valid=json.dumps({'files':{'paths':[str(folder/'missing')],'format':'jsonl'}})
        value=json.loads(rows(run(["SELECT thinkthen_decide_complete('{\"decide\":\"Refund?\"}',"+literal(valid)+")"],backend.base())[-1])[0][0])
        expect(value['native']['error']['kind'],'local','admitted format reaches the native reader')
        expect(backend.count(),0,'missing valid JSON-line file sends nothing')


@case
def complete_file_admission_stops_before_an_unread_suffix():
    with tempfile.TemporaryDirectory(prefix='thinkthen-owned-feed-') as tmp, Backend() as backend:
        folder=Path(tmp)
        prefix=[]
        for ordinal in range(64):
            path=folder/f'prefix-{ordinal:02d}'
            path.write_text(f'Refund {ordinal}.')
            prefix.append(str(path))
        sentinel=folder/'unread-sentinel'
        sentinel.write_bytes(b'\xffPRIVATE_UNREAD_SUFFIX')
        payload=json.dumps({'files':{'paths':prefix+[str(sentinel)]},'incremental':True,'attempts':True})
        trace=folder/'owned-feed.trace'
        wrap=['strace','-f','-e','trace=openat','-o',str(trace)] if sys.platform=='linux' else None
        got=run(['SET thinkthen_throttle=1','SET thinkthen_max_requests=1',
                 "SELECT thinkthen_decide_complete('Refund?',"+literal(payload)+",'{\"batch\":1}')"],backend.base(),wrap=wrap,timeout=5)
        value=json.loads(rows(got[-1])[0][0])
        expect(value['native']['error']['kind'],'usage','native admission wins over unread UTF-8 error')
        expect(value['native']['facts']['requests_sent'],1,'actual started prefix sends once')
        expect(len(value['completed']),1,'completed prefix is retained')
        expect(value['ordinals'],[0],'original prefix ordinal')
        expect('PRIVATE_UNREAD_SUFFIX' in json.dumps(value),False,'suffix secrecy')
        expect(backend.count(),1,'bounded native requests')
        if wrap:
            calls=trace.read_text()
            expect('"'+str(sentinel)+'"' in calls,False,'sentinel content was never opened')
            opened=sum('"'+path+'"' in calls for path in prefix)
            assert 1 <= opened <= 4,('native excess row, queued input and one producer retry',opened)


@case
def complete_file_preflight_and_eager_invalid_input_send_nothing():
    with tempfile.TemporaryDirectory(prefix='thinkthen-owned-preflight-') as tmp, Backend() as backend:
        folder=Path(tmp); good=folder/'good'; bad=folder/'bad'
        good.write_text('Refund please.');bad.write_bytes(b'\xffPRIVATE_INPUT_MARKER')
        files={'files':{'paths':[str(good),str(bad)]}}
        trace=folder/'preflight.trace'
        wrap=['strace','-f','-e','trace=openat','-o',str(trace)] if sys.platform=='linux' else None
        invalid=[('choose',{'choose':'Which?','options':['only']}),('decide',{'decide':'Refund?'})]
        statements=["SELECT thinkthen_"+verb+"_complete("+literal(json.dumps(question))+","+literal(json.dumps({**files,'reading':{'unknown':'PRIVATE_INPUT_MARKER'}}))+')' for verb,question in invalid]
        statements.append("SELECT thinkthen_decide_complete('{\"decide\":\"Refund?\",\"profile\":42}',"+literal(json.dumps(files))+')')
        path_refusals=[(None,'descriptor is one object with unique fields'),
                       ({},'files requires paths'),
                       ({'paths':None},'file paths is a text array'),
                       ({'paths':42},'file paths is a text array'),
                       ({'paths':'not-an-array'},'file paths is a text array'),
                       ({'paths':[42]},'file paths is a text array'),
                       ({'paths':[None]},'file paths is a text array'),
                       ({'paths':[str(good),42]},'file paths is a text array')]
        path_start=len(statements)
        statements.extend("SELECT thinkthen_decide_complete('Refund?',"+literal(json.dumps({'files':invalid_paths}))+')'
                          for invalid_paths,_ in path_refusals)
        image_files={'files':{'paths':[str(good)],'options':{'reading':{'unit':'file'},'media':'image'}}}
        statements.append("SELECT thinkthen_tag_complete('{\"tag\":\"Topics?\",\"labels\":[\"refund\"]}',"+literal(json.dumps(image_files))+')')
        got=run(statements,backend.base(),wrap=wrap,timeout=5)
        for result in got:
            value=json.loads(rows(result)[0][0]);expect(value['native']['error']['kind'],'usage','static refusal')
            assert 'facts' not in value['native'] and value['observations']==[],value
        for result,(_,message) in zip(got[path_start:],path_refusals):
            expect(json.loads(rows(result)[0][0])['native']['error']['message'],message,'native paths diagnostic')
        if wrap:
            calls=trace.read_text()
            expect(any('"'+str(path)+'"' in calls for path in (good,bad)),False,'preflight opens no content')
        eager=json.loads(rows(run(["SELECT thinkthen_decide_complete('Refund?',"+literal(json.dumps(files))+')'],backend.base())[-1])[0][0])
        expect(eager['native']['error']['kind'],'usage','exact native UTF-8 kind')
        assert 'facts' not in eager['native'],eager
        expect(backend.count(),0,'eager late-invalid input sends nothing')
        incremental={**files,'incremental':True,'attempts':True}
        prefix=json.loads(rows(run(["SELECT thinkthen_decide_complete('Refund?',"+literal(json.dumps(incremental))+",'{\"batch\":1}')"],backend.base())[-1])[0][0])
        expect(prefix['native']['error']['kind'],'usage','native reader error retains kind')
        expect(prefix['native']['error']['message'],eager['native']['error']['message'],'native reader diagnostic survives finish')
        expect(prefix['native']['facts']['requests_sent'],1,'completed prefix has actual send facts')
        expect(len(prefix['completed']),1,'reader failure retains accepted completed prefix')
        expect(prefix['ordinals'],[0],'reader failure preserves prefix ordinal')
        expect(len(prefix['observations']),2,'actual question and row observations precede failure')
        expect(backend.count(),1,'only the incremental completed prefix sends')


if __name__=='__main__':
    sys.exit(main())
