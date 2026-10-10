#!/usr/bin/env python3
"""Explicit file rows preserve bytes and positions without crossing schema authority."""
import json
import pathlib
import sys
import tempfile

from helper import Backend, child, environment, expect, main
from test_trusted_schema import TRUSTED

FIXTURE = pathlib.Path(__file__).resolve().parents[3] / 'specification/fixtures/files/documents'


def test_folder_reader_and_joined_judgment_keep_original_documents():
    backend = Backend()
    held = child(f"""
db = connect()
rows = run(db, 'SELECT * FROM thinkthen_read_files(?, ?)', ({str(FIXTURE)!r}, '{{"unit":"file"}}'))
judged = run(db, "SELECT ordinal,record,file,first_line,last_line,thinkthen_decide('Is this a contract?', record) FROM thinkthen_read_files(?, ?)", ({str(FIXTURE)!r}, '{{"unit":"file"}}'))
say(rows=rows, judged=judged)
""", environment(backend))
    expected = [[i, p.read_text(), str(p), 1, 4] for i, p in enumerate(sorted(FIXTURE.iterdir()), 1)]
    expect(held, {'rows': expected, 'judged': [r + [1] for r in expected]}, 'document rows and joined judgments')
    expect(backend.close(), 2, 'only two judgments send')


def test_reader_units_order_hidden_files_and_native_spans():
    with tempfile.TemporaryDirectory(prefix='thinkthen-reader-') as tmp:
        folder = pathlib.Path(tmp)
        (folder / '.hidden').write_bytes('café 😀\r\n\r\nAda\nlast'.encode())
        (folder / 'nested').mkdir()
        (folder / 'nested/z').write_text('tail\n')
        (folder / 'linked').symlink_to(folder / '.hidden')
        (folder / 'cycle').symlink_to(folder, target_is_directory=True)
        backend = Backend()
        held = child(f"""
db = connect()
path = {tmp!r}
lines = run(db, 'SELECT * FROM thinkthen_read_files(?)', (path,))
windows = run(db, 'SELECT * FROM thinkthen_read_files(?, ?)', (path, '{{"unit":"window","window":3}}'))
whole = run(db, 'SELECT * FROM thinkthen_read_files(?, ?)', (path, '{{"unit":"file"}}'))
link = run(db, 'SELECT record FROM thinkthen_read_files(?)', (path + '/linked',))
order = run(db, 'SELECT file FROM thinkthen_read_files(?, ?)', (json.dumps([path+'/nested/z',path+'/.hidden',path+'/nested/z']), '{{"unit":"file"}}'))
span = run(db, 'SELECT thinkthen_span_lines(?, ?, ?, ?)', ('café 😀\\r\\n\\r\\nAda\\nlast', 7, 10, 13))
say(lines=lines, windows=windows, whole=whole, link=link, order=order, span=span)
""", environment(backend))
        hidden, tail = str(folder / '.hidden'), str(folder / 'nested/z')
        expect(held['lines'], [[1,'café 😀',hidden,1,1],[2,'Ada',hidden,3,3],[3,'last',hidden,4,4],[4,'tail',tail,1,1]], 'physical lines')
        expect(held['windows'], [[1,'café 😀\r\n\r\nAda',hidden,1,3],[2,'last',hidden,4,4],[3,'tail',tail,1,1]], 'short final window')
        expect(held['whole'], [[1,'café 😀\r\n\r\nAda\nlast',hidden,1,4],[2,'tail\n',tail,1,1]], 'exact whole file bytes')
        expect(held['link'], [['café 😀'],['Ada'],['last']], 'explicit symlink file policy')
        expect(held['order'], [[tail],[hidden],[tail]], 'operand order and duplicate occurrences')
        expect(json.loads(held['span'][0][0]), {'first_line':9,'last_line':9}, 'Unicode CRLF span')
        expect(backend.close(), 0, 'all readers and mapping send nothing')


def test_invalid_operands_options_content_and_complete_manifest_send_nothing():
    with tempfile.TemporaryDirectory(prefix='thinkthen-reader-') as tmp:
        folder = pathlib.Path(tmp)
        good = folder / 'a'; good.write_text('good\n')
        bad = folder / 'b'; bad.write_bytes(b'\xffPRIVATE_INPUT_MARKER')
        big = folder / 'big'; big.write_bytes(b'x' * (16*1024*1024+1))
        special = folder / 'fifo'
        import os
        os.mkfifo(special)
        backend = Backend()
        held = child(f"""
db = connect()
path = {tmp!r}
cases = [
    (path+'/missing','{{}}'), (path+'/fifo','{{}}'), (path,'{{}}'),
    (json.dumps([path+'/a',path+'/missing']),'{{}}'),
    (path+'/a','{{"unit":"window","window":0}}'), (path+'/a','{{"unit":"line","window":2}}'),
    (path+'/a','{{"extra":"PRIVATE_INPUT_MARKER"}}'), ('[]','{{}}'),
    (path+'/b','{{}}'), (path+'/big','{{}}')]
say(errors=[run(db, "SELECT thinkthen_decide('Is it good?',record) FROM thinkthen_read_files(?, ?)", args) for args in cases],
    lazy=run(db, 'SELECT record FROM thinkthen_read_files(?) LIMIT 1', (json.dumps([path+'/a',path+'/b']),)))
""", environment(backend))
        for error in held['errors']:
            expect(isinstance(error, str) and error.startswith('thinkthen '), True, 'named refusal')
            expect('PRIVATE_INPUT_MARKER' in error, False, 'input secrecy')
        expect(held['lazy'], [['good']], 'unread next file is not admitted')
        expect(backend.close(), 0, 'all refusals send nothing')


def test_complete_files_reject_unknown_formats_before_source_access():
    with tempfile.TemporaryDirectory(prefix='thinkthen-complete-format-') as tmp:
        folder=pathlib.Path(tmp)
        import os
        os.mkfifo(folder/'fifo')
        (folder/'ordinary').write_text('Refund please.')
        paths=[str(folder/'fifo'),'/dev/zero',str(folder/'missing'),str(folder/'ordinary')]
        backend=Backend()
        held=child(f"""
db=connect()
errors=[]
for path in {paths!r}:
    for format in ('text',42,None,False,{{}},[]):
        value=json.loads(db.execute('SELECT thinkthen_decide_complete(?,?)',('{{"decide":"Refund?"}}',json.dumps({{'files':{{'paths':[path],'format':format}}}}))).fetchone()[0])
        errors.append([value['native']['error']['kind'],value['native']['error']['message'],'facts' in value['native'],value['observations']])
say(errors=errors)
""",environment(backend),5)
        expect(held['errors'],[['usage','file format is jsonl, csv or tsv',False,[]]]*24,'unknown explicit formats refuse before I/O')
        expect(backend.close(),0,'unknown complete formats send nothing')


def test_reader_is_direct_only_in_trusted_mode_and_caller_temp_still_works():
    backend = Backend()
    held = child(TRUSTED + f"""
results = []
for entry in ('sqlite3_thinkthen_init','sqlite3_thinkthen_trusted_init'):
    for setting in ('ON','OFF'):
        db = sqlite3.connect(':memory:', isolation_level=None)
        db.enable_load_extension(True)
        db.execute('SELECT load_extension(?, ?)', (LIB,entry))
        db.execute('PRAGMA trusted_schema='+setting)
        for schema in ('main','temp'):
            db.execute(f"CREATE VIEW {{schema}}.v AS SELECT * FROM thinkthen_read_files('{FIXTURE}')")
        results.append([run(db,'SELECT count(*) FROM main.v'),run(db,'SELECT count(*) FROM temp.v')])
say(results=results)
""", environment(backend))
    expect(held['results'], [['unsafe use of virtual table "thinkthen_read_files"', [[8]]]]*4, 'persistent schema blocked, caller TEMP retained')
    expect(backend.close(), 0, 'schema and direct readers send nothing')


def test_relate_retains_255_source_row_cap_before_deduplication():
    backend = Backend()
    held = child("""
db = connect()
db.execute('CREATE TABLE entities(id INTEGER,name TEXT,kind TEXT)')
db.executemany('INSERT INTO entities VALUES (?,?,?)', [(n,'same','*') for n in range(256)])
say(error=run(db, "SELECT * FROM thinkthen_relate('SELECT id,name,kind FROM entities','supports=*:*')"))
""", environment(backend))
    expect(held['error'], 'thinkthen usage: thinkthen_relate takes at most 255 source rows (retryable: no)', 'source row cap')
    expect(backend.close(), 0, 'oversized source set sends nothing')


def test_invalid_utf8_discovered_filename_refuses_before_any_judgment():
    import os
    with tempfile.TemporaryDirectory(prefix='thinkthen-names-') as tmp:
        pathlib.Path(tmp,'a').write_text('valid\n')
        descriptor=os.open(os.fsencode(tmp)+b'/z-\xff',os.O_CREAT|os.O_WRONLY,0o600)
        os.close(descriptor)
        backend=Backend()
        held=child(f"""
db=connect()
say(result=run(db, "SELECT thinkthen_decide('Is it valid?',record) FROM thinkthen_read_files(?)", ({tmp!r},)))
""", environment(backend))
        sends=backend.close()
        expect(isinstance(held['result'],str) and 'UTF-8' in held['result'],True,'native manifest filename preflight')
        expect(sends,0,'complete invalid filename manifest sends nothing')


def test_span_mapping_refuses_empty_spans_and_sql_integer_overflow():
    backend=Backend()
    held=child("""
db=connect()
say(empty=run(db,'SELECT thinkthen_span_lines(?,1,2,2)',('abc',)),
    large=run(db,'SELECT thinkthen_span_lines(?,9223372036854775807,2,3)',('a\\nb',)))
""",environment(backend))
    expect(held['large'],'thinkthen usage: source line exceeds SQL INTEGER (retryable: no)','SQL coordinate overflow')
    expect(held['empty'],'thinkthen usage: source span is outside its record (retryable: no)','empty span')
    expect(backend.close(),0,'mapper refusals send nothing')


if __name__ == '__main__':
    sys.exit(main(globals()))
