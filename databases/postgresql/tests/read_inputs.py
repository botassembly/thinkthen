"""The actual client reader validates explicit file formats before source access."""
import json,os,subprocess,tempfile
from pathlib import Path
import sys
ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT/'databases/sqlite/tests'))
from helper import Backend
READER=ROOT/'databases/postgresql/target/debug/thinkthen_read_inputs'
with tempfile.TemporaryDirectory(prefix='thinkthen-client-format-') as tmp:
    folder=Path(tmp)
    os.mkfifo(folder/'fifo')
    ordinary=folder/'ordinary.txt';ordinary.write_text('Refund please.')
    with_backend=Backend()
    try:
        for path in (str(folder/'fifo'),'/dev/zero',str(folder/'missing'),str(ordinary)):
            for format in ('text',42,None,False,{},[]):
                payload=json.dumps({'files':{'paths':[path],'format':format}})
                trace=folder/'format-io.trace'
                command=['strace','-f','-e','trace=openat,newfstatat,statx,access,readlink','-o',str(trace),str(READER)] if sys.platform=='linux' else [str(READER)]
                done=subprocess.run(command,input=payload,text=True,capture_output=True,timeout=5,
                                    env={'PATH':os.environ['PATH'],'HOME':tmp,'THINKTHEN_BASE_URL':with_backend.base(),'THINKTHEN_API_KEY':'sk-sqlite-loopback'})
                assert done.returncode==1 and done.stdout=='',(done.returncode,done.stdout,done.stderr)
                assert done.stderr.startswith('Error: Usage(') and 'file format is jsonl' in done.stderr,done.stderr
                if sys.platform=='linux':
                    calls=trace.read_text()
                    assert calls and '"'+path+'"' not in calls,calls
        for format in (None,'jsonl'):
            source=folder/'valid.jsonl';source.write_text('"Refund please."\n')
            files={'paths':[str(source)],**({'format':format} if format is not None else {})}
            done=subprocess.run([str(READER)],input=json.dumps({'files':files}),text=True,capture_output=True,timeout=5,
                                env={'PATH':os.environ['PATH'],'HOME':tmp})
            assert done.returncode==0 and done.stderr=='',(done.returncode,done.stderr)
            value=json.loads(done.stdout)
            assert 'files' not in value and len(value['records'])==1,value
            assert value['records'][0]['json_text' if format else 'text']=='"Refund please."',value
        assert with_backend.close()==0
        print('client reader: invalid formats refuse before I/O, admitted text/JSON lines preserve descriptors, zero requests')
    finally:
        if with_backend.process.poll() is None:with_backend.process.kill();with_backend.process.wait()
