"""Actual named SQL consumer; read complete native receipts through typed projections."""
import json,sys,sqlite3,threading
payload=json.loads(sys.stdin.readline())
db=sqlite3.connect(':memory:',isolation_level=None,check_same_thread=False)
db.enable_load_extension(True)
db.load_extension(sys.argv[1])
settings=payload['engine_settings']

if payload.get('held_cancel'):
    def cancel():
        sys.stdin.readline();db.interrupt();print('cancel-fired',flush=True)
    threading.Thread(target=cancel,daemon=True).start()
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from native_receipts import call
try:
    db.execute('SELECT thinkthen_configure(?)',[json.dumps(settings)])
    result=call(db,payload['verb'],payload['question'],payload['inputs'],json.dumps(payload.get('controls',{})))
    print(json.dumps(result),flush=True)
except sqlite3.Error as error:
    print(json.dumps({'host_error':str(error),'host_code':error.sqlite_errorcode}),flush=True)
