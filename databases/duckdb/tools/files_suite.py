"""Explicit host file rows and ten judgments over the shared folder fixture."""
import json
import sys
import tempfile
from pathlib import Path

from harness import Backend, REPO_ROOT, case, expect, main, rows, run, said

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
    expect(len(statements), 11, 'materialization plus all ten examples')
    with Backend() as backend:
        got = run(['SET thinkthen_cache=\'off\'', *statements], backend.base())
        expected = [[i, p.read_text(), str(p), 1, 4] for i,p in enumerate(sorted(FIXTURE.iterdir()), 1)]
        for index, values in ((2,[True,True]),(3,['policy','policy']),
                              (4,[['refund','support','billing']]*2),(5,[0.1,0.1])):
            expect(rows(got[index]), [r+[v] for r,v in zip(expected,values)], 'row judgment original bytes/locations')
        expect(rows(got[6]), expected, 'filter original document rows')
        expect(rows(got[7]), [r+[i,0.9] for i,r in enumerate(expected,1)], 'rank joins source ids')
        expect(rows(got[8]), [expected[0]+[0.9]], 'find mapped zero-based position')
        expect([r[:5] for r in rows(got[9])], expected, 'annotation metadata stays outside original record')
        expect([json.loads(r[5]) for r in rows(got[9])], [{'contract':True,'urgent':True}]*2, 'named annotations')
        recognized = rows(got[10])
        expect(len(recognized), 4, 'recognition rows for two requested kinds')
        for row in recognized:
            original = expected[row[0]-1]
            ordinal,record,file,name,start,end,kind,first,last = row
            expect([ordinal,record,file], original[:3], 'recognition source')
            expect(record[start:end],name,'native Unicode offsets')
            expect([first,last], [1+record[:start].count('\n'),1+record[:end-1].count('\n')], 'recognized physical lines from actual offsets')
        endpoints = rows(got[11])
        expect(len(endpoints), 2, 'both directed document edges')
        for edge in endpoints:
            expect(edge[1:6], expected[edge[1]-1], 'actual source endpoint row')
            expect(edge[6:11], expected[edge[6]-1], 'actual target endpoint row')
            expect(edge[0], 'supports', 'relation')
            expect(edge[11], 0.9, 'edge probability')
        expect(backend.count() > 0, True, 'owned loopback actually judged all ten examples')


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
def invalid_utf8_discovered_filename_is_refused_before_judgment():
    with tempfile.TemporaryDirectory(prefix='thinkthen-names-') as tmp, Backend() as backend:
        import os
        Path(tmp,'a').write_text('valid\n')
        descriptor=os.open(os.fsencode(tmp)+b'/z-\xff',os.O_CREAT|os.O_WRONLY,0o600)
        os.close(descriptor)
        got=run([f"SELECT thinkthen_decide('Is it valid?',record) FROM thinkthen_read_files({literal(tmp)})"],backend.base())
        expect(said(got[0]),'thinkthen local: source filename must be UTF-8 (retryable: no)','exact filename refusal')
        expect(backend.count(),0,'complete manifest refusal sends nothing')


@case
def oversized_sorted_manifest_refuses_before_admitting_content():
    with tempfile.TemporaryDirectory(prefix='thinkthen-manifest-') as tmp, Backend() as backend:
        folder=Path(tmp)
        for _ in range(14):
            folder=folder/('d'*220)
            folder.mkdir()
        needed=(16*1024*1024)//(len(str(folder))+8)+2
        for ordinal in range(needed):
            (folder/f'{ordinal:06d}').touch()
        got=run([f"SELECT thinkthen_decide('Is it valid?',record) FROM thinkthen_read_files({literal(tmp)})"],backend.base())
        expect(said(got[0]),'thinkthen local: source manifest exceeds 16 MiB (retryable: no)','exact manifest refusal')
        expect(backend.count(),0,'oversized manifest admitted no content')


if __name__=='__main__':
    sys.exit(main())
